"""C2-A: transporte isolado. Nada neste módulo é usado pelo executor.

O backend nativo é opt-in, possui conexão dedicada e NÃO implementa StopUnit.
SimulatedTransport nunca autentica um barramento real. Tempos são locais.
"""
from collections import deque
from dataclasses import asdict, dataclass, replace
import re
import os
from enum import Enum
import time
import uuid

import systemd_job_evidence as c1

# Socket canônico Debian 12: não consulta DBUS_*_BUS_ADDRESS nem autolaunch.
SYSTEM_BUS_ADDRESS = 'unix:path=/run/dbus/system_bus_socket'
SERVICE = 'org.freedesktop.systemd1'
PATH = '/org/freedesktop/systemd1'
INTERFACE = SERVICE + '.Manager'
MAX_MESSAGE_BYTES = 65536
MAX_BUFFER_BYTES = 262144
MAX_EVENTS = 128
MAX_ERRORS = 32
WAIT_SECONDS = 1.0
MAX_DISPATCH = 32
DISPATCH_SECONDS = 0.05
UNIQUE = re.compile(r':[0-9]+\.[0-9]+\Z')


class Manager(str, Enum):
    USER = 'USER'
    SYSTEM = 'SYSTEM'


@dataclass(frozen=True)
class ManagerTarget:
    """Seleção sintática Debian 12; não autentica runtime, socket ou serviço."""
    manager: Manager
    uid: int | None = None
    runtime_dir: str | None = None
    address: str | None = None

    def __post_init__(self):
        if not isinstance(self.manager, Manager):
            raise ValueError('gerenciador deve ser explícito')
        if self.manager is Manager.USER:
            if type(self.uid) is not int or not 0 <= self.uid < 4294967295:
                raise ValueError('UID inválido')
            runtime = '/run/user/' + str(self.uid)
            address = 'unix:path=' + runtime + '/bus'
            if self.runtime_dir not in (None, runtime) or self.address not in (None, address):
                raise ValueError('runtime/endereço USER divergente ou não canônico')
            object.__setattr__(self, 'runtime_dir', runtime)
        else:
            if self.uid is not None or self.runtime_dir is not None:
                raise ValueError('SYSTEM não recebe UID/runtime de usuário')
            address = SYSTEM_BUS_ADDRESS
            if self.address not in (None, address):
                raise ValueError('endereço SYSTEM divergente')
        object.__setattr__(self, 'address', address)

    @classmethod
    def experimental(cls):
        # UID efetivo local, não valor fixo nem inferido de ambiente.
        return cls(Manager.USER, os.geteuid())

    def validate_environment(self, environment):
        """Valida snapshot explícito; ambiente nunca seleciona o endereço."""
        if self.manager is Manager.USER:
            expected = {'XDG_RUNTIME_DIR': self.runtime_dir,
                        'DBUS_SESSION_BUS_ADDRESS': self.address}
        else:
            expected = {'DBUS_SYSTEM_BUS_ADDRESS': self.address}
        for key, value in expected.items():
            if key in environment and environment[key] != value:
                raise ValueError('ambiente divergente: ' + key)

    def report(self):
        return dict(manager=self.manager.value, uid=self.uid, runtime_dir=self.runtime_dir,
                    address=self.address, address_configured=True,
                    address_status='VALIDADO_SINTATICAMENTE',
                    runtime_ownership_verified=False, socket_verified=False,
                    real_authentication_proven=False, manager_identity_proven=False,
                    unit_correlated=False)


@dataclass(frozen=True)
class ScopedEvidence:
    """Namespace externo a C1 para pedidos/snapshots declarados no contexto."""
    target: ManagerTarget
    connection_id: str
    evidence: object


@dataclass(frozen=True)
class Envelope:
    """Cabeçalhos do backend; o corpo nunca define remetente ou timestamp."""
    kind: str
    sender: str
    signature: str
    body: tuple
    path: str = PATH
    interface: str = INTERFACE
    member: str = 'JobRemoved'
    reply_serial: int = 0
    message_serial: int = 0
    target: ManagerTarget | None = None
    connection_id: str | None = None


def message_size(message):
    """Limite de aplicação após decodificação; não limita alocação libdbus."""
    if not isinstance(message, Envelope) or not isinstance(message.body, tuple):
        raise ValueError('envelope inválido')
    fields = (message.kind, message.sender, message.signature, message.path,
              message.interface, message.member, *message.body)
    size = 64
    for value in fields:
        if type(value) is int:
            size += 8
        elif isinstance(value, str):
            if len(value) > MAX_MESSAGE_BYTES:
                raise ValueError('mensagem excede limite')
            size += len(value.encode('utf-8')) + 4
        else:
            raise ValueError('tipo de mensagem inválido')
        if size > MAX_MESSAGE_BYTES:
            raise ValueError('mensagem excede limite')
    return size


class SimulatedTransport:
    """Backend determinístico sem I/O. Asserções de identidade são fixtures."""
    provenance = 'SIMULADO_SEM_AUTENTICACAO_REAL'

    def __init__(self, *, owner=':1.10', available=True, subscribe_ok=True, target=None):
        self.target = target if target is not None else ManagerTarget.experimental()
        if not isinstance(self.target, ManagerTarget):
            raise ValueError('alvo inválido')
        self.connection_id = 'simulated-' + uuid.uuid4().hex
        self.owner = owner
        self.available = available
        self.subscribe_ok = subscribe_ok
        self.closed = False
        self.operations = []

    def open(self):
        self.operations.append('open')
        if not self.available or self.closed:
            raise OSError('transporte indisponível')
        return self.connection_id, self.owner

    def subscribe(self):
        self.operations.append('Subscribe')
        if not self.subscribe_ok:
            raise OSError('subscrição recusada')

    def close(self):
        if not self.closed:
            self.operations.append('close-dedicated-connection')
            self.closed = True


class Observation:
    """Um ciclo/uma solicitação; sem retries, threads, I/O ou envio de stop.

    request/reply são observados, nunca enviados. Backend nativo deve entregar
    cabeçalhos reais; a API pública não transforma fixtures em autenticação.
    """
    def __init__(self, transport, *, clock=time.monotonic_ns, max_events=MAX_EVENTS):
        if type(max_events) is not int or not 1 <= max_events <= MAX_EVENTS:
            raise ValueError('limite de eventos inválido')
        self.transport = transport
        self.target = transport.target
        self.clock = clock
        self.max_events = max_events
        self.connection_id = None
        self.owner = None
        self.ready = None
        self.request = None
        self.reply = None
        self.events = []
        self.buffer_bytes = 0
        self.errors = []
        self.error_count = 0
        self.received = 0
        self.last_time = 0
        self.closed = False
        self.started = False
        self.broken = False

    def fault(self, reason):
        self.broken = True
        self.error_count += 1
        if len(self.errors) < MAX_ERRORS:
            self.errors.append(reason)

    def unscoped(self, evidence):
        if isinstance(evidence, ScopedEvidence):
            if evidence.target != self.target or evidence.connection_id != self.connection_id:
                self.fault('EVIDENCIA_DE_OUTRO_GERENCIADOR_OU_CONEXAO')
                return (replace(evidence.evidence, emitted=False)
                        if isinstance(evidence.evidence, c1.StopRequest) else None)
            return evidence.evidence
        # DTO legado: declarado no contexto, nunca autenticado por esta conversão.
        return evidence

    def check_target(self):
        if self.transport.target != self.target:
            self.fault('ALVO_DO_TRANSPORTE_ALTERADO')

    def timestamp(self):
        now = self.clock()
        if type(now) is not int or now <= 0 or now < self.last_time:
            self.fault('ORDEM_TEMPORAL_INVALIDA')
            return None
        self.last_time = now
        return now

    def start(self):
        if self.started or self.closed:
            self.fault('CICLO_JA_INICIADO_OU_ENCERRADO')
            return
        self.started = True
        self.check_target()
        if self.broken:
            return
        try:
            self.connection_id, self.owner = self.transport.open()
            if not self.connection_id or not isinstance(self.owner, str) or not UNIQUE.fullmatch(self.owner):
                raise ValueError('proprietário único ausente')
            # O backend instala match/filtro antes de Subscribe e só retorna no ACK.
            self.transport.subscribe()
            self.ready = self.timestamp()
        except Exception as exc:
            self.fault('SUBSCRICAO_INDISPONIVEL:' + type(exc).__name__)
            try:
                self.transport.close()
            except Exception as close_error:
                self.fault('ENCERRAMENTO:' + type(close_error).__name__)

    def observe_request(self, request):
        if self.request is not None or self.closed:
            self.fault('SOLICITACAO_REPETIDA_OU_ENCERRADA')
            return
        self.check_target()
        request = self.unscoped(request)
        now = self.timestamp()
        # O início é registrado aqui; o timestamp declarado pelo chamador não
        # autentica emissão real. Esta etapa só observa/prepara solicitações.
        self.request = replace(request, request_monotonic_ns=now or 0)
        request = self.request
        if isinstance(self.transport, NativeTransport):
            self.transport.expect_declared_reply(request.call_serial)
        if (self.ready is None or now is None or self.ready >= request.request_monotonic_ns
                or request.connection_id != self.connection_id
                or request.manager_owner != self.owner
                or not 0 < request.deadline_monotonic_ns - request.request_monotonic_ns <= 5_000_000_000):
            self.fault('SOLICITACAO_SEM_SUBSCRICAO_PREVIA_OU_VINCULO')

    def receive(self, envelope):
        if not isinstance(self.transport, SimulatedTransport):
            self.fault('INJECAO_FORA_DO_BACKEND_SIMULADO')
            return
        self._receive_at(envelope, self.timestamp())

    def _receive_at(self, envelope, now):
        if self.closed:
            self.fault('EVENTO_APOS_ENCERRAMENTO')
            return
        self.received += 1
        self.check_target()
        if now is None:
            return
        try:
            size = message_size(envelope)
            if ((envelope.target is not None and envelope.target != self.target)
                    or (envelope.connection_id is not None and envelope.connection_id != self.connection_id)):
                self.fault('EVENTO_DE_OUTRO_GERENCIADOR_OU_CONEXAO')
                return
            if envelope.kind in ('disconnect', 'owner-change', 'lost'):
                # Somente o backend interpreta eventos do bus daemon/local.
                self.fault('CONTINUIDADE_PERDIDA:' + envelope.kind)
                return
            if envelope.sender != self.owner or self.owner is None:
                self.fault('REMETENTE_DIVERGENTE')
                return
            if envelope.kind == 'reply':
                if (self.request is None or envelope.reply_serial != self.request.call_serial
                        or not c1.positive(envelope.reply_serial) or envelope.reply_serial > 4294967295
                        or not self.request.request_monotonic_ns <= now <= self.request.deadline_monotonic_ns
                        or envelope.signature != 'o' or len(envelope.body) != 1
                        or c1.job_number(envelope.body[0]) is None
                        or type(envelope.reply_serial) is not int):
                    raise ValueError('resposta inválida')
                reply = c1.StopReply(envelope.body[0], now, envelope.reply_serial,
                                     self.connection_id, envelope.sender)
                if self.reply is not None:
                    self.fault('RESPOSTA_REPETIDA')
                else:
                    self.reply = reply
                return
            if (envelope.kind != 'signal' or envelope.signature != 'uoss'
                    or envelope.path != PATH or envelope.interface != INTERFACE
                    or envelope.member != 'JobRemoved' or len(envelope.body) != 4):
                raise ValueError('JobRemoved inválido')
            number, path, unit, result = envelope.body
            if (type(number) is not int or not 0 < number <= 4294967295
                    or c1.job_number(path) != number or not isinstance(unit, str)
                    or not unit or len(unit) > 255 or not isinstance(result, str)
                    or not result or len(result) > 64):
                raise ValueError('corpo JobRemoved inválido')
            if len(self.events) >= self.max_events or self.buffer_bytes + size > MAX_BUFFER_BYTES:
                self.fault('OVERFLOW')
                return
            self.events.append(c1.JobRemoved(number, path, unit, result, now,
                                             self.connection_id, envelope.sender))
            self.buffer_bytes += size
        except Exception as exc:
            self.fault('DECODIFICACAO_OU_LIMITE:' + type(exc).__name__)

    def poll_native(self):
        """Despacho cooperativo limitado; nunca usado pelo executor ou testes vivos."""
        if not isinstance(self.transport, NativeTransport) or self.closed:
            self.fault('BACKEND_NATIVO_INDISPONIVEL')
            return
        try:
            frames, lost = self.transport.drain()
            if lost:
                self.fault('CONTINUIDADE_NATIVA_PERDIDA')
            for timestamp, envelope in frames:
                # Tempo de recepção no filtro, não o tempo posterior do drain.
                if type(timestamp) is not int or timestamp <= 0 or timestamp < self.last_time:
                    self.fault('ORDEM_TEMPORAL_INVALIDA')
                    continue
                self.last_time = timestamp
                self._receive_at(envelope, timestamp)
        except Exception as exc:
            self.fault('DESPACHO_NATIVO:' + type(exc).__name__)

    def result(self, *, before=None, after=None):
        if self.request is None:
            raise ValueError('nenhuma solicitação observada')
        self.check_target()
        before, after = self.unscoped(before), self.unscoped(after)
        if isinstance(self.transport, NativeTransport) and self.transport.loss:
            if not self.broken:
                self.fault('CONTINUIDADE_NATIVA_PERDIDA')
        now = self.timestamp()
        if now is None:
            now = self.last_time
        subscription = c1.Subscription(self.ready or 0, self.connection_id or '', self.owner or '',
                                       confirmed=self.ready is not None and not self.broken,
                                       available=not self.broken)
        # DTO declarado não comprova emissão nativa. Nenhum serial operacional
        # é certificado até existir integração autorizada na mesma conexão.
        effective_request = (replace(self.request, emitted=False)
                             if isinstance(self.transport, NativeTransport) else self.request)
        result = c1.correlate_stop_job(effective_request, subscription, self.reply, self.events,
                                       now, before=before, after=after)
        result['transport'] = dict(provenance=self.transport.provenance,
                                   real_authentication_proven=False,
                                   continuity_valid=not self.broken, errors=list(self.errors),
                                   error_count=self.error_count, received=self.received,
                                   retained=len(self.events), buffer_bytes=self.buffer_bytes,
                                   closed=self.closed, subscription_ready_ns=self.ready,
                                   reply_envelope=asdict(self.reply) if self.reply else None,
                                   native_gaps=list(getattr(self.transport, "gaps", ())),
                                   bus_target=self.target.manager.value,
                                   selected_target=self.target.report(),
                                   call_binding_status='DECLARADO_SEM_EMISSAO_COMPROVADA',
                                   absolute_deadline_guaranteed=False,
                                   total_memory_bound_guaranteed=False)
        return result

    def __enter__(self):
        self.start()
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self.close()

    def close(self):
        if self.closed:
            return
        try:
            self.transport.close()
        except Exception as exc:
            self.fault('ENCERRAMENTO:' + type(exc).__name__)
        finally:
            self.closed = True


class NativeTransport:
    """dbus-python/GLib, conexão dedicada, desativado por padrão.

    Apenas infraestrutura de observação. Nenhuma API de envio StopUnit.
    Não usado nos testes ou importado pelo executor. Assinatura/headers provêm
    de libdbus. Ativação futura exige auditoria e autorização separadas.
    """
    provenance = 'DBUS_PYTHON_NATIVO_NAO_VALIDADO_OPERACIONALMENTE'

    def __init__(self, *, enabled=False, clock=time.monotonic_ns, target=None):
        if not enabled:
            raise RuntimeError('transporte real desativado')
        self.target = target if target is not None else ManagerTarget.experimental()
        if not isinstance(self.target, ManagerTarget):
            raise ValueError('alvo inválido')
        self.clock = clock
        self.connection_id = 'dedicated-' + uuid.uuid4().hex
        self.bus = None
        self.owner = None
        self.queue = deque()
        self.loss = False
        self.gaps = []
        self.queue_bytes = 0
        self.expected_reply_serial = None
        self.closed = False
        self.subscribed = False
        self.opened_target = None
        self.last_dispatch = None

    def expect_declared_reply(self, serial):
        """Filtro preparatório, não atestado de emissão ou autorização de stop."""
        if not c1.positive(serial) or serial > 4294967295 or self.expected_reply_serial is not None:
            self.mark_loss("SERIAL_DECLARADO_INVALIDO_OU_REPETIDO")
            return
        self.expected_reply_serial = serial

    def _call(self, destination, path, interface, member, signature='', args=()):
        # Lista fechada de destino/caminho/interface/método. Sem StopUnit.
        allowed = {('org.freedesktop.DBus', '/org/freedesktop/DBus', 'org.freedesktop.DBus', 'GetNameOwner'),
                   ('org.freedesktop.DBus', '/org/freedesktop/DBus', 'org.freedesktop.DBus', 'AddMatch'),
                   (self.owner, PATH, INTERFACE, 'Subscribe')}
        if (destination, path, interface, member) not in allowed or self.bus is None or self.closed:
            raise ValueError('operação proibida ou conexão indisponível')
        if self.opened_target != self.target:
            self.mark_loss('ALVO_ALTERADO_APOS_ABERTURA')
            raise ValueError('alvo da conexão alterado')
        from dbus.lowlevel import MethodCallMessage, MESSAGE_TYPE_METHOD_RETURN
        message = MethodCallMessage(destination, path, interface, member)
        message.append(*args, signature=signature)
        try:
            # Pending-call da biblioteca vincula ReplySerial ao serial enviado.
            # O timeout configurado não é um prazo absoluto de todo o processo.
            reply = self.bus.send_message_with_reply_and_block(message, timeout_s=WAIT_SECONDS)
            expected_signature = 's' if member == 'GetNameOwner' else ''
            if (reply.get_type() != MESSAGE_TYPE_METHOD_RETURN or reply.get_sender() != destination
                    or reply.get_signature() != expected_signature):
                raise ValueError('origem/tipo/assinatura da resposta inválidos')
            return reply
        except Exception as exc:
            self.mark_loss('CHAMADA:' + member + ':' + type(exc).__name__)
            raise

    def open(self):
        if self.bus is not None or self.closed:
            raise RuntimeError('conexão já usada')
        import dbus.bus
        from dbus.mainloop.glib import DBusGMainLoop
        try:
            if self.target.manager is Manager.USER:
                if self.target.uid != os.geteuid():
                    raise ValueError('UID efetivo diverge do alvo USER')
                # O ambiente é somente uma verificação de consistência do
                # contexto --user; jamais determina ou substitui o endereço.
                self.target.validate_environment(os.environ)
            self.opened_target = self.target
            self.bus = dbus.bus.BusConnection(self.target.address,
                                              mainloop=DBusGMainLoop(set_as_default=False))
            self.bus.set_exit_on_disconnect(False)
            self.bus.add_message_filter(self._filter)
            if not self.bus.get_is_authenticated() or not self.bus.get_is_connected():
                raise OSError("conexão não autenticada")
            # Match antes da descoberta: uma troca nesse intervalo invalida o ciclo.
            for rule in ("type='signal',sender='org.freedesktop.DBus',interface='org.freedesktop.DBus',member='NameOwnerChanged',arg0='org.freedesktop.systemd1'",
                         "type='signal',interface='org.freedesktop.systemd1.Manager',member='JobRemoved',path='/org/freedesktop/systemd1'"):
                self._call('org.freedesktop.DBus', '/org/freedesktop/DBus',
                           'org.freedesktop.DBus', 'AddMatch', 's', (rule,))
            reply = self._call('org.freedesktop.DBus', '/org/freedesktop/DBus',
                               'org.freedesktop.DBus', 'GetNameOwner', 's', (SERVICE,))
            args = reply.get_args_list()
            if reply.get_sender() != 'org.freedesktop.DBus' or reply.get_signature() != 's' or len(args) != 1:
                raise ValueError('resposta GetNameOwner inválida')
            self.owner = str(args[0])
            if not UNIQUE.fullmatch(self.owner):
                raise ValueError('proprietário inválido')
            return self.connection_id, self.owner
        except Exception:
            self.mark_loss('ABERTURA_FALHOU')
            try:
                self.close()
            except Exception as close_error:
                self.mark_loss('ENCERRAMENTO:' + type(close_error).__name__)
            raise

    def subscribe(self):
        if self.closed or self.loss or self.bus is None or self.owner is None or self.subscribed:
            raise RuntimeError('ciclo de subscrição inválido')
        reply = self._call(self.owner, PATH, INTERFACE, 'Subscribe')
        if reply.get_sender() != self.owner or reply.get_signature() != '':
            self.mark_loss('ACK_SUBSCRIBE_INVALIDO')
            raise ValueError('ACK Subscribe inválido')
        self.subscribed = True

    def _filter(self, connection, message):
        if self.closed:
            return
        try:
            self._decode_message(connection, message)
        except Exception as exc:
            self.mark_loss('FILTRO:' + type(exc).__name__)

    def _decode_message(self, connection, message):
        from dbus.lowlevel import MESSAGE_TYPE_SIGNAL, MESSAGE_TYPE_METHOD_RETURN, MESSAGE_TYPE_ERROR
        if self.closed:
            return
        if self.bus is not None and connection is not self.bus:
            self.mark_loss('CONEXAO_DIVERGENTE')
            return
        kind = message.get_type()
        if kind != MESSAGE_TYPE_SIGNAL:
            if kind not in (MESSAGE_TYPE_METHOD_RETURN, MESSAGE_TYPE_ERROR) or self.expected_reply_serial is None:
                return
            if message.get_reply_serial() != self.expected_reply_serial:
                return
            try:
                if self.owner is None or message.get_sender() != self.owner:
                    raise ValueError('remetente da resposta divergente')
                if kind == MESSAGE_TYPE_ERROR or message.get_signature() != 'o':
                    raise ValueError('resposta recusada ou inválida')
                args = message.get_args_list()
                if len(args) != 1:
                    raise ValueError('corpo resposta')
                envelope = Envelope('reply', str(message.get_sender()), 'o', (str(args[0]),),
                                    reply_serial=int(message.get_reply_serial()),
                                    message_serial=int(message.get_serial()))
                self._enqueue(envelope)
            except Exception as exc:
                self.mark_loss('RESPOSTA:' + type(exc).__name__)
            return
        try:
            sender, path = message.get_sender(), message.get_path()
            interface, member = message.get_interface(), message.get_member()
            if (sender == 'org.freedesktop.DBus' and path == '/org/freedesktop/DBus'
                    and interface == 'org.freedesktop.DBus' and member == 'NameOwnerChanged'):
                args = message.get_args_list()
                if message.get_signature() != 'sss' or len(args) != 3:
                    raise ValueError('evento de proprietário inválido')
                if str(args[0]) == SERVICE:
                    self.mark_loss('PROPRIETARIO_ALTERADO')
                return
            if path == '/org/freedesktop/DBus/Local' and interface == 'org.freedesktop.DBus.Local' and member == 'Disconnected':
                self.mark_loss('DESCONECTADO')
                return
            if path != PATH or interface != INTERFACE or member != 'JobRemoved':
                return
            if self.owner is None or sender != self.owner:
                raise ValueError('remetente do sinal divergente')
            if message.get_signature() != 'uoss':
                raise ValueError('signature')
            args = message.get_args_list()
            envelope = Envelope('signal', str(sender), 'uoss',
                                (int(args[0]), *(str(arg) for arg in args[1:])))
            self._enqueue(envelope)
        except Exception as exc:
            self.mark_loss('DECODIFICACAO_OU_LIMITE:' + type(exc).__name__)

    def mark_loss(self, reason):
        self.loss = True
        if len(self.gaps) < MAX_ERRORS:
            self.gaps.append(dict(reason=reason, monotonic_ns=self.clock()))

    def _enqueue(self, envelope):
        if self.opened_target is not None and self.opened_target != self.target:
            self.mark_loss('ALVO_ALTERADO_APOS_ABERTURA')
            raise ValueError('alvo da conexão alterado')
        if ((envelope.target is not None and envelope.target != self.target)
                or (envelope.connection_id is not None and envelope.connection_id != self.connection_id)):
            raise ValueError('origem de outro alvo/conexão')
        # Metadados locais do backend; não são campos do corpo D-Bus.
        envelope = replace(envelope, target=self.target, connection_id=self.connection_id)
        size = message_size(envelope)
        if len(self.queue) >= MAX_EVENTS or self.queue_bytes + size > MAX_BUFFER_BYTES:
            raise ValueError('overflow')
        self.queue.append((self.clock(), envelope))
        self.queue_bytes += size

    def drain(self):
        """Orçamento cooperativo; não preempta callback/libdbus/GLib."""
        if self.closed or self.bus is None:
            raise OSError('conexão encerrada')
        acquired = False
        context = None
        started = time.monotonic()
        dispatched = 0
        dispatch_failed = False
        try:
            from gi.repository import GLib
            # dbus-python 1.3.2 só suporta o contexto GLib padrão.
            context = GLib.MainContext.default()
            acquired = context.acquire()
            if not acquired:
                raise RuntimeError('contexto GLib ocupado')
            for _ in range(MAX_DISPATCH):
                if time.monotonic() - started >= DISPATCH_SECONDS or not context.pending():
                    break
                context.iteration(False)
                dispatched += 1
            if not self.bus.get_is_connected():
                self.mark_loss('DESCONECTADO')
            frames = list(self.queue)
            self.queue.clear()
            self.queue_bytes = 0
            return frames, self.loss
        except Exception as exc:
            dispatch_failed = True
            self.mark_loss('DESPACHO:' + type(exc).__name__)
            raise
        finally:
            self.last_dispatch = dict(iterations=dispatched,
                                      elapsed_seconds=time.monotonic() - started,
                                      absolute_deadline_guaranteed=False)
            if acquired:
                try:
                    context.release()
                except Exception as exc:
                    self.mark_loss('LIBERACAO_CONTEXTO:' + type(exc).__name__)
                    if not dispatch_failed:
                        raise

    def close(self):
        # Conexão dedicada: não se envia Unsubscribe em conexão compartilhada.
        if self.closed:
            return
        self.closed = True
        first_error = None
        try:
            if self.bus is not None:
                try:
                    self.bus.remove_message_filter(self._filter)
                except Exception as exc:
                    first_error = exc
                    self.mark_loss('REMOCAO_FILTRO:' + type(exc).__name__)
                try:
                    self.bus.close()
                except Exception as exc:
                    if first_error is None:
                        first_error = exc
                    self.mark_loss('FECHAMENTO_CONEXAO:' + type(exc).__name__)
        finally:
            self.bus = None
            self.queue.clear()
            self.queue_bytes = 0
        if first_error is not None:
            raise first_error
