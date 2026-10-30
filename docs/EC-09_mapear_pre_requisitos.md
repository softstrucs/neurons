# EC-09 — Mapeamento de pré-requisitos pedagógicos

## Escopo e definições

Base `b048dc200233ebbf7f834e25ee5c500e931b5ca7`, devs inicialmente limpa e sincronizada. A segunda subetapa nominal do plano v0.2 é **mapear pré-requisitos**, sem protocolo ou limiar experimental específico. Esta especificação organiza relações verificáveis; não implementa planejador, avalia empiricamente relações nem comprova o aceite global de melhora sustentada e adaptação pedagógica em ciclos sucessivos.

- **Conhecimento:** informação, representação ou proposição com origem e estado de validação identificáveis; armazená-la não garante seu uso ou verdade.
- **Habilidade:** execução observável de operação ou procedimento sob condições definidas; não implica domínio geral.
- **Competência:** desempenho demonstrado em família delimitada de tarefas, com critérios, referência, contexto e limites; uma resposta correta isolada pode ser insuficiente.
- **Objetivo de aprendizagem:** resultado pretendido, condições de aplicação e evidência necessária para avaliar progresso; não equivale ao resultado atingido.
- **Pré-requisito:** condição proposta como relevante para adquirir ou demonstrar objetivo sob método e contexto especificados; sua classificação exige evidência própria.
- **Lacuna:** diferença entre evidência atual e critério do objetivo, com cobertura e incerteza registradas; não significa incapacidade universal.
- **Evidência de domínio:** registros verificáveis de desempenho, incluindo casos, referência, contexto, erros, retenção quando relevante e limitações; não basta cadastrar uma relação.
- **Dependência pedagógica:** relação contextual entre condições, experiências e objetivo que afirma necessidade, facilitação, complementaridade ou sequência, conforme justificação.

## Tipos e representação proposta

| Tipo | Alegação e limite |
|---|---|
| Necessário | Sem a condição, o objetivo definido não pode ser atingido sob contexto e alternativas consideradas; exige argumento contratual ou evidência adequada ao escopo. Uma única falha sem a condição não demonstra necessidade universal. |
| Facilitador | Pode melhorar aquisição, custo ou retenção, mas não precisa ser indispensável; direção e magnitude requerem comparação. |
| Complementar | Contribui para objetivo conjunto ou aplicação mais ampla sem afirmar precedência nem necessidade isolada. |
| Hipotético | Relação ainda não verificada; suporte preliminar não equivale a causalidade. |
| Sequência por conveniência | Ordem escolhida por recursos, pedagogia ou organização; não deve ser rotulada como condição necessária. |

Uma representação conceitual deve registrar identidade/versionamento de cada competência e objetivo; tarefa e critérios de domínio; relação tipada e direção; contexto e validade; condição sobre grupos de pré-requisitos, inclusive alternativas e simultaneidade; proveniência, fonte e método; evidências favoráveis e contrárias; origem da relação; estado de verificação; incerteza; data/versão e histórico de revisões. Evidência e inferência devem ser distinguíveis. São requisitos de rastreabilidade, não novo contrato ou escolha de estrutura física.

A origem pode ser **declarada pelo projetista**, **fornecida por usuário autorizado**, **importada de fonte**, **inferida pelo sistema**, **testada empiricamente** ou **revisada diante de evidência contrária**. Origem, método e estado de validação são dimensões separadas: uma relação fornecida pode depois ser testada; uma inferência não é automaticamente descoberta ou validação. Quando avaliador externo executa inferência ou teste, registrar essa autoria, sem atribuí-la ao núcleo.

A rede pode conter grupos alternativos (A **ou** B), conjuntos conjuntos (A **e** B), habilidades paralelas e dependências com condição contextual. Detectar ciclo lógico exige verificar relações dirigidas e seus tipos/condições; um ciclo de prática recíproca pode ser legítimo, enquanto um ciclo de condições obrigatórias sem ponto inicial alcançável pode bloquear progressão. Não é suficiente rejeitar todo ciclo. Uma dependência contraditória requer mesmo contexto, condição e significado; diferença de versão ou método pode explicar divergência.

Lacunas de mapeamento podem ser suspeitadas quando objetivos exigem operações não cobertas, mas ausência de uma aresta não prova dependência ausente. Pré-requisitos excessivos podem impedir progressão apesar de sucesso válido por outra via; insuficientes podem permitir progressão seguida de falha recorrente. Ambos exigem referência e controle, não apenas inspeção do grafo. Caminhos alternativos e desenvolvimento paralelo exigem critérios de suficiência e condições explícitas; dados incompletos levam a suspensão ou revisão da hipótese, sem impor hierarquia universal.

## Exemplos conceituais — não executados

| Objetivo | Relações fornecidas para investigação | Estado e limite |
|---|---|---|
| Memória e recuperação | Identificar um registro e recuperar por pista podem facilitar uma tarefa de recuperação auditável; comparação com outra forma de acesso é necessária. | Hipótese fornecida; não comprova necessidade de um formato específico de engrama. |
| Raciocínio e verificação | Registrar premissas e referências pode facilitar verificação de uma composição; um procedimento alternativo pode oferecer prova equivalente. | Dependência proposta sob regra formal, não causalidade geral nem compreensão semântica. |
| Comunicação | Reconhecer sinais e relacioná-los ao propósito de uma interação são objetivos potencialmente complementares; modalidades alternativas podem existir. | Sequência/contexto a testar, não hierarquia universal. |
| Cognição fundamental | Distinguir estado observável e ausência de suporte pode facilitar diagnóstico de lacunas em uma tarefa delimitada. | Relação hipotética; não implica consciência ou habilidade de investigação autônoma. |
| Português brasileiro | Reconhecimento de símbolos, vocabulário, relações entre palavras, estrutura de frases, interpretação e produção são objetivos distintos com possíveis apoios mútuos. | Ordenação não é universal: modalidades orais, escritas e competências paralelas admitem percursos diferentes. Nenhum domínio geral é presumido. |

Essas relações são propostas por esta documentação, não descobertas pelo Neurons. As evidências históricas de A01 tratam vocabulário e frases controlados; não validam dependências pedagógicas ou competência linguística geral.

## Critérios falsificáveis e controles futuros

Uma alegação de facilitação pode prever desempenho posterior, retenção ou custo melhor com o mapeamento do que sem ele, sob oportunidades e conhecimento inicial equivalentes. Falha no critério previamente congelado, ou vantagem explicada por mais informação/tempo, contraria a alegação no escopo; amostra ou referência insuficiente é inconclusiva. Para necessidade, testar caminhos alternativos e, quando possível, intervenções que removam somente a condição mantendo informação, acesso e demais recursos comparáveis. Sucesso verificável sem a condição refuta sua necessidade no contexto estudado; falha com condição ausente não a comprova sem excluir explicações alternativas, teto de orçamento e formas diferentes de satisfação.

Controles prospectivos devem incluir mapeado versus sem mapeamento, ordem alternativa, dependência incorreta ou redundante, e conhecimento inicial equivalente; registrar intervenções, diferenças de informação e oportunidades. Medir estado inicial, desempenho em tarefas não usadas para definir dependências, erros e abstenções, aquisição, retenção após intervalo apropriado, revisão após evidência contrária, custo por recurso e transferência quando aplicável. Distinguir novidades de itens, combinações, estruturas e domínios. Congelar critérios e referências antes do conjunto final; sem instrumento independente, amostragem apropriada e limites de recurso, não afirmar efeito causal ou generalização. Não se definem agora amostras, limiares ou protocolos experimentais.

## Mecanismos existentes e lacunas

`EngramStore` registra pistas, status, contexto e referência de evidência; `Microcurriculum` organiza observações fornecidas por domínios; `TwoHopTask` compõe relações formais e `Monitor` identifica alguns limites. A [auditoria EC-06](EC-06_auditoria_cientifica_final.md) distingue regras fornecidas de inferência com prova. Esses mecanismos podem servir como peças, mas não interpretam automaticamente relações como pré-requisitos necessários, verificam domínio pedagógico ou selecionam currículo. Protótipos EC-04/05 mostram aquisição/revisão simbólica delimitada, não validador geral de dependências; EC-07/08 preservam ausência de investigação autônoma e calibração. EC-04 apresenta ressalvas de controles e instrumentação, não reescritas aqui.

A1/A2 orientam memória evolutiva e proveniência; A3/A4 formação progressiva e comunicação de limites; A5 prioriza pt-BR, A6 expansão multilíngue posterior; A7 exige controles e falsificação; A8 é objetivo futuro de competência generalizável. A arquitetura modular e orientada a eventos é preservada; não se fixa número de camadas ou unidade universal. Fundamentos e A1–A8 permanecem íntegros.

Esta entrega é documental; nenhum teste ou experimento novo foi executado. A subetapa fica concluída como especificação delimitada. Próxima subetapa nominal **selecionar experiências**, não iniciada.
