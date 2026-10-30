# Neurons — Plano Executivo de Evolução Cognitiva v0.3

**PROPOSTA DOCUMENTAL — NÃO APROVADA — NENHUMA ETAPA AUTORIZADA OU INICIADA.** Base `1f950dbcb4d35746b44e969e9ed22ee1bdf9ea8d`, branch `devs`. Este documento propõe questões e dependências; não substitui decisões científicas ou autorizações futuras, não encerra integralmente o v0.2 e não altera resultados históricos.

## 1. Resumo executivo e visão

O Neurons é uma arquitetura experimental modular, orientada a eventos, inspirada funcionalmente em mecanismos cerebrais, com neurônios, sinapses e engramas artificiais. Propõe-se investigar progressivamente percepção, conceitos, memória, aprendizagem, hipóteses, raciocínio, revisão, investigação, decisão e ação dentro de permissões, além de metacognição e autorregulação verificáveis. A visão inclui memória cognitiva evolutiva, formação pedagógica progressiva, investigação por evidências, pt-BR inicial, futura especialização e autonomia supervisionada. **A8 — Competência Cognitiva Generalizável — é objetivo científico, não capacidade homologada.**

Sete frentes propõem primeiro fortalecer validade experimental e depois investigar mecanismos e capacidades em ordem de dependência real. Trabalhos sem dependência podem ocorrer em paralelo, sob autorização própria; a ordem não é currículo fixo. Um resultado negativo ou inconclusivo é resultado científico legítimo e não será transformado em aprovação por ajuste de critérios. Não se propõe criar agentes, infraestrutura ou capacidades nesta elaboração.

## 2. Maturidade e fontes

Fontes: [v0.2](../../neurons_plano_executivo_evolucao_cognitiva_v0_2.md), [consolidação](../Plano_v0_2_consolidacao_final.md), [reconciliação EC-00–02](../Plano_v0_2_reconciliacao_historica_EC00_EC02.md), [auditoria EC-12](../EC-12_auditoria_cientifica_final.md), [inventário](../EC-12_inventario_competencias_demonstradas.md), [lacunas](../EC-12_avaliacao_lacunas.md), [contratos](../EC-12_pre_requisitos_especializacao.md), [questões](../EC-12_questoes_abertas.md), auditorias [EC-03](../EC-03.5_auditoria_experimental.md), [EC-04](../EC-04_auditoria_cientifica_final.md), [EC-05](../EC-05_auditoria_cientifica_final.md), [EC-06](../EC-06_auditoria_cientifica_final.md), [EC-07](../EC-07_auditoria_cientifica_final.md), [EC-08](../EC-08_auditoria_cientifica_final.md), [EC-09](../EC-09_auditoria_cientifica_final.md), [EC-10](../EC-10_auditoria_cientifica_final.md), [EC-11](../EC-11_auditoria_cientifica_final.md), [fundamento](../fundamentos/Neurons_Fundamento_Cognitivo_Metacognitivo_Pedagogico_Aprovado.md), [A1–A8](../fundamentos/Neurons_Diretrizes_Complementares_Memoria_Pedagogia_Idiomas.md) e [decisões](../decisions.md).

| Estado | Evidência e fronteira |
|---|---|
| Implementado | Motor/eventos, circuito, neurônios/sinapses, `EngramStore`, `TwoHopTask`, `Monitor`, `PlasticityRule`, currículo e sessão restritos; `IntegratedState` liga interfaces existentes. Código não equivale a competência cognitiva geral. |
| Demonstrado no escopo | Recuperação literal, composição programada e monitoramento por regras; checkpoint de motor/memória e reconstrução de consulta fixa; aquisição e revisão simbólica restritas em protótipos EC-04/10. |
| Parcial/inconclusivo | EC-03–11 preservam AJUSTAR e limites de controles, registros ou custos. R3 EC-10 não executou final por quota 20/28; H1 EC-11 não comparou relatórios completos entre repetições. |
| Não demonstrado | Aprendizagem pedagógica contínua, investigação autônoma, transferência genuína, cumulatividade, metacognição geral, autonomia cognitiva geral e A8. Não se infere impossibilidade geral. |
| Não avaliável pelo contrato atual | Continuidade de `Circuit`, pesos e estados neuronais após reinício e política integrada de permissões: checkpoint serializa apenas `Engine` e `EngramStore`, monitor é fornecido novamente. |

EC-00/01 têm fontes integrais históricas pendentes; encerramento global EC-02 não verificável; transcrições EC-02.2 e EC-02.4-R1 foram recuperadas e arquivadas posteriormente, sem comprovar pré-registro independente. Não se presume inexistência de fontes. Os encerramentos delimitados EC-03–12 não homologam seus critérios gerais nem o plano v0.2 inteiro.

## 3. Princípios, governança e metodologia

A1/A2 orientam memória evolutiva de engramas interconectados e camadas como abstrações flexíveis; A3/A4, pedagogia progressiva, comunicação honesta e investigação realmente executada; A5/A6, pt-BR inicial e expansão multilíngue posterior; A7, rigor; A8, generalização a demonstrar. Armazenamento, recuperação, aplicação de regra, aprendizagem persistente, retenção funcional e transferência têm evidências distintas. Eventos neurais não provam vantagem causal neural. Estruturas e conteúdo fornecidos são identificados; não converter avaliador externo em capacidade do núcleo.

**Ciclo de governança:** proposta → aprovação humana da etapa/questão e limites → execução autorizada → avaliação independente quando viável → decisão de MANTER/AJUSTAR/REJEITAR e eventual encerramento. Uma aprovação documental do plano não é autorização automática de implementação, acesso externo ou ação irreversível; autonomia operacional depende de permissões explícitas, orçamento, supervisão, interrupção e reversibilidade quando aplicável. Dentro do escopo já autorizado, inspeções, testes e rotinas reversíveis não exigem microautorizações. Encerramento administrativo, conformidade metodológica e homologação científica são estados separados.

Antes de cada painel: congelar hipótese falsificável, estado, geradores, partições, critérios, referências, controles, métricas e orçamento; registrar versões/hashes e cronologia sem alegar que hash sozinho é pré-registro independente. Referências externas independentes são preferidas; quando inviáveis, explicitar compartilhamento de autoria/pressupostos e restringir conclusões. Confrontar controles simples com acesso equivalente, sem privilegiar gabaritos, ferramentas ou contexto. Descrever diferenças inevitáveis em oportunidades de aquisição. Desenvolvimento, validação e final devem ser separados também em estruturas quando a hipótese exige novidade estrutural; repetições de mesma topologia não são amostras independentes. Registrar falhas, abstenções, resultados nulos e negativos, dependências, custos e incerteza.

**Mudança e interrupção:** não ajustar regras ou amostras após resultado final; violação de partições, vazamento, ausência de referência, limites de recursos ou ambiguidade científica substantiva interrompe a alegação afetada. Registrar falha técnica, resultado contrário ou inconclusão separadamente; qualquer correção é prospectiva e requer autorização compatível, sem reescrever história. AJUSTAR não desbloqueia automaticamente capacidades não demonstradas. Segurança e autorização não decorrem de utilidade estimada.

**Recursos:** por padrão de planejamento, não de aprovação automática, propor RSS <256 MiB, dados ≤256 KiB por condição, painel ≤30 s e ≥3 GiB livres em `/home`, sujeitos a viabilidade e aprovação por protocolo; medir tempo, memória e trabalho por condição, declarar parcelas não isoladas e suspender se limites efetivos forem violados. EC-11 observou 376 bytes e RSS externo 11.208 KiB em um cenário; não extrapolar para escala ou custo exclusivo. Limiares de desempenho, amostras e janelas de retenção são decisões científicas prospectivas, definidos antes do painel com justificativa e não inventados por este plano.

## 4. Dependências e marcos

`V03-01 → {V03-02, V03-03, V03-04}`; `V03-02 + resultados relevantes de 03/04 → V03-05`; `V03-04/05 → V03-06 conforme tarefa`; `V03-02/03/04/05 + contratos de permissões pertinentes → V03-07`. Setas significam evidência necessária para **a alegação específica**, não bloqueio de análise documental ou de trabalho experimental independente. Configuração funcional simples não exige aprendizado, transferência ou todas as modalidades pedagógicas. V03-03 pode estudar critério simbólico sem aguardar engrama neural; V03-04 pode avaliar regra fornecida sem aprendizagem; V03-06 pode avaliar reconhecimento restrito sem domínio geral. Pesquisas paralelas devem preservar recursos e partições.

Marcos: M1 contratos e controles verificáveis; M2 mudanças ou inferências delimitadas com evidência rastreável; M3 ciclos pedagógicos e comunicação avaliados no escopo; M4 retenção/transferência e coordenação avaliadas quando viáveis. Nenhum marco é presumido atingido e não existe exigência circular entre aprendizado elementar e pedagogia geral.

## 5. Etapas propostas

### V03-01 — Confiabilidade experimental, contexto e permissões

**Problema/objetivo:** rastreabilidade e controles de EC-03–11 são parciais; estabelecer condições verificáveis para experiências futuras. **Hipótese:** interfaces explícitas, referências e controles equivalentes detectarão erros e impedirão aceites indevidos em cenários congelados; alternativa: falhas persistem ou custos inviabilizam o contrato. **Pré-requisitos/mecanismos:** inventário atual, motor/memória/monitor; não requer aprendizado geral. **Subetapas:** inventariar estado e origem; definir contratos de evidência, versão, autorização e orçamento; verificar instrumentos e negativos; avaliar reproducibilidade com relatórios completos. **Entregáveis:** especificação e resultados rastreáveis; qualquer adaptador depende de necessidade e autorização próprias.

**Protocolo previsto:** cenários com contextos distintos, evidências conflitantes, checkpoint corrompido, permissão negada e limites; partições e expectativas congeladas. **Controles/referência:** memória literal, verificador independente quando viável e controles sem informação privilegiada. **Métricas/aceitação:** acertos de integridade e abstenção, falsos aceites, igualdade de registros, tempo/RSS/trabalho; aceitar somente contrastes e limites prospectivamente definidos, com violações reportadas. **Interrupção/risco:** vazamento, ausência de referência ou medição, permissão/limite violado; FNV não autentica, custo de instrumentos pode contaminar medição. **Posterior:** apoia todas as frentes; não autoriza política universal. **Rastreio:** L11,L13,L14,L15,L17; P05–P15 conforme papel; Q12,Q14,Q15,Q16,Q18; A1,A2,A4,A7.

### V03-02 — Memória evolutiva, proveniência e retenção funcional

**Problema/objetivo:** registros persistem sem demonstrar engramas aprendidos ou continuidade neuronal. **Hipótese:** uma mudança identificável de estado pode contribuir à recuperação funcional posterior; alternativa: resposta depende apenas de registro ou regra fornecida. **Pré-requisitos/mecanismos:** V03-01 aplicável, `EngramStore` e checkpoint existentes; extensão de circuito somente se necessidade e contrato forem aprovados. **Subetapas:** distinguir episódios, associações e evidências; especificar persistência e revisão contextual; comparar antes/depois e após intervalo/reinício quando suportado; avaliar interferência onde houver recursos compartilhados. **Entregáveis:** contrato, protocolo e resultados sem prescrever arquitetura física.

**Protocolo/controles:** estados pareados, memória vazia, literal, sem atualização e referência externa quando viável; experiência intermediária e versões explícitas, sem herança automática. **Métricas/aceitação:** recuperação correta, abstenção, retenção e regressão por classe, histórico, custo e origem; só alegar retenção funcional com avaliação posterior e controles adequados. **Interrupção/risco:** estado não serializado, histórico ausente ou vazamento; persistência de bytes não prova competência. **Posterior:** contribui a 03,05,07 quando necessário. **Rastreio:** L03,L10–L13,L17; P03,P10,P11,P14; Q02,Q03,Q11–Q14; A1,A2,A7.

### V03-03 — Formação e revisão delimitada de critérios e conceitos

**Problema/objetivo:** EC-04 e EC-10 não demonstram formação autônoma nem substituição de respostas antigas corretas em R3. **Hipótese:** experiência com evidência revisada produz mudança rastreável justamente nas testemunhas discriminantes, preservando invariantes; alternativa: mudança apenas reflete recuperação ou prática. **Pré-requisitos/mecanismos:** V03-01 e dados/estado reais apropriados; testes simbólicos podem preceder memória neural. **Subetapas:** definir família e identificação; estabelecer aquisição e revisão; avaliar testemunhas previamente corretas e invariantes; analisar abstenções e regressões. **Entregáveis:** protocolo novo se autorizado, implementação isolada mínima e relatório sem alterar R1–R3.

**Protocolo/controles:** partições disjuntas e propriedades pré-verificadas; controles congelado, prática antiga, literal e simbólico independente; diferenças informacionais declaradas. **Métricas/aceitação:** respostas antigas corretas, substituições, preservações, erros e abstenções por classe; comparação com referência independente; sucesso requer critérios e quotas previamente aprovados. **Interrupção/risco:** testemunhas inviáveis, vazamento ou alteração de gabarito; EC-10 R3 permanece não executado e não é reaberto automaticamente. **Posterior:** base opcional de 05/07. **Rastreio:** L01,L02,L13,L17; P01–P04; Q01,Q04; A1,A2,A7,A8.

### V03-04 — Raciocínio, questionamento e monitoramento por evidências

**Problema/objetivo:** inferência de dois passos e decisões de investigar não demonstram raciocínio ou metacognição gerais. **Hipótese:** em contratos formais determinados, conclusões e abstenções podem ser verificadas e uma intervenção realmente executada pode contribuir a evidência ou decisão; alternativa: regras e controles simples explicam desempenho. **Pré-requisitos/mecanismos:** V03-01, `TwoHopTask` e `Monitor` onde aplicáveis; nenhuma regra de transitividade implícita. **Subetapas:** especificar semântica e provas; comparar inferências e controles; investigar lacunas/intervenções autorizadas; avaliar decisão, execução e resultado separadamente. **Entregáveis:** tarefas, protocolos, provas e resultados, inclusive nulos.

**Protocolo/controles:** fatos/contextos equivalentes, literal, simbólico, fixo e pseudoaleatório adequado onde pertinentes, negativos e referências independentes; não consultar alvos ocultos. **Métricas/aceitação:** correção, validade de prova, proveniência, abstenção, benefício incremental, intervenção/custo; calibração probabilística somente com probabilidades justificadas e referência independente. **Interrupção/risco:** assimetria, prova sem suporte, ação não autorizada ou medição de custo não comparável; não atribuir ganho do avaliador ao núcleo. **Posterior:** informa 05/06/07 em tarefas pertinentes. **Rastreio:** L04–L06,L13–L15,L17; P02,P05,P10,P13,P15; Q06–Q08,Q14–Q16; A2,A3,A4,A7,A8.

### V03-05 — Pedagogia progressiva e competência demonstrada

**Problema/objetivo:** EC-09 especificou ciclos, sem melhora sustentada. **Hipótese:** uma experiência selecionada e executada sob critérios explícitos produz melhora persistente e adaptação observável; alternativa: ganho é devido a prática fixa, memorização ou informação inicial. **Pré-requisitos/mecanismos:** V03-01 e resultados aplicáveis de 02/03/04; aquisição elementar pode ser estudada antes do ciclo completo. **Subetapas:** diagnóstico; seleção e execução de experiência; feedback e revisão; consolidação e retenção; avaliação independente e progressão por competência. **Entregáveis:** ciclo experimental delimitado, métricas e registro por etapa, não currículo universal.

**Protocolo/controles:** pré/pós, sem prática, prática fixa, aleatória válida, feedback ausente/incorreto quando seguro, mesmo conhecimento inicial e exemplos finais reservados. **Métricas/aceitação:** desempenho por ciclo, retenção, erros, abstenção, mudança de estratégia, progressão indevida e custo; aprovação requer melhora sustentada e adaptação verificável nos critérios congelados, sem exigir perfeição universal. **Interrupção/risco:** vazamento, referência insuficiente ou ausência de mecanismo; seleção não é aprendizagem nem autorização de agir. **Posterior:** apoia linguagem e especialização adquirida. **Rastreio:** L02,L05–L08,L13–L17; P02,P03,P13,P15; Q04,Q05,Q07–Q09,Q15–Q17; A1–A5,A7,A8.

### V03-06 — Linguagem pt-BR e comunicação verificável

**Problema/objetivo:** léxico e gramática restritos não demonstram compreensão de português. **Hipótese:** competências linguísticas delimitadas podem ser adquiridas e aplicadas a exemplos inéditos; alternativa: tabelas, regras fornecidas ou memorização explicam desempenho. **Pré-requisitos/mecanismos:** V03-01 e, para alegação de aquisição, V03-05 pertinente; testes funcionais configurados podem ocorrer antes. **Subetapas:** representações e vocabulário; morfologia e sintaxe; semântica/pragmática sob contratos; interpretação de perguntas e produção de respostas; avaliar generalização em itens inéditos. **Entregáveis:** catálogo de tarefas, referências, controles e resultados por competência; expansão multilíngue é hipótese posterior dependente de evidência, não bloqueio de comunicação inicial.

**Protocolo/controles:** dados e gabaritos separados, referências linguísticas verificáveis e controles literais/regras com acesso igual; avaliar erro, desconhecimento e abstenção. **Métricas/aceitação:** desempenho por fenômeno, prova ou justificativa quando aplicável, retenção, novidade e custo; critérios não autorizam alegação de compreensão geral. **Interrupção/risco:** respostas vazadas, itens estruturalmente repetidos ou ausência de referência; sinais externos não são semântica neural. **Posterior:** especialização linguística somente com evidências. **Rastreio:** L02,L07,L08,L09,L17; P01–P04,P12; Q09,Q10,Q17; A3–A8.

### V03-07 — Retenção longitudinal, transferência e especialização supervisionada

**Problema/objetivo:** transferência genuína, cumulatividade e coordenação de agentes não foram demonstradas. **Hipótese:** experiência anterior identificável pode melhorar desempenho em tarefa estruturalmente distinta e permanecer após experiências intervenientes; alternativa: regras prévias, recuperação ou sorte explicam resultado. **Pré-requisitos/mecanismos:** V03-01 e evidências específicas de 02/03/05; papel multiagente somente se contratos de identidade, permissões, mensagens e supervisão estiverem previamente demonstrados para esse escopo. **Subetapas:** avaliar retenção e interferência compartilhada; contrastar com/sem experiência; distinguir especialização configurada, funcional, adquirida e transferível; estudar coordenação somente quando viável e autorizada. **Entregáveis:** avaliações por condição, limites, custos e decisões negativas/inconclusivas.

**Protocolo/controles:** estado anterior e posterior rastreáveis, intervalos reais/reinício se suportado, estruturas não isomorfas quando requeridas, controle sem experiência, literal e regras com informação relevante equivalente, supervisão e corte de autoridade. **Métricas/aceitação:** transferência positiva/nula/negativa, retenção, interferência, custo, erros de delegação e violações; homologação de transferência exige contribuição causal da experiência sobre controle apropriado, não mera renomeação. **Interrupção/risco:** falta de estado persistente, controle fraco, permissões indefinidas ou referência insuficiente; resultados negativos não demonstram impossibilidade geral. **Posterior:** eventual escolha humana de aplicação, sem agentes/orquestradores presumidos. **Rastreio:** L02,L09–L17; P01–P15 conforme modalidade; Q10–Q18; A1–A8.

## 6. Rastreabilidade integral dos identificadores

| Grupo | Correspondência principal, sem exclusividade |
|---|---|
| L01–L04 | 03; 02/05; 02; 04, respectivamente. |
| L05–L08 | 04; 04/05; 05; 06, respectivamente. |
| L09–L12 | 07; 02/07; 01/07; 02/07, respectivamente. |
| L13–L17 | 01/02; 01/04/07; 01/07; 07; todas as frentes, respectivamente. |
| P01–P04 | 03/06/07: configurada, funcional, adquirida e transferível; não são estágios obrigatórios universais. |
| P05–P10 | 01/07: principal, agente, equipe, suborquestrador, orquestrador, núcleo mínimo; papéis futuros, sujeitos à autorização. |
| P11–P15 | 01/02/04/05/07: memória, comunicação, supervisão, persistência e investigação, condicionais à tarefa. |
| Q01–Q04 | 03; 02; 02/03; 03/05, respectivamente. |
| Q05–Q09 | 05; 04; 04; 04; 06, respectivamente. |
| Q10–Q14 | 07; 02/07; 01/07; 02/07; 01/02, respectivamente. |
| Q15–Q18 | 01/07; 01/07; 07; 07, respectivamente. |
| Q19 | Investigação de consciência opcional e independente, sem bloqueio das frentes. |

Identificadores apontam fontes e perguntas, não comprovam que uma capacidade tenha sido implementada ou que todo contrato seja necessário para cada tarefa. Estados de evidência e exceções são os dos documentos originais.

## 7. Consciência e decisões humanas pendentes

Consciência artificial é investigação distinta, opcional e não bloqueante. Autorrepresentação, monitoramento, integração de informação ou relatos não provam experiência subjetiva; nenhuma teoria é adotada como verdade. Não é pré-requisito para memória, aprendizagem, raciocínio, metacognição funcional, comunicação ou especialização. Não se afirma consciência, senciência ou vida artificial.

Antes de aprovação: decisão humana sobre adoção ou revisão desta proposta; reconciliação ou aceitação explícita das pendências de fontes/estado global EC-02 do v0.2; seleção das questões, paralelismo e limites de autoridade; definição dos critérios de evidência, independência, amostras, quotas, métricas e limiares por protocolo; decisão sobre eventual persistência de circuito e fronteiras de segurança; aprovação de quaisquer ações externas, experiências e arquitetura nova pertinentes. Os recursos indicados são propostas sujeitas à aprovação e adequação ambiental. Nenhuma decisão de continuidade ou início de etapa foi tomada neste documento.

A aprovação futura exige rastreabilidade às fontes e aos identificadores, definição de responsáveis, riscos e autorização humana por decisão substantiva; preservação de resultados AJUSTAR, controles e critérios prospectivos; separação entre proposta, execução e homologação. **Este v0.3 permanece SOMENTE PROPOSTA, NÃO APROVADA e NÃO EXECUTADA.**

## 8. Verificações da elaboração documental

`cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` passaram. Regressão histórica: **185 testes aprovados, zero falhos e um ignorado**; o ignorado é `r3_frozen_final` EC-10 por quota insuficiente. Inclui duplicações e auxiliar inerte; não constitui experimento v0.3. Três avisos históricos EC-04 de métodos não utilizados. Links locais foram conferidos; `/home` observado com cerca de 3,9 GiB livres, sem monitoração contínua. Nenhuma etapa v0.3 foi iniciada.
