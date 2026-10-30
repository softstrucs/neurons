# EC-07 — Auditoria científica final e encerramento delimitado

## Parecer e sequência

Base `d9b2d0a718974b261e4662e2de30ed3831f34678`, devs inicialmente limpa e sincronizada. A EC-07 pode ser **encerrada como investigação experimental delimitada, com classificação AJUSTAR e conformidade metodológica parcial**. Os resultados restritos aos painéis são reproduzíveis; as ressalvas não invalidam suas contagens, mas impedem a aceitação global do plano como demonstração de que selecionar investigação causa aprendizagem posterior.

Sequência documental: detecção de lacunas (`dd1f998`); hipóteses operacionais (`f2eac68`); critérios de intervenção (`0bf240a`); protocolo e comparação inicial (`e70a15a`); correção prospectiva PRNG (`8b96174`); ganho informacional (`d9b2d0a`). As três primeiras subetapas são levantamento/especificações, não implementações autônomas. Os protocolos e resultados históricos permanecem íntegros. Identificação por hash antes do final está relatada nos resultados, mas não constitui comprovação independente de pré-registro cronológico.

## Rastreabilidade e conformidade

| Item | Evidência e conformidade | Limite |
|---|---|---|
| Lacunas | `Monitor::review` distingue falta de suporte, conflito e limites em consultas fornecidas; levantamento em [EC-07](EC-07_detectar_lacunas.md). | Não demonstra busca autônoma de lacunas em domínio aberto. |
| Hipóteses | [Especificação](EC-07_hipoteses_operacionais.md) distingue candidata, previsões e teste; `Hypothesis` existente representa consulta. | Não demonstra geração autônoma de hipótese investigável. |
| Intervenções | [Critérios documentais](EC-07_selecao_intervencoes_orcamento.md) distinguem utilidade, viabilidade e autorização. | Não há política integrada de planejamento ou permissões gerais. |
| Painel histórico | `tests/ec07_investigation.rs`: 336 decisões de três políticas, mais 112 do controle; previsões reproduzidas. | Índice `(seed-1)%n` é sequência fixa, não PRNG; comparação aleatória histórica é incompleta, como já reportado. |
| Painel R1 | `tests/ec07_random_r1.rs`: 2.688 decisões das três políticas, mais 896 do controle; duas repetições idênticas. | SplitMix64 com rejeição e dois vetores conhecidos verificados; sementes fixas e reinício em cada caso produzem dependências entre perfis/alvos. Não inferir população. |
| Ganho informacional | `tests/ec07_information.rs`: 256 casos, 64 por condição; filtragem reproduzida. | `apply` é avaliador externo, não atualização do núcleo. Dados individuais ficam em memória, não arquivados; `Row` não contém campo explícito de motivo de abstenção, apenas condição e booleano. Rastreabilidade individual é parcial. |
| Competência | Nenhuma medição de mudança persistente, avaliação posterior independente ou retenção no painel de informação. | **Não mensurada nem demonstrada aqui**; não apagar evidência histórica separada de aquisição simbólica EC-04. |

## Resultados reproduzidos

Na R1, em duas repetições, política ativa obteve 384/896 respostas corretas; PRNG 128/896; fixa 128/896; controle sem evidência 0/896. Nos perfis V/F/A, ativa obteve 128/128 cada; PRNG 36/128, 40/128 e 52/128; fixa 0/128, 128/128 e 0/128. Nos perfis sem discriminação, orçamento insuficiente, ação proibida e sinal inconclusivo não houve resposta. Assertivas confirmam autorização e custo das ações escolhidas. A ativa supera o PRNG no agregado dos três perfis em cada semente, mas a fixa empata na classe F. O controle inicial não era aleatório; R1 é PRNG determinístico, não entropia física.

O limiar `n.wrapping_neg()%n` e descarte de valores menores no código eliminam viés de módulo **sob hipótese de saídas uniformes**; o teste cobre posições possíveis, não certifica distribuição estatística. Os mesmos estados iniciais por semente são usados em perfis/alvos, e repetições são duplicatas. Relevância e previsões de discriminação são fornecidas à ativa e disponíveis às demais; o ambiente produz diretamente sinal sintético correspondente ao alvo. Compartilhamento de gerador, autoria, pressupostos e avaliador limita independência metodológica. Isso sustenta comportamento das regras no painel, não descoberta ou vantagem cognitiva geral.

No painel informacional, evidência discriminante reduz H={0,1} para um elemento em 64/64 casos; sem evidência, redução zero; repetição acrescenta zero após primeira leitura; 64 contradições levam a conjunto vazio e abstenção com ganho **indefinido**. A soma impressa zero para ganhos inexistentes não é um ganho observado zero. Custos somados 64, 0, 128 e 64; razões válidas 1 e 0,5 nas condições com evidência válida. Informação recebida, redução externa, estado transitório e aquisição persistente são diferentes: somente as três primeiras aparecem de modo delimitado; persistência e competência não foram avaliadas.

## Limites e julgamento

Conformidade funcional dos testes com os resultados agregados: **demonstrada no painel**. Conformidade metodológica: **parcial**, pelas previsões fornecidas, dependências e controles com autoria/pressupostos comuns; registro sem campo explícito do motivo no painel informacional; ausência de manifesto externo independente; dados por condição e trabalho de seleção não medidos integralmente. O protocolo informacional prevê registros com motivo e fonte; `Row` guarda ID e condição, mas não campo próprio de motivo nem identificador de fonte distinto do ID. A condição permite inferir o motivo nos casos fixos; isso é conformidade parcial de registro, não falha das contagens. O teste não valida dinamicamente uma fonte independente de verdade; evidência é sintética. Nenhuma correção foi aplicada após resultados.

Não houve avaliação de ganho persistente, retenção após reinício, transferência estrutural ou entre domínios, aprendizagem neural, descoberta ou investigação autônoma. Os controles literal e simbólico das etapas anteriores mantêm suas limitações históricas; não há prova de superioridade neural. Pendências futuras: fonte e avaliação independentes, rastreabilidade explícita de motivos e evidências, instrumentos de custos comparáveis, amostragem com dependências controladas e experimento causal de ganho de competência com retenção e dados inéditos. Não são protocolos ou implementações autorizados por esta auditoria.

A1/A2 orientam memória e proveniência; A3/A4 pedagogia e investigação comunicada somente quando realizada; A5/A6 idiomas; A7 rigor; A8 competência cognitiva generalizável. Os resultados contribuem para critérios e controles iniciais, sem demonstrar essas diretrizes como capacidades completas. Fundamentos e A1–A8 preservados.

## Reprodução e recursos

Foram reexecutados dez testes focais dos três arquivos EC-07, incluindo vetores PRNG e painéis. Também passaram `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check`: **153 aprovados, zero falhos e zero ignorados**. Três avisos preexistentes de código não utilizado em suportes EC-04. Os painéis isolados, sem `nocapture`, foram medidos por `wait4`: histórico 0,003814 s/7.336 KiB, R1 0,008177 s/7.348 KiB, informação 0,004075 s/7.348 KiB. RSS pode incluir base do lançador e não mede dados por condição; guardas internas de tempo/RSS/disco passaram. `/home` livre observado inicialmente 4.645.167.104 bytes e após painéis 4.642.537.472 bytes; sem amostragem contínua. Teto fino de dados por condição permanece não verificado integralmente.

Encerramento recomendado e registrado nesta auditoria como investigação delimitada, não homologação da aceitação global da etapa. Próxima etapa nominal do plano: **EC-08 — Metacognição e autorregulação**, não iniciada.
