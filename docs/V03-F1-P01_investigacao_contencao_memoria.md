# V03-F1-P01 — Investigação de contenção de memória

## Autorização e resultado

Base `c7b5244aecd40ed1051cf7afb9b58364d0b6b82f`, branch `devs`, inicialmente limpa e sincronizada com `origin/devs`. Investigação local sob as restrições de execução vigentes, sem privilégios administrativos, instalação, alterações globais ou painel F. Fontes: [plano aprovado](planos/Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_APROVADO.md), [protocolo original](protocolos/V03-F1-P01_confiabilidade_experimental_inicial_PROPOSTA.md), [E01](protocolos/V03-F1-P01_E01_identidade_S2_S3.md), [auditoria](V03-F1-P01_auditoria_prontidao_pre_final.md), [D01–D04](V03-F1-P01_decisoes_metodologicas_pre_finais_APROVADAS.md), [fechamento operacional](V03-F1-P01_fechamento_operacional_pre_congelamento.md), [manifesto prospectivo](protocolos/V03-F1-P01_especificacao_manifesto_pre_congelamento.md) e [decisões](decisions.md). A [D04-E02](protocolos/V03-F1-P01_D04-E02_dados_vivos_APROVADA.md) formaliza a decisão humana sobre dados vivos.

**Categoria C — CONTENÇÃO INDISPONÍVEL NO AMBIENTE ATUAL.** Nenhum mecanismo capaz de impor e validar aqui um teto de memória à árvore completa foi disponibilizado. Isso descreve o ambiente acessível nesta execução, não uma impossibilidade do host; uma delegação ou outro ambiente apropriado poderá tornar a contenção possível após autorização e validação próprias. A alternativa B é uma possibilidade condicional, não mecanismo demonstrado.

## Ambiente e mecanismos

| Mecanismo | Evidência observada | Estado e limite |
|---|---|---|
| Linux/cgroup v2 | Debian GNU/Linux 12, kernel `6.1.0-52-amd64`, UID 1000; `cgroup.controllers` contém `memory`; hierarquia do processo em `/user.slice/user-1000.slice/user@1000.service/app.slice/app-com.microsoft.VSCode-21749.scope`. | Controlador presente não implica delegação. Montagem `/sys/fs/cgroup` visível é `ro`; teste de escrita negativo. Não foi possível criar grupo ou impor `memory.max`. |
| Systemd do usuário | `systemd-run` instalado; socket de usuário visível. `systemctl --user show ...` retornou `Failed to connect to bus: Operação não permitida`. | Permissão efetiva indisponível neste ambiente; existência do socket não demonstra acesso nem delegação. Nenhum serviço ou scope foi criado. |
| Namespaces | `unshare --user --map-root-user true` retornou sucesso em controle vazio. | Namespace de usuário não transforma a montagem de cgroup somente leitura em delegação gravável; não foi usado para contornar restrições do ambiente. |
| Podman/Docker | Executáveis não localizados no PATH; sockets usuais rootless e `/var/run/docker.sock` ausentes. | Nenhum contêiner rootless disponível para validar limite no host. Não se testou acesso externo ou se instalou software. |
| `prlimit`/`RLIMIT_AS` | `prlimit` disponível. Controle filho sob 32 MiB de espaço de endereçamento tentou alocar 24 MiB e encerrou com `MemoryError`. | Limite efetivo por processo herdado, sobre memória virtual; não é RSS, nem teto agregado de uma árvore de processos. Não satisfaz D04. |
| Fiscalização amostrada | [`watchdog.py`](../tests/support/v03_f1_p01/watchdog.py) e controles históricos no relatório anterior. | Observa soma amostrada de RSS do grupo; processos curtos ou que mudem de grupo podem escapar. Não é contenção rígida. |

`/proc/self/status` registrou capacidades efetivas nulas, `NoNewPrivs: 1` e seccomp ativo. Essas observações são locais e não estabelecem política global do host. `/home` apresentava 4.105.703.424 bytes disponíveis na inspeção; verificar novamente imediatamente antes de eventual execução.

## Métrica e controles

Nenhum limite de cgroup foi imposto nesta investigação; `memory.current`, `memory.events`, `memory.max`, reação a OOM, inclusão de filhos, ausência de sobreviventes e limpeza de grupo **não foram validados**. Portanto não se executaram controles sintéticos fingindo contenção por cgroup nem D/V sob contenção supostamente efetiva. O único controle novo foi o de `RLIMIT_AS` descrito acima, com alocação controlada e sem envolvimento de F. As verificações de regressão podem repetir D/V sem proteção de cgroup e não devem ser apresentadas como D/V sob contenção.

Em eventual cgroup v2 delegado, seria necessário criar somente grupo próprio, verificar controlador e escrita, impor `memory.max`, incluir pai e descendentes, acompanhar `memory.current` e `memory.events`, testar abaixo/acima do limite e OOM com cargas pequenas, controlar tempo, confirmar ausência de sobreviventes e remover o grupo. A memória cobrada ao cgroup pode incluir memória anônima, cache e outras parcelas contabilizadas pelo kernel; sua relação com memória compartilhada depende da contabilização. Não equivale ao RSS por processo, à soma amostrada de RSS, ao pico simultâneo garantido, a espaço virtual ou aos dados vivos por condição. `memory.max` é um limite de cobrança, não uma medida de pico RSS; evento OOM não é automaticamente um resultado científico negativo.

## Pendências e decisão humana

D04-E02 retira os 256 KiB dos critérios indispensáveis à alegação local; a restrição permanece **NÃO VERIFICADA**. Persistem teto operacional de 256 MiB, tempo inferior a 30 s e espaço livre de 3 GiB; não se declara conformidade futura. Antes de congelar, obter ambiente acessível com contenção suficiente e validar controles e D/V nesse ambiente; definir escopo e tratamento de falhas. Substituir formalmente RSS por memória cobrada ao cgroup requer decisão humana específica. O protocolo não está congelado; F permanece bloqueado e exige autorização específica. Nenhuma conclusão amplia C01–C05 ou homologa F1, aprendizagem, autonomia ou A8.

Verificações de regressão: `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` passaram; **190 aprovados, zero falhos e um ignorado** (`r3_frozen_final`, EC-10). A suíte repetiu testes históricos e D/V como regressão, não como ensaio sob contenção. Permanecem três avisos históricos de código não usado na EC-04. Não houve modificação de código, instrumentos ou testes.
