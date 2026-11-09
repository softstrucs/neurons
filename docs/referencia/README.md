# Referência técnica

Planejamento ativo: [Plano Mestre proposto](../../NEURONS_PLANO_MESTRE.md).

| Tema | Fonte canônica e limite |
|---|---|
| Núcleo público | [módulos](../../src/lib.rs), [motor](../../src/engine.rs), [circuito](../../src/circuit.rs); artefatos computacionais, sem alegação cognitiva geral |
| Memória/estado | [EngramStore](../../src/engram.rs), [persistência](../../src/persistence.rs), [integração](../../src/integration.rs); checkpoint atual não cobre Circuit completo |
| Inferência/metacognição | [TwoHopTask](../../src/cognition.rs), [Monitor](../../src/metacognition.rs), [consulta tipada](../../src/integration/query.rs) |
| Aquisição/conceitos | [aquisição](../../src/integration/acquisition.rs), [ConceptCoordinator](../../src/integration/concepts.rs); revisão simbólica limitada, não aprendizagem neural comprovada |
| Linguagem/sessão | [léxico](../../src/languages.rs), [sessão restrita](../../src/session.rs); sem português livre |
| Evidência histórica | [decisões](../decisions.md), [M5](../protocolos/M5_execucao_auditoria_primeiro_ciclo_cognitivo_integrado.md), [CIC-I1](../protocolos/CIC-I1_contratos_mecanismo_conceitual.md) |
| Limites científicos | [EC-12](../EC-12_inventario_competencias_demonstradas.md), [EC-11](../EC-11_auditoria_cientifica_final.md), [bloqueio CIC-I3](../protocolos/CIC-I3_bloqueio_validacao_V.md) |
| Custódia | [P2](../protocolos/CIC-I6-P2_matriz_papeis_permissoes.json), [RC-01](../protocolos/CIC-I6-RC01_criterios_aceitacao_pendencias.md); G2/G7/DVF não autorizados |

Fontes úteis são referenciadas, não copiadas ou reinterpretadas como permissões.
`docs/architecture.md` sugerido no plano antigo não existe; a visão consolidada está no Plano Mestre.
