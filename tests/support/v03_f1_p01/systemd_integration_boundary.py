"""C2-C: fronteira pura, sem produtor nativo ou autoridade experimental.

Categorias e estados reservados são especificação, nunca credenciais. O limite
é de dados/contabilidade Python, não sandbox contra código arbitrário no processo.
"""
from dataclasses import dataclass, fields
from enum import Enum
from types import MappingProxyType
import json

import systemd_observation_contracts as b
import systemd_job_evidence as c1
import systemd_job_transport as t


class Trust(str, Enum):
    DECLARED = 'DADOS_DECLARADOS'
    SIMULATED = 'DADOS_SIMULADOS'
    NATIVE_UNAUTHENTICATED = 'OBSERVACAO_NATIVA_NAO_AUTENTICADA'
    NATIVE_AUTHENTICATED = 'OBSERVACAO_NATIVA_AUTENTICADA'
    VERIFIED_CORRELATION = 'CORRELACAO_VERIFICADA'
    EXPERIMENTAL = 'CONCLUSAO_EXPERIMENTAL'


class GuaranteeStatus(str, Enum):
    NOT_EVALUATED = 'NAO_AVALIADA'
    SPECIFIED = 'ESPECIFICADA'
    SIMULATED = 'SIMULADA'
    BLOCKED = 'BLOQUEADA'
    OPERATIONALLY_PROVEN = 'COMPROVADA_OPERACIONALMENTE'


class State(str, Enum):
    DISABLED = 'DESABILITADA'
    CONFIGURED = 'CONFIGURADA'
    PREVALIDATED = 'PRE_VALIDADA'
    SIMULATING = 'SIMULANDO'
    CONNECTED = 'CONEXAO_ESTABELECIDA'
    IDENTIFIED = 'IDENTIDADE_VERIFICADA'
    SUBSCRIBED = 'SUBSCRITA'
    OBSERVING = 'OBSERVANDO'
    INVALID = 'INVALIDADA'
    CLOSED = 'ENCERRADA'


RESERVED_STATES = frozenset((State.CONNECTED, State.IDENTIFIED, State.SUBSCRIBED, State.OBSERVING))


@dataclass(frozen=True)
class Guarantee:
    identifier: str
    description: str
    dependencies: tuple
    evidence: str
    verifier: str
    failures: str
    acceptance: str
    blocked: bool = False

    @property
    def prerequisites(self):
        return self.dependencies

    @property
    def required_origin(self):
        return Trust.NATIVE_AUTHENTICATED

    @property
    def current_status(self):
        return GuaranteeStatus.BLOCKED if self.blocked else GuaranteeStatus.SPECIFIED


# Arestas são requisitos necessários, nunca suficientes; G21 requer escopo e
# ausência de escape demonstrados separadamente, sem promessa universal.
GUARANTEES = (
    Guarantee('G01', 'Gerenciador correto', (), 'alvo e serviço no contexto do executor', 'comparar USER/SYSTEM e contexto autenticado', 'gerenciador/namespace errado', 'mesmo gerenciador do executor --user'),
    Guarantee('G02', 'UID correto', ('G01',), 'UID efetivo e credenciais do cliente', 'validar identidade local e credenciais do transporte', 'UID informado não corresponde ao processo', 'UID efetivo vinculado à conexão'),
    Guarantee('G03', 'Runtime esperado', ('G02',), 'tipo, dono, permissões, caminho e device/inode', 'inspeção segura de ancestrais e substituições', 'ausência/symlink/TOCTOU', 'política explícita e nenhuma divergência não resolvida'),
    Guarantee('G04', 'Socket esperado', ('G03',), 'socket, device/inode e peer conectado', 'validar objeto e peer separadamente', 'socket substituído ou redirecionado', 'endereço/objeto/peer coerentes sem alegar eliminar TOCTOU'),
    Guarantee('G05', 'Conexão dedicada', ('G04',), 'handle exclusivo e epoch', 'produtor nativo controla criação e fechamento', 'reuso de handle/conexão anterior', 'ciclo exclusivo sem reconexão silenciosa'),
    Guarantee('G06', 'Autenticação da conexão', ('G02', 'G05'), 'handshake e credenciais concretas', 'libdbus autentica transporte e expõe evidência verificável', 'campo textual ou handshake incompleto', 'produtor autenticado sem depender de alegação do chamador', True),
    Guarantee('G07', 'Identidade do serviço', ('G01', 'G06'), 'GetNameOwner e credenciais do serviço', 'verificar resposta do daemon e serviço no gerenciador esperado', 'nome resolvido em broker errado', 'serviço/gerenciador vinculados, não apenas nome único', True),
    Guarantee('G08', 'Proprietário único contínuo', ('G07',), 'owner, NameOwnerChanged e epoch', 'subscrever troca antes da descoberta e invalidar ao perder origem', 'owner alterado/desconexão', 'mesmo proprietário na janela relevante'),
    Guarantee('G09', 'Subscrição confirmada', ('G05', 'G08'), 'ACK AddMatch e Subscribe antes do envio', 'validar origem/retorno na conexão dedicada', 'ACK ausente/tardio ou perda', 'subscrição válida anterior à emissão'),
    Guarantee('G10', 'Instância da unidade identificada', ('G07', 'G08'), 'nome, object path, InvocationID, cgroup e identidade do executor', 'snapshots autenticados e política de referência/substituição', 'nome reutilizado ou geração indisponível', 'escopo da identidade declarado sem inventar vínculo atômico ao job', True),
    Guarantee('G11', 'Snapshot anterior válido', ('G10',), 'snapshot prévio com origem e tempo local', 'validar contexto/geração/cgroup anterior ao envio', 'snapshot ausente/trocado/fora de ordem', 'identidade consistente antes do pedido'),
    Guarantee('G12', 'Emissão nativa comprovada', ('G06', 'G09', 'G11'), 'registro do envio efetivo no handle dedicado', 'futuro produtor nativo autorizado, ainda inexistente', 'intenção/serial declarado apresentados como envio', 'emissão efetiva comprovada pelo mecanismo produtor', True),
    Guarantee('G13', 'Serial da chamada comprovado', ('G12',), 'serial atribuído pela biblioteca no envio', 'capturar retorno send_message ou serial da mensagem efetivamente enviada', 'serial inventado ou de outra conexão', 'uint32 vinculado à mesma emissão/conexão', True),
    Guarantee('G14', 'Resposta correlacionada', ('G08', 'G13'), 'ReplySerial, remetente e caminho do job', 'validar cabeçalhos e janela sem confundir serial da resposta', 'outro owner/serial/janela ou resposta ausente', 'resposta da emissão específica'),
    Guarantee('G15', 'JobRemoved correlacionado', ('G09', 'G14'), 'uoss, job ID/path/unidade, origem e tempo', 'C1 com envelopes autenticados e eventos precoces preservados', 'job divergente/duplicatas conflitantes', 'mesmo job sem assumir InvocationID no sinal'),
    Guarantee('G16', 'Snapshot posterior válido', ('G10', 'G15'), 'estado e geração posteriores autenticados', 'verificar mesmo contexto, geração e cgroup ou diagnosticar ausência', 'not-found, substituição ou lacuna', 'estado posterior vinculado ou inconclusivo, nunca vazio inferido'),
    Guarantee('G17', 'Continuidade temporal', ('G08', 'G11'), 'timestamps locais e lacunas explicitamente registradas', 'preservar ordem original e janela, sem relógios fabricados', 'regressão temporal ou lacuna desconhecida', 'ausência de lacuna registrada no intervalo relevante'),
    Guarantee('G18', 'Ausência de perda de eventos', ('G09', 'G17'), 'contadores, overflow e continuidade da subscrição', 'detectar perdas e documentar limitações de detecção', 'evento perdido sem diagnóstico confiável', 'nenhuma perda relevante conhecida; detecção incompleta bloqueia certeza', True),
    Guarantee('G19', 'Limites de recursos atendidos', ('G05',), 'contadores Python, orçamento e isolamento nativo', 'validar entrada antes de cópias e auditar limites nativos separadamente', 'asdict, relatórios, callback ou operação bloqueante', 'contratos locais atendidos sem alegar RAM/prazo absoluto nativo', True),
    Guarantee('G20', 'População do cgroup observada', ('G10', 'G17', 'G18', 'G19'), 'amostras reais de cgroup e metadados B2', 'B2 avalia identidade, ordem, lacunas e janela', 'remoção sem zero ou amostra incompleta', 'somente B2 classifica a subárvore/janela observadas'),
    Guarantee('G21', 'Limpeza integral comprovada', ('G15', 'G16', 'G18', 'G19', 'G20'), 'evidências independentes de população e sobreviventes/escape', 'definir alcance da conclusão e obter provas adicionais hoje ausentes', 'done/vazio local tratados como ausência universal', 'não homologar limpeza universal com dados locais insuficientes', True),
)


def validate_graph(graph):
    if type(graph) is not dict or not 1 <= len(graph) <= 21:
        raise ValueError('grafo inválido ou excessivo')
    for key, deps in graph.items():
        if (type(key) is not str or len(key) != 3 or key[0] != 'G' or not key[1:].isdigit()
                or not 1 <= int(key[1:]) <= 21 or type(deps) is not tuple or len(deps) > 21
                or any(type(dep) is not str or dep not in graph for dep in deps)
                or len(set(deps)) != len(deps)):
            raise ValueError('referência/dependência inválida')
    active, done, ordered = set(), set(), []
    def visit(node):
        if node in active:
            raise ValueError('dependência circular')
        if node in done:
            return
        active.add(node)
        for dependency in graph[node]:
            visit(dependency)
        active.remove(node)
        done.add(node)
        ordered.append(node)
    for node in graph:
        visit(node)
    return tuple(ordered)


GRAPH = MappingProxyType({g.identifier: g.dependencies for g in GUARANTEES})
ORDER = validate_graph(dict(GRAPH))


@dataclass(frozen=True)
class Assessment:
    identifier: str
    status: GuaranteeStatus = GuaranteeStatus.NOT_EVALUATED
    category: Trust = Trust.DECLARED

    def __post_init__(self):
        if (self.identifier not in GRAPH or not isinstance(self.status, GuaranteeStatus)
                or self.status is GuaranteeStatus.OPERATIONALLY_PROVEN
                or self.category not in (Trust.DECLARED, Trust.SIMULATED) or not isinstance(self.category, Trust)
                or (self.status is GuaranteeStatus.SIMULATED and self.category is not Trust.SIMULATED)):
            raise ValueError('garantia operacional ou origem forjada')

    @property
    def operationally_proven(self):
        return False


class BudgetExceeded(ValueError):
    pass


@dataclass(frozen=True)
class Budget:
    max_depth: int = 8
    max_nodes: int = 2048
    max_report_nodes: int = 16384
    max_report_depth: int = 16
    max_items: int = 128
    max_string_bytes: int = 65536
    max_input_bytes: int = 65536
    max_history_bytes: int = 262144
    max_report_bytes: int = 262144
    max_reports: int = 4
    max_errors: int = 32

    def __post_init__(self):
        ceilings = (8, 2048, 16384, 16, 128, 65536, 65536, 262144, 262144, 4, 32)
        if any(type(getattr(self, f.name)) is not int or not 1 <= getattr(self, f.name) <= ceiling
               for f, ceiling in zip(fields(self), ceilings)):
            raise ValueError('orçamento só pode reduzir limites')


RECORD_TYPES = frozenset((b.Origin, b.ManagerContract, b.ObjectFixture, b.RuntimeContract,
                         b.UnitSnapshot, b.EmissionContract, b.ReplyEvidence, b.EventEvidence,
                         t.ManagerTarget, c1.StopReply, c1.JobRemoved, c1.UnitIdentity))
ENUM_TYPES = frozenset((b.Provenance, b.Verification, b.EmissionState, t.Manager))
INTEGER_FIELDS = frozenset(('uid', 'mode', 'device', 'inode', 'cgroup_inode', 'monotonic_ns',
                            'deadline_ns', 'serial', 'subscription_ns', 'reply_serial', 'job_id',
                            'response_monotonic_ns', 'signal_monotonic_ns', 'observed_monotonic_ns'))
BOOLEAN_FIELDS = frozenset(('continuous', 'symlink', 'accepted'))
FORBIDDEN_CLAIMS = frozenset(('AUTENTICADO_OPERACIONALMENTE', 'EMISSAO_NATIVA_COMPROVADA',
                             'PARADA_CONFIRMADA', 'POPULACAO_VAZIA_OBSERVADA',
                             'LIMPEZA_INTEGRAL_COMPROVADA'))


def bounded_tree(value, budget=Budget(), *, report=False):
    """Sem deepcopy/asdict/hooks arbitrários; aceita tipos exatos e limita antes de copiar."""
    if type(budget) is not Budget:
        raise BudgetExceeded('orçamento inválido')
    Budget.__post_init__(budget)
    nodes, payload_bytes = 0, 0
    stack = set()
    node_limit = budget.max_report_nodes if report else budget.max_nodes
    byte_limit = budget.max_report_bytes if report else budget.max_input_bytes
    def walk(item, depth=0, field_name=None):
        nonlocal nodes, payload_bytes
        nodes += 1
        if nodes > node_limit or depth > (budget.max_report_depth if report else budget.max_depth):
            raise BudgetExceeded('nós/profundidade excedidos')
        if field_name in INTEGER_FIELDS and item is not None and type(item) is not int:
            raise BudgetExceeded('campo numérico aninhado ou não inteiro')
        if field_name in BOOLEAN_FIELDS and type(item) is not bool:
            raise BudgetExceeded('campo booleano malformado')
        kind = type(item)
        if item is None or kind is bool:
            return item
        if kind is int:
            if not -(2**63) <= item < 2**64:
                raise BudgetExceeded('inteiro excessivo')
            return item
        if kind in ENUM_TYPES:
            if (kind is b.Provenance and item in (b.Provenance.AUTENTICADA, b.Provenance.OBSERVADA_NATIVAMENTE)):
                raise BudgetExceeded('proveniência reservada')
            if not report and item in (b.Verification.AUTENTICADO_OPERACIONALMENTE, b.EmissionState.EMISSAO_NATIVA_COMPROVADA):
                raise BudgetExceeded('estado reservado')
            return walk(item.value, depth + 1)
        if kind is str:
            if len(item) > budget.max_string_bytes:
                raise BudgetExceeded('string excessiva')
            encoded = len(item.encode('utf-8'))
            payload_bytes += encoded
            if encoded > budget.max_string_bytes or payload_bytes > byte_limit:
                raise BudgetExceeded('bytes de entrada excedidos')
            return item
        if kind not in RECORD_TYPES and kind not in (dict, list, tuple):
            raise BudgetExceeded('tipo não autorizado')
        if id(item) in stack:
            raise BudgetExceeded('objeto circular')
        stack.add(id(item))
        try:
            if kind in RECORD_TYPES:
                result = {f.name: walk(getattr(item, f.name), depth + 1, f.name) for f in fields(item)}
                # Tipos/tamanhos são verificados ANTES dos validadores de domínio.
                if kind is t.ManagerTarget:
                    canonical = t.ManagerTarget(item.manager, item.uid, item.runtime_dir, item.address)
                    if canonical != item:
                        raise BudgetExceeded('alvo não canônico')
                elif hasattr(kind, '__post_init__'):
                    kind.__post_init__(item)
                return result
            if len(item) > budget.max_items:
                raise BudgetExceeded('coleção excessiva')
            if kind is dict:
                result = {}
                for key, child in item.items():
                    if type(key) is not str:
                        raise BudgetExceeded('chave inválida')
                    if not report and key in FORBIDDEN_CLAIMS and child is not False:
                        raise BudgetExceeded('promoção operacional declarada')
                    result[walk(key, depth + 1)] = walk(child, depth + 1)
                return result
            return [walk(child, depth + 1) for child in item]
        finally:
            stack.remove(id(item))
    tree = walk(value)
    # Contabilidade lógica explícita, não medição do heap Python.
    return tree, payload_bytes + nodes * 64


class Boundary:
    """Máquina descritiva e gateway de fixtures; nunca cria backend ou capacidade nativa."""
    def __init__(self, manager, budget=Budget()):
        if type(manager) is not b.ManagerContract or type(budget) is not Budget:
            raise ValueError('contrato/orçamento obrigatório')
        bounded_tree(manager, budget)
        self.manager = manager
        self.budget = budget
        self._state = State.DISABLED
        self._invalidated = False
        self._contracts = b.ObservationalContracts(manager)
        self._errors = []
        self._error_count = 0
        self._history_bytes = 0
        self._reports = 0
        self._serialized_bytes = 0

    @property
    def state(self):
        if not isinstance(self._state, State) or self._state in RESERVED_STATES:
            self.invalidate('ESTADO_INTERNO_FORJADO')
        return self._state

    def invalidate(self, reason):
        self._invalidated = True
        self._state = State.INVALID if self._state is not State.CLOSED else State.CLOSED
        self._error_count += 1
        code = reason if type(reason) is str and 0 < len(reason) <= 128 else 'FALHA_MALFORMADA'
        if len(self._errors) < self.budget.max_errors:
            self._errors.append(code)
        self._contracts.fail(code)
        return False

    def transition(self, destination):
        if not isinstance(self._state, State) or self._state in RESERVED_STATES:
            return self.invalidate('ESTADO_INTERNO_FORJADO')
        if not isinstance(destination, State) or destination in RESERVED_STATES:
            return self.invalidate('PRODUTOR_NATIVO_INEXISTENTE_OU_ESTADO_FORJADO')
        allowed = {State.DISABLED: State.CONFIGURED, State.CONFIGURED: State.PREVALIDATED,
                   State.PREVALIDATED: State.SIMULATING}
        if destination is State.CLOSED:
            self._contracts.close()
            self._state = State.CLOSED
            return True
        if destination is State.INVALID:
            return self.invalidate('INVALIDACAO_EXPLICITA')
        if self._invalidated or allowed.get(self._state) is not destination:
            return self.invalidate('TRANSICAO_RECUSADA_SEM_RECUPERACAO_AUTOMATICA')
        self._state = destination
        return True

    def ingest(self, kind, value, *, category=Trust.SIMULATED):
        if self._state is not State.SIMULATING or self._invalidated:
            return self.invalidate('FRONTEIRA_NAO_SIMULANDO')
        if not isinstance(category, Trust) or category not in (Trust.DECLARED, Trust.SIMULATED):
            return self.invalidate('ORIGEM_NATIVA_FORJADA')
        try:
            _, cost = bounded_tree(value, self.budget)
            if self._history_bytes + cost > self.budget.max_history_bytes:
                raise BudgetExceeded('orçamento acumulado excedido')
            self._history_bytes += cost  # Cobrar também tentativa que falhe no contrato.
            if category is Trust.DECLARED:
                self._contracts.simulation_only = False
            expected = {'before': b.UnitSnapshot, 'after': b.UnitSnapshot,
                        'emission': b.EmissionContract, 'reply': b.ReplyEvidence, 'event': b.EventEvidence}
            if kind == 'runtime':
                if type(value) is not dict or not set(value) <= {'runtime', 'socket', 'previous_runtime', 'previous_socket', 'environment', 'socket_mode'}:
                    raise ValueError('fixture runtime inválida')
                self._contracts.record_runtime(**value)
            elif kind in expected and type(value) is expected[kind]:
                if kind in ('before', 'after'):
                    self._contracts.record_snapshot(kind, value)
                elif kind == 'emission':
                    self._contracts.declare_emission(value)
                elif kind == 'reply':
                    self._contracts.receive_reply(value)
                else:
                    self._contracts.receive_job_removed(value)
            else:
                raise ValueError('operação/contrato inválido')
            if self._contracts.broken:
                return self.invalidate('CONTRATO_INVALIDADO')
            return True
        except Exception as exc:
            return self.invalidate('ENTRADA_RECUSADA:' + type(exc).__name__)

    def close(self, *, failure=None):
        if failure is not None:
            self.invalidate('FALHA_FECHAMENTO')
        return self.transition(State.CLOSED)

    def assessment(self, identifier, status=GuaranteeStatus.SPECIFIED, category=Trust.DECLARED):
        try:
            return Assessment(identifier, status, category)
        except (ValueError, TypeError):
            self.invalidate('PROMOCAO_DE_GARANTIA_RECUSADA')
            raise

    def report(self, observed_until_ns):
        if self._reports >= self.budget.max_reports:
            self.invalidate('COTA_DE_RELATORIOS_ESGOTADA')
            raise BudgetExceeded('cota de relatórios esgotada')
        self._reports += 1  # Tentativas também consomem orçamento.
        if not isinstance(self._state, State) or self._state in RESERVED_STATES:
            self.invalidate('ESTADO_INTERNO_FORJADO')
        try:
            if type(observed_until_ns) is not int or not 0 < observed_until_ns < 2**64:
                raise ValueError('fim de observação inválido')
            model = self._contracts.result(observed_until_ns)
            if model.get('simulated_c1') and model['simulated_c1']['conflicting_events']:
                self.invalidate('CONFLITO_DE_EVENTOS')
            result = dict(state=self.state.value, invalidated=self._invalidated,
                          disposition='INVALIDADO' if self._invalidated else 'INCONCLUSIVO',
                          errors=list(self._errors), error_count=self._error_count,
                          history_logical_bytes=self._history_bytes,
                          native_producer_available=False, operational_guarantees_proven=False,
                          model=model)
            tree, _ = bounded_tree(result, self.budget, report=True)
            encoded = json.dumps(tree, ensure_ascii=False, separators=(',', ':'), allow_nan=False).encode('utf-8')
            if len(encoded) > self.budget.max_report_bytes:
                raise BudgetExceeded('relatório serializado excessivo')
            self._serialized_bytes += len(encoded)
            return encoded
        except Exception as exc:
            self.invalidate('RELATORIO_RECUSADO:' + type(exc).__name__)
            raise BudgetExceeded('relatório recusado') from exc


@dataclass(frozen=True)
class Threat:
    identifier: str
    condition: str
    impact: str
    existing_control: str
    missing_control: str
    synthetic_test: str
    residual_risk: str
    operational_block: str


THREATS = (
    Threat('A01', 'gerenciador errado', 'job atribuído ao alvo errado', 'ManagerTarget e namespace', 'identificar gerenciador real', 'T01,T02', 'seleção não autentica serviço', 'G01/G07'),
    Threat('A02', 'UID divergente', 'contexto de outro usuário', 'UID sintático e comparação de contrato', 'credenciais efetivas do processo/peer', 'T03', 'UID declarado não é credencial', 'G02/G06'),
    Threat('A03', 'runtime substituído', 'inspeção perde validade', 'device/inode em fixtures', 'verificação real e vínculo com conexão', 'T04', 'TOCTOU residual', 'G03/G04'),
    Threat('A04', 'socket substituído', 'peer não pretendido', 'comparação de identidade sintética', 'objeto/peer autenticados', 'T05', 'path não identifica peer', 'G04/G06'),
    Threat('A05', 'symlink inesperado', 'redirecionamento', 'política sintética recusa symlink', 'resolução segura de ancestrais', 'T06', 'namespace pode mudar', 'G03/G04'),
    Threat('A06', 'corrida inspeção/conexão', 'objeto verificado difere do conectado', 'limitação explícita', 'mecanismo demonstrado de vínculo ao peer', 'T10', 'duas leituras não eliminam corrida', 'G04/G06'),
    Threat('A07', 'owner alterado', 'eventos de outra instância do serviço', 'comparação e invalidação sintéticas', 'sinais autenticados contínuos', 'T14', 'evento de troca pode ser perdido', 'G08/G18'),
    Threat('A08', 'conexão reaproveitada', 'mistura de epochs', 'namespace por conexão', 'posse exclusiva de handle real', 'T15', 'epoch textual é declarado', 'G05/G06'),
    Threat('A09', 'subscrição ausente', 'resultado perdido', 'C1/C2-B recusam confirmação', 'ACK real AddMatch/Subscribe', 'T21', 'mocks não comprovam ACK', 'G09'),
    Threat('A10', 'subscrição tardia', 'evento anterior não capturado', 'tempo anterior ao pedido obrigatório', 'tempo local ligado ao ACK', 'T22', 'timestamps podem ser declarados', 'G09/G17'),
    Threat('A11', 'serial falso', 'resposta atribuída a envio inexistente', 'emissão apenas declarada; operacional fechado', 'captura nativa na mesma conexão', 'T23,T24', 'formato correto não prova envio', 'G12/G13'),
    Threat('A12', 'ReplySerial errado', 'resposta de outro pedido', 'C1 compara serial da chamada', 'cabeçalhos autenticados', 'T26', 'correlação de fixture é simulada', 'G14'),
    Threat('A13', 'outro remetente', 'resposta forjada', 'checagem de owner/conexão', 'autenticação dos headers no transporte', 'T27', 'campo sender é declarado na fixture', 'G07/G14'),
    Threat('A14', 'job de outra unidade', 'falso resultado de parada', 'C1 compara ID/path/unidade', 'origem nativa e vínculo de instância', 'T32', 'nome pode ser reutilizado', 'G10/G15'),
    Threat('A15', 'job de outra conexão', 'mistura entre epochs', 'comparação de conexão/contexto', 'lifecycle nativo exclusivo', 'T33', 'conexão declarada não é handle', 'G05/G15'),
    Threat('A16', 'nome reutilizado', 'unidade substituída', 'snapshot compara geração/cgroup', 'evidência da instância real', 'T34', 'nome/object path reutilizáveis', 'G10'),
    Threat('A17', 'InvocationID diverge', 'outra ativação', 'C2-B invalida', 'snapshot autenticado', 'T35', 'JobRemoved não possui InvocationID', 'G10/G16'),
    Threat('A18', 'snapshot fora de ordem', 'cadeia temporal inválida', 'ordem/janela verificadas', 'timestamps do produtor nativo', 'T37', 'ordem válida não elimina lacunas', 'G11/G16/G17'),
    Threat('A19', 'evento perdido', 'janela incompleta', 'invalidação explícita', 'detecção de perda demonstrada', 'T48', 'perda silenciosa pode ser indetectável', 'G18'),
    Threat('A20', 'overflow', 'evidência truncada', 'prefixo preservado e resultado inválido', 'dimensionamento operacional', 'T53', 'truncamento não significa ausência', 'G18/G19'),
    Threat('A21', 'callback GLib externo', 'despacho/bloqueio inesperado', 'orçamento cooperativo documentado', 'isolamento de contexto/processo auditado', 'T56', 'conexão dedicada não isola callbacks', 'G19'),
    Threat('A22', 'operação nativa bloqueia', 'prazo excedido', 'nenhuma operação nativa nesta camada', 'contenção de vida do cliente', 'T57', 'timeout não preempta biblioteca/kernel', 'G19'),
    Threat('A23', 'memória libdbus sem limite', 'memória fora do orçamento Python', 'limites locais explícitos', 'isolamento/medição nativa', 'T58', 'bytes lógicos não são heap/RSS', 'G19'),
    Threat('A24', 'operação e close falham', 'causa original ocultada', 'diagnósticos limitados preservam prefixo', 'liberação real de recursos comprovada', 'T59', 'referência removida não prova fechamento', 'G05/G19'),
    Threat('A25', 'simulação alega origem operacional', 'promoção indevida', 'categorias/estados reservados recusados', 'produtor nativo autenticado ausente', 'T41,T42,T45', 'Python arbitrário não é sandbox', 'G06/G12'),
    Threat('A26', 'entrada excessiva/malformada', 'cópias e consumo não limitados', 'gateway recursivo antes de asdict', 'limitar alocação no produtor nativo', 'T51,T52,T54,T55', 'objeto já criado pelo chamador não é desfeito', 'G19'),
    Threat('A27', 'observação incompleta vira limpeza', 'homologação sem prova', 'saída operacional sempre inconclusiva', 'evidência real suficiente de população/escape', 'T50,T60', 'vazio local não prova ausência universal', 'G20/G21'),
)

GLIB_POLICY = MappingProxyType(dict(dedicated_connection_is_dedicated_context=False,
                   dedicated_context_is_dedicated_thread=False,
                   dedicated_thread_is_process_isolation=False,
                   dbus_python_1_3_2_nondefault_context_supported=False,
                   current_context='GLib padrão; compartilhável',
                   dispatch_iterations=32, cooperative_seconds=0.05,
                   absolute_native_deadline_proven=False, total_native_memory_bound_proven=False,
                   future_requirement='auditar processo/contexto isolado e vida do cliente; sem implementar aqui',
                   operational_activation_allowed=False))
