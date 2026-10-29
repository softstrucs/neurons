# EC-05 — Contexto, validade e revisão
## Primeira subetapa: modelar escopo de afirmações

### 1. Identificação, autorização e limite

Modelagem documental realizada na branch `devs`, base `051d021d9fa7d8ff681ff3bf2b82533492b3b9ea`, inicialmente limpa e sincronizada com origin/devs e a referência remota. Autorização: delegação “INÍCIO DA EC-05 — PLANO EXECUTIVO v0.2”, restrita à primeira subetapa, com documentação, verificações e versionamento.

Identificação literal no plano v0.2, §5: **EC-05 — Contexto, validade e revisão**. No mapa §4, a forma resumida é “EC-05 — Aprendizagem contextual”; seu objetivo é **reconhecer validade, exceções e mudança**, dependências **EC-03/04**, entregável mínimo da etapa **testes de contexto e revisão**, portão **robustez contextual demonstrada**. Essas duas denominações pertencem à mesma etapa, não a etapas distintas.

A ordem oficial é: **modelar escopo de afirmações**; testar mudança ambiental e conflito; definir política de revisão e abstenção; avaliar retenção sem misturar versões de contexto. O aceite da etapa completa é “conclusões revisadas mediante evidência, identificação de exceções e ausência de extrapolação indevida”. O plano não numera individualmente essas subetapas nem define seus arquivos. Este relatório é o entregável documental da primeira, não os testes ou o aceite experimental da etapa completa.

Questão desta subetapa: quais condições precisam acompanhar uma afirmação para delimitar onde, quando e com quais evidências ela se aplica? A entrega é uma modelagem conceitual rastreável à infraestrutura, **não um novo esquema Rust, protocolo executável, arquitetura obrigatória ou política de decisão**. Não se escolhem limiares, quotas, casos finais, regras de resolução de conflito ou de atualização. Subetapas posteriores não foram iniciadas.

### 2. Referências e dependências efetivas

Lido integralmente `neurons_plano_executivo_evolucao_cognitiva_v0_2.md`. Consultados Fundamento aprovado em `docs/fundamentos/`, `docs/decisions.md`, relatórios EC-03.1, EC-03.2 e EC-03.5, relatório EC-04 de implementação e parecer `EC-04_auditoria_cientifica_final.md`. Inspecionados contratos de engramas, cognição, monitoramento, eventos, persistência, contexto lexical e suportes experimentais de proveniência/indução.

A autorização atual considera EC-04 encerrada como investigação delimitada, AJUSTAR e conformidade parcial. EC-03 permanece encerrada com as mesmas qualificações documentadas. Os textos históricos dos planos/relatórios não são reescritos para atualizar estados passados; a decisão atual é registrada aqui e em decisões.

| Dependência | Tipo e evidência disponível | Consequência para a primeira subetapa |
|---|---|---|
| Fundamento, princípios 4, 5, 8 e 9 | Diretriz aprovada: transformação por experiência, discrepância, aplicação contextual e confiabilidade. | Orienta as distinções; não é prova de mecanismo ou competência. |
| EC-03: representações e memória | Inventário e comparação de múltiplas representações; protótipo com contexto, validade, versões e referências. | Permite mapear conceitos de escopo sem impor uma unidade universal. Não fornece proveniência geral integralmente validada. |
| EC-03.5: manutenção e limites | Painel histórico de 500 verificações e 124 testes; literal superior nas sondas pré-Refresh S1/S5. History, fontes, revisão de regra e instrumentação parciais. | Usar como referência delimitada, sem pressupor integração em produção ou superioridade de resumo. |
| EC-04: aquisição/revisão contextual | Auditoria reproduziu 132 testes, 14560 consultas finais, 3120 validações, 280 eventos e 12 manifestos. T-A/T-V simbólicas, controles equivalentes no painel. | Sustenta que escopo/contexto e revisão podem ser distinguidos no laboratório; não demonstra descoberta do contexto ou validade fora da família. |
| Competências ainda ausentes | Proveniência histórica completa, retenção em parâmetros compartilhados, transferência estrutural e formação autônoma de conceitos não demonstradas. | Não bloqueiam a descrição conceitual. Bloqueiam alegações futuras que dependam delas sem validação própria. |

Os números desta tabela são históricos, consultados nos relatórios, **não reexecutados nesta subetapa**. O encerramento de EC-03/04 não elimina suas não conformidades nem satisfaz automaticamente o portão experimental da EC-05.

### 3. Modelo conceitual do escopo

“Afirmação” designa aqui um conteúdo que pode ser consultado e justificado: uma associação registrada, relação, hipótese, conclusão derivada ou predição sob critério. Não exige armazenar todas essas representações em um único tipo. Um episódio pode evidenciar uma afirmação; um critério pode gerar várias; uma competência exige avaliações próprias e não decorre do registro de uma afirmação.

**Escopo declarado** é o conjunto de condições às quais o registro se refere segundo sua origem ou regra. **Escopo sustentado** é a parte para a qual existem evidências e transformações verificáveis. Receber um rótulo ou contexto declarado não comprova que o sistema tenha adquirido essa delimitação. Não ampliar automaticamente o segundo ao primeiro nem inferir validade universal por falta de metadados.

As dimensões abaixo são uma lista de informações a distinguir na análise, não campos de uma API já aprovada. Cada representação pode referenciá-las por registros distintos; um contrato futuro deverá declarar o que suporta e o que fica desconhecido.

| Dimensão | Significado e separação necessária | Correspondência existente / lacuna |
|---|---|---|
| Conteúdo e identidade | O que está sendo afirmado: associação, predicado/relação, argumentos, sentido e polaridade quando definidos. Identidade do registro não é verdade do conteúdo. | EngramId e Cue identificam associação; fatos EC-03 têm extremos e polaridade. Não há predicado universal aprendido. |
| Origem e natureza | Observação, informação recebida, hipótese ou transformação programada; professor e pré-processamento devem ser identificáveis. | RecordStatus distingue alguns estados; EC-03 usa observation/source/hypothesis, mas não separa integralmente Observed/Received. |
| Contexto e interpretação | Condições pertinentes à aplicação: ambiente/tarefa/sentido e convenção dos símbolos. Um ID é referência a essa descrição, não a descrição em si. | Cue.context e Query.context filtram igualdade; contextos lexicais são fornecidos. Descrições e espaços de nomes não são uniformes. |
| Versão da representação | Qual conteúdo/critério está em uso e de qual anterior difere. Duas versões do mesmo objeto não são dois contextos automaticamente. | Key(id,version) EC-03; EC-04 conta episódios e mantém sufixo atual, sem API histórica integral. |
| Estado do contexto | Qual configuração/interpretação do ambiente é pertinente. Não confundir com versão de fato, contador de episódio ou nome do laboratório. | Não há objeto geral de contexto versionado. O mesmo contexto A pode receber mudança de critério em EC-04. |
| Tempo da observação | Quando o evento/evidência foi obtido ou registrado, com domínio de relógio declarado. | Engram.observed_at; não define duração de validade nem autentica data. |
| Validade de aplicação | Em que condições temporais/ambientais se declara aplicável o conteúdo. Pode ser delimitada, não informada ou inaplicável à representação. | EC-03 tem start/end e epoch; produção não oferece intervalo geral de validade. |
| Corte do conhecimento | Quais registros já estavam disponíveis ao responder. Difere de “para qual instante do ambiente se consulta”. | EC-03 separa cut do diário e Query.epoch; replay é limitado, não recuperação histórica completa por objeto. |
| Suporte e transformação | Quais evidências/fontes/versões de regra sustentam a afirmação e quais dependências foram utilizadas. | Proof EC-03 referencia regra e fatos; TwoHopTask referencia dois engramas. Existência de ID não verifica a qualidade da evidência. |
| Estado e motivos | Situação do suporte: disponível, antigo, suspenso, retirado, expirado, conflitante ou desconhecido, conforme a representação. | Estados existem em componentes distintos; nem todos os motivos são preservados nos registros EC-04. Não constituem política geral. |
| Condições de observação e limites | Família, atributos, forma de consulta, orçamento e condições sob as quais a capacidade foi avaliada. | Devem acompanhar a interpretação científica; resultados de laboratório não autorizam extrapolação para conteúdo/domínio não fornecido. |

A modelagem conserva separadas quatro perguntas: **qual conteúdo foi armazenado?**, **qual interpretação contextual é pertinente?**, **que suporte estava disponível e em qual versão?**, **para qual condição se pretende aplicar a conclusão?** Uma resposta correta à primeira não resolve as outras.

### 4. Identidade, tempo, validade e mudança

**Contexto não é sinônimo de verdade.** IDs iguais só permitem comparação dentro de uma convenção identificada; IDs diferentes não provam incompatibilidade semântica, e igualdade numérica entre módulos não prova identidade de contexto. Não presumir hierarquia, herança, tradução de contexto ou similaridade entre Finance/Furniture e contextos opacos EC-03/04.

**Ausência de escopo não significa escopo irrestrito.** Distinguir metadado ausente de declaração explícita de abrangência. Saber que uma condição difere daquela documentada delimita a aplicabilidade; não demonstra que a afirmação oposta seja verdadeira. A modelagem admite informação incompleta sem definir qual ação o sistema deverá escolher: essa política pertence à terceira subetapa.

**Tempo de evento, observação e validade têm responsabilidades diferentes.** O relógio lógico ordena execução; observed_at registra uma observação; start/end delimitam aplicação no laboratório EC-03. Não existe conversão automática para tempo civil ou entre relógios de ensaios. EC-03 usa start <= epoch < end; isso descreve seu contrato histórico, não fixa por si só a convenção temporal de um futuro experimento EC-05.

**Versão de representação, de contexto, de regra/família e de arquivo não são intercambiáveis.** O VERSION=1 de persistence.rs identifica formato binário. O version de uma resposta EC-04 é contagem de episódios; não identifica sozinho todas as versões do critério. Um checkpoint válido não comprova validade semântica corrente nem preserva os protótipos experimentais EC-03/04.

**Corte e instante consultado são eixos distintos.** Consultar o que era justificável com um diário antigo não deve usar silenciosamente evidências posteriores. Consultar hoje uma condição passada também não é o mesmo que recuperar a decisão realmente emitida naquela ocasião. O replay EC-03 informa parte dessa distinção, sem resolver toda a recuperação histórica.

**Mudança ambiental não é descoberta de novo contexto por renomeação.** Revisar o conteúdo sob o mesmo identificador é diferente de registrar outro contexto; preservar versões permite descrever essa diferença. Na EC-04 o professor muda o critério em A e o modelo ajusta o sufixo, enquanto B tem estado separado. Isso não demonstra que o modelo detecte sozinho quais propriedades ambientais definem um contexto.

### 5. Associação, composição, contradição e exceções

Uma conclusão derivada deve conservar a associação entre seu suporte e as condições em que cada premissa e regra é pertinente. Uma cadeia não autoriza ampliar escopo além do justificado por suas premissas e transformação. Esse é um limite de interpretação, não aprovação de um algoritmo de propagação de validade.

Suportes alternativos devem permanecer distinguíveis: a perda de uma via não implica, por definição, a perda de outra ainda documentada. Tampouco se deve combinar fragmentos de contextos ou versões diferentes para fabricar uma prova. Uma operação futura de composição precisará especificar como verifica compatibilidade de contextos, períodos e versões; essa semântica não é oferecida genericamente pelo TwoHopTask atual.

| Distinção documental | Interpretação correta | O que permanece para as subetapas posteriores |
|---|---|---|
| Mesmo conteúdo em contextos distintos | Pode ter condições de aplicação diferentes; não é contradição automática. | Verificar experimentalmente aplicação e não mistura, com protocolo próprio. |
| Conteúdos opostos em condições comparáveis | Possível conflito de evidências/afirmações, com fontes e versões identificáveis. | Especificar comparação, precedência e resposta; não se escolhe fonte vencedora aqui. |
| Registro anterior sem validade corrente | Pode ser recuperável historicamente sem sustentar resposta atual. | Política de suspensão/revisão/abstenção e garantias de recuperação. |
| Exceção declarada | Restrição adicional ao alcance de uma afirmação sob condições identificadas. | Distinguir recebimento de exceção pronta de sua identificação por experiência. |
| Revisão de critério | Mudança da representação usada após novas evidências. | Avaliar transformação, retenção e efeito comportamental sem vazamento. |
| Falta de evidência ou contexto não descrito | Informação insuficiente sobre aplicabilidade, não confirmação nem refutação universal. | Política de investigação/abstenção e instrumentos que preservem motivo. |

Não se determina aqui uma prioridade geral entre conflito, obsolescência, expiração, limite e ausência. EC-03 já tem precedência programada no laboratório; EC-04 colapsa alguns motivos nos logs. Nenhuma delas é promovida automaticamente à política EC-05. Também não se define armazenamento de exceções, detecção de mudança, seleção de experiências ou comportamento metacognitivo novo.

### 6. Mapeamento técnico e reutilização delimitada

| Componente inspecionado | Reutilização possível no escopo | Limite que deve acompanhar seu uso |
|---|---|---|
| `src/engram.rs`: Cue:58, Engram:120, recall:222 | Identidade, consulta por contexto e associação, tempo/origem/evidência/status. | Igualdade exata; sem revisão/retratação ou intervalo temporal gerais. Validated exige referência, não prova a validade contextual. |
| `src/cognition.rs`: Hypothesis:37, Proof:56, TwoHopTask | Consulta contextual e ligação entre premissas de composição fornecida. | Não decide validade temporal; pesos são construídos dos fatos, não aprendidos; relação e regra exigem convenção externa. |
| `src/metacognition.rs`: Monitor::review:113 | Registro de tentativas e consulta de contextos candidatos declarados. | Revisar a consulta entre candidatos fornecidos não equivale a revisar uma afirmação/memória ou descobrir contextos. |
| `src/languages/contextual.rs`: Context e ContextLexicon | Exemplo de contexto semântico enumerado com evidência lexical. | Rótulos, sentidos e vínculos externos; sem compreensão geral ou aquisição automática de contexto. |
| `tests/support/provenance_types.rs`: Key:3, Fact:17, Rule:31, Query:86, Answer:93 | Referências versionadas, polaridade, intervalos, corte e motivos em laboratório. | Metadados de fonte e History parciais; não é contrato de produção ou esquema universal aprovado. |
| `tests/support/provenance_candidate.rs`: reasons:26, query:385 | Verificação local de validade/estado de suporte e replay por corte. | Regras e precedência programadas; dependência de Refresh, revisão de regra e instrumentação com ressalvas EC-03.5. |
| `tests/support/ec04_candidate.rs`: Context:4, query:111 | Separação de estados por contexto, versão por aquisição e sufixo ativo. | Sem dimensão temporal de validade, API histórica integral ou versão explícita da família na resposta; não é memória contextual geral. |
| `src/event.rs`, `src/engine.rs` | Ordenação determinística e orçamento de eventos. | Relógio de execução não fornece, sozinho, validade de afirmações ou história semântica. |
| `src/persistence.rs`: VERSION:16 | Preservação do motor/engramas no formato existente. | Não salva automaticamente os protótipos nem fornece versão contextual; nenhuma ampliação do checkpoint foi autorizada. |

Não se identificou necessidade de substituir infraestrutura para concluir a modelagem. Permanece aberta a escolha entre referências distribuídas entre representações, metadados associados ou projeções reconstruídas de registros conservados. A presente entrega não seleciona uma dessas arquiteturas nem cria um tipo universal de conhecimento.

### 7. Ressalvas herdadas e consequências

| Ressalva preservada | Efeito sobre a EC-05 |
|---|---|
| Literal T-C incompleto e validação de prova por presença (EC-04 NC-01) | Não usar o desempenho desse controle como comprovação de recuperação contextual forte ou de fiscalização independente de validade. |
| Contexto/versão T-R e proveniência histórica incompletos (NC-02/03; EC-03 History parcial) | Não presumir que o histórico por versão já esteja disponível para todas as afirmações. Uma investigação que dependa dele precisa de contrato e verificação próprios. |
| Motivos de abstenção e hipóteses incompletos nos registros (NC-04) | Não tratar abstenções diferentes como mesmo diagnóstico nem declarar equivalência integral de evidências. |
| Contagem de trabalho e memória parciais (NC-06/07; EC-03.5) | Não comparar eficiência, orçamentos ou segurança fina como se os contadores fossem completos. Limites futuros precisam de instrumentação prospectiva. |
| Ordem global de repetições e custo do ramo congelado divergentes (NC-05) | Determinismo local não demonstra conformidade de toda sequência ou igualdade de oportunidades; declarar o procedimento realmente executado. |
| Controles com pressupostos compartilhados | Independência de código não é independência de família, política, dados, avaliador ou equipe. Concordância não prova superioridade neural. |
| B estável com estado separado | Conservação contextual demonstrada no painel, não resistência a interferência em parâmetros compartilhados. |
| Literal forte EC-03 mais pronto pré-Refresh | Contexto e validade precisam ser examinados sobre evidências válidas, sem exigir artificialmente vitória do candidato ou concordância com cache obsoleto. |

Essas ressalvas afetam alegações e desenho futuros, mas não exigem corrigir código histórico para descrever o que “escopo” significa. A primeira subetapa depende de referências e mecanismos inspecionáveis, não de uma competência geral que EC-03/04 não demonstraram. Caso uma etapa posterior dependa dessa competência, a lacuna deve ser tratada explicitamente antes de executá-la.

### 8. Compatibilidade conceitual e conclusão

Princípio 4: distingue mudança adquirida de metadado fornecido. Princípio 5: explicita discrepância/lacuna sem conceder investigação autônoma. Princípio 8: aplicação fica vinculada às condições e suportes; acerto em um contexto não autoriza extrapolação. Princípio 9: conserva proveniência, separação entre avaliação/aquisição e limitações dos controles. Essa compatibilidade é documental, não nova demonstração científica.

**Primeira subetapa concluída:** modelagem do escopo com dimensões distintas, correspondência à infraestrutura, lacunas e limites de reutilização. Não se declara satisfeito o aceite da EC-05 completa. Nenhuma política operacional ou regra de experimento ausente precisou ser escolhida para concluir este trabalho.

Próxima subetapa literal: **testar mudança ambiental e conflito**, ainda não iniciada. O plano não é protocolo executável: antes de eventual execução será necessário ter estados, operações, partições, controles, métricas, ordem, regras e limites aprovados, sem inventá-los nesta entrega. A política geral de revisão/abstenção e a avaliação de retenção continuam nas posições posteriores do plano; não se antecipam aqui.

### 9. Verificações e artefatos

Conferidos branch/base, árvore limpa inicial, sincronização remota, plano integral, Fundamento e dependências, referências de arquivos/símbolos, distinção de tipos de versão, escopo documental, links locais e whitespace. Foram conservados hashes iniciais dos arquivos rastreados para conferir que somente o acréscimo autorizado em decisões os modifica.

Não foram executados testes ou experimentos cognitivos nem Cargo nesta subetapa exclusivamente documental. Os **132 testes** citados são da auditoria EC-04 anterior; não são resultado novo. Não se mediu RSS experimental inexistente. `target/` permaneceu com 55.415.100 bytes; espaço observado em /home antes da escrita documental: 4.632.883.200 bytes, acima do mínimo de 3 GiB.

Artefatos: somente este relatório e acréscimo em `docs/decisions.md`. Código, testes, Fundamento, planos e protocolos históricos preservados. Commit/push documentais autorizados nesta delegação; hash e confirmação final do Git são fornecidos após o versionamento, sem autorreferência neste documento.
