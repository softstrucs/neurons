# EC-09 — Selecionar experiências pedagógicas

## Escopo e definições

Base `6e8ce200e9ed9fe50a488c982ad8ae3512183ec5`, devs inicialmente limpa e sincronizada. O plano v0.2 nomeia a terceira subetapa **selecionar experiências**, sem protocolo ou limiar experimental específico. Esta especificação não executa seleção pedagógica pelo Neurons nem comprova o aceite global de melhora sustentada e adaptação verificável ao longo de ciclos.

- **Experiência pedagógica:** atividade delimitada, com entrada, contexto, possível interação, resultado observável e recursos, proposta para um objetivo; participação não implica aprendizagem.
- **Objetivo de aprendizagem:** resultado pretendido e condições de avaliação; **competência-alvo** é desempenho a demonstrar em família de tarefas delimitada.
- **Lacuna:** diferença entre evidência disponível e critério do objetivo, acompanhada de incerteza; **pré-requisito** é relação contextual cuja necessidade ou facilitação não se presume validada.
- **Evidência de progresso:** observação rastreável de mudança comparada ao estado anterior, com referência e controles; informação obtida não basta para estabelecer competência persistente.
- **Critério de seleção:** regra ou justificativa explícita para comparar candidatas elegíveis; custo e risco registram consumo e possíveis efeitos, sem supor estimativas calibradas.
- **Resultado esperado:** consequência observável prevista e condições de falsificação, não garantia. **Interrupção/revisão:** decisão diante de limite, falha, evidência contrária, risco ou mudança de autorização, preservando histórico.

## Modalidades e origem

Experiências candidatas podem ser observação, estudo orientado, exercício, experimentação, investigação, comparação, aplicação prática, revisão/recuperação e avaliação de retenção. São categorias conceituais; não se afirma que todas estejam implementadas. Uma mesma atividade pode atender mais de um objetivo, desde que resultados e referências sejam separáveis.

A experiência pode ser fornecida pelo projetista, sugerida por usuário autorizado, recuperada de histórico, escolhida por regra programada, escolhida por avaliação de evidências ou proposta autonomamente. Essas origens não são graus automáticos de aprendizagem. **Escolhida**, **executada** e **produziu aprendizagem verificável** são estados distintos; registrar quem escolheu/executou, com quais informações, em vez de atribuir ao núcleo uma decisão externa. Sugestão ou utilidade não é autorização para ação externa ou irreversível.

## Contrato conceitual e critérios

Registrar objetivo e competência-alvo; estado inicial, lacuna e evidências; contexto/versão e pré-requisitos com seu estado de validação; candidatas e respectivas origens; informação acessível; restrições de recurso, autorização e risco; critérios aplicados; escolhida ou abstenção com justificativa; resultado esperado, método e referência; execução real, consumo e evidências; decisão de reavaliação e histórico. Isto é um contrato de rastreabilidade proposto, não API ou arquitetura definitiva.

Para cada candidata examinar:

1. **Relevância e discriminação:** ligação explícita à lacuna, alternativa que poderia esclarecer incerteza e critérios de resultado; evidência irrelevante não é progresso.
2. **Pré-requisitos e alternativas:** compatibilidade com o mapeamento e validade contextual; relações hipotéticas ou meramente convenientes não devem virar bloqueios necessários sem evidência. Admitir percursos alternativos e paralelos.
3. **Verificação e qualidade:** evidência esperada, origem, independência, validade e resultado contrário; recuperação de um exemplo não demonstra competência em família de tarefas.
4. **Recursos, risco e permissão:** custo estimado por dimensão, tempo, memória, acesso, possíveis efeitos e autorização concreta; estimativa não é medição e utilidade não concede permissão.
5. **Histórico e diversidade:** desempenho e experiências anteriores podem orientar cobertura de casos, modalidades e evitar repetição redundante; diversidade é critério contextual, não ganho automático.
6. **Decisão e reavaliação:** justificar comparação ou abster-se/adiar quando não houver candidata adequada, autorizada, segura ou verificável. Limite atingido, falha, risco ou mudança de permissão interrompe execução; registrar resultado parcial e motivo.

Ganho informacional pode reduzir alternativas compatíveis sob um contrato; ganho de competência exige mudança persistente, avaliação posterior adequada e controles. Uma escolha racional segundo informação disponível pode não produzir aprendizagem; inversamente uma resposta correta não prova que a escolha causou aquisição. Nenhuma pontuação, probabilidade ou ordem universal é fixada aqui.

## Exemplos conceituais — não executados

| Lacuna/objetivo | Experiência candidata | Critério e limitação |
|---|---|---|
| Memória e recuperação | Exercitar recuperação por pista em itens autorizados e avaliar referência em itens separados. | Pode testar recuperação sob uma interface; não demonstra formação autônoma de engramas. |
| Raciocínio e verificação | Comparar duas provas formais com premissas e contraexemplo, depois verificar novo caso. | Regras e referência fornecidas; não demonstra descoberta de regra. |
| Identificação de lacunas | Apresentar caso com suporte incompleto e pedir identificação da insuficiência, incluindo limite de recurso como alternativa. | Deve distinguir ausência, erro e falha; rótulo pré-programado não prova diagnóstico geral. |
| Comunicação | Comparar respostas justificadas e abstenções diante de pergunta desconhecida, com revisão de fonte autorizada. | Não equivale a conversação geral nem prova de investigação executada. |
| Português brasileiro | Observar símbolos ou palavras, comparar relações semânticas, construir frases controladas e avaliar interpretação em exemplos separados, conforme modalidade e objetivo. | Percursos podem variar entre oral e escrito; experiências não demonstram domínio de pt-BR. |

Exemplos são fornecidos nesta documentação, sem execução experimental ou descoberta atribuída ao núcleo. Ausência de autorização, recursos ou referência adequada pode tornar qualquer exemplo inviável e exigir abstenção.

## Avaliação prospectiva e controles

Hipótese falsificável: sob mesmo estado inicial, acesso a evidências e orçamento, seleção orientada por lacunas produz melhor desempenho posterior em tarefas reservadas que seleção fixa ou aleatória válida, segundo critério fixado antes da avaliação. Resultado abaixo do critério, sem melhoria ou explicado por informação/oportunidade adicional contraria a hipótese no escopo; referência ou amostra insuficiente é inconclusiva. Uma segunda hipótese: selecionar com pré-requisito incorreto prejudica progresso ou aumenta custo frente ao mapeamento válido; ausência dessa diferença não comprova que o pré-requisito é desnecessário em geral.

Controles futuros: política orientada por lacunas, fixa, aleatória com gerador e amostragem definidos, pré-requisito incorreto/redundante, ausência de diagnóstico e comparação com conhecimento prévio equivalente. Todas recebem mesmas informações, permissões, candidatas e limites quando comparáveis; qualquer diferença intencional deve ser medida e declarada. Escolha adequada e aprendizagem causada precisam de avaliações separadas, incluindo controle sem atualização e verificação de persistência quando pertinente.

Definir antes da execução desenvolvimento, validação e avaliação final separadas, com alvo e respostas acessíveis somente ao avaliador após decisões; prevenir vazamento entre experiências e itens finais. Medir estado inicial, desempenho e erros após experiência, retenção posterior, adaptação diante de falhas, custos por recurso e transferência em itens/combinações/estruturas/domínios explicitamente inéditos. Registrar abstenções, resultados negativos e inconclusões. Sem referência independente, evidência suficiente ou orçamento comparável, não alegar ganho causal ou generalização. Não se fixam amostras, métricas quantitativas finais ou protocolo da próxima subetapa.

## Mecanismos, diretrizes e continuidade

`EngramStore`, `Microcurriculum`, `TwoHopTask`, `Monitor` e `PlasticityRule` fornecem peças de memória, inferência, monitoramento e feedback externo, não seletor pedagógico integrado. EC-04 mostrou aquisição simbólica delimitada em família fornecida, com ressalvas de controles e recursos; EC-05/06 preservam limites de contexto e composição; EC-07/08 não demonstraram investigação autônoma ou ganho persistente de competência. Seleção adaptativa e ciclos pedagógicos contínuos permanecem não demonstrados.

A1/A2 orientam memória evolutiva, histórico e proveniência; A3/A4 pedagogia progressiva e comunicação honesta de investigações executadas; A5 prioriza pt-BR e A6 expansão multilíngue futura; A7 exige validação e controles; A8 é objetivo futuro de competência generalizável. Preservam-se a arquitetura modular orientada a eventos e os fundamentos A1–A8, sem fixar uma unidade universal ou currículo rígido.

Entrega documental: nenhum teste ou experimento novo. Subetapa concluída no escopo limitado; próxima subetapa nominal **acompanhar prática, avaliação, revisão e consolidação**, não iniciada.
