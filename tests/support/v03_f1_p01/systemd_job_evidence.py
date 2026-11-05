"""C1: correlação pura de fixtures/eventos. Sem D-Bus, subprocessos ou I/O.

Envelope de conexão/remetente deve ser autenticado pelo futuro transporte.
Este módulo não realiza chamadas StopUnit nem autoriza qualquer parada.
"""
from dataclasses import asdict, dataclass
import re

MAX_EVENTS = 128
KNOWN_RESULTS = frozenset(('done', 'canceled', 'timeout', 'failed', 'dependency', 'skipped',
                           'invalid', 'assert', 'unsupported', 'collected', 'once'))
FAILURE_RESULTS = frozenset(('failed', 'timeout', 'dependency', 'invalid', 'assert', 'unsupported'))


@dataclass(frozen=True)
class StopRequest:
    stop_request_id: str
    unit_name: str
    unit_object_path: str
    invocation_id_before_stop: str | None
    cgroup_inode_before_stop: int | None
    request_monotonic_ns: int
    deadline_monotonic_ns: int
    call_serial: int
    connection_id: str
    manager_owner: str
    emitted: bool = True


@dataclass(frozen=True)
class Subscription:
    ready_monotonic_ns: int
    connection_id: str
    manager_owner: str
    confirmed: bool = True
    available: bool = True
    lost_monotonic_ns: int | None = None


@dataclass(frozen=True)
class StopReply:
    job_object_path: str
    response_monotonic_ns: int
    reply_serial: int
    connection_id: str
    sender: str
    accepted: bool = True


@dataclass(frozen=True)
class JobRemoved:
    job_id: int
    job_object_path: str
    unit_name: str
    job_result: str
    signal_monotonic_ns: int
    connection_id: str
    sender: str
    object_path: str = '/org/freedesktop/systemd1'
    interface: str = 'org.freedesktop.systemd1.Manager'
    member: str = 'JobRemoved'


@dataclass(frozen=True)
class UnitIdentity:
    unit_name: str
    unit_object_path: str
    invocation_id: str | None
    cgroup_inode: int | None
    observed_monotonic_ns: int
    connection_id: str
    sender: str
    active_state: str = 'active'
    load_state: str = 'loaded'


def positive(value):
    return type(value) is int and value > 0


def job_number(path):
    match = re.fullmatch(r'/org/freedesktop/systemd1/job/([1-9][0-9]*)', path) if isinstance(path, str) else None
    if not match or len(match[1]) > 10:
        return None
    number = int(match[1])
    return number if number <= 4294967295 else None


def same_transport(message, request):
    return message.connection_id == request.connection_id and message.sender == request.manager_owner


def correlate_stop_job(request, subscription, reply, events, observed_until_ns, *, before=None, after=None):
    """Snapshot de uma observação limitada; tempos são de RECEPÇÃO monotônica local.

    Eventos anteriores à resposta permanecem elegíveis, mas nunca antes do pedido.
    Somente a resposta ao call_serial fixa o job; nome/caminho de Unit não são geração.
    """
    limits = ['JobRemoved não carrega InvocationID nem inode',
              'StopUnit por nome e revalidação não são atômicos',
              'job done não comprova estado posterior, populated=0 ou ausência de escape',
              'transporte real e integração operacional não implementados']
    diagnostics = []
    transport = bool(isinstance(request.connection_id, str) and request.connection_id
                     and isinstance(request.manager_owner, str) and re.fullmatch(r':[0-9]+\.[0-9]+', request.manager_owner))
    requested = (request.emitted is True and positive(request.request_monotonic_ns)
                 and positive(request.deadline_monotonic_ns) and request.deadline_monotonic_ns > request.request_monotonic_ns
                 and positive(request.call_serial) and request.call_serial <= 4294967295 and bool(request.stop_request_id) and transport
                 and isinstance(request.unit_object_path, str)
                 and bool(re.fullmatch(r'/org/freedesktop/systemd1/unit/[A-Za-z0-9_]+', request.unit_object_path)))
    subscription_ok = (requested and subscription.available is True and subscription.confirmed is True
                       and positive(subscription.ready_monotonic_ns)
                       and subscription.ready_monotonic_ns < request.request_monotonic_ns
                       and subscription.connection_id == request.connection_id
                       and subscription.manager_owner == request.manager_owner)
    if not subscription_ok:
        diagnostics.append('subscrição/barramento não comprovado antes do pedido')
    observation_time_ok = positive(observed_until_ns) and positive(request.request_monotonic_ns) and observed_until_ns >= request.request_monotonic_ns
    job_id = job_number(reply.job_object_path) if reply else None
    accepted = bool(requested and observation_time_ok and subscription.available and reply and reply.accepted is True and same_transport(reply, request)
                    and reply.reply_serial == request.call_serial and positive(reply.response_monotonic_ns)
                    and request.request_monotonic_ns <= reply.response_monotonic_ns <= min(request.deadline_monotonic_ns, observed_until_ns)
                    and job_id is not None)
    if reply and not accepted:
        diagnostics.append('resposta ausente de vínculo, rejeitada ou temporalmente inválida')
    buffered = list(events[:MAX_EVENTS])
    overflow = len(events) > MAX_EVENTS
    unmatched, matched = [], []
    previous_time = None
    conflicting_order = False
    for index, event in enumerate(buffered):
        target = bool(accepted and event.job_id == job_id and event.job_object_path == reply.job_object_path
                      and event.unit_name == request.unit_name)
        reason = None
        if not target:
            reason = 'job ID/caminho/unidade não correlacionados'
        elif (not same_transport(event, request) or event.object_path != '/org/freedesktop/systemd1'
              or event.interface != 'org.freedesktop.systemd1.Manager' or event.member != 'JobRemoved'
              or type(event.job_id) is not int
              or job_number(event.job_object_path) != event.job_id):
            reason = 'remetente/conexão/job inválidos'
        elif (not positive(event.signal_monotonic_ns) or not observation_time_ok
              or not request.request_monotonic_ns <= event.signal_monotonic_ns <= min(request.deadline_monotonic_ns, observed_until_ns)):
            reason = 'timestamp do evento inválido ou fora da janela'
        elif previous_time is not None and event.signal_monotonic_ns < previous_time:
            reason = 'evento do job fora de ordem temporal'
            conflicting_order = True
        elif not isinstance(event.job_result, str) or not event.job_result or len(event.job_result) > 64:
            reason = 'resultado do evento inválido'
        if reason:
            unmatched.append({'index': index, 'reason': reason, 'event': asdict(event)})
        else:
            previous_time = event.signal_monotonic_ns
            matched.append(event)
    results = {event.job_result for event in matched}
    conflicting = len(results) > 1 or conflicting_order
    result_event = matched[0] if matched else None
    lost = subscription.lost_monotonic_ns
    window_end = max(reply.response_monotonic_ns, result_event.signal_monotonic_ns) if accepted and result_event else None
    continuous = (subscription_ok and observation_time_ok and window_end is not None
                  and (lost is None or (positive(lost) and lost > window_end)))
    received = bool(result_event and continuous and not overflow and not conflicting)
    result = result_event.job_result if received else None
    if overflow:
        diagnostics.append('buffer de eventos excedido; resultado não certificado')
    if conflicting:
        diagnostics.append('eventos do mesmo job conflitantes ou fora de ordem')
    if lost is not None:
        limits.append('subscrição perdida/interrompida; verificar posição na janela de correlação')
    if not received:
        diagnostics.append('resultado do job não comprovado')
    timed_out = bool(observation_time_ok and observed_until_ns >= request.deadline_monotonic_ns and not received)

    invocation = request.invocation_id_before_stop
    valid_invocation = bool(isinstance(invocation, str) and re.fullmatch(r'[0-9a-f]{32}', invocation)
                            and invocation != '0'*32 and positive(request.cgroup_inode_before_stop))
    before_ok = bool(requested and before and valid_invocation and same_transport(before, request)
                     and before.unit_name == request.unit_name and before.unit_object_path == request.unit_object_path
                     and before.invocation_id == invocation and before.cgroup_inode == request.cgroup_inode_before_stop
                     and positive(before.observed_monotonic_ns) and before.observed_monotonic_ns <= request.request_monotonic_ns
                     and before.active_state == 'active' and before.load_state == 'loaded')
    after_linked = bool(after and same_transport(after, request)
                        and after.unit_name == request.unit_name and after.unit_object_path == request.unit_object_path
                        and positive(after.observed_monotonic_ns) and observation_time_ok
                        and request.request_monotonic_ns <= after.observed_monotonic_ns <= observed_until_ns)
    removed = bool(after_linked and after.load_state == 'not-found')
    after_ok = bool(before_ok and after and same_transport(after, request) and result_event
                    and after.unit_name == request.unit_name and after.unit_object_path == request.unit_object_path
                    and after.invocation_id == invocation and after.load_state == 'loaded'
                    and positive(after.observed_monotonic_ns) and window_end is not None
                    and window_end <= after.observed_monotonic_ns <= observed_until_ns
                    and (lost is None or (positive(lost) and lost > after.observed_monotonic_ns)))
    identity_status = ('PRE_E_POS_CORROBORADAS' if after_ok else
                       'PRE_STOP_VERIFICADA_SEM_VINCULO_ATOMICO_AO_JOB' if before_ok else 'INCONCLUSIVA')
    if not before_ok:
        diagnostics.append('identidade anterior da instância ausente/divergente')
    # Mesmo snapshots coerentes não adicionam InvocationID ao JobRemoved.
    classifications = {'STOP_SOLICITADO': bool(requested), 'STOP_ACEITO': accepted,
        'JOB_IDENTIFICADO': accepted, 'RESULTADO_JOB_RECEBIDO': received,
        'JOB_CONCLUIDO_OBSERVADO': received and result == 'done',
        'JOB_FALHOU': received and result in FAILURE_RESULTS,
        'JOB_CANCELADO': received and result == 'canceled',
        'RESULTADO_JOB_DESCONHECIDO': not received or result not in KNOWN_RESULTS,
        'IDENTIDADE_INSTANCIA_INCONCLUSIVA': True,
        'EVENTO_JOB_NAO_CORRELACIONADO': bool(unmatched),
        'PARADA_CONFIRMADA': False, 'LIMPEZA_INTEGRAL_INCONCLUSIVA': True}
    return {'model': 'C1_SIMULADO_SEM_TRANSPORTE', 'stop_request_id': request.stop_request_id,
        'unit_name': request.unit_name, 'unit_object_path': request.unit_object_path,
        'invocation_id_before_stop': invocation, 'cgroup_inode_before_stop': request.cgroup_inode_before_stop,
        'job_id': job_id if accepted else None, 'job_object_path': reply.job_object_path if accepted else None,
        'job_result': result, 'request_monotonic_ns': request.request_monotonic_ns,
        'response_monotonic_ns': reply.response_monotonic_ns if reply else None,
        'signal_monotonic_ns': result_event.signal_monotonic_ns if received else None,
        'matched_events': [asdict(event) for event in matched],
        'unit_state_after': asdict(after) if after_linked else None,
        'subscription_status': 'CONTINUA_NA_JANELA' if continuous else 'NAO_COMPROVADA',
        'identity_binding_status': identity_status, 'unit_identity_before_verified': before_ok,
        'unit_identity_after_corroborated': after_ok, 'instance_proven_by_job': False,
        'unit_removed_corroborative': removed, 'observation_timeout': timed_out,
        'duplicate_events': max(0, len(matched)-len(results)), 'conflicting_events': conflicting,
        'uncorrelated_events': unmatched, 'classifications': classifications,
        'diagnostics': diagnostics, 'evidence_limitations': limits,
        'independent_evidence': {'POPULACAO_VAZIA_OBSERVADA': 'NAO_AVALIADA_C1',
                                 'AUSENCIA_DOS_PROCESSOS_CONHECIDOS': 'NAO_AVALIADA_C1'}}
