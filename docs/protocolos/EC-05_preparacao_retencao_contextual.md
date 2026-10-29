# EC-05 — Retenção contextual: adoção de política e preparação prospectiva

## 1. Autorização, requisito e estado real

Base verificada: `07bf036e3fd3e650c6034d7b0f55ea2ddca44694`, branch devs, árvore inicialmente limpa; HEAD, origin/devs e referência remota coincidentes. A delegação **EC-05 — Decisão científica, fundamentos e retenção contextual** aprova P1 e autoriza preparar o protocolo e executar somente se não houver ambiguidade científica substantiva.

Subetapa literal do [plano cognitivo v0.2](../../neurons_plano_executivo_evolucao_cognitiva_v0_2.md), §5: **avaliar retenção sem misturar versões de contexto**, última subetapa da EC-05 — Contexto, validade e revisão. Dependências EC-03/04; aceite da etapa: conclusões revisadas mediante evidência, identificação de exceções e ausência de extrapolação indevida. Não se inicia EC-06.

**Estado: preparação documental; protocolo operacional ainda não congelado. Implementação e execução não iniciadas.** Há duas definições operacionais indispensáveis em §6; não são pedidos de permissão administrativa nem reversão da adoção de P1. As matrizes abaixo delimitam casos e previsões condicionais, não resultados ou painel final pronto. Não foram gerados dados finais, implementados mecanismos ou reexecutados testes históricos.

## 2. Decisões científicas agora adotadas

1. **P1 — elegibilidade conservadora com manutenção explícita:** política experimental delimitada EC-05; não política cognitiva universal. Query é somente leitura; diagnosticar, responder e modificar memória são responsabilidades distintas. Manutenção/revisão exige comando e condições explícitas, sem sobrescrever o histórico.
2. **P2 — recomposição pura sob demanda:** controle forte e independente, com os mesmos fatos, fontes, regras, contextos, versões e cortes disponíveis. Não depende de Refresh de P1. Sua recomposição transitória não publica atualização de memória.
3. **P3:** restrita ao contrato EC-04, sem importação automática de recência. **P4:** não adotada, sem calibração suficiente.
4. **Conflito seletivo:** isolar apenas vias conflitantes demonstravelmente separáveis; usar alternativa somente se válida, suficiente e independente. Se independência ou suficiência não estiver demonstrada, abster-se integralmente. Não eleger fonte por recência, ID, maioria ou preferência arbitrária.
5. Preservar proveniência, histórico, condições de manutenção e causas completas. Divergências P1/P2 são medidas e investigadas, não automaticamente classificadas como erro.

Esta decisão posterior resolve a escolha P1/P2 da [proposta histórica](../EC-05_politica_revisao_abstencao.md), §10, e modifica prospectivamente a abrangência do bloqueio por conflito ali recomendada. Não reescreve esse documento, o contrato EC-03.3-R1 ou seus resultados. A aprovação conceitual da separação seletiva ainda exige um teste operacional de independência, não presente na interface histórica.

## 3. Fontes e fronteiras de reutilização

Referências examinadas: plano cognitivo v0.2, [Fundamento](../fundamentos/Neurons_Fundamento_Cognitivo_Metacognitivo_Pedagogico_Aprovado.md), decisões EC-03/04/05, modelagem do escopo e política EC-05, [protocolo diagnóstico EC-05](EC-05_protocolo_diagnostico_contextual.md), [resultados](../EC-05_resultados_diagnostico_contextual.md), contratos EC-03.3 original/R1 e auditorias EC-03.5/EC-04. As [diretrizes complementares A1–A7](../fundamentos/Neurons_Diretrizes_Complementares_Memoria_Pedagogia_Idiomas.md) são normativas, não fatos adquiridos pelo aprendiz.

| Evidência técnica atual | O que permite reutilizar | O que não oferece |
|---|---|---|
| `tests/support/provenance_types.rs`, Key/Fact/Rule/Proof/Query/Answer | IDs de objetos versionados, referências exatas, contexto u32, época, validade [start,end), corte e causas. | Objeto de contexto versionado, vínculo entre suas gerações ou certificado de independência de vias. |
| `provenance_candidate.rs:294–326` | Revalidação de provas materializadas e registro da oposição. | Isolamento seletivo: qualquer oposição na conclusão ou premissa de prova admissível gera Conflict para a consulta inteira. |
| `provenance_literal.rs:351–384` | Recomposição independente do Refresh, com leitura do diário permitido. | Também bloqueia globalmente por conflito; não implementa a nova regra de isolamento seletivo. |
| EC-03.3-R1 §§3–5 e EC-05 diagnóstico §§2–4 | Separação corte/época, dois passos, histórico por replay, manutenção explícita, suporte alternativo após retirada/expiração. | Critério completo para conflito em uma via com outra via alternativa limpa. Expiração não é oposição. |
| EC-05, trajetórias F/G | Conservação do contexto não afetado e recuperação de decisões em cortes antigos. | Retenção longitudinal geral, versões semânticas de contexto ou resistência à interferência neural. |

Contagem histórica consultada: **136 testes**, 516 verificações diagnósticas, 18 consultas históricas e 12 comparações do contexto não afetado. O literal teve seis respostas sustentadas adicionais pré-Refresh nas duas repetições. São resultados anteriores, não medidas novas desta preparação. Não se renomeiam esses casos como teste da política seletiva agora aprovada.

## 4. Hipótese e distinções para o protocolo futuro

H: sob a política delimitada, decisões correntes usam somente suporte válido no contexto e versão explicitamente consultados; manutenção não apaga decisões históricas; mudanças não atingem suportes não dependentes; uma via alternativa evita abstenção apenas quando sua separação, validade e suficiência forem demonstráveis segundo contrato congelado.

Separar quatro eixos: identidade do contexto; sua configuração/versão declarada; versão dos fatos/regras; corte de conhecimento e época de validade. Número de episódio, corte global, versão de arquivo/checkpoint e novo ID arbitrário não substituem uma versão de contexto. Histórico é a decisão e suas referências naquele corte/época; não é permissão para reutilizar suporte antigo no presente.

Escopo proposto permanece relacional L, dois passos, sem composição negativa ou recursão. Hipóteses não são suporte confirmatório. Fontes são declaradas externamente, não autenticadas; propriedades do contexto não são descobertas autonomamente. Decisão de não responder não implica mutação, retratação ou prova da negação.

## 5. Matriz prospectiva de cenários e rastreabilidade

Notação: duas vias positivas X→Y→Z e X→W→Z; contexto c e contexto d não afetado; q=+L(X,Z). S=Supported; demais resultados são abstenções qualificadas ou erro/inconclusão. Cada caso futuro deverá começar de estado vazio independente. Esta tabela não fixa IDs, quantidades de sondas ou ordem final enquanto §6 estiver aberto.

| Caso → regra de origem | Estado/eventos e consulta a especificar no painel | Previsão P1 / P2 e evidência exigida | Controle / critério |
|---|---|---|---|
| R1 — contextos diferentes → igualdade contextual R1 §3 | Duas premissas distribuídas entre c e d; q em cada contexto; depois completar somente c e fazer manutenção explícita. | Não compor fragmentos c/d; S em c somente após suporte suficiente, em P1 após manutenção. | P2 com o mesmo diário; nenhuma prova cruzada, d inalterado. |
| R2 — versões sucessivas → modelagem EC-05 §4 | Configurações declaradas c-v1/c-v2 com premissas que só formariam caminho se misturadas; consultas corrente e histórica. | Abster-se da prova mista; histórico anterior reproduzido com suas referências. **Depende de D2.** | P1/P2 recebem o mesmo vínculo de versões; não equiparar versão de fato a versão de contexto. |
| R3 — revisão de suporte → R1 §§3–5 | A1/B1 materializados; revisão autorizada A2; sondas antes/depois de Refresh e no corte antigo. | P1 não usa A1 como corrente; P2 pode responder com A2 antes de Refresh se suficiente. Depois, ambas usam A2/B1; histórico usa A1/B1. | Comparar conteúdo e provas, registrando diferença de prontidão sem erro automático. |
| R4 — validade temporal → intervalo semiaberto | A1 expira em t; q em t−1/t; receber A2 válido a partir de t, consultar pré/pós-manutenção. | Expired sem outra via válida; história em t−1 conservada; suporte novo não é retroativo. | P2 independente; distinguir expiração, revisão e obsolescência. |
| R5 — suspensão e retomada → manutenção explícita | Suspender fonte/premissa, consultar, manter, retomar quando autorizado e consultar após manutenção. | Nenhuma resposta com suporte suspenso; retomada não reativa versão terminalmente retratada; originais conservados. | P2 com mesmos comandos; erro de retomada terminal não vira Unknown. |
| R6 — conflito localizado → decisão atual 4 | Duas vias válidas; oposição em X→Y; consultar com alternativa via W e variar dependências compartilhadas. | S apenas se isolamento, suficiência e independência da alternativa forem demonstrados; caso contrário abstenção integral. **Resultado exato depende de D1.** | Mesmas informações e regra de isolamento em P2, com implementação própria; não usar executores históricos como se já implementassem seletividade. |
| R7 — conflito impeditivo/insuficiência → decisão atual 4 | Oposição à própria conclusão; ou nenhuma alternativa suficiente; ou origem/dependência da alternativa não verificável. | Não ignorar oposição direta à conclusão para responder por outra via; abster-se quando suficiência/independência não for demonstrável. Diagnósticos detalhados dependem de D1. | Negativos equivalentes; não equiparar ausência de evidência, conflito e falha técnica. |
| R8 — conhecimento não afetado → política §6 | Manter conhecimento sustentado em d; executar revisão, expiração e conflito apenas em c; reconsultar d e seus cortes anteriores. | Decisão/referências de d conservadas se suas dependências realmente não forem afetadas; corte global real é registrado. | Incluir caso de dependência compartilhada que legitimamente afete ambos; não exigir invariância artificial. |
| R9 — história versus corrente → replay R1 §3 | Salvar respostas antes/depois de mudança; consultar esses cortes após outras operações e testar corte futuro. | Reprodução das referências históricas sem fatos posteriores; corte inválido rejeitado; Query não altera estado. | Replay P2 próprio; registrar história indisponível como tal, não reconstrução apresentada como registro original. |

As previsões qualitativas decorrem das decisões existentes; não constituem novo algoritmo aprovado. Não se fixa uma lista artificial de acertos enquanto faltarem os contratos D1/D2 e as sondas correspondentes. Uma falha de preparação não é falha cognitiva observada.

## 6. Decisões operacionais indispensáveis antes de congelar

### D1 — Qual independência permite isolar o conflito?

O laboratório representa referências de fatos, fontes e regras, mas não uma relação validada de independência entre evidências. Considere a via Y em conflito e a via W com fatos/fontes distintos, ambas derivadas pela mesma R. Exigir disjunção de **todas** as dependências rejeitaria W pela regra comum; verificar apenas IDs de fatos aceitaria inclusive fatos distintos provenientes do mesmo relato/fonte. Verificar fontes distintas tampouco comprova independência estatística ou causal real.

**Decisão necessária:** fixar se a independência exigida é separação estrutural de dependências no laboratório ou uma propriedade empírica das fontes; quais dependências compartilhadas são admissíveis; e qual evidência torna a separação verificável. Isso muda a resposta prevista em R6 e não é simples organização de código. O texto aprovado exige a demonstração, mas não escolhe esse predicado.

**Recomendação para decisão:** limitar explicitamente a alegação à independência estrutural de suporte; exigir rastreabilidade das observações, fontes e premissas de ambas as vias, rejeitando alternativa que dependa do suporte em conflito ou cuja origem seja desconhecida. Submeter expressamente a possibilidade de compartilhar uma regra válida e não contestada, contexto e extremos da consulta. Não inferir independência por desigualdade de IDs. Oposição à própria conclusão deve impedir a resposta em todas as vias. Se a separação não for demonstrável, manter abstenção integral conforme já aprovado. **Essa operacionalização é proposta para aprovação, não regra implementada ou congelada aqui.**

### D2 — O que identifica uma versão do mesmo contexto e sua conservação?

`Query.context: u32` filtra igualdade; Key versiona fatos/regras, não contextos. O protocolo diagnóstico anterior excluiu expressamente versões gerais de contexto. Um novo ID não informa sozinho que se trata de sucessora do mesmo contexto; um corte novo pode apenas conter informação sobre outro contexto. É necessário evitar que o teste reduza “não misturar versões” a separar dois laboratórios sem ligação declarada.

**Decisão necessária:** determinar a unidade de versão contextual do ensaio e quais suportes, se houver, permanecem aplicáveis à versão sucessora sem nova aquisição. Herança implícita, revalidação explícita e ausência de herança produzem respostas diferentes; nenhuma é uma política geral já aprovada. Não se propõe alteração de API de produção.

**Recomendação para decisão:** usar, somente no manifesto experimental, identidade lógica e sucessão explícita de configurações, com evidências/validade e referências autorizadas por configuração; nenhuma herança presumida por nome, corte ou época. A aplicabilidade conservada deve ser declarada e rastreável, não selecionada por gabarito. Congelar o mapeamento para os DTOs experimentais antes da execução e identificar isso como descrição externa de contexto, não contexto aprendido. **Proposta pendente; não se materializou o manifesto.**

São decisões sobre o contrato científico, não solicitação de autorização para criar arquivos ou versionar. A regra de abster-se quando a independência é desconhecida já está aprovada; usá-la em todos os casos sem definir um caso demonstravelmente independente não avaliaria o ganho/risco do isolamento seletivo solicitado. Por isso a execução do painel completo está interrompida antes de implementar, como determina a delegação.

## 7. Controles, métricas e barreiras que devem compor o congelamento

P1 e P2 devem receber operações idênticas, mesmas referências, regras, contextos/configurações declarados, cortes e épocas. Compartilhar apenas DTOs passivos e dados; não compartilhar validação, escolha de caminho ou decisão. P2 recompõe diretamente dos registros válidos sem Refresh e conserva sua própria implementação. Expor diferenças previstas pré-manutenção; não reduzir fatos disponíveis ao controle.

Ramos diagnósticos futuros: ausência de manutenção, ausência de aquisição e indisponibilidade de memória, identificados como intervenções diferentes; não apresentá-los como equivalentes informacionalmente. Controlar conflitos com alternativas suficientes, insuficientes e de proveniência desconhecida; limite de consulta não deve virar ausência de conhecimento. Não aplicar ablação neural a um mecanismo inteiramente simbólico.

Registrar por cenário, configuração contextual, fase e condição: total de consultas, S e cobertura S/N; respostas corretas/indevidas; falsas abstenções; motivos completos; erros e inconclusões; provas e versões inelegíveis rejeitadas; histórico recuperado/solicitado; decisões e referências conservadas/elegíveis no contexto não afetado; mudanças justificadas/indevidas. Precisão entre S é indefinida quando S=0. Abstenção esperada mede conformidade diagnóstica, não acerto relacional. Divergência P1/P2 deve informar fase, evidência e eventual falta de manutenção, não apenas taxa de concordância.

Separar partições de desenvolvimento, validação prévia e final, com manifestos de operações/consultas e expectativas restritas ao avaliador. Fixar identidades, sequência completa e denominadores antes da implementação; registrar hashes antes do primeiro final. Modelos não recebem categoria, resposta esperada ou flag de sucesso. Avaliar depois da resposta, sem retroalimentar aquisição. Comparar estado antes/depois de cada consulta; duas sequências globais completas a partir do vazio, com registros integrais limitados e comparação determinística, excluindo apenas tempo/RSS. Novos IDs não demonstram novas estruturas. Não consultar final durante ajustes.

Sucesso funcional futuro exige todas as expectativas específicas congeladas, nenhuma prova mista/inelegível, histórico preservado, não mutação em Query, tratamento correto dos limites e repetição integral. Resposta indevida, mistura de versões ou perda de conservação requerida em execução válida falsifica a hipótese local. Falha de instrumento, vazamento, erro inesperado ou orçamento excedido interrompe e torna a conclusão correspondente inconclusiva. Falta de protocolo impede execução, não fornece resultado negativo de retenção.

## 8. Limites e situação operacional desta entrega

Manter como envelope de preparação os tetos locais: dois passos sem recursão; 64 versões/representações e 64 observações, 8 fontes, 4 versões de regra, 128 dependências e 128 operações/tentativas por trajetória; 8192 visitas/testes de pares por chamada; dados vivos/fixtures até 256 KiB por condição, incluindo estados e repetições coexistentes; RSS isolado <256 MiB; painel <=30 s; /home >=3 GiB. O protocolo executável futuro deve comprovar que suas sondas cabem nesses tetos antes de aceitar o painel. Zero eventos neurais se o ensaio permanecer simbólico.

A instrumentação histórica de Work e memória é parcial; reutilizá-la não prova orçamento uniforme ou alocações integrais. Medir tempo monotônico, RSS do processo isolado com escopo declarado, disco nas fronteiras e durante execução e capacidades dos dados coexistentes. Erros sem Work significam custo desconhecido, não zero. Nenhum custo/RSS de um experimento não executado é informado como medição.

Nesta preparação, /home apresentou 4633796608 bytes livres na verificação inicial (>3 GiB); target tinha 57618855 bytes. Verificações documentais e de integridade serão reconfirmadas antes do versionamento. Não houve Cargo, painel, teste novo ou regressão executada. Os 136 testes históricos não constituem resultado desta entrega. Não se altera produção, experimento antigo, checkpoint ou protocolos congelados.

## 9. Conclusão e continuidade

Concluídas a formalização A1–A7 e a adoção documental delimitada P1/P2. Preparada a investigação de retenção com rastreabilidade e limites; **não concluída sua avaliação experimental**, por D1/D2 pendentes. Não se declara encerramento da EC-05, aceite integral, aprendizagem neural, resistência à interferência compartilhada, memória integrada implementada ou autonomia cognitiva.

Próximo passo: decidir D1/D2 e completar/congelar o protocolo desta mesma subetapa antes de testes. Nenhuma subetapa posterior é iniciada. O [registro de decisões](../decisions.md) conserva a cronologia, incluindo a proposta anterior ainda não aprovada na data de sua elaboração. O versionamento documental desta preparação é autorizado e não prova pré-registro independente ou aprovação das recomendações pendentes.
