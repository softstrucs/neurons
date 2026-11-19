# NEURONS

Núcleo experimental em Rust para neurônios, sinapses, engramas e evolução cognitiva verificável.

**Autoridade científica:** [NEURONS_PLANO_MESTRE.md](NEURONS_PLANO_MESTRE.md).
Execução funcional PIFI-001 autorizada pelo responsável, com [registro operacional](NEURONS_PIFI_001_PROGRESSO.md).
Essa autorização não libera protocolos novos, dados reservados ou publicação Git.

- [Documentação e referências](docs/README.md).
- [Planos executivos históricos, preservados](docs/historico/planos_executivos/README.md).
- [Relatório de recuperação e limites](docs/referencia/RECUPERACAO_ESTRUTURAL.md).

Pacote local sem dependências externas de crates: `cargo check --offline --lib --bin neurons`.
Testes/painéis históricos e dados reservados exigem escopo próprio; não executar `cargo test`
indiscriminadamente: há fixtures reservadas em alvos científicos. A CLI oferece `--help`,
`doctor`, `session`, `session-explain` e **`session-f1`**.

A implementação técnica F1 integra os coordenadores existentes de consulta, aquisição explícita
e revisão conceitual H23. Inicie `cargo run --offline --locked --bin neurons -- session-f1` e
use `ajuda`. Gramática estruturada, fontes sintéticas declaradas e limites finitos; sem linguagem
natural, rede ou compreensão geral demonstrada. Os comandos anteriores mantêm seu comportamento.

`salvar CAMINHO` cria exclusivamente um checkpoint F1 v1; nunca substitui arquivo existente.
`restaurar CAMINHO` restaura fatos, proveniência, pendências, hipóteses, versões e histórico.
Circuitos, pesos e potenciais neuronais não fazem parte desse contrato. Detalhes, comandos de
regressão seguros, limitações e evidências estão no [Plano Mestre](NEURONS_PLANO_MESTRE.md#9-registro-de-progresso-e-próxima-ação).
F1/F2 e F3-01 possuem aceites delimitados com ressalvas registrados no Plano Mestre.
O ciclo experiencial adicional integra diagnóstico, fonte didática local, revisão H23 e
reconsulta verificada; não demonstra plasticidade neural causal ou compreensão geral.

Regressões públicas selecionadas: `python3 scripts/test_public.py --build`.
O executor usa modo offline, dependências bloqueadas e target externo em `/tmp`,
seleciona alvos explícitos e exclui os dois probes reservados. `--list` mostra a seleção.
Os cinco arquivos locais `src/{integration/{query,acquisition,concepts},protocol_verification{,/conceptual}}/tests.rs`
não rastreados pertencem à fronteira científica histórica; não são requisitos para
compilar lib/CLI nem autorização para executar testes internos. Não removê-los.

O módulo `integration::development::Runtime` adiciona inspeção do circuito real,
sensores sintéticos tipados, atenção limitada, experiências rastreáveis e portas
para MP05, MP09, MP07/MP08 e verificação F3-01. Uso e controles executáveis:
`python3 scripts/test_public.py --target development_runtime`.
Sensores escalares não são interpretados automaticamente como relações.

Os checkpoints operacionais `NRNPIFI1/2` encapsulam os codecs F1 v1/MP05 v1 intactos,
observações, memória e consumo de recursos. `Runtime::restore` exige reinstalação
explícita das interfaces e novas permissões. Episódios consumidos não são reabertos;
relatórios transitórios não são restaurados como autoridade. O checksum detecta
corrupção acidental, sem autenticar origem. Esse recorte não encerra E00–E12 nem
constitui validação científica de vantagem neural, causalidade ou generalização.
Os campos históricos de configuração/concessão dos codecs internos são preservados;
eles não substituem as novas permissões do runtime.

`Runtime::intervene` registra observações antes/depois de uma ação sintética,
com concessão vinculada a mundo, versão, contexto, fonte e ação. `Runtime::plan`
usa apenas transições observadas; `Runtime::pursue` liga lacunas, perguntas,
controles disponíveis, exploração autorizada e reconsulta do objetivo original.
A tabela de consequências do ambiente não é entrada do planejador.
`Runtime::predict` apresenta resultados observados, referências e conflitos,
sem votação por maioria ou promoção a lei causal.

São no máximo quatro tentativas de ação, oito experiências perceptivas MP05,
quatro consultas de modelo e 32 inspeções de planejamento por instância.
A seleção de controles compartilha o orçamento atencional existente.
Falhas conservam concessões consumidas/reservas; previsões divergentes ficam
no histórico e suspendem o plano antigo antes de qualquer revisão explícita.
O envelope v2 retém esse modelo e seus limites; imagens v1 continuam canônicas,
sem migração silenciosa. Planos ativos, concessões e o ambiente externo não são
restaurados automaticamente. Testes: `python3 scripts/test_public.py --target development_actions`.

`Runtime::observe_attributes` vincula seis atributos explícitos ao estado atual;
`teach_consequences` deriva exemplos somente de intervenções já observadas e
reutiliza diagnóstico, seleção informativa e revisão MP09/H23. O conceito possui
um significado operacional declarado (ação/destino/contexto), sem regra-alvo
fornecida ao aprendiz. `forecast_consequence` reúne H23 e o modelo de transições;
`execute_forecast` verifica o vínculo/versão, aplica uma concessão fresca e confere
a consequência. Conflitos conhecidos suspendem a aplicação; abstenção e divergência
produzem pedido de revisão, sem apagar evidências ou admitir leis/fatos derivados.

O envelope operacional `NRNPIFI3` acrescenta somente esse vínculo semântico a uma
base v1/v2 intacta. Não restaura previsão ativa, permissões ou direito de reexecução.
Relatórios de comparação são transitórios; observações, transições e versões H23
são retidas. Uma nova instância explicitamente autorizada pode herdar o contrato
e continuar a memória conceitual; isso não renova recursos da instância anterior.
Os atributos ainda vêm de sensores sintéticos declarados. Não se demonstra descoberta
aberta de atributos, causalidade neural ou conclusão integral de E04–E06.
Testes: `python3 scripts/test_public.py --target development_consequences`.

`Runtime::pursue_with_consequences` consulta outro Runtime como memória imutável
de um episódio anterior e reutiliza o mesmo planejador para comparar uma rota
observada com um prefixo H23 seguido de caminho observado. Escolhe entre as duas
alternativas elegíveis por permissão, reserva e comprimento; empate favorece a
rota observada. Não enumera todos os planos possíveis. A consequência real pode
motivar uma reconsulta e um novo plano para o objetivo original.

`pursue_learning_goal` permite, opcionalmente, uma experiência após diagnóstico
real de insuficiência. Exige `PredictiveRevision` com permissões pedagógicas
independentes; rótulos são derivados da ação observada. MP09 seleciona/incorpora
experiências e revisa H23; a consulta anterior fica histórica. Sem autorização,
não há ensino implícito. Relatórios distinguem aquisição, revisão e objetivo atingido.
Instâncias novas exigem sensores, circuito e autoridade fornecidos explicitamente;
a memória de origem não muda e seus recursos/concessões não são reabertos. Manter
os checkpoints de origem necessários; não há consolidação automática de arquivos
de múltiplos episódios ou retenção permanente dos relatórios transitórios.
Testes: `python3 scripts/test_public.py --target development_predictive`.

`EpisodeArchive` guarda até quatro snapshots operacionais imutáveis. A recuperação
`Runtime::retrieve_context` seleciona transições por mundo, revisão, contexto,
fonte e tempo; mantém origens e versões e compartilha a atenção de 32 inspeções
com as ações. Não funde modelos H23 nem escolhe a versão numericamente mais nova.
Uma consulta incompleta, identidade ambígua ou linhagem conceitual incompatível
não pode sustentar uma decisão. Evidência oposta permanece no modelo existente.
`RetrievedMemory` alimenta as mesmas portas de previsão, planejamento, perguntas
e aprendizagem. Mudanças na sessão exigem nova recuperação.

O contêiner operacional `NRNARC01` encapsula imagens existentes, sem alterar F1,
MP05 ou NRNPIFI1/2/3. `save` cria arquivo novo com sincronização; `load` exige
reinstalação das interfaces e nunca restaura Access. Consumos históricos, inclusive
de episódios não selecionados, impedem reciclar concessões. O checksum detecta
corrupção, não autentica fontes. Ainda há limites de quatro episódios/oito entradas
recuperadas; compartilhamento indevido de evidência ou concessão exige decisão.
Teste público: `python3 scripts/test_public.py --target development_archive`.

IDs locais de ações podem coincidir entre episódios independentes. A recuperação
mantém `RecoveredTransition.trial` e `origins` originais e atribui uma referência
transitória à vista. Suportes de `predict_retrieved` e provas de `plan_retrieved`
são resolvidos por `RetrievedMemory::resolve_reference`; não são novos fatos ou
concessões. Essas referências não são persistidas como planos/autoridade. Episódios
com evidência compartilhada indevidamente continuam bloqueados. Uma nova consulta
ou reinicialização exige resolver suas próprias referências.

`EpisodeCatalog` indexa até 16 segmentos NRNARC01 existentes (cada um continua
limitado a quatro episódios). O custo de construção é separado da consulta.
`select_evidence` procura por contexto e fechamento dos estados do objetivo,
incluindo todas as consequências das ações nesses estados, não só as favoráveis.
O seed conceitual é explícito. Não há novo planejador ou classificador.

`SelectiveSearch::coverage` distingue cobertura do objetivo, estados fechados,
postagens/modelos pendentes e limite atingido. Até quatro snapshots materializados,
oito evidências e 32 inspeções totais da busca, compartilhadas com a atenção da
instância, inclusive em retomadas. `resume_selection` continua uma página pendente;
`extend_selection` pesquisa explicitamente um novo estado, sem reset de consumo.
Uma vista parcial não pode sustentar previsão/plano; uma vista seletiva completa
não autoriza consultar estados ou outro destino fora de sua cobertura. Cada nova
materialização invalida referências transitórias anteriores. O empréstimo do
catálogo impede modificá-lo enquanto a busca/vista estiver em uso.

Teste público: `python3 scripts/test_public.py --target development_selective`.
A referência exaustiva dos testes lê registros brutos e enumera caminhos sem usar
índice/candidato como gabarito. Menos inspeções não implica menor tempo: o cenário
amostral de 40 episódios teve custo de seleção superior ao oráculo simples.

Após revisão de H23 ou descoberta de estado não coberto, refaça/expanda a busca
antes de continuar. `execute_retrieved_plan` reutiliza seleção de concessões e
executor existentes, confere a vista/sessão e oposição das experiências locais
novas, e exige reserva integral. Não reinicia o episódio. A continuação pode
suspender por cobertura, autoridade ou saldo insuficientes; isso não é uma prova
de ausência de solução. Os limites de quatro materializações/32 inspeções podem
impedir consultas mesmo quando o catálogo contém a evidência necessária.

Snapshots byte a byte idênticos compartilham uma imagem imutável no catálogo
(`SnapshotStorage::Coalesced`, padrão); `Separate` permite comparação sob os mesmos
tetos. IDs de episódio, origens, versões, observações e consumos permanecem
rastreáveis. NRNARC01 e os arquivos originais não são compactados ou alterados.
Imagens com versões, metadados ou tombstones diferentes não são fundidas.

`RetrievedMemory::evidence_profile` separa origens de armazenamento, registros, IDs
de evidência, fontes e referências de suporte H23. Essas contagens se sobrepõem e
não devem ser somadas como votos independentes. IDs distintos não certificam
independência causal; observações diferentes da mesma fonte continuam com uma
única fonte. Réplicas não completam uma premissa ausente nem superam oposição.

Teste público: `python3 scripts/test_public.py --target development_coalescence`.
Comparação sintética com quatro réplicas: payload retido 1768 contra 7072 bytes;
capacidade dos buffers 3424 contra 13696; postagens 2 contra 8; inspeções de índice
4 contra 7 e materialização 3 contra 12. Não é medição da RAM total. Amostra debug:
construção 185465/132440 ns e consulta 414626/433611 ns (coalescido/separado).
Tempos pontuais sem inferência de ganho de velocidade. Com 64 réplicas, a busca
coalescida completa usando uma imagem; o controle separado atinge quatro slots e
suspende, sem produzir conclusão definitiva ou ampliar limites.

Sobreposição parcial também consolida postagens de transições integralmente iguais
(ActionTrial e ambas as Reading), mantendo todas as origens. Igualdade de valores
sem igualdade de identidade/procedência não basta. Observações exclusivas, versões
e consumos de cada snapshot permanecem separados. O índice é reconstruído na
inserção, com custo de construção explícito; não há alegação de ganho de latência.

`pursue_learning_goal` também pode aproveitar o feedback de uma rota observada:
a conferência usa a reserva existente e, com concessão pedagógica explícita, MP09
pode reduzir incerteza H23 mesmo quando o objetivo já foi atingido. O relatório
mantém resultado da execução separado da incorporação. Revisão invalida vistas
antigas; não há reexecução automática após mudar o modelo. Contraexemplo que
refuta toda a família atual permanece como experiência e não é forçado a entrar
no conceito. A recuperação dessa família continua uma limitação explícita.

`assess_epistemic` reconstrói limitações a partir dos atributos, ações e
consequências persistidos em NRNPIFI3/NRNARC01. Não modifica codecs nem admite
fatos. `EpistemicDiagnosis` distingue insuficiência, candidato refutado, conflito
de observações e limitação expressiva relativa à base H23 admitida. O relatório
conserva a versão da sessão no snapshot, versão avaliada, contexto e evidências;
não afirma que o relatório histórico da previsão tenha sido serializado.

A recuperação seletiva inclui amostras conceituais relevantes ao contexto mesmo
fora do fechamento dos estados do objetivo, sob os mesmos quatro slots/oito
evidências/32 inspeções. A avaliação cobra atenção por registro (até oito) e
reutiliza até 23 avaliações de critérios H23. Cobertura incompleta suspende a
aplicação. No ciclo integrado, atributos completos iguais com contraexemplo
relevante bloqueiam a ação automática; relatórios H23 anteriores permanecem
subordinados à limitação. Questões indicam investigação de fontes/contexto, busca
de evidência independente ou revisão representacional autorizada. Não conferem
autoridade. Os testes públicos estão em `development_epistemic`.

`revise_recovered_counterexample` permite encaminhar uma experiência recuperada
admissível a MP09 com novas permissões de avaliação, apresentação e admissão.
Reutiliza a mesma reserva de cinco consultas e o mesmo episódio de ensino; não
repete a ação nem aceita rótulo fornecido pelo chamador. Limitação expressiva e
conflito não geram incorporação forçada. A revisão torna a vista anterior obsoleta.

O ciclo limitado também pode escolher, pelo BFS existente, uma rota inteiramente
observada que exclua a ação ligada à hipótese refutada. O objetivo pode ser
atingido sem resolver a limitação. Permissões, oposição e reserva integral
continuam obrigatórias; divergência da rota interrompe a execução.

`investigate_disagreement` transforma dúvida/conflito em uma experiência
conceitual limitada. Compara previsões dos critérios H23 elegíveis em até quatro
ofertas de atributos atuais declarados por sensores autorizados. Escolhe a
separação mais equilibrada, com desempate por ID; não recebe rótulos ou destinos
futuros do ambiente. Se não houver discriminação/autoridade/reserva, não age.

A experiência selecionada reutiliza observe_attributes, forecast_using, intervene,
review_consequence e o currículo MP09. Retidos/refutados no relatório são
diagnósticos transitórios; somente a incorporação MP09 altera o modelo. Uma
revisão invalida vistas anteriores, e o conflito histórico permanece registrado.
Até uma intervenção, três percepções, reserva neural 48, duas consultas de
previsão/conferência e cinco de ensino opcional, nos tetos existentes. Novos
critérios, causas e independência de fontes não são inventados. MP07 permanece a
porta de investigação relacional; a experiência conceitual não admite relações.
Teste: `python3 scripts/test_public.py --target development_discrimination`.

A continuação pública em `development_discrimination` liga investigação,
recuperação multiepisódio e planejamento: sem investigação, a meta inédita fica
insuficiente; com revisão admitida, um prefixo conceitual e uma transição observada
atingem a meta em duas ações. Um controle de transferência incorreta preserva a
previsão refutada e interrompe a execução; o contraexemplo é recuperável como
limitação expressiva. Esses cenários não demonstram generalização ampla nem
resolução do conflito histórico entre observações.

Experimentos de fonte/contexto: `development_source_context` distingue escopo
conceitual de procedência. H23 mantém modelos por Scope e aceita evidências
consistentes de fontes conhecidas distintas no mesmo escopo; SourceId não integra
os seis atributos da hipótese. Não foi demonstrado aprendizado de regras
condicionadas à fonte apenas por sua identidade. F1 preserva modelos de múltiplos
contextos; a ponte pedagógica MP09/Runtime continua exigindo um contexto por sessão.

Memória conceitual de Runtime agora valida identidade, escopo, fonte, mundo, revisão
e validade antes de herdar o contrato ou registrar atributos. O defeito anterior
já impedia a ação, mas deixava uma observação e um vínculo incompatível na instância
receptora. A correção evita essa mutação parcial, sem alterar critérios H23,
consulta científica ou formatos de checkpoint.

Quando uma “fonte” é o objeto cujo comportamento se observa, a condição usa
`World.id`/`Reading.item` (entidade), separadamente de `Reading.source` (informante).
`EntityCondition` explicita essa entidade opcional, Scope, revisão, ação/destino e
informante declarado. `recover_conditioned_knowledge` recupera a Session pareada
com o Runtime arquivado e compartilha a prova de ancestralidade da recuperação
anterior. Não deduz entidade de SourceId nem classifica confiabilidade por ID.

Identidade ausente/desconhecida, ramos incompatíveis, evidências contrárias ou
busca incompleta não liberam Session. Compatibilidade sem suportes ancorados
retorna UnprovenBinding. Evidências contrárias de outros informantes sobre a mesma
entidade/condição também são preservadas. A compatibilidade usa experiências já
registradas e Criterion::evaluate; não revisa H23 nem cria outra família. A sessão
retornada é imutável e histórica (`as_of`), com origens e suportes ligados às
observações; requer permissão de leitura, sem conceder autoridade de ação. Tetos: quatro candidatos,
oito registros e 32 inspeções de atenção compartilhada; nenhuma autoridade ativa
é restaurada. Ready significa estado pareado recuperado, não verdade universal.

A interface bruta que recebe Runtime e Session separadamente continua exigindo
pareamento correto pelo chamador. Sua limitação fica como controle negativo nos
testes: sessão alheia pode motivar uma ação refutada depois; o caminho condicionado
recupera o modelo correto e evita essa ação no cenário avaliado. Não há prova de
autenticidade da identidade física nem descoberta autônoma de condições por entidade.
Teste público: `python3 scripts/test_public.py --target development_entity_conditions`.

### Sondas discriminantes no fluxo operacional PIFI

`Runtime::select_discriminating_probes` propõe duas combinações hipotéticas de
atributos, sem rótulos, a partir da discordância H23 atual. Exclui atributos das
experiências já admitidas e das ofertas de aquisição. Proposta não é observação,
fato, permissão de intervenção ou autorização pedagógica.

`Runtime::investigate_with_selected_probes` constrói a avaliação sem receber
`PedagogyAssessment` do harness e delega ao investigador existente. Recebe ofertas
reais e permissões explícitas; `ProbeTeaching` contém somente registro, recibo e
concessões pedagógicas independentes. Sem essa concessão, uma observação pode
filtrar alternativas provisórias, mas não alterar o conhecimento admitido.

`ProbeBudget` limita a busca a até 32 padrões e 736 avaliações de critérios. Cada
padrão inspecionado consome o teto compartilhado de atenção (32), inclusive quando
excluído. A ordem é determinística, de 63 a 0; `Ready` significa duas sondas úteis,
sem garantir cobertura exaustiva ou seleção ótima. `ResourceLimited` impede a
investigação automática, inclusive quando uma única sonda já foi encontrada.
O relatório informa padrões/avaliações/exclusões, versão e cobertura, além do
diagnóstico epistemológico. `NoDisagreement` não comprova verdade: uma hipótese
única ainda pode ser refutada por experiências posteriores.

As sondas são de avaliação; experiências executáveis continuam dependendo dos
sensores e ofertas do ambiente. O prefixo pode não alcançar padrões úteis dentro
do teto: o controle de eixos (5,0) suspende a busca em 32 inspeções, sem ação ou
aprendizagem. Não há nova família H23, alteração de codec científico, autoridade restaurada,
conhecimento de destinos futuros ou demonstração de contribuição causal neural.

### Viabilidade e continuação explícita da investigação

`ProbeSearch::new` vincula o cursor ao contrato, imagem completa da Session e
atributos das ofertas excluídos da avaliação. `continue_probe_search` preserva
padrões examinados, propostas parciais e custos acumulados. O orçamento de cada
rodada continua limitado a 32 padrões/736 avaliações e à atenção restante no
Runtime. Não há recarga automática: um Runtime com 32 inspeções consumidas segue
esgotado, inclusive após restauração. Uma nova instância e seu acesso/orçamento
precisam ser fornecidos explicitamente pelo chamador.

`investigate_continued_probes` revalida os observáveis das ofertas antes de mover
o cursor. `ProbeFeasibility` separa informação discriminante, controle habilitado
no simulador (`realizable`), permissão atual (`authorized`) e execução efetiva
(relatório da investigação). A habilitação não substitui reservas de execução.
Correspondência exata de atributos e alternativa com a mesma separação prevista
são relatadas separadamente; esta última não comprova equivalência física.
Uma sonda hipotética sem oferta não é fabricada nem executada implicitamente.

Se a identidade, contexto, versão, evidência admitida ou conjunto de atributos
ofertados mudar, a continuação é recusada sem alterar o cursor. Essa comparação é
conservadora: qualquer alteração nos bytes da Session exige nova busca.
`dismiss_batch` retira um lote de propostas, preserva seu histórico e permite
percorrer o restante; não apaga evidências nem concede autoridade. `Exhausted`
descreve esgotamento dos candidatos, não conclusão sobre o objetivo.

`ProbeSearch::encode/restore` usa o envelope administrativo local `NRNPRB01`
(até 41.024 bytes), separado de F1/MP05/NRNPIFI/NRNARC, sem carregar concessões ou
restaurar Session/World. Verifica vínculo, contabilidade e checksum; este detecta
corrupção acidental, não autentica arquivos hostis. A validação dos candidatos
armazenados tem até 64×23 avaliações, reportadas por `integrity_evaluations`,
separadas dos custos ativos/cumulativos da busca e sem eventos neurais ou ações.
Os testes exercitam continuação 32+3, resto de atenção preservado, recusa de
proveniência/versão obsoleta, ofertas negadas/inviáveis, exaustão e novo processo.

### Evidências empíricas e limites da equivalência informacional

`informational_alternatives` significa igualdade do vetor de previsões dos
critérios atuais, não igualdade de efeitos físicos. Consequências observadas
continuam registradas integralmente: 77 e 88, por exemplo, são efeitos distintos,
ainda que ambos sejam negativos para o destino 20.

O diagnóstico existente agora reconhece duas experiências com atributos
distintos, vinculadas à mesma versão completa no snapshot, previsões iguais e
rótulos observados opostos. Nenhum critério dessa versão acomoda conjuntamente
essas testemunhas: retorna `ModelExpressiveness` e preserva origens/evidências.
A regra não altera H23, não admite lições e não reinterpreta automaticamente
conflitos de outras versões. `recorded_version` é a versão do snapshot; não é um
relatório histórico de previsão supostamente persistido.

`ProbeFeasibility.epistemic` expõe o diagnóstico. `complete` indica cobertura da
verificação, não aprovação da equivalência. Versão refutada não libera
correspondências operacionais; a investigação continuada suspende o cursor antes
da execução. O planejador existente evita transferência pelo conceito refutado,
conservando suas regras para alternativas inteiramente observadas.

São até oito registros/32 inspeções compartilhadas, até 23×9 avaliações de
critérios no diagnóstico e 28 comparações de assinaturas. O custo do diagnóstico
aparece em seu relatório, separado das avaliações de ofertas; nenhuma reserva
neural ou permissão é ampliada. Insuficiência/cobertura incompleta mantém
abstenção ou investigação explicitamente autorizada, não certificação física.
Os controles incluem consistência, divergência, ablação das evidências com a
mesma representação de memória, revisão autorizada, cursor obsoleto e novo processo.


### Investigação explicativa e preparação do MVP local

Os controles públicos compõem as APIs existentes de recuperação, H23/MP09,
seleção de sondas, investigação e planejamento, sem alterar o núcleo científico.
Uma condição já observável, mas ausente do candidato original, explica a
divergência no recorte sintético: seis critérios passam a três, dois e finalmente
`A∧C`, mediante experiências e concessões explícitas. Cinco exemplos de aquisição
são separados de 59 padrões adicionais verificados por referência independente.
O aprendizado permite atingir uma meta em estado inédito e é recuperado em outro
processo; o contraexemplo original mantém procedência e versão histórica.

Sem experiência explicativa, a meta permanece insuficientemente sustentada.
Ofertas com correlação espúria não resolvem a ambiguidade; condição ausente,
variável não observável ou critério já eliminado preservam abstenção, conflito ou
limitação expressiva. Trocar as hipóteses adquiridas, mantendo ofertas e destinos
privados iguais, muda a seleção de 741 para 742. Isso demonstra dependência
funcional computacional no controle, não causalidade física ou neural.

| Capacidade e componentes | Integração demonstrada e limite |
|---|---|
| EDA, Circuit e Runtime | Observação/atenção/ações sintéticas; não percepção aberta de sensores reais. |
| Engramas, Session, archive/selective | Proveniência, versões, recuperação e restauração; ciclo de vida ainda coordenado pelo hospedeiro. |
| H23, MP09 e consequences | Aprendizagem/revisão conceitual supervisionada; família fixa, sem descoberta de novas variáveis. |
| query, investigation, composed/composed_two | Inferência verificada e composição delimitada; não investigação multidomínio autônoma. |
| discrimination/search, epistemic | Perguntas, sondas sem rótulo, continuação e abstenção; ofertas estruturadas fornecidas pelo ambiente. |
| actions e predictive | Previsão e planejamento limitado com autorização; testados em mundos sintéticos. |
| learning e synaptic/MP05 | Plasticidade e transmissão testadas isoladamente; contribuição causal ao ciclo decisório integrado não demonstrada. |
| languages, session e main | Léxico e CLI restrita; sem compreensão linguística aberta nem entrada operacional do Runtime/PIFI. |

Para um MVP **local, supervisionado e sintético**, a próxima dependência é um
adaptador mínimo do Runtime à CLI existente, reutilizando o coordenador. Deve
explicitar objetivo, episódios, orçamento, concessões novas e atualização das
vistas após revisão. A recuperação deve coordenar Session, Runtime, arquivo e
cursor, além de revalidar o estado ambiental fornecido pelo hospedeiro, sem
restaurar autoridade. Falta demonstrar esse fluxo pelo binário real, incluindo
interrupção, novo processo, controles negativos e medições de CPU/RAM.

Esses são bloqueadores operacionais desse MVP delimitado. Contribuição neural
causal, causas físicas, novas famílias, generalização ampla e linguagem aberta
continuam questões científicas. A causalidade neural seria também bloqueadora de
qualquer MVP que a alegasse. E02–E06 permanecem parciais. Nenhuma integração CLI
nova foi implementada nesta investigação.
