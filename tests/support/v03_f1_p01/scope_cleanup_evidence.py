"""Política e observações conservadoras; importar este módulo não cria unidades."""
import os
from pathlib import Path
import subprocess
import selectors
import re
import time

MEMORY_MAX = 67108864
QUERY_S = 1.0
REQUEST_S = 1.0
COMPLETION_S = 5.0
FINAL_S = 2.0
MAX_NODES = 64
MAX_IDS = 512
MAX_KNOWN = 4096
POLICY = {'KillMode': 'control-group', 'KillSignal': '15', 'FinalKillSignal': '9',
          'SendSIGKILL': 'yes', 'TimeoutStopUSec': '2s', 'MemoryMax': str(MEMORY_MAX)}
PROPERTIES = ('Id', 'LoadState', 'ActiveState', 'SubState', 'ControlGroup', 'InvocationID',
              'Result', *POLICY, 'OOMPolicy')


def launch_properties():
    # TimeoutStopSec é convertido pelo cliente v252 para TimeoutStopUSec no D-Bus.
    return ['--property=' + x for x in ('MemoryMax=67108864', 'KillMode=control-group',
            'KillSignal=SIGTERM', 'FinalKillSignal=SIGKILL', 'SendSIGKILL=yes', 'TimeoutStopSec=2s')]


def send_release(stream, payload, deadline):
    if len(payload) > 4096:
        raise ValueError('liberação excede orçamento')
    fd = stream.fileno()
    blocking = os.get_blocking(fd)
    os.set_blocking(fd, False)
    try:
        with selectors.DefaultSelector() as selector:
            selector.register(fd, selectors.EVENT_WRITE)
            while payload:
                remaining = deadline - time.monotonic()
                if remaining <= 0 or not selector.select(remaining):
                    raise TimeoutError('timeout ao liberar coordenador')
                try:
                    sent = os.write(fd, payload)
                except BlockingIOError:
                    continue
                if sent <= 0:
                    raise OSError('canal não aceitou liberação')
                payload = payload[sent:]
    finally:
        os.set_blocking(fd, blocking)


def stamp():
    return {'monotonic_ns': time.monotonic_ns(), 'unix_ns': time.time_ns()}


def query(unit, timeout=QUERY_S):
    result = subprocess.run(['systemctl', '--user', 'show', unit,
                             *['--property=' + x for x in PROPERTIES]],
                            capture_output=True, text=True, timeout=timeout)
    return {'time': stamp(), 'exit': result.returncode, 'stderr': result.stderr,
            'properties': dict(x.split('=', 1) for x in result.stdout.splitlines() if '=' in x)}


def same_unit(shown, proof, active=False):
    p = shown['properties']
    return (shown['exit'] == 0 and p.get('Id') == proof.unit
            and p.get('InvocationID') == proof.invocation_id
            and (not active or (p.get('LoadState') == 'loaded' and p.get('ActiveState') == 'active'
                                and p.get('ControlGroup') == proof.cgroup)))


def valid_policy(properties):
    return all(properties.get(k) == v for k, v in POLICY.items())


def process_identity(pid):
    stat = Path(f'/proc/{pid}/stat').read_text().rsplit(')', 1)[1].split()
    cgroup = next(x[3:] for x in Path(f'/proc/{pid}/cgroup').read_text().splitlines() if x.startswith('0::'))
    return {'pid': pid, 'ppid': int(stat[1]), 'ticks': int(stat[19]), 'state': stat[0], 'cgroup': cgroup}


def verify_preflight(unit, proof, root):
    shown = query(unit)
    path = root / proof.cgroup.lstrip('/')
    supervisor = process_identity(os.getpid())
    if (not same_unit(shown, proof, active=True) or not valid_policy(shown['properties'])
            or (path / 'memory.max').read_text().strip() != str(MEMORY_MAX)
            or supervisor['cgroup'] == proof.cgroup or supervisor['cgroup'].startswith(proof.cgroup + '/')):
        raise ValueError('política, limite ou exclusão do supervisor não comprovados; trabalho recusado')
    return {'query': shown, 'supervisor': supervisor, 'time': stamp(),
            'oom_policy': shown['properties'].get('OOMPolicy', 'INDISPONIVEL_NAO_REQUERIDA')}


def read_small(path):
    with path.open() as stream:
        data = stream.read(16385)
    if len(data) > 16384:
        raise ValueError('arquivo de população excede limite')
    return data


def subgroups(path):
    result = []
    for item in path.iterdir():
        if item.is_dir():
            result.append(item)
            if len(result) > MAX_NODES:
                raise ValueError('limite de subgrupos excedido')
    return sorted(result)


def collect_population(path, deadline):
    """Varredura limitada e não atômica; qualquer lacuna impede prova de vazio."""
    out = {'time': stamp(), 'path': str(path), 'nodes': [], 'identities': [], 'errors': [],
           'status': 'INDISPONIVEL', 'empty': False}
    try:
        inode = path.stat().st_ino
        out['inode'] = inode
        pending = [path]
        ids = set()
        while pending:
            if time.monotonic() >= deadline or len(out['nodes']) >= MAX_NODES:
                raise ValueError('orçamento de coleta excedido')
            node = pending.pop()
            if node.is_symlink():
                raise ValueError('subgrupo simbólico recusado')
            events = dict(x.split() for x in read_small(node / 'cgroup.events').splitlines())
            pids = sorted(set(map(int, read_small(node / 'cgroup.procs').split())))
            tids = sorted(set(map(int, read_small(node / 'cgroup.threads').split())))
            if events.get('populated') not in ('0', '1') or any(x <= 0 for x in pids + tids):
                raise ValueError('população inválida')
            ids.update(pids + tids)
            if len(ids) > MAX_IDS:
                raise ValueError('limite de identidades excedido')
            children = subgroups(node)
            pending.extend(children)
            out['nodes'].append({'path': str(node), 'events': events, 'pids': pids, 'tids': tids,
                                 'children': [str(x) for x in children]})
        for pid in sorted(ids):
            if time.monotonic() >= deadline:
                raise ValueError('deadline de identidades')
            before = process_identity(pid)
            after = process_identity(pid)
            if any(before[k] != after[k] for k in ('pid', 'ppid', 'ticks', 'cgroup')) or (before['cgroup'] != '/' + str(path.relative_to(CGROOT))
                    and not before['cgroup'].startswith('/' + str(path.relative_to(CGROOT)) + '/')):
                raise ValueError('identidade instável ou fora do scope')
            out['identities'].append(after)
        # Releitura captura mudanças observáveis durante a varredura, sem alegar atomicidade.
        for node in out['nodes']:
            if time.monotonic() >= deadline:
                raise ValueError('deadline de releitura')
            directory = Path(node['path'])
            events = dict(x.split() for x in read_small(directory / 'cgroup.events').splitlines())
            if (events != node['events']
                    or sorted(set(map(int, read_small(directory / 'cgroup.procs').split()))) != node['pids']
                    or sorted(set(map(int, read_small(directory / 'cgroup.threads').split()))) != node['tids']
                    or [str(x) for x in subgroups(directory)] != node['children']):
                raise ValueError('população mudou durante coleta')
        if path.stat().st_ino != inode:
            raise ValueError('cgroup substituído durante coleta')
        out['status'] = 'COLETA_COMPLETA'
        out['empty'] = bool(out['nodes']) and all(x['events']['populated'] == '0' and not x['pids'] and not x['tids'] for x in out['nodes'])
    except (OSError, ValueError, IndexError, StopIteration) as exc:
        out['errors'].append(type(exc).__name__ + ': ' + str(exc))
    out['end_time'] = stamp()
    return out


CGROOT = Path('/sys/fs/cgroup')


def inspect_known(observations, deadline):
    identities = {}
    overflow = False
    for obs in observations:
        for item in obs.get('identities', []):
            key = (item['pid'], item['ticks'])
            if key not in identities and len(identities) >= MAX_KNOWN:
                overflow = True
                continue
            identities[key] = item
    result = []
    for (pid, ticks), previous in sorted(identities.items()):
        item = {'pid': pid, 'ticks': ticks, 'time': stamp()}
        try:
            if time.monotonic() >= deadline:
                raise ValueError('deadline de verificação final')
            current = process_identity(pid)
            item['current'] = current
            item['classification'] = ('PID_REUTILIZADO' if current['ticks'] != ticks else
                                      'ZUMBI' if current['state'] == 'Z' else 'SOBREVIVENTE')
        except (FileNotFoundError, ProcessLookupError):
            item['classification'] = 'AUSENTE'
        except (OSError, ValueError, IndexError, StopIteration) as exc:
            item.update(classification='NAO_IDENTIFICAVEL', reason=str(exc))
        result.append(item)
    if overflow:
        result.append({'classification': 'NAO_IDENTIFICAVEL', 'reason': 'limite de identidades acumuladas excedido'})
    return result


def stop_with_evidence(unit, proof, root, observations=()):
    """Chamado somente após validação de propriedade pelo executor."""
    if (proof.unit != unit or proof.supervisor_pid != os.getpid()
            or not re.fullmatch(r'neurons-v03-f1-cleanup-[0-9a-f]{16}\.scope', unit)
            or not proof.cgroup.startswith('/') or '..' in Path(proof.cgroup).parts
            or Path(proof.cgroup).name != unit):
        return {'status': 'INCONCLUSIVO', 'reason': 'propriedade inválida; stop não emitido'}
    started = time.monotonic()
    out = {'status': 'INCONCLUSIVO', 'unit': unit, 'invocation_id': proof.invocation_id,
           'time': stamp(), 'observations': list(observations), 'queries': [],
           'budgets_s': {'query': QUERY_S, 'request': REQUEST_S, 'completion': COMPLETION_S, 'final': FINAL_S}}
    path = root / proof.cgroup.lstrip('/')
    out['observations'].append(collect_population(path, time.monotonic() + FINAL_S))
    try:
        prior = query(unit)
        out['queries'].append(prior)
        if not same_unit(prior, proof, active=True) or not valid_policy(prior['properties']):
            out['reason'] = 'identidade/política não revalidada; stop não emitido'
            return out
        # Consulta e stop não são atômicos: nunca alegar proteção contra troca entre chamadas.
        out['request_started'] = stamp()
        try:
            request = subprocess.run(['systemctl', '--user', '--no-block', 'stop', unit],
                                     capture_output=True, text=True, timeout=REQUEST_S)
            out['request'] = {'exit': request.returncode, 'stderr': request.stderr, 'timeout': False}
        except subprocess.TimeoutExpired:
            out['request'] = {'timeout': True, 'reason': 'gerenciador pode continuar a parada'}
        out['request_finished'] = stamp()
        until = time.monotonic() + COMPLETION_S
        while time.monotonic() < until:
            shown = query(unit, min(QUERY_S, max(0.001, until - time.monotonic())))
            out['queries'].append(shown)
            sample = collect_population(path, min(until, time.monotonic() + 0.25))
            out['observations'].append(sample)
            p = shown['properties']
            if not same_unit(shown, proof):
                out['reason'] = 'unidade desapareceu, mudou ou não verificável'
                break
            if p.get('ActiveState') in ('inactive', 'failed'):
                out['completion'] = {'time': stamp(), 'state': p['ActiveState'], 'properties': p}
                break
            time.sleep(min(0.05, max(0, until - time.monotonic())))
        else:
            out['reason'] = 'deadline de conclusão; parada não comprovada'
    except (OSError, ValueError, subprocess.TimeoutExpired) as exc:
        out['reason'] = type(exc).__name__ + ': ' + str(exc)
    finally:
        until = time.monotonic() + FINAL_S
        out['observations'].append(collect_population(path, until))
        out['known_processes'] = inspect_known(out['observations'], until)
        # Só afirma vazio observado com coletas completas, mesmo inode e final vazio.
        # Remoção rápida pelo systemd mantém INCONCLUSIVO; nunca inventar populated=0.
        final = out['observations'][-1]
        complete_before = all(x.get('status') == 'COLETA_COMPLETA' for x in out['observations'])
        inodes = {x.get('inode') for x in out['observations']}
        complete_before = complete_before and len(inodes) == 1 and None not in inodes
        empty_after = final.get('status') == 'COLETA_COMPLETA' and final.get('empty')
        no_survivors = all(x['classification'] in ('AUSENTE', 'PID_REUTILIZADO') for x in out['known_processes'])
        completion = out.get('completion', {})
        request = out.get('request', {})
        if (complete_before and empty_after and no_survivors and completion.get('state') == 'inactive'
                and completion['properties'].get('ControlGroup') == ''
                and request.get('exit') == 0 and not request.get('timeout')):
            out['status'] = 'VAZIO_DO_CGROUP_OBSERVADO'
        out['scope_of_claim'] = 'observação local; escape ou descendentes nunca observados não são excluídos'
        out['end_time'] = stamp()
        out['elapsed_s'] = time.monotonic() - started
    return out
