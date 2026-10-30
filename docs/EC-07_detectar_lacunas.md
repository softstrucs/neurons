# EC-07 — Detectar lacunas: levantamento dos mecanismos existentes

## Escopo e critério

Base `132844254b37b0730d5d36f7a92d51cc081ae17d`, branch devs inicialmente limpa e sincronizada. O plano v0.2, seção EC-07, nomeia a primeira subetapa **detectar lacunas**; não fixa nela um protocolo, conjunto de dados ou limiar de sucesso. A aceitação da etapa completa exige demonstração de que escolher investigar contribui para aprendizagem posterior, sem depender de perguntas em linguagem natural. Esse critério global não foi avaliado neste levantamento, nem é aceitação automática da primeira subetapa.

Foi feita inspeção de mecanismos e evidências existentes, com reprodução focal de testes já versionados. Aqui, uma *lacuna detectada* é o reconhecimento operacional de insuficiência para uma consulta concreta, sob um mecanismo e orçamento definidos. Não é prova de inexistência de conhecimento no domínio nem diagnóstico autônomo de todas as necessidades de aprendizagem.

## Matriz de rastreabilidade

| Capacidade | Mecanismo e evidência | Limite interpretativo |
|---|---|---|
| Insuficiência para consulta | `TwoHopTask::evaluate` retorna `Unsupported`; `Monitor::review` pode registrar `MissingEvidence` e abster-se; `tests/metacognition.rs::missing_evidence_and_resource_exhaustion_have_distinct_reasons`. | Consulta e contexto vêm do chamador. `Unsupported` significa ausência de suporte no contrato de dois passos e no estado acessível, não ausência universal de fatos. |
| Inconclusão por recursos/falha | `Outcome::InconclusiveBudget`, `InconclusivePropagation` e `AbstainReason` distintos em `src/cognition.rs` e `src/metacognition.rs`; testes de limite e fila. | Limites de fatos, fila, eventos e tentativas são fornecidos externamente; não há calibração geral de incerteza ou seleção autônoma de orçamento. |
| Conflito e evidência | `Monitor` examina negativas de conclusão e premissas da prova selecionada e registra IDs (`Conflict`); testes de conflito em `tests/metacognition.rs`. | Não prova busca exaustiva de alternativas nem política geral de conflitos; EC-05 P1/P2 não é integração automática no núcleo. |
| Revisão limitada | `review` avalia contexto original e candidatos fornecidos, até oito tentativas; `AttemptLog` e `ReviewReport` retêm avaliações, evidências, erro e decisão em memória. | Contextos não são descobertos pelo sistema. Log retornado não constitui histórico persistente de investigação. O teste histórico 7/7 versus 5/7 não é comparação informacional equivalente: controle não recebe alternativa contextual. |
| Consulta/hipótese | `Hypothesis { context, from, to }` é construída pelo chamador em `src/session.rs` e `src/curriculum.rs`; `TwoHopTask` recupera ou compõe duas relações fornecidas. | Nome `Hypothesis` designa consulta estruturada; não demonstra geração autônoma, formulação de pergunta linguística ou hipótese científica. |
| Recuperação e memória | `EngramStore::record`, `get`, `recall`, status e referências; `src/persistence.rs` serializa estado compatível. | Armazenar ou recuperar informação fornecida não é aprendizagem ou descoberta; proveniência e validade não são verificação independente da verdade. |
| Registro de observações | `Microcurriculum::observe` recebe fato, domínio, IDs e polaridade; `evaluate` consulta Monitor; `tests/curriculum.rs`. | Operador fornece observações e consultas. Nomes de domínios simbólicos não provam compreensão nem transferência de conhecimento entre domínios. |
| Investigar e expectativa causal | `tests/support/expectation.rs` retorna `Decision::Investigate` para estados previstos; experimentos EC-02.4 usam observações pareadas e regras de atualização. | Etiqueta de decisão não executa investigação ou coleta externa por si. O protótipo é de teste, não política de produção; sua dependência de regras e controle literal superior em P4 permanecem. |
| Exploração e aprendizagem | A02.4 usa exploração fixa externa e recompensa; A02.5 registra quatro ações externas e zero elegibilidade a partir de pesos zero. EC-04 demonstra aquisição simbólica delimitada de critério em família fornecida. | Nenhum desses resultados prova que detectar lacuna seleciona uma investigação que cause aprendizagem posterior; não atribuir exploração externa ao circuito. |
| Justificação para resposta | EC-06 reproduziu 48/48 decisões acordadas com controle simbólico, 24 retornos sustentados e 24 abstenções nos perfis fixados; [auditoria](EC-06_auditoria_cientifica_final.md). | Fatos, regra e quatro topologias são fornecidos; controle compartilha pressupostos. Não demonstram descoberta, transferência estrutural ou vantagem neural. |

As referências de implementação são [cognição](../src/cognition.rs), [metacognição](../src/metacognition.rs), [memória](../src/engram.rs), [microcurrículo](../src/curriculum.rs) e [sessão](../src/session.rs). A capacidade de aceitar consulta externa não é uma lista de perguntas autonomamente geradas. Os testes usam perguntas/problemas construídos pelo protocolo; a geração por regras em fixtures é procedimento do avaliador, não investigação do aprendiz.

## Lacunas e dependências

Não foi identificado mecanismo integrado que, a partir de objetivos e estado de conhecimento, selecione lacunas, formule perguntas/hipóteses inéditas, escolha intervenção informativa sob orçamento e permissões, execute coleta real, valide resultados e incorpore aprendizagem rastreável. A busca foi limitada ao repositório e mecanismos examinados: ausência observada não prova impossibilidade arquitetural. Também não se demonstram critérios calibrados de importância de lacunas, custo-benefício, fontes autorizadas ou comparação causal com seleção aleatória e política fixa.

As pendências herdadas importam: EC-03/05 preservam diferenças entre manutenção e recomposição, incluindo vantagens pontuais do controle literal; EC-04 registra controle literal T-C incompleto, proveniência/versões e instrumentação parciais; EC-06 tem quatro topologias, controle com pressupostos compartilhados e trabalho não uniformizado. Não confundir conformidade local com generalização. As conclusões históricas permanecem inalteradas.

A distinção entre informação recuperada, critério adquirido em família fornecida, inferência com prova, hipótese gerada e descoberta verificada deve acompanhar qualquer investigação posterior. Um diagnóstico não prova que foi produzida pergunta; pergunta não prova intervenção; intervenção não prova aprendizagem; resposta plausível não prova descoberta.

## Fundamentos e limites

A1/A2 orientam memória evolutiva, proveniência e revisão: os engramas e registros são componentes disponíveis, sem formação autônoma de engramas demonstrada. A3/A4 orientam pedagogia e investigação comunicada com execução real; não se demonstrou ciclo pedagógico autônomo nem investigação externa. A5/A6 priorizam pt-BR e futura expansão multilíngue, não avaliados nesta subetapa. A7 exige controles e limites verificáveis. A8 orienta competência generalizável futura; abstenção localizada não comprova aquisição, descoberta ou transferência. As diretrizes não foram modificadas.

## Verificações e continuidade

Foram reexecutados testes existentes com `cargo test --offline --test metacognition --test cognition --test curriculum -- --test-threads=1`: respectivamente 9, 8 e 7, **24 aprovados, zero falhos ou ignorados**. Não foram criados testes nem executado experimento novo. A reprodução confirma as assertivas existentes, não acrescenta população ou controles. A árvore inicial estava limpa; /home indicou 4.646.318.080 bytes livres no início, acima de 3 GiB; não houve medição contínua de recursos. `git diff --check` e revisão documental são verificações desta entrega.

A primeira subetapa foi concluída como levantamento e diagnóstico delimitado. Próxima subetapa nominal no plano: **formular hipóteses operacionais**, não iniciada. Nenhuma arquitetura, protocolo futuro, política de seleção ou critério experimental posterior é fixado aqui.
