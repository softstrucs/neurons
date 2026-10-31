# V03-F1-P01 — Instrumentos para controles sintéticos cgroup v2

Base `29ea13a416e7c9cc45101c0f1ab3a4183812b161`, `devs` limpa e sincronizada. O usuário informou execução no Debian hospedeiro de scope com `memory.max=268435456`, `memory.current=1249280` e `memory.events` zerados. Trata-se de relato do usuário, não de medição independente desta sessão; ler o limite não valida sua aplicação.

Fontes: [D04-E03](protocolos/V03-F1-P01_D04-E03_memoria_cgroup_APROVADA.md), [diagnóstico anterior](V03-F1-P01_validacao_infraestrutura_cgroup.md), [manifesto prospectivo](protocolos/V03-F1-P01_especificacao_manifesto_pre_congelamento.md) e [decisões](decisions.md). Esta etapa cria instrumentos e testes puros; não executa controles de pressão, OOM, D/V sob contenção ou F.

## Instrumentos e controles

`tests/support/v03_f1_p01/cgroup_controls.py` é supervisor externo para um scope systemd transitório identificado com nonce aleatório. A carga interna solicita limite sintético de 48 MiB e no máximo 96 blocos de 1 MiB; valida o limite lido antes de iniciar a pressão, incluindo proteção contra execução direta fora do cgroup sintético esperado. O supervisor não pertence ao limite do scope: seu PID, comando e tempo são explicitamente registrados e sua memória não é incluída no teto. Cada execução cria nome próprio, sem modificar `user.slice` ou serviço existente.

O coordenador interno captura caminho do cgroup, `memory.max`, `memory.current`, `memory.peak` quando disponível, `memory.events` e PIDs antes/depois; registra tempo monotônico, estado dos subprocessos e diagnóstico JSON. Verifica processo leve e neto no mesmo cgroup (A/D), pressão limitada (B), diferença `oom_kill` e encerramento por SIGKILL conjuntamente (C), filho temporizado e encerrado (E/H), colheita dos filhos e lista de processos restante (F). O supervisor verifica após o retorno o estado do scope e `ControlGroup` vazio (G/J). Ausência de leitura, divergência, timeout ou evidência incompleta é **INCONCLUSIVO**; desaparecimento de scope que não possa ser comprovado também é inconclusivo. `memory.current` é instantâneo, `memory.peak` pode estar indisponível e `memory.max` não equivale a RSS nem assegura ausência de ultrapassagem transitória.

Os controles sintéticos podem produzir `CONTROLES_SINTETICOS_APROVADOS` somente se todas as verificações implementadas passarem; isso **não** declara contenção geral validada: D04-E03 também exige D/V sob contenção e avaliação do escopo integral. A referência de 256 KiB de dados vivos permanece não verificada. O script não aceita partições D/V/F nem contém caminhos para executar o núcleo.

## Execução futura no hospedeiro

**Não executada nesta etapa.** Quando a execução dos controles de pressão for autorizada e após verificar que há sessão systemd de usuário, cgroup v2 e recursos suficientes, no diretório do repositório executar no terminal do hospedeiro:

```sh
python3 -B tests/support/v03_f1_p01/cgroup_controls.py --run-host-controls
```

Preservar integralmente saída padrão, erros, versão do repositório, ambiente e status de saída para auditoria. Cada scope tem nome `neurons-v03-f1-` seguido de identificador aleatório; observar o nome impresso antes da execução. O teste é deliberadamente destrutivo **somente para seu processo sintético** e pode produzir evento OOM no próprio cgroup; não se deve executá-lo em grupo compartilhado ou caso haja insuficiência de recursos. O uso de `systemd-run --user --scope` não exige sudo nem persistência; falha de acesso torna o resultado inconclusivo. Nunca redirecionar saída para arquivo histórico.

Em timeout o supervisor tenta parar somente o scope criado e aguardar o processo. Após qualquer interrupção, conferir no hospedeiro a unidade única impressa e os processos associados; se ainda existir, parar **somente essa unidade** com `systemctl --user stop NOME.scope`, substituindo `NOME.scope` pelo nome registrado e verificado, e confirmar ausência de processos. Essa é a reversão dos recursos próprios; não parar `user.slice` ou unidades de terceiros. Se o controle não comprovar encerramento, sobreviventes ou limpeza, tratar como inconclusivo e investigar antes de qualquer repetição. As verificações não medem custos do supervisor nem autorizam alterar serviços globais.

## Verificações desta preparação

`python3 -B -m unittest discover -s tests/support/v03_f1_p01 -p test_cgroup_controls_safe.py -v` executou dois testes puros que usam registros fabricados apenas para testar a classificação, sem subprocessos, scope ou pressão: ambos passaram. Uma inspeção sintática por `ast.parse` e `--help` também passaram. Os registros sintéticos dos testes não são observações de cgroup. Nenhum controle real ou painel F foi executado.

Regressão: `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` passaram; **190 testes aprovados, zero falhos e um ignorado**, o painel R3 da EC-10. A suíte histórica não valida contenção e não executa F deste protocolo. Três avisos históricos de código não utilizado na EC-04 permanecem.

Pendências: execução futura autorizada dos controles no hospedeiro, exame das evidências e limpeza, eventual repetição de controles falhos ou inconclusivos com versão identificada, D/V sob contenção após sucesso, decisão de congelamento e autorização separada para F. A informação reportada pelo usuário não resolve essas pendências. F1 continua sem homologação experimental final.

## Correção prospectiva do controle C

Na execução da versão base `2a5d32f0189d829b03686a73939d4de6dfd5751b`, o usuário relatou `memory.max` e `memory.peak` de 50.331.648 bytes, aumento de 333 em `memory.events.max`, nenhum aumento de `oom` ou `oom_kill`, saída 1 por `RuntimeError`, controles A/B/D/E/F, limite e tempo verdadeiros e limpeza observada. **Resultado histórico inconclusivo**, não OOM e não validação de contenção.

A carga anterior criava blocos `bytearray` zerados sem escrever explicitamente em cada página. Uma explicação plausível é o adiamento ou compartilhamento das páginas zeradas; os contadores relatados não estabelecem isoladamente a causa exata, nem excluem comportamento do alocador, kernel ou cobrança. A nova versão usa mapeamentos anônimos retidos, até 96 blocos de 1 MiB, e escreve um byte em cada página conforme `SC_PAGE_SIZE`. O controle C exige agora conjuntamente incremento de `oom_kill`, término do processo de pressão por SIGKILL e confirmação de sua pertença ao cgroup. Falha de alocação, timeout ou ausência de evidência continua inconclusiva. O teto sintético, scope, supervisor e critérios históricos permanecem; nenhuma pressão foi executada nesta correção.

Verificações: três testes puros de instrumento passaram, incluindo escrita em amostra de 19 bytes e rejeição de evidência ausente ou pertencimento falso; `cargo fmt --check`, `cargo check --offline --all-targets` e `git diff --check` passaram. Para respeitar a proibição de executar D/V, a regressão foi `cargo test --offline -- --skip development --skip validation`: **133 aprovados, zero falhos, um ignorado e 57 filtrados**; não equivale à suíte integral. O R3 da EC-10 permaneceu ignorado. Controles reais e D/V sob contenção permanecem pendentes. O comando futuro de execução no hospedeiro permanece aquele acima, após autorização pertinente; sem novas evidências não se declara contenção validada.
