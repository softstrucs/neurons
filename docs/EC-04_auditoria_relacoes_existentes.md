# EC-04 — Formação progressiva de conceitos e relações
## Primeira subetapa: auditoria de relações existentes

### Escopo e identificação

Auditoria estática concluída em 2026-10-29, branch `devs`, base `82f8979e1f6ee0c848513a221d850bb4c7485451`, árvore inicialmente limpa. A definição integral da EC-04 no plano cognitivo v0.2 foi lida: princípios 1, 2, 4, 7 e 8; ordem de subetapas: **auditoria de relações existentes**; definição de tarefas com exemplos não cadastrados; protocolo com controles de memorização; implementação isolada; avaliação de discriminação, composição e exceções. O plano não atribui números individuais a essas subetapas; este relatório conserva a denominação literal da primeira.

O aceite da EC-04 completa exige evidência de aquisição e aplicação em condições controladas, sem confundir associações lexicais ou regras codificadas com compreensão. Esta auditoria não satisfaz esse aceite experimental nem executa a segunda subetapa.

Referências: Fundamento aprovado em `docs/fundamentos/`, plano v0.2, docs/decisions.md, relatórios EC-03.1/03.2/03.5, fontes de engramas, cognição, metacognição, currículo, idiomas e os suportes experimentais de expectativa/proveniência. A EC-03 está encerrada como investigação delimitada por decisão explícita nesta delegação, com conformidade parcial e conclusão AJUSTAR preservadas. Isso não demonstra formação autônoma de conceitos.

### Inventário de relações e conceitos

| Representação e localização | Como entra e é usada | O que existe e limite |
|---|---|---|
| Associação `Cue -> value`, `src/engram.rs` | Chamador fornece contexto, origem, padrão, valor, status e referências; record/get/recall por identidade ou igualdade exata. | Armazenamento e recuperação, ausência/ambiguidade explícitas. Não aprende tipo da relação ou conceito; EvidenceId não valida qualidade da evidência. |
| Relação dirigida em `src/cognition.rs`, RelationFact e TwoHopTask::run | Interpreta origem/destino em Cue e valor positivo 1; exclui Hypothesis, filtra contexto e constrói circuito com conexões unitárias. | Recuperação direta ou composição de dois fatos com Proof; busca simbólica auxiliar e regra de composição programadas. Não aprende o predicado nem armazena automaticamente conclusões. |
| Relações negativas e conflitos, `src/metacognition.rs`, Monitor::assess | Examina registros e referências acessíveis, inclusive negativas da conclusão/premissas, e decide responder ou abster-se. | Detecção delimitada por convenções; ausência de prova não equivale a falsidade. Contextos alternativos são fornecidos, não descobertos. |
| Inclusão simbólica de categorias, `tests/curriculum.rs:89` | Cadeia declarada pardal/ave/animal/organismo em IDs. | Inclusões e itens fornecidos; não há observação de propriedades que forme a categoria ave. |
| Ordem matemática, `tests/curriculum.rs:94` | IDs representam números e fatos de ordem são cadastrados; oráculo numérico pertence ao teste. | O circuito usa conectividade, não calcula ordem numérica nem aprende comparação de quantidades. |
| Precedência natural, `tests/curriculum.rs:100` | Cadeia de instantes/estados sintéticos frio→morno→quente→muito quente. | Precedência fornecida, não lei física, relação causal aprendida ou descoberta experimental. |
| Transição em computação, `tests/curriculum.rs:106` | Estados de contador codificados em cadeia com IDs externos. | Reutilização da mesma topologia; não inferir automaticamente transitividade de qualquer transição computacional. |
| Associação lexical direta, `src/languages.rs`, Lexicon | Pares pt-BR/en fornecidos e registrados em engramas; consulta literal restrita ao inventário. | Dicionário experimental bidirecional, sem aquisição de significado. |
| Forma/conceito/sentido, `src/languages.rs:254`, ConceptLexicon::add_concept/associate/compose | IDs e rótulos de conceitos externos; forma→conceito por idioma; seleção por conceito compartilhado com duas referências. | Composição simbólica dirigida de papéis definidos. Não cria conceito por regularidade, propriedades ou feedback. Não é transitividade irrestrita. |
| Forma/sentido/contexto, `src/languages/contextual.rs`, ContextLexicon | Contextos enumerados e relações declaradas; seleção exige evidência e respeita orçamento. | Seleção contextual de inventário, não inferência livre do contexto textual. |
| Relações gramaticais, `src/languages/sentences.rs`, SentenceLexicon | Léxico, gênero e regra fornecidos; substantivo+adjetivo pt e adjetivo+substantivo en. | Composição controlada com referências a regra e fatos, não aprendizagem de gramática ou estrutura nova. |
| Conexão sináptica, `src/synapse.rs`, `src/learning.rs` | Ligação física identificada, peso/atraso; feedback modifica peso indicado. | Relação operacional entre unidades, sem equivalência automática a um predicado semântico aprendido. |
| Expectativa causal local, `tests/support/expectation.rs` | Observações pareadas alimentam contraste e resumo por chave. | Conteúdo contextual adquirido por regra programada; não forma conceitos ou mecanismos causais gerais. |
| Relação L e dependências, `tests/support/provenance_*.rs` | Fatos e regra de dois passos fornecidos; versões, suportes, suspensão/retratação e consultas históricas. | Manutenção delimitada de representações, não aquisição da regra ou integração em produção. Ressalvas EC-03.5 permanecem. |

### Semântica, composição e segurança de interpretação

A identidade e o nome de um domínio não fornecem semântica ao circuito. Microcurriculum codifica uma relação por contexto; `Domain` seleciona contextos fixos e não regras aprendidas por domínio. A mesma cadeia é reutilizada nos quatro ensaios. O núcleo TwoHopTask não possui um registro geral de tipos de relação com propriedades como simetria, transitividade ou exceções. Sua convenção de dois passos deve ser aplicada somente a relações cuja composição tenha sido definida pelo experimento.

ConceptLexicon e ContextLexicon oferecem papéis mais específicos (forma, idioma, conceito, sentido e contexto), mas esses papéis e os rótulos são fornecidos externamente. Identidade compartilhada de conceito permite ligação lexical; não demonstra que o sistema reconheça propriedades comuns ou forme categorias por experiência.

TwoHopTask distingue DirectRecall de Inferred e devolve dois fatos de suporte. A busca final retorna uma prova encontrada na ordem determinística dos fatos; não enumera todas as interpretações ou prova unicidade semântica. Monitor examina evidências dentro do seu escopo; não é sistema geral de consistência lógica. Não extrapolar abstenção por cadeia de três passos para negação da relação: por exemplo, uma ordem numérica pode ser verdadeira e ainda estar fora do orçamento/escopo de prova.

Nenhum dos mecanismos auditados oferece formação autônoma de predicados, categorias por propriedades percebidas, seleção aprendida de regra de composição ou tratamento geral de exceções. Dados negativos explícitos, conflito, Unknown, limite e inconclusão têm papéis diferentes e devem continuar distintos.

### Evidências existentes e sua força

As métricas seguintes são registros históricos e assertivas inspecionadas, não resultados reexecutados nesta auditoria:

- Etapa 06: composição sintética de duas relações e controle simbólico com 8/8 consultas reservadas; envolve busca auxiliar por pares, sem evidência de necessidade neural geral.
- Etapa 09: por domínio, 4/4 decisões esperadas, contra 3/4 na consulta direta, 3/4 sem sinapses e 2/4 sem memória. Esses controles não têm a mesma capacidade inferencial; não justificam superioridade neural. A retenção 3/3 é isolamento de registros/contextos. Probes de transferência recebem suporte 0/3 antes e 3/3 depois de fornecer fatos do domínio de destino: reutilização estrutural, não transferência aprendida.
- A01.2: 16/16 por direção na composição conceitual; conceitos e sentidos manuais. O controle original verificava caminho para destino conhecido. A02.1 corrigiu a comparação com seletor cego: 48/48 decisões iguais e 32/32 evidências positivas equivalentes. A02.1-R1 preserva divergência de justificativa em uma das 32 combinações adversariais, sem orçamento comum de leituras; não declarar equivalência geral.
- A01.3/A01.4: seleção contextual e composição gramatical de inventários têm controles simbólicos equivalentes. Não demonstram aquisição de vocabulário, contexto ou estruturas novas.
- EC-02.4 e EC-03: aquisição/revisão de expectativa e manutenção de relações por regras explícitas, com recuperação literal forte. EC-03.5 reproduziu 124 testes e 500 verificações por painel; conformidade parcial. Esses resultados não validam uma capacidade de formar conceitos.
- A02.5: pesos zero e ausência de atividade pós-sináptica bloqueiam elegibilidade e atualização. Manutenção simbólica posterior não resolve esse limite nem demonstra aprendizagem neural geral.

### Lacunas e dependências

1. **Aquisição versus fornecimento:** há inserção de relações e composição sobre elas, mas falta evidência de aquisição do próprio conceito/tipo/regra a partir de exemplos. A mudança de um peso ou resumo não prova aquisição semântica.
2. **Propriedades e categoria:** não existe pipeline validado que extraia propriedades, construa categorias e aplique critérios adquiridos a novos itens. O rótulo Concept é estrutura, não demonstração desse processo.
3. **Tipos e exceções:** relações distintas precisam de semântica e composição explicitadas. Não há regra universal justificável para herdar dois passos entre todos os predicados. Ausência, evidência negativa e exceção não são intercambiáveis.
4. **Memorização e novidade:** recuperação por ID, troca de nomes, novos rótulos e recombinação de fatos conhecidos devem permanecer separados de novos itens por propriedades, novos padrões e estruturas não fornecidas. Os dados disponíveis não provam essa generalização.
5. **Proveniência/revisão:** engramas não têm revisão/retratação geral; o suporte EC-03 permanece experimental, com metadados parciais, revisão de regra incompleta e instrumentação limitada. Não promover esse suporte a infraestrutura validada integralmente.
6. **Contribuição neural:** fatos viram conexões com pesos configurados em parte dos ensaios; controles simbólicos explicam o resultado. Não atribuir aquisição de significado à propagação ou coatividade.

Preservar motor determinístico, tipos compactos, memória exata, distinção recall/inferência, relatórios com referências, tratamento de conflito e controles cegos. Não foi identificada necessidade de reconstrução arquitetural para concluir a auditoria. Qualquer uso futuro dessas interfaces dependerá da pergunta e do protocolo específicos, não de migração automática do experimento EC-03.

### Decisões necessárias somente para a subetapa seguinte

O plano não exige nova decisão científica para inventariar o que já existe; a presente auditoria está concluída. A segunda subetapa, **definição de tarefas com exemplos não cadastrados**, ainda não foi executada. Antes de propor seu protocolo posterior, será necessário distinguir: qual conteúdo se pretende adquirir (associação, categoria ou regra), qual informação já será fornecida e o que torna o exemplo novo. Recomenda-se escolher uma questão delimitada e comparável a controles de memorização, sem assumir superioridade de mecanismos neurais, simbólicos ou híbridos.

Não se escolhem aqui tarefa, exemplos, número de casos, limiares, algoritmo ou critério quantitativo de sucesso: isso anteciparia a segunda subetapa ou seu protocolo. A aceitação experimental da EC-04 completa continua pendente de futuras autorizações e evidências.

### Verificações e encerramento

Verificações desta entrega: branch/HEAD/árvore, leitura integral da definição EC-04 e Fundamento, inspeção de fontes/testes/resultados documentados, revisão do escopo do diff e whitespace. Nenhum teste, experimento cognitivo, implementação ou alteração de produção foi executado. A contagem de 124 testes é da reprodução anterior EC-03.5, não de nova execução.

**Conclusão da primeira subetapa:** auditoria concluída, com inventário rastreável e lacunas explícitas. Há base reutilizável de relações fornecidas e composição delimitada; formação progressiva adquirida de conceitos e relações permanece hipótese a investigar. EC-03 encerrada com ressalvas preservadas. Próximo passo previsto: definição de tarefas com exemplos não cadastrados, somente sob autorização própria. Planos, protocolos, código e testes permanecem inalterados.
