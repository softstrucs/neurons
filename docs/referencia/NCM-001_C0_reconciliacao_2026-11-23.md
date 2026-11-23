# NCM-001/C0 — Reconciliação de entrada

Data: 23/11/2026. Executor: Codex. Mandato: instrução atual do responsável para auditoria, preservação e continuidade técnica com gates. Resultado: reconciliação documental e Git concluída no recorte abaixo; homologação funcional e liberação da árvore preexistente para publicação **não concedidas**.

## Base verificável e precedência

Branch `devs`; HEAD inicial `bc42f236e508906fd37f31f18ca263f7bf1cc836`. Remoto `origin`, `git@github.com:softstrucs/neurons.git`. Consulta direta `git ls-remote origin refs/heads/devs refs/heads/main` confirmou devs no mesmo HEAD e main em `dee5148c5c8cbe124e2cfd8632ffebc5fb0cecd6`. Referências locais: zero commits exclusivos de cada lado; índice vazio; stash vazio. Nenhum commit local pendente de push identificado nesse instante. Isso não valida conteúdo da árvore nem configurações de proteção do GitHub.

Leitura integral das fontes principais: Controle Mestre, Plano Mestre, Visão v1.1 e registro de aprovação, PIFI v1.0 e progresso. Contratos/documentos complementares consultados: M1, CIC-I1, fundamentos aprovados e diretrizes complementares, CPF-I03/ENC-001 e proposta arquitetural de 21/11. Inspeção seletiva de interfaces e diffs; não houve execução de instrumentos reservados. Não se declara revisão integral dos 451 artefatos agrupados como custódia/protocolos.

Precedência: mandato atual do responsável; Plano Mestre e contratos científicos nas suas competências; Visão Estratégica como identidade conceitual; PIFI e progresso como execução; NCM como índice. A Visão §15 declara expressamente a autoridade científica do Plano Mestre. As proibições históricas de commit/push não revogam a autorização atual, condicionada à revisão e aos testes. A autorização atual não modifica critérios científicos, libera dados reservados nem aprova E13 ou produção. Cabeçalhos antigos de F2 não iniciada são registros históricos: os aceites posteriores específicos prevalecem no seu recorte.

## Inventário e preservação

O [inventário completo](NCM-001_C0_inventario_2026-11-23.json) registra caminho, status, tamanho, SHA-256, agrupamento preliminar, hashes normativos e cronologia de commits. Fotografia anterior aos artefatos desta auditoria: **347 rastreados, 15 modificados, 544 não rastreados, 1.707 ignorados; 559 arquivos pendentes**. O status padrão agrupa diretórios: as 233 entradas históricas não são comparáveis diretamente a 559 arquivos expandidos. Ignorados foram contados, sem revisão de conteúdo. Não foi criado backup integral novo nem segunda cópia independente.

| Grupo primário | Arquivos | Tratamento |
|---|---:|---|
| Documentação operacional | 4 | Revisar atualização histórica antes de versionar |
| Axonal/núcleo e dependências | 33 | Separar seleção de saídas, condução, adaptador e testes |
| Integração CPF/associativa/runtime | 21 | Revisão por hunk e validação pública focal |
| Custódia e protocolos | 451 | Preservar; publicação depende de revisão específica de acesso/conteúdo |
| Cópias históricas | 23 | Preservar; não adicionar por conveniência nem apagar |
| Coordenação/E13 | 12 | Código presente; aprovação da proposta não demonstrada |
| Interface conversacional | 5 | Unidade funcional candidata após revisão/testes |
| EDF experimental | 10 | Fontes presentes; não equivalem a protocolo aceito |

Classificação por caminho, não certificação semântica. EC-10 R3 e seus dois suportes estendidos estão explicitamente excluídos de publicação/execução, independentemente do grupo primário. Autoria e sequência exata das alterações não commitadas não são recuperáveis só pelo Git; mtime não é evidência de autorização. Nenhuma fonte, backup, evidência ou documento foi removido, movido ou formatado nesta auditoria.

## Timeline recuperada

| Data local dos commits/registro | Marco | Evidência e limite |
|---|---|---|
| 08/11 | F1 aceita com ressalvas | `afe2bd5`; aceitação delimitada, não produção |
| 08–10/11 | MP06, MP05, MP07–09, supervisão e composição, TCR-01 | Commits até `7c12e22`; resultados históricos preservados |
| 13/11 | Contrato/supervisor/CLI e aceite F2 | `94ff720`, `3d8c774`, `10c8983`, `60dcad1` |
| 13–14/11 | Pré-registro e aceite F3-01; Visão | `f5bcbeb`, `dc330ae`, `e4898f8`; não autoriza F3-02/03 |
| 14/11 | Ciclo experiencial e memória multiepisódio | `ead419f` até `5435f94`; PIFI mantém limites |
| 15–19/11 | Investigação, condições, sondas, contraexemplos, CLI Runtime | `295b6b5` até `c13139c` |
| 20/11 | Dendritos/persistência e MP05 | `b9a4d1d`, `bc42f23`, último commit atual |
| 21/11 | Conversação, validações V1, E12-C1, EDF inicial | Progresso local, sem commit correspondente posterior ao HEAD |
| 21–22/11 | E13 proposto; CPF-I03 encerrada no recorte | Documentos locais; CPF-001/E10 continuam abertas |
| Até 23/11 | EDF R2-E e série AXN/INT | Fontes locais; sem logs/contratos completos suficientes para datar e aceitar cada incremento |

Cronologia completa dos commits desde `5066a37` consta do inventário. Resultados relatados de épocas diferentes não são somados como cobertura independente.

## Marcos principais: evidência → lacuna → ação

| Marco | Estado reconciliado | Próxima evidência necessária |
|---|---|---|
| E00 | Parcial: inventário, remoto e compilação verificados | Revisão dos conjuntos e regressão pública seletiva; reprodução fonte limpa |
| E01 | Recorte técnico registrado no progresso | Reproduzir diferenças reais de configuração via alvo público revisado |
| E02 | V1 relatada, percepção estruturada limitada | Confirmar ineditismo, procedência e controles no conjunto atual |
| E03 | Adaptação MP05 e instrumentação; causalidade neural não demonstrada | Manter distinção entre integração funcional e hipótese causal com protocolo próprio |
| E04 | Aquisição/revisão H23 delimitadas | Validar o fluxo CPF existente sem ampliar família ou fornecer gabarito |
| E05 | Consultas compostas e abstenção delimitadas | Regressão focal de composição, oposição e recursos |
| E06 | Persistência/recuperação parcial | Reproduzir retenção, falhas, versões e autoridade nova |
| E07 | Investigação supervisionada limitada | Confirmar concessões separadas e rastreio de incorporação |
| E08 | Parcial, gramática/intenções controladas | Aprendizagem de expressões/composição inédita ainda pendente |
| E09 | Planejamento e conversação no recorte | Revalidar metas inéditas, recusa e recuperação; E09 conversacional não encerra E09 integral |
| E10 | Aberta | Prospectivos, controles, contexto/domínio, retenção/interferência próprios |
| E11 | Não encerrada | Mesma infraestrutura em múltiplos domínios com avaliação independente |
| E12 | E12-C1 relatada como pré-homologação | Instalação limpa, segurança, recursos, backup/restauração/rollback e gates |

F1/F2/F3-01 mantêm seus aceites formais específicos e todas as ressalvas. Nenhuma etapa recebe novo aceite científico por esta auditoria.

## CPF e desvios

| ID | Evidência localizada | Classificação e retorno |
|---|---|---|
| CPF-001 | Parecer CPF-I03 §5/§8, `curriculum.rs`, teste CPF-I01 | Obrigatória e aberta. Contador de trabalho concluído explicitamente não demonstra aprendizagem. Prioridade de retorno após estabilização E00 |
| CPF-002 | Supervisor MP09, Runtime e conversação; coordenador local | Cobertura supervisionada parcial; solicitação/prioridade não equivale a aprendizagem imediata garantida |
| CPF-003 | Proposta PCC-01/E13 e código de coordenação | Autonomia contínua não habilitada. Não executar E13 sem decisão própria |
| CPF-004 | `assess_epistemic`, investigação discriminante | Lacunas tipadas delimitadas; identificação autônoma geral não demonstrada |
| CPF-005 | MP07, seleção de sondas e investigação Runtime | Exploração limitada e permissionada; investigação aberta pendente |
| CPF-006 | `pursue_learning_goal`, currículo e coordenador | Objetivos fornecidos e supervisão finita; planejamento intelectual autônomo não demonstrado |
| CPF-007 | Diagnóstico ExperienceNeeded e reservas de custo | Necessidade local observável; utilidade/finalidade geral da aprendizagem não demonstrada |
| CPF-008 | Relatórios, `cpf_evidence`, generalização intrafamília | Provas delimitadas relatadas; competência ampla, retenção prolongada/interferência pendentes |
| CPF-I03 | Parecer de 22/11: 17 específicos/1.140 gerais relatados | Encerrada apenas no recorte documental. Não reproduzido; comando amplo relatado conflita com seleção restrita do Mestre |
| EDF-01/R2-E | Dez arquivos EDF, cinco R2-E; 28 funções `#[test]` no total | Implementação experimental presente; contagem estática não significa testes aprovados. Progresso para no EDF inicial |
| EDF-01/R2-F | Nenhum arquivo correspondente; referências apenas no NCM | Contrato, objetivo, autorização e execução não localizados no escopo pesquisado. Bloquear somente R2-F, sem inventar continuação |
| AXN-002 / AAN-002 | `axn_002_functional_distribution`, série INT e adaptador | AXN isolada predominantemente; equivalência do nome AAN não comprovada por contrato próprio |
| INT-P6.6.3.3-C1 | `axon_outcome_adapter.rs` e testes correspondentes | Adaptador calcula relatório e vetor de entregas; não agenda eventos nem demonstra aceite de fila |
| E13/PCC/PCE/SCDE | Proposta de 21/11 e módulos `e13_*` | Desvio com implementação preexistente, aprovação não localizada; preservar e separar da publicação |

Correção do resumo NCM: o `Circuit` atual chama `Axon::select_outgoing` e `lib.rs` declara `mod axon`. Portanto existe integração parcial de seleção de saídas, apesar do comentário de isolamento em `axon.rs`. Condução/latências/política/adaptador experimentais não estão demonstrados como fluxo produtivo integrado. `Emission::delayed` é uma extensão local. Validar equivalência, ordem, overflow e identidade antes de publicar esse conjunto; não prolongar série axonal por contagem de testes.

Busca R2-F: nomes de arquivos visíveis (incluindo ocultos fora de `.git`, `target` e resultados de protocolos), conteúdo dos documentos operacionais/fundamentos/decisões e referências pertinentes, histórico de commits e inventário. Não foram vasculhados dados reservados, ignorados, conversas externas ou objetos Git inalcançáveis; ausência é delimitada, não universal.

## Riscos e verificações atuais

1. **Crítico — EC-10 R3:** diff preexistente troca suportes, amplia partição final de 256 para 512 itens e remove `ignore` do teste congelado. Conflita com a preservação do bloqueio histórico exigida pelo Mestre. Preservado sem executar; requer decisão científica específica antes de aceitar/publicar. Não houve correção retroativa nem leitura dos suportes estendidos nesta auditoria.
2. **Alto — publicação conjunta:** módulos essenciais não rastreados, código compartilhado entre frentes e centenas de artefatos de custódia. `git add -A` publicaria material não revisado e possivelmente reservado. Autorização atual cobre somente conjuntos efetivamente revisados.
3. **Alto — autonomia/E13:** presença de código não comprova aprovação da proposta ou segurança operacional. Não ativar processamento contínuo por inferência.
4. **Alto — perda/reprodução:** hashes preservam identidade, não são backup. Evidências temporárias e backup no mesmo disco não satisfazem cópia independente/restauração de produção.
5. **Alto — segurança operacional:** TOCTOU, checksum sem autenticação, autoridade e locks exigem revisão específica; nenhuma resistência adversarial geral demonstrada.
6. **Médio — rastreabilidade:** progresso defasado, nomenclaturas ambíguas e ausência de contratos/logs completos dos últimos experimentos. Não converter relatos de suíte ampla em aprovação científica.

Executado: `git status`, `branch -vv`, histórico, `diff --stat`, `diff --numstat`, `diff --check` (aprovado), inventário expandido/hashes, consulta remota, stash e divergência; compilação `CARGO_TARGET_DIR=/tmp/neurons-ncm001-c0-target cargo check --offline --locked --lib --bin neurons` **aprovada**, 12 avisos `dead_code` (3 axonais e 9 experimentais neurais). Nenhum teste funcional foi executado em C0. Não houve instalação de dependências, deploy, serviço, mudança de rede ou acesso ao onedell.

Triagem de segredos: 82 arquivos pendentes públicos selecionados, padrões de chaves privadas, tokens GitHub/AWS e URLs com senha: nenhuma ocorrência. Excluídos custódia, backups, EC-10 R3 e suportes. Não é auditoria completa nem liberação dos demais arquivos. Cinco arquivos >1 MiB listados no inventário, todos no conjunto de protocolos. Espaço observado: aproximadamente 9,6 GiB livres em /home e 9,5 GiB em /tmp antes da compilação. Não se alegam limites de RSS medidos ou segurança integral.

## Agrupamento de commits e próxima execução

Ordem proposta, sempre revisão de conteúdo/dependências → testes pertinentes → staging nominal → revisão do índice → commit → push não forçado confirmado:

1. **C0:** somente relatório e inventário novos desta auditoria. Não incluir progresso/Controle Mestre preexistentes por arrasto. Registrar publicação efetiva em checkpoint posterior; nunca presumir push.
2. **C1/E00:** revisar/publicar Controle Mestre e atualização operacional, distinguindo os 375 acréscimos antigos do progresso dos registros novos. Preparar conjunto fonte completo e regressão segura. Primeiro teste focal candidato: `python3 scripts/test_public.py --target runtime_cli --target concept_goal_supervision --target development_runtime`, após revisão dos três alvos atuais e suas dependências. Não executar `--all-targets` nem EC-10 R3.
3. **CPF/interface:** separar por hunks núcleo de evidências/CPF, revisão do Runtime e conversação; revalidar cada unidade e todos os arquivos necessários, sem testes reservados.
4. **AXN:** integrar apenas conjunto mínimo de seleção de saídas e compatibilidade, se sustentado pelos testes públicos revisados; manter condução/adaptador como experimento explicitamente delimitado. Retornar a CPF-001.
5. **EDF:** reconciliar R1/R2/R2-C/D/E com critérios e logs próprios; R2-F depende de recuperar contrato. E13 e custódia ficam fora desses grupos.

Menor incremento funcional seguinte: verificar uma fatia CPF-001 no cérebro existente ligando solicitação supervisionada → diagnóstico real → experiência autorizada → revisão → avaliação inédita → retenção em novo processo, com controle sem ensino e concessões independentes. Primeiro reproduzir/revisar os componentes já presentes; implementar apenas lacuna efetivamente demonstrada. Não criar segundo aprendiz, ampliar H23, habilitar autonomia ou aprovar ciência por compilação.

## Preparação de produção e retomada

PROD-00 aberto até fonte/release reconciliados; PROD-01 capacidades delimitadas e demonstradas; PROD-02 instalação limpa; PROD-03 compatibilidade, backup/restauração e rollback; PROD-04 segurança; PROD-05 limites e observabilidade; PROD-06 staging equivalente e recuperação. Todos permanecem abertos. PROD-07 exige aprovação explícita para deploy/exposição; PROD-08/09 não iniciados. Nenhum acesso SSH ou mudança no onedell foi realizado. Debian 12 e topologia são informações documentais a confirmar, não observações desta sessão.

Antes de instalação: inventário somente leitura do host autorizado, usuário/diretórios/serviços, dependências e recursos; release imutável, estado separado, permissões mínimas, backup independente com restauração ensaiada, migração e rollback testados, logs limitados e health checks adequados à CLI. Alterações SSH/VPN/firewall/DNS/TLS exigem aprovação específica e rollback. Não há gate satisfeito por intenção.

**Estado de retomada:** C0 registra auditoria e riscos, sem liberar as alterações preexistentes. Próxima ação executável: revisar os três alvos públicos de C1 e suas dependências, reproduzir regressão seletiva e fechar manifesto de fontes; depois reconciliar Controle Mestre/progresso por acréscimos sem apagar história. Conflito EC-10 R3 e ausência R2-F bloqueiam seus próprios conjuntos, não essa manutenção. Registro da autorização atual neste relatório complementa, sem reescrever, proibições históricas de publicação.
