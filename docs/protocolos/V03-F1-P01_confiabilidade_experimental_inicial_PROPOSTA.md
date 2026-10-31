# V03-F1-P01 — Proposta: rastreabilidade e isolamento em consultas diretas

## 1. Identificação e estado

Base documental `e3957ca36cbe039670741cc4b4060abef8666957`, branch `devs`. **PROPOSTO / NÃO APROVADO; EXECUÇÃO NÃO AUTORIZADA.** Preparação exclusivamente documental; F1 não iniciada experimentalmente. O objetivo é avaliar uma fronteira pública estreita, não toda a F1. Este documento não é protocolo congelado para execução e não autoriza instrumentação, calibração, painel ou acesso externo.

## 2. Fontes e estado de referência

[Plano v0.3 aprovado](../planos/Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_APROVADO.md), [proposta](../planos/Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_PROPOSTA.md), [parecer B](../planos/Neurons_v0_3_revisao_cientifica_pre_aprovacao.md), [decisões](../decisions.md), [EC-11](../EC-11_auditoria_cientifica_final.md), [lacunas L01–L17](../EC-12_avaliacao_lacunas.md), [contratos P01–P15](../EC-12_pre_requisitos_especializacao.md), [questões Q01–Q19](../EC-12_questoes_abertas.md), fundamentos e A1–A8. Inspeção estática: [`IntegratedState`](../../src/integration.rs), [`Monitor`](../../src/metacognition.rs), [`TwoHopTask`](../../src/cognition.rs), [`EngramStore`](../../src/engram.rs), [`persistence`](../../src/persistence.rs) e [testes EC-11](../../tests/ec11_reliability.rs). Não se altera conclusão histórica: H1 EC-11 não comparou relatórios completos entre repetições; não se reabre EC-11.

## 3. Problema científico

Um resultado reproduzido pode coincidir apenas em sua decisão, apesar de divergência em tentativas, contexto ou evidências. Além disso, o núcleo referencia IDs de registros, mas não autentica sua fonte. Investiga-se se, para **consultas diretas**, a interface pública permite verificar decisão, contexto, registros de suporte e relatório integral, incluindo após restauração do estado coberto. É evidência de confiabilidade de instrumentação e contrato local, não de validade semântica universal da fonte.

## 4. Recortes candidatos e escolha

1. **Consultas diretas, contexto e rastreabilidade pública — escolhido:** baixo custo, estados finitos, referência simbólica pequena; avalia H1 integral e ausência de proveniência sem depender de aprendizado.
2. Persistência de `Circuit`/pesos: cientificamente relevante, porém contrato de serialização não existe; **não avaliável** sem mecanismo novo.
3. Permissões integradas de ações externas: contrato global ausente; **não avaliável** sem política nova. Testar apenas fila não resolveria essa questão.

O primeiro recorte reduz dependências e tem controles negativos discriminantes. Uma resposta correta isolada não basta; o avaliador deve detectar uma trilha pública adulterada e declarar campos ausentes, sem modificar o candidato.

## 5. Pergunta, hipóteses e unidade

**Pergunta:** em entradas diretas controladas, `IntegratedState::review` e a trilha pública acessível permanecem reproduzíveis e consistentes com uma referência independente, sem misturar contextos ou apresentar referência de evidência ausente como presente?

**H1:** com o mesmo estado, consulta e configuração do monitor, relatórios públicos completos e bytes de checkpoint coincidem entre repetições independentes de execução e processos, e decisões/IDs correspondem à referência para todos os casos avaliáveis. **H0/alternativa:** pelo menos uma comparação ou decisão diverge; uma falha de instrumento, referência inacessível ou recurso insuficiente torna a hipótese inconclusiva, não automaticamente falsa. A previsibilidade estrutural é limitação; três repetições iguais verificam determinismo local, não confiabilidade populacional.

Unidade primária: caso de consulta e estado inicial; pares original/restaurado e três repetições são medidas dependentes do mesmo caso. Contar separadamente consultas, comparações pareadas e alterações artificiais de trilha; não multiplicá-las como amostras independentes.

## 6. Sistema, interfaces e limites

**Implementado:** `EngramStore::record/get/iter`, `IntegratedState::review/checkpoint/restore`, `Monitor::review` e `ReviewReport`, `TwoHopTask` e `Engine`. **Demonstrado delimitadamente:** EC-11 comparou bytes e decisões, além de um relatório antes/depois no mesmo processo; não comparou integralmente relatórios entre repetições/processos. **Não demonstrado:** proveniência autenticada, história/versões, aprendizado e autonomia. **Não avaliável aqui:** permissões integradas e persistência de `Circuit`, pesos, estado neuronal ou configuração de `Monitor`.

Entradas: registros `Engram` de IDs únicos, `Cue` com contexto, origem e padrão de destino; valor ±1, `RecordStatus`, origem, tempo e `EvidenceId` opcional; consulta `Hypothesis(context,from,to)` e lista vazia de contextos alternativos, exceto caso inválido explicitado. Estado: motor recém-criado sem eventos e memória com capacidade e ordem fixas; orçamento de fatos/fila/eventos e monitor definidos antes do final. Saídas: `Result<ReviewReport,ReviewError>`; `ReviewReport.original`, `decision` e todas as tentativas com `hypothesis`, `inference` integral, `evaluation_error`, `assessment`, `evidence` e `conflict`. Registrar também registros efetivamente acessados por ID via `get`, seus campos públicos, bytes/hash do checkpoint e versão/configuração fornecida externamente. Não inventar versão temporal: ID de contexto é identificador, não sucessão.

**Observação crítica:** `Observed` pode ter `EvidenceId=None` e o `Monitor` pode responder por registro direto. O verificador externo deverá marcar **referência não informada**; não transformar isso em erro de decisão, falsidade, ausência de suporte ou abstenção do núcleo. `Validated` exige ID, mas sua presença não prova autenticidade ou validade externa.

## 7. Variáveis e condições

Controladas: número/ordem de registros, configuração e limites, tempo lógico, versão do binário, processo, consulta, configuração de monitor e ausência de contextos alternativos, salvo negativo de API. Observadas: resultado completo, estado público antes/depois, IDs/contextos, presença de referência, checkpoint, tempos e recursos. Fator deliberado: distribuição das mesmas relações por contextos e disponibilidade de `EvidenceId`; tratamento de trilha adulterada é teste do **verificador**, não falha injetada no núcleo.

## 8. Controles e referências

Indispensáveis: positivo com fato direto no contexto consultado; negativo sem fato; controle de contexto com fato positivo em A e negação apenas em B; conflito no mesmo contexto com registro positivo e negativo; proveniência com `Observed`/referência ausente versus `Validated`/referência presente; consulta inválida; memória vazia; restauração em processo novo e comparação com execução não restaurada; cópia de trilha com ID/contexto/referência removido ou trocado para verificar sensibilidade do avaliador. A adulteração ocorre **após coleta**, é rotulada e jamais altera o estado ou entra como exemplo do candidato. Opcional: comparação de `EngramStore::recall` para consulta direta, sem interpretá-lo como controle de conflito do Monitor.

Referência indispensável: verificador simbólico separado do candidato, sobre cópia somente leitura do manifesto público, implementado sem chamar `Monitor`, `TwoHopTask`, `IntegratedState::review` ou a própria função de validação do candidato. Para as classes especificadas usa apenas igualdade de contexto/origem/destino, sinal, estado e unicidade; detecta positivo, negativo conflitante e ausência. A tabela de expectativas é gerada prospectivamente pelo verificador e conferida manualmente antes do final. Independência de implementação e autoria devem ser inspecionadas; mesma especificação ou equipe são pressupostos compartilhados. Se não houver verificador distinto e conferência documentada, resultado de concordância é **NÃO AVALIÁVEL**, sem substituição por concordância consigo mesmo. Referência identifica consistência formal, não autenticidade de fonte externa.

## 9. Desenho prospectivo e partições

Três partições com bases de IDs/contextos distintas D=1000, V=2000, F=3000. Em cada partição, seis casos abaixo; `b` é a base da partição, nós `x=b+101`, `y=b+102`, contextos `A=b+1`, `B=b+2`; IDs de registro e evidência usam faixas não sobrepostas `b+10n` e `b+500+10n`, com `n=1..6`. Novas instâncias por caso impedem compartilhamento entre casos. Ordem de inserção é positiva primeiro, negativa depois. Nenhum campo de expectativa é fornecido ao candidato.

| Caso | Estado entregue e consulta | Referência formal prévia |
|---|---|---|
| S1 | `+1 Validated` em A com evidência; consulta A x→y | Resposta direta, ID positivo e contexto A. |
| S2 | `+1 Validated` em A e `-1 Observed` em B, mesma relação; consulta A | Resposta direta em A; B não altera decisão ou suporte. |
| S3 | Mesmo estado de S2; consulta B | Abstenção `MissingEvidence`: negação isolada não é suporte positivo nem conflito detectado sem prova positiva. |
| S4 | `+1 Validated` e `-1 Observed` em A; consulta A | Abstenção `ContradictoryEvidence`, com referência negativa identificável no `AttemptLog`. |
| S5 | `+1 Observed` em A sem `EvidenceId`; consulta A | Resposta direta prevista; verificador classifica referência de proveniência **ausente**, não a preenche. |
| S6 | Memória vazia; consulta A | Abstenção `MissingEvidence` sem ID inventado. |

Para IDs de cada caso: positivo `b+10n`; negativo, quando houver, `b+10n+1`; evidência do validado `b+500+10n`; `origin=Some(x)` e `observed_at=7`, sem evidência para negativos. Estado de S3 é construído independentemente mas contém os mesmos fatos estruturais de S2; equivalência é deliberada. D/V/F diferem por identificadores, **não por topologia**, e não constituem novidade estrutural ou amostras estatísticas independentes. A seleção é exaustiva das seis classes construídas, sem aleatoriedade/semente. Proibido usar casos finais para corrigir instrumento; D e V permitem revisão prospectiva antes de congelamento e novo hash.

Por caso: construir estado novo e monitor de uma tentativa; registrar manifesto de entrada e estado; chamar `review` duas vezes no mesmo processo sem mutação; codificar checkpoint e restaurar em processo separado com configuração idêntica fornecida pelo chamador; chamar `review` uma vez no filho; comparar todos os campos públicos normalizados do relatório e bytes de checkpoint antes/depois. Três repetições completas em processos novos: **18 casos/partição, 54 consultas por partição**, com três consultas por caso e duas comparações de relatórios por caso (intra-processo e pai/filho): **36 comparações em F**. Verificar explicitamente que o processo filho executou exatamente um caso e devolveu registro identificado, não apenas código de saída 0. O motor vazio e a memória são persistidos; circuito temporário e monitor não são. Comparar estados por campos públicos e bytes, sem reivindicar igualdade de estados internos não expostos.

Negativo de API adicional por repetição: chamar `review` com o mesmo contexto como alternativo, exigir `InvalidCandidateContext` e checkpoint inalterado; três chamadas F não são `ReviewReport` e não entram nas 54 consultas válidas. Para cada uma das três repetições em F, copiar uma trilha válida S1 e aplicar três alterações independentes: trocar contexto, trocar ID positivo e remover referência de evidência. **Nove trilhas adulteradas**, todas devem ser rejeitadas ou classificadas como incompletas conforme alteração; nenhuma entra no cálculo de decisões do núcleo. S5 é controle positivo de detecção de referência ausente, não adulteração. Definir esquema de serialização canônico e exaustivo para todos os campos públicos antes de executar D; revisão de `Debug` textual não é garantia de formato estável. A implementação futura deve verificar que nenhum campo ou variante foi omitido e interromper se aparecer variante não mapeada.

D para desenvolvimento; V independente de IDs para verificar o instrumento e a referência, sem ajustar F após observação. Antes do primeiro F, congelar código, manifesto algorítmico, esquema canônico, referência, expectativas, configuração, compilador e hashes; registrar momento e responsáveis. F não será impresso integralmente ou usado como aquisição; a referência terá acesso somente ao manifesto de seu caso e não compartilhará gabarito com `review`. Não se requer calibração de limiares funcionais: os critérios são invariantes exatos. Se instrumentos de tempo/RSS exigirem calibração, usar somente D, registrar procedimento e versão antes de V/F, sem relaxamento após F.

## 10. Métricas e critérios prospectivos

| Métrica / unidade | Medição e referência | Aceitação / falha / limite |
|---|---|---|
| Decisão formal | 18 casos F × 3 consultas = 54 resultados; classes e IDs do verificador. | 54/54 conforme; qualquer erro observado é falha local, sem generalização estatística. `ReviewError` de API não entra. |
| Reprodutibilidade pública | 18 pares intra-processo e 18 pai/filho em F, comparação integral canônica. | 36/36 iguais; qualquer divergência é falha de reprodução; referência dos campos é a API pública, não estado neuronal interno. |
| Estado preservado | Checkpoint antes/depois de cada consulta válida e inválida, comparação byte a byte. | 57/57 chamadas F sem mutação de bytes; falha se alterar. Não prova ausência de mutação fora do checkpoint. |
| Contexto e proveniência | 3 repetições S2/S3/S4/S5; verificar IDs e campos dos registros referenciados e estado de referência presente/ausente. | Todos os casos avaliáveis coerentes; ausência em S5 deve ser registrada, não considerada falsa referência. `EvidenceId` não autentica fonte. |
| Sensibilidade do verificador | 9 alterações rotuladas: 6 trocas e 3 remoções; comparar trilha com manifesto. | 6/6 rejeitadas e 3/3 incompletas; falso positivo = trilha íntegra marcada inválida, falso negativo = alteração aceita. Falha do verificador invalida a avaliação, não prova falha do núcleo. |
| Limites e custo | Tempo monotônico por consulta/partição; RSS de processo e descendentes conforme instrumento disponível; bytes e contagem de consultas. | Propor <30 s por painel, RSS <256 MiB, dados ≤256 KiB por condição, /home ≥3 GiB, sujeitos a autorização e instrumentação. Ausência de medida = **não medido**, não zero; sem medição indispensável, conformidade de recurso inconclusiva. |

Falha de decisão/comparação com referência válida é **resultado negativo local**; falha de verificador, ausência de referência, processo filho não confirmado, vazamento ou medição indispensável ausente é **inconclusão metodológica**. Sucesso técnico = invariantes acima; conformidade metodológica = controles e partições íntegros e registros completos; homologação científica = somente conclusão delimitada sobre fronteira pública avaliada, após avaliação autorizada, não validação integral de F1.

## 11. Critérios de interrupção e exclusão

Interromper diante de processo filho não verificado, referência não independente no sentido implementacional, erros de serialização, mudança pós-congelamento, resultado final utilizado para ajuste, recurso excedido, falta de 3 GiB livres ou ausência de instrumento indispensável. Não excluir caso por resposta desfavorável; exclusão apenas por evento técnico predefinido documentado e classificado como inconclusivo, sem substituição silenciosa. Preservar estado e resultados; sem ajustes automáticos.

## 12. Riscos e recursos

Os casos têm mesma estrutura com IDs distintos: repetição não prova generalização. Referência e candidato podem compartilhar especificação ou autor; inspeção e testes de sensibilidade reduzem, mas não eliminam, erro comum. Hash é integridade/rastreio, não pré-registro externo nem autenticação. Checkpoint usa checksum não criptográfico, sem segurança geral. Processos, arquivos temporários e métricas são instrumentos futuros condicionados à autorização; preferir bibliotecas já disponíveis, saída em diretório temporário e limpeza de artefatos próprios, sem acesso externo ou operação destrutiva. Limites propostos no item 10 não são aprovados automaticamente nem benchmarks. Não executar calibração ou painel nesta preparação.

## 13. Condicionantes C01–C05

**C01:** depende de `IntegratedState`/memória/monitor, não conclusão de F2–F7; dependência de verificador independente é específica da alegação de concordância. **C02: NÃO APLICÁVEL** à hipótese: consultas diretas e integridade pública não alegam contribuição causal neuronal/sináptica; se alegada futuramente, exigir controles equivalentes e ablações. **C03:** política integrada de permissões e estado neuronal persistido são **NÃO AVALIÁVEIS** e não são pré-requisitos da consulta local; o protocolo não testa ação externa. **C04: NÃO APLICÁVEL** às métricas cognitivas: restauração de registros é comparador técnico, não retenção funcional aprendida, transferência, cumulatividade ou especialização. **C05:** hipóteses, métricas e critérios acima são propostos; implementação de instrumento e congelamento de D/V/F devem preceder qualquer final, com autorização específica.

## 14. Rastreabilidade ao v0.3

F1/V03-01, C01–C05; lacunas L11 (H1 integral), L13 (proveniência/contexto), L15 (medição) e L17 (referência); P10/P11/P14 somente enquanto contratos condicionais, não demonstração de agente; Q12/Q14/Q16; A1/A2 como diretrizes de memória e proveniência, A7 rigor, sem homologar A8. EC-04–11, EC-10 R3 não executado e limitações EC-11 preservados.

## 15. Prontidão para decisão de execução

| Verificação | Estado nesta preparação |
|---|---|
| Recorte, hipótese, estados, cenários, comparações e métricas | Definidos prospectivamente. |
| Referência, controles e acesso equivalente | Projetados; **implementação separada e independência ainda não verificadas**. |
| Critérios e partições | Propostos; sem necessidade de calibração de exatidão, mas esquema canônico e instrumentos dependem de validação em D/V e congelamento posterior. |
| Recursos, risco e permissões | Limites propostos; instrumentos e limites finais ainda dependem de aprovação e verificação. |
| Dependências ausentes | Verificador independente e serializador completo são pendentes; permissões globais/estado neuronal não são necessários à alegação e seguem não avaliáveis. |
| Condicionantes | Analisados; atendimento operacional C05 e independência requerem trabalho futuro autorizado. |

**Não declarar PRONTO PARA DECISÃO DE EXECUÇÃO do painel final ainda:** falta evidência de que o verificador separado e o esquema integral são implementáveis e discriminantes, bem como aprovação de seus contratos e recursos. Este é um impedimento de prontidão técnica, não uma suposta decisão científica sobre desempenho. Uma decisão humana pode apreciar a proposta e, se desejar, autorizar em escopo próprio a preparação dos instrumentos e validação D/V; isso não constitui aprovação do painel final. Antes de F, revisar manifestos, controles, independência e congelamento; sem aprovação específica, não executar.

## 16. Decisões humanas pendentes

Aprovar ou revisar recorte e referência; autorizar eventualmente instrumentação isolada e validação prévia, com limites de recursos; decidir se a mesma equipe de implementação é aceitável com revisão independente ou se é necessária autoria externa; aprovar esquema canônico, manifestos e o painel final após evidências de prontidão. Não se solicita decisão sobre política geral de permissões, arquitetura neural ou capacidades de outras frentes.

## 17. Limitações e alegações vedadas

Não há resultados novos, evidência de aprendizagem, transferência, retenção neuronal, autonomia operacional, segurança geral ou competência A8. Igualdade de rótulos e execução em processo separado não demonstram generalização ou independência estrutural. Não se afirma autenticidade de proveniência apenas por `EvidenceId`. O teste negativo de trilha só mede o verificador; não é manipulação do núcleo. Sem novas autorizações, F1 permanece não iniciada experimentalmente.

## Verificações da preparação documental

`cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` passaram. Regressão: **185 testes aprovados, zero falhos e um ignorado**, o painel final R3 da EC-10; inclui duplicações históricas e auxiliar inerte. Essas verificações não executam V03-F1-P01 nem validam seu futuro instrumento. Três avisos históricos EC-04 de métodos não usados. Links locais conferidos; `/home` observado com cerca de 3,9 GiB livres, sem monitoração contínua.
