# EC-11 — Resultados de confiabilidade integrada

Base `501b928ad85f26b56cad6e1afb004d2318151282`. **AJUSTAR; conformidade parcial**, terceira subetapa executada como avaliação delimitada; EC-11 continua aberta e aceite global não homologado. [Protocolo prospectivo](protocolos/EC-11_confiabilidade_integrada.md) e [instrumento](../tests/ec11_reliability.rs) foram escritos e seus hashes registrados antes do primeiro painel: respectivamente `5e54c37d051445700e4774fa0b39e5557eed33b7cfe9484a1646e46b92ffad72` e `7fd7c4df2d72a9ac43dbe459269937d5502cf002c37f39f4f8fd9136eb2522eb`. Conferidos novamente após o painel, sem alterações. Hashes não são pré-registro externo independente.

## Resultados do painel

Duas repetições da mesma topologia; cada uma com dois registros e dois eventos. Em ambas: checkpoint de **376 bytes**, bytes determinísticos, resposta com premissas 1/2, evidência 11 preservada, abstenção em contexto sem suporte, abstenção por orçamento de revisão, rejeição de contexto candidato inválido, preservação do checkpoint após consultas, rejeição de fila cheia e despacho ordenado 10/20. Checkpoints corrompido e acima de 256 KiB foram rejeitados; objeto original permaneceu inalterado. O controle de memória vazia absteve-se. Os critérios são programados e compartilhados com o avaliador, não referência externa independente.

Cada checkpoint foi gravado em arquivo temporário, lido e verificado por um **processo separado de teste**, com nova configuração equivalente de monitor. Isso demonstra recuperação funcional do cenário e dos campos cobertos, não reinício do serviço completo ou persistência de `Circuit`, pesos, estado neuronal e configuração do monitor. O filho valida a ordem, decisões e ausência de mutação antes do despacho. Há dois processos filhos e duas execuções equivalentes; não são duas topologias independentes. `child_restore` sem variável de ambiente é apenas auxiliar inerte da suíte, não um resultado adicional de restauração.

| Dimensão | Classificação | Evidência e fronteira |
|---|---|---|
| Regressão/reprodutibilidade | Demonstrada no contrato e cenário | Bytes e decisões equivalentes nas duas repetições; nenhuma alegação de generalização. |
| Recursos | Parcialmente demonstrada | Limites observados abaixo dos tetos no painel; sem isolamento de custos do adaptador, amostragem contínua ou análise de escala. |
| Rastreabilidade | Parcialmente demonstrada | IDs públicos, evidência 11, contexto 1, eventos e resposta verificados; versões sucessivas, proveniência interna e histórico não expostos não avaliáveis. |
| Segurança/integridade | Parcialmente demonstrada | Corrupção, tamanho, contexto inválido, fila cheia e imutabilidade de consulta testados; FNV não autentica nem representa segurança geral; não foram testadas todas as classes de estados inconsistentes. |
| Retenção/restauração | Demonstrada apenas para registros, fila e consulta fixa | Arquivo e processo separados com monitor reconstruído; circuito/pesos e retenção neuronal após reinício não avaliáveis. Não equivale a retenção cognitiva aprendida. |
| Autonomia limitada | Parcialmente avaliável | Limite de tentativas, fila e abstenção observados; não existe política integrada de permissões nem decisão autônoma de investigar, portanto essas condições não são avaliáveis. |
| Revisão/retração | Parcialmente avaliável | Abstenção por limite em contextos alternativos; revisão de conhecimento persistido, sucessão de versões e retração não expostas. |

## Recursos e verificações

Com relógio monotônico, checkpoint levou 16 e 10 µs; restauração 9 e 8 µs; sequência interna duas repetições 2.389 µs, incluindo criação de processos. São amostras únicas sensíveis a ambiente e instrumentação; incluem componentes e não isolam adaptador. Medição externa Python `resource.getrusage(RUSAGE_CHILDREN)` e relógio monotônico: 0,003726 s, pico observado 11.208 KiB; este valor pode refletir memória herdada/instrumentação, não atribui RSS a um componente nem comprova amostragem contínua. `/usr/bin/time` não estava disponível. `/home` observado com aproximadamente 4,4 GiB livres; sem monitoração contínua. Dois eventos foram enfileirados, dois processados e duas tentativas de enfileiramento excedente rejeitadas no painel, além das verificações repetidas nos processos filhos.

`cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline`, `git diff --check` passaram. Regressão: **185 aprovados, zero falhos, um ignorado**, incluindo o painel novo, auxiliar sem variável e duplicações históricas. O ignorado é `r3_frozen_final` EC-10, não um resultado aprovado. Três avisos históricos EC-04 de métodos não utilizados. Regressões históricas não são novos experimentos.

EC-08 não demonstrou investigação autônoma nem metacognição geral; EC-09 não demonstrou ciclo pedagógico contínuo; EC-10 não homologou transferência ou cumulatividade. A1–A8 permanecem diretrizes, em especial A8 como objetivo não demonstrado. Nenhuma conclusão de aprendizagem neural, autonomia geral ou competência generalizável decorre deste painel. O plano não lista outra subetapa nominal dentro da EC-11; EC-12 — Consolidação, especialização futura e questões abertas — não foi iniciada. Encerramento da EC-11 não é decidido aqui.
