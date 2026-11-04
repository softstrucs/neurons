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
    # Trabalho sintético só começa após liberação independente do supervisor.
    if not select.select([sys.stdin], [], [], 4)[0]:
        raise TimeoutError('liberação do supervisor ausente')
    release = json.loads(sys.stdin.readline())
    identity = process_identity(os.getpid())
    if (release.get('release') is not True or release.get('pid') != identity['pid']
            or release.get('ticks') != identity['ticks'] or release.get('cgroup') != str(path)
            or release.get('unit') != path.name):
        raise RuntimeError('liberação do supervisor inválida; trabalho não iniciado')
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


def validate_marker(record, proof):
    """Marcador pode liberar a avaliação, nunca cria a prova de propriedade."""
    try:
        members = record['members']
        group = str(executor.ROOT / proof.cgroup.lstrip('/'))
        return (record['scope_cgroup'] == group and record['memory_max'] == str(64 * 1024 * 1024)
                and len(members) == 3 and len({x['pid'] for x in members}) == 3
                and all(type(x['pid']) is int and x['pid'] > 0 and type(x['ticks']) is int
                        and x['ticks'] > 0 and x['cgroup'] == group for x in members)
                and members[0]['pid'] == proof.launcher_pid and members[0]['ticks'] == proof.launcher_ticks
                and members[0]['ppid'] == proof.supervisor_pid
                and members[1]['ppid'] == members[0]['pid'] and members[2]['ppid'] == members[1]['pid'])
    except (KeyError, TypeError, ValueError):
        return False


def classify_pid(expected, live):
    if expected['pid'] != live['pid']:
        raise ValueError('PID de consulta divergente')
    return 'original' if expected['ticks'] == live['ticks'] else 'pid_reutilizado'


def host_test():
    unit = 'neurons-v03-f1-cleanup-' + secrets.token_hex(8) + '.scope'
    command = ['systemd-run', '--user', '--scope', '--quiet', '--unit=' + unit,
               '--property=MemoryMax=67108864', sys.executable, str(Path(__file__).resolve()), '--coordinator']
    started = time.monotonic()
    print(json.dumps({'unit': unit, 'supervisor_pid': os.getpid(), 'command': command}), flush=True)
    with tempfile.TemporaryFile(mode='w+t') as log:
        proc = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=log, stderr=log)
        record = None
        proof = None
        try:
            # O supervisor usa seu próprio PID lançado, /proc e systemd, nunca o marcador filho.
            until = started + 4
            while time.monotonic() < until:
                try:
                    proof = executor.attest_scope(unit, proc.pid)
                    break
                except (OSError, ValueError, subprocess.TimeoutExpired):
                    if proc.poll() is not None:
                        break
                    time.sleep(0.05)
            if proof is None:
                raise RuntimeError('propriedade do scope não confirmada; trabalho não liberado')
            release = {'release': True, 'pid': proof.launcher_pid, 'ticks': proof.launcher_ticks,
                       'unit': proof.unit, 'cgroup': str(executor.ROOT / proof.cgroup.lstrip('/'))}
            proc.stdin.write((json.dumps(release) + '\n').encode())
            proc.stdin.flush()
            proc.stdin.close()
            print(json.dumps({'ownership_proof': vars(proof), 'work_release': True}), flush=True)
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
            if not validate_marker(record, proof):
                raise RuntimeError('scope ou membros divergentes')
            for expected in record['members'][1:]:
                live = process_identity(expected['pid'])
                if live['ticks'] != expected['ticks'] or live['cgroup'] != record['scope_cgroup']:
                    raise RuntimeError('descendente divergente')
            if proc.wait(timeout=2) != 77:
                raise RuntimeError('falha abrupta não confirmada')
        except (OSError, ValueError, KeyError, IndexError, RuntimeError, TimeoutError, subprocess.TimeoutExpired) as exc:
            print(json.dumps({'stage': 'preflight', 'status': 'INCONCLUSIVO', 'reason': str(exc)}), flush=True)
        finally:
            if proc.stdin and not proc.stdin.closed:
                proc.stdin.close()
            cleanup = executor.stop_owned_scope(unit, proof)
        survivors, unknown_processes, reused_pids = [], [], []
        if isinstance(record, dict) and isinstance(record.get('members'), list):
            for item in record['members']:
                try:
                    if not isinstance(item, dict) or type(item.get('pid')) is not int or type(item.get('ticks')) is not int:
                        unknown_processes.append({'reason': 'identidade de sobrevivente inválida'})
                        continue
                    live = process_identity(item['pid'])
                    if classify_pid(item, live) == 'original':
                        survivors.append(live)
                    else:
                        reused_pids.append({'pid': item['pid'], 'old_ticks': item['ticks'],
                                            'new_ticks': live['ticks']})
                except (FileNotFoundError, ProcessLookupError):
                    pass
                except (OSError, ValueError, IndexError) as exc:
                    unknown_processes.append({'pid': item['pid'], 'reason': str(exc)})
        elif record is not None:
            unknown_processes.append({'reason': 'marcador incompleto ou inválido'})
        else:
            unknown_processes.append({'reason': 'marcador ausente; descendentes não enumerados'})
        output = os.pread(log.fileno(), 1_048_576, 0).decode('utf-8', errors='replace')
        print(output, end='')
        print(json.dumps({'status': 'INCONCLUSIVO', 'cleanup': cleanup, 'survivors': survivors,
                          'unknown_processes': unknown_processes,
                          'reused_pids': reused_pids,
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
