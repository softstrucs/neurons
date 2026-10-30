# EC-06 — Auditoria da inferência existente

## 1. Requisito, base e conclusão desta subetapa

Base `30ef11a0377cf1bbfcdd4a4d7edb4b5dc056c3a9`, branch devs, árvore inicialmente limpa; HEAD, origin/devs e referência remota coincidentes. Autorização: **EC-06 — Raciocínio com evidências, subetapa inicial: auditar a inferência existente**. A EC-05 está encerrada como investigação delimitada, AJUSTAR e conformidade metodológica parcial; suas conclusões não foram reabertas ou reescritas.

O plano cognitivo v0.2, §5, define EC-06 — **Raciocínio com evidências**, princípios1/4/7/8/9. O mapa §4 usa a forma resumida Raciocínio fundamentado, dependências EC-04/05 e objetivo compor inferências/verificar evidências. Ordem literal: **auditar inferência existente**; definir tarefas composicionais; separar conhecimento fornecido e adquirido; testar raciocínio com evidências, controles e problemas não cadastrados. Aceite da etapa completa: justificativas rastreáveis e resultados em avaliações independentes, com reconhecimento de incerteza.

**Primeira subetapa concluída:** linha de base rastreável de mecanismos e evidências. Há recuperação exata, composição de dois fatos com propagação e busca simbólica, seleção contextual programada e laboratórios de revisão/proveniência. Não há demonstração de aquisição da regra geral de composição, raciocínio generalizável, compreensão geral ou autonomia. O aceite integral da EC-06 não é declarado atendido. Não se definiu tarefa, algoritmo novo, protocolo futuro ou arquitetura definitiva; a segunda subetapa não foi iniciada.

## 2. Referências e método

Examinados o plano cognitivo v0.2, Fundamento original, complemento A1–A7 e decisões; [auditoria EC-03.5](EC-03.5_auditoria_experimental.md), [auditoria EC-04](EC-04_auditoria_cientifica_final.md), [parecer final EC-05](EC-05_auditoria_cientifica_final.md); protocolos/resultados de proveniência, formação de critérios, diagnóstico/retenção contextual e expectativas causais. Consultada a [auditoria de relações anterior à EC-04](EC-04_auditoria_relacoes_existentes.md), distinguindo suas lacunas históricas das evidências posteriores de indução de critérios.

Código principal: cognition/metacognition/engram/circuit/neuron/synapse/engine/event/learning/curriculum/session/persistence; languages e seus módulos contextual/sentences. Testes: cognition, metacognition, curriculum, concepts, a02_1_final/r1, contextual_final, sentences_final e suportes; expectativa causal, proveniência, EC-04 e EC-05. Consultados os registros A01/A02, incluindo resultado nulo A02.5. As capacidades são atribuídas às interfaces e aos testes efetivos, não aos nomes dos módulos.

Método: análise estática com funções/linhas e testes identificáveis; reprodução focal dos testes existentes de cognição/monitoramento; regressão integral existente. Métricas focais atuais estão em §5. Números consolidados dos relatórios EC-03/04/05 e A01/A02 são históricos consultados e assertivas existentes cobertas pela regressão; não se fez nova extração independente de seus painéis nesta auditoria. Não somar testes Cargo, consultas, eventos e experiências independentes.

## 3. Distinções operacionais para interpretar a linha de base

| Termo | Significado nesta auditoria | O que não autoriza concluir |
|---|---|---|
| Recuperação literal | Igualdade de pista, forma ou chave retorna conteúdo adquirido/cadastrado. | Compreensão, composição ou verdade da associação. |
| Regra previamente programada | Transformação/composição e condições de aplicação definidas externamente. | Aprendizagem da própria regra. |
| Composição simbólica | Duas ou mais referências fornecidas são relacionadas sob uma transformação explícita, produzindo conclusão não recuperada diretamente. | Semântica autônoma, transitividade de qualquer relação ou raciocínio geral. |
| Inferência demonstrada | Resultado derivado e evidências verificados no contrato e nos casos declarados. | Generalização além desse contrato. |
| Hipótese como consulta | `cognition::Hypothesis` contém contexto/origem/destino pedidos pelo chamador. | Geração autônoma de perguntas ou de explicações. |
| Hipótese como registro | `RecordStatus::Hypothesis` marca conteúdo declarado e excluído das premissas de TwoHopTask. | Evidência confirmatória ou promoção automática a conhecimento. |
| Hipótese em família finita | EC-04 enumera critérios de H, conserva os consistentes e decide por unanimidade. | Criação da linguagem de hipóteses, predicados ou gramática de raciocínio. |
| Aprendizagem/transferência | Exige distinguir mudança interna por experiências, retenção e uso posterior em condições realmente novas. | Chamar aplicação de regra pronta ou renomeação de símbolos de aprendizagem neural/generalização estrutural. |

EC-04 possui evidência de **aquisição e revisão simbólicas de critérios em H**. Portanto não se afirma ausência de toda aprendizagem ou de toda aplicação a exemplos novos. O que falta demonstrar é aquisição do mecanismo/regra de composição, formação autônoma de representações semânticas e transferência além das famílias/estruturas fornecidas.

## 4. Inventário dos mecanismos efetivos

### 4.1 Evidências e recuperação

`src/engram.rs:58–180,202–249`: Cue(context,source,pattern), EngramId, origem, observed_at, EvidenceId opcional e estado declarado. EngramStore mantém registros por capacidade, recupera por ID ou pista exata e sinaliza ausência/ambiguidade. `Validated` exige uma referência, não verifica sua qualidade. Recall não decide validade empírica, não filtra por si só hipóteses e não aplica revisão/retratação/intervalo temporal geral. O consumidor precisa distinguir recuperação de evidência utilizável.

Tempo de observação não é validade; contexto numérico não é descrição semântica aprendida. Produção não oferece a sucessão contextual/dependências e histórico dos laboratórios EC-03/05 como contrato integrado. `src/persistence.rs:1,37–72` salva Engine+EngramStore, não Circuit nem os estados experimentais de expectativa, proveniência, H ou retenção. Checkpoint de associação não garante persistência do raciocínio ou dos parâmetros desses ensaios.

### 4.2 TwoHopTask: circuito real e regra fornecida

`src/cognition.rs:118–262`, TwoHopTask::run:

1. Filtra registros do contexto, valor1, destino numérico positivo, excluindo Hypothesis; rejeita duplicação/limite de fatos.
2. Retorna DirectRecall sem executar circuito se já existe a aresta consultada.
3. Para composição, reconstrói circuito com limiar1, conexões de peso1 e atraso1, um fato fornecido por conexão. Não aplica PlasticityRule.
4. Estimula a origem, executa eventos limitados e verifica erros/orçamento/ativação de origem/destino.
5. Busca explicitamente nos fatos um intermediário ativo com duas premissas adequadas; retorna a primeira prova na ordem determinística, com dois EngramId e ativações.

Há propagação efetiva por neurônios/sinapses e dependência local desse caminho de implementação: neutralizar todas as sinapses impede as respostas compostas no ensaio. Mas a topologia vem dos fatos, os pesos são fixos e a regra de dois passos/busca de prova é programada. O controle simbólico com os mesmos fatos resolve as oito consultas. Isso não demonstra necessidade neural geral ou aquisição da regra.

A execução pode ativar o destino por caminho mais longo e ainda retornar Unsupported por não existir prova de exatamente dois passos: teste `longer_path_or_unconfirmed_fact_does_not_count_as_two_hop_proof`. Portanto ativação não é prova suficiente. A condição adicional não identifica por si só a contribuição causal individual de cada conexão; os diagnósticos EC-02 são evidência local separada.

TwoHopTask devolve **uma prova**, não todas as provas nem unicidade semântica. Não possui sistema geral de tipos/predicados, aprendizagem de transitividade, exceções, validade temporal ou recursão de conclusões. Carrega o grafo positivo do contexto; não exige DAG geral. Ciclos/fanout podem consumir orçamento; não se testou uma condição nova nesta auditoria para caracterizar seu efeito. Os limites de eventos/filas não representam contabilização integral de CPU, buscas ou memória.

### 4.3 Monitor e integração com observações

`src/metacognition.rs:113–300,302–352`: Monitor::review/assess avaliam o relatório, detectam negativa explícita na conclusão/premissas da prova retornada, distinguem falta, contradição, limite, propagação e evidência inválida. Se a consulta original é suficiente, responde imediatamente; a busca de contexto alternativo ocorre após falta/contradição. Alternativas são fornecidas pelo chamador; revisão só é aceita quando uma candidata sustenta e os limites permitem concluir a busca. Não se descobre o contexto nem se alteram fatos.

`find_conflict` examina a **prova selecionada** por TwoHopTask. Não enumera vias alternativas dentro do mesmo contexto nem implementa o isolamento seletivo D1 da EC-05. Não é política geral de consistência ou avaliação de todas as interpretações. A precedência e os critérios de abstenção são programados, não metacognição aprendida.

`src/curriculum.rs:45–103`, Microcurriculum, insere observações externas em quatro contextos de Domain e chama esse monitor com uma tentativa. Os quatro domínios reutilizam a mesma forma de composição, não regras semânticas aprendidas por domínio. `src/session.rs` oferece comandos estruturados de observação/consulta; não interpretação livre de linguagem. Engramas de conclusões não são gravados automaticamente por TwoHopTask/Microcurriculum.

### 4.4 Idiomas e gramática: papéis tipados locais

- `src/languages.rs:72–184`, Lexicon: dicionário bidirecional fornecido, recuperação em engramas; sem inferência semântica nova.
- `ConceptLexicon::associate/compose`, `:276–428`: duas associações forma→conceito, por idioma, produzem forma de destino compartilhando conceito/sentido declarado. Enumeração simbólica com conflito, ambiguidade e limite; não usa TwoHopTask/Circuit e não registra aresta direta de tradução para esse ensaio.
- `src/languages/contextual.rs:161–259`, ContextLexicon::translate: contextos enumerados externamente filtram as associações/sentidos; sem contexto pode haver mais de uma prova. Não descobre contexto textual.
- `src/languages/sentences.rs:198–279`: substantivo+adjetivo pt-BR, adjetivo+substantivo en, gênero e léxico fornecidos; compõe saída com referências à regra e aos dois fatos. Não aprende sintaxe ou gramática nova.

Esses contratos são mais tipados para suas tarefas que a convenção relacional genérica do núcleo, mas não compartilham uma regra universal de raciocínio nem a política P1/P2. Seus budgets/candidatos/entrada não medem a mesma unidade do orçamento de eventos do núcleo. UTF-8 literal não oferece normalização Unicode geral ou compreensão de pt-BR.

### 4.5 Laboratórios isolados: distinção entre inferência, manutenção e aprendizagem

| Laboratório | Mecanismo existente | Fronteira científica |
|---|---|---|
| EC-02.4, `tests/support/expectation.rs`, Learner::acquire/query | Compara ações factual/bloqueada, confirma após dois contrastes consecutivos e aplica permitir/proteger/investigar. | Expectativa local adquirida por atualização programada; sem aprendizagem do mecanismo causal geral. Literal supera resumo em P4. |
| EC-03, `tests/support/provenance_*.rs` | Regra L de dois passos, evidências/versionamento, Refresh, suspensão, retratação e replay. | Manutenção e composição simbólicas, contrato histórico de conflito global; History/revisão de regra/metadados parciais. |
| EC-04, `ec04_candidate.rs:35–161` e independente | Enumera H de576 critérios sobre8 propriedades, elimina inconsistentes e revisa pelo maior sufixo consistente. Query por unanimidade. | Aprendizagem simbólica supervisionada de critérios em família fornecida; não aprende a família nem a regra de composição. |
| EC-05, `retention_candidate.rs`/`retention_symbolic.rs` | P1 materializa e revalida; P2 recompõe; D1 rastreia dependências; D2 exige sucessão e continuidade declaradas. | Suporte válido e memória revisável no ensaio; sem integração automática ao núcleo ou descoberta de credibilidade/contexto. |
| A01/A02, `learning.rs`, FeedbackExperiment e suportes de circuitos | Pesos ajustados por feedback; escolhas por ativação em condições determinadas, com controles e ablações. | Plasticidade local observada não é aquisição de regras inferenciais. Supervisão/exploração/codificação externas permanecem; A02.5 conserva resultado nulo partindo de pesos zero. |

Motor determinístico, Circuit, neurônios, sinapses e PlasticityRule são infraestrutura reutilizável. Não se deve confundir a disponibilidade desses tipos com participação neural nos laboratórios simbólicos ou com integração validada entre memória, aprendizagem e raciocínio.

## 5. Matriz rastreável de capacidades e evidências

Legenda: **atual** é saída/inspeção desta auditoria; **histórico** são resultados dos documentos anteriores, com testes existentes novamente aprovados na regressão, sem nova extração independente de cada painel. “Demonstrado” é restrito ao contrato indicado.

| Capacidade alegada | Mecanismo / código e teste | Evidência observada | Limitação | Lacuna experimental |
|---|---|---|---|---|
| Recuperar conteúdo conhecido | EngramStore::recall, tests/memory.rs; DirectRecall, tests/cognition.rs | Atual: recuperação direta distinguida de composição, zero eventos nessa rota. | Igualdade exata; referência não certifica verdade. | Distinguir efeito da recuperação de transformação posterior sob suporte verificável. |
| Inferir conclusão não cadastrada por dois passos | TwoHopTask::run; `protocol_scores_held_out_queries_against_simple_baselines` | **Atual:8/8** consultas; simbólico8/8; recall/tabela/ablação4/8; pseudoaleatório de semente fixa3/8. | Mesma topologia/regra fornecida, quatro positivas e quatro sem suporte; busca simbólica e circuito combinados. | Composição além da família já fornecida e controles realmente equivalentes. |
| Justificar resposta | cognition::Proof, EvidenceRefs, testes cognitivos | Atual: primeiro/segundo fato e intermediário verificados no caso positivo. | Uma prova; sem regra identificada por versão, todos os caminhos ou evidência empírica autenticada no núcleo. | Completude e validade de justificativas diante de múltiplas dependências. |
| Detectar contradição e revisar consulta | Monitor::review/find_conflict; tests/metacognition.rs | **Atual:7/7** decisões do painel; uma revisão e quatro abstenções adequadas; inferência sem monitor5/7. | Controle não recebe alternativas; negativa avaliada só na prova escolhida. | Comparabilidade informacional e cobertura de alternativas; não atribuir vantagem geral à rede. |
| Reutilizar composição em domínios | Microcurriculum; tests/curriculum.rs | Histórico:4/4 decisões por domínio; destino precisa de fatos próprios na sonda chamada transferência. | Mesma forma de cadeia em contextos externos, sem transferência aprendida. | Distinguir renomeação/topologia de nova relação ou estrutura. |
| Compor por conceito/sentido | ConceptLexicon::compose; tests/concepts.rs | Histórico:16/16 por direção, duas referências e nenhuma aresta direta final. | Conceitos/rótulos recebidos; controle original usa destino esperado. | Não usar o verificador de caminho original como seletor cego. |
| Seleção conceitual com controle cego | tests/support/concept_blind.rs; a02_1_final/r1 | Histórico:48/48 decisões,32/32 evidências positivas; validação adicional18/18. | Validação posterior; caso adversarial limite/conflito tem justificativa diferente, sem orçamento comum de leituras. | Independência metodológica e orçamento comparável, não equivalência geral presumida. |
| Seleção contextual de expressão | ContextLexicon::translate; tests/contextual_final.rs | Histórico:24/24 por direção; simbólico48/48; negativos16/16; sem contexto44 respostas corretas e4 abstenções. | Contextos/sentidos fornecidos, sem aprendizagem do vínculo semântico. | Contexto não fornecido ou estruturas novas continuam sem demonstração. |
| Composição gramatical | SentenceLexicon::translate; tests/sentences_final.rs | Histórico:48/48, simbólico48/48,16/16 negativos; sem regra48 inconclusões. | Gramática de dois constituintes e gênero declarados; regra não adquirida. | Aquisição linguística e aplicação além dessa gramática não avaliadas. |
| Adquirir/revisar critério | EC-04, tests/ec04_learning.rs/ec04_panel.rs | Histórico: aplicação em vetores não adquiridos e revisão em H; controle de indução equivalente. | H e codificação externos; T-C literal/proveniência/custos têm não conformidades preservadas. | Não extrapolar aprendizagem simbólica de critério a geração de predicados ou regra nova. |
| Adquirir expectativa causal local | EC-02.4, tests/causal_expectation.rs | Histórico: P0–P5 e revisão reproduzidos nas auditorias; literal melhor em P4. | Intervenções e decisão programadas, sem descoberta causal autônoma. | Utilidade frente a episódio literal e atribuição causal fora do diagnóstico local. |
| Manter suporte e histórico contextual | EC-03/05; provenance_revision, ec05_context_change, ec05_retention | Histórico:500/516/272 verificações nos respectivos painéis; retenção60/60 histórias, P1 80/126 versus P2 90/126. | Regras e aplicabilidade externas; dez abstenções adicionais P1 pré-manutenção; não integrado à produção. | Histórico/validação gerais, evidências independentes e custos uniformes permanecem parciais. |
| Aprender regra de inferência por sinapses | PlasticityRule, A02 e TwoHopTask separados | Ajuste local de pesos existe; **não demonstrada** aquisição da regra inferencial. | TwoHopTask sempre reconstrói pesos unitários; A02.5 silenciosa não atualiza. | Não há evidência que conecte aprendizagem de regras ao desempenho inferencial generalizado. |
| Formular hipóteses/investigar autonomamente | Hypothesis, RecordStatus e enumeração H | Consulta recebida, rótulo declarado e seleção em família finita implementados. | Não geram investigação executada e justificada pelo próprio sistema. | Formulação aberta, escolha de experiências e revisão autônoma não demonstradas. |
| Raciocínio generalizável | Nenhum mecanismo/teste identificado como prova suficiente | **Não demonstrado.** | Composição pronta, novas combinações e vetores em H não são transferência estrutural. | Continua questão experimental futura, sem arquitetura selecionada nesta auditoria. |

## 6. Limitações e riscos metodológicos

1. **Equivalência de informação:** o painel cognitivo tem controle simbólico capaz de compor os mesmos fatos e empata8/8. Recall, tabela direta e ablação não têm a mesma capacidade inferencial;3/8 de uma semente pseudoaleatória não é análise estatística. No painel metacognitivo, `tests/metacognition.rs`, score_case, o próprio comentário registra que o controle não recebe/avalia contexto alternativo. O7/7 versus5/7 mede o pacote monitorado nesse desenho, não superioridade com informação equivalente. Não se alteram resultados ou nomes históricos dos testes.
2. **Regra e semântica fornecidas:** microcurrículos, grafos e conceitos recebem significado externo. A regra de dois passos não é transitividade válida para qualquer predicado, tradução ou transição. A ausência de um sistema geral de tipos/exceções limita reaproveitamento; não autoriza inventar esse sistema agora.
3. **Prova selecionada e contexto:** TwoHopTask ordena fatos e encerra na primeira prova encontrada. Monitor considera essa prova e pode revisar para contextos candidatos externos; não implementa D1/D2 nem uma procura completa de interpretações. A restrição é demonstrável estaticamente, sem alegar falha dinâmica nova fora dos testes existentes. Políticas EC-03/04/05 não são intercambiáveis automaticamente.
4. **Fronteiras de validação:** Hypothesis tem campos públicos; evaluate encaminha para run sem repetir Hypothesis::new. Monitor::review revalida a consulta. Não se deve atribuir a todos os pontos de entrada a mesma garantia do construtor/monitor. Esse é risco de pré-condição de interface identificado estaticamente, não novo teste adversarial executado ou impedimento à auditoria.
5. **Manutenção versus recomposição:** literal EC-03/05 responde antes de Refresh em sondas previstas. P1 tem dez abstenções a mais que P2 na retenção. Não classificar toda diferença como erro, nem premiar automaticamente conservação de cache como eficiência ou segurança superior. Em memória incompleta, Unknown não equivale a falsidade.
6. **Controles e proveniência:** A01.2 originalmente verificava caminho até resposta esperada; A02.1 acrescentou seletor cego, sem validar retroativamente o controle antigo. Código independente ainda compartilha política/família/dados/autor/avaliador. IDs e fontes declaradas não são independência empírica ou autenticação. Existência de EvidenceId não prova suporte válido.
7. **Aprendizagem e causalidade:** coatividade, atividade do destino, alterações locais de peso e dependência por ablação não demonstram aquisição de significado ou necessidade neural geral. A02.2 registra12 erros/24 após primeiro feedback e interferência; A02.5 mantém pesos zero e nenhuma elegibilidade real. Resultados simbólicos posteriores não corrigem esses limites; também não anulam plasticidade local efetivamente observada em outros ensaios.
8. **Instrumentação/proveniência histórica:** EC-03/04/05 encerraram com ressalvas de Work, dados vivos, histórico e controles. EC-04 tem literal T-C incompleto, contrato T-R/proveniência e ordem global de repetições parciais; EC-05 preserva a ressalva da regra c2 completada pré-testes, após início da redação de código. Hash atual/commit posterior não prova pré-registro independente. Nenhuma dessas conclusões foi modificada aqui.
9. **Generalização e persistência:** mudança de nomes, novos pares consultados, critérios em H e manutenção de estados separados têm interpretações distintas. Não há demonstração de aprendizagem da própria linguagem de hipóteses, inferência recursiva geral, resistência neural à interferência compartilhada ou recuperação histórica universal integrada. Os protótipos não são automaticamente salvos pelo checkpoint de produção.

Não foi identificada necessidade de reconstruir arquitetura para concluir esta auditoria. Preservar motor determinístico, tipos compactos, recuperação com referências, separação de resultados, controles fortes e protótipos históricos. Qualquer adaptação dependerá de tarefa/autorização futuras; nenhuma foi realizada.

## 7. Relação com as diretrizes A1–A7

| Diretriz preservada | Contribuição atual reutilizável | Capacidade que não se presume |
|---|---|---|
| A1 Memória evolutiva interconectada | EngramId, pistas, provas e dependências experimentais podem ser referências de evidência. | Memória cognitiva integrada/escalável ou camadas físicas já implantadas; não impor unidade universal ou número fixo de níveis. |
| A2 Mecanismos de memória | Recall e laboratórios de revisão/retratação/histórico. | Consolidação/reorganização neural e revisão geral de engramas em produção. |
| A3 Inicialização pedagógica | Microcurrículos e experiências delimitadas fornecidos pelo chamador. | Inicialização/autoseleção pedagógica ou investigação de domínios já autônomas. |
| A4 Comunicação durante aprendizagem | Relatórios estruturados, motivos de abstenção e CLI podem comunicar limitações. | Conversação/compreensão geral ou investigação real disparada por mensagem de desconhecimento. |
| A5 pt-BR prioritário | Inventários lexicais, contextos e gramática restrita ajudam a auditar evidências linguísticas. | Aquisição de alfabeto, fonologia, semântica, leitura/interpretação/conversação ou competência geral pt-BR. |
| A6 Expansão multilíngue | Representação explícita de idioma/direção e conceitos bilíngues fornecidos. | Aprendizagem autônoma de idiomas, alfabetos e estruturas novas. |
| A7 Rigor científico | Separação de dados/controles, relatórios e distinções acima. | Consciência subjetiva, compreensão geral, aprendizagem neural de regras ou autonomia cognitiva comprovadas. |

Compatibilidade é conceitual, não escolha de arquitetura definitiva. O complemento e o Fundamento permanecem byte a byte inalterados, assim como planos e protocolos. A investigação EC-06 não incorpora agora diretrizes futuras como fatos fornecidos ao aprendiz.

## 8. Lacunas prioritárias e recomendação para a próxima subetapa

A próxima subetapa nominal é **definir tarefas composicionais**. Recomenda-se que sua futura especificação distinga o que será fornecido, o que poderá ser adquirido e o que contará como composição, conclusão nova e transferência; exija justificativa fiscalizável, negativas/abstenções e controles com as mesmas informações pertinentes. Isso é orientação derivada da auditoria, não tarefa ou protocolo já definido.

Prioridades que precisam ser consideradas nessa definição, sem iniciar sua execução:

- Delimitação da relação/regra e da novidade pretendida, evitando atribuir ao circuito a semântica recebida ou aprender uma regra que já foi fornecida.
- Suficiência/validade das premissas, alternativas, contexto, oposição e alcance das provas, distinguindo o contrato do núcleo dos contratos EC-03/05.
- Controles comparáveis e critérios verificáveis de justificativa/custo, preservando diferenças legítimas entre recuperação, manutenção e recomposição e as lacunas instrumentais existentes.

Não se escolhem exemplos, quotas, limiares, política adicional, família nova, algoritmo ou arquitetura. Tampouco se elabora o protocolo de etapas posteriores. As competências ainda não demonstradas só poderão ser dependências futuras quando explicitamente avaliadas; encerramentos AJUSTAR não as concedem.

## 9. Verificações realmente executadas e recursos

Reprodução focal atual: **8 testes de cognition** e **9 de metacognition**, todos aprovados. Saídas atuais: `stage06_protocol total=8 model=8 direct_recall=4 fixed_table=4 symbolic_two_hop=8 no_synapses=4 seeded_random=3`; `stage07_protocol cases=7 monitor_correct=7 control_correct=5 adequate_revisions=1 adequate_abstentions=4`, sem falsos alarmes do monitor e uma falha de detecção do controle bruto.

Esses nomes stage06/stage07 pertencem às Etapas06/07 do plano v1.1; não são novos experimentos da série EC-06/EC-07. Foram reexecutados somente testes já existentes. As8/7 consultas desses dois protocolos não são contagem de testes Cargo nem novas amostras independentes.

Passaram `cargo fmt --check`, `cargo check --offline`, `cargo check --offline --all-targets`, os testes focais com `--nocapture --test-threads=1`, `cargo test --offline -- --test-threads=1` e `git diff --check`. **Suíte:140 aprovados,0 falhos,0 ignorados**; os focais também são executados dentro da regressão e não devem ser somados como novos testes.

Rust/Cargo1.88.0, cache existente, sem instalação ou rebuild limpo. Focais de cognição/monitor:0,039459/0,039743s externos pelo lançador Cargo, picos wait4 de25228/25068KiB. Esses valores incluem Cargo/filhos aguardados/fork-exec; **não são benchmarks isolados do mecanismo cognitivo**. Regressão2,965102s e84280KiB reportados; amostra /proc de Cargo não mede soma instantânea da árvore de testes. Não se mede aprendizagem pelo tempo da suíte.

Monitor externo observou disco/tempo/RSS com escopo declarado; guardas não violadas. /home inicial4486242304 bytes; mínimo observado nas verificações4484182016 (>3GiB). Target permaneceu59619598 bytes, crescimento0. Instrumentação histórica de memória fina/trabalho não foi corrigida ou recertificada. Não há medição de novo painel EC-06, pois ele não foi criado.

Um inventário SHA-256 da base protege código, testes, planos, fundamentos e protocolos; antes do versionamento deve confirmar somente acréscimo em docs/decisions.md entre os arquivos rastreados e este relatório novo. Verificados referências locais, UTF-8, whitespace e escopo. Logs/instrumentos temporários permanecem em /tmp. Commit/push documentais são autorizados nesta delegação e seus hashes são fornecidos depois do versionamento, sem autorreferência.

## 10. Encerramento da primeira subetapa

Linha de base entregue: inferência relacional limitada com circuito configurado, recuperação/composição simbólicas tipadas, monitoramento programado e laboratórios de atualização de critérios/proveniência/contexto. As evidências positivas e limitações são compatíveis com a continuidade documental, sem satisfazer antecipadamente a etapa EC-06 completa.

Nenhuma decisão científica nova foi necessária para inventariar os mecanismos. Não foram iniciadas tarefas composicionais novas, aprendizagem de regras, integração de memória, protocolos futuros, serviços ou Etapa12 API/web. Não se declara raciocínio geral, compreensão ou consciência artificial. Próximo passo nominal: **definir tarefas composicionais**, ainda não iniciado e dependente de autorização própria.
