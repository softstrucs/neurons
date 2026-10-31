# V03-F1-P01 — Diagnóstico da infraestrutura cgroup v2

Base `c2d0df217b3eaa2abd32f300f309bc6cd6a10967`, branch `devs`, árvore inicialmente limpa e sincronizada com `origin/devs`. Autorização para preparação e validação delimitadas, sem F, congelamento, alterações do núcleo ou administração não confirmada. Fontes: [plano v0.3](planos/Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_APROVADO.md), [D01–D04](V03-F1-P01_decisoes_metodologicas_pre_finais_APROVADAS.md), [D04-E02](protocolos/V03-F1-P01_D04-E02_dados_vivos_APROVADA.md), [D04-E03](protocolos/V03-F1-P01_D04-E03_memoria_cgroup_APROVADA.md), [fechamento operacional](V03-F1-P01_fechamento_operacional_pre_congelamento.md), [investigação anterior](V03-F1-P01_investigacao_contencao_memoria.md), [manifesto prospectivo](protocolos/V03-F1-P01_especificacao_manifesto_pre_congelamento.md), [decisões](decisions.md), `watchdog.py` e instrumentos D/V existentes.

## Diagnóstico e fronteira do ambiente

Comandos locais de leitura: `uname -a`, `cat /etc/os-release`, `id`, `cat /proc/self/cgroup`, leitura de `cgroup.controllers` e `cgroup.subtree_control`, `findmnt -T /sys/fs/cgroup`, `ls -ld /sys/fs/cgroup`, `df -B1 /home`, inspeção de `/proc/self/mountinfo` e `/proc/self/status`, leitura dos `memory.max` ancestrais visíveis, `systemctl --user show --property=ControlGroup --property=Delegate`, `systemd --version` e busca dos executáveis de contêiner. Nenhuma configuração ou grupo foi criado.

| Dimensão | Observação nesta sessão |
|---|---|
| Sistema | Debian 12, kernel `6.1.0-52-amd64`, systemd 252; UID 1000. |
| Hierarquia | cgroup v2, processo em `/user.slice/user-1000.slice/user@1000.service/app.slice/app-com.microsoft.VSCode-21749.scope`; controlador `memory` listado na raiz. |
| Escrita e limites | Montagem visível `/sys/fs/cgroup` é `ro`; caminhos ancestrais visíveis não graváveis e seus `memory.max` lidos como `max`. Isso não comprova ausência de limites externos em outra hierarquia ou vista. |
| Origem da restrição | `mountinfo` mostra montagem visível `ro` com opções de superbloco da origem `rw`; perfil desta sessão restringe escrita fora da área de trabalho e `/tmp`, com seccomp e `NoNewPrivs`. A restrição observada é da vista desta execução/sandbox; não se conclui que o hospedeiro inteiro seja somente leitura. A presença ou ausência de contêiner hospedeiro não foi estabelecida. |
| Systemd e contêineres | `systemctl --user show` retornou `Operação não permitida` para o barramento; `systemd-run` existe, `podman` e `docker` não foram localizados no PATH. Delegação utilizável nesta sessão não demonstrada. |
| Recursos | `/home` tinha 4.107.902.976 bytes disponíveis na leitura pontual; reavaliar imediatamente antes de qualquer execução. |

Sem cgroup próprio gravável nem acesso ao gerenciador, não há mecanismo validável para impor `memory.max` aqui. Não se tentou contornar o sandbox, aplicar limite a `user.slice`, `system.slice` ou grupo compartilhado, executar sudo, instalar software ou modificar serviços. Possibilidade de outro ambiente ou terminal do hospedeiro não prova sua disponibilidade ou autorização administrativa.

## Controles e D/V

**Contenção configurada: não. Controles sintéticos A–G: não executados**, pois não existe nesta sessão um cgroup próprio em que possam validar contenção efetiva. `memory.max`, `memory.current`, `memory.peak`, `memory.events`, OOM, inclusão de filhos, timeout sob contenção, sobreviventes e limpeza permanecem não validados. O watchdog histórico faz apenas amostragem de RSS e não satisfaz o critério cgroup; não foi apresentado como contenção rígida.

**D/V sob contenção: não executados**; a autorização condiciona esses testes ao sucesso dos controles sintéticos. Não se atribuem os números 54 consultas, 36 comparações e nove adulterações a uma execução contida nesta atividade. Regressão ordinária da suíte, inclusive eventual repetição D/V fora de contenção, não prova o requisito operacional. Nenhum F foi gerado, consultado, habilitado ou executado.

## Procedimento mínimo para o terminal do hospedeiro

Não executar os comandos abaixo dentro do sandbox para contorná-lo. São verificações não privilegiadas para um operador no terminal do hospedeiro; não criam serviços permanentes nem alteram grupos compartilhados:

```sh
findmnt -T /sys/fs/cgroup
systemctl --user show --property=ControlGroup --property=Delegate
systemd-run --user --scope --unit=neurons-v03-f1-diagnostico --property=MemoryMax=268435456 /usr/bin/cat /proc/self/cgroup
```

O terceiro comando cria apenas um **scope transitório próprio**, que termina com `cat`; seu efeito é identificar se o gerenciador aceita um escopo limitado. Não comprova sozinho o valor efetivo de `memory.max`, inclusão de filhos, OOM ou validação; o operador deve registrar saída e falhas e confirmar desaparecimento do scope. Riscos: o gerenciador do usuário pode recusar a criação ou existir colisão de nome; não se altera `user.slice` ou outro grupo existente. Reversão: o scope transitório deve desaparecer após a saída; se permanecer, o operador deve interromper somente `neurons-v03-f1-diagnostico.scope` com `systemctl --user stop neurons-v03-f1-diagnostico.scope` e verificar seu estado. Não executar se o nome já estiver em uso.

Se o terminal do hospedeiro não permitir escopo delegado, será necessária decisão específica antes de qualquer comando administrativo: apresentar comando exato, justificativa, efeito, risco e reversão; não se solicita senha nem se presume que acesso sudo supere o isolamento. Com delegação disponível, uma atividade futura autorizada deverá validar os controles prospectivos A–J de [D04-E03](protocolos/V03-F1-P01_D04-E03_memoria_cgroup_APROVADA.md), inclusive leitura efetiva do limite, contadores e descendentes, testes sintéticos pequenos, timeout, sobreviventes, limpeza e somente então D/V. Não se fornece um comando de execução F.

## Classificação e limites

**C. CONTENÇÃO INDISPONÍVEL no ambiente desta sessão.** Nenhum limite efetivo foi configurado ou validado; não se afirma que a infraestrutura seja impossível no hospedeiro. Pendências: acesso a cgroup próprio efetivamente delegado, validação dos controles, D/V sob contenção, rechecagem de tempo inferior a 30 s e `/home` com pelo menos 3 GiB livres, congelamento prospectivo e autorização específica de F. O teto é 256 MiB de memória cobrada ao cgroup v2; 256 KiB de dados vivos continuam restrição não verificada, não indispensável à alegação local. Nenhuma conclusão homologa F1 ou competência cognitiva.

Regressões `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` passaram: **190 aprovados, zero falhos e um ignorado** (`r3_frozen_final`, EC-10). Houve três avisos históricos de métodos não usados na EC-04. D/V repetidos pela suíte não foram executados sob contenção validada.
