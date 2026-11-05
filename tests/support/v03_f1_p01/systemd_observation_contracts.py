"""C2-B: contratos e avaliação pura de fixtures. Sem relógio, filesystem ou D-Bus.

Nenhuma entrada desta etapa pode produzir autenticação/emissão nativa comprovada.
Estados reservados descrevem uma interface futura, não capacidades existentes.
"""
from dataclasses import asdict, dataclass
from enum import Enum
import re

import systemd_job_evidence as c1
from systemd_job_transport import Manager, ManagerTarget, SERVICE, MAX_EVENTS, MAX_ERRORS, MAX_BUFFER_BYTES


class Verification(str, Enum):
    NAO_VERIFICADO = 'NAO_VERIFICADO'
    VALIDADO_SINTATICAMENTE = 'VALIDADO_SINTATICAMENTE'
    OBSERVADO_SIMULADAMENTE = 'OBSERVADO_SIMULADAMENTE'
    AUTENTICADO_OPERACIONALMENTE = 'AUTENTICADO_OPERACIONALMENTE'
    INVALIDADO = 'INVALIDADO'


class Provenance(str, Enum):
    SIMULADA = 'SIMULADA'
    DECLARADA = 'DECLARADA'
    OBSERVADA_NATIVAMENTE = 'OBSERVADA_NATIVAMENTE'
    AUTENTICADA = 'AUTENTICADA'
    INCONCLUSIVA = 'INCONCLUSIVA'
    INVALIDADA = 'INVALIDADA'


class EmissionState(str, Enum):
    EMISSAO_NAO_COMPROVADA = 'EMISSAO_NAO_COMPROVADA'
    EMISSAO_DECLARADA = 'EMISSAO_DECLARADA'
    EMISSAO_NATIVA_COMPROVADA = 'EMISSAO_NATIVA_COMPROVADA'
    RESPOSTA_CORRELACIONADA = 'RESPOSTA_CORRELACIONADA'
    JOB_CONCLUIDO_OBSERVADO = 'JOB_CONCLUIDO_OBSERVADO'


def text_ok(value, limit=255):
    return isinstance(value, str) and 0 < len(value) <= limit and '\x00' not in value


@dataclass(frozen=True)
class Origin:
    provenance: Provenance = Provenance.DECLARADA
    source: str = 'entrada-declarada'

    def __post_init__(self):
        if (not isinstance(self.provenance, Provenance)
                or self.provenance not in (Provenance.SIMULADA, Provenance.DECLARADA,
                                           Provenance.INCONCLUSIVA, Provenance.INVALIDADA)
                or not text_ok(self.source, 128)):
            raise ValueError('proveniência nativa/autenticada indisponível em C2-B')

    @property
    def state(self):
        return {Provenance.SIMULADA: Verification.OBSERVADO_SIMULADAMENTE,
                Provenance.DECLARADA: Verification.VALIDADO_SINTATICAMENTE,
                Provenance.INCONCLUSIVA: Verification.NAO_VERIFICADO,
                Provenance.INVALIDADA: Verification.INVALIDADO}[self.provenance]


@dataclass(frozen=True)
class ManagerContract:
    target: ManagerTarget
    connection_id: str | None = None
    owner: str | None = None
    subscription_ns: int | None = None
    continuous: bool = True
    origin: Origin = Origin()

    def __post_init__(self):
        if (not isinstance(self.target, ManagerTarget) or not isinstance(self.origin, Origin)
                or type(self.continuous) is not bool
                or (self.connection_id is not None and not text_ok(self.connection_id, 128))
                or (self.owner is not None and (not text_ok(self.owner, 64)
                    or re.fullmatch(r':[0-9]+\.[0-9]+', self.owner) is None))
                or (self.subscription_ns is not None and not c1.positive(self.subscription_ns))):
            raise ValueError('contrato do gerenciador inválido')

    @classmethod
    def experimental(cls, uid, **kwargs):
        return cls(ManagerTarget(Manager.USER, uid), **kwargs)

    @property
    def service(self):
        return SERVICE

    @property
    def authentication(self):
        return Verification.NAO_VERIFICADO

    @property
    def state(self):
        return self.origin.state if self.continuous else Verification.INVALIDADO


@dataclass(frozen=True)
class ObjectFixture:
    path: str
    canonical_path: str
    kind: str
    uid: int
    mode: int
    device: int
    inode: int
    monotonic_ns: int | None
    symlink: bool = False
    origin: Origin = Origin()

    def __post_init__(self):
        if not isinstance(self.origin, Origin):
            raise ValueError('origem ausente')
        for value in (self.path, self.canonical_path, self.kind):
            if not text_ok(value):
                raise ValueError('objeto sintético sem limites')


@dataclass(frozen=True)
class RuntimeContract:
    target: ManagerTarget
    # Política explícita, sem inferir autenticação de metadados de caminho.
    socket_mode: int | None = None

    def __post_init__(self):
        if (not isinstance(self.target, ManagerTarget) or (self.socket_mode is not None
                and (type(self.socket_mode) is not int or not 0 <= self.socket_mode <= 0o777))):
            raise ValueError('política de runtime inválida')

    @property
    def expected_socket(self):
        return self.target.address.removeprefix('unix:path=')

    def evaluate(self, runtime=None, socket=None, *, previous_socket=None, previous_runtime=None, environment=None):
        errors = []
        if self.socket_mode is None:
            errors.append('POLITICA_PERMISSAO_SOCKET_NAO_DEFINIDA')
        if environment is not None:
            try:
                self.target.validate_environment(environment)
            except (ValueError, TypeError):
                errors.append('AMBIENTE_INCONSISTENTE')
        expected_uid = self.target.uid if self.target.manager is Manager.USER else 0
        objects = [(socket, self.expected_socket, 'socket', self.socket_mode)]
        if self.target.manager is Manager.USER:
            objects.insert(0, (runtime, self.target.runtime_dir, 'directory', 0o700))
        for value, path, kind, mode in objects:
            if value is None:
                errors.append('RUNTIME_AUSENTE' if kind == 'directory' else 'SOCKET_AUSENTE')
                continue
            if not isinstance(value, ObjectFixture):
                errors.append('OBJETO_INVALIDO')
                continue
            if value.path != path or value.canonical_path != path or not value.path.startswith('/'):
                errors.append('CAMINHO_DIVERGENTE')
            if value.symlink is not False:
                errors.append('SYMLINK_RECUSADO')
            if value.kind != kind:
                errors.append('TIPO_DIVERGENTE')
            if type(value.uid) is not int or value.uid != expected_uid:
                errors.append('PROPRIEDADE_DIVERGENTE')
            if type(value.mode) is not int or value.mode != mode:
                errors.append('PERMISSAO_INADEQUADA')
            if not c1.positive(value.inode) or type(value.device) is not int or value.device < 0:
                errors.append('IDENTIDADE_OBJETO_INVALIDA')
            if not c1.positive(value.monotonic_ns):
                errors.append('TIMESTAMP_INVALIDO')
            if value.origin.provenance is not Provenance.SIMULADA:
                errors.append('OBJETO_NAO_OBSERVADO_SIMULADAMENTE')
        for previous, current, label in ((previous_socket, socket, 'SOCKET'),
                                          (previous_runtime, runtime, 'RUNTIME')):
            if previous is None:
                continue
            if not isinstance(previous, ObjectFixture) or not isinstance(current, ObjectFixture):
                errors.append('COMPARACAO_OBJETO_INDISPONIVEL')
            elif previous.origin.provenance is not Provenance.SIMULADA:
                errors.append('OBJETO_ANTERIOR_NAO_SIMULADO')
            elif (previous.symlink is not False or previous.kind != current.kind
                  or type(previous.uid) is not int or type(previous.mode) is not int
                  or not c1.positive(previous.inode) or type(previous.device) is not int or previous.device < 0
                  or previous.uid != current.uid or previous.mode != current.mode):
                errors.append('OBJETO_ANTERIOR_INVALIDO')
            elif ((previous.device, previous.inode, previous.path, previous.canonical_path) !=
                  (current.device, current.inode, current.path, current.canonical_path)):
                errors.append(label + '_SUBSTITUIDO')
            elif (not c1.positive(previous.monotonic_ns) or not c1.positive(current.monotonic_ns)
                  or previous.monotonic_ns >= current.monotonic_ns):
                errors.append('ORDEM_TEMPORAL_INVALIDA')
        return dict(target=self.target.report(), expected_uid=expected_uid,
                    expected_runtime=self.target.runtime_dir, expected_socket=self.expected_socket,
                    expected_runtime_mode=0o700, expected_socket_mode=self.socket_mode,
                    state=Verification.INVALIDADO if errors else Verification.OBSERVADO_SIMULADAMENTE,
                    simulated_object_verified=not errors, simulated_ownership_verified=not errors,
                    simulated_socket_verified=not errors, errors=errors,
                    connection_authenticated=False, operational_object_verified=False,
                    provenance=Provenance.SIMULADA,
                    inputs=dict(previous_runtime=asdict(previous_runtime) if isinstance(previous_runtime, ObjectFixture) else None,
                                runtime=asdict(runtime) if isinstance(runtime, ObjectFixture) else None,
                                socket=asdict(socket) if isinstance(socket, ObjectFixture) else None,
                                previous_socket=asdict(previous_socket) if isinstance(previous_socket, ObjectFixture) else None),
                    limitation='fixtures não eliminam substituição entre inspeção e conexão')


@dataclass(frozen=True)
class UnitSnapshot:
    manager: ManagerContract
    unit_name: str
    unit_path: str
    invocation_id: str | None
    cgroup_path: str | None
    cgroup_inode: int | None
    monotonic_ns: int | None
    active_state: str = 'active'
    load_state: str = 'loaded'
    job_state: str | None = None
    continuous: bool = True
    origin: Origin = Origin()

    def __post_init__(self):
        if (not isinstance(self.manager, ManagerContract) or not isinstance(self.origin, Origin)
                or not text_ok(self.unit_name) or not text_ok(self.unit_path)
                or re.fullmatch(r'/org/freedesktop/systemd1/unit/[A-Za-z0-9_]+', self.unit_path) is None
                or any(v is not None and not text_ok(v) for v in
                       (self.invocation_id, self.cgroup_path, self.active_state, self.load_state, self.job_state))
                or type(self.continuous) is not bool):
            raise ValueError('snapshot inválido')
        if (self.cgroup_path is not None and (not self.cgroup_path.startswith('/')
                or (self.cgroup_path != '/' and any(part in ('', '.', '..') for part in self.cgroup_path.split('/')[1:])))):
            raise ValueError('cgroup não canônico')

    @property
    def authentication(self):
        return Verification.NAO_VERIFICADO

    @property
    def state(self):
        return self.origin.state if self.continuous else Verification.INVALIDADO


@dataclass(frozen=True)
class EmissionContract:
    manager: ManagerContract
    unit_name: str
    unit_path: str
    serial: int | None
    monotonic_ns: int | None
    deadline_ns: int
    origin: Origin = Origin()

    def __post_init__(self):
        if (not isinstance(self.manager, ManagerContract) or not isinstance(self.origin, Origin)
                or not text_ok(self.unit_name) or not text_ok(self.unit_path)
                or (self.serial is not None and (not c1.positive(self.serial) or self.serial > 4294967295))):
            raise ValueError('contrato de emissão inválido')

    @property
    def method(self):
        return 'StopUnit'  # Intenção descritiva; não há implementação de envio.

    @property
    def state(self):
        return (EmissionState.EMISSAO_DECLARADA if self.serial is not None and c1.positive(self.monotonic_ns)
                else EmissionState.EMISSAO_NAO_COMPROVADA)


@dataclass(frozen=True)
class ReplyEvidence:
    manager: ManagerContract
    reply: c1.StopReply
    origin: Origin = Origin()

    def __post_init__(self):
        if not isinstance(self.manager, ManagerContract) or not isinstance(self.reply, c1.StopReply) or not isinstance(self.origin, Origin):
            raise ValueError('resposta inválida')
        if (not c1.positive(self.reply.reply_serial) or self.reply.reply_serial > 4294967295
                or not text_ok(self.reply.job_object_path) or c1.job_number(self.reply.job_object_path) is None
                or not text_ok(self.reply.connection_id, 128) or not text_ok(self.reply.sender, 64)):
            raise ValueError('resposta sem limites ou caminho válido')


@dataclass(frozen=True)
class EventEvidence:
    manager: ManagerContract
    event: c1.JobRemoved
    origin: Origin = Origin()

    def __post_init__(self):
        if not isinstance(self.manager, ManagerContract) or not isinstance(self.event, c1.JobRemoved) or not isinstance(self.origin, Origin):
            raise ValueError('evento inválido')
        if (not text_ok(self.event.unit_name) or not text_ok(self.event.job_object_path)
                or not text_ok(self.event.job_result, 64) or not text_ok(self.event.sender, 64)
                or not text_ok(self.event.connection_id, 128) or not text_ok(self.event.object_path)
                or not text_ok(self.event.interface, 128) or not text_ok(self.event.member, 64)):
            raise ValueError('evento sem limites')


class ObservationalContracts:
    """Interface descritiva pura. Todos os tempos são entradas, nunca inventados.

    Não possui backend, relógio, chamadas de conexão/subscrição ou envio.
    Os positivos C1 ficam exclusivamente em simulated_c1; operacional é fechado.
    """
    def __init__(self, manager):
        if not isinstance(manager, ManagerContract):
            raise ValueError('gerenciador obrigatório')
        self.manager = manager
        self.emission = None
        self.before = None
        self.after = None
        self.reply = None
        self.events = []
        self.history = []
        self.errors = []
        self.error_count = 0
        self.broken = False
        self.closed = False
        self.last_ns = 0
        self.buffer_bytes = 0
        self.simulation_only = manager.origin.provenance is Provenance.SIMULADA
        self.runtime_report = None

    def fail(self, code, timestamp=None):
        if not text_ok(code, 128):
            raise ValueError('diagnóstico inválido')
        self.broken = True
        self.error_count += 1
        if len(self.errors) < MAX_ERRORS:
            self.errors.append(dict(code=code, monotonic_ns=timestamp))

    def record(self, value, timestamp):
        # Registros limitados e imutáveis: não substituir evidências anteriores.
        if self.closed:
            self.fail('CONEXAO_ENCERRADA', timestamp)
            return False
        if len(self.history) >= MAX_EVENTS:
            self.fail('OVERFLOW', timestamp)
            return False
        self.history.append(value)
        if not c1.positive(timestamp) or timestamp < self.last_ns:
            self.fail('TIMESTAMP_INVALIDO_OU_FORA_DE_ORDEM', timestamp)
            return False
        self.last_ns = timestamp
        self.simulation_only &= value.origin.provenance is Provenance.SIMULADA
        if value.origin.provenance in (Provenance.INVALIDADA, Provenance.INCONCLUSIVA):
            self.fail('PROVENIENCIA_INSUFICIENTE', timestamp)
            return False
        return True

    def check_manager(self, manager):
        valid = (manager.target == self.manager.target
                 and manager.connection_id == self.manager.connection_id
                 and manager.owner == self.manager.owner and manager.owner is not None
                 and manager.subscription_ns == self.manager.subscription_ns
                 and manager.connection_id is not None and manager.continuous and self.manager.continuous
                 and manager.origin.provenance not in (Provenance.INVALIDADA, Provenance.INCONCLUSIVA))
        if not valid:
            self.fail('GERENCIADOR_CONEXAO_PROPRIETARIO_OU_CONTINUIDADE_DIVERGENTE')
        self.simulation_only &= manager.origin.provenance is Provenance.SIMULADA
        return valid

    def record_runtime(self, runtime=None, socket=None, *, previous_socket=None, previous_runtime=None, environment=None, socket_mode=None):
        if self.closed or self.runtime_report is not None:
            self.fail('RUNTIME_REPETIDO_OU_CONEXAO_ENCERRADA')
            return self.runtime_report
        self.runtime_report = RuntimeContract(self.manager.target, socket_mode=socket_mode).evaluate(
            runtime, socket, previous_socket=previous_socket, previous_runtime=previous_runtime, environment=environment)
        for error in self.runtime_report['errors']:
            self.fail(error)
        return self.runtime_report

    def record_manager(self, phase, manager):
        if phase not in ('connection', 'owner', 'subscription') or not isinstance(manager, ManagerContract):
            raise ValueError('fase/gerenciador inválido')
        valid = self.check_manager(manager)
        if phase == 'subscription' and manager.subscription_ns is None:
            self.fail('SUBSCRICAO_AUSENTE')
            valid = False
        return dict(phase=phase, consistent=valid, authentication=Verification.NAO_VERIFICADO,
                    provenance=manager.origin.provenance)

    def describe_manager(self):
        return dict(target=self.manager.target.report(), service=self.manager.service,
                    connection_id=self.manager.connection_id, unique_owner=self.manager.owner,
                    authentication=self.manager.authentication, subscription_ns=self.manager.subscription_ns,
                    continuous=self.manager.continuous and not self.broken,
                    state=Verification.INVALIDADO if self.broken else self.manager.state,
                    origin=asdict(self.manager.origin))

    def record_snapshot(self, phase, snapshot):
        if phase not in ('before', 'after') or not isinstance(snapshot, UnitSnapshot):
            raise ValueError('fase/snapshot inválido')
        valid = self.record(snapshot, snapshot.monotonic_ns) and self.check_manager(snapshot.manager)
        if not snapshot.continuous:
            self.fail('LACUNA_DE_SNAPSHOT', snapshot.monotonic_ns)
            valid = False
        if not valid:
            return
        if getattr(self, phase) is not None:
            self.fail('SNAPSHOT_REPETIDO', snapshot.monotonic_ns)
            return
        setattr(self, phase, snapshot)
        if self.emission and ((phase == 'before' and snapshot.monotonic_ns > self.emission.monotonic_ns)
                              or (phase == 'after' and not self.emission.monotonic_ns <= snapshot.monotonic_ns <= self.emission.deadline_ns)):
            self.fail('SNAPSHOT_FORA_DA_FASE_TEMPORAL', snapshot.monotonic_ns)
        if self.emission and (snapshot.unit_name != self.emission.unit_name or snapshot.unit_path != self.emission.unit_path):
            self.fail('UNIDADE_DIVERGENTE', snapshot.monotonic_ns)
        if self.before and self.after:
            if (self.before.unit_name, self.before.unit_path) != (self.after.unit_name, self.after.unit_path):
                self.fail('UNIDADE_DIVERGENTE', snapshot.monotonic_ns)
            if self.before.invocation_id != self.after.invocation_id:
                self.fail('INVOCATION_ALTERADO_NOME_REUTILIZADO_OU_INSTANCIA_INDISPONIVEL', snapshot.monotonic_ns)
            if (self.after.load_state != 'not-found' and
                    (self.before.cgroup_path, self.before.cgroup_inode) != (self.after.cgroup_path, self.after.cgroup_inode)):
                self.fail('CGROUP_DIVERGENTE', snapshot.monotonic_ns)

    def declare_emission(self, emission):
        if not isinstance(emission, EmissionContract):
            raise ValueError('emissão inválida')
        if not self.record(emission, emission.monotonic_ns) or not self.check_manager(emission.manager):
            return
        if self.emission is not None:
            self.fail('EMISSAO_REPETIDA', emission.monotonic_ns)
            return
        self.emission = emission
        if self.after is not None:
            self.fail('SNAPSHOT_POSTERIOR_ANTES_DO_PEDIDO', emission.monotonic_ns)
        if (not c1.positive(emission.deadline_ns) or
                not 0 < emission.deadline_ns - emission.monotonic_ns <= 5_000_000_000):
            self.fail('JANELA_INVALIDA', emission.monotonic_ns)
        if (self.manager.subscription_ns is None or self.manager.subscription_ns >= emission.monotonic_ns):
            self.fail('SUBSCRICAO_AUSENTE_OU_TARDIA', emission.monotonic_ns)
        if self.before and (self.before.unit_name, self.before.unit_path) != (emission.unit_name, emission.unit_path):
            self.fail('UNIDADE_DIVERGENTE', emission.monotonic_ns)

    def receive_reply(self, evidence):
        if not isinstance(evidence, ReplyEvidence):
            raise ValueError('resposta inválida')
        if not self.record(evidence, evidence.reply.response_monotonic_ns) or not self.check_manager(evidence.manager):
            return
        if self.reply is not None:
            self.fail('RESPOSTA_DUPLICADA', evidence.reply.response_monotonic_ns)
            return
        self.reply = evidence.reply

    def receive_job_removed(self, evidence):
        if not isinstance(evidence, EventEvidence):
            raise ValueError('evento inválido')
        event = evidence.event
        size = sum(len(v.encode('utf-8')) for v in
                   (event.unit_name, event.job_object_path, event.job_result, event.sender, event.connection_id)
                   if isinstance(v, str)) + 128
        if (size > 65536 or self.buffer_bytes + size > MAX_BUFFER_BYTES or len(self.events) >= MAX_EVENTS):
            self.fail('OVERFLOW', event.signal_monotonic_ns)
            return
        if self.record(evidence, event.signal_monotonic_ns) and self.check_manager(evidence.manager):
            self.events.append(event)
            self.buffer_bytes += size

    def close(self, *, failure=None, timestamp=None):
        if not self.closed:
            if failure is not None:
                self.fail('FALHA_FECHAMENTO:' + failure, timestamp)
            self.closed = True

    def result(self, observed_until_ns):
        # Consulta incompleta não destrói a possibilidade de completar a fixture.
        pending = []
        if not c1.positive(observed_until_ns) or observed_until_ns < self.last_ns:
            pending.append('FIM_OBSERVACAO_INVALIDO')
        before = self.before
        emission = self.emission
        if before is None:
            pending.append('SNAPSHOT_ANTERIOR_AUSENTE')
        if self.after is None:
            pending.append('SNAPSHOT_POSTERIOR_AUSENTE')
        if emission is None:
            pending.append('EMISSAO_AUSENTE')
        if before and (not isinstance(before.invocation_id, str)
                       or re.fullmatch('[0-9a-f]{32}', before.invocation_id) is None
                       or before.invocation_id == '0'*32 or not c1.positive(before.cgroup_inode)):
            pending.append('IDENTIDADE_INSTANCIA_INCONCLUSIVA')
        incomplete = self.broken or bool(pending)
        report = None
        if emission is not None:
            request = c1.StopRequest('c2b-fixture', emission.unit_name, emission.unit_path,
                                     before.invocation_id if before else None,
                                     before.cgroup_inode if before else None,
                                     emission.monotonic_ns, emission.deadline_ns, emission.serial,
                                     self.manager.connection_id, self.manager.owner,
                                     emitted=self.simulation_only and not incomplete)
            subscription = c1.Subscription(self.manager.subscription_ns or 0, self.manager.connection_id,
                                           self.manager.owner, confirmed=not incomplete,
                                           available=not incomplete)
            def identity(snapshot):
                if snapshot is None:
                    return None
                return c1.UnitIdentity(snapshot.unit_name, snapshot.unit_path, snapshot.invocation_id,
                                       snapshot.cgroup_inode, snapshot.monotonic_ns,
                                       snapshot.manager.connection_id, snapshot.manager.owner,
                                       snapshot.active_state, snapshot.load_state)
            report = c1.correlate_stop_job(request, subscription, self.reply, self.events,
                                           observed_until_ns, before=identity(before), after=identity(self.after))
        def snapshot_report(snapshot):
            if snapshot is None:
                return None
            return dict(asdict(snapshot), authentication=snapshot.authentication, verification=snapshot.state)
        return dict(manager=self.describe_manager(), emission_state=emission.state if emission else EmissionState.EMISSAO_NAO_COMPROVADA,
                    emission_details=dict(asdict(emission), method=emission.method, native_proven=False) if emission else None,
                    snapshots=dict(before=snapshot_report(before), after=snapshot_report(self.after)),
                    simulated_c1=report, runtime=self.runtime_report,
                    errors=(list(self.errors) + [dict(code=code, monotonic_ns=None) for code in pending])[:MAX_ERRORS],
                    error_count=self.error_count + len(pending), assessment_inconclusive=True,
                    inputs_complete=not incomplete,
                    continuity_valid=not self.broken, retained_records=len(self.history),
                    history=[dict(kind=type(value).__name__, evidence=asdict(value)) for value in self.history],
                    operational=dict(AUTENTICADO_OPERACIONALMENTE=False, EMISSAO_NATIVA_COMPROVADA=False,
                                     PARADA_CONFIRMADA=False, POPULACAO_VAZIA_OBSERVADA=False,
                                     LIMPEZA_INTEGRAL_COMPROVADA=False, LIMPEZA_INTEGRAL_INCONCLUSIVA=True),
                    limitations=['dados declarados/simulados não comprovam operações reais',
                                 'JobRemoved não vincula atomicamente InvocationID',
                                 'runtime inspecionado não elimina corrida até conexão',
                                 'B2 não é avaliado nem promovido por C2-B'])
