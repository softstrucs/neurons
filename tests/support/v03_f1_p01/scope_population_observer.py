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

    def error(self, exc):
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
                return
            if len(self.samples) >= MAX_SAMPLES:
                self.dropped += 1
                return
            self.samples.append({'monotonic_ns': now, 'unix_ns': time.time_ns(),
                                 'path': str(self.path), 'inode': self.inode, 'path_inode': path_inode,
                                 'invocation_id': self.invocation_id, 'events': events})
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
        transitions = []
        for previous, current in zip(self.samples, self.samples[1:]):
            if previous['events']['populated'] == '1' and current['events']['populated'] == '0':
                transitions.append({'from_ns': previous['monotonic_ns'], 'to_ns': current['monotonic_ns'],
                                    'meaning': 'valores 1 e 0 lidos; instante exato entre leituras desconhecido'})
        return {'started_ns': self.started_ns, 'first_sample_ns': self.samples[0]['monotonic_ns'] if self.samples else None,
                'path': str(self.path), 'inode': self.inode,
                'invocation_id': self.invocation_id, 'instance_bound': self.instance_bound, 'samples': self.samples, 'errors': self.errors,
                'readings_1_then_0': transitions, 'transition_instant_observed': False, 'dropped_samples': self.dropped,
                'descriptors_closed': self.directory is None and self.events is None,
                'scope': 'subárvore e janela lidas; não exclui processos escapados'}


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


def classifications(report):
    observer = report.get('concurrent_population', {})
    proof_invocation = report.get('invocation_id')
    limitations = []
    rejected = []
    valid = []
    started = report.get('request_started', {}).get('monotonic_ns')
    finished = report.get('request_finished', {}).get('monotonic_ns')
    positive_ns = lambda value: type(value) is int and value > 0
    request_time_valid = positive_ns(started) and positive_ns(finished) and finished >= started
    accepted = report.get('request', {}).get('status') == 'ACEITA'
    if not request_time_valid:
        limitations.append('timestamps da solicitação ausentes ou inválidos')
    binding = (observer.get('instance_bound') is True and isinstance(proof_invocation, str)
               and len(proof_invocation) == 32 and all(x in '0123456789abcdef' for x in proof_invocation)
               and observer.get('invocation_id') == proof_invocation
               and type(observer.get('inode')) is int and observer['inode'] > 0)
    if not binding:
        limitations.append('vínculo de inode/InvocationID não comprovado')
    previous_time = None
    ordered = True
    for index, sample in enumerate(observer.get('samples', [])):
        if not isinstance(sample, dict):
            rejected.append({'index': index, 'reason': 'amostra inválida'})
            continue
        timestamp = sample.get('monotonic_ns')
        if not positive_ns(timestamp):
            ordered = False
            rejected.append({'index': index, 'reason': 'timestamp ausente/inválido'})
            continue
        if previous_time is not None and timestamp <= previous_time:
            ordered = False
            rejected.append({'index': index, 'reason': 'timestamps fora de ordem ou repetidos'})
            continue
        previous_time = timestamp
        if (not binding or sample.get('invocation_id') != proof_invocation
                or type(sample.get('inode')) is not int or sample.get('inode') != observer.get('inode')
                or sample.get('path_inode', observer.get('inode')) not in (None, observer.get('inode'))):
            rejected.append({'index': index, 'reason': 'inode/InvocationID divergente'})
            continue
        if not isinstance(sample.get('events'), dict) or sample['events'].get('populated') not in ('0', '1'):
            rejected.append({'index': index, 'reason': 'populated ausente/inválido'})
            continue
        valid.append(sample)
    if not ordered:
        limitations.append('ordem temporal incompleta; esvaziamento pós-solicitação não classificável')
    if observer.get('errors'):
        limitations.append('erros de leitura: ver concurrent_population.errors')
    if observer.get('dropped_samples', 0):
        limitations.append('amostras descartadas pelo limite de volume')
    if rejected:
        limitations.append('amostras rejeitadas na classificação')
    before = [x for x in valid if request_time_valid and x['monotonic_ns'] < started]
    after = [x for x in valid if request_time_valid and x['monotonic_ns'] > started]
    zero_before = [x['monotonic_ns'] for x in before if x['events']['populated'] == '0']
    one_before = [x['monotonic_ns'] for x in before if x['events']['populated'] == '1']
    zero_after = [x['monotonic_ns'] for x in after if x['events']['populated'] == '0']
    pairs = [{'one_ns': one['monotonic_ns'], 'zero_ns': zero['monotonic_ns']}
             for one, zero in zip(valid, valid[1:])
             if one['events']['populated'] == '1' and zero['events']['populated'] == '0']
    # B exige 1 prévio e 0 após INÍCIO do pedido aceito; não afirma causalidade/conclusão.
    anchored = [pair for pair in pairs if request_time_valid and pair['one_ns'] < started < pair['zero_ns']]
    emptied_after_request = bool(accepted and request_time_valid and ordered and anchored)
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
                'exact_transition_instant_observed': False,
                'removed_without_zero': bool(report.get('unit_removed') and not any(x['events']['populated'] == '0' for x in valid)),
                'rejected_samples': rejected, 'limitations': limitations,
                'observer_error_count': len(observer.get('errors', [])),
                'dropped_sample_count': observer.get('dropped_samples', 0),
                'stop_accepted': accepted},
            'limits': 'B exige 1 antes e 0 após início de pedido aceito; não prova causalidade, parada concluída ou instante da transição; A e C são independentes'}
