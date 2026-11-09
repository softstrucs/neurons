# NEURONS — Relatório executivo consolidado de recuperação

**Atividade:** preservação, organização e planejamento,2026-11-08. Nenhuma implementação cognitiva nova. O [Plano Mestre](../../NEURONS_PLANO_MESTRE.md) está **PROPOSTO, aguardando aprovação**, sem autorização executiva implícita.

## Estado local versus remoto

Repositório `softstrucs/neurons`, raiz `/home/aguiar/Documentos/Trabalho/Softstrucs/Softwares/Sys/neurons`, branch `devs`, HEAD `5066a3745709d45df6bfd5129abc71acc660f955`, origin `git@github.com:softstrucs/neurons.git`. A consulta `git ls-remote --heads origin` confirmou o mesmo commit para devs e `dee5148c5c8cbe124e2cfd8632ffebc5fb0cecd6` para main. **devs:0 commits exclusivos locais e0 remotos**, conferidos também por rev-list com a referência local correspondente. Não foi preciso fetch; não houve pull, merge ou rebase.

Igualdade de commits não significa árvore limpa: cinco Rust já estavam modificados e468 arquivos não rastreados continham trabalho essencial, além de702 ignorados. As cinco modificações foram preservadas. Os novos caminhos/remoções de origem dos seis movimentos são trabalho desta organização, não exclusão de conteúdo. Estado inicial completo está no inventário do backup; final no manifesto local e no diretório do backup.

## Backup integral realizado antes de reorganizar

- Arquivo: [/tmp/neurons-recovery-backup-8dt15y2x/neurons-integral.tar](/tmp/neurons-recovery-backup-8dt15y2x/neurons-integral.tar).
- SHA-256: `d7ff33e5a87d051ea942ab9028c4e4617089bd0ce550264b8aa7a479b9424eb4`.
- Tamanho: 136069120 bytes.
- Inventário/resultado: [/tmp/neurons-recovery-backup-8dt15y2x/input_inventory.json](/tmp/neurons-recovery-backup-8dt15y2x/input_inventory.json) e [/tmp/neurons-recovery-backup-8dt15y2x/verification.json](/tmp/neurons-recovery-backup-8dt15y2x/verification.json).
-2419 entradas de arquivo, incluindo1010 em .git e1409 da árvore de trabalho; todos os arquivos arquivados tiveram conteúdo/modo confrontados e hardlinks foram verificados contra seus alvos no arquivo. Diretórios, inclusive vazios, estão no tar.
- Oito amostras restauradas e comparadas: .git/HEAD, .git/index, .git/config, Cargo.toml, src/cognition.rs, plano v1.1, registro não rastreado RC01 e um artefato ignorado de target.
- Conteúdo-fonte integral foi reconferido antes de liberar os movimentos. Primeiro verificador recusou representação de hardlink do tar; tentativa comprimida foi conservada e não contou como backup aprovado. Segunda cópia/verificação tratou hardlinks explicitamente e passou antes de qualquer reorganização.
- **Limitação de retenção:** cópia local em /tmp, fora do worktree, não é cópia durável/off-host. Recomenda-se transferi-la para armazenamento permanente antes de futura manutenção/limpeza de /tmp. ACL/xattr e restauração integral em outra máquina não foram demonstradas; restauração de amostra não é essa alegação.

## Arquivos preservados, movidos e retirados da estrutura ativa

Os seis movimentos abaixo foram explícitos e conferidos por SHA-256. O destino comum é `docs/historico/planos_executivos/`; não houve modificação do conteúdo dos originais.

| Origem anterior | Nome no arquivo histórico |
|---|---|
| `neurons_plano_executivo_v1_1.md` | [neurons_plano_executivo_v1_1.md](../historico/planos_executivos/neurons_plano_executivo_v1_1.md) |
| `neurons_plano_executivo_evolucao_cognitiva_v0_1.md` | [neurons_plano_executivo_evolucao_cognitiva_v0_1.md](../historico/planos_executivos/neurons_plano_executivo_evolucao_cognitiva_v0_1.md) |
| `neurons_plano_executivo_evolucao_cognitiva_v0_2.md` | [neurons_plano_executivo_evolucao_cognitiva_v0_2.md](../historico/planos_executivos/neurons_plano_executivo_evolucao_cognitiva_v0_2.md) |
| `docs/planos/Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_PROPOSTA.md` | [Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_PROPOSTA.md](../historico/planos_executivos/Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_PROPOSTA.md) |
| `docs/planos/Neurons_v0_3_revisao_cientifica_pre_aprovacao.md` | [Neurons_v0_3_revisao_cientifica_pre_aprovacao.md](../historico/planos_executivos/Neurons_v0_3_revisao_cientifica_pre_aprovacao.md) |
| `docs/planos/Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_APROVADO.md` | [Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_APROVADO.md](../historico/planos_executivos/Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_APROVADO.md) |

Não restam planos executivos concorrentes na raiz nem em docs/planos, que agora só aponta ao Plano Mestre e ao arquivo. A v0.3 aprovada documentalmente com condições foi considerada como fonte adicional e arquivada junto de sua proposta e parecer; suas condições C01–C05 continuam preservadas e incorporadas como limites, não revogadas pela movimentação.

Somente dois arquivos existentes de navegação foram editados: README.md e docs/fundamentos/README.md. Foram criados o Plano Mestre e índices de docs, referência, histórico e planejamento, mapa de localização, classificação, este relatório e manifesto. Nenhum Rust, Python, Cargo.toml/Cargo.lock, teste, exemplo, protocolo, resultado científico ou catálogo de segurança foi modificado. Seis arquivos aparecem como origem removida no diff Git, mas o conteúdo está integralmente nos novos destinos e no backup; não são seis exclusões permanentes.

## Classificação e pendências de remoção

- **ESSENCIAL: 179 arquivos.**
- **HISTÓRICO: 523 arquivos.**
- **REFERÊNCIA: 5 arquivos.**
- **INCERTO: 702 arquivos.**

A [classificação completa](RECUPERACAO_classificacao.json) cobre1409 entradas anteriores, com razão por arquivo, categoria Git, origem/destino, hash e conflitos secundários. CONFLITANTE é anotação secundária para código essencial com mensagem inconsistente e planos históricos usados fora de seu contexto; não significa remover o módulo. Não houve item comprovado removível como REDUNDANTE.54 grupos de bytes idênticos foram identificados, incluindo cópias de evidência: identidade de conteúdo não elimina sua função de proveniência/path. Nenhuma dessas cópias foi apagada.

Permanecem no caminho original:702 artefatos ignorados/caches, classificados INCERTO quanto à necessidade de conservação específica; fontes/testes/scripts essenciais; protocolos e resultados históricos canônicos. Estes últimos não são fila executiva ativa, mas removê-los/movê-los afetaria consumidores, hashes e rastreabilidade. Sua conservação foi priorizada sobre uma limpeza visual genérica. Qualquer remoção de fonte ou caminho funcional necessita revisão de dependências/autorização específica; nenhuma foi executada.

## Estrutura final

```text
neurons/
├── NEURONS_PLANO_MESTRE.md       # único roteiro ativo proposto
├── README.md
├── Cargo.toml / Cargo.lock / .gitignore
├── src/                        # módulos funcionais;42 arquivos Rust preservados
├── tests/                      #94 arquivos Rust e instrumentos preservados
├── examples/                   #3 arquivos Rust preservados
├── scripts/                    # instrumentos existentes, sem execução operacional
├── docs/
│   ├── README.md
│   ├── referencia/             # índice, classificação, relatório e manifesto
│   ├── fundamentos/            # fontes conceituais e índice
│   ├── historico/
│   │   └── planos_executivos/  #6 originais íntegros, índice e mapa
│   ├── planos/README.md        # direciona ao Mestre e histórico
│   ├── protocolos/             # contratos/resultados preservados nos paths canônicos
│   ├── decisions.md            # registro histórico intacto
│   └── EC-*/V03-*              # evidências canônicas, não novos planos executivos
├── target/                     # ignorado; conservado
└── .git/                       # integral no backup, nenhum commit/publicação
```

## Conflitos técnicos comprovados e limites

1. **REC-T01:** src/main.rs doctor informa “núcleo ainda não implementado”, mas src/lib.rs expõe motor/memória/inferência; tests/cli.rs exige essa mensagem antiga. Corrigir programa e teste juntos é o próximo trabalho funcional recomendado, **não executado aqui**.
2. **REC-T02:** persistência atual codifica Engine+EngramStore e IntegratedState recebe Monitor externo; não inclui Circuit/pesos/potenciais/configuração cognitiva completa. É limite do contrato, não bug de um round-trip que nunca prometeu esses dados.
3. **REC-T03:** sessão textual usa Monitor/Observed legados, sem expor o fluxo completo tipado QueryCoordinator/aquisição/conceitos. Não atribuir à CLI as garantias dos novos coordenadores antes de integrá-los.
4. **REC-T04:** partes essenciais da árvore local estão não rastreadas; o commit remoto não reproduz sozinho essa implementação. O backup preserva o conjunto; não foi feito commit para ocultar essa pendência.
5. **REC-T05:** planos históricos têm estados diferentes conforme época (p.ex. EC-02.4-R2 pendente em v0.1 e concluída em v0.2). Isso foi reconciliado por fonte/cronologia, sem chamar o snapshot antigo de falso nem reimplementar etapas.

Não foi comprovada dependência Rust desnecessária: o pacote não declara crates externas. Nenhuma foi instalada/removida. Não existe docs/architecture.md sugerido como alvo no plano antigo; a arquitetura funcional consolidada está no Mestre, sem fingir que o arquivo anterior existia.

### Navegação histórica e pública

Antes dos movimentos, a varredura encontrou753 links Markdown locais fora de resultados, sem alvo inexistente. Não foram encontradas referências aos seis nomes de planos em Rust/Python de src/tests/scripts/examples; isso é busca estática, não prova de inexistência de consumidor externo/dinâmico. Caminhos funcionais não foram alterados.

Após a movimentação, todos os links dos novos índices ativos/Plano Mestre foram conferidos. **92 referências relativas internas históricas não resolvem diretamente fora de seu contexto original.** Não foram silenciosamente retocadas em protocolos congelados. O [mapa de localização original](../historico/planos_executivos/LOCALIZACAO_ORIGINAL.json) registra97 vínculos afetados com origem/resolução antiga e alvo atual existente; o backup permite restaurar a árvore original. Isso é pendência explícita de navegação histórica, não afirmação de ausência global de links quebrados. Resolução pelo mapa não é alteração da semântica, critério ou hash científico.

Essa escolha preserva integralmente fontes científicas e evita aliases de planos concorrentes na raiz. Uma futura vista de consulta pode adaptar links sem substituir os originais; não há necessidade de outra etapa de planejamento antes de corrigir o comportamento funcional da CLI.

## Estado real das capacidades e verificações

Código de motor, neurônios/sinapses, memória, plasticidade explícita, dois passos, monitor e sessão existe. M1–M5/CIC acrescentam contratos e coordenadores locais. **Compilação atual lib+CLI passou**, em1,99s reportados pelo Cargo, com rustc/cargo1.88.0 e target exclusivamente em /tmp. Isso confirma tipos/compilação, não comportamento ou ciência.

Comando efetivamente executado: `CARGO_TARGET_DIR=/tmp/neurons-recovery-cargo-check cargo check --offline --lib --bin neurons`. Nenhum teste funcional, painel científico, benchmark, serviço, systemd/cgroup ou programa candidato foi executado. Testes unitários e resultados históricos foram examinados como fontes; não declarados reexecutados. Não houve download/dependência nova. A consulta remota inicialmente falhou por DNS restrito e passou após acesso de rede autorizado para Git somente leitura.

M5 registra150 avaliações e28 recuperações com sucesso funcional restrito, não aprendizagem autônoma/neural. CIC-I1 registra revisão simbólica em H23; CIC-I3 registra bloqueio de V. EC03–12 guardam AJUSTAR e limites, EC-10 R3 final não executado por quota insuficiente e checkpoint limitado EC11. Não se homologam compreensão geral, transferência genuína, aprendizagem neural ou autonomia a partir da existência de módulos/relatórios.

## Plano Mestre e próximo passo

O plano reconcilia v1.1/v0.1/v0.2 e a v0.3 condicionada com o estado real, preserva neurônios, sinapses, engramas, percepção, aprendizagem, raciocínio, investigação, metacognição, pedagogia, agentes e CLI/chat. MP00 é esta recuperação; MP01–MP11 são entregas funcionais propostas com pré-requisitos, módulos, implementação, testes/aceite, avanço dentro de intervalo autorizado e parada humana. Não cria fases apenas para produzir documentos.

F0/F1/F2 são recortes de versão funcional com critérios objetivos, não homologações de inteligência. Aprovação do índice não autoriza todo roteiro; uma autorização futura pode delimitar um intervalo com rotina autônoma sem novos prompts por cada leitura/teste/edição reversível, respeitando gates e restrições. Dependência ausente/falha/escopo novo interrompe o correspondente avanço; não alterar critérios para passar.

**Próximo passo funcional recomendado:** autorizar MP01 para corrigir doctor e teste CLI, executar uma regressão básica previamente inspecionada e tornar a base local honesta/reproduzível. Depois, apenas se incluído no intervalo autorizado, integrar a sessão ao núcleo tipado existente MP02; não criar outro motor ou reiniciar uma sequência de dossiês de governança.

## Restrições e encerramento

G2_real=NotEvaluable; G7_reserved_access=NotAuthorized; reserved_DVF_access=NotAuthorized. P2(378 células/22 fluxos e quatro deny) intacta;11 achados/33 alternativas/14 propostas/20 riscos/P7-RISK01 não recebem promoção. CIC-I6/P8 não iniciado. Nenhuma criptografia real, chave, certificado, credencial, acesso reservado ou mudança de permissão. Sem commit, push, merge, rebase, reset ou limpeza/exclusão em massa.

Esta organização não é aprovação do Plano Mestre nem autorização para MP01. O manifesto final verifica preservação do código/testes/protocolos/resultados/ignorados e cada movimento em relação ao backup. O histórico anterior é recuperável, mas a cópia em /tmp requer retenção permanente posterior. Encerrar esta recuperação e aguardar revisão do plano/escopo funcional seguinte.
