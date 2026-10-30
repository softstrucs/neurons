# EC-08 — Auditoria dos monitores existentes

## Escopo e aceitação

Base `f2b9ca80584c41b7359c36a01580ac270aa5f125`, devs inicialmente limpa e sincronizada. O plano v0.2 denomina a primeira subetapa **auditar monitores existentes**. Não fixa para ela um protocolo ou limiar experimental específico. O critério da EC-08 como etapa inteira é monitoramento útil sobre processos efetivos, não apenas mensagens ou estados predefinidos; a auditoria não equivale a uma demonstração global desse aceite.

Inspeção técnico-científica de código e testes existentes e das auditorias EC-03–07, sem implementação ou experimento novo. Distinguem-se comportamentos verificados no código, resultados históricos e competências ainda não demonstradas. Próxima subetapa nominal: **definir estimativas de incerteza e erro**, não iniciada.

## Mecanismos e evidências rastreáveis

| Aspecto | Mecanismo e evidência | Limitação |
|---|---|---|
| Processo monitorado | [`Monitor::assess`](../src/metacognition.rs) chama `TwoHopTask::evaluate` e recebe `InferenceReport`, incluindo resultado e eventos processados. Teste `sufficient_evidence_yields_auditable_response_without_revision`. | É monitoramento de execução efetiva da tarefa delimitada de dois passos, com regras fixas; não supervisiona todos os processos do sistema. |
| Falta de suporte | `Outcome::Unsupported` vira `Assessment::MissingEvidence` e pode levar a `AbstainReason::MissingEvidence`. Teste `missing_evidence_and_resource_exhaustion_have_distinct_reasons`. | Ausência de caminho suportado sob o contrato não significa inexistência universal de conhecimento nem probabilidade calibrada de erro. |
| Contradições | `find_conflict` verifica negativa da conclusão e das premissas da prova selecionada; `Conflict` registra IDs positivos/negativos. Teste `contradictory_conclusion_and_premise_are_detected_by_record_id`. | A procura ocorre quando há evidência positiva selecionada; não é exame exaustivo de todas as vias ou resolução geral de conflitos e validade temporal. |
| Limites e falhas | Orçamento de até oito tentativas; motivo `ReviewBudget`; eventos e fatos limitados, propagação e dados inválidos geram motivos distintos. Testes de orçamento e fila em `tests/metacognition.rs`. | Orçamentos são configurados por quem chama. Não há estimativa geral de tempo, memória disponível, custo-benefício ou calibração de confiança. |
| Revisão | `review` reavalia a consulta em contextos alternativos fornecidos; resposta revisada exige exatamente um candidato sustentado, e múltiplos candidatos levam a abstenção. Testes de revisão e ambiguidade. | Não descobre contextos, não altera a regra nem aprende estratégia; mudar o contexto de resposta não é revisão persistente da memória. Aplicabilidade semântica dos contextos depende de informação externa. |
| Registro | `ReviewReport` guarda consulta, decisão e `AttemptLog` por tentativa, inclusive avaliação, inferência, erro, evidência e conflito; teste de repetibilidade compara relatórios. | Relatório retornado em memória não é histórico persistente de todas as atividades nem auditoria independente da verdade da fonte. |
| Interrupção e ajuda | Retornos e abstenções interrompem a revisão diante de limites/falhas; `src/session.rs` limita comandos e tamanho de entrada. | Não foi identificado pedido autônomo de ajuda ou mecanismo integrado de autorização para ações externas; não inferir inexistência em todo ambiente a partir de busca local. |
| Outros monitores delimitados | `src/languages` e suportes experimentais registram ausência, conflito, ambiguidade e orçamento; [EC-05](EC-05_auditoria_cientifica_final.md) examina manutenção/recomposição P1/P2; [EC-07](EC-07_auditoria_cientifica_final.md) usa guardas e avaliadores externos. | Protótipos e avaliadores de `tests/` não são capacidades integradas ao monitor de produção. Não presumir política universal ou autorregulação adaptativa. |

A [auditoria EC-06](EC-06_auditoria_cientifica_final.md) reproduziu concordância 48/48 entre monitor e controle simbólico em quatro perfis, incluindo 24 abstenções adequadas; fatos, regra, topologias e pressupostos permanecem delimitados. O teste histórico de sete casos em `tests/metacognition.rs` registra monitor 7/7 versus inferência bruta 5/7; o controle não recebe contexto alternativo no caso de revisão, portanto a comparação não demonstra superioridade com informação equivalente. Esses são resultados anteriores, não novas execuções desta auditoria.

## Diagnóstico e dependências

Há detecção programada de algumas falhas e revisão condicional de respostas com registro local. Isso é evidência funcional limitada de monitoramento do próprio processo de inferência, não prova de metacognição geral. Não foram demonstradas autorregulação adaptativa de estratégias, aprendizagem metacognitiva, estimativa calibrada de incerteza, reconhecimento geral de competência ou autoconhecimento funcional abrangente. `Monitor` lê relatórios e evidências, mas não aprende seus critérios nem valida autonomamente fontes externas.

EC-04 demonstrou aquisição simbólica de critérios em família fornecida, com ressalvas de controles e instrumentação; EC-05 demonstrou revisão contextual em protótipos com diferenças entre manutenção e recomposição; EC-06 avaliou composição com regras fornecidas; EC-07 avaliou seleção programada e cálculo informacional externo, sem ganho persistente de competência. Nada disso demonstra aprendizagem de monitoramento ou que escolher investigar cause aprendizagem posterior. As conclusões anteriores não são reabertas.

Para avaliações futuras permanecem necessidades de critérios de referência independentes, distinção de erro e falta de evidência, medição de calibração, custo e utilidade, alternativas com informação equivalente, registro histórico e limites de acesso verificáveis. Não se escolhem estimadores, arquitetura, protocolo ou limiares nesta subetapa.

## Diretrizes e verificações

A1/A2 orientam memória, proveniência e revisão; os relatórios do monitor não demonstram formação autônoma de engramas. A3/A4 orientam pedagogia e comunicação honesta de limitações e investigações efetivamente realizadas; pedido autônomo de ajuda ou investigação não foi demonstrado. A5/A6 orientam pt-BR e expansão multilíngue, fora desta avaliação. A7 exige evidências e controles; A8 inclui reconhecimento de limites e competência generalizável, não demonstrados integralmente por este monitor. Fundamento e A1–A8 preservados.

Foram executadas verificações documentais, de integridade e Git; nenhum teste ou experimento novo foi executado. Os testes mencionados são históricos. A primeira subetapa é concluída como auditoria documental delimitada; a aceitação global da EC-08 permanece não avaliada.
