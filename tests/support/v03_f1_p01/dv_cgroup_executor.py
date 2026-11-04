#!/usr/bin/env python3
"""Executor D/V prospectivo; nenhuma opção admite F ou painel final."""
import argparse
from dataclasses import dataclass
import glob
import hashlib
import json
import os
from pathlib import Path
import re
import secrets
import selectors
import subprocess
import sys
import tempfile
import threading
import time

import cgroup_controls as common
import scope_cleanup_evidence as cleanup_evidence

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


def cgroup_relative(value):
    """Forma canônica do kernel: caminho absoluto dentro da hierarquia cgroup."""
    value = os.fspath(value)
    if not value.startswith('/') or any(x in ('.', '..') for x in value.split('/')):
        raise ValueError('caminho cgroup inválido')
    path = Path(value)
    if path == ROOT or ROOT in path.parents:
        return '/' + str(path.relative_to(ROOT)) if path != ROOT else '/'
    return str(path)


def verify_live_test_ready(line, run_id, pid, parent_pid, expected_group):
    ppid, ticks, group = live_process(pid)
    if (ppid != parent_pid or cgroup_relative(group) != cgroup_relative(expected_group)
            or not verify_test_ready(line, run_id, pid, ppid, ticks, group)):
        raise RuntimeError('identidade ou pertencimento divergente; nenhuma consulta liberada')
    return ppid, ticks, cgroup_relative(group)


def read_limited_line(stream, deadline, max_bytes=65536):
    """Leitura sem threads: prazo inclui linha parcial e escritor ainda aberto."""
    fd = stream.fileno()
    blocking = os.get_blocking(fd)
    data = bytearray()
    os.set_blocking(fd, False)
    try:
        with selectors.DefaultSelector() as selector:
            selector.register(fd, selectors.EVENT_READ)
            while True:
                remaining = deadline - time.monotonic()
                if remaining <= 0 or not selector.select(remaining):
                    raise TimeoutError('deadline aguardando linha completa')
                try:
                    chunk = os.read(fd, 1)
                except BlockingIOError:
                    continue
                if not chunk:
                    raise EOFError('EOF com linha parcial' if data else 'EOF antes da linha')
                data.extend(chunk)
                if len(data) > max_bytes:
                    raise ValueError('linha excede limite de bytes')
                if chunk == b'\n':
                    return data.decode('utf-8', errors='strict')
    finally:
        os.set_blocking(fd, blocking)


def close_channel(stream):
    try:
        if stream is not None and not stream.closed:
            stream.close()
        return {'status': 'CANAL_FECHADO'}
    except Exception as exc:
        return {'status': 'INCONCLUSIVO', 'reason': str(exc), 'error_type': type(exc).__name__}


def finish_owned_scope(unit, proof, stream, observations=None):
    """Falha de canal nunca suprime a tentativa independente de limpeza própria."""
    channel = close_channel(stream)
    try:
        cleanup = (stop_owned_scope(unit, proof) if observations is None
                   else stop_owned_scope(unit, proof, observations=observations))
    except Exception as exc:
        cleanup = {'status': 'INCONCLUSIVO', 'reason': str(exc), 'error_type': type(exc).__name__}
    return {'channel_close': channel, 'cleanup': cleanup}


def verify_test_ready(line, run_id, pid, parent_pid, ticks, group):
    marker = line.find('V03TEST_READY|')
    fields = line[marker:].strip().split('|') if marker >= 0 else []
    try:
        return (len(fields) == 6 and fields[:5] == ['V03TEST_READY', run_id, str(pid), str(parent_pid), str(ticks)]
                and cgroup_relative(fields[5]) == cgroup_relative(group))
    except (TypeError, ValueError):
        return False


def only_coordinator(pids, coordinator_pid):
    return type(coordinator_pid) is int and coordinator_pid > 0 and pids == [coordinator_pid]


def read_test_ready(stream, deadline):
    """Lê bytes sem buffering antecipado, com prazo e limite de saída."""
    data = bytearray()
    with selectors.DefaultSelector() as selector:
        selector.register(stream, selectors.EVENT_READ)
        while time.monotonic() < deadline:
            if not selector.select(max(0, deadline - time.monotonic())):
                raise TimeoutError('timeout aguardando V03TEST_READY')
            chunk = os.read(stream.fileno(), 1)
            if not chunk:
                raise RuntimeError('canal encerrado antes de V03TEST_READY')
            data.extend(chunk)
            if len(data) > 65536:
                raise RuntimeError('saída de identidade excede limite')
            if chunk == b'\n' and b'V03TEST_READY|' in data:
                return data.decode('utf-8', errors='strict')
    raise TimeoutError('timeout aguardando V03TEST_READY')


@dataclass(frozen=True)
class ScopeOwnership:
    unit: str
    supervisor_pid: int
    launcher_pid: int
    launcher_ticks: int
    cgroup: str
    invocation_id: str
    coordinator_pid: int
    coordinator_ticks: int
    coordinator_parent: int


def discover_coordinator(unit, launcher_pid, group):
    """Descobre pelo kernel; nenhum PID vem do marcador do processo testado."""
    path = ROOT / group.lstrip('/')
    script = 'synthetic_scope_cleanup.py' if '-cleanup-' in unit else 'dv_cgroup_executor.py'
    flag = '--coordinator' if '-cleanup-' in unit else '--internal-worker'
    expected = [sys.executable, str(Path(__file__).resolve().with_name(script)), flag]
    candidates = []
    for pid in map(int, (path / 'cgroup.procs').read_text().split()):
        try:
            before = live_process(pid)
            argv = Path(f'/proc/{pid}/cmdline').read_bytes().rstrip(b'\0').split(b'\0')
            after = live_process(pid)
        except (FileNotFoundError, ProcessLookupError):
            continue
        if (before == after and before[1] > 0 and before[2] == str(path)
                and argv == [os.fsencode(x) for x in expected]
                and before[0] == (os.getpid() if pid == launcher_pid else launcher_pid)):
            candidates.append((pid, before[1], before[0]))
    if len(candidates) != 1:
        raise ValueError('coordenador vivo único não comprovado por cgroup, argv e parentesco')
    return candidates[0]


def attest_scope(unit, launcher_pid, query_timeout=3):
    """Prova independente do marcador: PID lançado, /proc e unidade systemd viva."""
    if not re.fullmatch(r'neurons-v03-f1-(?:dv|cleanup)-[0-9a-f]{16}\.scope', unit):
        raise ValueError('nome fora do escopo próprio')
    before_parent, before_ticks, before_group = live_process(launcher_pid)
    shown = subprocess.run(['systemctl', '--user', 'show', unit, '--property=LoadState',
                            '--property=ActiveState', '--property=ControlGroup', '--property=InvocationID'],
                           capture_output=True, text=True, timeout=query_timeout)
    props = dict(x.split('=', 1) for x in shown.stdout.splitlines() if '=' in x)
    group = props.get('ControlGroup', '')
    if not group.startswith('/') or '..' in Path(group).parts or Path(group).name != unit:
        raise ValueError('cgroup da unidade inválido')
    coordinator = discover_coordinator(unit, launcher_pid, group)
    after_parent, after_ticks, after_group = live_process(launcher_pid)
    if live_process(coordinator[0]) != (coordinator[2], coordinator[1], str(ROOT / group.lstrip('/'))):
        raise ValueError('coordenador mudou durante atestação')
    invocation = props.get('InvocationID', '')
    group = props.get('ControlGroup', '')
    if (shown.returncode != 0 or props.get('LoadState') != 'loaded'
            or props.get('ActiveState') != 'active' or not re.fullmatch(r'[0-9a-f]{32}', invocation)
            or invocation == '0' * 32 or not group.startswith('/') or Path(group).name != unit
            or before_parent != os.getpid() or after_parent != os.getpid()
            or before_ticks <= 0 or before_ticks != after_ticks
            or before_group != after_group):
        raise ValueError('propriedade do scope não comprovada')
    return ScopeOwnership(unit, os.getpid(), launcher_pid, before_ticks, group, invocation, *coordinator)


def stop_owned_scope(unit, proof=None, *, observations=None):
    """Não envia sinais por PID nem age sobre unidades fora do namespace próprio."""
    if (not isinstance(proof, ScopeOwnership) or proof.unit != unit or proof.supervisor_pid != os.getpid()
            or not re.fullmatch(r'neurons-v03-f1-(?:dv|cleanup)-[0-9a-f]{16}\.scope', unit)):
        return {'status': 'INCONCLUSIVO', 'reason': 'prova de propriedade ausente ou divergente; stop não emitido'}
    if '-cleanup-' in unit:
        return cleanup_evidence.stop_with_evidence(unit, proof, ROOT, observations or ())
    try:
        prior = subprocess.run(['systemctl', '--user', 'show', unit, '--property=LoadState',
                                '--property=ActiveState', '--property=ControlGroup', '--property=InvocationID'],
                               capture_output=True, text=True, timeout=5)
        current = dict(x.split('=', 1) for x in prior.stdout.splitlines() if '=' in x)
        if (prior.returncode != 0 or current.get('InvocationID') != proof.invocation_id
                or current.get('ControlGroup') != proof.cgroup):
            return {'status': 'INCONCLUSIVO', 'reason': 'unidade mudou, desapareceu ou não verificável; stop não emitido', 'properties': current}
        stopped = subprocess.run(['systemctl', '--user', 'stop', unit], capture_output=True, text=True, timeout=5)
        shown = subprocess.run(['systemctl', '--user', 'show', unit, '--property=LoadState',
                                '--property=ActiveState', '--property=ControlGroup', '--property=Result'],
                               capture_output=True, text=True, timeout=5)
        props = dict(x.split('=', 1) for x in shown.stdout.splitlines() if '=' in x)
        clean = stopped.returncode == 0 and common.scope_cleanup_verified(props, shown.returncode)
        return {'status': 'LIMPEZA_OBSERVADA' if clean else 'INCONCLUSIVO',
                'stop_exit': stopped.returncode, 'stop_error': stopped.stderr,
                'show_exit': shown.returncode, 'properties': props}
    except (OSError, subprocess.TimeoutExpired) as exc:
        return {'status': 'INCONCLUSIVO', 'reason': str(exc)}


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
    release = json.loads(read_limited_line(sys.stdin, started + DEADLINE))
    if (not isinstance(release, dict) or release.get('release') is not True or release.get('pid') != os.getpid()
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
        stderr_file = tempfile.TemporaryFile(mode='w+t')
        proc = subprocess.Popen([str(exe), '--exact', name, '--nocapture'],
                                stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                stderr=stderr_file, text=True, env=env)
        observations = {'samples': [], 'unverified': [], 'inconsistent': [], 'errors': []}
        stop = threading.Event()
        monitor = threading.Thread(target=sample_members, args=(path, stop, observations), daemon=True)
        monitor.start()
        try:
            inside = common.cgroup_path(proc.pid) == path
            if not inside:
                raise RuntimeError('processo fora da contenção')
            prefix = read_test_ready(proc.stdout, started + DEADLINE)
            line = next(x for x in prefix.splitlines() if 'V03TEST_READY|' in x)
            ppid, start_ticks, group = verify_live_test_ready(
                line, run_id, proc.pid, os.getpid(), env['NEURONS_V03_TRACE_CGROUP'])
            entered_ns = time.time_ns()
            entered_monotonic_ns = time.monotonic_ns()
            proc.stdin.write(f'V03TEST_GO|{run_id}|{proc.pid}|{start_ticks}\n')
            proc.stdin.flush()
            proc.stdin.close()
            proc.stdin = None
            rest, _ = proc.communicate(timeout=max(0.01, DEADLINE - (time.monotonic() - started)))
            stderr_file.seek(0)
            error = stderr_file.read()
            output = prefix + rest
            if output.count('V03TEST_READY|') != 1:
                raise RuntimeError('identidade do teste duplicada')
        finally:
            if proc.poll() is None:
                proc.kill()
                proc.wait(timeout=3)
            stop.set()
            monitor.join(timeout=1)
            stderr_file.close()
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
    chunks, pending, verified, timed_out, proof = [], b'', False, False, None
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
                                proof = attest_scope(unit + '.scope', proc.pid)
                                print(json.dumps({'ownership_proof': vars(proof)}), flush=True)
                                shown = subprocess.run(['systemctl', '--user', 'show', unit + '.scope',
                                                        '--property=LoadState', '--property=ActiveState',
                                                        '--property=ControlGroup'], capture_output=True, text=True, timeout=3)
                                props = dict(x.split('=', 1) for x in shown.stdout.splitlines() if '=' in x)
                                path = Path(ready['path'])
                                verified = (ready['pid'] == proof.coordinator_pid and shown.returncode == 0
                                            and validate_scope(path, unit, ready['pid'], props)
                                            and ready['before']['max'] == str(LIMIT) and free_home() >= MIN_FREE)
                                if not verified:
                                    raise ValueError('teto, scope ou disco não verificados')
                                proc.stdin.write((json.dumps({'release': True, 'pid': ready['pid'], 'path': str(path)}) + '\n').encode())
                                proc.stdin.flush()
                            except (OSError, ValueError, KeyError, TypeError, subprocess.TimeoutExpired) as exc:
                                print('preflight_inconclusivo=' + str(exc), file=sys.stderr, flush=True)
                                print(json.dumps({'channel_close': close_channel(proc.stdin)}), flush=True)
        proc.wait(timeout=max(0.01, DEADLINE - (time.monotonic() - started)))
    except subprocess.TimeoutExpired:
        timed_out = True
    except (OSError, ValueError, RuntimeError) as exc:
        print('supervisor_inconclusivo=' + str(exc), file=sys.stderr, flush=True)
    finally:
        finalization = finish_owned_scope(unit + '.scope', proof, proc.stdin)
        print(json.dumps({'finalization': finalization}), flush=True)
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            print('launcher_still_active_inconclusivo', file=sys.stderr, flush=True)
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
