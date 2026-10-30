# EC-07 — Formular hipóteses operacionais

## Escopo

Base `dd1f998f48c6737336bf8e592e2e5e74496e8335`, devs inicialmente limpa e sincronizada. O plano v0.2 nomeia esta segunda subetapa **formular hipóteses operacionais**, sem definir formato, dados, implementação ou limiar experimental próprio. A aceitação da EC-07 como um todo exige demonstrar que escolher investigar contribui para aprendizagem posterior, sem depender de perguntas em linguagem natural. A presente especificação não executa essa comparação nem conclui a etapa inteira. Continuidade do [levantamento de lacunas](EC-07_detectar_lacunas.md), com fundamentos e diretrizes A1–A8, decisões e auditorias EC-04/05/06 preservados.

## Definições e critérios

Uma **hipótese operacional investigável** é uma proposição candidata delimitada que responde a uma pergunta ligada a uma lacuna e admite observações possíveis que, sob pressupostos e método declarados, distinguem suporte de oposição ou inconclusão. O qualificativo operacional descreve a possibilidade de teste especificado, não uma verdade, implementação existente ou autonomia do produtor.

| Elemento | Requisito para rastreabilidade e verificação |
|---|---|
| Lacuna | Identificador, objetivo e razão observada da insuficiência; distinguir ausência de suporte, conflito, erro e limite de recursos. Não inferir inexistência de conhecimento. |
| Pergunta | Relação com a lacuna, escopo e contexto; pode ser estruturada e não precisa ser formulada em linguagem natural. |
| Candidata | Proposição, identidade, versão, origem e transformação que a produziu; termos e domínio definidos, sem atribuição prematura de verdade. |
| Suporte e oposição | Referências a evidências acessíveis e suas fontes, validade e contexto; separar observação de inferência e pressupostos. |
| Previsões | Consequências observáveis sob condições especificadas, incluindo resultado contrário ou divergente e alternativas que possam explicar a observação. |
| Teste | Método reproduzível e controlável, limitações, acesso permitido, recursos e forma de registrar observação; método impossível ou não autorizado não é investigação executada. |
| Avaliação | Critérios prévios de suporte, oposição/refutação sob pressupostos, revisão e inconclusão; distinguir falta de observação, falha instrumental e evidência contrária. |
| Estado e histórico | Candidata, apoiada no escopo, contrariada, revisada, rejeitada ou suspensa/inconclusiva com justificativa; preservar versões, dependências e resultados anteriores. Estado de confiança, quando usado, deve ter definição e calibração justificadas. |

Esses são requisitos conceituais para futuras avaliações, não contrato público, estrutura universal nem política geral de atualização. Evidência favorável não comprova verdade universal; resultado contrário pode questionar pressupostos ou medição em vez da proposição isolada. Falha de orçamento, permissão ou medição determina inconclusão quando impede uma comparação válida.

## Origem e capacidades atuais

| Origem ou mecanismo | O que pode ser afirmado | O que permanece não demonstrado |
|---|---|---|
| Fornecida por usuário ou fonte | `Hypothesis::new(context, from, to)` recebe uma consulta estruturada; `session` e `Microcurriculum` a constroem de entradas fornecidas. | A estrutura contém contexto e extremos, mas não necessariamente previsão, método, refutação ou proveniência completa. Fonte externa não valida a proposição. |
| Gerada por regra programada | `Monitor::review` aplica a mesma consulta a contextos candidatos fornecidos; EC-04 enumera e elimina candidatos de uma família de critérios em suportes de teste. | Variação de contexto ou enumeração em família predeterminada deve ser descrita como operação programada; não é descoberta autônoma nem construção da família. |
| Derivada por inferência | `TwoHopTask` retorna prova de duas relações e referências; EC-06 verificou casos delimitados com regra e fatos fornecidos. | A conclusão derivada pode fundamentar candidata a investigar, mas a implementação não fornece automaticamente um método e previsão refutável. Não se aprende a regra no painel. |
| Adaptativa | EC-04 registra mudança simbólica delimitada de critérios por exemplos, com controles e limitações de sua auditoria. | Não há evidência de mecanismo integrado que construa hipóteses investigáveis abertas e selecione testes com ganho causal de aprendizagem. Alterar estado por si só não estabelece essa competência. |

`Engram` permite status `Hypothesis`, `Observed` ou `Validated` e referência de evidência; `Validated` exige ID de evidência, mas isso não verifica por si a validade científica. `Monitor` retorna avaliação, referências, conflito, erro e tentativas, não um plano de pesquisa persistente. A seleção contextual usa alternativas fornecidas. EC-05 modela suporte e revisão em protótipos delimitados; não integra política universal. Consultar [cognição](../src/cognition.rs), [metacognição](../src/metacognition.rs), [memória](../src/engram.rs), [critérios EC-04](../tests/support/ec04_candidate.rs) e [auditoria EC-06](EC-06_auditoria_cientifica_final.md).

Recuperar hipótese registrada não é gerá-la. Reordenar opções cadastradas, substituir nomes ou reformular texto pode produzir uma nova instância sintática, sem demonstrar geração cognitiva autônoma. Para alegação futura de geração substantiva, exigir rastreio da origem, comparação com inventário prévio e controles literal e de regras com acesso equivalente, distinção entre novidade de rótulo, combinação e estrutura, além de teste independente da utilidade e limites. Mesmo candidatos novos não comprovam descoberta verificada ou aprendizagem. Não confundir autonomia operacional limitada com autonomia cognitiva geral.

## Exemplos conceituais, não executados

| Caso | Registro proposto e resultado possível | Demonstração e limite |
|---|---|---|
| Positivo delimitado | Lacuna: não há suporte para relação `a→c` em contexto C. Pergunta: há caminho `a→b→c` válido em C? Candidata: existem essas duas relações. Previsão: consulta autorizada encontra ambos os registros válidos; oposição: verificação confiável mostra ausência de pelo menos um sob cobertura definida; dados inacessíveis → inconclusão. | Mostra como formular hipótese refutável dentro de inventário e cobertura especificados; não demonstra causalidade, compreensão ou que a ausência de registro implica ausência no mundo. |
| Negativo | Recuperar uma frase cadastrada ou trocar `a` por `d` e chamá-la descoberta sem previsão, fonte ou teste. | Pode demonstrar recuperação ou reformulação; não satisfaz o registro investigável nem demonstra autonomia. |
| Ambíguo | Dois suportes conflitam e uma fonte está indisponível ou expirada; teste não distingue erro de fonte de mudança de contexto. | Preservar conflito e suspensão/inconclusão; não escolher uma conclusão por recência nem tratar a indisponibilidade como refutação. |
| Resultado adverso | A previsão falha sob condições controladas, mas pressuposto de validade temporal também falha. | Registrar oposição à previsão e alternativa explicativa; rejeitar a hipótese isolada somente se o método e pressupostos autorizarem essa inferência. |

Esses exemplos não são conjunto experimental, resultados observados ou protocolo congelado. Fontes, critérios de cobertura e validade seriam necessários antes de considerar uma oposição refutação. A distinção de contextos/versões e histórico requer vínculo explícito; não se herda automaticamente evidência entre versões.

## Diretrizes e continuidade

A1/A2 orientam memória evolutiva, fontes, evidências e revisões, sem pressupor engramas aprendidos autonomamente. A3/A4 exigem aprendizagem pedagógica e comunicação honesta: uma intenção ou etiqueta de investigação não é pesquisa efetivamente executada. A5/A6 orientam pt-BR e futura expansão multilíngue, sem exigir linguagem natural para este contrato. A7 exige refutabilidade e controles; A8 orienta investigação e transferência futuras, sem certificá-las. Todas as diretrizes permanecem inalteradas.

Para próximas subetapas permanecem decisões sobre catálogo de intervenções, permissões e custos, controles com informação equivalente, métrica de ganho e avaliação independente de aprendizagem. Não se escolhe política de seleção, orçamento ou protocolo experimental aqui. A próxima subetapa nominal é **selecionar intervenções sob orçamento**, não iniciada.

Inspeção e verificações documentais nesta entrega; os testes e resultados citados são históricos. Nenhum novo teste, implementação ou experimento foi criado ou executado. Preservam-se as ressalvas das auditorias EC-04/05/06, inclusive controles com pressupostos comuns, limites de proveniência e instrumentação, e ausência de generalização ou aprendizagem neural demonstradas.
