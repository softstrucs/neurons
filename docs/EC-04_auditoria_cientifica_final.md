# EC-04 — Avaliação científica e auditoria final

## 1. Parecer e estado verificado

**Conclusão: AJUSTAR. Conformidade experimental parcial.** Recomenda-se encerrar a EC-04 como investigação delimitada, preservando os resultados e as não conformidades abaixo. Não homologar êxito integral do protocolo: aprovação funcional não supre requisitos ausentes de controle, proveniência e instrumentação. Não há impedimento para registrar esse encerramento limitado; há impedimento para usar a EC-04 como comprovação de conformidade integral ou das competências não testadas. A decisão de avanço continua humana; EC-05 não foi iniciada.

Base auditada: `fa2ed059dd3c57bcb79995557211c7b319ff9091`, branch `devs`. HEAD, origin/devs e refs/heads/devs remota coincidiram; árvore inicialmente limpa. Esta delegação autoriza somente alterações documentais e reprodução de verificações existentes. Nenhuma correção foi aplicada ao experimento, aos protocolos ou às expectativas.

A reprodução confirmou 132 testes aprovados, 14.560 consultas finais, 3.120 validações associadas e 280 eventos de circuito. As 288 linhas detalhadas de resultados e os 54 hashes de partições coincidem integralmente com o relatório da implementação. Esses fatos preservam as evidências positivas; não apagam as divergências estáticas.

## 2. Fontes e método

Lidos o plano cognitivo v0.2 integral, protocolo EC-04 §§1–14, revisão R1 integral, tarefas T-R/T-C/T-A/T-V, auditoria de relações existentes e relatório de implementação (incluindo matrizes/hashes). Código: tests/ec04_generator_preflight.rs, tests/ec04_learning.rs, tests/ec04_panel.rs e os cinco suportes ec04_candidate, ec04_controls, ec04_generator, ec04_independent e ec04_types. Consultado também o trecho produtor de Proof em src/cognition.rs:200–262.

Foram diferenciadas análise estática, reprodução atual e alegações históricas. Antes da execução, registraram-se SHA-256 dos 14 arquivos de referência/código em inventário temporário externo ao repositório. Todos permaneceram iguais após a reprodução. Os dez hashes publicados no relatório anterior, incluindo os dois protocolos, também conferiram. Nenhum teste novo, nova condição, novo dado, novo mecanismo ou ajuste foi executado.

Os cabeçalhos históricos do plano v0.2 e dos documentos de preparação registram sua situação na redação original; não são usados para negar autorizações posteriores nem foram atualizados retroativamente.

## 3. Matriz de conformidade

“Conforme” aplica-se ao requisito e painel indicados, não a competência geral. “Não verificável” não significa resultado negativo. Classificações parciais podem conter subitens não conformes explicitados na seção seguinte.

| Requisito congelado | Classificação | Evidência e alcance |
|---|---|---|
| Protocolo §§1/14: hipótese e separação de capacidades | Conforme | Recuperação/composição/indução/revisão separadas no desenho e no relatório; T-A/T-V são simbólicas. |
| §2: família H de 576 e oito propriedades | Conforme | ec04_candidate.rs:15–33,74–96; ec04_independent.rs:25–52; enumerações próprias da mesma família. |
| §2: Query sem alvo/índice de exemplo | Conforme | ec04_types.rs:29–33; ec04_panel.rs:210–234; pontuação ocorre depois de query. |
| §2/6: contrato completo T-R com contexto, versão, conflito e aquisição atômica | Parcialmente conforme | ec04_panel.rs:435–490 reproduz pares e ausências, mas não implementa API/validação do lote, contexto 4101/4102/4103 ou versão 1. NC-02. |
| §3: revisão pelo maior sufixo consistente | Conforme | Candidato parte do sufixo anterior e descarta prefixos; independente procura desde o início. Adicionar restrições não pode tornar consistente um sufixo anteriormente inconsistente; essa otimização preserva a regra. Resultados coincidem. |
| §3: publicação atômica e log conservado | Parcialmente conforme | Candidato trabalha em clone; independente publica após calcular sufixo; nenhuma exclusão física de episódios. Falhas/orçamento não foram exercitados exaustivamente nos testes existentes. |
| §3: rejeição de episódios avaliativos | Parcialmente conforme | Flag acquisition validada nos três executores; harness oferece somente fatias de aquisição. Não existe autorização não falsificável ligada ao manifesto nem teste adversarial completo; o chamador é confiável no painel. |
| §3: proveniência consultável por versão/família | Não conforme | Answer guarda contagem/índices, sem versão da família e sem interface de recuperação histórica de estado. NC-03. |
| §3: abstinência/estado inválido explícitos | Parcialmente conforme | Missing/Ambiguous existem; o controle independente não trata H vazio como Inconclusive. Estado vazio inválido não foi alcançado pelo painel. NC-08. |
| §§4/5: alvos D/V/F e partições por contexto | Conforme | targets no avaliador; 32/16/32, revisão 16/8/16 e retenção B como subconjunto. Novidade é contextual, não global. |
| §5/R1: slots, domínios, pares, testemunhas e ordem | Conforme | ec04_generator.rs:31–118,123–193,510–605; 12 construções repetidas com resultados iguais. |
| §5: unicidade e identificabilidade | Conforme | Preflight confere unicidade, disjunção, domínios e um critério consistente na aquisição inicial/revisada. Não entrega esse critério ao modelo. |
| R1: expansões/retrocesso e teto de 2.000.000 | Conforme | Contador incrementado antes de cada tentativa efetiva, inclusive forçada/rejeitada, não reinicia no retrocesso; máximo observado 1853. Ramos de esgotamento/teto identificados estaticamente, sem testes dinâmicos novos. |
| §§5/13: hashes/custódia atual | Conforme | 54/54 hashes de partições iguais ao arquivo versionado; código/protocolos íntegros. Manifestos canônicos são bytes em memória, não arquivos de corpus externo. |
| §13: congelamento anterior ao primeiro F histórico | Não verificável | Relatório e conversa registram congelamento; commit conjunto posterior e hashes atuais não provam sozinhos ordem histórica ou pré-registro independente. Sem indício de mudança nesta auditoria. |
| §§6/7: desempenho T-R e T-C | Conforme | Saídas atuais reproduzem 16/16 e 32/32 nos mecanismos previstos, com ressalva de validação das provas NC-01. |
| §7: semântica/provas T-C fiscalizadas pelo avaliador | Parcialmente conforme | IDs de premissas são guardados e produção os constrói por dois fatos correspondentes; proof_valid verifica apenas Some. Sem comparação dinâmica das premissas entre controles. NC-01. |
| §8: controle literal T-C | Não conforme | Condição 1 cai em _ => None, sem consulta direta de aresta. Coincide com ausência nas consultas congeladas, mas o mecanismo descrito está ausente. NC-01. |
| §8: indução simbólica independente | Conforme | Arquivo separado, enumeração reversa/sufixos própria, imports só de DTOs. Mesmos episódios/família e limites nominais. Independência de pressupostos/equipe não demonstrada. |
| §8: literal conceitual/respostas fixas/ablação de aquisição | Conforme | ec04_controls.rs:49–125; igualdade exata, +p7 fixo, respostas triviais; não acessam professor. Sem aquisição recebe chamada e descarta o payload: ablação de conservação/uso, não isolamento do recebimento. |
| §8: ramo T-V sem atualização | Parcialmente conforme | Copia estado treinado, guarda revisão sem usá-la e preserva 32 erros agregados. Harness retreina candidato auxiliar em vez de copiar o candidato de outra condição: oportunidades extras NC-05. |
| §9: aquisição separada de avaliação e não mutação | Conforme | Fatias disjuntas; ask recebe &Control, compara clone antes/depois; consulta não alimenta Acquire. T-R/T-C só leem estados locais após aquisição. |
| §9: ordem de duas repetições completas | Parcialmente conforme | Executa a/b consecutivamente por condição, em vez de repetir toda a sequência global; estado independente mantém resultados comparáveis. NC-05. |
| §10: quantidade de consultas/eventos | Conforme | 14560/3120/280 conferidos por assertiva; unidade é chamada incluindo condições/repetições, não caso independente. |
| §11: limiares por bloco e conservação B | Conforme | ec04_panel.rs:389–392,700–735; todos os oito contextos e quatro revisões atingem previsões, com igualdade das referências compactas de B. |
| §11: métricas completas e motivos de abstenção | Parcialmente conforme | Classes/acertos publicados para T-A/T-V; motivos são colapsados em -1 e erros não são impressos por classe. T-R/T-C imprimem apenas acerto total; validações associadas não têm tabela de desempenho própria. NC-04. |
| §11: falsificação automática de T-R/T-C | Parcialmente conforme | run imprime acertos e compara repetição, mas não exige 16/16 ou 32/32 por assertiva. Foram conferidos nesta auditoria, sem alterar os testes. NC-01. |
| §12: contagem uniforme de trabalho | Não conforme | Visitas/cópias/comparações não são integralmente contadas e Err perde Work; fórmulas diferem. NC-06. Não foi constatado estouro real de trabalho. |
| §12: dados vivos/fixtures até 256 KiB | Parcialmente conforme | Capacidade de rows mais reservas fixas de 64/96 KiB, sem contabilidade integral de alocações coexistentes. NC-07; nenhum excesso medido ou demonstrado. |
| §12: tempo, RSS isolado e disco nesta reprodução | Conforme | Tempo <30 s, wait4 reporta 8300 KiB no escopo descrito, /home >3 GiB, guardas não acionadas. Não equivale a certificação de todas as alocações. |
| §13: repetibilidade integral das evidências | Parcialmente conforme | Rows são iguais nas duas repetições; não guardam conjunto H completo, motivos, Work ou snapshots de cada versão. NC-04. |
| §14: limites de interpretação | Conforme | Evidências simbólicas delimitadas; nenhuma formação semântica autônoma, transferência estrutural ou superioridade neural sustentada. |

## 4. Divergências e consequências

### NC-01 — T-C literal e fiscalização de prova; gravidade metodológica alta

O protocolo §§7/8 exige consulta direta literal e provas válidas. Em tests/ec04_panel.rs:531–576 só condições 0/4/5 calculam provas; 1 (literal) retorna None sem procurar aresta. Nas quatro consultas de cada grafo não há aresta direta, portanto a saída observada é a prevista; **não se demonstrou o funcionamento de um mecanismo literal T-C**. Isso é divergência implementada, não somente falta de teste adversarial.

Em :583 proof_valid = proof.is_some(); não há consulta pelo ID para validar contexto, origem, intermediário e destino. O código de produção src/cognition.rs:239–260 encontra os dois fatos correspondentes e as topologias são consistentes; não foi encontrada prova errada. Entretanto, presença de IDs não substitui validação independente. As referências de T-C participam da igualdade entre repetições, não de comparação candidato/controle simbólico. Os resultados 32/32 permanecem observados, com garantia de fiscalização menor que a prevista.

Em ec04_panel.rs:630–663 os totais T-R/T-C são impressos, sem assertiva do limiar de acerto; um resultado errado, porém repetível, pode passar essas verificações se as demais contagens forem iguais. Não houve essa falha nos resultados atuais. Uma correção futura exige código e nova reprodução autorizados; esta auditoria não os fez.

### NC-02 — Contrato T-R incompleto; gravidade de engenharia média

Ec04_panel.rs:435–490 constrói memória local por chave, sem os contextos opacos 4101–4103/versão 1 na interface do componente. O campo version de Row:483 é usado como presença de resposta, não versão da memória. Não há operação de aquisição de lote para rejeitar duplicações/conflitos atomicamente, nem limite validado por essa API; as fórmulas do harness produzem somente os 16 pares válidos. O controle independente usa BTreeMap, enquanto candidato/literal compartilham a mesma busca na mesma ramificação. Não há vazamento de expected para essa busca, mas separação de componente/avaliador é mais fraca que em T-A/T-V.

As saídas de recuperação e ausência neste inventário são sustentadas; não é sustentada conformidade geral do contrato ou distinção de conflitos T-R. Correção prospectiva exigiria implementação e reprodução; não é necessária para conservar os acertos históricos limitados.

### NC-03 — Proveniência histórica incompleta; gravidade metodológica alta

Ec04_types.rs:16–24 e trait:29–33 não oferecem consulta de estado por versão nem versão da família/regra na resposta. Os logs mantêm episódios, mas só o sufixo e H atuais são materializados. start/end são índices do log (com IDs sequenciais interpretáveis no harness), não um objeto histórico autocontido com contexto e versão de família. Rows acrescentam contexto local/versão, mas não conservam tabelas históricas completas exigidas pelos §§2/3/12. Não foi demonstrada recuperação histórica das representações ou equivalência integral de evidências internas. Isso limita o uso da EC-04 como validação de proveniência; não anula a mudança observada de classificações.

### NC-04 — Registro e equivalência mais estreitos que os requisitos; gravidade média

Ec04_panel.rs:215–219 reduz Missing e Ambiguous ao mesmo -1; nenhuma coluna preserva motivo. hipóteses em Row:233 é apenas cardinalidade, não os elementos de H. Work não entra nas comparações de repetição. Erros totais são publicados, sem separar seus dois sentidos; cobertura, precisão e acurácia balanceada são deriváveis das classes, mas não emitidas para todos os grupos. Validações associadas são contadas e guardadas em rows; score exclui validation (:407), sem sua tabela/limiar de desempenho.

Por isso, “registros compactos iguais” é correto; “todos os estados, hipóteses, motivos, custos e versões históricos integralmente iguais” extrapola o que é comparado. O critério funcional por classe está assertado para candidato/indução, mas registro completo não está. Não foram criados testes para suprir essa lacuna.

### NC-05 — Ordem e oportunidades de controle; gravidade média

Ec04_panel.rs:613–698 executa cada condição duas vezes consecutivas (a/b); não são duas passagens globais completas na ordem do §9. O isolamento de estado e ausência de aleatoriedade explicam a equivalência observada, mas não tornam a sequência literalmente conforme.

Em :249–254,272–275 e 304–305, o ramo sem atualização T-V é construído treinando um candidato auxiliar com os 64 episódios iniciais por bloco, além das ofertas ao ramo sem atualização T-A. São 256 Acquire auxiliares por repetição, 512 nas duas, além das ofertas nominais; não são consultas extras e não alteram 14560/3120. Mesma informação inicial, mas custo/oportunidades não idênticos aos de uma cópia direta do A1 já executado. O tempo medido inclui esse trabalho. NoUpdate é ablação intencional dependente do candidato; não é um executor independente. Não inferir comparação justa de eficiência desses custos.

### NC-06 — Trabalho não integralmente contabilizado; gravidade de instrumentação alta

O §12 exige uma unidade por avaliação de critério, leitura adicional de episódio/referência e comparação de identidade, inclusive rejeições. Exemplos concretos: ec04_candidate.rs:62–65 soma duas unidades fixas para uma visita com até duas comparações; :56 clona log/estado sem registrar as visitas dessa cópia; :113–115 soma uma unidade para busca/igualdade. Controle literal ec04_controls.rs:58–61 usa três para outra busca condicional. Erros devolvem apenas &'static str, perdendo o Work acumulado para o chamador. A enumeração/ordenação e cópias do controle independente têm contabilização parcial também.

As guardas nominais de 4000000/8192 existem e passaram; isso **não comprova o limite na unidade congelada**, nem equivalência de orçamento efetivo entre executores. Não se constatou excedente real, que não pode ser inferido somente dessa omissão. Correção prospectiva requer instrumentação autorizada e nova execução, sem reescrever métricas antigas.

### NC-07 — Memória fina e instrumentação histórica; gravidade média

Guardas em ec04_panel.rs:394–397 e 752–758 somam capacidades de buffers mais reservas de 96/64 KiB. Faltam contabilização explícita de todos os estados, clones, contextos auxiliares e alocações de bibliotecas; durante a construção de b, a e reference já coexistem. As reservas podem ser suficientes, mas sua demonstração não está instrumentada. Limite de RSS não é substituto do teto de dados/fixtures de 256 KiB. Não foi comprovada violação; tampouco conformidade integral desse teto fino.

O relatório anterior distingue honestamente pico amostrado de pico terminal. Nesta auditoria foi usado wait4, mantendo o código intacto. O valor atual não corrige retroativamente a instrumentação da execução original.

### NC-08 — Fronteiras defensivas incompletas; gravidade baixa no painel atual

Ec04_independent.rs:141–147 retornaria Negative para conjunto H vazio, enquanto candidato retorna Inconclusive (:142–146) e protocolo §3 exige InvalidState. Esse estado não é produzido pela aquisição válida observada; a divergência é estática, não um erro de classificação reproduzido. O enum Outcome não inclui Conflict nem razões detalhadas de Inconclusive. declare também não rejeita ID de contexto duplicado explicitamente, e a capacidade B=32 é satisfeita pelos dados, não imposta como limite distinto na API de aquisição (limite comum 48). Não se certifica robustez fora das entradas congeladas.

## 5. Integridade dos dados e análise de vazamento

Os 12 manifestos foram gerados duas vezes pelo preflight existente, com igualdade dos bytes e expansões. Os 54 hashes (seis partições por A e três por B, em D/V/quatro blocos F) coincidiram com o relatório histórico. Nenhum gabarito final foi publicado nesta auditoria. Os slots A somam 120, B 80; 1200 bytes de vetores no conjunto D/V/F. As verificações de domínio/ordem usam funções compartilhadas com o gerador, portanto não são um gerador independente: foram também examinados diretamente os predicados, slots e complementos contra R1. As quotas positivas e identificabilidade têm verificações separadas; provas de inexistência/esgotamento completo não foram exercitadas por novas condições.

| Contexto | A: expansões | B: expansões |
|---|---:|---:|
| Desenvolvimento | 1833 | 798 |
| Validação | 1817 | 770 |
| Final 1 | 1833 | 770 |
| Final 2 | 1853 | 770 |
| Final 3 | 1801 | 770 |
| Final 4 | 1830 | 770 |

Família de hipóteses e critérios do professor são conhecidos ao autor, mas só o avaliador importa Target/gerador. Os arquivos de modelos importam apenas tipos passivos; a aquisição recebe rótulos permitidos e Query não recebe alvo. O ramo congelado recebe um estado adquirido, não máscara secreta. Não foi encontrado fluxo de gabarito final para inferência. O professor determina previamente as partições diagnósticas e o treinamento posterior; isso é dependência declarada, não inferência autônoma de propriedades.

Ordenação rank usa x, não a recompensa/resposta futura; IDs servem a duplicação/proveniência e não à decisão. Como x determina classe e os alvos são conhecidos ao gerador, não se prova ausência estatística de correlação da ordem/ID com classe; prova-se que os modelos inspecionados não os usam como atalho de classificação. Não há cegueira de autoria, dados naturais ou avaliador externo.

As fatias 0..32 e 80..96 são as únicas fornecidas ao modelo pela aquisição do painel; 32..48,48..80,96..104 e104..120 entram em Query/pontuação. Retenção B reutiliza 16 itens finais de B intencionalmente, nunca na aquisição. Sobreposição de vetores entre contextos/laboratórios é permitida: novidade contextual não equivale a independência global de padrões. T-R repete âncoras adquiridas por definição; T-C renomeia a mesma topologia. Não foi localizada contaminação por uso de exemplos finais para corrigir modelos. A ausência histórica de ajustes, porém, não é certificável somente pelo commit que reúne todos os artefatos finais.

## 6. Reprodução e controles

Por execução do painel existente (que contém duas repetições):

| Unidade | Previsto | Observado |
|---|---:|---:|
| Finais T-R | 224 | 224 |
| Finais T-C | 512 | 512 |
| Finais T-A | 9216 | 9216 |
| Finais T-V | 4608 | 4608 |
| Finais totais | 14560 | 14560 |
| Validação associada | 3120 | 3120 |
| Total | 17680 | 17680 |
| Eventos de circuito T-C | 280 | 280 |

As 288 linhas DETAIL por condição/bloco/contexto/fase coincidiram exatamente com a tabela histórica. Não são 288 casos independentes: há reutilização pré/pós e agregação. A igualdade das duas repetições é assertada pelo painel. A suíte completa reproduziu o mesmo teste existente novamente como regressão; não se somam essas execuções às contagens de um painel.

Resultados por repetição, distinguindo acertos de classe de abstenções:

| Condição | T-A A1: acertos/erros/abstenções em 256 | T-V A V1: acertos/erros/abstenções em 64 | B V1: acertos/erros/abstenções em 64 |
|---|---|---|---|
| Candidato | 256/0/0 | 64/0/0 | 64/0/0 |
| Indução independente | 256/0/0 | 64/0/0 | 64/0/0 |
| Literal | 0/0/256 | 0/0/64 | 0/0/64 |
| Sempre positivo | 64/192/0 | 8/56/0 | 16/48/0 |
| Sempre negativo | 192/64/0 | 56/8/0 | 48/16/0 |
| Sempre abster-se | 0/0/256 | 0/0/64 | 0/0/64 |
| Sem aquisição | 0/0/256 | 0/0/64 | 0/0/64 |
| Sem atualização | 0/0/256 | 32/32/0 | 64/0/0 |
| Regra +p7 | 110/146/0 | 29/35/0 | 24/40/0 |

Candidato/indução: em cada um dos oito contextos, A0=32 abstenções; A1=8/8 positivos e 24/24 negativos. Cobertura e precisão entre respostas A1=100%; acurácia balanceada=1. Sem atualização A1 tem cobertura zero, precisão indefinida e acurácia balanceada zero; ganho do candidato=1. Sempre negativo tem acurácia simples 75%, mas balanceada 50%; não equivale ao candidato.

Em cada bloco de T-V A: V0=2/2 positivos corretos e 6/14 negativos corretos, oito erros negativos explícitos (falsos positivos), cobertura 100%, precisão 50%, acurácia balanceada 5/7. V1=2/2 e 14/14, zero erros/abstenções, acurácia balanceada 1. Ganho frente ao ramo congelado = 2/7; 16/16 sondas B conservam decisão e referências compactas. Retenção decorre de estados separados, sem alegação de pesos compartilhados. Literal conceitual tem cobertura zero, precisão indefinida e acurácia balanceada zero; abstenções em positivos não são acertos.

T-R: candidato, literal e memória independente recuperam 8/8 e reconhecem 8/8 ausências. Sem aquisição, sem atualização e sempre Absent reconhecem 8 ausências e perdem 8 âncoras; Value(0) tem 8 valores errados e 8 respostas indevidas. T-C candidato/simbólico/regra programada têm 16 respostas com IDs de prova e 16 Unsupported; os quatro controles sem composição têm 16 Unsupported corretos e 16 positivas perdidas; sempre Supported sem prova erra 32. Em T-C ausência de prova é abstenção Unsupported, não negação universal. Nenhum caso Inconclusive foi registrado no painel; isso não mede robustez frente a estados inválidos.

### Independência e explicações alternativas

A indução independente não chama candidato: enumera máscaras/valores em ordem reversa, testa os sufixos desde zero e ordena resultados; candidato conserva sufixo ativo e enumera H em ordem crescente. Isso dá independência de implementação parcial, com funções decisórias próprias, mas **mesmo viés e mesma política de recência**. Não é uma hipótese científica concorrente independente de pressupostos nem avaliação externa. Ambos utilizam a mesma codificação de propriedades, professor, família e harness; erros compartilhados do desenho/DTO/avaliação podem passar.

O candidato T-R e literal compartilham ramo de busca; BTreeMap independente oferece outra implementação da memória. T-C composição independente por adjacências e regra por produto de fatos são separadas do TwoHopTask, mas usam o mesmo grafo pronto; só a busca simbólica de dois passos está em jogo. As ablações de aquisição/atualização são deliberadamente menos capazes; respostas fixas não recebem treinamento útil. Não representam competidores de mesma capacidade informacional para alegação de superioridade.

T-A não é mera recuperação literal: finais disjuntos são classificados depois de experiências rotuladas, em contraste com literal/sem atualização que se abstêm. É indução supervisionada simbólica dentro de linguagem de hipóteses fornecida. T-V aplica uma política programada de recência, revisando restrições após contradição; não descobre livremente significado, atributos ou exceções. Seleção dos exemplos é diagnóstica e garante identificabilidade, não amostra natural de generalização aberta. A reformulação de dois para três literais pertence à mesma família; não é transferência estrutural.

## 7. Recursos, verificações e limites da auditoria

Instrumentação externa, sem modificar testes: processo filho executado via fork/exec, wait4 com rusage terminal; /proc amostrado a cada 3 ms e guardas de disco/tempo. Os registros de stdout ficaram temporariamente em /tmp; somente este parecer e decisões serão versionados. O uso de wait4 inclui o escopo do processo iniciado e contabilização do sistema, inclusive fase do lançador antes de exec e filhos aguardados; não é memória exclusiva de estruturas Rust nem soma instantânea da árvore inteira. Não instalaram-se instrumentos ou dependências.

| Verificação existente | Saída | Tempo de parede (s) | ru_maxrss (KiB) |
|---|---:|---:|---:|
| cargo fmt --check | 0 | 0.248979 | 47368 |
| cargo check --offline | 0 | 0.037192 | 25152 |
| cargo check --offline --all-targets | 0 | 0.049874 | 25180 |
| cargo test --offline --test ec04_generator_preflight -- --nocapture --test-threads=1 | 0 | 0.100781 | 25080 |
| binário ec04_panel frozen_final_panel --exact --nocapture --test-threads=1 | 0 | 1.335699 | 8300 |
| cargo test --offline -- --test-threads=1 | 0 | 2.420433 | 87068 |
| git diff --check | 0 | 0.004435 | 9372 |

Painel isolado: 1,331597s internos; 1,335699s incluindo lançamento/espera. wait4: 8300KiB (~8,11MiB), abaixo de 256 MiB; /proc amostrou 7224KiB. Diferente dos 1688KiB amostrados anteriormente, com escopos/limitações explicitados, não é prova de crescimento real. O valor terminal limita a lacuna de amostragem desta reprodução, sem provar conformidade de instrumentação histórica.

Suíte:132 aprovados, 0 falhos, 0 ignorados. Os124 anteriores foram preservados; três dos oito testes adicionais são integrações repetidas do mesmo preflight. Não confundir contagem Cargo com número de experimentos. Verificações/compilação reutilizaram cache; não se mediu rebuild limpo. Maior ru_maxrss dos comandos de regressão:87068KiB; não atribuir esse valor ao painel isolado.

Target permaneceu55.415.100bytes por du -sb; soma lógica dos tamanhos de arquivos, que conta hardlinks separadamente, permaneceu57.116.540bytes. Crescimento nesta auditoria:0 em ambas as convenções. /home antes:4.627.918.848bytes (~4,31GiB); após testes:4.627.841.024bytes (~4,31GiB), sempre acima3GiB nas guardas. O estado final após documentação/versionamento é reconfirmado na entrega.

Memória fina/trabalho: não verificados integralmente pelos motivos NC-06/07. Não há evidência de violação de RSS/tempo/disco nesta reprodução; isso não converte os demais limites em conformes. Sem nova execução adversarial, não se certificam atomicidade sob todas as falhas, orçamento esgotado, rejeições, conflitos ou corrupção. O escopo desta auditoria proibiu corrigir ou criar testes.

## 8. Alcance científico, encerramento e pendências

**Manter as observações:** memória exata funciona no inventário; TwoHopTask compõe fatos fornecidos; exemplos supervisionados identificam critérios dentro de H e mudam classificações de vetores não adquiridos; novos episódios revisam essas classificações e o outro contexto permanece estável. O controle simbólico reproduz o candidato, sem superioridade.

**Não demonstrado:** formação autônoma de semântica/conceitos/atributos; aprendizagem neural de critérios; aquisição da regra de composição; compreensão textual; descoberta autônoma de mudança ou investigação; transferência estrutural; resistência à interferência em parâmetros compartilhados; competência geral ou consciência. Não converter invariância B em resistência neural à interferência.

O aceite qualitativo da etapa (aquisição e aplicação controladas) tem evidência delimitada em T-A/T-V. O êxito integral definido pelo protocolo §11 **não pode ser certificado**, porque inclui requisitos de registros, controles e limites que não foram integralmente cumpridos/verificados. Esses dois juízos não são contraditórios. Não rejeitar acertos reproduzidos, nem declarar todos os requisitos atendidos.

Parecer favorável ao **encerramento como investigação delimitada com conformidade parcial e conclusão AJUSTAR**, sem apagar NC-01–08. As não conformidades impedem certificação integral, mas não impedem concluir e arquivar a investigação com essas ressalvas. Uma alegação futura que dependa de proveniência histórica completa, orçamento uniforme ou literal T-C real exige correção prospectiva autorizada e reprodução própria; não completar provas históricas retroativamente. Mudanças na ciência do protocolo exigem revisão identificada; simples adequação de implementação ao texto existente ainda exige autorização de código/testes e nova execução. Nada disso foi iniciado.

A prioridade é preservar as evidências e exigir que dependências futuras indiquem exatamente quais competências precisam, sem transformar encerramento em autorização automática. A próxima etapa nominal do plano é EC-05, mas não está iniciada nem autorizada por este parecer. Não se recomenda reconstrução da arquitetura ou infraestrutura nova com base nas lacunas encontradas.

## 9. Artefatos desta delegação

Somente docs/EC-04_auditoria_cientifica_final.md (novo) e acréscimo em docs/decisions.md. Relatório original de implementação, tarefas, planos, protocolos EC-04/R1, código e testes preservados byte a byte. Versionamento documental autorizado; hash final do commit e sincronização são informados após commit/push, evitando referência autorreferente.
