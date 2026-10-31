#!/usr/bin/env python3
"""Controles sintéticos V03-F1-P01; nenhuma partição cognitiva é acessada."""
import argparse
import json
import mmap
import os
from pathlib import Path
import secrets
import selectors
import signal
import subprocess
import sys
import tempfile
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


def emit_stage(name, **details):
    """Entrega marcos ao supervisor externo antes de eventual morte do grupo."""
    print('NEURONS_CGROUP_STAGE=' + json.dumps({'stage': name, **details}, sort_keys=True), flush=True)


def verify_live_policy(unit, coordinator):
    """O supervisor externo verifica a unidade e a identidade do cgroup ativo."""
    shown = subprocess.run(['systemctl', '--user', 'show', unit + '.scope',
                            '--property=LoadState', '--property=ActiveState',
                            '--property=ControlGroup', '--property=OOMPolicy'],
                           capture_output=True, text=True, timeout=3)
    props = dict(line.split('=', 1) for line in shown.stdout.splitlines() if '=' in line)
    expected = str(coordinator.get('cgroup', ''))
    pid = coordinator.get('coordinator_pid')
    try:
        pid_in_scope = type(pid) is int and pid > 0 and str(cgroup_path(pid)) == expected
    except (OSError, ValueError):
        pid_in_scope = False
    valid = (shown.returncode == 0 and props.get('LoadState') == 'loaded'
             and props.get('ActiveState') == 'active' and props.get('OOMPolicy') == 'continue'
             and props.get('ControlGroup', '').startswith('/')
             and str(ROOT / props['ControlGroup'].lstrip('/')) == expected
             and Path(expected).name == unit + '.scope' and pid_in_scope)
    return {'confirmed': valid, 'scope': unit + '.scope', 'properties': props,
            'coordinator_pid': pid, 'cgroup': expected, 'pid_in_scope': pid_in_scope}


def confirmed_policy_evidence(evidence, unit):
    """Confronta política, unidade e caminho sem aceitar marcador isolado."""
    try:
        props = evidence['properties']
        path = Path(evidence['cgroup'])
        return (evidence['confirmed'] is True and evidence['pid_in_scope'] is True
                and evidence['scope'] == unit + '.scope'
                and type(evidence['coordinator_pid']) is int and evidence['coordinator_pid'] > 0
                and props['LoadState'] == 'loaded' and props['ActiveState'] == 'active'
                and props['OOMPolicy'] == 'continue' and props['ControlGroup'].startswith('/')
                and str(ROOT / props['ControlGroup'].lstrip('/')) == str(path)
                and path.name == unit + '.scope')
    except (KeyError, TypeError, ValueError):
        return False


def scope_cleanup_verified(properties, returncode):
    return (returncode == 0 and properties.get('ActiveState') == 'inactive'
            and 'ControlGroup' in properties and properties['ControlGroup'] == '')


def worker(mode):
    if mode == 'pressure':
        path = cgroup_path(os.getpid())
        require_synthetic_limits(path)
        try:
            policy = json.loads(os.environ.get('NEURONS_CGROUP_LIVE_POLICY', ''))
            if not confirmed_policy_evidence(policy, path.name.removesuffix('.scope')):
                raise ValueError('política inválida')
        except (ValueError, TypeError):
            raise RuntimeError('política OOM não confirmada; pressão não iniciada')
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
    emit_stage('coordinator_ready', coordinator_pid=os.getpid(), cgroup=str(path), snapshot=out['before'])
    try:
        policy = json.loads(sys.stdin.readline())
        unit = path.name.removesuffix('.scope')
        if not confirmed_policy_evidence(policy, unit) or policy['coordinator_pid'] != os.getpid():
            raise ValueError('política ou identidade não confirmada')
    except (ValueError, KeyError, TypeError):
        raise RuntimeError('política OOM não confirmada; pressão não iniciada')
    out['live_policy'] = policy
    emit_stage('policy_confirmed_before_pressure', **policy)
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
    emit_stage('before_pressure', coordinator_pid=os.getpid(), cgroup=str(path), snapshot=snapshot(path))
    pressure_env = dict(os.environ, NEURONS_CGROUP_LIVE_POLICY=json.dumps(policy))
    pressure = subprocess.Popen([sys.executable, __file__, '--worker', 'pressure'], env=pressure_env)
    try:
        out['pressure_inside'] = cgroup_path(pressure.pid) == path
        out['pids']['pressure'] = pressure.pid
        emit_stage('pressure_started', pressure_pid=pressure.pid, pressure_inside=out['pressure_inside'])
        out['pressure_exit'] = pressure.wait(timeout=5)
        emit_stage('pressure_finished', pressure_pid=pressure.pid, pressure_exit=out['pressure_exit'], snapshot=snapshot(path))
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
               '--property=MemoryMax=' + str(LIMIT), '--property=MemorySwapMax=0', '--property=OOMPolicy=continue',
               sys.executable, str(Path(__file__).resolve()), '--coordinate']
    started = time.monotonic()
    print(json.dumps({'supervisor_pid': os.getpid(), 'scope': unit + '.scope', 'command': command}), flush=True)
    error_file = tempfile.TemporaryFile(mode='w+t')
    proc = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=error_file)
    timed_out = False
    live_policy = None
    chunks = []
    pending = b''
    try:
        with selectors.DefaultSelector() as selector:
            selector.register(proc.stdout, selectors.EVENT_READ)
            while selector.get_map():
                remaining = DEADLINE - (time.monotonic() - started)
                if remaining <= 0:
                    raise subprocess.TimeoutExpired(command, DEADLINE)
                for key, _ in selector.select(remaining):
                    chunk = os.read(key.fd, 65536)
                    if not chunk:
                        selector.unregister(key.fileobj)
                        break
                    chunks.append(chunk)
                    pending += chunk
                    while b'\n' in pending:
                        line, pending = pending.split(b'\n', 1)
                        prefix = b'NEURONS_CGROUP_STAGE='
                        if line.startswith(prefix):
                            try:
                                stage = json.loads(line[len(prefix):])
                                if stage.get('stage') == 'coordinator_ready':
                                    if live_policy is not None:
                                        raise ValueError('marcador duplicado')
                                    live_policy = verify_live_policy(unit, stage)
                                    if not confirmed_policy_evidence(live_policy, unit):
                                        raise ValueError('política não confirmada')
                                    proc.stdin.write((json.dumps(live_policy) + '\n').encode())
                                    proc.stdin.flush()
                            except (OSError, ValueError, TypeError, AttributeError, subprocess.TimeoutExpired) as exc:
                                print('politica_inconclusiva=' + str(exc), file=sys.stderr, flush=True)
                                proc.stdin.close()
        proc.wait(timeout=max(0.01, DEADLINE - (time.monotonic() - started)))
    except subprocess.TimeoutExpired:
        timed_out = True
        try:
            subprocess.run(['systemctl', '--user', 'stop', unit + '.scope'], timeout=5, check=False)
        except (OSError, subprocess.TimeoutExpired) as exc:
            print('scope_stop_inconclusivo=' + str(exc), file=sys.stderr, flush=True)
        finally:
            if proc.poll() is None:
                proc.kill()
        proc.wait(timeout=5)
    finally:
        if proc.stdin and not proc.stdin.closed:
            proc.stdin.close()
    stdout = b''.join(chunks).decode('utf-8', errors='replace')
    error_file.seek(0)
    stderr = error_file.read()
    error_file.close()
    print(stdout, end='')
    print(stderr, end='', file=sys.stderr)
    # Scope disappearance is a necessary cleanup check, never sufficient alone.
    shown = subprocess.run(['systemctl', '--user', 'show', unit + '.scope', '--property=LoadState', '--property=ActiveState', '--property=ControlGroup', '--property=Result', '--property=OOMPolicy'], capture_output=True, text=True, timeout=5)
    properties = dict(line.split('=', 1) for line in shown.stdout.splitlines() if '=' in line)
    cleaned = scope_cleanup_verified(properties, shown.returncode)
    evidence = [x for x in stdout.splitlines() if x.startswith('NEURONS_CGROUP_EVIDENCE=')]
    stages = [x for x in stdout.splitlines() if x.startswith('NEURONS_CGROUP_STAGE=')]
    policy_confirmed = confirmed_policy_evidence(live_policy, unit) if live_policy else False
    full_evidence_valid = False
    if len(evidence) == 1:
        try:
            record = json.loads(evidence[0].split('=', 1)[1])
            full_evidence_valid = (record['live_policy'] == live_policy
                                   and evaluate(record)['status'] == 'CONTROLES_SINTETICOS_APROVADOS')
        except (KeyError, TypeError, ValueError):
            pass
    success = not timed_out and proc.returncode == 0 and full_evidence_valid and cleaned and policy_confirmed
    print(json.dumps({'supervisor_elapsed_s': time.monotonic()-started, 'scope_exit': proc.returncode,
                      'timeout': timed_out, 'cleanup_observed': cleaned, 'scope_properties': properties,
                      'stages_received': len(stages), 'full_evidence_received': len(evidence) == 1,
                      'full_evidence_valid': full_evidence_valid,
                      'oom_policy_confirmed': policy_confirmed, 'live_policy': live_policy,
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
