#!/usr/bin/env python3
"""Controles sintéticos V03-F1-P01; nenhuma partição cognitiva é acessada."""
import argparse
import json
import mmap
import os
from pathlib import Path
import secrets
import signal
import subprocess
import sys
import time

LIMIT = 48 * 1024 * 1024
DEADLINE = 12.0
ROOT = Path('/sys/fs/cgroup')


def cgroup_path(pid):
    for line in Path(f'/proc/{pid}/cgroup').read_text().splitlines():
        if line.startswith('0::'):
            relative = line[3:]
            if not relative.startswith('/') or '..' in Path(relative).parts:
                raise ValueError('cgroup inválido')
            return ROOT / relative.lstrip('/')
    raise ValueError('cgroup v2 indisponível')


def events(path):
    return {k: int(v) for k, v in (line.split() for line in path.read_text().splitlines())}


def snapshot(path):
    peak = path / 'memory.peak'
    swap_events = path / 'memory.swap.events'
    return {'max': (path / 'memory.max').read_text().strip(),
            'current': int((path / 'memory.current').read_text()),
            'peak': int(peak.read_text()) if peak.exists() else None,
            'events': events(path / 'memory.events'),
            'swap_max': (path / 'memory.swap.max').read_text().strip(),
            'swap_current': int((path / 'memory.swap.current').read_text()),
            'swap_events': events(swap_events) if swap_events.exists() else None,
            'pids': sorted(int(x) for x in (path / 'cgroup.procs').read_text().split())}


def require_synthetic_limits(path):
    """Falha fechada: pressão só é permitida com ambos os limites verificáveis."""
    try:
        memory_max = (path / 'memory.max').read_text().strip()
        swap_max = (path / 'memory.swap.max').read_text().strip()
    except OSError as exc:
        raise RuntimeError('limites de memória e swap não verificáveis') from exc
    if memory_max != str(LIMIT) or swap_max != '0':
        raise RuntimeError('limites sintéticos de memória ou swap divergentes')


def valid_swap_events(value):
    return value is None or (isinstance(value, dict) and bool(value)
                             and all(isinstance(k, str) and type(v) is int and v >= 0
                                     for k, v in value.items()))


def evaluate(record):
    """Resultado conservador: evidência incompleta nunca passa."""
    try:
        before, after = record['before'], record['after']
        delta = {k: after['events'][k] - before['events'][k]
                 for k in ('max', 'oom', 'oom_kill')}
        checks = {
            'A_abaixo': record['below_exit'] == 0 and record['below_inside'],
            'B_pressao': delta['max'] > 0,
            'C_oom': (delta['oom_kill'] > 0 and record['pressure_exit'] == -signal.SIGKILL
                      and record['pressure_inside'] is True),
            'D_filhos': (record['below_inside'] and record['grandchild_inside']
                         and record['pressure_inside'] and record['timeout_inside']),
            'E_timeout': record['timeout_occurred'] and record['timeout_exit'] == -signal.SIGKILL,
            'F_sobreviventes': record['children_reaped'] and record['remaining_pids'] == [record['coordinator_pid']],
            'limite': before['max'] == str(LIMIT) and after['max'] == str(LIMIT),
            'swap_isolado': (before['swap_max'] == '0' and after['swap_max'] == '0'
                              and type(before['swap_current']) is int and before['swap_current'] >= 0
                              and type(after['swap_current']) is int and after['swap_current'] >= 0
                              and valid_swap_events(before['swap_events'])
                              and valid_swap_events(after['swap_events'])),
            'tempo': 0 <= record['elapsed_s'] < DEADLINE,
        }
        return {'checks': checks, 'event_delta': delta,
                'status': 'CONTROLES_SINTETICOS_APROVADOS' if all(checks.values()) else 'INCONCLUSIVO'}
    except (KeyError, TypeError, ValueError):
        return {'status': 'INCONCLUSIVO', 'reason': 'evidência ausente ou inválida'}


def touch_pages(block, size, page_size):
    """Escreve um byte em cada página, sem pressupor alocação física de zeros."""
    if size <= 0 or page_size <= 0:
        raise ValueError('tamanho inválido')
    for offset in range(0, size, page_size):
        block[offset] = 1


def worker(mode):
    if mode == 'pressure':
        require_synthetic_limits(cgroup_path(os.getpid()))
    if mode == 'below':
        grand = subprocess.Popen([sys.executable, __file__, '--worker', 'grandchild'], stdout=subprocess.PIPE, text=True)
        try:
            line = grand.stdout.readline().strip()
            print(json.dumps({'pid': os.getpid(), 'grandchild': int(line)}), flush=True)
            time.sleep(1.0)
        finally:
            grand.terminate()
            grand.wait(timeout=2)
    elif mode == 'grandchild':
        print(os.getpid(), flush=True)
        time.sleep(5)
    elif mode == 'pressure':
        time.sleep(0.1)
        blocks = []
        page_size = os.sysconf('SC_PAGE_SIZE')
        while len(blocks) < 96:
            block = mmap.mmap(-1, 1024 * 1024, access=mmap.ACCESS_WRITE)
            blocks.append(block)
            touch_pages(block, len(block), page_size)
        raise RuntimeError('pressão não provocou OOM dentro do teto de carga')
    elif mode == 'timeout':
        time.sleep(10)
    else:
        raise ValueError('modo desconhecido')


def coordinate():
    started = time.monotonic()
    path = cgroup_path(os.getpid())
    out = {'coordinator_pid': os.getpid(), 'cgroup': str(path), 'before': snapshot(path)}
    require_synthetic_limits(path)
    below = subprocess.Popen([sys.executable, __file__, '--worker', 'below'], stdout=subprocess.PIPE, text=True)
    try:
        ids = json.loads(below.stdout.readline())
        out['pids'] = ids
        out['below_inside'] = cgroup_path(ids['pid']) == path
        out['grandchild_inside'] = cgroup_path(ids['grandchild']) == path
        out['below_exit'] = below.wait(timeout=3)
    finally:
        if below.poll() is None:
            below.kill()
            below.wait()
    require_synthetic_limits(path)
    pressure = subprocess.Popen([sys.executable, __file__, '--worker', 'pressure'])
    try:
        out['pressure_inside'] = cgroup_path(pressure.pid) == path
        out['pids']['pressure'] = pressure.pid
        out['pressure_exit'] = pressure.wait(timeout=5)
    finally:
        if pressure.poll() is None:
            pressure.kill()
            pressure.wait()
    timed = subprocess.Popen([sys.executable, __file__, '--worker', 'timeout'])
    try:
        out['timeout_inside'] = cgroup_path(timed.pid) == path
        out['pids']['timeout'] = timed.pid
        try:
            timed.wait(timeout=0.2)
            out['timeout_occurred'] = False
        except subprocess.TimeoutExpired:
            out['timeout_occurred'] = True
            timed.kill()
            timed.wait(timeout=2)
        out['timeout_exit'] = timed.returncode
    finally:
        if timed.poll() is None:
            timed.kill()
            timed.wait()
    out['children_reaped'] = all(p.poll() is not None for p in (below, pressure, timed))
    out['remaining_pids'] = sorted(int(x) for x in (path / 'cgroup.procs').read_text().split())
    out['after'] = snapshot(path)
    out['elapsed_s'] = time.monotonic() - started
    out['assessment'] = evaluate(out)
    print('NEURONS_CGROUP_EVIDENCE=' + json.dumps(out, sort_keys=True), flush=True)
    return 0 if out['assessment']['status'] == 'CONTROLES_SINTETICOS_APROVADOS' else 1


def run_host():
    unit = 'neurons-v03-f1-' + secrets.token_hex(8)
    command = ['systemd-run', '--user', '--scope', '--quiet', '--unit=' + unit,
               '--property=MemoryMax=' + str(LIMIT), '--property=MemorySwapMax=0',
               sys.executable, str(Path(__file__).resolve()), '--coordinate']
    started = time.monotonic()
    print(json.dumps({'supervisor_pid': os.getpid(), 'scope': unit + '.scope', 'command': command}), flush=True)
    proc = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    timed_out = False
    try:
        stdout, stderr = proc.communicate(timeout=DEADLINE)
    except subprocess.TimeoutExpired:
        timed_out = True
        try:
            subprocess.run(['systemctl', '--user', 'stop', unit + '.scope'], timeout=5, check=False)
        except (OSError, subprocess.TimeoutExpired) as exc:
            print('scope_stop_inconclusivo=' + str(exc), file=sys.stderr, flush=True)
        finally:
            if proc.poll() is None:
                proc.kill()
        stdout, stderr = proc.communicate(timeout=5)
    print(stdout, end='')
    print(stderr, end='', file=sys.stderr)
    # Scope disappearance is a necessary cleanup check, never sufficient alone.
    shown = subprocess.run(['systemctl', '--user', 'show', unit + '.scope', '--property=LoadState', '--property=ActiveState', '--property=ControlGroup'], capture_output=True, text=True, timeout=5)
    properties = dict(line.split('=', 1) for line in shown.stdout.splitlines() if '=' in line)
    cleaned = (shown.returncode == 0 and properties.get('ActiveState') == 'inactive'
               and 'ControlGroup' in properties and properties['ControlGroup'] == '')
    evidence = [x for x in stdout.splitlines() if x.startswith('NEURONS_CGROUP_EVIDENCE=')]
    success = not timed_out and proc.returncode == 0 and len(evidence) == 1 and cleaned
    print(json.dumps({'supervisor_elapsed_s': time.monotonic()-started, 'scope_exit': proc.returncode,
                      'timeout': timed_out, 'cleanup_observed': cleaned, 'scope_properties': properties,
                      'status': 'CONTROLES_SINTETICOS_APROVADOS' if success else 'INCONCLUSIVO'}), flush=True)
    return 0 if success else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--run-host-controls', action='store_true')
    group.add_argument('--coordinate', action='store_true', help=argparse.SUPPRESS)
    group.add_argument('--worker', choices=['below', 'grandchild', 'pressure', 'timeout'], help=argparse.SUPPRESS)
    args = parser.parse_args()
    if args.worker:
        worker(args.worker)
        return 0
    if args.coordinate:
        return coordinate()
    return run_host()


if __name__ == '__main__':
    raise SystemExit(main())
