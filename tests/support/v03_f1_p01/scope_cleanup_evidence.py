"""Política e observações conservadoras; importar este módulo não cria unidades."""
import os
from pathlib import Path
import subprocess
import selectors
import re
import time

import scope_population_observer as concurrent

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


def query(unit, timeout=QUERY_S, observe=None):
    command = ['systemctl', '--user', 'show', unit, *['--property=' + x for x in PROPERTIES]]
    result = (subprocess.run(command, capture_output=True, text=True, timeout=timeout) if observe is None
              else concurrent.run_observed(command, timeout, observe))
    return {'time': stamp(), 'exit': result.returncode, 'stderr': result.stderr,
            'client_observation_errors': getattr(result, 'observer_errors', []),
            'properties': dict(x.split('=', 1) for x in result.stdout.splitlines() if '=' in x)}


def run_request(unit, timeout, observe):
    return concurrent.run_observed(['systemctl', '--user', '--no-block', 'stop', unit], timeout, observe)


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


def observe_population(path, deadline):
    """Fronteira de observabilidade: erros comuns e inesperados viram evidência."""
    try:
        if time.monotonic() >= deadline:
            raise TimeoutError('deadline de coleta já expirado')
        sample = collect_population(path, deadline)
        if (not isinstance(sample, dict) or sample.get('status') not in ('COLETA_COMPLETA', 'INDISPONIVEL')
                or type(sample.get('empty')) is not bool):
            raise ValueError('resultado de coleta inválido')
        sample = dict(sample)
        sample.setdefault('identities', [])
        sample.setdefault('errors', [])
        if not isinstance(sample['identities'], list) or not isinstance(sample['errors'], list):
            raise ValueError('estrutura de observação inválida')
        for identity in sample['identities']:
            if (not isinstance(identity, dict) or type(identity.get('pid')) is not int
                    or type(identity.get('ticks')) is not int or identity['pid'] <= 0 or identity['ticks'] <= 0):
                raise ValueError('identidade coletada inválida')
        if sample['status'] == 'COLETA_COMPLETA' and (type(sample.get('inode')) is not int or sample['errors']):
            raise ValueError('coleta declarada completa sem evidência consistente')
        if sample['status'] == 'COLETA_COMPLETA':
            nodes = sample.get('nodes')
            if not isinstance(nodes, list) or not nodes:
                raise ValueError('coleta completa sem população de grupos')
            for node in nodes:
                if (not isinstance(node, dict) or node.get('events', {}).get('populated') not in ('0', '1')
                        or not isinstance(node.get('pids'), list) or not isinstance(node.get('tids'), list)):
                    raise ValueError('população de grupo incompleta')
            empty = all(x['events']['populated'] == '0' and not x['pids'] and not x['tids'] for x in nodes)
            if sample['empty'] != empty:
                raise ValueError('classificação de vazio inconsistente')
        if time.monotonic() > deadline:
            sample.update(status='INDISPONIVEL', empty=False)
            sample['errors'] = [*sample['errors'], 'TimeoutError: orçamento de coleta ultrapassado']
        return sample
    except Exception as exc:
        # Não inclui BaseException: interrupção humana/morte do supervisor não é este ensaio.
        return {'time': stamp(), 'path': str(path), 'status': 'INDISPONIVEL', 'empty': False,
                'identities': [], 'errors': [type(exc).__name__ + ': ' + str(exc)]}


def record_failure(out, stage, exc):
    failure = {'stage': stage, 'error_type': type(exc).__name__, 'reason': str(exc), 'time': stamp()}
    for attr in ('observer_errors', 'client_cleanup_errors'):
        details = getattr(exc, attr, None)
        if isinstance(details, list):
            failure[attr] = details
    out['errors'].append(failure)
    out.setdefault('reason', stage + ': ' + failure['error_type'] + ': ' + failure['reason'])


def append_observation(out, path, deadline, stage):
    sample = observe_population(path, deadline)
    out['observations'].append(sample)
    if sample['status'] != 'COLETA_COMPLETA':
        record_failure(out, stage, RuntimeError('coleta incompleta: ' + '; '.join(map(str, sample['errors']))))
    return sample


def classify_cleanup(out):
    final = out['observations'][-1]
    complete = all(x.get('status') == 'COLETA_COMPLETA' for x in out['observations'])
    inodes = {x.get('inode') for x in out['observations']}
    complete = complete and len(inodes) == 1 and None not in inodes
    empty_after = final.get('status') == 'COLETA_COMPLETA' and final.get('empty')
    no_survivors = all(x['classification'] in ('AUSENTE', 'PID_REUTILIZADO') for x in out['known_processes'])
    out['observation_status'] = 'COMPLETA' if complete else 'INCOMPLETA'
    completion = out['completion']
    request = out['request']
    if (not out['errors'] and complete and empty_after and no_survivors
            and completion.get('state') == 'inactive' and completion['properties'].get('ControlGroup') == ''
            and request.get('exit') == 0 and not request.get('timeout')):
        out['status'] = 'VAZIO_DO_CGROUP_OBSERVADO'
        out['cleanup_observation']['status'] = 'VAZIO_DO_CGROUP_OBSERVADO'


def stop_with_evidence(unit, proof, root, observations=()):
    """Falha de observação nunca autoriza nem impede stop da unidade revalidada."""
    try:
        valid = (proof.unit == unit and proof.supervisor_pid == os.getpid()
                 and re.fullmatch(r'neurons-v03-f1-cleanup-[0-9a-f]{16}\.scope', unit)
                 and proof.cgroup.startswith('/') and '..' not in Path(proof.cgroup).parts
                 and Path(proof.cgroup).name == unit)
    except Exception:
        valid = False
    if not valid:
        return {'status': 'INCONCLUSIVO', 'reason': 'propriedade inválida; stop não emitido',
                'request': {'status': 'NAO_SOLICITADO'}}
    started = time.monotonic()
    out = {'status': 'INCONCLUSIVO', 'unit': unit, 'invocation_id': proof.invocation_id,
           'time': stamp(), 'observations': [], 'queries': [], 'errors': [],
           'request': {'status': 'NAO_SOLICITADO'}, 'completion': {'status': 'NAO_COMPROVADA'},
           'cleanup_observation': {'status': 'INCONCLUSIVO'}, 'observation_status': 'INCOMPLETA',
           'budgets_s': {'before': FINAL_S, 'query': QUERY_S, 'request': REQUEST_S,
                         'completion': COMPLETION_S, 'final': FINAL_S}}
    path = root / proof.cgroup.lstrip('/')
    monitor = concurrent.PopulationObserver(path, proof.invocation_id, time.monotonic() + QUERY_S + REQUEST_S + COMPLETION_S + FINAL_S)
    # Observações anteriores são auxiliares; nem sua estrutura pode suprimir a parada.
    try:
        out['observations'] = list(observations)
        for index, sample in enumerate(out['observations']):
            if not isinstance(sample, dict) or sample.get('status') != 'COLETA_COMPLETA':
                record_failure(out, 'population_previous_' + str(index),
                               RuntimeError('evidência anterior incompleta: ' + str(sample)))
    except Exception as exc:
        record_failure(out, 'observations_import', exc)
    append_observation(out, path, time.monotonic() + FINAL_S, 'population_before_stop')
    try:
        # Qualquer falha de autenticação/revalidação é fechada: sem stop.
        monitor.start()
        prior = query(unit, observe=monitor.poll)
        out['queries'].append(prior)
        if not same_unit(prior, proof, active=True) or not valid_policy(prior['properties']):
            record_failure(out, 'revalidation', ValueError('identidade/política não revalidada; stop não emitido'))
            return out
        monitor.bind(out['observations'][-1].get('inode'))
        # Consulta e stop não são atômicos; a corrida residual continua documentada.
        out['request_started'] = stamp()
        try:
            request = run_request(unit, REQUEST_S, monitor.poll)
            out['request'] = {'status': 'ACEITA' if request.returncode == 0 else 'FALHOU',
                              'exit': request.returncode, 'stderr': request.stderr, 'timeout': False,
                              'client_observation_errors': (request.observer_errors if isinstance(getattr(request, 'observer_errors', None), list) else [])}
            if request.returncode != 0:
                record_failure(out, 'stop_request', OSError(request.stderr or 'stop recusado'))
        except subprocess.TimeoutExpired as exc:
            out['request'] = {'status': 'TIMEOUT', 'timeout': True,
                              'reason': 'gerenciador pode continuar a parada'}
            record_failure(out, 'stop_request', exc)
        except Exception as exc:
            out['request'] = {'status': 'FALHOU', 'error_type': type(exc).__name__, 'reason': str(exc)}
            record_failure(out, 'stop_request', exc)
        out['request_finished'] = stamp()
        until = time.monotonic() + COMPLETION_S
        while time.monotonic() < until:
            monitor.poll()
            shown = query(unit, min(QUERY_S, max(0.001, until - time.monotonic())), observe=monitor.poll)
            out['queries'].append(shown)
            append_observation(out, path, min(until, time.monotonic() + 0.25), 'population_during_stop')
            p = shown['properties']
            if not same_unit(shown, proof):
                if p.get('LoadState') == 'not-found' and p.get('ActiveState') == 'inactive':
                    out['unit_removed'] = True
                record_failure(out, 'completion_identity', ValueError('unidade desapareceu, mudou ou não verificável'))
                break
            if p.get('ActiveState') in ('inactive', 'failed'):
                out['completion'] = {'status': 'CONCLUIDA' if p['ActiveState'] == 'inactive' else 'UNIDADE_FAILED',
                                     'time': stamp(), 'state': p['ActiveState'], 'properties': p}
                break
            time.sleep(min(0.05, max(0, until - time.monotonic())))
        else:
            record_failure(out, 'completion_wait', TimeoutError('deadline de conclusão; parada não comprovada'))
    except Exception as exc:
        record_failure(out, 'revalidation' if out['request']['status'] == 'NAO_SOLICITADO' else 'completion_query', exc)
    finally:
        try:
            out['concurrent_population'] = monitor.finish()
        except Exception as exc:
            record_failure(out, 'concurrent_population_final', exc)
            out['concurrent_population'] = {'samples': [], 'errors': [str(exc)], 'instance_bound': False}
        finally:
            # Última defesa para os únicos descritores do monitor, sem sinais.
            for attr in ('events', 'directory'):
                fd = getattr(monitor, attr, None)
                if fd is not None:
                    try:
                        os.close(fd)
                    except OSError as exc:
                        record_failure(out, 'monitor_descriptor_close', exc)
                    setattr(monitor, attr, None)
        monitoring = out['concurrent_population']
        request_ns = out.get('request_started', {}).get('monotonic_ns')
        monitoring['started_before_stop'] = bool(request_ns and monitoring.get('started_ns')
                                                  and monitoring['started_ns'] < request_ns)
        monitoring['first_sample_before_stop'] = bool(request_ns and monitoring.get('first_sample_ns')
                                                       and monitoring['first_sample_ns'] < request_ns)
        until = time.monotonic() + FINAL_S
        append_observation(out, path, until, 'population_final')
        try:
            out['known_processes'] = inspect_known(out['observations'], until)
        except Exception as exc:
            record_failure(out, 'known_processes_final', exc)
            out['known_processes'] = [{'classification': 'NAO_IDENTIFICAVEL', 'reason': str(exc)}]
        try:
            classify_cleanup(out)
        except Exception as exc:
            record_failure(out, 'cleanup_classification', exc)
        try:
            out['classifications'] = concurrent.classifications(out)
        except Exception as exc:
            record_failure(out, 'independent_classifications', exc)
            out['classifications'] = {'LIMPEZA_INTEGRAL_INCONCLUSIVA': True}
        out['manager_evidence'] = {'source': 'state_queries', 'job_tracking': 'NAO_IMPLEMENTADO',
                                   'journal_alone_confirms_stop': False}
        out['scope_of_claim'] = 'observação local; escape ou descendentes nunca observados não são excluídos'
        out['end_time'] = stamp()
        out['elapsed_s'] = time.monotonic() - started
    return out
