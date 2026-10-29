# EC-05 — Protocolo operacional de diagnóstico contextual

## 1. Estatuto e hipótese

Protocolo prospectivo elaborado antes da implementação/execução desta retomada, base `b5995bfba23510e56244ebbb9439687735d42dce`, devs. A delegação “CONTRATO DIAGNÓSTICO LOCAL E RETOMADA EXPERIMENTAL” aprovou expressamente EC-03.3-R1 como contrato local. O protocolo EC-03.3 original, R1, modelagem e preparação EC-05 permanecem históricos e inalterados. Nenhuma política geral é criada; recência EC-04 fica excluída.

H: os executores existentes distinguem contexto, oposição, validade, suspensão e revisão de suporte; recompõem somente onde permitido e reproduzem decisões no corte antigo sem aprender com consultas. Este é diagnóstico de regras programadas, não aprendizagem, descoberta de contexto ou revisão autônoma.

Fontes: plano cognitivo v0.2 integral; Fundamento; modelagem EC-05 e cenários A–G; protocolos EC-03.3/R1; relatórios EC-03.4/03.5, EC-04 implementação/auditoria e decisões. A adoção local resolve a pendência anterior; as não conformidades históricas de metadados, History e instrumentação não são apagadas.

## 2. Regras reutilizadas e fronteiras

Reutilizar sem editar `provenance_candidate.rs`, `provenance_symbolic.rs`, `provenance_literal.rs` e `provenance_types.rs`. DTOs passivos e trait Laboratory existentes. Candidato conserva projeções; simbólico tem reconstrução própria; literal recompõe diretamente do diário. Não compartilham decisão/validação. Família, professor e avaliador comuns impedem independência científica completa.

Aplicar R1 §§2–5: relação L; entidades X/Y/Z/W distintas; regra positiva de dois passos, sem recursão ou composição negativa; identidades/versões/corte; contexto exato; intervalo [start,end); fonte ativa; hipótese não prova; versão substituída histórica; Refresh explícito; oposição ativa não se resolve por recência, ID ou fonte. Sem conflito, prova elegível sustenta Supported. Sem prova, causas seguem Suspended, Retracted, Expired, Stale, Unknown. Erros do Result são falhas/inconclusões qualificadas, não Unknown. Preservar essa diferença de transporte já auditada.

Mudança ambiental aqui é **nova informação recebida de um professor externo declarado**, não observação sensorial ou descoberta autônoma. Em A, Revise altera o conteúdo de uma relação; em C, um relato independente acrescenta oposição sem substituir o positivo. As duas operações não são confundidas. Nenhuma operação tem resposta esperada no payload.

Fora do contrato: decisão de qual fonte diz a verdade, política aprendida de mudança, contexto inferido do texto, versões gerais de contexto, History universal por identidade, revisão da própria regra aprendida, integração com recência EC-04, conflitos entre domínios arbitrários, persistência nova, competência geral, transferência estrutural. Não criar soluções para esses casos.

## 3. Identidades, partições e estados

Três laboratórios separados: desenvolvimento offset=100000, validação=200000, final=300000. Cada um executa os mesmos tipos de sondas com identidades distintas; não alegar novidade estrutural ou avaliação cega de autoria. Final não será consultado para ajustar código.

Oito trajetórias independentes indexadas i=0..7: A,B,C,D-principal,D-alternativa,E,F,G. São sete cenários, com duas trajetórias temporais D. Cada uma começa vazia, época1, sem regras/fontes/conteúdo. b=offset+1000*(i+1). X/Y/Z/W=b+11/23/37/41; c/d/e=b+53/67/79. Fontes A/B/C/D=b+101/102/103/104, versão1; fatos A/B/C/D=b+201/202/203/204; observação da versão v e papel p=1..4: b+300+p+50*(v-1). Regras R-c/R-d=b+401/402, versão1. Versões do fato mantêm seu ID; observação nova tem ID novo. e é contexto sem conteúdo. Namespaces por tipo, sem índice de classe recebido pelos modelos.

Base comum: registrar fontes A,B; fatos A1=+L(X,Y), B1=+L(Y,Z), contexto c, observed=1, validade [1,10); R-c em c [1,10); Refresh. Ordem: fontes, fatos, regra, Refresh. Em D-principal A1 termina em3. Em B/F, registrar fontes A..D, fatos A1/B1 em c e C1/D1 em d; regras R-c/R-d; Refresh. Em B C1=-L(X,Y), D1=+L(Y,Z); em F ambos positivos. Em D-alternativa registrar fontes A..D, A1/B1 via Y e C1=+L(X,W), D1=+L(W,Z) em c; A1 termina em3, demais em10; R-c; Refresh.

Todas as consultas correntes são +L(X,Z), contexto c e época corrente, salvo indicação explícita. Cada linha de resultado abaixo inclui uma sonda, não aquisição. Em cada consulta preservar estado/corte. Nenhum fato direto X→Z é adquirido. Validação/final não atualizam os executores.

## 4. Matriz operacional congelada

Abreviações U=Unknown, S=Supported, C=Conflict, E=Expired, T=Stale, P=Suspended, R=Retracted. Sequências são previsões analíticas; não resultados. Cada trajetória começa com consulta vazia U. Candidato e simbólico seguem a coluna CS; literal forte segue L. As operações entre sondas são executadas exatamente na ordem escrita, sem Refresh implícito.

| Cenário → regra R1 | Estado/eventos e consultas na ordem | CS | L | Controle e critério |
|---|---|---|---|---|
| A → §§3/4 revisão/substituição e dois passos | Vazio; base; Revise A2 para +L(X,W), mesma fonte/época/validade; Refresh; fonte C + fato C1=+L(W,Z) e Refresh. Consultar após cada grupo. | U,S,T,T,S | U,S,T,T,S | Prova final A2/C1; A1 não sustenta corrente; sem inferir falsidade enquanto falta ligação. |
| B → §§3/5 contexto exato/negativa não compõe | Vazio c; base dupla definida em §3; consultar c, depois d, depois e. | U,S,U,U | U,S,U,U | Negativo em d não conflita com positivo em c; d sem caminho positivo é U, não prova da conclusão negativa. |
| C → §§4/5 oposição e retirada | Vazio; base; fonte C e C1=-L(X,Y), consultar antes de Refresh; Refresh e consultar; retratar C1 e consultar antes de Refresh; Refresh e consultar. | U,S,C,C,S,S | U,S,C,C,S,S | Conflict contém A1/C1; nenhum vencedor; retirada afeta corrente, não apaga registro. |
| D-principal → §§3/4/5 tempo e revisão | Vazio; base com A1 [1,3); Epoch2 e consultar; Epoch3 e consultar antes de Refresh; Refresh e consultar; Revise A2 mesmo conteúdo, observed3, [3,10), consultar antes de Refresh; Refresh e consultar. | U,S,S,E,E,E,S | U,S,S,E,E,S,S | Fronteira3 exclui A1; literal usa A2 antes de Refresh; prova corrente pós-Refresh A2/B1. |
| D-alternativa → §§3/5 suporte alternativo | Vazio; base dupla via Y/W; Epoch3 e consultar antes de Refresh; Refresh e consultar. | U,S,S,S | U,S,S,S | Na época1 duas provas; época3 somente C1/D1 via W; causa Expired não invalida a outra via. |
| E → §§4/5 suspensão, retomada, revisão | Vazio; base; suspender fonte A e consultar; Refresh/consultar; retomar A+Refresh/consultar; Revise A2 mesmo conteúdo e consultar pré-Refresh; Refresh/consultar; suspender R-c e consultar; Refresh/consultar; retomar R-c+Refresh/consultar. | U,S,P,P,S,T,S,P,P,S | U,S,P,P,S,S,S,P,P,S | Suspensão imediata; retomada sondada após Refresh; literal mais pronto na revisão; A2/B1 após revisão. |
| F → §§3/4/5 identidade contextual | Vazio c; base dupla positiva; consultar c e d; suspender fonte A; consultar c e d; Refresh; consultar c e d. | U,S,S,P,S,P,S | U,S,S,P,S,P,S | Fontes/regras distintas; respostas/provas de d preservadas, exceto corte global naturalmente maior. Não testa retenção longitudinal. |
| G → §§3/4/5 histórico por corte | Vazio; base e salvar corte/Answer H1; Revise A2 mesmo conteúdo/consulta pré-Refresh; Refresh/consulta e salvar H2; retratar A2+Refresh/consulta corrente; History H1 época1; History H2 época1; History corte0 época1. | U,S,T,S,R,S,S,U | U,S,S,S,R,S,S,U | H1/H2 devem reproduzir Answer registrado, incluindo versões/evidências; corte0 vazio não usa conhecimento posterior. Depois, Query no corte corrente+1 exige InvalidCut, estado inalterado. |

Em G, H1/H2 guardam o corte da consulta indicada, sem incluí-la no diário. Em F, comparação de d conserva decisão, causas, conflitos, provas e evidências; normalizar somente Answer.cut para comparar o conteúdo, mas registrar ambos os cortes reais. Em C Conflict pode conservar provas positivas elegíveis: decisão não permite responder S; essas provas não são usadas para afirmar que o conflito está resolvido.

## 5. Controles, ordem e contagem

Por laboratório, executar condições nesta ordem: candidato, simbólico independente, literal forte, sem aquisição, informação indisponível. Nas três principais, mesmo conjunto de operações, consultas e limites. Validar todas as provas devolvidas mediante evidências do diário prefixado: identidade/versão corrente, status/fonte, contexto, intervalo, polaridade positiva, conectividade dos dois fatos/regra e observação correspondente. Comparar também referências de conflito. O verificador é apenas avaliador; não produz a decisão do modelo nem devolve oráculo.

Sem aquisição: ignorar operações de conteúdo, consultar componente vazio no corte0; registrar consultas solicitadas e resposta U, sem pretender replay de conhecimento que não existe. Informação indisponível: aplicar mesmas operações ao candidato, mas o adaptador diagnóstico impede consulta ao conteúdo e devolve Unavailable (transporte existente), sem prova. São ablações explícitas, não concorrentes informacionalmente equivalentes; não executar nelas o negativo InvalidCut. Nenhum ramo de treinamento auxiliar é necessário.

Sondas pré-Refresh já constituem comparação com ausência de manutenção: operações anteriores idênticas; falta somente o Refresh até a sonda. Não adicionar controle que congele evidências em segredo. O contexto d estável de F é controle de isolamento local, não a subetapa posterior de retenção.

| Contagem por repetição | Principal por executor | Três principais | Duas ablações |
|---|---:|---:|---:|
| Consultas A/B/C/D/E/F/G | 5/4/6/11/10/7/8 =51 |153|102|
| Destas: históricas em G |3|9|6 consultas vazias/indisponíveis, não história recuperada|
| Erro InvalidCut em G |1|3|0|
| Verificações totais |52|156|102|

Final: duas **sequências globais completas**, cada uma com todas as cinco condições e trajetórias na ordem fixada; reset total por trajetória. 306 consultas principais +6 erros InvalidCut +204 consultas de ablação = **516 verificações**, das quais510 são consultas e6 testes de erro. Há18 consultas históricas principais nas306. Desenvolvimento e validação: uma sequência completa cada,258 verificações cada, fora do final. Mesmas estruturas com outros IDs, sem aprendizagem ou generalização alegada.

Previsão por executor/repetição: CS 24 S e27 abstenções (11 U,2 C,3 E,4 T,6 P,1 R); literal27 S e24 abstenções (11 U,2 C,2 E,2 T,6 P,1 R). Três diferenças previstas CS/literal, nas revisões pré-Refresh D/E/G; demais48/51 decisões iguais. Candidato/simbólico51/51 iguais. Cada ablação51 abstenções, zero S; coincidências de U com oráculo não contam como capacidade adquirida.

## 6. Congelamento e critérios

Antes de implementar, conferir esta matriz por derivação do contrato e sua soma. Preparar fixtures de desenvolvimento/validação/final em arquivo separado dos modelos. Modelos recebem somente Op e Query/cut. Expectativas, categoria, referências esperadas e fases ficam no avaliador. Registrar SHA-256 do protocolo antes de executar desenvolvimento; depois congelar também código e manifestos canônicos de operações/consultas/expectativas, antes do primeiro final. Não imprimir gabaritos como se fossem dados adquiridos. SHA do manifesto prova identidade atual, não pré-registro histórico independente.

Executar desenvolvimento, depois validação. Ajustes de código somente nessas preparações e sem alterar política/previsões deste protocolo; se exigirem mudança científica, parar. Nenhum ajuste após consultar final. Repetir final conforme §5; comparar exatamente comandos, respostas completas, erros e Work registrado entre as duas sequências, excluindo tempo e memória de alocação. Entre implementações comparar semântica/evidências, não custos diferentes; não exigir igualdade nas três diferenças previstas do literal. Guardar registros auditáveis, não apenas hashes.

Sucesso funcional local: todas as decisões previstas, zero respostas indevidas/falsas abstenções no contrato respectivo, premissas/evidências fiscalizadas, H1/H2 íntegros, corte inválido rejeitado sem mutação, contexto d conservado e repetição exata, sem limite observado excedido. Abstenções qualificadas são conformidade diagnóstica, não acertos relacionais. Publicar resultados por cenário/condição, causas completas e divergências. Publicar cobertura S/N, precisão entre S quando definível, erros/abstenções por motivo, histórico, conservação e trabalho de aquisição/consulta separadamente.

Falha funcional: usar suporte inelegível/contexto cruzado, perder decisão histórica requerida, aceitar consulta inválida ou alterar estado em Query, perder suporte alternativo válido. Inconclusão técnica: erro inesperado, divergência não explicada, esquema sem dados suficientes, vazamento ou teto excedido. Interromper sem adaptar expectativas; resultado negativo/inconclusivo pode ser conclusão legítima. Métricas finas não instrumentadas não viram conformes por testes passarem.

## 7. Recursos e limitações

Preservar tetos R1:64 versões/observações,8 fontes,4 regras,128 dependências e128 operações/tentativas por trajetória;8192 visitas/pares por chamada;256 KiB de dados vivos/fixtures por condição incluindo repetições;RSS isolado <256 MiB; painel <=30s; /home >=3 GiB. Nenhum Circuit/Engine é chamado: **zero eventos neuronais** previstos. Comandos do diário não são eventos neurais. Sem dependências externas, threads próprias ou mudanças de produção.

Guardas: tempo monotônico, RSS /proc e disco nas fronteiras e monitor externo; pico terminal por wait4 quando disponível. Somar capacidades reais dos registros novos e fixtures, estados/snapshots coexistentes e reserva transitória declarada; comparar cada condição/repetição e memória total conservada. Reutilizar bytes/Work existentes, explicitando que têm ressalvas da EC-03.5 e não provam contagem fina uniforme. Não corrigir automaticamente o mecanismo antigo para satisfazer uma métrica. Se alguma alegação fina não for verificável, registrar como parcial; se houver excesso observado, parar.

Não são corrigidas as lacunas históricas de Source/History/revisão de regra nem de instrumentação EC-03/04. O painel testa trajetórias compatíveis e fontes fornecidas, sem autenticação, percepção, aprendizagem neural, semântica autônoma, retenção geral ou transferência estrutural. Continuidade posterior depende de autorização própria. Concluir esta subetapa não estabelece a política geral de revisão/abstenção da etapa seguinte.
