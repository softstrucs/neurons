# EC-06 — Conhecimento fornecido, adquirido e verificado

## Escopo e estatuto

Base `902909272c92b358e0fb5fdf4a4787e10688edae`, branch devs inicialmente limpa e sincronizada. O plano cognitivo v0.2, §5, identifica esta terceira subetapa como **separar conhecimento fornecido e adquirido**; a etapa EC-06 exige justificativas rastreáveis, avaliações independentes e reconhecimento de incerteza. Esta entrega estabelece critérios e uma matriz documental. Não é protocolo congelado, resultado de novo experimento ou aceite integral da EC-06.

Referências: [auditoria de inferência](EC-06_auditoria_inferencia_existente.md), [tarefas composicionais](EC-06_tarefas_composicionais.md), Fundamento aprovado e diretrizes A1–A8, decisões e auditorias EC-03/04/05. O encerramento dessas investigações como delimitadas, AJUSTAR e com ressalvas permanece. A8 é objetivo futuro, não evidência de competência já adquirida.

## Distinções e dimensões independentes

| Termo | Definição operacional | Limite da alegação |
|---|---|---|
| Informação fornecida | Conteúdo, rótulo, estrutura, regra, contexto ou feedback disponibilizado por fonte externa ou programado inicialmente; identificar responsável e momento. | Ser fornecida não determina se é verdadeira ou validada. |
| Representação armazenada | Estado que conserva uma informação ou referência para uso posterior, com identidade e escopo. | Alteração de armazenamento, por si só, não prova competência adquirida. |
| Recuperação literal | Consulta retorna conteúdo já armazenado por pista, chave ou correspondência. | Resposta correta por recuperação não é inferência ou aprendizagem de regra. |
| Composição/inferência | Transformação gera conclusão a partir de premissas e regra identificáveis, sujeita ao seu contrato. | Novidade da conclusão não prova aquisição da regra ou descoberta autônoma. |
| Aquisição simbólica de critério | Experiências modificam critérios, conjunto de hipóteses ou parâmetros simbólicos e alteram aplicação posterior. | Regras de atualização, atributos e família podem ter sido fornecidos; não implica semântica autônoma ou aprendizagem neural. |
| Aprendizagem efetivamente demonstrada | Mudança interna atribuível à experiência, com efeito posterior e controles adequados ao conhecimento ou competência alegados. | Uma resposta, memorizar um exemplo ou aplicar mecanicamente regra pronta não basta para competência cognitiva geral. |
| Descoberta | Produção de uma afirmação ou relação antes não fornecida, seguida de validação independente suficiente no escopo declarado. | Uma conclusão dedutível de regras fornecidas pode ser nova para a memória sem ser descoberta autônoma. |
| Verificação/validação | Verificação examina aderência ao contrato, referências e operações; validação confronta a alegação com evidências e critérios adequados ao uso. | Passar teste interno ou possuir EvidenceId não autentica fonte nem prova verdade externa universal. |

Usar quatro eixos simultâneos, não uma etiqueta exclusiva: **origem** (fornecida, observada, gerada/derivada ou desconhecida); **produção** (registro, recuperação, composição, atualização por experiência ou investigação); **validação** (não avaliada, apoiada no contrato, contrariada, inconclusiva ou validada no escopo); **incorporação** (não armazenada, armazenada, revisada, suspensa ou histórica). Estes valores descrevem funções conceituais; não definem enum, formato ou política universal. Um fato recebido pode ser armazenado e depois verificado; uma conclusão derivada pode permanecer não armazenada; um critério adquirido pode depois ser revisado e perder apoio. Registrar em cada eixo o que não é conhecido.

## Evidência mínima para aquisição

Uma alegação de aquisição de conhecimento ou competência específica exige, em conjunto:

1. Estado anterior observado, mostrando ausência do conhecimento/competência específica ou desempenho insuficiente; não presumir mente inicialmente vazia.
2. Experiências, informações fornecidas, supervisão, regras iniciais e processo de aquisição identificáveis; separar o que o professor já forneceu.
3. Alteração interna persistente entre consultas, rastreável à experiência e ao conteúdo/escopo; diferenciar registro episódico de mudança de critério ou parâmetro.
4. Efeito posterior sobre exemplos inéditos adequados à alegação, distinguindo itens, combinações e estruturas inéditas. Para competência cognitiva, mera recuperação do exemplo ou aplicação de regra pronta não basta.
5. Controles contra memorização, ausência de atualização, vazamento e explicações por regra fornecida, com mesma informação e oportunidades quando comparáveis; oráculo somente no avaliador.
6. Retenção após experiências intervenientes e revisão diante de evidência contrária, quando aplicáveis; explicitar interferência e resultados negativos. Não afirmar retenção de conhecimento que não foi adquirido.

O mínimo não define um limiar numérico universal nem garante generalização ampla. Se houver somente armazenamento e recuperação, classificar esses comportamentos; se houver atualização simbólica com aplicação reservada, atribuir aprendizagem delimitada aos elementos efetivamente modificados. Se o controle equivalente reproduzir o resultado, não alegar superioridade neural.

Para **descoberta autônoma**, exigir adicionalmente seleção efetiva de questão/intervenção pelo sistema dentro das permissões, sem destino correto ocultamente fornecido; cadeia rastreável entre observação, hipótese, teste e revisão; comparação com políticas fixas/aleatórias e recuperação com informação e orçamento equivalentes; validação independente da afirmação. Uma busca executada por instrução externa ou associação fornecida deve ser identificada como tal. Para **transferência entre domínios**, especificar domínios, pré-requisitos, conhecimento e codificações comuns, nova tarefa/itens/estrutura, controles sem experiência de origem e com igual acesso ao destino; demonstrar ganho atribuível à experiência de origem, retenção e limites. Renomeação, recombinação no mesmo domínio ou fatos novos fornecidos no destino não bastam.

## Mecanismos e evidências atuais

| Mecanismo | Origem, produção e incorporação | Evidência e classificação delimitada | Lacuna |
|---|---|---|---|
| `src/engram.rs`, EngramStore | Registros recebidos são conservados e recuperados por pista/ID; EvidenceId e status declarados. | Recuperação exata e ambiguidade existentes; armazenamento simbólico. | Qualidade da fonte, validade geral e aquisição de competência não decorrem do registro. |
| `src/cognition.rs`, TwoHopTask | Fatos externos viram conexões de peso fixo; consulta produz prova de dois fatos, sem gravar conclusão. | Inferência delimitada por regra programada; teste histórico 8/8 e simbólico equivalente 8/8. | Não aprende regra; uma prova, não raciocínio geral ou necessidade neural. |
| `src/metacognition.rs`, Monitor | Reavalia relatório e candidatos de contexto fornecidos, com abstenção programada. | Monitoramento e revisão da consulta no contrato. | Não descobre contextos; controle histórico de 5/7 não recebeu as mesmas alternativas do monitor 7/7. |
| `src/languages.rs` e módulos contextual/sentences | Associações, conceitos, sentidos e gramática fornecidos; recuperação e composição tipadas. | A01 demonstra recuperação/composição restritas; A02.1 adicionou controle cego. | Sem aquisição autônoma de significado, vocabulário ou estruturas novas; controle anterior A01.2 conhecia o destino esperado. |
| EC-04 `tests/support/ec04_candidate.rs` e painel | Episódios rotulados alteram critérios consistentes dentro de H fornecida; log/contexto conservados no ensaio. | Aquisição/revisão simbólica delimitada, com aplicação a vetores não adquiridos e controle de indução equivalente. | Não aprende H ou gramática de inferência; controles, proveniência histórica e instrumentação têm ressalvas da auditoria EC-04. |
| EC-02.4 `tests/support/expectation.rs` | Observações pareadas recebidas atualizam resumo de expectativa, separado dos episódios. | Aquisição local por regra programada; controle literal superou resumo em P4. | Não demonstra descoberta causal autônoma ou superioridade sobre recuperação episódica. |
| EC-03/05 `tests/support/provenance_*.rs`, `retention_*.rs` | Relatos, regras, versões e operações externas mantêm/recompõem provas e histórico local. | Revisão, abstenção e retenção simbólicas nos painéis delimitados; P2 respondeu antes de P1 em sondas previstas. | Não há History universal, credibilidade aprendida ou integração automática em produção; custos e memória são parcialmente instrumentados. |
| `src/learning.rs`, A01/A02 | Feedback externo altera pesos delimitados; circuitos efetivamente ativam em alguns ensaios. | Plasticidade e efeitos locais observados; A02.5 partindo de pesos zero produziu ausência de elegibilidade e aprendizagem. | Não demonstra aquisição de regra inferencial, compreensão ou superioridade neural; preservar resultados negativos e dependências externas. |

Os números acima são resultados históricos registrados, não novas execuções. Regras ou mecanismos fornecidos não se tornam adquiridos por uma saída correta; ao mesmo tempo, a atualização de critérios EC-04 é evidência de aprendizagem simbólica no escopo H, não deve ser reduzida a mera recuperação literal.

## Relação com as dez tarefas EC-06

| Tarefa | O que pode testar conforme catálogo | Aquisição e controles adicionais |
|---|---|---|
| EC06-R | Recuperação de associação cadastrada e ausência/ambiguidade. | Não demonstra aquisição cognitiva por si; medir registro/recall separadamente. |
| EC06-C2 | Inferência de dois fatos com regra fornecida e prova. | Testa composição, não aprendizagem da regra; para aquisição exigir estado anterior, experiências e controles de regra pronta/memória. |
| EC06-I | Abstenção por ausência, hipótese ou limite. | Não mede aquisição isoladamente; exigir teste posterior se houver mudança por experiência. |
| EC06-S | Compatibilidade com tipos e regra declarados. | Testa aplicação de contrato fornecido; aquisição de tipos exigiria experiências e validação independentes adicionais. |
| EC06-P | Seleção de caminho e prova suficiente. | Não prova aquisição por selecionar caminho existente; controlar enumeração simbólica e acesso equivalente. |
| EC06-X | Oposição, dependências e abstenção em perfil declarado. | Detecção/revisão programada não prova aprendizagem; distinguir bloqueio global e D1 local, controles equivalentes e evidência anterior/posterior. |
| EC06-K | Contextos, versões e continuidade declarada. | Testa uso de escopo fornecido; descoberta ou aquisição de contexto exige controles e intervenção próprios. |
| EC06-V | Validade, revisão e histórico. | Pode investigar efeito de atualização sobre estado/uso posterior; aquisição de competência requer controles de memorização, retenção e aplicação reservada. |
| EC06-M | Composição de múltiplas relações com regras explícitas. | Testa inferência programada quando implementada; aprendizagem da regra exigiria separar fornecimento, atualização e avaliação. |
| EC06-G | Aplicação formal a classes de novidade. | Pode avaliar efeito de conhecimento adquirido se houver processo e estado anterior controlados; regra pronta e renomeação não provam aquisição ou transferência entre domínios. |

Essas classificações não executam tarefas, escolhem conjuntos ou fixam critérios quantitativos. Mesmo EC06-V/G só podem sustentar aquisição se todas as condições mínimas acima forem verificadas; a definição atual, isoladamente, não fornece essa demonstração.

## Proveniência e controles necessários

Registrar, conforme o contrato de cada representação: identidade e origem da informação, fonte e natureza recebida/observada; premissas, regra e versões; operação ou experiência que produziu/modificou o estado; contexto, época, validade e corte; justificativas e evidências de validação; estado antes/depois, armazenamento e revisão; incerteza, abstenção e custos conhecidos/desconhecidos. Referências compartilhadas não comprovam independência das fontes; disponibilidade de origem não autentica conteúdo. Não impor um formato universal de memória ou alterar os contratos existentes nesta etapa.

Comparar memória literal e composição simbólica forte com os mesmos fatos, regras, contexto e oportunidades, declarando diferenças de capacidade; incluir ausência de aquisição, ausência de atualização, exemplos reservados e controles negativos. Ablação de circuito pode medir dependência local; não comprova necessidade neural geral. Separar aquisição/validação/final, não usar o oráculo como entrada e reservar novo final se dados finais forem usados para ajuste. Quando orçamento ou instrumentação não puderem ser verificados, relatar inconclusão ou limitação correspondente.

## Decisões abertas e conclusão

Permanecem para protocolo posterior: qual conteúdo/competência será adquirido; unidade de estado persistente; formas de novidade e domínio; equivalência de informação e intervenção dos controles; definição de métricas e limiares; retenção, revisão, fontes e orçamento. Alternativas simbólicas, neurais e híbridas permanecem abertas. Não se fixam algoritmo, arquitetura, dados, quotas ou protocolo experimental nesta entrega.

**Subetapa documental concluída.** A próxima subetapa nominal do plano é **testar raciocínio com evidências, controles e problemas não cadastrados**, ainda não iniciada. A aceitação integral da EC-06 permanece pendente. Não se demonstraram descoberta autônoma, transferência geral, compreensão, aprendizagem neural de regras ou AGI nesta atividade. Nenhum código, teste, experimento, protocolo histórico ou fundamento foi modificado.
