#!/usr/bin/env python3
"""Executor D/V prospectivo; nenhuma opção admite F ou painel final."""
import argparse
import glob
import hashlib
import json
import os
from pathlib import Path
import secrets
import selectors
import subprocess
import sys
import tempfile
import threading
import time

import cgroup_controls as common

LIMIT = 268435456
DEADLINE = 30.0
MIN_FREE = 3 * 1024 ** 3
TEST_NAMES = ('development', 'validation')
ROOT = Path('/sys/fs/cgroup')


def free_home():
    stats = os.statvfs('/home')
    return stats.f_bavail * stats.f_frsize


def allowed_test(name):
    if name not in TEST_NAMES:
        raise ValueError('somente D/V permitidos')
    return name


def validate_scope(path, unit, pid, props):
    """Aprovação exige unidade ativa, PID e memória, não só texto do marcador."""
    try:
        return (type(pid) is int and pid > 0 and path.name == unit + '.scope'
                and props['LoadState'] == 'loaded' and props['ActiveState'] == 'active'
                and str(ROOT / props['ControlGroup'].lstrip('/')) == str(path)
                and str(common.cgroup_path(pid)) == str(path)
                and (path / 'memory.max').read_text().strip() == str(LIMIT))
    except (OSError, KeyError, TypeError, ValueError):
        return False


def ancestors(path):
    result = {}
    current = path
    while current != ROOT:
        result[str(current)] = (current / 'memory.max').read_text().strip()
        current = current.parent
        if ROOT not in (current, *current.parents):
            raise ValueError('cgroup fora da raiz')
    return result


def binary():
    candidates = [Path(x) for x in glob.glob('target/debug/deps/v03_f1_p01_instruments-*')
                  if Path(x).is_file() and os.access(x, os.X_OK)]
    if len(candidates) != 1:
        raise RuntimeError('binário D/V ausente ou ambíguo; não executado')
    return candidates[0].resolve()


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def live_process(pid):
    """Lê criador e geração do PID enquanto o processo aguarda liberação."""
    data = Path(f'/proc/{pid}/stat').read_text()
    suffix = data.rsplit(')', 1)[1].split()
    return int(suffix[1]), int(suffix[19]), str(common.cgroup_path(pid))


def verify_test_ready(line, run_id, pid, parent_pid, ticks, group):
    marker = line.find('V03TEST_READY|')
    return (marker >= 0 and line[marker:].strip().split('|')
            == ['V03TEST_READY', run_id, str(pid), str(parent_pid), str(ticks), group])


def only_coordinator(pids, coordinator_pid):
    return type(coordinator_pid) is int and coordinator_pid > 0 and pids == [coordinator_pid]


def snapshot(path):
    peak = path / 'memory.peak'
    return {'max': (path / 'memory.max').read_text().strip(),
            'current': int((path / 'memory.current').read_text()),
            'peak': int(peak.read_text()) if peak.exists() else None,
            'events': common.events(path / 'memory.events'),
            'pids': sorted(map(int, (path / 'cgroup.procs').read_text().split()))}


def sample_members(path, stop, result):
    """Observação periódica: não garante captura de filhos muito curtos."""
    while not stop.is_set():
        try:
            pids = sorted(map(int, (path / 'cgroup.procs').read_text().split()))
            result['samples'].append(pids)
            for pid in pids:
                try:
                    if common.cgroup_path(pid) != path:
                        result['inconsistent'].append(pid)
                except (OSError, ValueError):
                    result['unverified'].append(pid)
        except (OSError, ValueError) as exc:
            result['errors'].append(str(exc))
        stop.wait(0.002)


def verify_restore_traces(output, run_id, partition, parent_pid, cgroup):
    """Verifica 18 registros de filho distintos; polling não é referência."""
    if partition not in ('D', 'V'):
        raise ValueError('partição não autorizada')
    expected = {f'{partition}-{rep}-{case}' for rep in range(3) for case in range(1, 7)}
    seen, identities, records = set(), set(), []
    for line in output.splitlines():
        marker = line.find('V03TRACE_RECORD|')
        if marker < 0:
            continue
        fields = line[marker:].split('|')
        if len(fields) != 14:
            raise ValueError('registro incompleto')
        _, run, child_id, role, pid, ppid, ticks, group, begin, entered, end, elapsed, code, sig = fields
        try:
            pid, ppid, ticks, begin, entered, end, elapsed, code, sig = map(
                int, (pid, ppid, ticks, begin, entered, end, elapsed, code, sig))
        except ValueError as exc:
            raise ValueError('registro numérico inválido') from exc
        identity = (pid, ticks)
        if (run != run_id or child_id not in expected or child_id in seen or role != 'restore'
                or pid <= 0 or ppid != parent_pid or ticks <= 0 or identity in identities
                or group != cgroup or not (0 < begin <= entered <= end) or elapsed <= 0
                or code != 0 or sig != 0):
            raise ValueError('identidade, cgroup, tempo ou saída divergente')
        seen.add(child_id)
        identities.add(identity)
        records.append({'child_id': child_id, 'pid': pid, 'parent_pid': ppid,
                        'start_ticks': ticks, 'cgroup': group, 'start_unix_ns': begin,
                        'entry_unix_ns': entered, 'end_unix_ns': end,
                        'elapsed_monotonic_ns': elapsed, 'exit_code': code, 'signal': sig})
    if seen != expected:
        raise ValueError('registros ausentes')
    return records


def worker():
    started = time.monotonic()
    path = common.cgroup_path(os.getpid())
    before = snapshot(path)
    print('DV_READY=' + json.dumps({'pid': os.getpid(), 'path': str(path), 'before': before}), flush=True)
    release = json.loads(sys.stdin.readline())
    if (release.get('release') is not True or release.get('pid') != os.getpid()
            or release.get('path') != str(path) or before['max'] != str(LIMIT)):
        raise RuntimeError('liberação externa ou teto não verificado; D/V não iniciados')
    if free_home() < MIN_FREE:
        raise RuntimeError('espaço insuficiente; D/V não iniciados')
    exe = binary()
    entry = {'coordinator_pid': os.getpid(), 'cgroup': str(path), 'before': before,
             'ancestors': ancestors(path), 'binary': str(exe), 'binary_sha256': sha256(exe),
             'home_free_bytes': free_home(), 'partitions': {}}
    env = dict(os.environ)
    env.pop('NEURONS_V03_F1_DV_CASE', None)
    env.pop('NEURONS_V03_F1_DV_CHECKPOINT', None)
    env['NEURONS_V03_TRACE_CGROUP'] = '/' + str(path.relative_to(ROOT))
    for name in TEST_NAMES:
        allowed_test(name)
        run_id = secrets.token_hex(16)
        env['NEURONS_V03_TRACE_RUN'] = run_id
        if (path / 'memory.max').read_text().strip() != str(LIMIT):
            raise RuntimeError('teto alterado; interrompido')
        remaining = DEADLINE - (time.monotonic() - started)
        if remaining <= 0:
            raise TimeoutError('limite total')
        spawned_ns = time.time_ns()
        spawned_monotonic_ns = time.monotonic_ns()
        proc = subprocess.Popen([str(exe), '--exact', name, '--nocapture'],
                                stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE, text=True, env=env)
        observations = {'samples': [], 'unverified': [], 'inconsistent': [], 'errors': []}
        stop = threading.Event()
        monitor = threading.Thread(target=sample_members, args=(path, stop, observations), daemon=True)
        monitor.start()
        try:
            inside = common.cgroup_path(proc.pid) == path
            if not inside:
                raise RuntimeError('processo fora da contenção')
            prefix = ''
            while True:
                line = proc.stdout.readline()
                if not line:
                    raise RuntimeError('identidade do teste ausente')
                prefix += line
                if 'V03TEST_READY|' in line:
                    break
            ppid, start_ticks, group = live_process(proc.pid)
            if (ppid != os.getpid() or group != env['NEURONS_V03_TRACE_CGROUP']
                    or not verify_test_ready(line, run_id, proc.pid, ppid, start_ticks, group)):
                raise RuntimeError('identidade do teste divergente; nenhuma consulta liberada')
            entered_ns = time.time_ns()
            entered_monotonic_ns = time.monotonic_ns()
            proc.stdin.write(f'V03TEST_GO|{run_id}|{proc.pid}|{start_ticks}\n')
            proc.stdin.flush()
            proc.stdin.close()
            proc.stdin = None
            rest, error = proc.communicate(timeout=max(0.01, DEADLINE - (time.monotonic() - started)))
            output = prefix + rest
            if output.count('V03TEST_READY|') != 1:
                raise RuntimeError('identidade do teste duplicada')
        finally:
            if proc.poll() is None:
                proc.kill()
                proc.wait()
            stop.set()
            monitor.join(timeout=1)
        traces = verify_restore_traces(output, run_id, 'D' if name == 'development' else 'V',
                                       proc.pid, env['NEURONS_V03_TRACE_CGROUP'])
        entry['partitions'][name] = {'pid': proc.pid, 'inside': inside, 'returncode': proc.returncode,
                                     'run_id': run_id, 'restore_traces': traces,
                                     'role': 'test_harness', 'parent_pid': ppid,
                                     'start_ticks': start_ticks, 'cgroup': group,
                                     'start_unix_ns': spawned_ns, 'entry_unix_ns': entered_ns,
                                     'end_unix_ns': time.time_ns(),
                                     'start_monotonic_ns': spawned_monotonic_ns,
                                     'entry_monotonic_ns': entered_monotonic_ns,
                                     'end_monotonic_ns': time.monotonic_ns(),
                                     'exit_code': proc.returncode if proc.returncode >= 0 else None,
                                     'signal': -proc.returncode if proc.returncode < 0 else 0,
                                     'stdout': output, 'stderr': error, 'membership_samples': observations}
        if monitor.is_alive() or observations['errors'] or observations['inconsistent']:
            raise RuntimeError('verificação de processos incompleta; interrompido')
        if proc.returncode != 0:
            raise RuntimeError('D/V terminou com erro; resultado inconclusivo')
    entry['after'] = snapshot(path)
    entry['elapsed_s'] = time.monotonic() - started
    entry['remaining_pids'] = entry['after']['pids']
    if not only_coordinator(entry['remaining_pids'], os.getpid()):
        raise RuntimeError('descendente ativo ou lista de processos inconsistente')
    # A instrumentação é prospectiva: validação real e cobertura além do harness permanecem pendentes.
    entry['descendant_membership'] = 'REGISTROS_LOCAIS_VERIFICADOS_NAO_HOMOLOGADOS'
    entry['status'] = 'INCONCLUSIVO'
    print('DV_EVIDENCE=' + json.dumps(entry, sort_keys=True), flush=True)
    return 1


def supervisor():
    started = time.monotonic()
    if free_home() < MIN_FREE:
        raise RuntimeError('menos de 3 GiB em /home; nenhum scope iniciado')
    unit = 'neurons-v03-f1-dv-' + secrets.token_hex(8)
    command = ['systemd-run', '--user', '--scope', '--quiet', '--unit=' + unit,
               '--property=MemoryMax=' + str(LIMIT), sys.executable,
               str(Path(__file__).resolve()), '--internal-worker']
    print(json.dumps({'scope': unit + '.scope', 'supervisor_pid': os.getpid(),
                      'command': command, 'executor_sha256': sha256(Path(__file__))}), flush=True)
    stderr_file = tempfile.TemporaryFile(mode='w+t')
    proc = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=stderr_file)
    chunks, pending, verified, timed_out = [], b'', False, False
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
                        continue
                    chunks.append(chunk)
                    pending += chunk
                    while b'\n' in pending:
                        line, pending = pending.split(b'\n', 1)
                        if line.startswith(b'DV_READY='):
                            try:
                                if verified:
                                    raise ValueError('marcador duplicado')
                                ready = json.loads(line.split(b'=', 1)[1])
                                shown = subprocess.run(['systemctl', '--user', 'show', unit + '.scope',
                                                        '--property=LoadState', '--property=ActiveState',
                                                        '--property=ControlGroup'], capture_output=True, text=True, timeout=3)
                                props = dict(x.split('=', 1) for x in shown.stdout.splitlines() if '=' in x)
                                path = Path(ready['path'])
                                verified = (shown.returncode == 0 and validate_scope(path, unit, ready['pid'], props)
                                            and ready['before']['max'] == str(LIMIT) and free_home() >= MIN_FREE)
                                if not verified:
                                    raise ValueError('teto, scope ou disco não verificados')
                                proc.stdin.write((json.dumps({'release': True, 'pid': ready['pid'], 'path': str(path)}) + '\n').encode())
                                proc.stdin.flush()
                            except (OSError, ValueError, KeyError, TypeError, subprocess.TimeoutExpired) as exc:
                                print('preflight_inconclusivo=' + str(exc), file=sys.stderr, flush=True)
                                proc.stdin.close()
        proc.wait(timeout=max(0.01, DEADLINE - (time.monotonic() - started)))
    except subprocess.TimeoutExpired:
        timed_out = True
        try:
            subprocess.run(['systemctl', '--user', 'stop', unit + '.scope'], timeout=5, check=False)
        except (OSError, subprocess.TimeoutExpired) as exc:
            print('stop_inconclusivo=' + str(exc), file=sys.stderr)
        finally:
            if proc.poll() is None:
                proc.kill()
        proc.wait(timeout=5)
    finally:
        if proc.stdin and not proc.stdin.closed:
            proc.stdin.close()
    stderr_file.seek(0)
    print(b''.join(chunks).decode('utf-8', errors='replace'), end='')
    print(stderr_file.read(), end='', file=sys.stderr)
    stderr_file.close()
    try:
        shown = subprocess.run(['systemctl', '--user', 'show', unit + '.scope', '--property=ActiveState',
                                '--property=ControlGroup', '--property=Result'], capture_output=True, text=True, timeout=5)
        props = dict(x.split('=', 1) for x in shown.stdout.splitlines() if '=' in x)
        cleanup = common.scope_cleanup_verified(props, shown.returncode)
    except (OSError, subprocess.TimeoutExpired) as exc:
        props = {'query_error': str(exc)}
        cleanup = False
    if not cleanup:
        try:
            stopped = subprocess.run(['systemctl', '--user', 'stop', unit + '.scope'],
                                     capture_output=True, text=True, timeout=5)
            print(json.dumps({'cleanup_stop_exit': stopped.returncode, 'cleanup_stop_error': stopped.stderr}), flush=True)
        except (OSError, subprocess.TimeoutExpired) as exc:
            print('cleanup_stop_inconclusivo=' + str(exc), file=sys.stderr, flush=True)
    print(json.dumps({'status': 'INCONCLUSIVO', 'scope_exit': proc.returncode, 'timeout': timed_out,
                      'scope_verified': verified, 'cleanup_observed': cleanup, 'properties': props,
                      'supervisor_elapsed_s': time.monotonic() - started}), flush=True)
    return 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--execute-dv-with-explicit-authorization', action='store_true')
    group.add_argument('--internal-worker', action='store_true', help=argparse.SUPPRESS)
    args = parser.parse_args()
    if args.internal_worker:
        return worker()
    return supervisor()


if __name__ == '__main__':
    raise SystemExit(main())
