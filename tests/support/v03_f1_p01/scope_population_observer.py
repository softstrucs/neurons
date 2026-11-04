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
    samples = [x for x in observer.get('samples', []) if x.get('invocation_id') == proof_invocation
               and x.get('inode') == observer.get('inode')]
    empty = [x for x in samples if x.get('events', {}).get('populated') == '0'] if observer.get('instance_bound') else []
    completion = report.get('completion', {})
    completed = (completion.get('status') == 'CONCLUIDA' and bool(proof_invocation)
                 and completion.get('properties', {}).get('InvocationID') == proof_invocation
                 and completion.get('properties', {}).get('Id') == report.get('unit'))
    known = report.get('known_processes', [])
    absent = bool(known) and all(x.get('classification') == 'AUSENTE' for x in known)
    failed = (report.get('request', {}).get('status') == 'FALHOU'
              or report.get('completion', {}).get('status') == 'UNIDADE_FAILED')
    return {'PARADA_CONFIRMADA': completed,
            'POPULACAO_VAZIA_OBSERVADA': bool(empty),
            'AUSENCIA_DOS_PROCESSOS_CONHECIDOS': absent,
            'LIMPEZA_INTEGRAL_INCONCLUSIVA': True,
            'FALHA_DE_PARADA': failed,
            'UNIDADE_REMOVIDA_OBSERVADA': report.get('unit_removed', False),
            'empty_observation_times_ns': [x['monotonic_ns'] for x in empty],
            'limits': 'A, B e C são independentes; B cobre apenas a subárvore e instantes lidos'}
