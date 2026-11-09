# NEURONS

Núcleo experimental em Rust para neurônios, sinapses, engramas e evolução cognitiva verificável.

**Roteiro único proposto:** [NEURONS_PLANO_MESTRE.md](NEURONS_PLANO_MESTRE.md).
Sua elaboração não autoriza implementação, experimentos ou acesso reservado.

- [Documentação e referências](docs/README.md).
- [Planos executivos históricos, preservados](docs/historico/planos_executivos/README.md).
- [Relatório de recuperação e limites](docs/referencia/RECUPERACAO_ESTRUTURAL.md).

Pacote local sem dependências externas de crates: `cargo check --offline --lib --bin neurons`.
Testes/painéis históricos e dados reservados exigem escopo próprio; não executar `cargo test`
indiscriminadamente: há fixtures reservadas em alvos científicos. A CLI oferece `--help`,
`doctor`, `session`, `session-explain` e **`session-f1`**.

A implementação técnica F1 integra os coordenadores existentes de consulta, aquisição explícita
e revisão conceitual H23. Inicie `cargo run --offline --locked --bin neurons -- session-f1` e
use `ajuda`. Gramática estruturada, fontes sintéticas declaradas e limites finitos; sem linguagem
natural, rede ou compreensão geral demonstrada. Os comandos anteriores mantêm seu comportamento.

`salvar CAMINHO` cria exclusivamente um checkpoint F1 v1; nunca substitui arquivo existente.
`restaurar CAMINHO` restaura fatos, proveniência, pendências, hipóteses, versões e histórico.
Circuitos, pesos e potenciais neuronais não fazem parte desse contrato. Detalhes, comandos de
regressão seguros, limitações e evidências estão no [Plano Mestre](NEURONS_PLANO_MESTRE.md#9-registro-de-progresso-e-próxima-ação).
O aceite funcional final permanece pendente do responsável.
