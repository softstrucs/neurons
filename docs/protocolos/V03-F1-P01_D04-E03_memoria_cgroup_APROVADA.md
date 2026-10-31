# V03-F1-P01 — D04-E03: métrica operacional de memória

**DECISÃO D04-E03 — APROVADA POR AUTORIZAÇÃO HUMANA EXPRESSA.** Base `166c7cbff16128748fcdf55adca6be937b629d2b`, branch `devs`. Esta decisão é prospectiva e aplica-se exclusivamente ao V03-F1-P01. Complementa [D04](../V03-F1-P01_decisoes_metodologicas_pre_finais_APROVADAS.md) e [D04-E02](V03-F1-P01_D04-E02_dados_vivos_APROVADA.md), sem alterar seus registros históricos, D01–D03, E01, o protocolo histórico ou o Plano v0.3.

## Métrica aprovada e limites

O critério operacional de memória deixa de usar RSS e passa a usar **memória cobrada ao controlador `memory` do cgroup v2**, com teto de **256 MiB = 268435456 bytes**. O mecanismo previsto é `memory.max` no hospedeiro ou equivalente que imponha efetivamente a contenção e cuja equivalência e abrangência sejam verificadas. RSS pode ser observado complementarmente, mas deixa de ser o critério operacional obrigatório deste protocolo.

`memory.max` limita memória cobrada ao cgroup, não equivale a RSS e não garante ausência absoluta de excedentes transitórios. `memory.current` informa a cobrança no instante da leitura; `memory.peak`, quando disponível, auxilia a observação; `memory.events` fornece contadores de eventos relevantes, incluindo eventos de pressão ou OOM conforme suporte do kernel. Registrar valores iniciais e finais, diferenças e limitações; identificar e registrar OOM, falhas de alocação, encerramento e causa conhecida sem atribuí-los automaticamente a falha científica. Ausência de evento OOM não prova ausência de todas as falhas. Cobrança de memória anônima, cache, memória compartilhada e demais componentes depende das regras do kernel; não é sinônimo de soma de RSS, memória virtual ou dados vivos.

O tempo total do painel permanece **inferior a 30 segundos**; verificar monotonicamente início, término e interrupção. Deve haver **pelo menos 3 GiB livres em `/home` imediatamente antes da execução**, verificados no ambiente efetivo. Conformidade operacional depende de validação real e não é declarada por esta emenda.

D04-E02 permanece aprovada: o teto de **256 KiB de dados vivos por condição** não é indispensável à alegação científica local e permanece **restrição não verificada**. Não foi medido com confiança e não pode ser declarado atendido; checkpoint ou memória de cgroup não o substituem.

## Escopo verificável da contenção

O grupo ou ambiente equivalente deve abranger o processo principal, filhos e descendentes, restauração de checkpoint, comparações entre processos e instrumentos auxiliares pertinentes durante toda a execução. Antes e durante a validação, identificar processos e comprovar sua pertença e a herança da contenção; detecção de processos experimentais relevantes fora do escopo exige interrupção ou tratamento explícito previamente documentado e justificado, sem afirmar contenção integral.

Um supervisor externo pode permanecer fora do grupo somente se sua função, custos e exclusões forem declarados, se ele não executar trabalho experimental relevante sem tratamento explícito e se tiver meios verificados de monitorar e encerrar integralmente o grupo, inclusive quando o processo principal falhar. Sua memória não deve ser contabilizada como coberta pelo teto. O início e fim do relógio devem cobrir a operação experimental e seus auxiliares; registrar também o tempo de supervisão e limpeza quando fora dessa janela.

Antes de eventual uso, criar somente ambiente isolado próprio em local delegado, verificar controlador `memory`, disponibilidade e permissões, configurar `memory.max`, ler de volta o limite efetivo e verificar limites ancestrais pertinentes. Identificar processos e descendentes; acompanhar `memory.current`, `memory.events` e, quando disponível, `memory.peak`; preservar valores, versões, timestamps, status de saída e motivo de interrupção. Em timeout, OOM, falha de fiscalização ou saída anômala, encerrar o conjunto de processos, verificar ausência de sobreviventes, preservar evidências e limpar apenas o ambiente próprio. Nenhuma configuração global é autorizada nesta etapa.

## Controles futuros obrigatórios

Os controles abaixo são **prospectivos e não executados nesta atividade**. Usar limites reduzidos e cargas sintéticas controladas, com versão e parâmetros fixados antes de cada validação, sem consumir desnecessariamente 256 MiB. O ambiente e as ferramentas devem ser identificados.

| Controle | Critério verificável futuro |
|---|---|
| A. Abaixo do limite | Processo sintético termina normalmente, limite efetivo é lido e a cobrança/eventos são registrados. |
| B. Acima do limite | Carga controlada provoca aplicação observável do teto, sem comprometer o hospedeiro; registrar resultado e limites da observação. |
| C. OOM ou falha de alocação | Identificar mecanismo de falha por status e eventos pertinentes; distinguir falha de alocação de OOM e registrar inconclusão se causa não for verificável. |
| D. Processos filhos | Confirmar inclusão de filhos e descendentes relevantes no mesmo escopo de cobrança, inclusive restauração e comparação. |
| E. Encerramento seguro | Interrupção por limite ou timeout encerra o conjunto sob controle, com motivo e estado registrados. |
| F. Sobreviventes | Verificar que nenhum processo pertinente permanece após interrupção; processos externos identificados devem ter tratamento previamente definido. |
| G. Limpeza | Remover somente recursos próprios após preservar evidências; demonstrar ausência de grupo ou artefato residual próprio. |
| H. Tempo | Relógio monotônico e teste de sensibilidade comprovam interrupção, incluindo auxiliares, com limite final inferior a 30 s. |
| I. D/V | Executar D e V sob contenção já validada; conferir integridade de consultas, comparações, adulterações e registros de recursos sem gerar F. |
| J. Evidências | Registrar versões, configuração efetiva, limite lido, contadores antes/depois, saídas, falhas, processos e resultados; preservar rastreabilidade. |

Classificações operacionais futuras: **CONTENÇÃO CONFIGURADA** quando o limite for instalado e lido, sem presumir testes; **CONTENÇÃO VALIDADA** somente com escopo e controles A–J satisfatórios; **CONTENÇÃO INDISPONÍVEL** quando não houver mecanismo acessível; **CONTENÇÃO INCONCLUSIVA** quando evidências forem insuficientes ou houver falha de observação. Somente **CONTENÇÃO VALIDADA** satisfaz este requisito operacional. Nenhuma classificação positiva decorre desta formalização.

## Estado da infraestrutura e prontidão

A [investigação anterior](../V03-F1-P01_investigacao_contencao_memoria.md) registrou Debian 12, controlador `memory` presente, cgroup v2 visível somente para leitura, delegação gravável não demonstrada, barramento de usuário inacessível, Podman/Docker indisponíveis e `RLIMIT_AS` insuficiente para limite agregado. O ambiente anteriormente investigado permanece **NÃO VALIDADO** para F; esta decisão não cria infraestrutura nem demonstra contenção.

| Item | Estado |
|---|---|
| D01 | APROVADA. |
| D02 | APROVADA COM RESTRIÇÕES. |
| D03 | APROVADA PARA O RECORTE. |
| D04-E02 | APROVADA. |
| D04-E03 | APROVADA DOCUMENTALMENTE. |
| Contenção efetiva | NÃO VALIDADA. |
| D/V sob contenção | PENDENTE. |
| Congelamento prospectivo | NÃO APROVADO. |
| Painel final F | NÃO AUTORIZADO. |
| F1 | SEM HOMOLOGAÇÃO EXPERIMENTAL FINAL. |

Os condicionantes C01–C05 permanecem aplicáveis; C05 exige congelamento prospectivo futuro. Nenhuma decisão aqui autoriza F, aprova o congelamento, prova capacidades cognitivas ou reclassifica resultados históricos.

Verificações de regressão desta formalização: `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` passaram; **190 testes aprovados, zero falhos e um ignorado** (`r3_frozen_final` da EC-10). A suíte pode repetir D/V sem contenção validada e não constitui execução de F. Permanecem três avisos históricos de métodos não utilizados na EC-04.
