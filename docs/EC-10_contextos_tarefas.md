# EC-10 — Definir novos contextos e tarefas

## Escopo e aceite

Base `180ce67e2cced3615e77c359e5cf142382fd7ff5`, devs inicialmente limpa e sincronizada. O plano v0.2 denomina a primeira subetapa **definir novos contextos e tarefas**; etapas seguintes estabelecem controles de renomeação/recuperação e avaliam transferência genuína, retenção longitudinal e interferência. O aceite global exige desempenho acima de controles adequados em situações não usadas na aquisição, com fracassos e limites de domínio. O plano não fixa para esta primeira subetapa quotas, protocolo executável ou limiar; esta é definição prospectiva, não avaliação realizada ou aceite da EC-10.

## Distinções operacionais

- **Recuperação:** retornar registro prévio sob pista; não comprova aquisição de regra ou capacidade nova.
- **Composição simbólica:** combinar fatos sob regra fornecida e produzir conclusão com justificativa; não é descoberta da regra.
- **Generalização dentro de família:** aplicar critério adquirido ou fornecido a itens não treinados da mesma família; especificar o que foi novo.
- **Transferência entre tarefas:** mudança verificável em desempenho de tarefa-alvo associada a experiência em tarefa-fonte, comparada com estado anterior e controle apropriado. Sem ganho comparativo, relatar transferência nula ou inconclusiva.
- **Transferência entre domínios:** mesma avaliação em domínios com semântica e distribuição explicitadas; trocar rótulos ou IDs é insuficiente.
- **Aprendizagem persistente:** mudança interna rastreável que permanece e sustenta desempenho posterior avaliado. **Cumulatividade:** ganhos sucessivos e retenção do anterior sob experiências posteriores, com comparação e interferência; salvar registros não basta.
- **Regra fornecida** é parte da informação inicial, mesmo quando gera conclusão inédita; **aquisição de regra ou relação** exige processo e mudança identificáveis e avaliação independente; **descoberta autônoma** exige evidências adicionais de origem, verificação e ausência de resposta fornecida.

Uma transferência pode ser positiva, nula ou negativa conforme comparação pré-definida com controle e custo; sem referência ou medição suficiente é inconclusiva. Não assumir que um resultado acima de recuperação literal demonstra superioridade neural ou causa específica da transferência.

## Contextos e tarefas prospectivos

As tarefas abaixo definem perguntas e condições, não instâncias finais, respostas, quotas ou protocolo congelado. Cada família precisa de referência, contratos semânticos e dados independentes em etapa autorizada. A informação fornecida, alteração interna e o responsável por cada operação devem ser rastreáveis.

| Tarefa | Contexto e situação proposta | O que pode testar e limite |
|---|---|---|
| T10-R — recuperação | Adquirir registros em um contexto e consultar âncoras e ausências no mesmo contexto. | Linha de base para memória; âncoras conhecidas não são transferência ou aprendizagem de regra. |
| T10-C — recombinação de relações | Fornecer relações tipadas e regra formal explícita; consultar conclusão não cadastrada formada por fatos conhecidos, sem aresta de resposta direta. | Composição programada em combinações novas; regra e semântica fornecidas, não descoberta autônoma. Ausência de premissas/contradição exige abstenção segundo contrato. |
| T10-F — critério em família | Adquirir critério simbólico por exemplos rotulados em uma família declarada; avaliar vetores não apresentados no treino com classes e contextos explícitos. | Aplicação a itens ou combinações reservadas da mesma família; controlar memorização, vazamento e distribuição. Protótipo EC-04 é ponto de partida experimental, não ciclo integrado. |
| T10-T — tarefa-alvo | Medir desempenho inicial em tarefa-alvo, realizar experiência em tarefa-fonte e reavaliar alvo com itens reservados; fonte e alvo devem compartilhar condição declarada e não gabaritos. | Transferência entre tarefas apenas se houver mudança rastreável e ganho/perda comparativo; sinalizar informação comum e custo. Se a fonte não modificar estado acessível, não alegar transferência aprendida. |
| T10-D — domínio-alvo | Selecionar dois domínios com contratos, observações e representações descritos independentemente; especificar correspondências autorizadas e quais fatos do alvo já são conhecidos. | Transferência entre domínios, se o desempenho alvo exceder controles adequados sem resposta cadastrada. Troca de nomes com estrutura e semântica idênticas é apenas renomeação, não domínio novo. |
| T10-L — retenção e interferência | Após aquisição em A, medir desempenho após intervalo/reinício definido e após experiência B relacionada ou concorrente; reavaliar A e B. | Retenção, esquecimento e interferência sob condições rastreáveis. Preservar pesos/estado se a hipótese exigir compartilhamento; estados independentes não provam resistência de parâmetros compartilhados. |
| T10-K — cumulatividade | Acompanhar pelo menos dois momentos de aquisição e avaliação, com estado inicial, mudanças, avaliação reservada após cada momento e reavaliação anterior. | Ganho sucessivo e retenção dentro do protocolo; repetição de registros ou melhoria transitória não basta. Necessita mecanismo de atualização persistente e controle de exposição. |
| T10-N — negativos e limites | Apresentar ausência de evidência, relações incompatíveis com tipo/contexto, conflito, orçamento esgotado e alvo não autorizado. | Medir respostas indevidas, abstenções e inconclusões; não atribuir rejeição por triagem externa ao circuito. |

Contextos são identidade e versão com critérios de validade e proveniência; uma versão sucessora não herda automaticamente fatos da anterior. Para cada tarefa declarar o que é conhecido antes, o que foi adquirido, o que está acessível na consulta, fonte e confiabilidade da referência e o que foi mantido após intervenção. A escolha de não executar uma ação por permissão ou orçamento não é prova de incapacidade cognitiva.

## Desenho futuro e separação experimental

Definir desenvolvimento, validação e final antes da avaliação, separando exemplos e alvos finais da aquisição e do ajuste. Declarar equivalências permitidas entre contextos e verificar possíveis vazamentos por IDs, ordem, convenções e construção de fixtures. Topologia idêntica com IDs novos deve ser identificada como renomeação; combinações inéditas, estruturas inéditas e domínios inéditos têm reivindicações distintas. Para retenção/transferência, medir estado anterior, pós-experiência e posterior, com referência válida e incerteza; registrar acertos, erros, abstenções, alterações de estado, interferência e custo. Preservar falhas e resultados nulos.

As tarefas requerem comparação com controles adequados de recuperação, regras fornecidas e acesso equivalente à informação; especificação e execução desses controles pertencem à próxima subetapa, ainda não iniciada. Não presumir igualdade de custos entre eventos neurais e buscas simbólicas; documentar recursos e instrumentos. Um rótulo de referência não deve entrar nas decisões antes da avaliação. Não há neste documento demonstração empírica, escolha de limiares ou seleção definitiva de arquitetura.

## Base existente e dependências

`EngramStore` e persistência registram dados; `TwoHopTask` compõe relações fornecidas; `Monitor` retorna evidência e abstenção; `Microcurriculum` recebe observações externas; `PlasticityRule` altera pesos com feedback fornecido. Nenhum deles, isoladamente, demonstra os requisitos de T10-T/D/K. [EC-04](EC-04_auditoria_cientifica_final.md) mostrou aquisição/revisão simbólica delimitada com família fornecida e ressalvas de controles/proveniência/instrumentação; EC-05 tem revisão contextual delimitada; [EC-06](EC-06_auditoria_cientifica_final.md) mostrou composição e controle simbólico equivalente em quatro topologias; EC-07/08 não demonstraram investigação autônoma ou aprendizagem metacognitiva. [EC-09](EC-09_auditoria_cientifica_final.md) encerrou especificações, sem ciclo pedagógico integrado. Histórico preservado.

A1/A2 orientam memória e revisão com proveniência; A3/A4 aprendizagem pedagógica e investigação efetiva; A5 pt-BR e A6 expansão multilíngue; A7 rigor e A8 competência generalizável futura. Tarefas linguísticas devem declarar semântica e revisão independente; não se presume domínio de pt-BR. Arquitetura modular orientada a eventos e memória evolutiva como diretriz permanecem preservadas.

Nenhum teste ou experimento novo foi executado. A primeira subetapa é concluída documentalmente; próxima subetapa nominal **estabelecer controles de renomeação e recuperação**, não iniciada. EC-10 permanece aberta e aceite global não homologado.
