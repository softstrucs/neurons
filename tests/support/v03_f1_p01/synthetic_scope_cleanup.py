#!/usr/bin/env python3
"""Validação prospectiva de limpeza: somente processos sintéticos e scope próprio."""
import argparse
import json
import os
from pathlib import Path
import secrets
import select
import subprocess
import sys
import tempfile
import time

import cgroup_controls as common
import dv_cgroup_executor as executor


def process_identity(pid):
    stat = Path(f'/proc/{pid}/stat').read_text().rsplit(')', 1)[1].split()
    return {'pid': pid, 'ppid': int(stat[1]), 'state': stat[0], 'ticks': int(stat[19]),
            'cgroup': str(common.cgroup_path(pid))}


def child():
    grand = subprocess.Popen([sys.executable, __file__, '--grandchild'])
    print('CHILD=' + json.dumps({'pid': os.getpid(), 'grandchild': grand.pid}), flush=True)
    time.sleep(10)
    grand.wait(timeout=1)


def coordinator():
    path = common.cgroup_path(os.getpid())
    memory_max = (path / 'memory.max').read_text().strip()
    if memory_max != str(64 * 1024 * 1024):
        raise RuntimeError('limite sintético não confirmado')
    proc = subprocess.Popen([sys.executable, __file__, '--child'], stdout=subprocess.PIPE, text=True)
    try:
        # O filho emite uma única linha; o supervisor global também aplica prazo.
        if not select.select([proc.stdout], [], [], 3)[0]:
            raise TimeoutError('filho sintético não identificado')
        line = proc.stdout.readline()
        data = json.loads(line.removeprefix('CHILD='))
        members = [process_identity(os.getpid()), process_identity(data['pid']), process_identity(data['grandchild'])]
        if any(item['cgroup'] != str(path) for item in members):
            raise RuntimeError('processo fora do scope')
        print('SYNTH_SCOPE_READY=' + json.dumps({'scope_cgroup': str(path), 'memory_max': memory_max,
                                                'members': members}), flush=True)
        os._exit(77)  # falha abrupta deliberada, sem recolher o filho sintético
    except BaseException:
        proc.kill()
        proc.wait(timeout=3)
        raise


def host_test():
    unit = 'neurons-v03-f1-cleanup-' + secrets.token_hex(8) + '.scope'
    command = ['systemd-run', '--user', '--scope', '--quiet', '--unit=' + unit,
               '--property=MemoryMax=67108864', sys.executable, str(Path(__file__).resolve()), '--coordinator']
    started = time.monotonic()
    print(json.dumps({'unit': unit, 'supervisor_pid': os.getpid(), 'command': command}), flush=True)
    with tempfile.TemporaryFile(mode='w+t') as log:
        proc = subprocess.Popen(command, stdout=log, stderr=log)
        record = None
        ownership_confirmed = False
        try:
            until = started + 5
            while time.monotonic() < until:
                lines = os.pread(log.fileno(), 1_048_576, 0).decode('utf-8', errors='replace').splitlines()
                matches = [x for x in lines if x.startswith('SYNTH_SCOPE_READY=')]
                if len(matches) == 1:
                    record = json.loads(matches[0].split('=', 1)[1])
                    break
                if len(matches) > 1:
                    raise RuntimeError('identidade duplicada')
                time.sleep(0.02)
            if record is None:
                raise TimeoutError('identidade ausente')
            group = record['scope_cgroup']
            if Path(group).name != unit or len(record['members']) != 3:
                raise RuntimeError('scope ou membros divergentes')
            shown = subprocess.run(['systemctl', '--user', 'show', unit, '--property=ControlGroup',
                                    '--property=LoadState'], capture_output=True, text=True, timeout=3)
            props = dict(x.split('=', 1) for x in shown.stdout.splitlines() if '=' in x)
            if shown.returncode != 0 or str(executor.ROOT / props.get('ControlGroup', '').lstrip('/')) != group:
                raise RuntimeError('scope não confirmado pelo systemd')
            for expected in record['members'][1:]:
                live = process_identity(expected['pid'])
                if live['ticks'] != expected['ticks'] or live['cgroup'] != group:
                    raise RuntimeError('descendente divergente')
            ownership_confirmed = True
            if proc.wait(timeout=2) != 77:
                raise RuntimeError('falha abrupta não confirmada')
        except (OSError, ValueError, KeyError, IndexError, RuntimeError, TimeoutError, subprocess.TimeoutExpired) as exc:
            print(json.dumps({'stage': 'preflight', 'status': 'INCONCLUSIVO', 'reason': str(exc)}), flush=True)
        finally:
            cleanup = executor.stop_owned_scope(unit, ownership_confirmed)
        survivors, unknown_processes = [], []
        if record:
            for item in record['members']:
                try:
                    live = process_identity(item['pid'])
                    if live['ticks'] == item['ticks']:
                        survivors.append(live)
                except (FileNotFoundError, ProcessLookupError):
                    pass
                except (OSError, ValueError, IndexError) as exc:
                    unknown_processes.append({'pid': item['pid'], 'reason': str(exc)})
        output = os.pread(log.fileno(), 1_048_576, 0).decode('utf-8', errors='replace')
        print(output, end='')
        print(json.dumps({'status': 'INCONCLUSIVO', 'cleanup': cleanup, 'survivors': survivors,
                          'unknown_processes': unknown_processes,
                          'scope_exit': proc.poll(), 'elapsed_s': time.monotonic()-started}), flush=True)
        # Validação exige inspeção humana dos dados; nunca promove automaticamente F ou D/V.
        return 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--run-host-synthetic', action='store_true')
    group.add_argument('--coordinator', action='store_true', help=argparse.SUPPRESS)
    group.add_argument('--child', action='store_true', help=argparse.SUPPRESS)
    group.add_argument('--grandchild', action='store_true', help=argparse.SUPPRESS)
    args = parser.parse_args()
    if args.coordinator:
        coordinator()
    elif args.child:
        child()
    elif args.grandchild:
        time.sleep(10)
    else:
        return host_test()
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
