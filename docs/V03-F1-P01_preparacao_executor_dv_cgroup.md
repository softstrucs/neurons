# V03-F1-P01 — Preparação do executor D/V sob cgroup

Base `375fef13b5442fa480a25760187c179927763d35`, `devs` inicialmente limpa e sincronizada. O usuário informou **9/9 controles sintéticos aprovados no hospedeiro**, política `continue` confirmada, `memory.max=50331648`, `memory.swap.max=0`, `oom_kill=1`, pressão −9, evidência completa e limpeza, duração 1,51 s. É relato do usuário preservado como tal; não foi repetido nem auditado independentemente aqui. [D04-E03](protocolos/V03-F1-P01_D04-E03_memoria_cgroup_APROVADA.md), [D04-E02](protocolos/V03-F1-P01_D04-E02_dados_vivos_APROVADA.md), [D01–D03](V03-F1-P01_decisoes_metodologicas_pre_finais_APROVADAS.md), [protocolo](protocolos/V03-F1-P01_confiabilidade_experimental_inicial_PROPOSTA.md) e [histórico](V03-F1-P01_instrumentos_controles_cgroup.md) permanecem inalterados.

## Arquitetura e contrato

`tests/support/v03_f1_p01/dv_cgroup_executor.py` prepara um supervisor externo e coordenador num scope systemd transitório próprio com `MemoryMax=268435456`. A interface pública exige a opção explícita `--execute-dv-with-explicit-authorization`; ela expressa a escolha operacional e **não é, por si, autorização humana**. Esta atividade não autoriza usá-la. Não há opção para F; as únicas seleções do binário são `--exact development` e `--exact validation`. O gerador existente continua rejeitando partições fora de D/V.

Antes de qualquer consulta, supervisor e coordenador verificam o `memory.max` efetivo; o supervisor confronta unidade ativa, caminho e PID no cgroup e somente então envia liberação. O coordenador confere liberação, PID e caminho; antes das partições revalida o teto e exige pelo menos 3 GiB disponíveis em `/home`. O supervisor verifica esse espaço antes de criar o scope e novamente antes da liberação. O relógio monotônico externo inicia antes do scope e impõe prazo inferior a 30 segundos para a execução e processos auxiliares; timeout tenta parar somente o scope próprio. Limpeza e consultas posteriores têm tempo separado registrado, sem alegar que ocorreram dentro dos 30 segundos.

O cgroup abrange coordenador, processo de teste, referência e comparações executadas no processo de teste, e os filhos lançados por ele para restaurar checkpoint. O supervisor e seus comandos `systemctl` ficam fora: sua função, PID, duração e exclusão de memória são registrados; não fazem inferência nem escolhem respostas. São capturados versão do executor por SHA-256, parâmetros do comando, hash e caminho do binário D/V, limites ancestrais legíveis, espaço, `memory.current`, `memory.peak` quando disponível, `memory.events`, PIDs observados, saídas e códigos. Não se trata bytes de checkpoint ou RSS como memória do cgroup; 256 KiB de dados vivos continuam restrição não verificada.

O processo de teste é verificado explicitamente no cgroup e a composição do grupo é amostrada a cada 2 ms, com erros e PIDs não verificáveis registrados. **Limitação impeditiva:** filhos de restauração podem existir menos que o intervalo e a amostragem não comprova todos os descendentes ou ausência de escape. A herança usual do cgroup não substitui evidência individual. Por isso, mesmo se D/V forem futuramente executados, esta versão marca `descendant_membership=NAO_VERIFICADO_INTEGRALMENTE` e retorna **INCONCLUSIVO**, sem homologar contenção nem resultados por esse motivo. Os relatórios completos e as contagens impressas pelo teste permanecem disponíveis para avaliação, sem convertê-las em aprovação automática. Se houver falha, teto divergente, timeout ou limpeza não comprovada, registrar inconclusão; a limpeza tenta parar somente o scope próprio, sem presumir sucesso.

## Segurança e verificação desta preparação

Não se executaram D/V, F, EC-10 R3, pressão real, scope, congelamento ou alterações globais; não houve alterações em `src/` ou nos critérios científicos. Testes seguros de `test_dv_cgroup_executor_safe.py` usam mocks e arquivos temporários para verificar aceitação exclusiva de D/V, recusa por limite ou identidade ausentes, bloqueio sem liberação e recusa de espaço insuficiente, sem criar processos experimentais. `--help` apenas apresenta a opção; seu uso não valida o executor.

Quatro testes seguros passaram. `cargo fmt --check`, `cargo check --offline --all-targets` e `git diff --check` passaram. Regressão `cargo test --offline -- --skip development --skip validation --skip frozen --skip final --skip panel --skip protocol`: **98 aprovados, zero falhos e 93 filtrados**; D/V, F e R3 não executados. Três avisos históricos de métodos não usados na EC-04 permanecem. A regressão não substitui testes sob cgroup.

## Pendências para autorização de D/V

Auditar o executor e sua confirmação com systemd real; definir e implementar, sob autorização apropriada, observabilidade comprovável para todos os filhos de restauração e sobreviventes, sem inferir completude da amostragem; validar que interrupção e limpeza do grupo são efetivas, inclusive falhas e limites ancestrais. Reavaliar a prontidão operacional, registrar versões e recursos e obter **autorização humana específica posterior para executar D/V**. Se não for possível cumprir a rastreabilidade de descendentes, preservar resultado inconclusivo. Congelamento prospectivo não está aprovado; F não está autorizado e nenhuma competência foi homologada.

**Estado: controles sintéticos aprovados no hospedeiro conforme relato; executor D/V em preparação com limitação aberta; D/V sob contenção não executados; contenção D04-E03 integral não homologada.**

## Instrumentação prospectiva dos descendentes

Base `2529e0035743d0292d0c2feb0d06e0f502dab5d1`. A inspeção de `tests/v03_f1_p01_instruments.rs` identificou **um ponto centralizado** de criação dos filhos de restauração: o `Command` do próprio binário para `child_process`, uma vez para cada um dos seis casos em três repetições. A criação do processo de teste é feita pelo coordenador Python; o scope é iniciado pelo supervisor externo, que também usa `systemctl` para controle e limpeza. Esses processos externos são supervisão, não restauração ou inferência. Não foram identificados outros pontos explícitos de criação de processos experimentais nesse harness; isso não prova ausência em bibliotecas externas ou outros executores.

Quando habilitada pelo executor, a nova instrumentação é **opt-in**: o processo de teste comunica PID, PID do criador, ticks de início e cgroup e aguarda liberação do coordenador antes de qualquer consulta D/V. O coordenador confere `/proc` enquanto o processo está vivo. Cada filho de restauração envia identidade e aguarda o pai ler seu PID real, pai, ticks e cgroup, compará-los com a execução e com o grupo esperado e só então liberar a restauração. Se identidade, cgroup ou comunicação forem ausentes, divergentes ou incompletos, a sequência falha; um guard tenta matar e recolher o filho em falha normal ou unwind. Para cada filho concluído, o pai emite registro com run ID, partição/repetição/caso, papel, PID, criador, ticks, cgroup, tempos de criação/entrada/término, duração monotônica e código ou sinal. O coordenador exige 18 registros únicos por partição, com identidades, tempos, saída e cgroup coerentes, antes de prosseguir; registros ausentes, duplicados ou divergentes são rejeitados. Os registros são encaminhados ao supervisor externo no relatório; amostragem continua apenas complementar.

O handshake mantém o processo vivo durante a comparação e o par PID/ticks mitiga reutilização de PID; `ppid` e cgroup são verificados no momento da liberação. Filho curto sem handshake não consegue executar a restauração na modalidade rastreada. Uma falha de pipe, inicialização, timeout, OOM, morte abrupta do pai, filho órfão, escape posterior de cgroup ou remoção da unidade exige interrupção, limpeza e classificação inconclusiva; guard em unwind não garante limpeza após SIGKILL. O relógio de parede para instantes pode sofrer ajuste; duração monotônica é registrada separadamente e tempos incoerentes são rejeitados. O supervisor só recebe o relatório de uma partição após seu término, não uma prova em tempo real independente de todos os processos. Não há autenticação criptográfica de mensagens nem prova de que terceiros ou bibliotecas não criem processos não instrumentados. Não se declara cobertura integral fora dos caminhos identificados.

Sete testes Python seguros passaram para identidade de teste, registros completos, ausentes, duplicados, divergentes e liberação ausente; um teste Rust puro foi executado com filtro exato para validar falhas de comunicação, sem iniciar D/V. `cargo fmt --check`, `cargo check --offline --all-targets` e `git diff --check` passaram. Regressão `cargo test --offline -- --skip development --skip validation --skip frozen --skip final --skip panel --skip protocol`: **99 aprovados, zero falhos e 93 filtrados**; D/V, F e R3 não executados. O harness alterado preserva os caminhos históricos quando a variável de rastreio não está definida. Nenhuma execução real do handshake foi feita nesta etapa.

O executor continua deliberadamente a retornar **INCONCLUSIVO** após coletar os registros: falta validar o handshake, a integridade completa das saídas, descendentes e limpeza no hospedeiro real sob contenção; dependem de autorização humana posterior para D/V. A observabilidade preparada não homologa D04-E03, não aprova congelamento e não autoriza F.

## Validação sintética segura do handshake e limpeza

Base `03747ce5e81307391650e85ab8db94a711d58343`. A auditoria confirmou um ponto `Command` para restauração, dentro de seis casos e três repetições: **18 oportunidades de registro por partição** no caminho inspecionado. O teste sintético não executa esses casos nem carrega checkpoints. O comparador de registros exige exatamente esse conjunto, IDs e pares PID/ticks únicos, criador, cgroup, tempos e saída válidos; isso não demonstra caminhos não inspecionados.

Foram executados processos auxiliares reais de curta duração no cgroup já existente da sessão, sem scope novo ou pressão: handshake válido com leitura real de `/proc`, confirmação de PID, geração, pai, cgroup, identidade e liberação antes do trabalho; saída normal; e carga de 128 KiB em `stderr`. Foram exercitados ausência ou divergência de identidade, término prematuro, marcador duplicado, canal de entrada fechado, liberação inválida e timeout. Os filhos diretos desses testes foram aguardados e sua ausência em `/proc` verificada. Um teste Python criou pai e descendente sintéticos, detectou o descendente ativo, encerrou-o por meio do pai e confirmou que foi recolhido. Os testes de registros ausentes/duplicados e cgroup divergente são também exercitados por dados simulados, não por D/V.

A auditoria identificou bloqueio potencial ao esperar `stdout` enquanto `stderr` enchia e ausência de prazo local na espera do filho. O caminho rastreado agora drena ambos em paralelo, limita dados retidos e aplica prazo local de três segundos; falhas normais e unwind acionam guarda que tenta matar e recolher o filho direto. O executor rejeita lista final com processo além do coordenador. O primeiro teste sintético falhou porque o simulador usou `sys.stdin.close()` sem fechar o descritor efetivo; após correção para `os.close(0)`, repetição passou. O resultado anterior não foi apresentado como sucesso.

Limites: a leitura de identidade do processo de teste pelo coordenador Python ainda pode bloquear em `readline` ou por `stderr` antes da liberação; o supervisor externo dispõe do prazo global e tenta parar o scope, mas isso não foi validado sob systemd nesta atividade. Leituras muito grandes sem quebra de linha, falha de comunicação do supervisor, cgroup inacessível, OOM, SIGKILL do pai e netos que escapem da contenção não foram comprovados em ambiente real. A guarda local não garante recolher descendentes órfãos após morte abrupta. O teste de descendente ativo verifica o caso sintético controlado, não todos os caminhos de falha. PID/ticks e cgroup observados são verificações locais, não autenticação externa. O executor continua **INCONCLUSIVO**.

Verificações finais: dois testes Rust de processos sintéticos passaram (`trace::tests::safe_`, execução serial, cinco testes do binário filtrados); oito testes Python seguros passaram. `cargo fmt --check`, `cargo check --offline --all-targets` e `git diff --check` passaram. Regressão `cargo test --offline -- --skip development --skip validation --skip frozen --skip final --skip panel --skip protocol`: **102 aprovados, zero falhos e 93 filtrados**; D/V, F e R3 não executados. Os controles sintéticos cgroup seguem aprovados no hospedeiro apenas conforme relato anterior; não foram repetidos aqui. Falta validar handshake e limpeza no hospedeiro sob cgroup apropriado com autorização específica, depois avaliar D/V; nenhum congelamento ou F autorizado.

## Supervisão externa e limpeza prospectiva

Base `e55209f77a5cba0fcbbe8502b8e408c99ee0083f`. A auditoria encontrou `readline()` sem prazo na espera de `V03TEST_READY` e possível bloqueio de `stderr` antes da liberação. O coordenador agora usa leitura por descritor com deadline monotônico e teto de 64 KiB para a identidade; `stderr` vai para arquivo temporário próprio, evitando encher um pipe. Timeout, EOF ou excesso impedem liberação. O supervisor externo mantém prazo global e aciona somente `systemctl --user stop` para nome próprio validado; reconsulta estado e registra falha de comando, unidade `failed` ou limpeza não comprovada como **INCONCLUSIVO**. Não envia sinais a grupos arbitrários. Se stop falhar, a ausência de sobreviventes não é presumida. Os arquivos temporários de `stderr` são anônimos e encerrados pelo processo.

Foi preparado `tests/support/v03_f1_p01/synthetic_scope_cleanup.py`: um supervisor externo cria um scope sintético próprio de 64 MiB, com coordenador, filho e neto sem checkpoints; registra identidades e cgroup, provoca saída abrupta do coordenador e solicita stop apenas do scope gerado. Registra estado e PIDs ainda presentes; resultado permanece **INCONCLUSIVO** para inspeção. É um ensaio prospectivo, não valida limpeza automaticamente. Não executado aqui porque `systemctl --user show` retornou `Operação não permitida` nesta sessão. Não se criou scope, não se alterou configuração global e não houve pressão/OOM ou D/V.

Para futura execução **com autorização humana específica**, no terminal do hospedeiro:

```sh
python3 -B tests/support/v03_f1_p01/synthetic_scope_cleanup.py --run-host-synthetic
```

O teste provoca falha intencional do coordenador sintético; registrar integralmente nome único do scope, saídas, retorno, PIDs e evidências. Se stop falhar ou houver sobreviventes, interromper e investigar; no hospedeiro, o operador pode parar somente a unidade própria impressa com `systemctl --user stop NOME.scope` após conferir sua identidade, sem agir sobre grupos compartilhados. Não executar no sandbox para contorná-lo. A evidência de processos após término é limitada por acesso a `/proc`, reutilização de PID e estados zumbis; falha de leitura ou identidade mantém inconclusão. O mecanismo de stop não depende da cooperação do coordenador, mas sua eficácia real sob systemd ainda não foi demonstrada aqui.

O stop automático exige tanto o padrão de nome próprio quanto confirmação prévia da identidade e do pertencimento do scope; nome aleatório isolado não autoriza interromper uma unidade. Se a confirmação não ocorrer, o instrumento não emite stop e registra inconclusão, podendo exigir inspeção manual da unidade impressa. Nenhum sinal global é enviado.

Verificações: 11 testes Python seguros passaram, incluindo pipes reais com deadline, EOF e marcador válido, falha simulada de stop, estado `failed` e recusa de unidade externa; dois testes Rust sintéticos de handshake passaram. `cargo fmt --check`, `cargo check --offline --all-targets` e `git diff --check` passaram. Regressão filtrada `cargo test --offline -- --skip development --skip validation --skip frozen --skip final --skip panel --skip protocol`: **102 aprovados, zero falhos e 93 filtrados**; D/V, F e R3 não executados. A limpeza após falha abrupta sob scope real permanece pendente de execução autorizada; executor D/V permanece inconclusivo, sem congelamento ou F.

## Recuperação após confirmação parcial

Base `ee45ec24088c378248e9d349f46222211e1986c0`. A versão anterior somente autorizava a limpeza depois de confirmar todos os descendentes a partir do marcador deles. Uma falha antes desse marcador podia deixar um scope próprio ativo sem tentativa de stop, mesmo que fosse possível identificá-lo por evidências independentes.

A versão preparada separa três condições: **propriedade da unidade**, **liberação do trabalho** e **limpeza**. Antes de liberar o coordenador sintético, o supervisor confronta o PID do processo que ele próprio lançou, seu criador e ticks de início lidos duas vezes em `/proc`, cgroup, estado ativo, caminho e `InvocationID` não vazio da unidade consultada ao systemd. Essa prova não depende do marcador dos descendentes. Só então envia uma liberação ligada a PID, ticks, unidade e cgroup; o coordenador valida esses campos antes de criar trabalho sintético. Marcador posterior ausente, parcial ou divergente não autoriza trabalho adicional, mas não descarta a prova independente já obtida para limpeza.

Antes de `systemctl stop`, o instrumento exige prova tipada pertencente ao supervisor, nome próprio válido e nova consulta com **mesmo `InvocationID` e cgroup**; falha de consulta, mudança de unidade ou prova ausente impedem stop e produzem diagnóstico inconclusivo. A consulta e o stop são operações distintas: há uma janela residual de corrida, portanto não se alega ligação atômica ao `InvocationID` nem proteção contra substituição adversarial entre as chamadas. Se esse risco não for aceitável para uma execução, o stop automático não deve ser usado até haver mecanismo apropriado. Falha de stop, unidade `failed`, grupo desconhecido e limpeza não comprovada permanecem inconclusivos. A verificação posterior distingue sobrevivente com mesmos PID/ticks de PID reutilizado e registra identidades desconhecidas; marcador ausente não é interpretado como lista vazia comprovada. O executor D/V usa a mesma prova independente para suas tentativas de limpeza, sem autorizar D/V nesta atividade.

O teste real de scope no hospedeiro **não foi executado**. Passaram 16 testes Python seguros com mocks e processos locais, incluindo falha antes da confirmação dos três processos, marcador ausente ou divergente, coordenador encerrado, PID reutilizado, barramento/stop indisponíveis e recusa de unidade externa; dois testes Rust sintéticos passaram. `cargo fmt --check`, `cargo check --offline --all-targets` e `git diff --check` passaram. Regressão filtrada `cargo test --offline -- --skip development --skip validation --skip frozen --skip final --skip panel --skip protocol`: **102 aprovados, zero falhos e 93 filtrados**; D/V, F e R3 não executados. A comprovação do comportamento real de `InvocationID`, timeout, descendentes e limpeza no hospedeiro continua pendente de autorização específica; não há homologação da contenção.

## Identidade do lançador e do coordenador

Base `2e471c79aafd3ece5d9b3b7552f13390ab67ed5d`. A auditoria do [código upstream systemd v252](https://github.com/systemd/systemd/blob/v252/src/run/run.c), família do Debian 12, identificou que `transient_scope_set_properties()` registra `getpid_cached()` em `PIDs` e `start_transient_scope()` termina com `execvpe()`. Nesse caminho local `--user --scope`, o lançador é substituído pelo comando, preservando PID e ticks; não há necessariamente dois processos simultâneos. A hipótese de que a igualdade sempre falharia não se confirma nesse código. Não se executou systemd-run nem scope real para confirmar o comportamento do pacote instalado.

A prova agora distingue supervisor, lançador e coordenador explicitamente. O lançador vem de `Popen`; seu parentesco com o supervisor, geração e cgroup são lidos antes/depois. O coordenador é descoberto por `cgroup.procs` do caminho consultado no systemd, sem PID fornecido por marcador: exige candidato único vivo com argumentos exatos do Python/script/modo esperado, parentesco e PID/ticks/cgroup estáveis, com nova leitura antes de concluir a prova. Aceita o mesmo PID após exec ou filho direto do lançador com PID distinto; nesse segundo caso não exige que o lançador pertença ao scope. Outros arranjos de parentesco, processos inacessíveis, ausência, ambiguidade ou saída durante atestação não produzem prova. Argumentos e parentesco são evidência local combinada, não autenticação contra processos adversariais com privilégios equivalentes.

`ScopeOwnership` registra separadamente PID/ticks do lançador e PID/ticks/pai do coordenador, supervisor, unidade, cgroup e `InvocationID`. Liberação e marcador são confrontados com a identidade do coordenador. A prova completa é registrada antes da liberação sintética e pelo supervisor D/V prospectivo. Identificação incompleta impede liberação e stop automático; uma prova já concluída permanece utilizável após falhas posteriores de comunicação ou saída do coordenador, sujeita à reconsulta da mesma unidade antes do stop. Logs de liberação, propriedade e resultado de limpeza permanecem distintos. Não se presume ausência de descendentes quando falta marcador.

Passaram **19 testes Python seguros**, incluindo descoberta por evidência do kernel simulada, lançador fora do cgroup com coordenador distinto, argumentos/parentesco/cgroup divergentes, múltiplos candidatos, coordenador encerrado durante atestação e os caminhos anteriores de falha antes/depois da prova, marcador ausente e substituição de unidade. Passaram dois testes Rust sintéticos, `cargo fmt --check`, `cargo check --offline --all-targets` e `git diff --check`. Regressão filtrada: **102 aprovados, zero falhos e 93 filtrados**. Permanecem três avisos históricos de métodos não usados na EC-04.

Limitações: não há prova antes de identificar um coordenador vivo; se ele morrer antes dessa etapa, a limpeza automática é recusada e requer inspeção humana. A janela entre consulta de `InvocationID` e stop continua não atômica. Não há suporte presumido a coordenadores reparentados ou wrappers arbitrários. Validação real de lançamento, prazos e limpeza sob systemd permanece pendente. Não se executaram scope real, D/V, F, EC-10 R3 ou pressão/OOM; `src/` e critérios científicos não foram modificados.

**Estado: controles sintéticos aprovados no hospedeiro conforme relato anterior; limpeza após falha abrupta com validação real pendente; D/V não executado; D04-E03 não homologada; congelamento não aprovado; painel F não autorizado.**

## Correção consolidada F1, F2 e F3

Base `5eabe7342e9d6f594393c987c2f9aaad15130e6e`. Nenhum ensaio no hospedeiro ou scope real foi executado nesta atividade.

**F1 — finalização independente.** `close_channel()` converte BrokenPipeError/OSError e canal inválido em diagnóstico; canal já fechado é aceito. `finish_owned_scope()` preserva a prova recebida e tenta `stop_owned_scope()` independentemente desses erros, registrando separadamente `channel_close` e `cleanup`, inclusive se ambas falharem. O sintético usa essa finalização no finally. A auditoria confirmou o mesmo problema no supervisor D/V: o fechamento do preflight também passa a ser diagnosticado, e a finalização com tentativa de limpeza ocorre no finally tanto no timeout quanto na saída normal ou erro. Essa tentativa permanece sujeita à prova de propriedade e reconsulta; não envia sinais nem autoriza parada de unidade externa. Garantia restrita às falhas de canal tratadas, não à eficácia do stop ou à morte do próprio supervisor.

**F2 — leitura limitada.** As duas sequências select/readline do coordenador sintético foram substituídas por `read_limited_line()`: descritor temporariamente não bloqueante, selector, deadline monotônico absoluto, teto de 65.536 bytes incluindo newline, EOF vazio/parcial explícito e UTF-8 estrito. Linha parcial com escritor aberto expira; EOF antes do newline é falha. O modo original do descritor é restaurado. A liberação do worker D/V também usa esse leitor, dentro do prazo já existente de 30 segundos. JSON inválido ou liberação que não seja objeto não libera trabalho. Nenhuma thread ou processo de leitura foi introduzido. Os auxiliares reais dos testes seguros são encerrados e recolhidos em finally.

**F3 — representação canônica.** `live_process()` continua devolvendo caminho completo do filesystem. A comparação agora usa `cgroup_relative()`, cujo resultado é o caminho absoluto na hierarquia do controlador, como `/user.slice/.../nome.scope`, equivalente ao campo `0::` do kernel. Caminho completo `/sys/fs/cgroup/...` é convertido para esse formato; caminho da hierarquia já nesse formato é aceito; componentes ponto/ponto-ponto e caminhos sem barra inicial são recusados. `verify_live_test_ready()` confronta o processo vivo, PPID, ticks, pertencimento esperado e identidade do marcador. Somente o campo cgroup é normalizado: run ID, PID, PPID e ticks continuam exigindo igualdade. A verificação prévia de inclusão no scope e o teto de memória permanecem.

Verificações: **27 testes Python seguros do executor e 10 dos controles**, dois testes Rust sintéticos, `cargo fmt --check`, `cargo check --offline --all-targets` e `git diff --check` passaram. Regressão `cargo test --offline -- --skip development --skip validation --skip frozen --skip final --skip panel --skip protocol`: **102 aprovados, zero falhos e 93 filtrados**. Os comandos Rust removeram do próprio ambiente as variáveis opcionais de caso/checkpoint D/V e trace para impedir ativação incidental. Permanecem três avisos históricos EC-04. Os dois testes Rust específicos também integram a regressão, não são cobertura independente somável.

Cenários novos: BrokenPipeError/OSError no fechamento com prova preservada, falha simultânea de fechamento e stop, canal já fechado, falha de stop/barramento, finalização do supervisor D/V com mocks, pipe real com linha parcial e escritor aberto, EOF vazio/parcial, excesso de bytes, UTF-8 inválido, duas linhas sem perda, auxiliares curtos recolhidos após sucesso/EOF/timeout, JSON/liberação inválidos sem spawn, cgroups completo/hierárquico válidos e divergências de pertencimento, PID, PPID, ticks e marcador. Testes de recusa de unidade externa e de substituição de InvocationID continuam aprovados.

### Auditoria complementar e pendências preservadas

Não se demonstrou ausência integral de descendentes. O relatório sintético continua INCONCLUSIVO; sem marcador, a lista de descendentes continua desconhecida. Uma futura prova exige identidade da unidade preservada, estado systemd coerente, verificação de população de todo o cgroup/subgrupos (`cgroup.events`/`populated` e enumeração pertinente), confronto dos processos conhecidos por PID/ticks, tratamento de zumbis e evidência independente contra escape. Grupo vazio isolado não demonstra que um descendente não escapou; unidade desaparecida ou PID ausente isoladamente tampouco basta. Essa instrumentação adicional não foi implementada nem validada aqui.

A corrida entre reconsulta de InvocationID/cgroup e stop por nome permanece não atômica. Sem prova antes da morte do coordenador, o stop automático permanece recusado. No supervisor D/V, o acionamento da atestação ainda depende de DV_READY; prova já obtida não é apagada por falha posterior de canal. A morte do supervisor externo não tem recuperação integral demonstrada.

Outros caminhos examinados: `read_test_ready()` tem deadline e teto de saída, `communicate()` e waits Python pertinentes possuem timeout; entretanto escritas/flush/close de pipes continuam síncronos e não têm deadline próprio. A separação de exceções F1 não prova interrupção de uma operação de close que fique bloqueada. `trace.rs` mantém gates com read_line, espera do ChildGuard sem timeout próprio e leitores auxiliares; há fiscalização externa em outros caminhos, mas não se declara prazo universal nem ausência de bloqueios em todos os cenários. O instrumento histórico cgroup_controls mantém caminhos antigos de comunicação e stop por nome e não foi promovido à mesma garantia de propriedade desta finalização. Nenhuma dessas limitações foi declarada resolvida por F1–F3.

Antes de autorizar scope real: auditoria independente do commit, decisão sobre suficiência da prova de limpeza e risco de corrida, revisão dos prazos restantes, confirmação no terminal Debian 12 de versões, barramento, delegação, limites efetivos, política de interrupção, observabilidade e recuperação manual de unidade própria. O comando sintético histórico é apenas referência; esta correção não autoriza sua execução. Controles de memória aprovados anteriormente não substituem a validação real de limpeza.

**Estado: controles sintéticos aprovados no hospedeiro conforme evidência anterior; limpeza após falha abrupta com validação real pendente; D/V não executado; D04-E03 integral não homologada; congelamento não aprovado; painel F não autorizado.**

## Política de parada, prazos e população independente

Base `1483a166cef63ade4f42e5f7602ca093aaa36941`. **IMPLEMENTADO e VALIDADO POR TESTES SEGUROS; PENDENTE DE VALIDAÇÃO REAL.** Nenhum scope, ensaio no hospedeiro, D/V, F, EC-10 R3, checkpoint D/V ou pressão/OOM foi executado nesta atividade. A falha abrupta permanece exclusivamente a saída 77 do coordenador; morte do supervisor externo não faz parte do ensaio.

### Auditoria systemd v252 e política própria

Fontes upstream: [cliente de propriedades](https://github.com/systemd/systemd/blob/v252/src/shared/bus-unit-util.c), [D-Bus de scope](https://github.com/systemd/systemd/blob/v252/src/core/dbus-scope.c), [contexto de sinais](https://github.com/systemd/systemd/blob/v252/src/core/dbus-kill.c) e [propriedades cgroup](https://github.com/systemd/systemd/blob/v252/src/core/dbus-cgroup.c). O cliente UNIT_SCOPE aceita propriedades de cgroup, kill e scope; `TimeoutStopSec` é convertido para `TimeoutStopUSec`. O gerenciador admite essas propriedades no estado de criação da unidade transitória. Não se deduziu a política a partir de um scope alheio nem se alterou unidade histórica.

| Propriedade | Configuração proposta no lançamento | Verificação da própria unidade |
|---|---|---|
| MemoryMax | 67108864 | MemoryMax e leitura independente de memory.max |
| KillMode | control-group | Igualdade exata; process/mixed/none recusados |
| KillSignal | SIGTERM | KillSignal=15 |
| FinalKillSignal | SIGKILL | FinalKillSignal=9 |
| SendSIGKILL | yes | Igualdade exata |
| TimeoutStopSec | 2s | TimeoutStopUSec=2s na saída systemctl show |
| OOMPolicy | Não configurada | Não aparece no caminho de propriedades de scope upstream v252; ausência registrada como INDISPONIVEL_NAO_REQUERIDA neste ensaio sem pressão |
| Id, InvocationID, ControlGroup, estados e Result | Somente observação; não são prova por nome isolado | Confrontados com a prova independente |

Os defaults do gerenciador seriam usados para valores omitidos; a política de parada acima é explicitamente solicitada, não herdada de outra unidade. Não se alega que um barramento ou pacote específico aceitará a solicitação: recusa ou propriedade ausente/divergente impede liberação e registra inconclusão. A prova PID/PPID/ticks/cgroup/InvocationID foi preservada. Antes da liberação, a consulta da unidade, memory.max e identidade/cgroup do supervisor externo são conferidos. A coleta inicial deve ser completa e conter PID/ticks do coordenador. O supervisor deve estar fora do scope e de seus subgrupos.

### Orçamento temporal do ensaio sintético

| Fase | Orçamento configurado |
|---|---|
| Descoberta/atestação | Até 4 s desde início; cada consulta limitada ao restante ou 1 s |
| Política e liberação | Consulta 1 s, coleta inicial 0,25 s e escrita não bloqueante 0,25 s; coordenador espera liberação por até 7 s desde sua entrada |
| Marcador de trabalho | Até 9 s desde início; leitura local do marcador do filho tem 3 s |
| Confirmação da falha abrupta | wait de até 2 s, exigindo saída 77 |
| Coleta anterior ao stop | Até 2 s |
| Reconsulta da unidade | Até 1 s |
| Solicitação de stop | systemctl --user --no-block stop; cliente limitado a 1 s |
| Conclusão | Até 5 s após a solicitação; consultas de até 1 s ou restante e amostras de até 0,25 s |
| Verificação final | Até 2 s compartilhados entre coleta e identidades conhecidas |

O caminho nominal de supervisão tem orçamento aproximado de 22 s, com margem operacional proposta de 24 s para revisão humana; essa margem não é um watchdog externo implementado nem garantia de tempo real. Os 5 s de conclusão excedem o TimeoutStopSec solicitado de 2 s, deixando margem para a escalada e comunicação; o comportamento real ainda deve ser confirmado. Timeout do cliente não significa conclusão nem cancelamento do job: o gerenciador pode continuar a parada. Falha de solicitação, consulta, estado failed ou prazo sem conclusão comprovada permanece INCONCLUSIVO, mesmo que uma consulta posterior mostre inatividade.

A escrita da liberação sintética usa descritor não bloqueante, mensagem de até 4096 bytes, deadline e pipe sem buffering; não há threads auxiliares novas. Leituras de população são limitadas a 16 KiB por arquivo, 64 grupos e 512 IDs por amostra, e 4096 identidades acumuladas na verificação final; excesso é evidência incompleta. Chamadas de filesystem/proc, criação de processos, escrita dos relatórios e escalonamento do kernel não têm garantia de latência absoluta. Prazos de espaço de usuário não resolvem processo em estado ininterruptível nem morte do supervisor. Não se declara deadline universal do sistema.

### Coleta e classificação

O novo `scope_cleanup_evidence.py` coleta cgroup.procs, cgroup.threads, cgroup.events/populated e subdiretórios, independentemente do marcador de filho/neto: antes da liberação, durante a espera pelo marcador, imediatamente antes do stop, durante a conclusão e na verificação final. Registra timestamps monotônicos e civis, caminho, inode, processos/threads, PID/PPID/ticks/estado/cgroup e erros. Releitura de diretórios e população detecta mudanças observáveis; a varredura é não atômica. Erros, limites excedidos, identidades instáveis ou grupos que desaparecem geram INDISPONIVEL. A política e a unidade consultadas ficam junto das evidências de parada. Processos conhecidos são confrontados por PID/ticks após a tentativa, com classes AUSENTE, SOBREVIVENTE, ZUMBI, PID_REUTILIZADO e NAO_IDENTIFICAVEL. Nenhum sinal individual é enviado pelo coletor.

A classificação local VAZIO_DO_CGROUP_OBSERVADO exige todas as amostras completas, mesmo inode, estado final inactive da mesma InvocationID, ControlGroup vazio, solicitação aceita sem timeout, coleta final com populated=0 e listas de processos/threads vazias em todos os grupos, e nenhuma identidade conhecida sobrevivente, zumbi ou desconhecida. Essa classificação NÃO significa limpeza integral de toda linhagem: escape ou descendentes nunca observados não são excluídos. O resultado geral do ensaio continua INCONCLUSIVO e retorna 1 para auditoria humana. Sem marcador, a coleta de população continua, mas a falha abrupta prevista e a lista do harness não são presumidas.

Se o cgroup ou a unidade desaparecer antes da leitura final, o relatório preserva as observações anteriores e os resultados de PID/ticks; o desaparecimento não substitui a evidência final vazia. Assim, até uma remoção normal e rápida pelo systemd pode produzir INCONCLUSIVO. Trata-se de limitação conservadora conhecida, não de prova de falha do kernel nem de sobreviventes. Zumbi não é processo executando, mas impede declarar recolhimento completo. PID reutilizado não é o processo original e não autoriza sinal. Falha de leitura impede conclusão positiva.

### Segurança e recuperação futura

`stop_owned_scope()` preserva a validação tipada e do namespace próprio; para o scope sintético, delega ao protocolo de parada com evidências, revalidando Id, InvocationID, cgroup ativo e política imediatamente antes da solicitação. Prova ausente ou divergente recusa stop. Consulta e solicitação são chamadas separadas: a janela de substituição entre elas permanece não atômica. A nova política é restrita a neurons-v03-f1-cleanup; o caminho D/V prospectivo anterior não foi promovido à mesma validação operacional.

Em futura execução especificamente autorizada, preservar saída completa, commit/hashes, versões, prova de propriedade, propriedades da unidade, limites, todas as coletas e códigos/tempos de parada. Se inconclusivo, no terminal do hospedeiro consultar somente o nome completo registrado: `systemctl --user show NOME_EXATO.scope --property=Id --property=InvocationID --property=ControlGroup --property=ActiveState --property=KillMode --property=SendSIGKILL --property=TimeoutStopUSec`. Conferir a prova independente e a mesma InvocationID/cgroup antes de qualquer ação. Só então o operador poderá solicitar `systemctl --user stop NOME_EXATO.scope` sob a autorização específica da recuperação e coletar novamente estado, população e identidades. Se a prova estiver ausente ou divergente, não parar por nome isolado. Não usar curingas, slices, PIDs individuais ou reset-failed; não tocar em scopes históricos. Esse procedimento não foi executado aqui.

Comando de referência futuro, NÃO EXECUTADO e NÃO AUTORIZADO por esta alteração: `python3 -B tests/support/v03_f1_p01/synthetic_scope_cleanup.py --run-host-synthetic`.

### Verificações e parecer

Passaram **27 testes Python seguros do executor, 19 do protocolo de evidências e 10 dos controles**. Cobrem política válida/ausente/divergente, supervisor dentro do scope, recusa de unidade externa, stop concluído, timeout de solicitação/conclusão/consulta, falha de stop, unidade desaparecida/substituída antes/depois da solicitação, população vazia/residual, subgrupos e threads sem marcador, PID reutilizado, zumbi, falha de leitura, coleta incompleta, orçamento excedido, canal fechado/timeout de escrita e falha simultânea de canal/limpeza. Só mocks, arquivos temporários, pipes e processos locais seguros; nenhum scope real. Passaram dois testes Rust sintéticos, cargo fmt --check, cargo check --offline --all-targets e git diff --check. Regressão filtrada: **102 aprovados, zero falhos e 93 filtrados**; três avisos históricos EC-04 permanecem. Contagens dos testes específicos Rust não devem ser somadas como cobertura independente da regressão.

**Parecer: APTO PARA AUDITORIA DE AUTORIZAÇÃO DO ENSAIO**, para avaliar a política e o procedimento conservador com as limitações acima, não autorização de execução nem compromisso de conclusão positiva. Aceitação das limitações, infraestrutura e permissão real no Debian 12 continuam dependentes de auditoria/decisão posterior. **Estado: controles sintéticos anteriores aprovados no hospedeiro; limpeza após falha abrupta com validação real pendente; D/V não executado; D04-E03 não homologada; congelamento não aprovado; painel F não autorizado.**

## Robustez final da observação e da tentativa de parada

Base `ec50819d9fe1115fd6fabc0f7efaca5815171119`. **IMPLEMENTADO; VALIDADO POR TESTES SEGUROS; PENDENTE DE VALIDAÇÃO REAL.** Nenhum scope real, ensaio no hospedeiro, D/V, F, EC-10 R3, checkpoint, pressão/OOM ou alteração de unidade histórica nesta atividade.

A auditoria confirmou dois caminhos: collect_population anterior ao stop estava fora do try, e a coleta/inspeção no finally podia lançar exceção que ocultava a falha original. Agora observe_population é a fronteira de exceções de observabilidade: erros esperados e exceções inesperadas derivadas de Exception, estrutura de retorno inválida, coleta incompleta e deadline ultrapassado são convertidos em INDISPONIVEL, com causa e timestamp. O resultado completo exige estrutura de população de grupos e classificação de vazio coerentes. Deadline já expirado não inicia coleta. O supervisor sintético usa a mesma fronteira antes da liberação e durante a espera pelo marcador. Observação inicial inválida continua impedindo trabalho, mas uma prova já existente é preservada para a tentativa independente de limpeza.

stop_with_evidence separa observação de autorização: mesmo que a coleta anterior falhe, reconsulta a própria unidade e somente solicita stop quando propriedade, Id, InvocationID, cgroup ativo e política forem revalidados. Consulta/identidade/política ausente ou divergente continua recusando stop, inclusive se houver falha simultânea de observação. Nenhum sinal individual ou ação sobre unidades externas é introduzido. A separação não torna consulta e stop atômicos.

O relatório mantém errors em ordem de ocorrência, com etapa, tipo, mensagem e timestamp. reason preserva a primeira causa; falhas posteriores não a substituem. Evidências anteriores incompletas também são registradas. Os eixos são explícitos: observation_status, request (NAO_SOLICITADO/ACEITA/FALHOU/TIMEOUT), completion (NAO_COMPROVADA/CONCLUIDA/UNIDADE_FAILED), cleanup_observation e status geral. Uma solicitação pode falhar ou expirar e a mesma unidade ser observada inativa depois: ambos os fatos são registrados, sem atribuir causalidade nem promover limpeza. A coleta durante a conclusão não impede registrar o estado retornado pelo systemd. Coleta final, inspeção de identidades e classificação são protegidas separadamente; erro em uma não apaga solicitação, conclusão ou causas anteriores. Falhas inesperadas de fechamento do canal também são isoladas da chamada de limpeza. O supervisor preserva execution_errors e falhas da leitura final do log, separadamente da limpeza.

### Matriz de falhas e recuperação implementada

| Falha | Decisão de parada | Evidência/resultado |
|---|---|---|
| Coleta anterior com OSError, RuntimeError, KeyError ou retorno inválido | Revalidar unidade; solicitar se válida | Falha original preservada; limpeza INCONCLUSIVA |
| Coleta incompleta ou deadline expirado | Mesma regra; não libera novo trabalho sem coleta inicial suficiente | INDISPONIVEL com motivo |
| Propriedade/InvocationID/cgroup/política ausente ou divergente | Não solicitar | NAO_SOLICITADO; erro de revalidation independente do erro de observação |
| Solicitação aceita após falha de coleta | Acompanhar conclusão | ACEITA e possível CONCLUIDA coexistem com INCONCLUSIVO |
| Solicitação com erro/timeout | Registrar e consultar conclusão sem presumir cancelamento | FALHOU/TIMEOUT; eventual inatividade posterior não apaga falha |
| Coleta durante/final com exceção inesperada | Parada já solicitada não é desfeita | Causa adicional; estado de conclusão preservado |
| Inspeção final ou classificação com exceção | Não reenviar sinais nem inventar vazio | Identidade desconhecida ou classificação inconclusiva; todas as causas preservadas |
| Canal, coleta e solicitação falham simultaneamente | Fechamento isolado; revalidar e tentar stop se autorizado | Diagnósticos separados e encadeados |
| Unidade/cgroup desaparece | Não presumir vazio | INCONCLUSIVO sem a evidência exigida |
| Solicitação/conclusão excede prazo | Encerrar a espera limitada do cliente; preservar recuperação humana | INCONCLUSIVO; gerenciador pode continuar trabalhando |

Captura-se Exception somente nas fronteiras de diagnóstico/controle para impedir que uma falha de observação suprima a parada autorizada. KeyboardInterrupt, SystemExit, término forçado e falha do próprio processo supervisor não são transformados em continuidade normal. Nenhuma thread/processo de coleta ou mecanismo de morte do supervisor foi adicionado. Uma função de coleta que fique bloqueada sem levantar exceção não é interrompida por essa fronteira: continua limitação das operações de filesystem/kernel, sem garantia de tempo real.

### Revisão temporal de pior caso controlado

O orçamento de execução continua: marcador até 9 s desde início (inclui atestação de até 4 s, consulta de política até 1 s, coleta inicial 0,25 s, liberação 0,25 s e espera do marcador; não somar esses tempos novamente); wait da saída abrupta até 2 s; coleta anterior 2 s; reconsulta 1 s; solicitação 1 s; conclusão 5 s; verificação final 2 s. Soma serial dos orçamentos controlados: **9 + 2 + 2 + 1 + 1 + 5 + 2 = 22 s**. A janela de atestação pode terminar com sleep de 0,05 s e a de marcador com sleep de 0,02 s; a primeira normalmente está contida nos 9 s. Não se declara 22 s como limite absoluto medido. A espera de liberação de 7 s no coordenador e a leitura de 3 s do filho são internas ao fluxo, não novas fases seriais externas.

A margem anteriormente proposta de 24 s continua apenas referência para revisão, não watchdog implementado. Criação de processo, leituras de /proc/cgroup, writes de logs, encerramento interno dos subprocessos pelo Python, serialização e escalonamento não possuem deadline absoluto garantido pelo instrumento. Consequentemente, não há pior caso real finito comprovado para todo o sistema sob qualquer falha de kernel/I/O. Os limites de bytes, nós e identidades limitam trabalho conhecido, não preempção de syscall bloqueada. Testes de prazo confirmam somente os caminhos locais exercitados. Essa ressalva deve integrar eventual auditoria de autorização; não foi ampliado o ensaio para morte do supervisor.

### Validações e prontidão

**64 testes Python seguros passaram**: 27 do executor, 27 de política/evidências e 10 dos controles. Novos cenários cobrem erro antes do stop, exceção inesperada, retorno incompleto/malformado, coleta final falha, stop aceito após falha de coleta, recusa de propriedade/política, múltiplas falhas sem perda da primeira causa, inspeção/classificação final com erro, canal inesperado e deadlines expirados/ultrapassados. Os cenários anteriores de desaparecimento, PID reutilizado, zumbi, unidade externa/substituída e timeout continuam passando. Passaram dois testes Rust sintéticos, cargo fmt --check, cargo check --offline --all-targets e git diff --check. Regressão filtrada: **102 aprovados, zero falhos e 93 filtrados**, sem development/validation/frozen/final/panel/protocol; três avisos históricos EC-04 permanecem. Contagens específicas Rust se sobrepõem à regressão.

**Parecer: APTO PARA AUDITORIA DE AUTORIZAÇÃO DO ENSAIO**, restrito à robustez implementada e às limitações explícitas; não é autorização nem validação operacional real. A limpeza geral permanece INCONCLUSIVA no instrumento: desaparecimento não comprova vazio, varredura não é atômica e escape/descendentes nunca observados não são excluídos. LIMPEZA REAL PENDENTE; D/V NÃO EXECUTADO; D04-E03 NÃO HOMOLOGADA; CONGELAMENTO NÃO APROVADO; PAINEL F NÃO AUTORIZADO. Não avançar automaticamente.

## Observação concorrente após o primeiro ensaio real

Base `627abc1ef61b051c56304dcbe6ea5483715c70a4`. **Nesta atividade não houve nova tentativa real, scope, recuperação manual, D/V, F, EC-10 R3, pressão/OOM ou checkpoint.** Implementação e testes seguros não reclassificam o primeiro ensaio: **PARADA CORROBORADA; LIMPEZA INTEGRAL INCONCLUSIVA**.

### Evidência histórica efetivamente disponível

Foram lidos, sem alteração, `/tmp/neurons-cleanup-627abc1.stdout.log` e o stderr vazio. SHA-256 do stdout permaneceu `30889f8325d98e26cb5516e1d12970af58f4ccff6be1a6972585d54cbecfdae5`. Unidade `neurons-v03-f1-cleanup-05caedd90c3367a9.scope`, InvocationID `a43ce597b7fb45a3ae76b89db00b57cb`. O relatório anterior citou journal Started/Stopping/Stopped; não há stream D-Bus ou JSON do journal incorporado ao stdout, portanto não se inventou vinculação adicional de job/InvocationID.

Tomando o início da solicitação de stop como t=0: a última amostra de população começou em **−5,292196 ms**, encontrou filho e neto, populated=1, e terminou aproximadamente em −4,836828 ms. A reconsulta ativa da mesma InvocationID foi registrada em −0,020362 ms. O cliente de stop retornou zero em **+4,672105 ms**. Em **+8,594129 ms**, a consulta retornou not-found/inactive com InvocationID vazio; em **+8,615130 ms**, a coleta já encontrou o diretório ausente. Não se conhece o instante exato da parada ou remoção nem sua posição em relação ao retorno do cliente. Não houve leitura de populated=0. O coordenador saiu 77; PID/ticks conhecidos estavam ausentes no final. OOMPolicy foi de fato observada como stop no pacote instalado, sem solicitação explícita desse valor; isso não demonstra sua configurabilidade em scopes nem altera o recorte sem pressão.

### Monitor sem threads e recursos próprios

O novo `scope_population_observer.py` abre o diretório e cgroup.events antes do pedido de stop e conserva descritores do objeto observado. O inode é confrontado com a coleta anterior e a unidade revalidada antes de vincular as amostras à InvocationID. Falha de abertura/vinculação não autoriza parar unidade externa nem transforma uma amostra em prova da instância. A revalidação de propriedade/política existente permanece obrigatória. A corrida entre essa revalidação e o pedido continua não atômica.

Os clientes próprios de solicitação e de consulta pós-stop agora são executados assincronamente por Popen. Um único laço no supervisor lê a população enquanto esses clientes executam: não há thread de monitoramento ou processo observador adicional. O monitor é iniciado antes da consulta de revalidação; `started_ns`, `first_sample_ns`, `started_before_stop` e `first_sample_before_stop` distinguem início do monitor de primeira leitura bem-sucedida. A coleta recursiva anterior/posterior continua registrando subgrupos, processos e identidades. O monitor rápido acrescenta leituras do populated da raiz, cujo significado cgroup v2 abrange processos vivos na subárvore.

Se o caminho desaparecer mas o descritor ainda fornecer conteúdo, registra-se a leitura efetivamente obtida do objeto aberto e path_inode ausente. Se kernfs retornar erro/EOF, isso é erro, não populated=0. Substituição observável do inode no caminho é recusada. Uma leitura zero válida e vinculada prova apenas a ausência de processos vivos naquela subárvore naquele instante; não prova recolhimento de zumbis nem ausência de processos escapados.

`readings_1_then_0` registra os pares de valores efetivamente lidos, com timestamps. `transition_instant_observed=false` explicita que não se observou o instante exato da mudança nem continuidade entre amostras. Ausência depois de uma leitura 1 não cria leitura 0. A amostragem pode perder transições rápidas; nenhuma promessa de captura universal foi introduzida.

O laço solicita intervalo de até 1 ms entre verificações, sujeito a duração de I/O/escalonamento. Valores iguais são registrados no máximo a cada 25 ms; mudanças são registradas na leitura que as detecta. Limites: 512 registros, 32 erros e 16 KiB por leitura; excedentes são contados, não crescem indefinidamente. Os clientes têm saída limitada a 64 KiB por canal e usam arquivos temporários fechados ao término. O monitor fecha seus descritores em finally. Em erro/timeout, só o cliente Popen próprio é encerrado e recolhido, com espera adicional de até 1 s; nunca se sinalizam PIDs experimentais por esse mecanismo. Falha de encerramento do cliente fica anexada à causa original. Não há garantia contra syscall ou processo em estado ininterruptível. Não foram adicionados workers órfãos ou daemon threads.

O monitor dura até a soma query/request/completion/final (9 s desde sua criação) e é fechado na finalização, inclusive nas recusas de stop. O orçamento anterior de 22 s continua sendo soma de fases controladas; encerramento excepcional de cliente pode acrescentar até 1 s por cliente afetado. Não há um limite universal real de 22/24 s sob falhas de kernel, I/O ou escalonamento. Esgotamento de amostras, erros ou ausência de leitura não são corrigidos por repetição automática do ensaio.

### Evidência systemd: avaliação e limite implementado

A [API oficial v252](https://www.freedesktop.org/software/systemd/man/252/org.freedesktop.systemd1.html) permite Subscribe, StopUnit com objeto de job retornado e JobRemoved com resultado do job; a assinatura precisa preceder o pedido para evitar perder o evento. Vincular o job à unidade previamente atestada exige preservar essa associação e a InvocationID; JobRemoved/UnitRemoved não carregam sozinhos a InvocationID. RefUnit pode conservar referência da unidade em uma conexão persistente, mas não garante retenção do diretório cgroup. Nenhum desses métodos foi chamado nesta atividade.

**Captura nativa de jobs/sinais D-Bus não foi implementada**: o protocolo continua usando consultas de estado, agora sem suspender a observação de população. PARADA_CONFIRMADA exige estado inactive da mesma Id/InvocationID; unidade removida antes dessa confirmação continua sem A. `manager_evidence.job_tracking=NAO_IMPLEMENTADO` torna essa limitação explícita. O analisador conservador `journal_evidence` não confirma parada por mensagem: nome sozinho não vincula instância; mesmo campos USER_UNIT/INVOCATION_ID coincidentes são apenas candidatos a correlação, exigindo origem e job/estado verificáveis. Não há coleta automática nova do journal nem promoção por texto Started/Stopped. O journal histórico não foi reinterpretado como captura de job.

Referência para o significado hierárquico de populated: [documentação cgroup v2](https://docs.kernel.org/admin-guide/cgroup-v2.html#un-populated-notification).

### Classificações independentes

| Campo em classifications | Critério e alcance |
|---|---|
| PARADA_CONFIRMADA | Estado inactive observado com Id e InvocationID esperadas; aceite do stop ou journal por nome não basta |
| POPULACAO_VAZIA_OBSERVADA | Leitura real populated=0 no objeto/inode vinculado à instância revalidada; restrita à subárvore e timestamps das amostras |
| AUSENCIA_DOS_PROCESSOS_CONHECIDOS | Lista não vazia de identidades rastreadas, todas classificadas AUSENTE; zumbis, PID reutilizado e falhas de leitura permanecem categorias distintas |
| LIMPEZA_INTEGRAL_INCONCLUSIVA | Sempre verdadeiro no recorte: não se excluem processos escapados ou nunca observados |
| FALHA_DE_PARADA | Erro explícito na solicitação ou estado failed; falta de confirmação/timeout pode ser inconclusão sem demonstrar falha do gerenciador |
| UNIDADE_REMOVIDA_OBSERVADA | Consulta not-found/inactive após o pedido; não é A nem B |

A, B e C não se implicam. A classificação local legada de vazio por coletas completas permanece separada da nova evidência rápida de populated=0. O resultado geral continua INCONCLUSIVO e exige auditoria humana; mesmo uma leitura B não homologa limpeza universal ou D04-E03. Erros de coleta, solicitação e finalização continuam registrados separadamente e não apagam a causa original.

### Higiene e validações

Foram confirmados tipo, localização, ausência no índice Git e assinatura bytecode dos três arquivos não rastreados: `__pycache__/dv_cgroup_executor.cpython-311.pyc`, `__pycache__/cgroup_controls.cpython-311.pyc` e `__pycache__/scope_cleanup_evidence.cpython-311.pyc`. Somente esses três arquivos exatos foram removidos; não se usou git clean nem remoção recursiva. O diretório vazio não foi removido indiscriminadamente. Testes Python usam PYTHONDONTWRITEBYTECODE=1; o lançamento interno sintético passa a propagar essa variável aos descendentes, evitando novos caches.

Passaram **74 testes Python seguros** (27 executor, 29 protocolo de evidências, 10 controles e 8 observador), dois testes Rust sintéticos, cargo fmt --check, cargo check --offline --all-targets e git diff --check. Regressão Rust filtrada: **102 aprovados, zero falhos, 93 filtrados**; três avisos históricos EC-04 permanecem. Cenários novos: remoção rápida, leituras 1/0, zero não observado, grupo ausente/substituído, vínculo de inode/InvocationID divergente, journal sem identidade suficiente, stop aceito sem conclusão, ausência de conhecidos, sobreviventes, erros simultâneos, deadline, limite de registros, fechamento de descritores e clientes locais recolhidos após timeout. Não se executaram systemctl ou scopes reais nos testes novos.

**Estado preservado:** primeiro ensaio com parada corroborada e limpeza integral inconclusiva; nova observação IMPLEMENTADA e VALIDADA POR TESTES SEGUROS, PENDENTE DE VALIDAÇÃO REAL. D/V NÃO EXECUTADO; D04-E03 NÃO HOMOLOGADA; CONGELAMENTO NÃO APROVADO; PAINEL F NÃO AUTORIZADO. Nenhuma nova tentativa real está autorizada por esta alteração.

## B1 — Classificação temporal da população

Base `dbae5a27d9688a2ce15554912162de51b8a91338`. **IMPLEMENTADO e VALIDADO POR TESTES SEGUROS; PENDENTE DE VALIDAÇÃO REAL.** Não houve novo scope, ensaio real, D/V, F, EC-10 R3, pressão/OOM ou checkpoint D/V. Esta correção substitui o critério B da seção anterior, sem reinterpretar o primeiro ensaio.

A auditoria de classifications() confirmou que qualquer leitura zero vinculada podia produzir POPULACAO_VAZIA_OBSERVADA, mesmo antes do pedido ou sem timestamps da solicitação. O classificador agora exige, para B positivo: pedido com status ACEITA; request_started/request_finished com monotonic_ns inteiros positivos e fim >= início; vínculo explícito da instância, InvocationID hexadecimal correspondente e inode inteiro positivo correspondente; timestamps das amostras válidos e estritamente crescentes na ordem registrada; sequência de leituras válidas adjacentes 1/0, com o 1 estritamente antes e o 0 estritamente depois do início da solicitação. Não se ordenam timestamps retrospectivamente para reconstruir uma sequência. Timestamp ausente, inválido, repetido ou fora de ordem impede B positivo. Divergências de inode, path_inode ou InvocationID e populated ausente/inválido excluem a amostra e ficam registradas em rejected_samples.

O referencial de “após” é explicitamente o **início da solicitação**, não a conclusão da parada. Uma leitura zero durante a execução do cliente pode qualificar a sequência quando o pedido for posteriormente aceito; continua sem provar causalidade ou parada concluída. population_timing separa zero_before_stop_ns, one_before_stop_ns, zero_after_request_start_ns, zero_during_request_ns e zero_after_request_finished_ns. O intervalo durante o cliente é início < t <= fim; depois do retorno exige t > fim. Leitura exatamente no início não é anterior nem posterior. Zero posterior sem leitura 1 anterior permanece registrado como ponto observado, mas não produz B. Zero anterior nunca é promovido a esvaziamento pós-solicitação. Stop não solicitado, recusado ou com timeout não produz B, mesmo se houver leituras locais.

readings_1_then_0 e readings_1_before_0_after preservam os pares e seus timestamps, sem alegar instante exato de transição: exact_transition_instant_observed permanece false. Leituras rejeitadas, erros do observador e contagem de amostras descartadas são limitações explícitas. Erros/descartes não apagam valores válidos efetivamente lidos: uma sequência ancorada pode continuar registrada com essas limitações, mas não prova continuidade entre amostras, limpeza integral ou ausência de escape. Uma unidade removida sem qualquer zero válido é classificada separadamente em removed_without_zero; desaparecimento não cria zero.

| Evidência | B: POPULACAO_VAZIA_OBSERVADA |
|---|---|
| Zero somente antes do pedido | Falso; zero anterior preservado |
| 1 antes e 0 após início do pedido aceito | Verdadeiro, limitado aos pontos/subárvore lidos |
| Zero antes e 1 depois | Falso |
| Zero depois, sem 1 anterior | Falso; zero posterior isolado preservado |
| Remoção sem leitura zero | Falso; remoção separada |
| Inode/InvocationID divergente | Amostra rejeitada; sem B baseado nela |
| Tempos ausentes, repetidos ou fora de ordem | Falso; limitação temporal registrada |
| Stop não solicitado, recusado ou com timeout | Falso; leituras não viram evidência de esvaziamento pelo pedido |

PARADA_CONFIRMADA continua dependendo do estado da mesma unidade/InvocationID; AUSENCIA_DOS_PROCESSOS_CONHECIDOS continua independente da população. LIMPEZA_INTEGRAL_INCONCLUSIVA permanece true. B não equivale a homologação e sua semântica foi deliberadamente restringida ao esvaziamento temporalmente ancorado. Não há afirmação de que o stop causou a mudança ou de ausência universal de descendentes.

Passaram **83 testes Python seguros** (27 executor, 29 evidências, 10 controles, 17 observador), dois Rust sintéticos, cargo fmt --check, cargo check --offline --all-targets e git diff --check. Regressão Rust filtrada: **102 aprovados, zero falhos e 93 filtrados**; três avisos históricos EC-04 permanecem. Foram cobertos os cenários da tabela, zero durante e após retorno do cliente, erros de leitura, amostras descartadas, estrutura incompleta e distinção entre A/B/C. Somente testes locais seguros; bytecode desabilitado. Critérios de propriedade e segurança da parada não foram modificados.

**Estado preservado:** primeiro ensaio com PARADA CORROBORADA e LIMPEZA INTEGRAL INCONCLUSIVA. D/V NÃO EXECUTADO; D04-E03 NÃO HOMOLOGADA; CONGELAMENTO NÃO APROVADO; PAINEL F NÃO AUTORIZADO. Nenhuma etapa posterior ou nova tentativa real autorizada por esta correção.

## B2 — Continuidade e integridade das sequências

Base `48377b0f00020f48fb57cc20e68e74f05ee57107`. Verificação inicial: origin softstrucs/neurons, branch devs, HEAD igual à base e árvore limpa. **IMPLEMENTADO e VALIDADO POR TESTES SEGUROS; VALIDAÇÃO REAL PENDENTE.** Não houve scope, repetição do ensaio ou recuperação manual. Nenhuma política systemd, limite de 64 MiB, sinal, prazo, propriedade, revalidação ou solicitação de parada foi alterada.

### Defeito e correção delimitada

B1 formava pares com zip(valid, valid[1:]) após remover amostras rejeitadas. Isso podia aproximar duas leituras originalmente separadas por amostra inválida, identidade divergente ou informação temporal insuficiente. PopulationObserver.finish() também formava pares sem consultar erros e descartes. Descartes agregados eram apenas limitações textuais e não interrompiam qualificação. A asserção antiga que aceitava B positivo com descarte de posição desconhecida foi corrigida.

O novo population_sequences() é compartilhado por finish() e classifications(). Toda leitura validada conserva o índice original. Só se qualifica par 1/0 quando seus índices originais são consecutivos, os timestamps são válidos e estritamente crescentes e ambos pertencem ao mesmo inode/InvocationID vinculado. Amostras rejeitadas nunca são removidas para reconstruir adjacência positiva: pares separados são registrados em interrupted_pairs com suas causas. A ordem original não é reordenada. B continua exigindo pedido aceito, timestamps válidos da solicitação, 1 anterior ao início e 0 posterior ao início; zero isolado não basta.

Uma falha temporal ou amostra inválida interrompe o segmento correspondente. Um segmento posterior íntegro pode continuar qualificável, mesmo que outro segmento tenha sido interrompido; ORDEM_TEMPORAL_INVALIDA permanece explícita para o conjunto. Assim, o diagnóstico global não inventa continuidade através da falha nem descarta automaticamente uma sequência independente válida. São mantidas separadamente as leituras zero, as sequências qualificáveis e a confirmação da parada.

### Tipos de observação e lacuna

| Tipo | Registro e efeito |
|---|---|
| Amostra armazenada válida | Índice original, read_index, gap_epoch e deduplicated_since_previous |
| Amostra rejeitada/inválida | Índice e causa; quebra de adjacência original |
| Leitura que falhou | error_count e evento read_error com timestamp/read_index; incremento de gap_epoch |
| Amostra válida não armazenada por MAX_SAMPLES | dropped_samples e evento sample_limit com posição; incremento de gap_epoch |
| Leitura válida repetida deduplicada | Contador próprio e número desde a amostra anterior; não incrementa gap_epoch nem dropped_samples |
| Intervalo sem leitura registrada | Intervalo entre timestamps preservado; nenhuma observação física ininterrupta é alegada |
| Posição de erro/descarte desconhecida | Continuidade NÃO COMPROVADA; nenhum par é qualificado com base nesse log incompleto |

Os eventos de lacuna têm limite de 64 registros; gap_events_lost conta excedentes. Os limites anteriores de amostras/erros permanecem; overflow ou divergência de contadores não vira ausência de falha. Logs antigos com erros timestampados podem localizar o erro; descarte somente agregado, erro sem timestamp ou contabilidade inconsistente mantém posição desconhecida. No coletor atual, read_index e gap_epoch também detectam leituras intermediárias não explicadas. Diferença de read_index entre amostras deve corresponder exatamente às deduplicações válidas mais a leitura corrente. Uma deduplicação contabilizada não é falha automática.

Eventos de erro/descarte que intersectam o intervalo fechado entre as duas leituras bloqueiam o par. Erro comprovadamente anterior ou posterior ao intervalo fica como limitação, sem invalidar outro segmento íntegro. Se a posição não puder ser determinada, a regra é conservadora: não qualificar sequência. Ao atingir MAX_SAMPLES, uma leitura zero não armazenada não é inventada; um descarte posterior a um par já observado não o torna retroativamente inexistente quando a posição é conhecida. Qualquer perda de posição permanece conservadora.

Diagnósticos estruturados: SEQUENCIA_CONTINUA_OBSERVADA, SEQUENCIA_INTERROMPIDA, AMOSTRA_INTERMEDIARIA_REJEITADA, DESCARTE_COM_POSICAO_DESCONHECIDA e ORDEM_TEMPORAL_INVALIDA. SEQUENCIA_INTERROMPIDA sinaliza que há interrupção no conjunto; interrupted_pairs localiza os pares candidatos atingidos. Pode coexistir com sequência íntegra independente. Os pares incluem índices originais, intervalo em ns e deduplicações conhecidas.

**Adjacência significa ausência de lacuna registrada entre as leituras, não continuidade física da população.** Leituras e pausas entre chamadas não dão cobertura ininterrupta; a amostragem pode perder mudanças. physical_continuity_claimed e exact_transition_instant_observed permanecem false. B não prova instante exato, causalidade, ausência universal de escape ou limpeza integral. PARADA_CONFIRMADA, ausência de conhecidos, falha, unidade removida e limpeza integral inconclusiva continuam separadas. Nenhum critério científico D04-E03 foi alterado.

### Matriz de testes B2

| Teste | Caso | Resultado esperado e obtido |
|---|---|---|
| T01 | 1/0 válido, sem lacuna | Sequência qualificável |
| T02 | Amostra inválida intermediária | Interrompida |
| T03 | Timestamp inválido intermediário | Interrompida; ordem inválida |
| T04 | Timestamp fora de ordem | Interrompida |
| T05 | InvocationID intermediário divergente | Interrompida |
| T06 | Inode intermediário divergente | Interrompida |
| T07 | populated intermediário inválido | Interrompida |
| T08 | Descarte posicionado entre 1/0 | Interrompida |
| T09 | Descarte/contabilidade sem posição suficiente | Continuidade não comprovada |
| T10 | Deduplicação válida | Sequência mantida; contador próprio |
| T11 | Zero isolado após pedido | Sem B positivo |
| T12 | Zero só antes do pedido | Sem B positivo |
| T13 | Pedido recusado | Sem B positivo |
| T14 | Pedido aceito com sequência íntegra | B positivo local; A/C não promovidos |
| T15 | Remoção sem zero | Sem B positivo |
| T16 | Erro de leitura entre amostras | Interrompida, também no finish() real com arquivo temporário |
| T17 | Erro fora do intervalo | Segmento válido preservado; limitação registrada |
| T18 | MAX_SAMPLES atingido | Zero descartado não inventado; descarte posterior posicionado não apaga par anterior |
| T19 | Múltiplos segmentos válidos/interrompidos | Só o segmento íntegro pode qualificar |
| T20 | Timestamps do pedido ausentes | Sem B positivo |

Passaram **20 testes B2 pertinentes** e a suíte completa de **103 testes Python seguros** (inclui os 20), dois testes Rust sintéticos, cargo fmt --check, cargo check --offline --all-targets e git diff --check. Regressão Rust filtrada: **102 aprovados, zero falhos e 93 filtrados**. PYTHONDONTWRITEBYTECODE=1 foi usado nos testes; variáveis opcionais D/V/checkpoint/trace foram removidas do ambiente dos comandos Rust. Permanecem três avisos históricos EC-04. Não se somam os dois testes Rust específicos como cobertura independente da regressão.

**Parecer: APTO PARA AUDITORIA DE AUTORIZAÇÃO DO SEGUNDO ENSAIO.** Isso não autoriza executá-lo. Primeiro ensaio preservado: PARADA CORROBORADA, LIMPEZA INTEGRAL INCONCLUSIVA. Segundo ensaio NÃO AUTORIZADO. D/V NÃO EXECUTADO; D04-E03 NÃO HOMOLOGADA; CONGELAMENTO NÃO APROVADO; PAINEL F NÃO AUTORIZADO. Não avançar automaticamente.

## C1 — Evidência independente de resultado de job systemd

Referência `46ad822f78aff948d3b1b8d8a7842302187d3e9c`. Confirmados origin softstrucs/neurons, branch devs, HEAD esperado e árvore limpa antes das alterações; B1/B2 e a decisão D04-E03 foram examinados. O pacote observado é systemd `252.39-1~deb12u2`, Python 3.11.2. **C1 implementa somente modelo puro com dados simulados; não integra transporte D-Bus nem altera o fluxo de parada.** Nenhum scope, StopUnit real, ensaio, recuperação, pressão/OOM, D/V, F ou EC-10 R3 foi executado. A única interação com o gerenciador foi introspecção de leitura da interface Manager; nenhuma unidade histórica foi consultada ou modificada.

### Interfaces investigadas e evidência de compatibilidade

Fontes compatíveis: [API v252](https://www.freedesktop.org/software/systemd/man/252/org.freedesktop.systemd1.html), [dbus-manager.c](https://github.com/systemd/systemd/blob/v252/src/core/dbus-manager.c), [dbus-job.c](https://github.com/systemd/systemd/blob/v252/src/core/dbus-job.c), [dbus-unit.c](https://github.com/systemd/systemd/blob/v252/src/core/dbus-unit.c), [resultados em job.c](https://github.com/systemd/systemd/blob/v252/src/core/job.c) e [geração de InvocationID em unit.c](https://github.com/systemd/systemd/blob/v252/src/core/unit.c). A introspecção efetiva no gerenciador de usuário instalado confirmou:

| Interface Manager | Entrada / saída observadas | Utilidade e limite |
|---|---|---|
| StopUnit | ss / o | Resposta fornece objeto de job, não conclusão nem InvocationID |
| JobRemoved | sinal uoss | ID, caminho do job, unidade e resultado; sem InvocationID |
| Subscribe / Unsubscribe | sem argumentos | Controle da emissão dos sinais; precisa conexão persistente |
| GetJob | u / o | Resolve job ainda disponível; pode desaparecer após remoção |
| GetUnit | s / o | Resolve unidade carregada; caminho não é geração da instância |
| GetUnitByPID | u / o | Associação enquanto PID existe; não resolve retroativamente pai encerrado |
| GetUnitByInvocationID | ay / o | Resolve ciclo ainda disponível; não torna StopUnit por nome condicional ao ID |
| RefUnit / UnrefUnit | s / sem saída | Possível retenção de referência por conexão, não garantia de retenção do cgroup |

Também foram observados GetUnitByControlGroup e GetUnitProcesses. Não foram chamados StopUnit, Subscribe, RefUnit ou qualquer método operacional. As propriedades Unit/Job foram investigadas na documentação e código v252, sem criar um Job de inspeção. Job expõe Id, Unit, JobType e State; estado running/waiting não substitui o resultado final recebido. O resultado é transitório: GetJob posterior pode não recuperá-lo. UnitRemoved representa descarregamento do objeto; nome/caminho podem ser reutilizados. InvocationID/propriedades podem ficar indisponíveis após descarregamento.

A [descrição v252 de InvocationID](https://www.freedesktop.org/software/systemd/man/252/systemd.exec.html#%24INVOCATION_ID) identifica um ciclo de execução por ID aleatório de 128 bits, renovado na ativação. Para esta camada, sua fonte deve ser propriedade do gerenciador, não variável/declaração de um filho. Não é credencial assinada nem parâmetro de comparação atômica de StopUnit. O código de JobRemoved em dbus-job.c confirma o corpo uoss e a ausência de InvocationID. O caminho do objeto Unit deriva da unidade, não prova uma geração particular.

### Modelo implementado e correlação formal

O novo `tests/support/v03_f1_p01/systemd_job_evidence.py` importa somente dataclasses e re: não abre arquivos, conecta ao barramento, cria processos ou solicita parada. DTOs imutáveis representam StopRequest, Subscription, StopReply, JobRemoved e UnitIdentity. `correlate_stop_job()` avalia um snapshot limitado de eventos sintéticos. Não é mecanismo de autorização de stop. O fluxo operacional atual e o classificador B2 não importam esse módulo.

Os campos incluem stop_request_id local, unidade/caminho, InvocationID e inode anteriores, serial da chamada, identificador da conexão e nome único do remetente do gerenciador, status/instante da subscrição, objeto/ID/resultado do job, tempos monotônicos de pedido/resposta/recepção e snapshots de identidade. O resultado preserva eventos correspondentes, eventos não correlacionados, duplicatas, conflitos, timeout, diagnóstico de identidade e limitações. O ID local da solicitação não é campo do sinal D-Bus nem autenticação criptográfica.

Para certificar resultado de job simulado, são exigidos:

1. Pedido registrado como emitido, tempos válidos e serial de chamada positivo; resposta aceita na mesma conexão/remetente, reply_serial igual ao serial da chamada, caminho canônico de job com ID uint32 correspondente, dentro da janela observada.
2. Subscrição confirmada antes do pedido na mesma conexão/gerenciador. Em integração futura isso deverá representar confirmação de AddMatch e Subscribe, não mera intenção de subscrever.
3. Evento Manager.JobRemoved no caminho /org/freedesktop/systemd1 com mesmo ID/caminho retornado e unidade solicitada, origem/conexão correspondentes e timestamp monotônico de recepção dentro da janela. O transporte futuro deverá verificar remetente D-Bus, proprietário do nome, cabeçalhos, serial de resposta e epoch da conexão. Estes metadados são fixtures nesta etapa, não autenticação real executada pelo modelo.
4. Resultado único, sem conflito ou overflow do buffer; ausência de perda de subscrição até o maior instante entre resposta e evento. Limite de 128 eventos por snapshot; resultados textuais limitados a 64 caracteres. Excesso não certifica conclusão.

O futuro adaptador deve registrar match/subscrição antes do pedido e bufferizar sinais até obter a resposta. O sinal pode ser recebido antes dela: não se exige signal_time >= response_time. Só a resposta correlacionada fixa o job escolhido; evento por nome isolado ou sem resposta não é suficiente. Os tempos representam recepção local, não o instante de conclusão interna do systemd.

Duplicatas idênticas são contadas sem duplicar a conclusão. Resultados conflitantes do mesmo job ou ordem temporal inconsistente desse job impedem certificação; não são reordenados para escolher o resultado desejado. Jobs de outras unidades ficam separados. Eventos antigos, fora da janela, de outro remetente/conexão ou com ID/caminho divergente não são correlacionados. Desconexão/reinício do gerenciador precisa invalidar o vínculo; um nome único/ID de job não é garantia universal entre barramentos/gerações. Perda posterior à janela já certificada não apaga resultado recebido, mas permanece limitação para evidência posterior.

`done` produz JOB_CONCLUIDO_OBSERVADO apenas para o job correlacionado. `failed`, timeout e demais falhas explícitas conhecidas são separados de canceled. skipped/collected/once não são tratados como sucesso de parada; resultado novo é preservado e classificado como desconhecido. Canceled não prova que toda operação subjacente tenha sido cancelada. Sem resposta/evento ou com subscrição indisponível, permanece resultado desconhecido; timeout da observação não autoriza reenviar ou cancelar um job.

### Identidade: o que não é comprovado

Snapshot anterior exige nome, caminho Unit, InvocationID, inode, origem/conexão, estado ativo e tempo anterior ao pedido coerentes com a referência. Snapshot posterior correspondente pode corroborar a persistência da identidade; remoção com InvocationID ausente enfraquece esse vínculo sem apagar um resultado de job já correlacionado. A correspondência de snapshots é observacional, não transforma JobRemoved em mensagem autenticada pelo InvocationID.

O modelo distingue PRE_STOP_VERIFICADA_SEM_VINCULO_ATOMICO_AO_JOB, PRE_E_POS_CORROBORADAS e INCONCLUSIVA. **instance_proven_by_job permanece false, IDENTIDADE_INSTANCIA_INCONCLUSIVA permanece true e esta camada nunca promove PARADA_CONFIRMADA.** Uma correlação de job válida pode existir mesmo sem identidade prévia suficiente: comprova o resultado desse job, não sua atribuição incontestável ao ciclo anteriormente observado. O transporte e as decisões de autorização não podem usar esse fato para contornar a atestação operacional existente.

StopUnit por nome não recebe InvocationID como precondição; há corrida entre a leitura e a chamada. GetUnit/caminho do objeto e sinal JobRemoved não eliminam essa janela. Jobs podem ser mesclados/coalescidos: a resposta associa a chamada ao job retornado, não prova autoria exclusiva da parada. Um evento antigo de instância anterior não pode ser identificado por um InvocationID inexistente no sinal: janela/conexão/job rejeitam casos observáveis, mas a atribuição de instância continua explicitamente limitada. Não se inventou esse campo em fixtures de JobRemoved.

O journal, estado posterior e ausência de PIDs são corroborativos, não substitutos da correlação. Nome isolado, resposta ausente e mensagem não correlacionada são insuficientes. Unidade inativa/removida é registrada separadamente. Resultado de job done não comprova populated=0, recolhimento de todos os descendentes, ausência de escape ou limpeza integral. C1 não avalia POPULACAO_VAZIA_OBSERVADA nem AUSENCIA_DOS_PROCESSOS_CONHECIDOS e não altera B2. Os campos C1 devem permanecer em namespace próprio numa integração futura, sem sobrescrever classificações operacionais já existentes.

### Testes sintéticos C1

| Teste | Caso e resultado verificado |
|---|---|
| T01 | done correlacionado: job concluído observado; PARADA_CONFIRMADA permanece false |
| T02 | failed: JOB_FALHOU |
| T03 | canceled: JOB_CANCELADO, sem sucesso |
| T04 | Resultado novo: preservado/desconhecido |
| T05 | ID de job ou serial de resposta divergente: não correlacionado |
| T06 | Caminho do job divergente: não correlacionado |
| T07 | Nome de unidade divergente: não correlacionado |
| T08 | InvocationID indisponível: instância inconclusiva, sem promoção |
| T09 | InvocationID divergente: snapshot prévio recusado |
| T10 | Duplicata idêntica contada; conflito/overflow não certificam resultado |
| T11 | Evento ausente: job identificado sem resultado comprovado |
| T12 | Evento antes da resposta: correlacionado após conhecer caminho retornado |
| T13 | Evento após resposta: correlacionado |
| T14 | Tempos inválidos/fora de ordem: sem conclusão certificada |
| T15 | Timeout da observação: resultado desconhecido |
| T16 | Barramento indisponível simulado: sem aceite/job identificado |
| T17 | Unidade removida antes da consulta posterior: job preservado, identidade não promovida |
| T18 | Journal por nome sem vínculo: somente corroborativo |
| T19 | Job done sem zero: B não avaliado/promovido |
| T20 | B positivo com eventos sintéticos sem conclusão do job: evidências independentes |
| T21 | PIDs conhecidos ausentes sem job: não conclui job |
| T22 | Jobs concorrentes, envelope/conexão divergentes: somente alvo válido correlacionado |
| T23 | Evento antigo/outro remetente e instância posterior divergente: sem confirmação da instância |
| T24 | Subscrição ausente/tardia/perdida: não certifica janela interrompida |

Passaram **24 testes C1** e a suíte V03-F1-P01 com **127 testes Python seguros**, incluindo C1, além dos dois testes Rust sintéticos. cargo fmt --check, cargo check --offline --all-targets e git diff --check passaram. Regressão Rust filtrada: **102 aprovados, zero falhos e 93 filtrados**; três avisos históricos EC-04 permanecem. PYTHONDONTWRITEBYTECODE=1 foi usado; variáveis opcionais de caso/checkpoint/trace removidas dos comandos Rust. A introspecção de leitura não fez parte de teste mutável. Nenhuma fixture é resultado operacional real.

### Integração futura pendente e parecer

Não se substituiu systemctl --no-block nem se integrou nova chamada StopUnit. Uma etapa posterior especificamente autorizada deverá escolher biblioteca/transporte disponível sem instalações presumidas; definir ciclo de vida de conexão e subscrição, autenticação do proprietário/cabeçalhos, buffer limitado antes da resposta, timers, reconexão, duplicatas e associação do job. Deverá preservar a atestação de propriedade/política, a observação B2 e o fechamento/recolhimento de recursos. Precisará ainda decidir como manter referência de Unit e tratar a corrida de instância sem prometer atomicidade inexistente. Primeiro validar adaptador com mocks; nenhuma chamada real ou terceiro ensaio fica autorizada aqui.

**Parecer: C1 APROVADA PARA AUDITORIA**, no escopo documental e de modelo simulado. Integração operacional e confirmação de identidade de instância por job permanecem pendentes/limitadas. Primeiro ensaio: parada corroborada, limpeza integral inconclusiva. Segundo ensaio: parada corroborada, população vazia não observada, limpeza integral inconclusiva. TERCEIRO ENSAIO NÃO AUTORIZADO. D/V NÃO EXECUTADO; D04-E03 NÃO HOMOLOGADA; CONGELAMENTO NÃO APROVADO; PAINEL F NÃO AUTORIZADO.
