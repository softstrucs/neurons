# EC-12 — Avaliar lacunas

Base `5db7d3ef77f4b2572e09bfb464cb4ffbd9aabb05`; branch `devs` inicialmente limpa e sincronizada. Esta subetapa é análise documental, **AJUSTAR**, sem experimento novo ou alteração de produção. A EC-12 permanece aberta. Fonte primária desta síntese: [inventário EC-12](EC-12_inventario_competencias_demonstradas.md), [plano v0.2](../neurons_plano_executivo_evolucao_cognitiva_v0_2.md), decisões e auditorias [EC-03](EC-03.5_auditoria_experimental.md), [EC-04](EC-04_auditoria_cientifica_final.md), [EC-05](EC-05_auditoria_cientifica_final.md), [EC-06](EC-06_auditoria_cientifica_final.md), [EC-07](EC-07_auditoria_cientifica_final.md), [EC-08](EC-08_auditoria_cientifica_final.md), [EC-09](EC-09_auditoria_cientifica_final.md), [EC-10](EC-10_auditoria_cientifica_final.md) e [EC-11](EC-11_auditoria_cientifica_final.md). EC-00–02 e limitações de arquivos históricos constam do inventário e plano.

## Método

A = implementação ou contrato ausente/incompleto; B = validação insuficiente de mecanismo ou capacidade; C = questão científica sobre viabilidade, mecanismo ou explicação. Tipos podem coexistir. **Não implementado**, **não avaliado**, **não demonstrado** e **resultado negativo** não são sinônimos: ausência de evidência não prova impossibilidade. Onde não há mecanismo, a causa conhecida é ausência do contrato descrito; hipóteses sobre causalidade de desempenho são explicitadas como hipóteses. A matriz seguinte e a matriz de investigação compartilham IDs estáveis; nenhuma proposta é autorização ou protocolo congelado.

## Estado, causas e impacto

| ID / capacidade | Tipo; estado e evidência | Causa conhecida ou hipótese; impacto e risco científico | A1–A8 |
|---|---|---|---|
| L01 conceitos e revisão | A/B/C; EC-04 aquisição simbólica delimitada, R2 EC-10 abstenção→acerto | Testemunhas antigas corretas não avaliadas em R3; hipótese: critérios insuficientes explicam abstenções, sem generalização causal. Revisão ampla não demonstrada. | A1,A2,A7,A8 |
| L02 aprendizagem progressiva | A/B/C; feedback local e EC-04 não demonstram ciclos sustentados | Ciclo integrado ausente; associar feedback externo a aprendizagem geral seria inferência indevida. | A2,A3,A7,A8 |
| L03 engramas/consolidação | A/B/C; `EngramStore` registra e recupera; EC-03/04 parciais | Armazenamento simbólico não demonstra formação ou reorganização neural; impacto em memória evolutiva. | A1,A2,A7 |
| L04 raciocínio com evidências | B/C; EC-06 duas relações e regras fornecidas | Estruturas e controles compartilham pressupostos; vantagem causal neuronal ou raciocínio amplo não demonstrados. | A2,A7,A8 |
| L05 metacognição | A/B/C; EC-08 monitor delimitado, benefício incremental zero | Probabilidades e decisão de investigar ausentes; zero local não prova inutilidade geral. Sem calibração ou adaptação demonstrada. | A4,A7,A8 |
| L06 questionamento/investigação | A/B/C; EC-07 escolha programada, avaliador externo | Não há demonstração de decisão autônoma no núcleo nem ganho persistente causado pela busca. Risco de atribuição indevida. | A3,A4,A7,A8 |
| L07 pedagogia e competências | A/B/C; EC-09 modelos documentais, critério global não homologado | Ciclo integrado e avaliação causal não demonstrados; não equivale a número de práticas ou registros. | A3,A4,A7,A8 |
| L08 pt-BR e comunicação | A/B/C; léxico e sessão restritos | Conteúdos e gramática fornecidos; domínio, aquisição e uso em exemplos novos não demonstrados. | A4,A5,A6,A7 |
| L09 generalização/transferência | B/C; EC-10 renomeação e famílias restritas | Novidade estrutural e contribuição causal da experiência não estabelecidas; R3 final não executado (20/28). Não prova impossibilidade. | A7,A8 |
| L10 cumulatividade e interferência | A/B/C; revisão e estados separados | Não há demonstração de acumulação de competências nem interferência em recursos compartilhados; substituição deliberada não é esquecimento espontâneo. | A1,A2,A7,A8 |
| L11 integração longitudinal | A/B; EC-11 bytes/decisões no cenário fixo | H1 não comparou relatórios completos entre repetições; limita reprodutibilidade integral, não invalida bytes observados. | A1,A2,A7 |
| L12 persistência neuronal | A/B; checkpoint atual persiste apenas Engine e EngramStore | Circuit, pesos, estado neuronal e configuração do Monitor não são serializados; continuidade neuronal após reinício não avaliável pelo contrato. | A1,A2,A7 |
| L13 proveniência/versões/retração | A/B; IDs e referências locais; EC-03/05 parciais | Contrato integrado de sucessão, validade, histórico e retração ausente; risco de misturar versões e justificativas. | A1,A2,A7 |
| L14 permissões/autonomia | A/B/C; limites locais e abstenções; sem política integrada | Ausência de contrato global impede testar adequação de autorização como sistema; não presumir autonomia segura. | A3,A4,A7,A8 |
| L15 recursos/escalabilidade | A/B; EC-04/05/11 instrumentação parcial | Custos por componente, dados vivos e RSS em escala não isolados; ausência de medição não é violação comprovada. | A1,A7 |
| L16 especialização de agentes | A/B/C; mecanismos locais e configuração não demonstram competência especializada adquirida | Processo de aquisição e transferência especializado não evidenciado; não inferir de estilo ou parametrização. | A3,A6,A7,A8 |
| L17 integridade científica | A/B; controles de EC-04–11, textos históricos EC-01/02 pendentes no plano | Autoria/pressupostos compartilhados e arquivos ausentes limitam verificação independente; hipótese de erro comum não é erro comprovado. | A7,A8 |

## Dependências e condições verificáveis de avanço

Critérios abaixo descrevem evidência mínima a especificar previamente em eventual investigação autorizada; não estabelecem limiares retrospectivos nem uma arquitetura definitiva.

| ID | Dependências e condição necessária para avaliar | Proposta futura e avanço verificável |
|---|---|---|
| L01 | L13 e L17; testemunhas com resposta antiga correta, rótulos distintos e conjuntos viáveis. | Contrastar revisão e prática sem mudança; demonstrar substituições e preservação de invariantes contra referência independente. Não criar R4 automaticamente. |
| L02 | L03, L07, L17; estado anterior, experiência rastreável e alteração persistente. | Comparar pré/pós com controle de experiência e exemplos inéditos; verificar retenção e limites. |
| L03 | L12/L13 e referência; mecanismo identificável para associação/consolidação. | Contrastar registro simbólico e mudanças persistentes funcionais; testar retorno e revisão sem confundir armazenamento com competência. |
| L04 | L13/L17; contratos semânticos e provas verificáveis. | Avaliar inferência em estruturas novas com controles informacionalmente equivalentes; relatar provas inválidas e abstenções. |
| L05 | L04/L17; indicadores do próprio núcleo e referência independente. | Medir erro/abstenção e custo incremental; calibração apenas com probabilidades fundamentadas e dados adequados. |
| L06 | L05/L14/L17; mecanismo real de decisão e execução autorizada. | Comparar seleção ativa, fixa e aleatória com dados iguais; verificar causalmente efeito posterior sem atribuir cálculo externo ao núcleo. |
| L07 | L02/L05/L06; diagnóstico e experiência rastreáveis. | Avaliar ciclos sucessivos pré/pós, controles de prática e feedback, retenção e adaptação por lacuna. |
| L08 | L02/L07 e referências linguísticas; exemplos inéditos. | Avaliar produção e interpretação pt-BR com controle de memorização; não inferir domínio geral de vocabulário cadastrado. |
| L09 | L02/L17; experiência prévia, estado persistente, tarefas estruturalmente novas, controle sem experiência. | Medir diferença positiva, nula ou negativa entre condições com referência e informação comparáveis. |
| L10 | L02/L09/L12; experiência sucessiva e representações compartilhadas identificáveis. | Avaliar desempenho antes/depois, retenção e interferência, distinguindo revisão deliberada de regressão espontânea. |
| L11 | L13/L17; relatórios completos acessíveis e estado equivalente. | Comparar integralmente relatórios entre repetições e processos em protocolo novo; registrar diferenças sem corrigir história. |
| L12 | Contrato de estado de circuito e política explícita de restauração, se autorizados. | Avaliar processo reiniciado e continuidade de estados/decisões frente a execução ininterrupta; serialização isolada não basta. |
| L13 | Identidade e sucessão contextual explícitas; fonte e referências de decisões. | Verificar versões, validade, suspensão/retração e histórico sem transferência automática de evidências. |
| L14 | Contratos de ação e autorização; limites observáveis. | Testar ações permitidas e recusadas, interrupção e ausência de efeitos indevidos; nenhuma utilidade estimada concede permissão. |
| L15 | Instrumentação e cargas representativas previamente definidas. | Medir custos por condição, tempo e memória com unidades e limites; não inferir custo exclusivo por subtração injustificada. |
| L16 | L02/L07/L09/L14; competência e domínio alvos definidos com autorização. | Comparar pré/pós e controles de configuração versus aprendizagem em exemplos inéditos; transferência requer ensaio próprio. |
| L17 | Arquivos primários disponíveis e referências independentes quando viáveis. | Verificar proveniência, partições, cegueira e controles; registrar ausência de fonte em vez de reconstruir evidência histórica. |

## Priorização recomendada e limitações

**Imediatos:** L17 (rastreabilidade e controles), L13 (identidade, contexto e evidências), L15 (medição) e L14 (fronteira de permissões), pois condicionam interpretações e testes seguros. L11 é verificação estreita e falsificável da lacuna H1; L12 depende de contrato e necessidade concreta antes de implementação. Evidência e custo de cada escolha devem ser avaliados em autorização futura.

**Intermediários:** L01–L08 conforme dependências e disponibilidade de mecanismos: revisão de critério, aprendizagem e inferência com referências adequadas, seguidas por investigação, pedagogia e linguagem verificáveis. Resultados já existentes oferecem ponto de partida, não garantia de viabilidade.

**Longo prazo:** L09, L10 e L16 requerem experiências, persistência e avaliações transferíveis que ainda não foram demonstradas. Ordenação é heurística e não sequência universal ou definição formal de pré-requisitos de especialização. Custos não são estimativas calibradas. Não se inicia a terceira subetapa nem se cria plano executivo.

EC-04–11 preservam seus pareceres AJUSTAR e limitações; a R3 EC-10 permanece ignorada, sem painel final; H1 EC-11 permanece parcialmente verificada. A1–A8 continuam diretrizes, não capacidades homologadas. A próxima subetapa nominal é **definir pré-requisitos de especialização**, não iniciada.

## Verificações documentais e regressão

`cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` passaram. Regressão: **185 testes aprovados, zero falhos e um ignorado**, o painel final R3 da EC-10; auxiliares e duplicações históricas não são novos experimentos. Três avisos históricos EC-04 de métodos não utilizados. Links locais conferidos; `/home` observado com cerca de 4,4 GiB livres, sem monitoração contínua.
