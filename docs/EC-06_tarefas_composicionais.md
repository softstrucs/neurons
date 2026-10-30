# EC-06 — Tarefas composicionais

## Escopo e estado

Base `8978ecd7869af5a38e2be2083f49fc54abc1ee0e`, devs inicialmente limpa e sincronizada. Esta especificação atende à subetapa **definir tarefas composicionais** do plano cognitivo v0.2. É catálogo prospectivo, não protocolo congelado, implementação ou resultado experimental. A [auditoria inicial](EC-06_auditoria_inferencia_existente.md), as auditorias EC-03/04/05 e os fundamentos A1–A7 permanecem referências preservadas. O aceite integral da EC-06 continua pendente.

Os níveis abaixo organizam exigências formais, não afirmam dificuldade psicológica nem progressão pedagógica comprovada. Cada tarefa recebe fatos, escopo e regras declaradas pelo ambiente. Referências a evidências são dados disponíveis, não prova da verdade de fontes reais.

## Contrato comum e provas

Usar símbolos abstratos e relações tipadas. No perfil homogêneo local, a regra explicitamente fornecida `ρ2: L(a,b) ∧ L(b,c) ⇒ L(a,c)` aplica-se somente aos tipos, contexto, versão e intervalo declarados; não pressupõe transitividade de qualquer relação arbitrária. Para relações distintas P e Q, uma conclusão R exige regra explícita `ρPQ: P(a,b) ∧ Q(b,c) ⇒ R(a,c)` e compatibilidade de papéis. Sem essa regra, não se infere R, equivalência ou causalidade. A escolha de perfis para um ensaio posterior permanece aberta.

Uma prova válida deve identificar consulta e escopo, regra e versão quando aplicável, premissas e suas versões, origem/observação, relações e argumentos, contexto, validade e dependências. O avaliador deve verificar conectividade, tipo, polaridade, validade, oposição relevante e suficiência. Prova circular, referência inexistente, premissa hipotética não confirmada ou dependência inválida não sustentam resposta. Uma resposta coincidente sem prova adequada conta separadamente como conclusão sem justificativa válida.

`Supported` indica suporte segundo o contrato; ausência de suporte não prova falsidade no mundo. Abstenção deve conservar motivo, como ausência, ambiguidade, conflito, versão/validade ou limite. Erro de entrada e falha técnica não se confundem com ausência. Um limite previsto pode ser resposta inconclusiva adequada do mecanismo; impossibilidade de fiscalizar o painel torna a avaliação inconclusiva. Consulta não fornece ao mecanismo o gabarito nem atualiza sua memória.

## Catálogo de tarefas

As previsões abaixo são condicionais aos fatos e contratos declarados, sem fixar dados finais, quotas, sequências de execução ou limiares de aceitação.

### EC06-R — Recuperação direta, nível 0

- **Objetivo e dados:** recuperar uma associação única explicitamente cadastrada; incluir consulta ausente e pista ambígua. Conhecimento prévio permitido: registros, contexto e chave declarados.
- **Operações e resultado:** consulta literal somente leitura; associação única retorna conteúdo e referência; ausência ou múltiplos registros produzem abstenção qualificada.
- **Prova e falsificação:** referência exata ao registro e escopo; responder com registro inexistente, escolher arbitrariamente na ambiguidade ou alterar memória é erro. Orçamento insuficiente é inconclusão, não ausência.
- **Limite:** resposta é recuperação, mesmo quando correta. `EngramStore::recall` cobre igualdade e ambiguidade, sem validar por si só a qualidade da evidência ou sua validade temporal.

### EC06-C2 — Composição de duas relações, nível 1

- **Objetivo e dados:** derivar uma conclusão não cadastrada a partir de duas premissas admissíveis e regra `ρ2` fornecida. Consulta identifica a proposição, não sua classificação esperada; o mecanismo recebe os mesmos fatos e contexto dos controles.
- **Operações e resultado:** aplicar somente a regra declarada; retornar Supported quando houver caminho válido, sem exigir aresta direta da conclusão.
- **Prova e falsificação:** dois IDs de premissas, intermediário e referência da regra/contexto; resposta sem ligação, com hipótese como fato ou baseada em conclusão cadastrada indevidamente falha. Ausência de premissa exige abstenção; limite excedido é inconclusão.
- **Limite:** demonstra aplicação da regra fornecida. `TwoHopTask` cobre composição homogênea e retorna uma prova de dois fatos, mas não identifica versão da regra nem verifica validade temporal geral; fiscalização adicional exigiria autorização própria.

### EC06-I — Premissas insuficientes, nível 1

- **Objetivo e dados:** diferenciar ausência de caminho, hipótese não confirmada e orçamento insuficiente; fornecer regra e registros parciais, inclusive negativos explícitos quando declarados.
- **Operações e resultado:** consultar evidências disponíveis sem completar premissas por suposição. Sem prova admissível, abster-se; hipótese isolada não confirma conclusão e ausência não produz negação.
- **Prova e falsificação:** registrar escopo, motivo e referências disponíveis. Resposta sustentada sem todas as premissas é erro; esgotamento de orçamento não pode ser classificado como ausência comprovada.
- **Limite:** `TwoHopTask`/`Monitor` distinguem alguns casos existentes, mas Unsupported não determina verdade externa nem explica todas as causas.

### EC06-S — Compatibilidade semântica de relações, nível 2

- **Objetivo e dados:** distinguir composição autorizada de encadeamento com tipos ou papéis incompatíveis. Fornecer assinaturas P, Q e R e, separadamente, presença ou ausência de `ρPQ`.
- **Operações e resultado:** aplicar uma regra somente quando seus papéis e escopo coincidirem. Com premissas e regra compatíveis, sustentar com prova; sem regra ou com papéis incompatíveis, abster-se.
- **Prova e falsificação:** referências às assinaturas, regra e premissas; tratar igualdade de IDs intermediários como autorização semântica é erro. Metadados incompletos ou verificação de tipos indisponível exigem abstenção ou inconclusão declarada.
- **Limite:** o núcleo genérico não representa predicados tipados gerais; os léxicos possuem papéis locais. Uma fiscalização tipada para a tarefa exigiria trabalho futuro autorizado.

### EC06-P — Caminhos alternativos, nível 2

- **Objetivo e dados:** verificar uma proposição consultada com mais de um caminho; incluir caminho incompleto ou inelegível e alternativa completa, usando as mesmas evidências nos controles.
- **Operações e resultado:** buscar suporte sem eleger por ID. Uma prova válida e suficiente pode sustentar a proposição; múltiplas provas da mesma conclusão não são automaticamente conclusões contraditórias. Não alegar enumeração completa se apenas uma prova foi examinada.
- **Prova e falsificação:** registrar o caminho efetivamente usado e suas dependências; usar caminho incompleto/inválido ou omitir limite que impeça decidir é erro. Se não se puder demonstrar suporte suficiente, abster-se.
- **Limite:** `TwoHopTask` retorna a primeira prova encontrada; não fiscaliza todas as alternativas. Seleção de uma conclusão desconhecida entre candidatos é modalidade distinta, ainda pendente de escolha; nesta modalidade a proposição consultada é entrada, não gabarito de sua validade.

### EC06-X — Oposição e isolamento de conflitos, nível 2

- **Objetivo e dados:** contrastar oposição admissível à conclusão, oposição a uma premissa, ausência de alternativa e alternativa com dependências rastreáveis. Manter evidências positivas e negativas e suas origens.
- **Operações e resultado:** oposição direta à conclusão, ou falta de suporte independente e suficiente, exige abstenção. Para alternativa limpa, comparar perfis explicitamente identificados: bloqueio global EC-03.3-R1 e isolamento D1 do laboratório EC-05; no perfil D1, só responder quando separação e suficiência estiverem demonstradas.
- **Prova e falsificação:** expor oposição e suporte usado; resposta com dependência comprometida ou resolução por ID/recência sem regra autorizada falha. Independência desconhecida exige abstenção; fonte ou regra comum comprometida não é alternativa independente.
- **Limite:** os perfis têm expectativas diferentes e não são política universal da EC-06. `Monitor` verifica a prova selecionada e não implementa enumeração geral de alternativas nem D1; os laboratórios EC-05 são isolados e seus estados de comprometimento são fornecidos externamente.

### EC06-K — Contextos e versões, nível 2

- **Objetivo e dados:** distinguir contextos diferentes de versões sucessivas de um mesmo contexto. Fornecer identidades, sucessão e aplicabilidade explicitamente declaradas.
- **Operações e resultado:** compor somente premissas e regras aplicáveis à configuração consultada. Sem continuidade declarada, não transportar evidência antiga; consulta histórica preserva corte e época anteriores.
- **Prova e falsificação:** identificar contexto lógico, versão, corte, regra, referências e declarações de continuidade. Misturar versões ou apresentar história como validade corrente é erro; aplicabilidade indeterminada exige abstenção.
- **Limite:** filtros de contexto existem no núcleo; sucessão e continuidade D2 existem no laboratório EC-05, não como contrato geral de produção. Consultas pontuais não demonstram retenção neural.

### EC06-V — Validade, revisão e histórico, nível 2

- **Objetivo e dados:** investigar evidência válida, expirada, suspensa ou revisada, com época, intervalo, versão, operação de manutenção e corte histórico explícitos.
- **Operações e resultado:** usar apenas suporte vigente em `[início,fim)`; conservar referência histórica. Se não houver alternativa válida, abster-se; uma revisão autorizada pode sustentar resposta posterior conforme o perfil declarado.
- **Prova e falsificação:** registrar versão, fonte, intervalo e transformação; suporte expirado na consulta corrente ou conhecimento posterior no corte antigo é erro. Ausência de história acessível deve ser declarada, não reconstruída como registro original.
- **Limite:** laboratórios EC-03/05 oferecem replay delimitado. Diferenças entre P1 pré-Refresh e recomposição P2 são medidas segundo seus contratos, sem classificá-las automaticamente como erro. O núcleo não possui validade temporal geral.

### EC06-M — Múltiplas relações, nível 3

- **Objetivo e dados:** investigar prova com mais de duas premissas sem assumir que a regra de dois passos pode ser repetida indiscriminadamente. Fornecer explicitamente uma regra de maior aridade ou regras intermediárias tipadas com seus escopos.
- **Operações e resultado:** quando o contrato futuro autorizar a composição e todas as dependências estiverem disponíveis, sustentar com uma árvore de prova completa; sem regra, suporte ou orçamento, abster-se com motivo apropriado.
- **Prova e falsificação:** listar cada premissa, intermediário e regra usada; caminho longo sem regra correspondente, prova circular ou elo ausente não sustentam conclusão. Ativação do destino isolada não é prova.
- **Limite:** o teste existente de `TwoHopTask` retorna Unsupported para caminho de três passos sem prova de dois, mesmo com ativação do destino. Ele não executa esta capacidade geral; regra de maior aridade versus encadeamento tipado permanece decisão para protocolo posterior.

### EC06-G — Novidade dentro do domínio formal, nível 4

- **Objetivo e dados:** avaliar aplicação da regra declarada a consultas não cadastradas, distinguindo renomeação de IDs, recombinação de fatos conhecidos, entidades novas com fatos fornecidos e topologias novas ainda cobertas pelo mesmo contrato.
- **Operações e resultado:** aplicar apenas regras e evidências disponíveis; para situação sem suporte ou regra autorizada, abster-se. Não inserir respostas finais como associações diretas nas tarefas de composição.
- **Prova e falsificação:** avaliar conclusão e prova em cada classe de novidade; resposta correta sem prova não comprova generalização justificada. Sem partições independentes ou orçamento verificável, resultado é inconclusivo.
- **Limite:** aplicação de regra fornecida a novos exemplos formais não demonstra aprendizagem da regra, vocabulário novo, compreensão ou transferência estrutural. Quais classes serão reservadas e como serão geradas ficam para protocolo posterior.

## Matriz de comparação

| Tarefa | Capacidade e evidências | Controle comparável | Resultado esperado / falsificação | Limitação |
|---|---|---|---|---|
| EC06-R | Recuperação e registro exato | Busca literal independente com os mesmos registros | Uma referência única ou abstenção; ID errado/mutação falsifica | Não é inferência. |
| EC06-C2 | Dois fatos e regra declarada | Composição simbólica independente, mesmos fatos/contexto | Prova de dois elos; falta de elo/regra invalida | Núcleo não registra versão geral da regra. |
| EC06-I | Ausência, hipótese e orçamento | Fiscal independente com mesmas entradas e limites | Abstenção qualificada; conclusão sem suporte falsifica | Falta não é falsidade. |
| EC06-S | Tipos, papéis e regras explícitas | Fiscal tipado independente com mesmas assinaturas | Regra compatível ou abstenção; encadeamento não autorizado falha | Tipagem geral não existe no núcleo. |
| EC06-P | Prova válida entre caminhos | Enumeração simbólica equivalente | Sustentar com via válida ou abster; via inválida falha | Uma prova não mostra enumeração completa. |
| EC06-X | Oposição e dependências | Bloqueio global e D1 em perfis identificados | Abstenção ou via separável conforme perfil; ignorar conflito direto falha | Monitor não implementa D1 geral. |
| EC06-K | Identidade, sucessão, continuidade | Replay simbólico com mesmo diário e declarações | Sem mistura/herança implícita; prova cruzada falha | D2 é laboratório isolado. |
| EC06-V | Tempo, revisão e histórico | Recomposição P2 com mesmas evidências | Suporte válido ou abstenção; uso de evidência expirada falha | P1/P2 podem divergir legitimamente. |
| EC06-M | Cadeias maiores com regra explícita | Composição simbólica equivalente sob o mesmo contrato | Prova completa; elo/regra ausente falha | Capacidade de múltiplos passos ainda não demonstrada no núcleo. |
| EC06-G | Novidade formal e justificativa | Literal e simbólico com mesmos dados disponíveis | Medir por classe; coincidência sem prova não basta | Novidade formal não prova aprendizagem da regra. |

## Controles, partições e métricas preliminares

Controle literal recebe todos os registros permitidos, mas só recupera associações exatas; sua ausência de resposta em conclusão derivada não demonstra superioridade neural. Controle simbólico forte recebe os mesmos fatos, regras, contextos, versões e candidatos e decide independentemente, sem chamar a lógica do candidato. Ablação de sinapses, quando houver circuito, altera a propagação e demonstra dependência local, não necessidade neural geral. Controles sem aquisição, sem manutenção e respostas fixas são diagnósticos de informação ou comportamento, não concorrentes com capacidade equivalente.

Controles negativos devem abranger falta de premissa, prova fabricada, conflito direto, alternativa dependente comprometida, contexto/versão trocados, expiração, tipo incompatível, hipótese não confirmada e limite. Não reduzir fatos ou oportunidades do controle forte. Registrar diferenças de capacidade, orçamento efetivo, arquitetura e pressupostos compartilhados; independência de código não implica independência de dados, equipe ou avaliador.

Separar aquisição, validação e avaliação final antes da execução futura. Nas tarefas de composição, não cadastrar respostas finais como pares completos nem entregar ao mecanismo/controlador o destino correto, índice de resposta, categoria oculta ou gabarito. Para verificação de proposição, os argumentos da consulta são entrada legítima; sua classificação e prova esperadas pertencem ao avaliador. Para recuperação direta, a associação consultada necessariamente pode ter sido cadastrada: classificar esse caso como recuperação. Se dados finais forem usados para ajuste, requerem nova reserva antes de avaliação posterior.

Métricas propostas, sempre com denominadores por tarefa, classe, condição e escopo: correção da conclusão; provas válidas por respostas sustentadas; respostas indevidas; falsas abstenções; abstenções adequadas por motivo; falhas de detecção de conflito; resultados inconclusivos/erros; cobertura; concordância de decisões e de evidências; repetibilidade de registros; custos de aquisição, manutenção e consulta, tempo, dados vivos e RSS com método declarado. Precisão entre respostas é indefinida quando não há resposta. Reportar novidade por renomeação, recombinação, entidade e estrutura separadamente. Não pontuar uma abstenção correta como prova de conclusão positiva ou negativa.

Limites exatos, unidades de trabalho uniformes, conjuntos, quotas, ordem, controles executáveis e critérios quantitativos de sucesso/falha deverão ser congelados em protocolo próprio antes de qualquer experimento. Os tetos locais históricos de 3 GiB livres em /home e RSS isolado inferior a 256 MiB continuam referência; medições de Work e memória EC-03/04/05 são parciais e não certificam custo uniforme.

## Decisões pendentes e diretrizes

Permanecem para decisão científica prospectiva: quais tarefas e perfis serão avaliados primeiro; verificação de proposição versus geração de conclusão; regra de maior aridade versus encadeamento tipado; semântica de conflito global versus isolamento D1 fora do laboratório EC-05; critérios de independência e contabilização de trabalho; partições e limiares. Não são decisões administrativas nem impedem entregar este catálogo. Nenhum perfil é promovido a política universal.

A1/A2 orientam rastrear engramas, dependências e histórico sem presumir memória integrada ou reorganização neural. A3/A4 orientam progressão e comunicação de lacunas sem presumir investigação autônoma. A5/A6 motivam eventual aplicação linguística, mas estes símbolos formais não demonstram aprendizagem de pt-BR ou de idiomas. A7 exige controles e alegações proporcionais. Os fundamentos e diretrizes permanecem intactos.

Próxima subetapa nominal do plano: **separar conhecimento fornecido e adquirido**, ainda não iniciada. As informações permitidas acima definem fronteiras das tarefas; não constituem avaliação dessa próxima subetapa. Não foram criados testes, dados finais, código ou protocolo congelado.
