# EC-12 — Inventariar competências demonstradas

Base `35c10b7503aea3b360228b08b353ecf1faa63962`, branch `devs`, árvore limpa e remoto sincronizado. Inventário técnico-científico documental, classificação **AJUSTAR**: o termo competência é usado apenas para comportamento delimitado sustentado por evidência, não certificação de capacidade cognitiva geral. Não há novo experimento nem reabertura de etapas; a EC-12 permanece aberta.

## Plano, objetivo e método

O [Plano Executivo v0.2](../neurons_plano_executivo_evolucao_cognitiva_v0_2.md) apresenta na tabela o objetivo **“Documentar maturidade e lacunas”**, dependência **“EC-11”**, entregável **“Relatório de capacidades e limites”** e critério resumido **“Decisão humana de continuidade”**. Na especificação nominal, a etapa é **“EC-12 — Consolidação, especialização futura e questões abertas”** e suas subetapas são **“inventariar competências demonstradas; avaliar lacunas; definir pré-requisitos de especialização; registrar questões abertas, incluindo consciência artificial como investigação distinta e não pré-requisito.”** O aceite literal é **“relatório científico e decisão explícita sobre próximas linhas de pesquisa, sem alegar experiência subjetiva.”** O plano não fixa quotas específicas para o inventário. Executa-se somente a primeira subetapa; avaliar lacunas, definir pré-requisitos e registrar questões abertas são posteriores, não realizadas aqui. O critério global EC-12 não foi homologado.

Método: ler plano, decisões, auditorias e interfaces; registrar para cada capacidade a fonte, mecanismo, observação e limite. **Implementado** é código existente; **demonstrado no escopo** exige evidência especificada; **parcial** tem condições e/ou conformidade limitadas; **não demonstrado** é ausência de evidência suficiente; **não avaliável** indica falta da interface, instrumento ou condição no ensaio considerado. Um comportamento observado em suporte experimental não é atribuído ao núcleo. Esta classificação preserva os pareceres originais e não reavalia seus critérios globais.

## Inventário rastreável EC-00 a EC-11

| Etapa / evidência | Comportamento ou entrega documentada | Classificação e limites |
|---|---|---|
| EC-00 — [plano, linha de base](../neurons_plano_executivo_evolucao_cognitiva_v0_2.md) | Diagnóstico de infraestrutura e fundamentos. | Documentação de base; não demonstra competência cognitiva. |
| EC-01 — [plano, EC-01](../neurons_plano_executivo_evolucao_cognitiva_v0_2.md) | Análise de coatividade, associação e causalidade; EC-01/H1 preservada sem implementação. | Analítica, não prova atribuição causal de crédito; textos integrais históricos apontados como pendentes no plano. |
| EC-02 — [decisões, EC-02.4](decisions.md) e [plano](../neurons_plano_executivo_evolucao_cognitiva_v0_2.md) | Expectativa contextual simbólica delimitada e revisão em painel experimental; recuperação literal superou o resumo em P4. | Parcial; controles e metodologia limitados, sem descoberta causal autônoma nem vantagem neural. O plano registra protocolos/relatórios históricos integrais pendentes de arquivo; não inferir pré-registro independente. |
| EC-03 — [auditoria experimental](EC-03.5_auditoria_experimental.md) | Manutenção simbólica de representações e provas, com controles e cenários delimitados. | AJUSTAR, conformidade parcial; histórico e instrumentação incompletos, autoria e pressupostos compartilhados. Protótipos não são automaticamente produção. |
| EC-04 — [auditoria](EC-04_auditoria_cientifica_final.md) | Aquisição e revisão simbólica de critérios em família fornecida e exemplos reservados; recuperação e composição restritas. | AJUSTAR, parcial; controle literal T-C incompleto, histórico contextual e instrumentação parciais; não é aprendizagem neural ou formação autônoma de conceitos. |
| EC-05 — [auditoria](EC-05_auditoria_cientifica_final.md) | Diagnóstico, conservação e revisão contextual simbólica delimitados sob contratos fornecidos. | AJUSTAR, parcial; dez diferenças pré-manutenção P1/P2 e custos parciais não demonstram política cognitiva geral ou interferência sináptica. |
| EC-06 — [auditoria](EC-06_auditoria_cientifica_final.md) | Composição de dois passos com fatos e regra fornecidos, referências e eventos, em casos restritos. | AJUSTAR, parcial; controle simbólico equivalente e topologias fixas não demonstram vantagem neural, descoberta ou transferência estrutural. |
| EC-07 — [auditoria](EC-07_auditoria_cientifica_final.md) | Políticas programadas e controle pseudoaleatório; avaliador externo mede redução de hipóteses. | AJUSTAR, parcial; previsões fornecidas e controles compartilham pressupostos; sem evidência causal de aquisição persistente por investigação. |
| EC-08 — [auditoria](EC-08_auditoria_cientifica_final.md) | Monitor observa inferência real, detecta algumas falhas, revisa com contexto fornecido e abstém-se sob regras. | AJUSTAR, parcial; benefício incremental nulo frente ao controle equivalente, calibração probabilística não mensurável e custos não isolados. |
| EC-09 — [auditoria](EC-09_auditoria_cientifica_final.md) | Diagnóstico, pré-requisitos, experiências e progressão especificados documentalmente. | AJUSTAR, parcial; ciclo pedagógico integrado e melhora sustentada não demonstrados, aceite global não homologado. |
| EC-10 — [auditoria](EC-10_auditoria_cientifica_final.md) | Atualização simbólica delimitada e preservação funcional restrita; testes de partição e renomeação. | AJUSTAR, parcial; R2 abstenção→acerto não substitui resposta antiga correta; R3 pré-verificação falhou (20/28 invariantes positivas), painel final ignorado e não executado. Sem transferência genuína ou cumulatividade homologadas. |
| EC-11 — [auditoria](EC-11_auditoria_cientifica_final.md) | `IntegratedState` reúne contratos de motor, memória, monitor e checkpoint; cenário fixo mantém bytes, decisão e fila com restauração em processo separado. | AJUSTAR, parcial; H1 não compara relatórios completos entre repetições; checkpoint não serializa circuito/pesos/estado neuronal nem configuração do monitor. Critério parcialmente atendido no cenário; confiabilidade geral não homologada. |

## Capacidades de produção versus evidências

| Código / interface | Estado e evidência | O que não demonstra |
|---|---|---|
| [Motor e eventos](../src/engine.rs), [circuito](../src/circuit.rs), neurônios e sinapses | Implementados; ordem, limites e propagação em [testes](../tests/engine.rs). | Atividade neuronal não prova aprendizagem neural ou competência geral. |
| [EngramStore](../src/engram.rs) e [persistência](../src/persistence.rs) | Implementados; registro, recuperação exata e checkpoint limitado em [testes](../tests/memory.rs) e [persistência](../tests/persistence.rs). | Referência de evidência não autentica sua qualidade; dados persistidos não provam retenção cognitiva, revisões gerais ou estado neuronal. |
| [TwoHopTask](../src/cognition.rs) e [Monitor](../src/metacognition.rs) | Implementados; recuperação, inferência com regra fornecida, algumas contradições e abstenção delimitadas. | Não demonstram raciocínio generalizável, investigação autônoma, calibração probabilística ou revisão autônoma de conhecimento. |
| [Feedback](../src/learning.rs), [Microcurriculum](../src/curriculum.rs) e [linguagem](../src/languages.rs) | Implementados para ajustes explícitos e entradas fornecidas; capacidades locais testadas. | Feedback externo e registros não demonstram aprendizagem pedagógica autônoma, domínio de pt-BR ou especialização aprendida. |
| [IntegratedState](../src/integration.rs) | Implementado; integração e restauração de contratos em [testes](../tests/ec11_integration.rs) e [painel](../tests/ec11_reliability.rs). | Persistência apenas de motor/memória; monitor reconfigurado pelo chamador, sem continuidade neuronal ou garantia global de permissões. |

Não demonstrados no conjunto inventariado: aprendizagem pedagógica contínua, transferência genuína entre domínios, cumulatividade, autonomia cognitiva geral, competência generalizável A8, consciência ou compreensão geral. Continuidade de circuito/pesos após reinício e política integrada de permissões são **não avaliáveis pelo contrato atual**; isto não prova impossibilidade. Configuração ou seleção de um agente/estilo não equivale a especialização adquirida; nenhuma especialização autônoma é atribuída aqui.

## Limites e fundamentos

Resultados negativos, inconclusivos e controles insuficientemente independentes acima são preservados. Arquivamento incompleto da EC-01/EC-02 é limite de rastreabilidade; relatórios existentes e decisões não substituem os textos integrais ausentes. Repetições e testes duplicados não são amostras independentes. Dependências observáveis neste inventário incluem referência externa, controle independente, rastreabilidade de versões, instrumento de recursos e evidência longitudinal; sua avaliação ou priorização pertence às subetapas posteriores.

A1–A2 orientam memória evolutiva, associação e proveniência; A3–A4 formação e investigação efetivas; A5–A6 pt-BR e expansão multilíngue; A7 rigor; A8 competência generalizável futura. Permanecem diretrizes preservadas, não capacidades homologadas. A ausência de evidência não prova impossibilidade geral ou experiência subjetiva.

Primeira subetapa concluída como inventário documental delimitado, **AJUSTAR**; EC-12 permanece aberta. Próxima subetapa nominal: **avaliar lacunas**, não iniciada.

## Verificações desta subetapa

`cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` passaram. Regressão histórica: **185 testes aprovados, zero falhos e um ignorado**; `r3_frozen_final` EC-10 foi ignorado pela quota insuficiente, não aprovado. A suíte inclui auxiliar inerte e duplicações históricas; não são novos experimentos EC-12. Três avisos históricos EC-04 de métodos não utilizados. Links locais conferidos; `/home` observado em aproximadamente 4,4 GiB livres, sem monitoração contínua.
