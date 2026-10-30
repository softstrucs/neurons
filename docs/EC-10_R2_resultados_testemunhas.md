# EC-10 — R2: testemunhas discriminantes de revisão

## Parecer e escopo

Base `29ab8b3380809cb313e479ec791f64d23e743a74`. **AJUSTAR, conformidade metodológica parcial.** R2 corrige a ausência de testemunhas da [R1](EC-10_resultados_particoes_renomeacao_R1.md): há rótulos antigos/revisados distintos e resposta posterior condizente no ensaio delimitado. A inferência não demonstra substituição de uma resposta antiga correta nas testemunhas discriminantes, pois antes da revisão o candidato se absteve em todas elas. Controle de indução não é integralmente independente: candidato e controle compartilham `rules` e `query`, embora com políticas diferentes de busca de sufixo. A referência é função externa de bits, mas de mesma autoria e pressupostos. Não homologar transferência genuína ou aceite global EC-10.

A união de 122 vetores do diagnóstico inicial com as três partições R1 de 134 valores esgota o universo `u8` de 256 valores. Para não reutilizá-los, R2 criou um **indutor experimental isolado de nove bits**, inspirado na família/sufixo EC-04, não uma capacidade pré-existente do núcleo. O bit adicional identifica a faixa final e pode servir de correlação contextual; semântica ou generalização de domínio não são demonstradas. Nenhum código de produção alterado.

## Critérios, partições e congelamento

F antigo: `bit0=1 e bit1=0`; F revisado: antigo e `bit2=1`. D/V usam critérios de um bit antigos e dois revisados, F dois/três; a família conjuntiva é compartilhada e estruturas semelhantes ocorreram em EC-04. D=0..127, V=128..255, F=256..511, com conjuntos disjuntos; F não contém vetores `u8` de diagnóstico ou R1. Em cada partição, aquisição A tem 24, revisão A 24, avaliação 24 e B oito vetores adicionais, todos disjuntos. F contém **oito testemunhas discriminantes na revisão e oito na avaliação**, além de **16 invariantes** em cada conjunto (oito positivas, oito negativas). Pré-verificação automática confirmou propriedades e exclusão antes de F; autor conhecia regras e estratificação, portanto não há cegueira de autoria.

D/V e preflight ocorreram antes do primeiro F. [Protocolo congelado](protocolos/EC-10_R2_testemunhas_revisao.md) SHA-256 `7de586a64c5a99fada79da6853b14367f349c077a3fc031331af43d6e5e2a6f2`; teste/gerador/referência `577edf39909ad0bf646ecf0cf92e750a65c4bf5741793d7c89b2589bb3a78c41`, conferidos após F. Esses hashes não provam pré-registro independente. Repetições usam mesmos exemplos; não são amostras independentes.

## Resultados por repetição

Quatro fases × 24 testemunhas × seis condições pontuadas × duas repetições = **1.152 decisões pontuadas**; o candidato renomeado foi consultado em mais 192 avaliações pareadas. As referências antigas e revisadas são ambas registradas nos agregados; formato abaixo acertos/erros/abstenções, por repetição.

| Fase | Classe | Candidato e controle de sufixo | Congelado | Prática com rótulo antigo | Literal e sem aquisição |
|---|---|---:|---:|---:|---:|
| Antes de A | Dsc, Inv+, Inv− | cada 0/0/8 | cada 0/0/8 | cada 0/0/8 | cada 0/0/8 |
| Após A | Dsc | 0/0/8 | 0/0/8* | 0/0/8 | 0/0/8 |
| Após A | Inv+ | 0/0/8 | 0/0/8* | 0/0/8 | 0/0/8 |
| Após A | Inv− | 8/0/0 | 0/0/8* | 8/0/0 | 0/0/8 |
| Após intervalo e B | Dsc e Inv+ | cada 0/0/8 | cada 0/0/8 | cada 0/0/8 | cada 0/0/8 |
| Após intervalo e B | Inv− | 8/0/0 | 8/0/0 | 8/0/0 | 0/0/8 |
| Após revisão, referência revisada | Dsc | **8/0/0** | 0/0/8 | 0/0/8 | 0/0/8 |
| Após revisão, referência revisada | Inv+ | **8/0/0** | 0/0/8 | 0/0/8 | 0/0/8 |
| Após revisão, referência revisada | Inv− | **8/0/0** | 8/0/0 | 8/0/0 | 0/0/8 |

*O congelado é criado após B; antes disso é estado vazio, não comparador pós-treino. Após revisão, os oito Dsc têm resultado negativo correto pelo critério novo, mas são divergências frente ao critério antigo por definição; eram ambíguos antes, não oito respostas positivas anteriormente corretas revertidas. Inv− preservou 8/8 respostas previamente corretas; Inv+ mudou de oito abstenções para oito respostas positivas corretas, portanto não se pode dizer que 16/16 decisões invariantes já corretas foram preservadas. Não houve erro nas respostas emitidas frente à referência vigente no painel; 16 abstenções iniciais não são acertos. Após B, **24/24 decisões** permaneceram iguais (incluindo 16 abstenções); retenção de respostas previamente corretas é **8/8**.

A versão A evoluiu de 24 episódios/quatro hipóteses para 48 episódios/início de sufixo 8/uma hipótese; B tem oito episódios em contexto separado. O controle de prática recebeu os mesmos vetores adicionais com rótulos antigos e permaneceu ambíguo nos oito Dsc e oito Inv+; congelado não recebeu revisão. Essas condições diferem nos rótulos/experiências, como necessário ao contraste, e não sustentam superioridade geral. O literal só verifica ausência dos vetores reservados no conjunto apresentado; não testa sua recuperação positiva. A redução de hipóteses e decisões após rótulos revisados é evidência de atualização simbólica condicionada ao ensaio, não prova de aquisição neural ou significado.

## Renomeação, controles e não conformidades

Contextos 7→70, 8→80 e IDs de episódios +1000 foram invertidos e comparados no estado integral da **extensão experimental**, incluindo logs e regras. Em 192 consultas renomeadas, decisões e contagens de regras examinadas coincidiram. Isso não mede proveniência interna de produção, tempo operacional equivalente, ou transferência. O controle de sufixo coincidiu nas decisões e estados da extensão, mas **compartilha função de enumeração `rules` e função `query` com o candidato**; portanto a independência de implementação é parcial, apesar da busca de sufixo distinta. Esta limitação impede considerar satisfeita integralmente a exigência de controle simbólico independente. Não alterar código após F.

Contextos A/B possuem estados separados: coexistência não prova resistência à interferência em parâmetros compartilhados. A revisão é deliberada, não interferência espontânea. Os intervalos efetivamente medidos foram 25,061 e 25,108 ms; não houve reinício ou persistência em disco. Retenção após reinício, transferência entre tarefas/domínios e cumulatividade permanecem não demonstradas. A avaliação usa função de referência separada da indução, mas sem revisão externa independente; não autentica verdade de domínio.

## Verificações e recursos

Pré-verificação, desenvolvimento e validação passaram antes de F. `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check` passaram: **174 testes aprovados**, zero falhos/ignorados, incluindo duplicações históricas por inclusão. Três avisos anteriores de métodos EC-04 não utilizados. Painel isolado 0,350976 s de parede e pico `wait4` 7.380 KiB, podendo incluir base do lançador. Guardas internas de tempo <30 s, RSS <256 MiB e /home >=3 GiB passaram; espaço observado inicialmente 4.620.484.608 bytes e após F 4.618.842.112 bytes, não monitoramento contínuo. Limite fino de 256 KiB de dados por condição não medido integralmente. Contagem de regras consultadas não inclui construção das regras, busca de sufixos, alocações ou CPU; não é custo total equiparável. Registros agregados em `/tmp/neurons_ec10_r2_final.log` não são custódia independente.

A1–A8, arquitetura e histórico preservados. Comparado com R1, R2 contém testemunhas discriminantes e rótulos efetivamente contraditórios, mas controle independente completo e reavaliação de respostas anteriormente corretas permanecem pendentes. EC-10 continua aberta; não iniciar próxima etapa.
