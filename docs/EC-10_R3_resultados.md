# EC-10 — R3: resultado inconclusivo por insuficiência de testemunhas

## Parecer e interrupção

Base `ac2b889b067b6487244560141a30da73169dc5d4`. **R3 inconclusiva e limitada; classificação AJUSTAR. O painel final não foi executado.** A pré-verificação encontrou, depois de excluir os 80 vetores realmente usados no final R2, apenas **20 testemunhas invariantes positivas** elegíveis para o critério final R3, enquanto o protocolo congelado exige 28: 12 para aquisição, oito para revisão e oito para avaliação. Não foram relaxadas quotas, alterados critérios ou escolhidos subconjuntos convenientes após essa constatação. A EC-10 permanece aberta; esta era a última correção autorizada para este ensaio e nenhuma R4 foi iniciada.

A R2 já tinha mostrado que as oito consultas discriminantes estavam abstidas antes da revisão: 24 episódios deixaram quatro hipóteses compatíveis. Acerto posterior significou abstenção→resposta, não substituição de resposta antiga correta. Seu controle compartilhava enumeração e consulta. Os resultados inicial, R1 e R2 permanecem intactos.

## Critérios e pré-verificação

[Protocolo R3](protocolos/EC-10_R3_revisao_controle_independente.md): F antigo `bit3=1 e bit5=0`, revisado acrescenta `bit4=1`. F=256..511 exclui os vetores efetivamente usados em F R2 e está fora do espaço u8 do diagnóstico/R1. D=0..127, V=128..255. A família conjuntiva e a extensão simbólica de nove bits R2 são compartilhadas; não se alega novidade estrutural global. As faixas são disjuntas, mas F não foi materializado como conjunto válido de aquisição/revisão/avaliação.

Contagem prévia F por classe: **20 discriminantes, 20 invariantes positivas e 136 invariantes negativas**. O desenho exigia respectivamente 20, 28 e 32 valores distintos; falta de oito invariantes positivas impede satisfazer simultaneamente treino, revisão e avaliação. A tentativa inicial de `r3_preflight` falhou explicitamente por essa condição. Em seguida, sem alterar quotas, critérios ou gerador, o instrumento foi ajustado para registrar/assertar o impedimento e marcar `r3_frozen_final` como ignorado, evitando execução acidental na suíte. Isso torna o teste de guarda aprovado como detecção da insuficiência, **não** transforma o painel final em aprovado ou executado.

A R3 portanto não estabeleceu uma quantidade válida de testemunhas finais adquiridas e avaliadas, nem verificou respostas antigas corretas, substituições, retenção ou regressões em F. Desenvolvimento e validação passaram e chegaram à guarda de 24 respostas antigas corretas nos seus próprios conjuntos; não substituem o painel final e não são resultados generalizáveis. Os hashes dos instrumentos foram registrados após a introdução da guarda explícita; como F não foi executado, não há alegação de congelamento anterior a um painel final.

## Controle e limites

`tests/support/ec10_r3_candidate.rs` copia o corpo da R2 byte a byte, excluindo somente o comentário inicial; o histórico não foi modificado. `tests/support/ec10_r3_control.rs` implementa estado próprio, enumeração recursiva por combinações de literais, consulta própria e busca de sufixo desde zero, sem importar ou chamar enumeração, consulta, estado ou atualização do candidato. Os dois compartilham especificação pública, dados, regras de avaliação e autoria. Essa separação de implementação é verificável por inspeção e comparação em D/V; não equivale a referência externa independente, nem foi validada em F. O teste compara decisões nos casos D/V; não estabelece equivalência completa de estados/proveniência ou custos.

A referência é função do avaliador, sem revisão externa independente. Controle congelado e prática com rótulos antigos foram preparados para comparar estados, mas não há resultados finais. Renomeação não foi avaliada nesta R3. Não se demonstram substituição de respostas antigas corretas, interferência em pesos compartilhados, retenção após reinício, transferência ou cumulatividade; não inferir incapacidade universal a partir de um desenho inviável.

## Verificações e recursos

`cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check` passaram como comandos: **181 testes aprovados, zero falhos e um ignorado**. O ignorado é `r3_frozen_final`, não executado devido à guarda; a suíte inclui duplicações históricas por `include!`. A execução inicial de pré-verificação **falhou** antes de a guarda ser registrada como detecção esperada; a regressão posterior não apaga essa falha metodológica. Três avisos históricos EC-04 de código não utilizado. Desenvolvimento e validação isolados levaram aproximadamente 0,10 s cada segundo seus relógios internos; guardas internas de 30 s, RSS 256 MiB e /home >=3 GiB passaram nesses testes. Não houve medição isolada de RSS nem painel final; teto fino de dados por condição não foi medido integralmente. /home observado inicialmente 4.620.484.608 bytes e após preparação 4.616.929.280 bytes; não há monitoração contínua.

Hashes: protocolo `36cbe7349abdb3ff13f1ae8dea753eb2316665945a0cc23e2b4dddd4c78008ab`; instrumento `ef7586ac47515209b6f78ee1ac30a16d69fc7c0d27849b7a763b45f81162bdbf`; controle `62765f7196f3be2b72d44ebf713fe49b27acb3c721dbb7e458f15610d99e4106`; cópia do candidato `a8b0646d2ab332a62cb3ca9cc7aa8087f87ecfc4563a65b5c02fb260f9efc607`. Hashes não comprovam pré-registro independente.

A1–A8, produção e histórico preservados. Uma avaliação futura exigiria outro desenho de partição ou espaço, quotas e testemunhas verificáveis antes da execução, com nova autorização. Não se iniciou correção posterior; EC-10 permanece aberta.
