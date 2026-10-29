# EC-04 — Definição de tarefas com exemplos não cadastrados

## 1. Escopo, estado e referências

Definição documental concluída em 2026-10-29, branch `devs`, base `ef22bc55df35f87831359ab8e4857259aca950f0`, árvore inicialmente limpa. Esta é a segunda subetapa literal da EC-04 do plano cognitivo v0.2, **definição de tarefas com exemplos não cadastrados**. O plano denomina a etapa “Formação progressiva de conceitos e relações” e determina, a seguir, protocolo com controles de memorização, implementação isolada e avaliação de discriminação, composição e exceções.

Foram consultados a definição integral da EC-04, o Fundamento aprovado, a auditoria `EC-04_auditoria_relacoes_existentes.md`, as decisões e os limites EC-03.5. A EC-03 permanece encerrada como investigação delimitada, com conformidade parcial e conclusão AJUSTAR. Nenhuma lacuna foi corrigida nesta entrega.

Este documento define tarefas, partições exigidas e critérios científicos; **não é o protocolo operacional completo da subetapa seguinte**. Não implementa algoritmos, gera fixtures finais, cadastra respostas, executa testes ou autoriza experiências. Os critérios abaixo são propostas de tarefa produzidas nesta delegação, não resultados ou critérios históricos inventados.

## 2. Distinções operacionais

| Operação | Condição necessária para classificá-la | O que não autoriza concluir |
|---|---|---|
| Recuperação exata | Resposta vem de registro da mesma entrada, contexto e versão. | Aprendizagem de regra, compreensão ou generalização. |
| Composição conhecida | Resposta não cadastrada, mas produzida por fatos e regra explicitamente fornecidos. | Aquisição da regra ou formação autônoma de conceito. |
| Aplicação a exemplo novo | Entrada completa e resposta não ocorreram na aquisição; no teste conceitual, o vetor de propriedades também é novo. | Transferência estrutural se formato e família de problemas continuam iguais. |
| Transferência estrutural | Critério adquirido contribui em representação/estrutura de tarefa diferente, com controle sem essa aquisição e sem fornecer a solução nova. | Renomeação, novos IDs, outra cadeia de mesma topologia ou regra nova entregue pronta não bastam. |
| Formação operacional de conceito | Estado modificável extraído de experiências passa a classificar entradas novas sem receber o critério-alvo pronto; ganho depende dessas experiências. | Formação semântica autônoma: propriedades, linguagem de hipóteses e professor continuam externos. |
| Revisão de conceito | Novas experiências mudam o critério usado posteriormente; mudança observável em casos novos e casos antes válidos. | Regravar respostas exatas ou trocar contexto externamente sem mudar critério não basta. |

**Transferência estrutural não será avaliada pelas quatro tarefas definidas aqui.** Elas mantêm a linguagem de vetores/relações e, portanto, não produzirão evidência dessa capacidade. Não introduzir uma tarefa de transferência sem objetivo, condições e controles próprios em autorização futura.

## 3. Mecanismos disponíveis e limites de reutilização

EngramStore pode servir à recuperação literal e às referências; TwoHopTask permite tarefa de composição de dois passos com orçamento; Monitor distingue algumas ausências, conflitos e limites. IDs/contextos, sinapses e eventos dão suporte estrutural, não significado aprendido. A regra de composição de TwoHopTask é programada e não deve ser aplicada a qualquer predicado.

Os suportes EC-03 podem informar rastreabilidade e revisão, mas não são infraestrutura integralmente validada de produção. PlasticityRule demonstra ajustes locais sob feedback; não fornece, por si, indução de conceitos a partir de propriedades. A02.5 conserva o bloqueio de aprendizagem no circuito silencioso. Não há mecanismo de formação conceitual pronto cuja capacidade possa ser presumida.

A tarefa conceitual exige capacidade experimental ainda a especificar, não alteração imediata dos contratos existentes. Permanecem abertas alternativas simbólicas, neurais e híbridas; não há preferência causal ou arquitetural decretada pela tarefa.

## 4. Tarefas definidas

### T-R — Recuperação literal e detecção de ausência

**Questão:** o sistema distingue associação registrada de entrada sem associação? Entradas são símbolos opacos, contexto e versão; saídas são valor registrado, ausência, conflito ou inconclusão por limite. Aquisição define um inventário de 16 associações; validação faz oito consultas (quatro presentes e quatro ausentes) e final faz 16 (oito presentes e oito ausentes), sobre cópias independentes do mesmo estado adquirido, sem registrar as consultas. Consultas de validação e final são distintas entre si; as âncoras presentes pertencem intencionalmente ao inventário de aquisição. Os oito presentes finais consultam associações adquiridas: são **âncoras de recuperação**, não exemplos novos de conceito. Os oito ausentes finais não são cadastrados nem acompanhados de resposta no componente.

Não apresentar T-R como teste em que todos os itens são inéditos: sua função é estabelecer o desempenho e a cobertura da memória exata. Êxito funcional proposto: 8/8 âncoras recuperadas e 8/8 ausências reconhecidas, sem resposta arbitrária ou mutação na consulta. Limite excedido não conta como ausência.

### T-C — Composição de relações conhecidas sem conclusão cadastrada

**Questão:** fatos conhecidos permitem responder consultas não armazenadas? Utilizar relação sintética L cuja regra de dois passos é expressamente fornecida e aplicável somente a ela. Aquisição recebe fatos positivos e seus IDs, nunca a conclusão final como fato direto. Não usar palavras ou tipos de domínio para embutir respostas.

Partições exigidas: quatro grafos de desenvolvimento, dois de validação e oito finais, todos disjuntos em identidades de nós/fatos/contextos. Cada grafo final contém duas rotas disjuntas de comprimento dois e distratores sem rota até o destino; recebe quatro consultas, duas com prova de dois passos e duas sem prova dentro desse escopo: **32 consultas finais, 16 com suporte e 16 sem suporte**. Nenhuma consulta final com suporte pode coincidir com uma aresta direta. A ausência de rota não significa falsidade universal da relação.

Cada grafo final deve conter de seis a oito nós e de quatro a oito arestas, com orçamento suficiente para os casos declarados; a escolha concreta e a verificação do não cadastramento pertencem ao protocolo seguinte. Êxito funcional proposto: 16/16 respostas com referências corretas e 16/16 abstenções de ausência de suporte; resultados inconclusivos são reportados separadamente. Mesmo com 32/32, classificar somente como composição programada em novas instâncias da mesma estrutura, não aquisição de L ou transferência estrutural.

### T-A — Aquisição de critério conceitual e classificação de propriedades novas

**Questão falsificável:** experiências rotuladas podem modificar um critério interno que classifica vetores não apresentados, superando recuperação exata e respostas fixas? Objetos possuem oito propriedades booleanas pré-processadas. Um ID serve somente à proveniência: não entra na classificação. Propriedades e rótulos são abstratos, sem promessa de percepção ou significado linguístico.

A família de alvos é a conjunção de um a três literais distintos dentre oito propriedades, com sinais positivo/negativo, sem usar simultaneamente uma propriedade e sua negação. Há **576 critérios possíveis**: 16 de um literal, 112 de dois e 448 de três. A família é viés indutivo declarado. O critério específico fica somente no professor/avaliador; o aprendiz recebe vetores e rótulos de aquisição. Nesta tarefa, o alvo usa dois literais; a identidade, polaridade e posições relevantes não são fornecidas ao mecanismo como máscara, nome, ordem especial ou índice de exemplo.

Partições por contexto: **32 exemplos de aquisição (8 positivos/24 negativos), 16 de validação (4/12) e 32 finais (8/24)**. Vetores completos são distintos entre as três partições; não basta renomear o mesmo vetor. Há 256 vetores possíveis, dos quais 64 positivos para cada conjunção de dois literais; os tamanhos propostos não esgotam as classes. Exemplos finais não podem ser pré-inseridos em léxico, memória de respostas ou instruções do mecanismo.

A composição concreta da aquisição deve permitir identificar o critério-alvo dentro da família declarada. Isso requer verificação de identificabilidade pelo avaliador, sem entregar a identificação ao aprendiz. Caso os dados não distingam hipóteses relevantes, declarar inconclusão de desenho e não impor escolha arbitrária. A regra de seleção/revisão de hipóteses e a fronteira do professor ainda deverão ser especificadas e aprovadas na subetapa de protocolo.

Critério exploratório proposto, por alvo/contexto avaliado: pelo menos 7/8 positivos e 21/24 negativos finais corretamente respondidos (recall por classe >=87,5%), com abstenções contadas como não acerto; nenhuma consulta altera o estado. Para sustentar aquisição, exigir mudança interna rastreável aos exemplos e melhora de pelo menos 0,25 na acurácia balanceada sobre a condição sem atualização no mesmo conjunto. Não basta atingir limiar com critério fornecido pronto ou sem dependência da experiência.

A família inclui muitos alvos; um único alvo exitoso não sustenta capacidade geral. O protocolo seguinte deverá fixar quais alvos serão avaliados, sua quantidade e agregação, mantendo publicação por alvo. Não se declara agora que o modelo foi aprendido, que o conjunto final existe ou que é independente de autoria.

### T-V — Revisão delimitada e conservação em contexto não alterado

**Questão:** o critério adquirido muda após contradições válidas, sem apenas memorizar exceções ou perder conhecimento de outro contexto? Usar dois contextos explícitos com critérios independentes adquiridos em T-A. Em um contexto, o professor altera a regra de dois literais para a mesma conjunção acrescida de um terceiro literal; o contexto permanece com a mesma identidade. A mudança não é anunciada por campo que contenha o novo critério. O outro contexto não muda e não recebe novos exemplos na fase de revisão.

No contexto alterado, separar **16 novos exemplos de aquisição da revisão (2 positivos/14 negativos), oito de validação da revisão (1/7) e 16 finais (2/14)**. Esses 40 vetores são distintos entre si e dos 80 usados anteriormente nesse contexto. O universo de 256 comporta os 120 exemplos; a seleção deve ser verificada conjuntamente com as partições iniciais, não presumida só a partir do tamanho total. O novo alvo tem 32 positivos no universo, mas isso sozinho não garante uma amostra diagnóstica: precisam existir experiências que distingam o terceiro literal e evidenciem a mudança.

Dos 14 negativos finais de revisão, oito devem ser positivos pelo critério antigo e negativos pelo novo, e seis negativos sob ambos; os dois positivos permanecem positivos sob ambos. Esse painel é diagnóstico e selecionado por estratos pelo avaliador, não uma amostra de prevalência natural. A mistura torna alcançável o ganho de acurácia balanceada exigido frente à conservação perfeita do critério antigo, sem facilitar apenas a classe majoritária. Não fornecer os rótulos desses casos como experiências de revisão. Registrar desempenho antes/depois e diferença entre conservação do critério antigo, nova aquisição e memória de exceções. A avaliação sem atualização retém explicitamente o estado adquirido anterior.

Critério proposto: após revisão, 2/2 positivos e pelo menos 13/14 negativos finais corretamente respondidos; antes/depois medidos no conjunto congelado sem atualização pelas consultas. Acurácia balanceada deve melhorar pelo menos 0,25 frente ao controle sem atualização. No contexto não alterado, conservar exatamente decisões e referências de 16 sondas reservadas (4 positivas/12 negativas), disjuntas dos exemplos adquiridos e de validação naquele contexto; não há feedback dessas sondas. Informar separadamente quantos casos mudaram indevidamente. Não chamar isso de resistência geral à interferência se os parâmetros forem independentes; a identidade dos parâmetros efetivamente compartilhados precisa constar do protocolo.

A conservação é contextual, e a mudança de critério é limitada à família fornecida. Não demonstra criação autônoma de semântica, revisão em ambiente aberto ou transferência estrutural.

## 5. Controles exigidos e informação disponível

| Controle | Informação/operação | Interpretação |
|---|---|---|
| Recuperação literal | Mesmos exemplos adquiridos e metadados; consulta exata por vetor/conteúdo e contexto. Em T-C, consulta direta sem composição. | Testa memorização. Sua falta de composição é diferença de capacidade declarada, não prova de superioridade neural. Em T-A/T-V, final realmente novo deve produzir ausência literal. |
| Respostas fixas | Sempre positivo, sempre negativo, sempre abster-se; nenhuma informação do alvo. | Medir por classe e cobertura; maioria negativa pode inflar acurácia simples. |
| Sem atualização | Mesmo estado inicial; T-V conserva exatamente o estado anterior à revisão; recebe as mesmas consultas e oportunidades. | Distingue transformação por experiência de regra pronta ou estado prévio. Registrar se exemplos são conservados mas não utilizados. |
| Regra simbólica fixa | Critério único escolhido antes dos dados, sem consultar o alvo; em T-C, composição simbólica da regra fornecida com todos os mesmos fatos. | Não confundir regra fornecida de T-C com critério adquirido de T-A. Publicar eventual coincidência casual com alvo. |
| Indução simbólica independente | Mesma família, episódios, propriedades e orçamento de T-A/T-V; sem critério-alvo. Implementação e decisão próprias a especificar. | Controle informacionalmente equivalente: pode igualar/superar qualquer candidato; isso impede alegação de superioridade neural. |
| Oráculo de tarefa | Somente avaliador conhece critério-alvo/soluções e pontua após a resposta. | Não é baseline equivalente. Um classificador com alvo fornecido é referência de verificabilidade, não concorrente com mesma informação. |

Nenhum controle independente poderá chamar validação, atualização ou decisão do candidato. Entradas finais e contexto são os mesmos; orçamento e informação extra devem ser explicitados. Não retirar fatos necessários de um controle e chamar isso de comparação justa. Ablações de fatos, evidência ou estado podem ser propostas no protocolo como diagnósticos identificados, sem reinterpretá-las como controles equivalentes.

## 6. Separação, reserva final e prevenção de vazamento

A preparação termina antes de qualquer consulta final. Em aquisição, somente os exemplos designados e seus rótulos entram no mecanismo. Validação é separada; se orientar escolhas de configuração, permanece validação e não pode ser apresentada como teste final. Nenhum vetor ou consulta final é usado para ajustar algoritmo, selecionar hiperparâmetros ou escolher critério vencedor.

O conjunto final concreto **não foi produzido nesta subetapa**. O documento define sua composição exigida; geração/seleção, identificação por hash, controle de acesso e custódia de rótulos serão especificados no protocolo seguinte, antes de implementação e execução. Não reconstruir um conjunto supostamente pré-registrado a partir de resultados. A definição pública de tamanho/família não é garantia de cegueira ao alvo ou independência de dados.

O protocolo deverá fixar entradas e alvos concretos em material exclusivo do avaliador e registrar hashes antes da primeira avaliação final. O mecanismo recebe somente entradas e fatos permitidos, nunca categorias de cenário, índice de resposta, ordem correlacionada com alvo ou campo auxiliar revelador. IDs e ordem não podem codificar classe. Parâmetros, seleção de atributos e política são congelados antes de consultar o final.

Pré/pós-aquisição ou revisão sobre os mesmos itens finais só serão permitidos se previstos antes da execução, sem atualizar por avaliação, sem devolver pontuações ao mecanismo e sem ajustes de desenho após observar resultados. Os episódios de treinamento posteriores a uma avaliação devem ser os predeterminados, independentes dela. Se houver acesso ao final para ajuste, o conjunto torna-se diagnóstico; será necessária nova reserva independente, com fato registrado, sem apagar resultados.

Não garantir novidade somente por ID: comparar igualdade de vetores completos em T-A/T-V e de consultas/conclusões em T-C, registrando as âncoras intencionalmente repetidas de T-R. O professor externo e a extração de propriedades são dependências explícitas, não capacidades do Neurons.

## 7. Métricas e critérios de interpretação

Publicar por tarefa, alvo, contexto e fase, com denominadores explícitos: positivos/negativos corretos, erros por classe, cobertura, abstenções, conflitos e inconclusões. Para T-A/T-V, acurácia balanceada = (acertos positivos/total positivo + acertos negativos/total negativo)/2; abstenções não contam como acertos. Relatar também precisão entre respostas, sem usá-la sozinha para premiar abstenção total. Se denominador de precisão for zero, registrar indefinido, nunca 100%.

Registrar mudanças internas e referências aos exemplos que as sustentam; memória exata não é critério geral adquirido. Medir retenção separada de generalização; composição nova separada de aprendizado de composição. Repetibilidade exige mesmos dados, ordem, estado inicial, decisões e evidências, não necessariamente mesmo tempo/RSS.

- **Êxito restrito:** satisfazer o critério da tarefa, com controles e integridade dos dados. T-R/T-C demonstram somente suas operações; T-A/T-V exigem transformação atribuível à experiência, além de desempenho reservado.
- **Resultado negativo válido:** desenho identificável, orçamento suficiente e execução correta não atingem critério ou não superam condição sem atualização. Preservar falha e abstenções; não ajustar limiares para sucesso.
- **Inconclusão:** aquisição não identifica alvos, vazamento, execução inválida, dados sobrepostos indevidamente, controles com informação desigual, orçamento excedido ou instrumentação insuficiente para a alegação pretendida. Distinguir essas causas de ausência de relação/classe negativa.
- Controle simbólico equivalente bem-sucedido explica a tarefa por indução/composição programadas; nenhum resultado aqui prova necessidade neural geral. Ablação só sustenta dependência local da configuração estudada.

Não executar uma tarefa dependente de mecanismo ausente presumindo que o resultado será positivo. A classificação de um novo item por critério adquirido pode sustentar generalização dentro da família; não prova generalização irrestrita, compreensão linguística ou competência geral.

## 8. Recursos e dependências do protocolo seguinte

Requisitos de desenho: execução local determinística em Rust/std, sem dependências externas ou serviços. Para o futuro painel isolado: RSS <256 MiB, dados vivos/fixtures até 256 KiB por condição incluindo logs e repetições coexistentes, tempo total até 30 s e `/home` livre >=3 GiB. T-C mantém no máximo oito nós/oito arestas por grafo e teto inicial de 128 eventos por consulta. T-R limita inventário a 16 associações; T-A/T-V usam oito bits de propriedades e no máximo 576 critérios na família declarada. Esses são limites propostos de tarefa, não medições efetuadas.

O protocolo deverá verificar viabilidade e especificar carga de alvos/contextos, total de episódios/consultas/repetições, contagem de passos/visitas inclusive nas rejeições, operações atômicas e instrumentos de interrupção antes de executar. Nenhum agregado de consultas ou custo total está congelado aqui porque a quantidade de alvos ainda pertence à especificação operacional. Não usar a ausência desse número para improvisar durante implementação.

Permanecem decisões da próxima subetapa: identidade e balanceamento dos alvos, exemplos concretos, regra/algoritmo do candidato e do controle independente, feedback e elegibilidade se houver circuito, tratamento de conflito/múltiplas hipóteses, sequência exata, orçamento comum e custódia final. Não são decisões que impedem concluir esta definição documental de tarefas; são pré-requisitos impeditivos de execução até que o protocolo esteja completo e aprovado.

## 9. Conclusão e próxima atividade

Subetapa concluída: quatro tarefas com funções distintas, partições exigidas, controles, métricas e critérios falsificáveis, sem dados finais cadastrados e sem experimentos. Compatibilidade com o Fundamento é conceitual: explicita transformações observáveis (2/4), aplicação e limites contextuais (8), confiabilidade/controles (9) e aquisição progressiva (1/7/11/12), sem declarar realização desses objetivos.

Próxima subetapa literal do plano: **protocolo com controles de memorização**. Não foi executada neste documento: a especificação operacional completa e sua aprovação permanecem pendentes. Nenhum código, teste, plano ou protocolo histórico foi alterado. Verificações desta entrega: leitura das referências, revisão documental/aritmética, escopo do diff e whitespace; não houve execução Rust ou nova contagem de testes.
