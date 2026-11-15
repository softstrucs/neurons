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
