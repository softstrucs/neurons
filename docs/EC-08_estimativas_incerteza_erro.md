# EC-08 — Estimativas de incerteza e erro

## Escopo e princípios

Base `734332164173444abe3d4d9008a80957890aeafc`, devs inicialmente limpa e sincronizada. A segunda subetapa nominal do plano v0.2 é **definir estimativas de incerteza e erro**; o plano não exige implementação nem ensaio nesta subetapa. Seu aceite global exige monitoramento útil de processos efetivos, ainda não demonstrado em geral. Esta especificação não estabelece arquitetura definitiva, política de decisão, limiares numéricos ou protocolo experimental.

Uma avaliação deve identificar tarefa, objetivo, consulta, contexto/versão, estado acessível, processo, evidências e limitações. Categorias abaixo podem coexistir; não são classes mutuamente exclusivas nem probabilidades. Sua atribuição vale apenas para a consulta, dados e contrato examinados. Ausência de contradição não prova correção; ausência de suporte não demonstra falsidade; falha de recurso não é, por si, evidência epistêmica contrária.

## Categorias operacionais

| Categoria e identificação | Dados, proveniência e limites | Respostas possíveis, sem política automática |
|---|---|---|
| Insuficiência epistêmica: evidência acessível não sustenta conclusão sob contrato e busca efetivamente realizada. | Consulta, regras, cobertura, fontes e motivo de ausência; não pressupõe busca exaustiva ou falsidade. | Abster-se; solicitar fonte/ajuda autorizada ou rever após evidência nova. |
| Ambiguidade: duas ou mais alternativas plausíveis permanecem compatíveis no contexto considerado. | Identidades, suporte e contexto/versionamento de cada alternativa; alternativas não enumeradas podem existir. | Abster-se de escolher; investigar ou pedir esclarecimento quando autorizado. |
| Conflito: evidências válidas no mesmo escopo sustentam proposições incompatíveis sob regra explícita. | IDs, origens, validade, contexto, versão e regra de incompatibilidade; versões distintas não são automaticamente contraditórias. | Suspender, investigar ou rever quando justificável; preservar histórico. |
| Erro observado: resposta ou procedimento diverge de referência válida sob comparação apropriada. | Resposta, referência independente quando possível, contexto, critérios e proveniência; referência pode falhar ou ser incompleta. | Registrar erro e evidência, corrigir/revisar se autorizado; preservar resultado anterior. |
| Suspeita de erro: sinal ou discrepância sugere erro sem referência suficiente para confirmá-lo. | Sinal, origem, hipóteses alternativas e limitações; conflito isolado não identifica necessariamente qual fonte errou. | Suspender ou investigar; não marcar erro confirmado. |
| Falha procedimental: execução, propagação, dados ou verificação falham contra contrato operacional. | Relatório de execução, erro e estado da tarefa; distinguir falha do resultado falso. | Interromper, relatar e eventualmente repetir sob condições autorizadas. |
| Limite de recursos/permissões: orçamento, capacidade ou autorização impede conclusão. | Recurso, limite, consumo ou evidência de ausência de permissão; estimativas separadas de medições. | Interromper, abster-se, adiar ou solicitar ajuda/permissão apropriada. Pode coexistir com insuficiência. |
| Sem referência suficiente: nenhuma referência válida permite julgar correção. | Evidências disponíveis, qualidade, contexto e cobertura; distinguir referência indisponível de referência negativa. | Resultado não verificável, suspensão/abstenção conforme tarefa; não classificar como erro ou acerto. |

**Resultado correto demonstrado** significa concordância verificada com referência e método válidos no escopo delimitado; não verdade universal. **Erro observado** exige divergência verificável; **suspeita** não confirma erro; **incerteza não resolvida** mantém alternativas ou evidências insuficientes; **não verificável** descreve a falta de referência adequada. A qualidade de evidência — origem, integridade, independência, validade, cobertura e conflito — deve ser avaliada separadamente da correção da resposta. Um acordo com fonte não autenticada não prova correção externa.

## Representações e calibração

`Assessment`, `AbstainReason`, `EvidenceRefs`, `Conflict` e `AttemptLog` em [metacognição](../src/metacognition.rs) já representam categorias e referências delimitadas; `InferenceReport` em [cognição](../src/cognition.rs) registra eventos, resultado e limites. Essas estruturas não implementam todas as categorias acima, validade contextual completa ou estimativa de confiança calibrada. `MissingEvidence`, `ResourceLimited` e `ContradictoryEvidence` são rótulos de condição sob contrato, não percentuais.

Um **indicador heurístico** é sinal programado com significado e limitações declarados; um **escore** é número obtido por regra, sem interpretação probabilística automática. Uma **probabilidade estimada** requer evento, população, condições e método definidos; uma **probabilidade empiricamente calibrada** requer demonstração em dados de avaliação apropriados. Não atribuir percentuais ao monitor existente.

Se futura avaliação produzir probabilidades, congelar definição do evento e referência, separar desenvolvimento/calibração/avaliação, impedir vazamento, representar distribuição de casos e subgrupos relevantes e testar generalização fora da amostra. Usar referências independentes e auditáveis quando viáveis; documentar dependência, ruído e discordâncias. Avaliar adequação probabilística com medidas apropriadas, por exemplo Brier ou log loss com tratamento declarado de extremos, e confiabilidade por faixas com número de casos e incerteza amostral. Uma medida agregada só não prova calibração em todos os subgrupos ou contextos. Separar correção da decisão, cobertura/abstenção e custo. Sem referência ou amostra adequada, declarar calibração **não verificável**.

Em conjuntos suficientemente avaliáveis, excesso de confiança ocorre quando probabilidades previstas excedem sistematicamente a frequência observada do evento, e insuficiência quando ficam sistematicamente abaixo, para faixa/contexto definidos. Comparar frequências e incerteza amostral; um único erro com confiança alta não demonstra por si descalibração sistemática. Estimativa incorreta pode ser identificada por referência válida e critério congelado; sem tais condições registrar suspeita ou impossibilidade de verificação.

## Rastreabilidade e exemplos conceituais

Registro mínimo proposto: tarefa/objetivo, consulta, contexto e versão, estado acessível, evidências e origem/validade, inferência ou decisão, categoria ou estimativa com método, limites, referência e critério de verificação, resultado observado ou indisponibilidade, erro/suspeita/indeterminação, revisão e histórico. É requisito para avaliação futura, não estrutura já implementada.

| Caso hipotético | Classificação e consequência admissível |
|---|---|
| Prova válida e referência independente concordante no escopo | Suporte adequado e correção verificada nesse escopo; não prova geral. |
| Caminho incompleto sem negativa | Insuficiência; abstenção pode ser apropriada, sem declarar proposição falsa. |
| Provas positiva e negativa no mesmo contexto e versão | Conflito; registrar ambas e suspender ou investigar, sem escolher fonte por recência automaticamente. |
| Resposta diverge de referência independente válida | Erro observado no escopo; registrar e revisar com histórico. |
| Resposta plausível sem referência verificável | Ainda não verificável; não contar como correta ou errada. |
| Fila ou orçamento esgotado | Limitação operacional; resultado inconclusivo mesmo se também houver lacuna epistêmica. |
| Dois contextos alternativos incompatíveis ou sustentados | Ambiguidade/contexto não resolvido; abster-se até regra e evidência permitirem distinção. |
| Revisão sem evidência nova | Comparar regras, contexto e justificativa: uma reavaliação não prova aquisição ou correção; decisão pode legitimamente permanecer inalterada. |
| Abstenção diante de referência insuficiente | Pode ser apropriada para evitar resposta não sustentada; avaliar cobertura e custo separadamente. |
| Escore `0,9` gerado por regra sem dados de calibração | Escore não calibrado; não interpretá-lo como 90% de probabilidade. |

Todos os exemplos são conceituais e não constituem resultados observados nesta entrega.

## Controles e falsificação futura

Para avaliar utilidade, monitor e controles devem receber mesmos fatos, contextos, alternativas, referências acessíveis e orçamentos, salvo diferença explicitamente medida; incluir controle sem monitor e política fixa, definir partições e decisões antes da avaliação e comparar custos. Referência deve ser isolada das decisões e independente tanto quanto possível. Testar casos negativos, conflitos, falhas, ausência de referência e combinações de limites. A comparação histórica de sete casos com contexto alternativo ausente no controle não prova superioridade.

Hipóteses futuras falsificáveis, condicionadas a protocolo prospectivo: (1) o indicador de insuficiência detecta casos sem suporte acima de controle comparável, sem aumento não justificado de falsas abstenções; falha se não superar o critério previamente definido ou se violações excederem limite congelado; (2) decisão de revisão/abstenção reduz erro verificado em relação ao controle com igual informação e orçamento, considerando cobertura e custo; falha se ganho não atingir critério anterior à avaliação; (3) quando um modelo probabilístico existir, suas estimativas satisfazem critério de calibração fixado antecipadamente nas populações e subgrupos avaliáveis; dados insuficientes implicam inconclusão. Não se fixam aqui limiares, parâmetros ou protocolo da subetapa seguinte.

EC-03/04/05 preservam limitações de proveniência, manutenção e controles; EC-06 usa composição de regras fornecidas; EC-07 mediu informação em avaliador externo sem ganho persistente de competência. Essas evidências não demonstram metacognição geral, aprendizado metacognitivo ou confiança calibrada.

## Diretrizes e continuidade

A1/A2 orientam memória e histórico rastreáveis; A3/A4 orientam pedagogia e comunicação honesta de desconhecimento e investigação realmente executada; A5/A6 orientam pt-BR e expansão multilíngue, não testados aqui; A7 exige evidência e falsificação; A8 requer reconhecimento de limitações e competência generalizável, sem presumir sua realização. A1–A8 e conclusões anteriores preservadas.

Esta subetapa é concluída como especificação documental. Inspeção e verificações documentais apenas; nenhum teste ou experimento novo. A próxima subetapa nominal é **testar decisões de revisar, investigar e abster-se**, não iniciada.
