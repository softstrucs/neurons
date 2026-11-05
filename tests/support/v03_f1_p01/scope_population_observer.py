"""Observador cooperativo: descritores próprios, sem threads ou scopes."""
import os
from pathlib import Path
import subprocess
import tempfile
import time

MAX_SAMPLES = 512
MAX_CAPTURE = 65536
INTERVAL_S = 0.001
LOG_INTERVAL_NS = 25000000
CLIENT_REAP_S = 1.0


class PopulationObserver:
    def __init__(self, path, invocation_id, deadline):
        self.path = Path(path)
        self.invocation_id = invocation_id
        self.deadline = deadline
        self.directory = None
        self.events = None
        self.samples = []
        self.errors = []
        self.inode = None
        self.finished = False
        self.dropped = 0
        self.instance_bound = False
        self.started_ns = None
        self.read_index = 0
        self.gap_epoch = 0
        self.gap_events = []
        self.gap_events_lost = 0
        self.error_count = 0
        self.deduplicated = 0
        self.deduplicated_since_previous = 0

    def gap(self, kind, timestamp):
        self.gap_epoch += 1
        if len(self.gap_events) < 64:
            self.gap_events.append({'kind': kind, 'monotonic_ns': timestamp, 'read_index': self.read_index})
        else:
            self.gap_events_lost += 1

    def error(self, exc):
        self.error_count += 1
        self.gap('read_error', time.monotonic_ns())
        if len(self.errors) < 32:
            self.errors.append({'monotonic_ns': time.monotonic_ns(), 'type': type(exc).__name__, 'reason': str(exc)})

    def start(self):
        self.started_ns = time.monotonic_ns()
        try:
            self.directory = os.open(self.path, os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC)
            self.inode = os.fstat(self.directory).st_ino
            self.events = os.open('cgroup.events', os.O_RDONLY | os.O_CLOEXEC | os.O_NONBLOCK, dir_fd=self.directory)
            self.poll()
        except Exception as exc:
            self.error(exc)
        return self

    def poll(self):
        if self.finished or time.monotonic() >= self.deadline or self.events is None:
            return
        self.read_index += 1
        try:
            os.lseek(self.events, 0, os.SEEK_SET)
            data = os.read(self.events, 16385)
            if len(data) > 16384:
                raise ValueError('cgroup.events excede limite')
            events = dict(x.split() for x in data.decode('utf-8', errors='strict').splitlines())
            if events.get('populated') not in ('0', '1'):
                raise ValueError('populated ausente/inválido')
            try:
                path_inode = self.path.stat().st_ino
            except FileNotFoundError:
                path_inode = None
            if path_inode is not None and path_inode != self.inode:
                raise ValueError('cgroup substituído no caminho observado')
            now = time.monotonic_ns()
            if (self.samples and self.samples[-1]['events'] == events
                    and self.samples[-1]['path_inode'] == path_inode
                    and now - self.samples[-1]['monotonic_ns'] < LOG_INTERVAL_NS):
                self.deduplicated += 1
                self.deduplicated_since_previous += 1
                return
            if len(self.samples) >= MAX_SAMPLES:
                self.dropped += 1
                self.gap('sample_limit', now)
                return
            self.samples.append({'monotonic_ns': now, 'unix_ns': time.time_ns(),
                                 'path': str(self.path), 'inode': self.inode, 'path_inode': path_inode,
                                 'invocation_id': self.invocation_id, 'events': events,
                                 'read_index': self.read_index, 'gap_epoch': self.gap_epoch,
                                 'deduplicated_since_previous': self.deduplicated_since_previous})
            self.deduplicated_since_previous = 0
        except Exception as exc:
            self.error(exc)

    def bind(self, expected_inode):
        self.instance_bound = expected_inode is not None and expected_inode == self.inode
        if not self.instance_bound:
            self.error(ValueError('inode não vinculado à coleta anterior e unidade revalidada'))

    def finish(self):
        if not self.finished:
            self.poll()
            self.finished = True
            for attr in ('events', 'directory'):
                fd = getattr(self, attr)
                if fd is not None:
                    try:
                        os.close(fd)
                    except OSError as exc:
                        self.error(exc)
                    setattr(self, attr, None)
        result = {'started_ns': self.started_ns, 'first_sample_ns': self.samples[0]['monotonic_ns'] if self.samples else None,
                'path': str(self.path), 'inode': self.inode,
                'invocation_id': self.invocation_id, 'instance_bound': self.instance_bound,
                'samples': self.samples, 'errors': self.errors, 'error_count': self.error_count,
                'gap_events': self.gap_events, 'gap_events_lost': self.gap_events_lost,
                'deduplicated_samples': self.deduplicated,
                'transition_instant_observed': False, 'dropped_samples': self.dropped,
                'descriptors_closed': self.directory is None and self.events is None,
                'scope': 'subárvore e janela lidas; não exclui processos escapados'}
        sequence = population_sequences(result, self.invocation_id)
        result['readings_1_then_0'] = [{'from_ns': x['one_ns'], 'to_ns': x['zero_ns'],
               'meaning': 'leituras sem lacuna registrada; instante exato desconhecido'} for x in sequence['pairs']]
        result['continuity'] = sequence['diagnostics']
        return result


def run_observed(command, timeout, observe):
    """Executa só o cliente próprio; drena saída em arquivos temporários limitados."""
    until = time.monotonic() + timeout
    observer_errors = []
    failure = None
    with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
        proc = subprocess.Popen(command, stdin=subprocess.DEVNULL, stdout=stdout, stderr=stderr,
                                env=dict(os.environ, PYTHONDONTWRITEBYTECODE='1'))
        try:
            while True:
                # O callback não pode suprimir ou abandonar o cliente de parada.
                try:
                    observe()
                except Exception as exc:
                    if len(observer_errors) < 32:
                        observer_errors.append({'type': type(exc).__name__, 'reason': str(exc)})
                if stdout.tell() > MAX_CAPTURE or stderr.tell() > MAX_CAPTURE:
                    raise ValueError('saída do cliente excede limite')
                if proc.poll() is not None:
                    break
                remaining = until - time.monotonic()
                if remaining <= 0:
                    raise subprocess.TimeoutExpired(command, timeout)
                time.sleep(min(INTERVAL_S, remaining))
            proc.wait(timeout=CLIENT_REAP_S)
            stdout.seek(0)
            stderr.seek(0)
            result = subprocess.CompletedProcess(command, proc.returncode,
                     stdout.read(MAX_CAPTURE).decode('utf-8', errors='strict'),
                     stderr.read(MAX_CAPTURE).decode('utf-8', errors='strict'))
            result.observer_errors = observer_errors
            return result
        except Exception as exc:
            failure = exc
            exc.observer_errors = observer_errors
            raise
        finally:
            # Somente o cliente Popen criado aqui; nunca PIDs do ensaio ou unidade.
            try:
                if proc.poll() is None:
                    proc.kill()
                proc.wait(timeout=CLIENT_REAP_S)
            except Exception as exc:
                if failure is None:
                    raise
                failure.client_cleanup_errors = [{'type': type(exc).__name__, 'reason': str(exc)}]


def journal_evidence(record, proof):
    """Mensagem por nome não autentica instância nem substitui resultado de job."""
    linked = (record.get('USER_UNIT') == proof.unit and record.get('INVOCATION_ID') == proof.invocation_id)
    return {'instance_linked': linked, 'confirms_stop': False,
            'reason': 'journal é corroborativo; requer origem e job/estado verificáveis'}


def population_sequences(observer, proof_invocation):
    """Adjacência original + ausência de lacuna registrada, nunca continuidade física."""
    positive_ns = lambda value: type(value) is int and value > 0
    binding = (observer.get('instance_bound') is True and isinstance(proof_invocation, str)
               and len(proof_invocation) == 32 and all(x in '0123456789abcdef' for x in proof_invocation)
               and observer.get('invocation_id') == proof_invocation
               and type(observer.get('inode')) is int and observer['inode'] > 0)
    rejected, entries, limitations = [], [], []
    previous_time = None
    ordered = True
    for index, sample in enumerate(observer.get('samples', [])):
        reason = None
        timestamp = sample.get('monotonic_ns') if isinstance(sample, dict) else None
        if not isinstance(sample, dict):
            reason = 'amostra inválida'
        elif not positive_ns(timestamp):
            reason = 'timestamp ausente/inválido'
            ordered = False
        elif previous_time is not None and timestamp <= previous_time:
            reason = 'timestamps fora de ordem ou repetidos'
            ordered = False
        else:
            previous_time = timestamp
            if (not binding or sample.get('invocation_id') != proof_invocation
                    or type(sample.get('inode')) is not int or sample['inode'] != observer['inode']
                    or sample.get('path_inode', observer['inode']) not in (None, observer['inode'])):
                reason = 'inode/InvocationID divergente'
            elif not isinstance(sample.get('events'), dict) or sample['events'].get('populated') not in ('0', '1'):
                reason = 'populated ausente/inválido'
        if reason:
            rejected.append({'index': index, 'reason': reason})
        else:
            entries.append((index, sample))
    raw_gaps = observer.get('gap_events', [])
    errors = observer.get('errors', [])
    gaps = list(raw_gaps) if isinstance(raw_gaps, list) else []
    malformed_metadata = not isinstance(raw_gaps, list) or not isinstance(errors, list)
    if not isinstance(errors, list):
        errors = []
    drops = observer.get('dropped_samples', 0)
    error_count = observer.get('error_count', len(errors))
    if type(error_count) is not int or error_count < len(errors):
        malformed_metadata = True
        error_count = len(errors)
    unknown = malformed_metadata or bool(observer.get('gap_events_lost', 0))
    recorded_errors = sum(isinstance(x, dict) and x.get('kind') == 'read_error' for x in gaps)
    recorded_drops = sum(isinstance(x, dict) and x.get('kind') == 'sample_limit' for x in gaps)
    # Logs antigos: erros com timestamp são posicionáveis; descartes agregados não são.
    if recorded_errors > error_count:
        unknown = True
    if recorded_errors < error_count:
        if recorded_errors == 0 and error_count == len(errors):
            gaps.extend(dict(error, kind='read_error') if isinstance(error, dict) else {} for error in errors)
        else:
            unknown = True
    if type(drops) is not int or drops < 0 or recorded_drops != drops:
        unknown = True
    if any(not isinstance(x, dict) or not positive_ns(x.get('monotonic_ns')) for x in gaps):
        unknown = True
    pairs, interrupted = [], []
    for (left_index, left), (right_index, right) in zip(entries, entries[1:]):
        # Filtragem é usada só para diagnóstico: índice original é obrigatório.
        if left['events']['populated'] != '1' or right['events']['populated'] != '0':
            continue
        reasons = []
        if right_index != left_index + 1:
            reasons.append('AMOSTRA_INTERMEDIARIA_REJEITADA')
        if unknown:
            reasons.append('DESCARTE_COM_POSICAO_DESCONHECIDA')
        if any(left['monotonic_ns'] <= x.get('monotonic_ns', 0) <= right['monotonic_ns']
               for x in gaps if isinstance(x, dict) and positive_ns(x.get('monotonic_ns'))):
            reasons.append('LACUNA_REGISTRADA_NO_INTERVALO')
        epochs = (left.get('gap_epoch'), right.get('gap_epoch'))
        if epochs != (None, None) and (any(type(x) is not int or x < 0 for x in epochs) or epochs[0] != epochs[1]):
            reasons.append('EPOCA_DE_CONTINUIDADE_DIVERGENTE')
        indexes = (left.get('read_index'), right.get('read_index'))
        if indexes != (None, None):
            dedup = right.get('deduplicated_since_previous')
            if (any(type(x) is not int or x <= 0 for x in indexes)
                    or type(dedup) is not int or dedup < 0 or indexes[1] - indexes[0] != dedup + 1):
                reasons.append('LEITURAS_INTERMEDIARIAS_NAO_EXPLICADAS')
        pair = {'one_ns': left['monotonic_ns'], 'zero_ns': right['monotonic_ns'],
                'one_index': left_index, 'zero_index': right_index,
                'interval_ns': right['monotonic_ns'] - left['monotonic_ns'],
                'deduplicated_reads_between': right.get('deduplicated_since_previous'),
                'physical_continuity_claimed': False}
        if reasons:
            interrupted.append(dict(pair, reasons=reasons))
        else:
            pairs.append(pair)
    if not binding:
        limitations.append('vínculo de inode/InvocationID não comprovado')
    if rejected:
        limitations.append('amostras rejeitadas interrompem adjacência original')
    if errors:
        limitations.append('erros de leitura registrados por intervalo quando posicionáveis')
    if drops:
        limitations.append('amostras não armazenadas por limite')
    if unknown:
        limitations.append('posição de descarte/erro desconhecida impede qualificar sequências')
    diagnostics = {'SEQUENCIA_CONTINUA_OBSERVADA': bool(pairs),
                   'SEQUENCIA_INTERROMPIDA': bool(interrupted or rejected or gaps or unknown),
                   'AMOSTRA_INTERMEDIARIA_REJEITADA': any('AMOSTRA_INTERMEDIARIA_REJEITADA' in x['reasons'] for x in interrupted),
                   'DESCARTE_COM_POSICAO_DESCONHECIDA': unknown,
                   'ORDEM_TEMPORAL_INVALIDA': not ordered}
    return {'valid': [x[1] for x in entries], 'pairs': pairs, 'interrupted_pairs': interrupted,
            'rejected': rejected, 'ordered': ordered, 'limitations': limitations, 'diagnostics': diagnostics}


def classifications(report):
    observer = report.get('concurrent_population', {})
    proof_invocation = report.get('invocation_id')
    sequence = population_sequences(observer, proof_invocation)
    valid = sequence['valid']
    pairs = sequence['pairs']
    rejected = sequence['rejected']
    ordered = sequence['ordered']
    limitations = list(sequence['limitations'])
    started = report.get('request_started', {}).get('monotonic_ns')
    finished = report.get('request_finished', {}).get('monotonic_ns')
    request_time_valid = type(started) is int and started > 0 and type(finished) is int and finished >= started
    accepted = report.get('request', {}).get('status') == 'ACEITA'
    if not request_time_valid:
        limitations.append('timestamps da solicitação ausentes ou inválidos')
    before = [x for x in valid if request_time_valid and x['monotonic_ns'] < started]
    after = [x for x in valid if request_time_valid and x['monotonic_ns'] > started]
    zero_before = [x['monotonic_ns'] for x in before if x['events']['populated'] == '0']
    one_before = [x['monotonic_ns'] for x in before if x['events']['populated'] == '1']
    zero_after = [x['monotonic_ns'] for x in after if x['events']['populated'] == '0']
    anchored = [pair for pair in pairs if request_time_valid and pair['one_ns'] < started < pair['zero_ns']]
    emptied_after_request = bool(accepted and request_time_valid and anchored)
    if not accepted:
        limitations.append('stop não aceito; leituras não qualificam esvaziamento pós-solicitação')
    if zero_after and not anchored:
        limitations.append('zero posterior isolado ou sem sequência 1 prévio/0 posterior')
    completion = report.get('completion', {})
    completed = (completion.get('status') == 'CONCLUIDA' and bool(proof_invocation)
                 and completion.get('properties', {}).get('InvocationID') == proof_invocation
                 and completion.get('properties', {}).get('Id') == report.get('unit'))
    known = report.get('known_processes', [])
    absent = bool(known) and all(x.get('classification') == 'AUSENTE' for x in known)
    failed = (report.get('request', {}).get('status') == 'FALHOU'
              or report.get('completion', {}).get('status') == 'UNIDADE_FAILED')
    return {'PARADA_CONFIRMADA': completed,
            'POPULACAO_VAZIA_OBSERVADA': emptied_after_request,
            'AUSENCIA_DOS_PROCESSOS_CONHECIDOS': absent,
            'LIMPEZA_INTEGRAL_INCONCLUSIVA': True,
            'FALHA_DE_PARADA': failed,
            'UNIDADE_REMOVIDA_OBSERVADA': report.get('unit_removed', False),
            'empty_observation_times_ns': zero_after,
            'population_timing': {'request_time_valid': request_time_valid, 'sample_order_valid': ordered,
                'zero_before_stop_ns': zero_before, 'one_before_stop_ns': one_before,
                'zero_after_request_start_ns': zero_after,
                'zero_during_request_ns': [t for t in zero_after if t <= finished],
                'zero_after_request_finished_ns': [t for t in zero_after if t > finished],
                'readings_1_then_0': pairs, 'readings_1_before_0_after': anchored,
                'continuity': sequence['diagnostics'], 'interrupted_pairs': sequence['interrupted_pairs'],
                'exact_transition_instant_observed': False,
                'removed_without_zero': bool(report.get('unit_removed') and not any(x['events']['populated'] == '0' for x in valid)),
                'rejected_samples': rejected, 'limitations': limitations,
                'observer_error_count': len(observer.get('errors', [])),
                'dropped_sample_count': observer.get('dropped_samples', 0),
                'stop_accepted': accepted},
            'limits': 'B exige 1 antes e 0 após início de pedido aceito; não prova causalidade, parada concluída ou instante da transição; A e C são independentes'}
