# EC-07 — Selecionar intervenções sob orçamento

## Escopo e situação

Base `f2eac683397b50669526142838908587ec9d28a6`, devs inicialmente limpa e sincronizada. O plano v0.2 nomeia a terceira subetapa **selecionar intervenções sob orçamento**, sem determinar algoritmo, orçamento numérico, experimento ou política definitiva. Esta especificação estabelece critérios e possibilidades; não implementa seletor, executa intervenção ou satisfaz automaticamente o critério global da EC-07: mostrar que escolher investigar contribui para aprendizagem posterior, sem depender de perguntas em linguagem natural.

A [detecção delimitada de lacunas](EC-07_detectar_lacunas.md) e a [especificação de hipóteses](EC-07_hipoteses_operacionais.md) distinguem consulta, proposição, evidência e teste. Preservam-se Fundamento, A1–A8 e ressalvas das auditorias EC-04/05/06.

## Objetos e rastreabilidade

| Objeto | Informação necessária à análise |
|---|---|
| Objetivo e lacuna | Origem, finalidade, contexto e versão; diferença entre ausência de suporte, conflito, falha e limite de recursos. |
| Hipóteses | Identidade e origem das candidatas, pressupostos, previsões distinguíveis e evidências favoráveis ou contrárias; incerteza explicitada sem inventar probabilidades. |
| Intervenção | Operação candidata, entrada, fonte e contexto; evidências ou resultados possíveis, método verificável, dependências e possibilidade de falha ou inconclusão. |
| Recursos | Estimativa e unidade de custo, memória, tempo, eventos e outros recursos relevantes; orçamento restante por dimensão e margem quando definida. Não somar dimensões distintas sem regra justificada. |
| Permissão e risco | Autoridade e escopo concretos, validade da permissão, efeitos externos e reversibilidade; não presumir permissão a partir da utilidade. |
| Decisão | Alternativas efetivamente consideradas, elegibilidade, razões de preferência ou abstenção, estimativas, incertezas e critérios adotados. |
| Execução e revisão | Registrar se houve execução real, resultado e fonte, consumo medido com método e limites, divergência entre estimado e efetivo, justificativa e histórico de revisões. Intenção não é execução. |

Uma intervenção pode consultar memória ou fonte autorizada, comparar evidências disponíveis, executar teste permitido, solicitar informação ao usuário, ou resultar em abstenção/adiamento por falta de informação, recursos ou autorização. Abster-se é decisão de não executar, não evidência nova obtida.

## Critérios para comparar alternativas

A avaliação proposta é qualitativa quando faltarem dados calibrados; não introduz escala numérica obrigatória nem define a política final. Cada candidata deve ser examinada quanto a:

1. **Relevância:** relação explícita entre resultado possível, objetivo e lacuna. Uma leitura barata de informação que não modifica nenhuma conclusão relevante não demonstra ganho.
2. **Discriminação:** previsões das hipóteses sob resultados possíveis, incluindo falha, ausência e observação ambígua. Se resultados forem iguais para todas, a intervenção não as distingue sob as suposições informadas; pode servir a outro objetivo, que deve ser declarado.
3. **Qualidade da evidência:** proveniência, validade contextual/temporal, cobertura, independência e possibilidade de verificação; declarar dependências compartilhadas e fontes desconhecidas.
4. **Viabilidade e custo:** disponibilidade, tempo, memória, operações/eventos e demais limites em suas unidades; diferenciar custo estimado de medido e não afirmar calibração sem dados.
5. **Permissões e risco:** autorização aplicável, efeitos e possibilidade de interrupção; qualquer operação externa ou potencialmente irreversível requer autorização adequada, mesmo quando útil. Uma classificação de utilidade nunca concede permissão.
6. **Justificativa da preferência:** comparar alternativas elegíveis com informação equivalente, explicitar critérios e empates ou incerteza. Não tratar menor custo como maior aprendizado nem atribuir ganho informacional a uma estimativa não verificada.

Uma sequência conservadora para análise é identificar objetivo e candidatas; verificar permissões, riscos e viabilidade; comparar relevância, discriminação, qualidade e custos das alternativas elegíveis; registrar uma escolha justificada ou abster-se. Esta sequência é orientação documental, não algoritmo implementado nem ordenação experimental congelada. Se custo, permissão, risco ou discriminação estiverem indeterminados, pode-se solicitar informação autorizada ou adiar; não presumir viabilidade. Se nenhuma candidata for elegível ou puder distinguir hipóteses no objetivo declarado, registrar o motivo e abster-se ou reformular a investigação.

Interromper uma execução futura diante de revogação de permissão, limite atingido, risco inesperado, falha de guarda ou impossibilidade de verificar o resultado. Registrar o estado parcial e consumo efetivo; não promover uma hipótese apenas por uma observação inconclusiva. Revisão de hipótese ou plano depende do resultado, contexto e pressupostos, com histórico preservado; nenhuma regra de atualização ou recência universal é estabelecida aqui.

## Mecanismos já existentes

| Mecanismo | Evidência no repositório | Limite |
|---|---|---|
| Limites operacionais | `TwoHopTask` recebe limite de fatos, fila e eventos; `Engine` limita processamento; `EngramStore` limita capacidade. `Monitor` limita tentativas e distingue `ReviewBudget`/`ResourceLimited`. | Parâmetros são configurados externamente; não estimam utilidade de uma intervenção nem custo total uniforme. |
| Escolha programada | `Monitor::review` avalia contexto original e alternativas fornecidas em ordem e admite resposta sob regra de suporte único; retorna `AttemptLog`. | É regra delimitada de revisão de consulta, não planejamento adaptativo de intervenções. A ordem e as alternativas são dadas ao mecanismo. |
| Indicador de investigação | Protótipo EC-02.4 em `tests/support/expectation.rs` pode retornar `Investigate`. | Decisão simbólica não seleciona fonte, não obtém evidência e não executa investigação. |
| Exploração e critérios | A02.4 emprega exploração fixa externa; EC-04 induz critério simbólico de família fornecida com limites de trabalho. | Não há evidência de seleção investigativa com utilidade calibrada ou ganho causal posterior; EC-04 tem ressalvas de controles e instrumentação. |
| Inferência e evidência | `TwoHopTask` e `Monitor` retornam referências/avaliações; EC-06 obteve concordância com controle simbólico em quatro topologias. | Fatos e regras são fornecidos; pressupostos compartilhados e trabalho não uniformizado impedem extrapolação a planejamento ou vantagem neural. |

Referências: [cognição](../src/cognition.rs), [metacognição](../src/metacognition.rs), [motor de eventos](../src/engine.rs), [memória](../src/engram.rs), [expectativa experimental](../tests/support/expectation.rs) e [auditoria EC-06](EC-06_auditoria_cientifica_final.md). Não foi identificado no escopo examinado mecanismo integrado de autorização, estimativa calibrada e escolha autônoma de intervenção; isso não prova impossibilidade geral.

## Exemplos conceituais — não executados

| Situação | Análise e decisão justificável | Limite |
|---|---|---|
| Informativa e viável | Duas hipóteses preveem resultados distintos em uma consulta autorizada, com fonte válida, custo estimado dentro de cada limite e método verificável; selecioná-la pode ser justificado. | Resultado ainda precisa ser obtido e avaliado; utilidade não é aprendizagem demonstrada. |
| Barata e irrelevante | Uma leitura custa pouco mas retorna o mesmo dado sob ambas as hipóteses e não atende outro objetivo declarado; não priorizá-la para discriminação. | Poderia ser relevante a outro objetivo, explicitado separadamente. |
| Útil fora do orçamento | Teste distinguiria hipóteses, mas memória estimada excede limite disponível; adiar ou considerar alternativa elegível. | Não reduzir limite científico nem declarar que hipótese foi refutada. |
| Não autorizada | Consulta externa potencialmente informativa não tem permissão para a fonte; não executar, registrar impedimento e considerar solicitação de autorização apropriada. | Benefício previsto não substitui permissão. |
| Hipóteses indistinguíveis | Todas as evidências autorizadas previstas são iguais para as candidatas; registrar incapacidade de discriminar e abster-se de concluir. | Não significa que as hipóteses sejam equivalentes em todos os contextos. |
| Risco ou custo desconhecido | Uma ação parece informativa, mas seus efeitos ou consumo não podem ser estimados suficientemente; solicitar esclarecimento ou adiar. | Uma hipótese de risco não é medição e não autoriza uma ação. |

## Diretrizes, pendências e verificações

A1/A2 orientam memória, proveniência e histórico; armazenamento não demonstra reorganização neural. A3/A4 orientam investigação pedagógica progressiva e comunicação verídica de ações efetivamente executadas. A5/A6 orientam pt-BR e expansão futura, sem exigir que esta seleção dependa de linguagem natural. A7 requer controles e medidas verificáveis; A8 orienta competência generalizável futura, não demonstrada por escolha programada. As diretrizes permanecem intactas.

Persistem para decisão prospectiva em subetapas futuras: catálogo e ambiente de intervenções, equivalência informacional e de orçamento entre políticas, métricas de ganho de informação e competência, instrumentos de custo, autorização e critérios de falsificação. A existência destas opções não congela protocolo nem antecipa comparação. Ressalvas EC-04/05/06 sobre proveniência, controles compartilhados, validade contextual, generalização e instrumentação permanecem.

Inspeção e verificações documentais nesta entrega; nenhum teste, implementação ou experimento novo executado. Resultados mencionados são históricos. Próxima subetapa nominal do plano: **comparar investigação ativa com seleção aleatória e política fixa**, não iniciada.
