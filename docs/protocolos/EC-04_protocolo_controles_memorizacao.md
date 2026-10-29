# EC-04 — Protocolo com controles de memorização

## 1. Estatuto, base e hipótese

Especificação documental prospectiva concluída em 2026-10-29, base `d4fb08ddcffee38a0e21b3800817345498eb5d68`, branch `devs`. Subetapa literal do plano cognitivo v0.2: **protocolo com controles de memorização**. Referências: Fundamento aprovado, `docs/EC-04_auditoria_relacoes_existentes.md`, `docs/EC-04_tarefas_exemplos_nao_cadastrados.md`, decisões e ressalvas EC-03.5. Preservados os tamanhos, limiares e distinções das tarefas anteriores; nenhum protocolo histórico foi editado.

H-R: memória exata distingue registro presente de entrada sem registro. H-C: regra conhecida compõe dois fatos sem cadastrar a conclusão. H-A: experiências rotuladas identificam um critério dentro de família fornecida, alterando respostas a vetores não adquiridos. H-V: experiências posteriores incompatíveis podem revisar o critério utilizado no mesmo contexto, preservando o contexto não alterado.

O candidato conceitual aqui especificado é **simbólico**, delimitado e reversível; não altera arquitetura de produção nem escolhe a arquitetura definitiva do Neurons. O resultado, se positivo, será indução/aplicação contextual sob viés fornecido e professor externo, não formação semântica autônoma ou aprendizagem neural. Transferência estrutural permanece não avaliada.

Esta entrega congela regras, procedimento de seleção, alvos do professor, controles e previsões, **não os dados finais concretos**. A restrição da delegação impede gerá-los agora. Sua materialização determinística e os hashes de fixtures serão pré-condições da futura implementação/execução autorizada, sem modificar este protocolo. Não há protótipo, execução ou resultado experimental nesta entrega. Versionamento não equivale a autorização de execução.

## 2. Interfaces, identidade e estados iniciais

Objetos conceituais: vetor `x` de oito bits (p0 é bit menos significativo), contexto opaco, ID de episódio, rótulo booleano e procedência da aquisição. Um critério é par de máscaras `(M,V)`, com um a três bits em M, V subconjunto de M; satisfaz x se `(x & M)==V`. A família H contém 576 critérios, ordenados por M crescente e depois V crescente. Não há critério vazio, literal repetido ou contraditório. As posições relevantes e V do alvo não entram na interface do aprendiz.

Operações: Acquire(contexto, episódio, x, rótulo), Query(contexto,x), e consulta de proveniência de estado por versão. A classificação não recebe ID de objeto, índice de cenário, fase, pontuação, alvo ou rótulo esperado. Contexto seleciona estado separado; não é atributo de classe. Cada condição começa sem episódios, com H completo para cada contexto declarado e memória literal vazia. Estado por contexto: log imutável de episódios adquiridos, início do sufixo ativo, conjunto compatível, versão local incrementada somente por Acquire aceito. Contextos não compartilham parâmetros aprendidos; compartilhar código não significa compartilhar pesos.

Rótulos e alvos ficam no professor/avaliador. Query só recebe contexto/x e devolve Positive, Negative ou Abstain(Ambiguous/Missing/Conflict), ou Inconclusive(reason). Também devolve versão do estado e referências ao sufixo usado. Não retorna escore que esconda abstenção. O modelo não acessa manifestos de partições ou o código do oráculo. Estado inicial em qualquer vetor: H completo contém previsões opostas, portanto candidato e controle de indução se abstêm.

T-R usa chave simbólica opaca, contexto, versão e valor não nulo; T-C usa fatos dirigidos positivos, contexto, IDs e a regra L fornecida. Não converter essas saídas em rótulos conceituais: T-R retorna Value/Absent/Conflict/Inconclusive; T-C retorna Supported(proof)/Unsupported/Conflict/Inconclusive. Unsupported significa ausência de prova no escopo, não negação universal.

## 3. Atualização e revisão conceitual congeladas

Ao adquirir um episódio, validar contexto, ID, vetor, rótulo e capacidade antes de qualquer alteração. Rejeitar ID repetido ou vetor já adquirido no mesmo contexto, mesmo com outro rótulo: as partições exigem novidade de vetores; corrigir um episódio é operação fora deste protocolo. Episódios de validação/final não possuem autorização Acquire e devem ser rejeitados se entregues a essa operação. Erros não mudam estado nem contadores de conteúdo.

Candidato: acrescentar o episódio ao log prospectivo; encontrar o **maior sufixo consecutivo** do log desse contexto para o qual existe ao menos um critério de H consistente com todos os rótulos. Equivale a retirar os episódios mais antigos do conjunto ativo, um por vez, até H compatível ficar não vazio. Conservar integralmente os episódios retirados com suas referências; eles deixam de ser restrições atuais, não viram fatos falsos ou evidência apagada. Publicar atomicamente log, início do sufixo, conjunto compatível e versão; interrupção por orçamento conserva o estado anterior.

A política é uma hipótese programada de recência para ambiente não estacionário. Não representa confiança estatística nem descoberta de mudança causal. Não fornecer sinal BeginRevision, novo alvo, máscara ou reset do professor. O mesmo Acquire é usado antes/depois da mudança; a inconsistência dispara a revisão interna. Critérios não são escolhidos por ID ou primeiro elemento de H.

Query: avaliar todos os critérios do conjunto compatível. Se todos positivos, Positive; se todos negativos, Negative; se divergirem, Abstain(Ambiguous). Contexto inexistente: Abstain(Missing). Conjunto vazio fora de atualização atômica: Inconclusive(InvalidState), nunca escolha arbitrária. A resposta não modifica log, conjunto, versão, contadores do aprendiz ou referências.

Proveniência: toda resposta conceitual referencia contexto/versão, intervalo de IDs do sufixo ativo e versão da família/regra programada. Conservar o log para verificar essa referência e distinguir episódios descartados das restrições atuais. Não chamar esse modelo de implementação completa da proveniência EC-03. Fontes são episódios sintéticos do professor declarado, sem autenticação externa.

## 4. Professor, contextos e alvos — exclusivos do avaliador

Quatro blocos finais independentes, cada um com contextos A (alterado) e B (estável). As expressões abaixo são parâmetros do professor, não dados de entrada do componente. `+pi` significa pi=1 e `-pi`, pi=0.

| Bloco | Critério inicial A | Critério inicial B | Acréscimo posterior em A |
|---|---|---|---|
| 1 | +p0 e +p1 | +p2 e -p3 | +p4 |
| 2 | -p2 e +p3 | +p4 e +p5 | -p6 |
| 3 | +p4 e -p5 | -p6 e -p7 | +p0 |
| 4 | -p6 e -p7 | -p0 e +p1 | -p2 |

A função do professor inicial usa dois literais; a posterior de A usa os mesmos dois mais o terceiro. B não muda. Os rótulos são calculados exclusivamente ao formar episódios adquiridos ou pontuar respostas. Os alvos públicos ao autor não tornam o estudo cego: exige-se isolamento de runtime e independência de implementação, não alegar independência de autoria/equipe.

Desenvolvimento conceitual usa um bloco próprio: A=+p1 e -p4, B=+p0 e +p6, acréscimo A=+p7. Validação de implementação usa outro: A=-p3 e +p5, B=+p2 e -p4, acréscimo A=-p0. Mesmos tamanhos e sequência; namespaces distintos. Não usar blocos finais para desenvolvimento. Os vetores podem coincidir entre contextos diferentes; a garantia de novidade é por contexto/conceito, conforme a definição anterior. Não alegar novidade global de padrões ou transferência entre contextos.

IDs de contexto não são atributos de decisão: namespaces D/V/F e numeração sequencial somente no avaliador; interface pode usar inteiros opacos 1001+2*b e 1002+2*b para blocos finais b=0..3, 2001/2002 em desenvolvimento, 3001/3002 em validação. O classificador não recebe b nem a tabela. Episódios recebem ID sequencial local não correlacionado com rótulo por estratos; IDs servem somente à proveniência.

## 5. Seleção dos dados — procedimento futuro, sem materialização nesta etapa

O avaliador materializará os manifestos somente após autorização posterior. Não incluir tabelas finais prontas neste documento. Enumerar o universo de 256 vetores por ordem de prioridade `rank(x)=(197*x+101) mod 256` crescente, uma permutação determinística; desempate pelo valor de x. Toda seleção abaixo escolhe a primeira opção nessa ordem que atende às restrições declaradas e preserva as reservas seguintes. Não consultar desempenho do candidato para escolher dados.

### 5.1 T-A, por contexto

Aquisição 32 (8 positivos/24 negativos), validação de dados 16 (4/12), final 32 (8/24); 80 vetores distintos. Todos os positivos satisfazem os dois literais do alvo. Dividir negativos em três estratos: somente primeiro literal falso, somente segundo falso e ambos falsos; cada estrato tem 64 vetores.

Aquisição positiva: escolher quatro pares distintos de vetores que satisfazem o alvo e são complementares nas seis propriedades não utilizadas pelo alvo; nenhum vetor se repete. Os oito exemplos têm somente os dois literais-alvo constantes em comum. Negativos de aquisição: oito de cada estrato. Validação negativa: quatro de cada; final negativo: oito de cada. Positivos de validação/final são distintos dos adquiridos, sem nova condição de generalização estrutural.

Em A, reservar conjuntamente a revisão: positivos iniciais distribuem-se igualmente entre satisfazer ou não o futuro terceiro literal (aquisição 4/4, validação 2/2, final 4/4). Isso é estratificação pelo avaliador, não informação entregue ao aprendiz. Em B não há terceiro literal e não se aplica essa subdivisão.

### 5.2 T-V, somente contexto A

Reservar outros 40 vetores, disjuntos dos 80 iniciais: aquisição de revisão 16 (2 positivos/14 negativos), validação de revisão oito (1/7), final de revisão 16 (2/14).

Os dois positivos adquiridos de revisão formam um par complementar nas cinco propriedades não utilizadas pelo novo alvo de três literais. Entre os 14 negativos adquiridos: oito satisfazem o alvo antigo mas violam o terceiro literal; seis já eram negativos. Entre esses seis, incluir um testemunho com somente o primeiro literal-alvo falso e terceiro verdadeiro e um com somente o segundo falso e terceiro verdadeiro. Os quatro restantes são os primeiros vetores elegíveis não usados. Validação de revisão: quatro negativos que mudaram de classe e três negativos sob ambos; positivo satisfaz os três literais. Final de revisão: exatamente oito negativos que mudaram de classe, seis negativos sob ambos e dois positivos sob ambos, preservando a definição anterior.

Selecionar conjuntamente todas as partições de A, não formar a aquisição de revisão a partir de pontuações ou respostas finais. Seleção canônica: primeira atribuição lexicográfica completa aos slots de aquisição, validação e final iniciais, depois aquisição, validação e final de revisão, sob todas as restrições; pares são slots agrupados e internamente ordenados por rank. Usar busca finita com poda de quotas e disjunção, máximo de 2.000.000 expansões por contexto, cada expansão é uma escolha de vetor para um slot. Se não houver atribuição nesse limite, parar como Inconclusive(DataConstruction), sem relaxar dados, substituir alvos ou favorecer candidato. Não executar essa busca nesta entrega.

As condições são matematicamente plausíveis sem gerar instâncias: por alvo inicial há 64 positivos e 192 negativos; por alvo revisado, 32 positivos, 32 casos que mudam e 192 negativos antigos. Consumo de positivos iniciais em A: dez em cada subgrupo; revisão consome cinco positivos e vinte casos que mudam, restando espaço. As testemunhas da aquisição tornam o critério único em H: os pares positivos eliminam literais irrelevantes; negativos testemunham cada literal necessário. A existência e os hashes da atribuição concreta ainda devem ser conferidos no preflight futuro; as contagens isoladas não são prova de execução da busca dentro do limite.

Sondas de retenção B: subconjunto de 16 exemplos do seu final T-A, primeiros quatro positivos e primeiros 12 negativos por rank. Não são novas aquisições nem novo corpus; são consultas repetidas para conservação. Não adicionar 16 vetores à partição B de 80.

### 5.3 Inclusão, exclusão e identificabilidade

Excluir sobreposição indevida, IDs duplicados, vetores fora de 0..255, rótulos inconsistentes com professor, quotas incorretas e consulta final cadastrada como resposta. Identificabilidade é verificada pelo avaliador enumerando H sobre somente os exemplos adquiridos: conjunto consistente com os 32 iniciais deve conter exatamente o alvo inicial; conjunto consistente com os 16 de revisão, exatamente o novo alvo. Não fornecer esse conjunto ou máscara resultante ao aprendiz. Sem identificabilidade: inconclusão de desenho antes de pontuar o mecanismo.

O manifesto concreto e seus hashes não existem nesta subetapa. Futuramente registrar SHA-256, tamanho, partição e versão de professor de cada arquivo; entradas e gabaritos finais separados do código dos modelos. A procedência desses manifestos é seleção sintética determinística, não corpus externo ou revisão independente.

## 6. T-R — aquisição e consulta exatas

Um contexto e versão 1 por laboratório: contextos 4101, 4102 e 4103 para desenvolvimento, validação de implementação e painel final, respectivamente. Esses contextos não compartilham memória. Gerador futuro atribui 16 chaves opacas distintas e 16 valores positivos distintos; usar posições i=0..15 apenas no avaliador: chave `101+37*i`, valor `1009+53*i`. O componente recebe os 16 pares, não a fórmula; não há alegação de aprendizagem de valores. Ordenar aquisição por chave crescente.

Quatro primeiras chaves adquiridas são âncoras de validação; as oito seguintes são âncoras finais. Ausentes: quatro chaves de validação `10001+37*i` e oito finais `20001+37*i`, com índices iniciados em zero em cada grupo. Não instanciar as listas agora. Validação tem oito consultas e final 16; chaves consultadas são distintas entre validação/final. Consultar cópias independentes do mesmo estado adquirido; nenhum registro é criado pelas consultas.

Candidato: memória exata limitada a 16 pares, mesma chave/contexto/versão. Ausência: Absent; dois valores opostos/duplicados na entrada de aquisição: rejeitar lote antes de aplicar, não escolher primeiro. Essa validade é pré-condição, não novos casos finais. Êxito: oito âncoras finais corretas e oito ausências reconhecidas; rótulos e contadores de classe aqui são presente/ausente, não conceitos booleanos.

## 7. T-C — fatos fornecidos e regra de dois passos

Cada grafo tem oito nós locais a,b,c,d,e,f,g,h, seis arestas: a→b, b→c, d→e, e→f, g→h, b→h. Duas rotas disjuntas a→b→c e d→e→f; distratores não chegam aos destinos c/f das rotas opostas. L é somente a relação sintética definida por esse ambiente. Regra fornecida: dois fatos positivos consecutivos sustentam a consulta; nenhuma inferência recursiva.

Quatro consultas na ordem a→c, a→f, d→f, d→c: Supported, Unsupported, Supported, Unsupported. Conclusões não são fatos cadastrados; manter referências às duas premissas. Semântica do alvo é existência dessa prova limitada, não transitividade de qualquer relação. Essa topologia repetida testa instanciação de estrutura conhecida, não transferência estrutural.

Quatro grafos de desenvolvimento, dois de validação e oito finais, com identidades disjuntas. Índice global g=0..13 nas faixas 0..3,4..5,6..13; nó local k=0..7 recebe ID `10000+32*g+k+1`, contexto `5000+g`, fato de posição j=0..5 recebe ID `1000+16*g+j+1`. Não materializar instâncias finais agora. Adquirir as seis arestas na ordem listada e consultar somente após aquisição completa. Reset total por grafo.

Candidato futuro: reutilizar TwoHopTask/EngramStore existentes em suporte de teste, max_facts=8, fila=32 e orçamento=128 eventos por consulta. Não modificar produção. O controle independente calcula a existência de dois passos sem chamar a decisão ou propagação do candidato. Controle de regra programada usa o mesmo grafo e regra, com implementação própria. Nenhuma regra é aprendida nessa tarefa. Êxito: 16/16 consultas finais sustentadas com provas válidas e 16/16 sem prova com abstenção Unsupported; erro de execução é Inconclusive, não ausência.

## 8. Condições e informação dos controles

Os controles abaixo são escolhas experimentais, não funcionalidades a implementar nesta subetapa. Todos recebem as mesmas consultas e oportunidades; ablações deliberadas de informação são identificadas como tal. Nenhum modelo lê professor/gabarito, e nenhum controle chama validação, atualização ou decisão de outro. Compartilhar somente DTOs passivos.

| Tarefa | Condições fixadas |
|---|---|
| T-R — sete | Candidato, literal, sem aquisição, sem atualização, executor simbólico independente de memória, sempre Value(0), sempre Absent |
| T-C — oito | Candidato, literal direto, sem aquisição, sem atualização, executor simbólico independente, regra programada independente, sempre Supported sem prova, sempre Unsupported |
| T-A/T-V — nove | Candidato, literal, sempre Positive, sempre Negative, sempre Abstain, sem aquisição, sem atualização, indução simbólica independente, regra simbólica fixa +p7 |

Literal T-R tem a mesma memória; literal T-C consulta só aresta direta, retornando Unsupported nas consultas sem registro; essa diferença de capacidade deve ser declarada. Literal conceitual conserva todos os exemplos adquiridos e consulta somente igualdade `(contexto,x)`, sem inferência e sem usar ID de objeto. Final disjunto implica Abstain(Missing), não classe negativa. Em T-V ele também recebe os 16 episódios de revisão e não recebe finais.

Sem aquisição: não recebe/conserva episódios. Sem atualização: recebe episódios, conserva-os como log não consultável para decidir e mantém estado inicial em T-R/T-C/T-A. Para T-V, é um ramo criado da cópia integral do estado principal adquirido antes da revisão; recebe/conserva os novos episódios em log à parte, mas decide com o estado congelado anterior. Recebe a mesma informação de treinamento e não a utiliza por ablação explícita. Resultados da condição T-A sem atualização não se confundem com esse ramo T-V.

Indução simbólica independente: para cada sufixo possível, do mais longo ao mais curto, enumerar H diretamente sobre os episódios e selecionar o primeiro sufixo não vazio; consulta por unanimidade própria. Candidato pode manter/intersectar conjuntos, mas não compartilhar funções com o controle. Validação de entradas também independente. Ambas recebem a mesma família, história, contexto e orçamento, sem alvo. Previsão de igualdade semântica não é exigência de superioridade nem equivalência geral de algoritmos.

Regra fixa +p7 não recebe máscara específica do alvo nem se adapta aos exemplos. Sua correção depende dos manifestos; congelar a regra antes deles e pontuar sem prometer um número não derivado. O professor com alvo pronto é apenas oráculo, não baseline equivalente. Resposta sempre Supported sem premissas em T-C é erro explícito nas positivas e resposta indevida nas negativas, nunca prova válida.

## 9. Ordem temporal e isolamento de avaliação

Preparação futura, após autorização: materializar os dados pelo procedimento fixado, verificar quotas/disjunção/identificabilidade, registrar hashes e custódia; desenvolver somente nos namespaces de desenvolvimento e depois validar implementação no namespace de validação. Congelar código de candidato, controles, avaliador e manifestos antes da primeira consulta final. Nenhum ajuste depois de observar final; troca de mecanismo ou dados exige revisão identificada e nova reserva, sem apagar resultados.

Painel de avaliação, com todos os dados já congelados: condições na ordem da tabela, tarefas T-R, T-C, depois blocos 1..4 de T-A/T-V. Cada condição tem estado independente. Executar duas repetições completas do vazio; congelar e registrar também as ordens. Dentro de cada conjunto de aquisição, ordenar episódios por rank(x), sem estratos/rótulos no payload; desempate por x. Em T-R/T-C usar ordens próprias definidas acima.

Por bloco/condição conceitual:

1. Criar estados vazios de A e B; consultar os 32 finais iniciais de cada contexto (A0). Gabaritos ficam somente no avaliador; não devolver pontuações nem alterar estado.
2. Adquirir 32 episódios de A, depois 32 de B; validar somente leitura nos 16 itens de validação de cada contexto. Consultar os mesmos 32 finais iniciais por contexto (A1).
3. Medir antes da revisão nos 16 finais de revisão de A e nas 16 sondas de B (V0); registrar decisões, referências e estado. Criar o ramo sem atualização T-V da cópia de A1 do candidato.
4. Professor muda somente a regra usada para rotular os próximos episódios de A, sem aviso ao modelo. Entregar os 16 episódios predeterminados de revisão a cada condição; nenhuma aquisição em B. No ramo sem atualização conservar estado A1; na condição sem aquisição continuar vazio.
5. Consultar os oito itens de validação da revisão, os 16 finais de revisão A e as mesmas 16 sondas B (V1). Verificar B sem qualquer mudança de decisões e referências. Não entregar nenhuma observação avaliativa à aquisição, nem posteriormente.

T-R: aquisição 16, validação oito em cópia isolada e final 16 em outra cópia. T-C: validação nos dois grafos próprios e final nos oito, todos reiniciados. Desenvolvimento é preparação separada, não conta como painel final. Nos blocos conceituais D e V executar uma única vez a mesma sequência A0/A1/V0/V1, com suas partições locais de 32/16/32 e 16/8/16; suas consultas de formato final são sondas de preparação, não dados F. Em T-R, D/V executam aquisição, oito consultas de validação e 16 sondas locais, com inventários e contextos próprios. T-C usa quatro grafos em D e dois em V. A preparação D soma 2384 consultas (168 T-R, 128 T-C, 2088 conceituais); V soma 2320 (168, 64, 2088). Executar D antes de V; se houver ajuste em D/V, repetir preparação e recongelar antes do primeiro final F, sem usar F para esse ajuste. A reutilização dos grafos V em validações associadas ao painel deve ser declarada, sem chamá-los dados novos. Não chamar consultas de validação associadas aos contextos finais de treino: não fornecem rótulos, escolha de critério ou ajuste ao modelo. Seus resultados são diagnósticos somente, pois código está congelado.

Em toda consulta: registrar estado/versão antes, produzir e registrar resposta/referências, só depois pontuar no avaliador; exigir igualdade do estado antes/depois. Contexto e vetor não incluem categoria, rótulo ou partição; o mecanismo não recebe A0/A1/V0/V1. As regras de atualização são fixadas antes de qualquer pré-avaliação final; experiências posteriores são predeterminadas, nunca selecionadas pelos erros finais.

## 10. Contagem congelada de consultas

As contagens são chamadas de avaliação, não testes Cargo ou observações independentes. Nenhum final foi gerado para calculá-las.

| Bloco de avaliação | Por repetição, incluindo condições | Duas repetições |
|---|---:|---:|
| T-R final: 16 ×7 | 112 | 224 |
| T-C final: 32 ×8 | 256 | 512 |
| T-A finais A0/A1: 4 blocos ×2 contextos ×32 ×2 fases ×9 | 4608 | 9216 |
| T-V finais V0/V1: 4 blocos ×(16 A +16 B) ×2 fases ×9 | 2304 | 4608 |
| **Subtotal de consultas finais** | **7280** | **14560** |
| T-R validação: 8 ×7 | 56 | 112 |
| T-C validação: 2 grafos ×4 ×8 | 64 | 128 |
| T-A validação associada: 4 ×2 ×16 ×9 | 1152 | 2304 |
| T-V validação associada: 4 ×8 ×9 | 288 | 576 |
| **Total do painel, finais + validações** | **8840** | **17680** |

Há 320 episódios conceituais oferecidos por condição/repetição (4×(32+32+16)); condições sem aquisição são intervenções que não os recebem. Controles podem ignorar episódios conforme sua definição. Os ramos congelados T-V reutilizam A1, não recontam aquisição como nova aprendizagem. Cópias e igualdade/proveniência são assertivas adicionais, não consultas extras. Publicar separadamente desenvolvimento, validação de implementação, validações associadas, finais e repetições.

## 11. Métricas, previsões e critérios falsificáveis

Registrar por tarefa, bloco, contexto, fase e condição: acertos por classe, erros explícitos por classe, cobertura, abstenções por motivo e inconclusões. Em T-R classes são presente/ausente; em T-C, com/sem prova; em T-A/T-V, positivo/negativo. Não chamar abstenção correta de previsão positiva ou de classe negativa. Uma resposta sem prova exigida é incorreta mesmo se o booleano coincidir.

Acurácia balanceada conceitual = metade da soma de acertos positivos/Npositivo e acertos negativos/Nnegativo, incluindo abstenções como não acerto. Precisão entre respostas é acertos/(respostas explícitas), indefinida se zero respostas. Cobertura = respostas explícitas/N. Erros explícitos e abstenções devem permanecer em colunas separadas. Retenção B mede igualdade exata de decisão e referências em V0/V1; revisão A mede ganho, erros antigos corrigidos e novos erros introduzidos.

Previsões analíticas, condicionadas ao preflight e à implementação correta:

- T-R: candidato/literal/simbólico recuperam 8/8 e reconhecem 8/8 ausências; sem aquisição/atualização e sempre Absent perdem as oito âncoras; Value(0) não acerta valores não nulos e responde indevidamente a ausências.
- T-C: candidato, simbólico e regra programada produzem 16 provas e 16 Unsupported. Literal, sem aquisição/atualização e sempre Unsupported não produzem as 16 provas. Sempre Supported sem prova não satisfaz positivos nem negativos. Isso não é comparação informacionalmente equivalente com memória direta.
- T-A: candidato/indução começam com 32 abstenções por contexto (A0) e, com aquisição identificável, podem chegar a 32/32 (A1). Literal e condições sem aquisição/atualização se abstêm nas 32 entradas novas. Sempre positivo/negativo têm acurácia balanceada 0,5, ainda que sempre negativo acerte 24/32 na acurácia simples; sempre abster-se tem zero acerto.
- T-V: critério antigo correto em V0 acerta os dois positivos e seis dos 14 negativos, erra oito: acurácia balanceada 5/7. Candidato/indução identificados pelo novo sufixo podem atingir 2/2 e 14/14 em V1, ganho 2/7. O ramo sem atualização preserva 5/7. Literal continua sem correspondência. Em B, decisões e referências não mudam porque parâmetros são separados, não por resistência neural à interferência.

Essas previsões não são resultados nem licença para adaptar expectativas. A regra fixa +p7 é pontuada pela sua função; publicar coincidências/diferenças após os dados, sem escolher outra regra para melhorar candidato. Controles exatos podem superar mecanismos mais complexos sem que isso seja ocultado.

Critérios preservados das tarefas: T-R 8/8 e 8/8; T-C 16/16 e 16/16 com prova; T-A pelo menos 7/8 positivos e 21/24 negativos, mudança interna rastreável e ganho balanceado >=0,25 frente a sem atualização, **por cada um dos oito alvos/contextos**, sem compensar falha pela média; T-V por bloco 2/2 positivos e pelo menos 13/14 negativos, ganho >=0,25 frente ao ramo congelado, e 16/16 sondas B com decisões/referências iguais. O limiar T-V é coerente: (1+13/14)/2 -5/7 =1/4. Informar também resultado exato previsto de candidato simbólico, não apenas aprovação por limiar.

**Êxito delimitado:** todos os critérios por tarefa e bloco, invariantes e limites atendidos, controle independente semanticamente concordante e repetição íntegra. Sustenta recuperação/composição em T-R/T-C e indução/revisão contextual nesta família em T-A/T-V. Não se exige superioridade sobre controle simbólico equivalente.

**Resultado negativo válido:** preflight e execução válidos não satisfazem limiares, transformação por experiência ou retenção; preservar erros, abstenções e eventual equivalência ao controle sem atualização. Discordância analítica reproduzida com ambas implementações válidas deve ser relatada, nunca resolvida mudando alvos.

**Inconclusão técnica/metodológica:** vazamento, identificabilidade insuficiente, falha de construção dos dados, referência inválida, discordância entre executores sem explicação, mutação por consulta ou orçamento excedido. Interromper; não absorver erro como classe negativa, ausência de prova ou episódio de aprendizagem. Se causa exigir mudar protocolo, solicitar revisão antes de repetir.

## 12. Orçamento, instrumentação e interrupção

Somente em futura execução autorizada: Rust/std, local, sem threads próprias, serviços, dependências externas ou mudança de produção. Preferir suporte de teste; perfis release são permitidos para o painel existente a implementar, sem benchmark ou tarefa adicional.

| Recurso | Limite congelado |
|---|---|
| Vetor / família | 8 bits /576 critérios |
| Contextos conceituais simultâneos | Dois por bloco; reset entre blocos |
| Episódios adquiridos por contexto | Até 48 A, 32 B; logs externos de validação/final não entram na aquisição |
| T-R | 16 associações; consultas não expandem memória |
| T-C | 8 nós, 6 arestas; fila 32; 128 eventos por consulta |
| Trabalho conceitual por Acquire, incluindo busca de sufixo | Até 4.000.000 avaliações de critério, comparações de vetor/ID ou visitas de referência |
| Trabalho por Query conceitual/literal | Até 8192 avaliações/visitas, inclusive resposta abortada |
| Construção de dados futura | Até 2.000.000 expansões por contexto, sem regeneração por desempenho |
| Dados vivos e fixtures por condição | Até 256 KiB, incluindo logs e repetições coexistentes |
| RSS do painel isolado | Inferior a 256 MiB |
| Tempo do painel completo de 17680 consultas | Até 30 s, incluindo aquisição, cópias, controles e repetição; geração e preparação medidas separadamente, cada laboratório preparatório <=30 s |
| Disco livre em /home | Pelo menos 3 GiB antes e durante |

Uma avaliação de critério calcula `(x&M)==V` e conta uma unidade; cada leitura adicional de episódio/ref e comparação de identidade conta uma unidade. Contar trabalho de rejeições antes de retornar erro; não usar fórmulas não uniformes diferentes entre candidato e controle. O orçamento é igual, o custo efetivo não precisa ser. Nenhuma saída parcial conta como decisão sustentada. Uma ultrapassagem aborta a operação sem publicar novo estado; a aquisição não pode ficar parcialmente revisada.

Manter logs compactos por referências: no máximo 64 bytes por linha de consulta; detalhes de episódios e versões em tabelas separadas conservadas. O plano de memória deve contar capacidades reais, estados temporários, clones, contextos e repetições simultâneas antes de executar. Se não couber, parar para revisão, não perder evidência nem declarar prova de orçamento apenas por RSS. Não manter registros repetidos desnecessários; comparação de repetibilidade pode ser sequencial, com registros compactos exatos. Hash de resumo sozinho não substitui as evidências exigidas.

RSS: registrar pico reportado pelo sistema para o processo isolado e escopo do lançador, além de amostragem/guardas; tempo monotônico; crescimento de target e espaço antes/depois. Não instalar instrumentos. Instrumentação insuficiente limita a alegação correspondente, não autoriza presumir atendimento. Não forçar rebuild para medir compilação limpa.

Apenas T-C do candidato utiliza Circuit/Engine. Previsão de contagem unitária pela topologia: a→c e a→f ativam a,b,c,h, quatro eventos cada; d→f e d→c ativam d,e,f, três cada. Assim, 14 eventos por grafo do candidato com fatos, 112 finais +28 validação por repetição, **280 eventos de circuito em duas repetições**. Controles simbólicos não são contados como eventos neuronais, e componentes vazios não precisam iniciar Circuit para declarar Unsupported. O teto conservador do candidato com dez grafos ×quatro consultas ×128 ×duas repetições é 10240 eventos. Diferença efetiva da previsão exige investigação, sem alterar parâmetros; desenvolvimento conta separadamente (quatro grafos, previsão 56 eventos por execução).

## 13. Custódia, reprodução e verificação futura

Separar arquivos do modelo, controles, gerador/manifestos e avaliador. Modelos não importam o oráculo nem recebem structs com campos-alvo disfarçados; a validação dessa fronteira exige inspeção independente de chamadas. A tabela pública deste protocolo e alvos conhecidos ao autor impedem alegação de cegueira de autoria; declarar isso no relatório.

As partições são selecionadas conjuntamente pelo professor antes do primeiro teste final, não em resposta a erros. Registrar integridade por SHA-256 de protocolo, código e manifestos antes de consultar final. O gerador não é executado nesta delegação; eventual manifesto posterior não será apresentado como documento existente hoje. Sem acesso indevido a rótulos não significa independência de dados/equipe: são exemplos sintéticos do mesmo processo.

Repetir duas vezes do estado vazio, mesmos dados e ordens, comparando decisões, abstenções, hipóteses compatíveis, sufixos, referências, erros e contadores; excluir somente medições de tempo/RSS da igualdade. Preservar saídas desfavoráveis. Validações de implementação precedem os blocos finais; validações associadas às condições finais são apenas medições fixas do painel, sem ajuste. Publicar ambos os escopos claramente.

Antes da execução futura, confirmar branch/base autorizadas, arquivos previstos e guardas; verificar hashes e originalidade das partições; depois executar o painel, regressões pertinentes e revisão de escopo. Se qualquer requisito não puder ser satisfeito sem alterar o protocolo, interromper. Nenhuma autorização de implementação decorre deste texto.

## 14. Conclusões permitidas, decisões e encerramento documental

Um resultado positivo em T-A/T-V poderá sustentar que restrições adquiridas de exemplos rotulados alteraram classificações de vetores não adquiridos e foram revistas neste ambiente. Será aprendizagem simbólica delimitada com representações, atributos e viés fornecidos. T-R é recuperação; T-C é composição por regra pronta. Nenhum dos quatro demonstra formação autônoma de significado, percepção, descoberta livre de conceitos, aprendizagem neural, transferência estrutural, competência pedagógica geral ou consciência.

A política de sufixo, os alvos, a construção de exemplos e os controles são decisões experimentais prospectivas desta especificação. Não são arquitetura universal, adaptação de produção ou resultados históricos. Foram escolhidos para tornar a dependência de experiência verificável sem esconder a capacidade do controle simbólico. Limitações EC-03/A02 permanecem intactas.

Protocolo documental concluído e congelado por esta versão e seu commit. Verificações nesta entrega: leitura das referências, revisão lógica e aritmética de tamanhos/limiares/consultas, diff e whitespace. Não foram gerados conjuntos finais, executados algoritmos de indução ou criado código/teste. O SHA do próprio arquivo pode ser calculado no relatório sem inserção autorreferente.

Próxima subetapa literal da EC-04: **implementação isolada**, somente com autorização explícita. A avaliação de discriminação, composição e exceções permanece posterior; nenhuma dessas atividades foi iniciada nesta entrega.
