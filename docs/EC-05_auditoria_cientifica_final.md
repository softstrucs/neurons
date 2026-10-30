# EC-05 — Auditoria científica final e encerramento delimitado

## 1. Parecer e autorização

Base auditada: `51f444c5f35052e2edf0d3927234034a6cd187d4`, branch devs, árvore inicialmente limpa; HEAD, origin/devs e referência remota coincidentes. Delegação: **EC-05 — Auditoria científica final e decisão de encerramento**. Foram examinados documentos/código e reproduzidos somente testes existentes. Nenhum protocolo, dado, expectativa, mecanismo ou teste foi corrigido.

**Classificação final: AJUSTAR. Conformidade funcional no painel reproduzido; conformidade metodológica parcial. EC-05 encerrada documentalmente como investigação experimental delimitada, com pendências preservadas.** A autorização atual permite essa decisão na ausência de bloqueio substantivo. Não foi encontrada falha que invalide os resultados limitados reproduzidos; isso não certifica política cognitiva geral, completude das métricas ou todas as condições possíveis.

O aceite qualitativo do plano — conclusões revisadas mediante evidência, identificação de exceções e ausência de extrapolação indevida — tem evidência somente no laboratório descrito. Mudanças e aplicabilidade são fornecidas externamente; “exceção” aqui é delimitação explícita de suporte/configuração, não descoberta autônoma. Encerramento não converte dependências ainda não demonstradas em competências disponíveis para etapas futuras.

## 2. Fontes e rastreabilidade das quatro subetapas

Referências: plano cognitivo v0.2, Fundamento dos 12 princípios, complemento A1–A7, índice de fundamentos, decisões pertinentes; documentos EC-05 listados abaixo; auditorias EC-03.5 e EC-04 final. Examinaram-se `tests/ec05_context_change.rs`, seus suportes ec05_cases/ec05_evidence e os quatro provenance_*.rs; `tests/ec05_retention.rs` e os quatro suportes retention_*.rs. Foram diferenciadas evidência documental histórica, análise estática atual e reprodução atual.

| Subetapa/decisão | Documento ou implementação verificável | Commit e estado |
|---|---|---|
| Modelar escopo de afirmações | [Modelagem](EC-05_modelagem_escopo_afirmacoes.md): conteúdo/contexto/versão/tempo/corte/proveniência separados. | `af42f9c3de1fb8ec2c0eab992a2bf779d8fde718`, documental concluída. |
| Preparar mudança/conflito | [Cenários e pendência](EC-05_cenarios_mudanca_ambiental_conflito.md): não misturar recência EC-04 com EC-03. | `b5995bfba23510e56244ebbb9439687735d42dce`, interrupção pré-implementação legítima. |
| Diagnóstico contextual local | [Protocolo](protocolos/EC-05_protocolo_diagnostico_contextual.md), [resultados](EC-05_resultados_diagnostico_contextual.md), teste ec05_context_change e dois suportes. | `e90e8ffaf7f2c13efbeb828a3af1f685b5e71112`, contrato EC-03.3-R1 adotado localmente; painel executado. |
| Definir política de revisão/abstenção | [Alternativas P1–P4](EC-05_politica_revisao_abstencao.md): recomendação ainda pendente na redação histórica. | `07bf036e3fd3e650c6034d7b0f55ea2ddca44694`, especificação documental. |
| Adotar P1/P2; diretrizes; preparar retenção | [Preparação](protocolos/EC-05_preparacao_retencao_contextual.md), complemento e índice de fundamentos. | `38d5325b76b4694113a300e7b10424fa42404ef5`, P1 delimitada aprovada, P2 forte; D1/D2 operacionais ainda pendentes nessa redação. |
| Avaliar retenção sob D1/D2 aprovadas | [Protocolo](protocolos/EC-05_protocolo_retencao_contextual.md), [resultados](EC-05_resultados_retencao_contextual.md), ec05_retention e quatro suportes. | `51f444c5f35052e2edf0d3927234034a6cd187d4`, implementação/execução após a decisão de retomada. |

A sequência não contém adoção automática de P3/recência ou P4/confiança. A política inicial de conflito global, o isolamento seletivo aprovado depois e as condições D1/D2 são contratos prospectivos distintos. Textos históricos de aprovação pendente ou atividade futura não contradizem decisões posteriores registradas; não foram reescritos para simular conhecimento anterior. O cabeçalho histórico “proposta” do plano v0.2 também não substitui as autorizações posteriores.

Conferidos os hashes de código/protocolo publicados nos dois relatórios e os seis hashes de manifestos, iguais nesta reprodução. Os manifestos diagnósticos têm16112 bytes; os de retenção24362 bytes. D/V/F são namespaces distintos com estruturas conhecidas; não são conjuntos de estruturas genuinamente inéditas ou dados naturais independentes.

O relatório de retenção explicita a correção de completude da regra própria de c2 no caso2. O histórico desta sessão a situa antes dos testes, mas com parte do código experimental já redigida. Portanto não afirmar congelamento integral antes de qualquer implementação. O protocolo operacional final foi fixado antes dos testes e os mecanismos/avaliador antes do primeiro final, segundo os registros da sessão; a auditoria atual confirma os bytes, não certifica independentemente essa cronologia histórica. Não foi identificado ajuste orientado por resultado final. A ressalva limita alegações de pré-registro, não a conformidade determinística atualmente reproduzida.

## 3. Reprodução atual e unidades de contagem

Preparação atual: fmt/check/all-targets; desenvolvimento, validação e preflight existentes de ambos os testes; painéis isolados; suíte completa; whitespace. Rust/Cargo1.88.0, offline e cache existente. Nenhum teste ou condição novo foi criado. A regressão repete painéis existentes; não somar suas execuções às contagens de um painel isolado.

| Painel/unidade | Registro histórico | Reprodução atual |
|---|---:|---:|
| Diagnóstico: sete cenários, oito trajetórias, cinco condições, duas sequências |516 verificações|516|
| Consultas diagnósticas principais |306|306|
| Consultas das duas ablações |204|204|
| Erros InvalidCut diagnósticos |6|6|
| Consultas históricas principais incluídas nas306 |18|18|
| Comparações de contexto não afetado |12|12|
| Retenção:16 cenários, P1/P2, duas sequências |272 verificações|272|
| Consultas de retenção |252|252|
| Erros rejeitados de retenção |20|20|
| Consultas históricas incluídas nas252 |60|60|
| Eventos neuronais nos dois painéis |0|0|
| Suíte Rust completa |140 aprovados|**140 aprovados,0 falhos,0 ignorados**|

Desenvolvimento/validação diagnósticos:258 verificações cada. Retenção:42 de desenvolvimento e54 de validação. São verificações preparatórias, não consultas finais adicionais nem amostras independentes. A suíte contém os136 testes anteriores à retenção e os quatro novos, sem alterações.

No diagnóstico, por executor/repetição, candidato/simbólico:24 Supported,11 Unknown,2 Conflict,3 Expired,4 Stale,6 Suspended,1 Retracted. Literal:27 Supported,11 Unknown,2 Conflict,2 Expired,2 Stale,6 Suspended,1 Retracted. Ablações:51 Unknown e51 Unavailable. Conferidas as saídas, os totais e assertivas existentes; candidato/simbólico102/102 respostas completas coincidentes; literal96/102, com seis diferenças pré-Refresh previstas. Ausência/indisponibilidade são ablações de informação, não concorrentes equivalentes.

Na retenção, por condição **nas duas repetições**, denominador126 consultas:

| Condição |Supported|Indeterminate|Conflict|Suspended|Retracted|Expired|Stale|Unknown|
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| P1 |80|2|12|6|2|4|2|18|
| P2 |90|2|12|6|2|2|0|12|

Conformidade de decisões à tabela específica de cada condição:126/126. As20 rejeições são resultados esperados, não testes falhos. História60/60, cinco diferenças por repetição,80/80 comparações de provas entre respostas Supported/Supported iguais e registros comparáveis idênticos nas duas sequências. Precisão entre respostas sustentadas no contrato:80/80 e90/90; cobertura63,49% e71,43%. Não tratar as abstenções como acertos relacionais nem afirmar que os dados comprovam verdade externa.

## 4. As dez diferenças P1/P2

Cada linha ocorre uma vez por repetição; total10 entre126 pares de consultas, com116 decisões iguais. Conferência: protocolo §4; fixtures `tests/support/retention_cases.rs:134–179,216–244`; fluxo P1 `retention_candidate.rs:234–269,316–395`; P2 `retention_symbolic.rs:260–364`.

| Caso e sonda | P1 | P2 | Explicação e consequência |
|---|---|---|---|
| 1: nova premissa completa c, antes de Refresh |Unknown|Supported|P1 ainda não materializou o caminho; não há composição entre c e d. P2 compõe os dois fatos agora disponíveis em c.|
| 2: novo fato completa c2, antes do segundo Refresh |Unknown|Supported|c2 tem regra própria e fatos novos; a diferença não depende de herdar c1. Falta materialização em P1.|
| 3: A2 substitui A1, antes de Refresh |Stale|Supported|P1 rejeita prova antiga; P2 usa A2. Não há resposta sustentada com versão obsoleta.|
| 4: A1 expirada e A2 válida, antes de Refresh |Expired|Supported|P1 mantém motivo principal Expired da prova antiga, também desatualizada; P2 usa a nova observação válida na época3.|
| 12: fatos e regra declarados aplicáveis a c2, antes de Refresh |Unknown|Supported|Declarações são suficientes para P2 recompor; P1 só terá prova materializada após manutenção.|

Assim, há seis Unknown, duas Stale e duas Expired adicionais de P1. São abstenções conservadoras previstas, não dez conclusões falsas nem dez falhas novas. P2 emite dez respostas sustentadas adicionais com suporte verificável: vantagem local de disponibilidade, não superioridade neural ou eficiência demonstrada. Depois de manutenção, o suporte previsto reaparece em P1. “Zero falsas abstenções” vale somente frente à tabela da própria política; frente à informação utilizável por P2, essas dez perdas de cobertura permanecem relevantes.

## 5. Matriz de conformidade e evidências

| Requisito | Classificação | Evidência e limite |
|---|---|---|
| Modelagem separa conteúdo/contexto/versão/época/corte |Conforme documentalmente|Modelagem EC-05 §§3–4; DTOs distintos da retenção; não implementação universal em produção.|
| Diagnóstico segue contrato EC-03.3-R1 local |Conforme no painel|ec05_context_change.rs:107–265 e ec05_evidence; sequências A–G/negativos/cortes reproduzidas.|
| P1 manutenção explícita, Query sem Refresh |Conforme no painel|retention_candidate.rs:234–269,273–407; provas só entram no cache por Refresh, consulta revalida.|
| P2 forte com mesmas informações |Conforme no painel|retention_symbolic.rs:87–224,260–394; replay próprio, join sob demanda, Refresh não concede nem retira fatos.|
| Independência metodológica geral |Parcialmente conforme|Arquivos/validação/decisão independentes; DTOs, política, dados, autoria, professor e avaliador compartilhados. Não há avaliação independente de equipe.|
| D1, observação comprometida e isolamento |Conforme nos casos6/7/11/15|Taint de observação nos modelos; alternativa válida sustenta, compartilhamento comprometido/oposição direta/hipótese insuficiente impede. Não exaure todas as topologias.|
| D1, dependência essencial comum |Conforme nos casos8/9/14|Regra/ancestral Contested invalida vias; origem comum íntegra não invalida automaticamente. Comprometimento global é recebido externamente.|
| D1, origem indeterminada |Conforme no caso10|None não vira raiz independente; Conflict com alternativa indeterminada leva a Indeterminate. Não é estimativa calibrada de confiança.|
| D2, sucessão explícita |Conforme nos casos2/12/16|Candidate:93–109,178–209; Symbolic:89–105,175–197. Sucessora consecutiva, referência exata e declaração de continuidade; não sucessão inferida por tempo/nome.|
| Não transferência automática de evidência |Conforme nos casos1/2/12|Aplicabilidade por configuração e grants; fatos sem regra aplicável não bastam. Só duas versões exercitadas; não generalizar a longas cadeias.|
| Histórico e não mutação |Conforme no painel; capacidade geral parcial|ec05_retention.rs:178–217 compara clones e Answer histórico; versões/diário conservados. Replay não é API universal por objeto nem persistência em disco.|
| Conservação do contexto não afetado |Conforme no painel|Diagnóstico compara Answer normalizado apenas por corte; retenção caso13 fiscaliza provas/decisões de d e seus históricos. Sem resistência à interferência sináptica demonstrada.|
| Fiscalização de provas |Conforme às verificações descritas, parcial como garantia geral|ec05_retention.rs:72–156 verifica referências, estado, tempo, continuidade, origens e dependências; não reimplementa toda oposição/seleção negativa.|
| Separação aquisição/avaliação |Conforme nos fluxos inspecionados|Modelos importam apenas DTOs; recebem Op/Query/cut; gabaritos são usados após retorno. Contested/aplicabilidade são informações autorizadas, não descobertas autônomas.|
| Negativos e falsificação |Conforme nos casos congelados|Expected explícitos; Invalid/Terminal/Duplicate/InvalidCut e abstenções ponta a ponta. Sem teste exaustivo de todos os limites/inputs.|
| Repetição global/determinismo |Conforme nos registros comparados|Loops P1→P2→repetir e igualdade Record; diagnóstico compara Trace integral. Não amplia cobertura científica.|
| Métricas/trabalho/dados finos |Parcialmente conforme|Contagem de consultas/decisões/erros íntegra; custos nominais não uniformes e memória transitória parcial. Não certifica orçamento fino nem eficiência comparável.|
| RSS/tempo/disco atuais |Conforme às observações e guardas|Dois painéis isolados sem excesso; escopos de wait4/amostragem declarados abaixo.|
| Pré-registro histórico independente |Não verificável|Hashes/commits atuais e relato da sessão preservam rastreabilidade, não comprovação independente de congelamento histórico antes de toda implementação.|

## 6. Ressalvas, divergências e pendências preservadas

1. **Instrumentação incompleta, prioridade alta antes de alegar custos estritos.** Em `retention_candidate.rs:90–91,234–245` várias operações devolvem1 e Refresh conta pares, sem contar todas as buscas/cópias. P2 `:378–388` soma tamanho do diário, não todas as validações do replay. Histórico de P1 soma custos nominais. Erros não transportam custo: desconhecido, não zero. O máximo nominal60 reproduz, mas não demonstra teto efetivo8192 ou comparação justa de trabalho. É recorrência das ressalvas EC-03/04; nenhum estouro real foi demonstrado.
2. **Pico de dados parcial, prioridade alta para certificação fina.** `ec05_retention.rs:65–69,233–247` usa tamanhos/capacidades e reserva32 KiB; BTreeMap transitório de P2, mapas do fiscal e cópias temporárias não são auditados integralmente. Verificação ocorre após a consulta, não no pico de toda alocação; registros retidos usam tamanho de fatia, não necessariamente capacidade. Somar alguns tamanhos duas vezes não comprova que omissões sejam cobertas. Pico reportado170144 bytes é contabilização parcial, não medição integral do alocador; RSS não substitui o teto256 KiB. Nenhum excesso foi observado.
3. **Fiscal/controles parcialmente independentes, prioridade metodológica.** Ambos os executores conhecem a mesma política e usam o mesmo professor/avaliador. O fiscal Supported não testa por conta própria todo o efeito de oposições; dois erros semelhantes de seleção poderiam concordar. Não foi encontrado erro dessa natureza nos casos reproduzidos; não foram criadas condições adversariais novas para procurar fora do escopo. P2 corrige a fraqueza de lookup reduzido, mas não oferece cegueira de autoria ou pressupostos.
4. **Metadados e manutenção fornecidos externamente, limite científico.** Fonte/ancestral e declaração são identificáveis, não autenticados. Contested para fonte/regra já informa o comprometimento; o sistema testa propagação, não descobre sua verdade. Fato novo, revisão, suspensão, sucessão e continuidade são comandos do professor. A declaração guarda ID/referências, não um dossiê completo de autoria/motivo/validação empírica. A política anterior era especificação conceitual mais ampla; não declarar todos os seus metadados implementados.
5. **Versionamento/histórico delimitados, prioridade quando houver dependência futura.** Regra em retention_candidate.rs:162–176 e retention_symbolic.rs:159–173 admite versão1 e rejeita nova identidade repetida; não implementa revisão geral de regras. Fonte também não tem cadeia própria de versões. Fatos/contextos e estados exercitados têm histórico por corte, não History universal por objeto, revisão arbitrária de schema ou checkpoint integrado. A EC-03 já registrava limites correlatos; não são resolvidos retroativamente.
6. **Generalização e retenção limitadas.** Casos sintéticos de dois passos, IDs opacos e segunda versão contextual; mesmos padrões em D/V/F. Continuidade é declarada. O caso13 conserva um domínio separado, não resiste a interferência em parâmetros compartilhados. Não há comprovação de linguagem natural, aprendizagem de origem, autonomia, consolidação neural ou transferência estrutural.
7. **Registros e contagens têm unidades próprias.** Record novo compara resposta/erro/custo de consulta e posição, não todos os snapshots internos nem todo custo de aquisição; fixtures fixam comandos. Logs completos transitórios em memória não são arquivo permanente de estados. Abstenções conformes não são acertos relacionais; erros esperados não são falhas Cargo. Respostas fixas não foram executadas como condições adicionais na retenção: sua inadequação é inferência analítica; ablações efetivamente executadas pertencem ao diagnóstico.
8. **Cronologia anterior à implementação não certificada.** Correção da regra c2 antes dos testes, mas após início da redação de código, e hashes presentes não equivalem a pré-registro independente. Não há evidência de ajuste após resultado final. Não altera a reprodução funcional nem sustenta alegação de protocolo historicamente intocado desde antes de qualquer implementação.

Essas ressalvas são limitações efetivas de engenharia/metodologia ou fronteiras da evidência; não foram convertidas em resultados favoráveis. Fora das trajetórias exercitadas, possibilidades de falha permanecem riscos não reproduzidos, não defeitos novos comprovados. Não houve necessidade de alterar política, protocolo ou dados para concluir a auditoria. Correções futuras exigem autorização e verificação prospectivas; não foram implementadas.

## 7. Comandos, recursos e preservação

Executados fmt/check/all-targets com `--offline` quando aplicável; para cada teste ec05_context_change/ec05_retention: filtros development, validation e manifest_preflight com `--exact --nocapture --test-threads=1`; depois binários existentes frozen_final/frozen_final_panel; regressão `cargo test --offline -- --test-threads=1`; `git diff --check`. Todos terminaram com código0. Não houve rebuild limpo, instalação, alteração de produção ou novo experimento.

| Execução atual | Tempo externo(s) | Pico wait4(KiB) | RSS amostrado(KiB) |
|---|---:|---:|---:|
| Diagnóstico isolado |0,207252|8844|8104|
| Retenção isolada |0,054699|9052|8320|
| Regressão completa |2,949981|87464|25124 do processo Cargo amostrado|

Tempos internos: diagnóstico0,205356s; retenção0,052844s. Painéis muito abaixo30s e RSS observado abaixo256MiB. Watchdog externo de2ms fiscalizou tempo/RSS/disco; guardas internas também passaram. wait4 tem escopo do processo lançado/fork-exec e filhos aguardados, não apenas objetos Rust; amostra /proc pode perder picos e, no Cargo, não soma compiladores/testes descendentes. O pico terminal maior que a amostra não demonstra crescimento do estado cognitivo. Primeiro script temporário de monitoramento teve erro de sintaxe corrigido antes de iniciar qualquer comando de verificação; não afetou o experimento.

Picos contabilizados internos repetidos: diagnóstico239964 bytes; retenção118008/170144 nas repetições. Custos/limitações declarados em §6; não compará-los como medidas integrais de eficiência. Retenção máximo nominal60; diagnóstico4324. Não houve guarda violada.

Target permaneceu **59619598 bytes**, crescimento0 nesta auditoria. /home inicial4503629824 bytes; mínimo amostrado durante verificações4501164032 (>3GiB). Estado final após documentação/push é confirmado na entrega. Resíduos de logs e instrumentos de auditoria ficaram somente em /tmp; não são novos artefatos científicos do repositório.

Todos os128 arquivos inicialmente rastreados foram comparados por SHA-256 antes da escrita documental e estavam idênticos após a reprodução. Os códigos/protocolos/resultados históricos continuam protegidos; somente acréscimos documentais autorizados em decisões e índice acompanham este parecer. O Fundamento permanece com SHA-256 `3127ba83af2c056e4b11cf403402fbbc384670c5cea6e49871a1b87a952e1850`; complemento A1–A7 com `72df9b02a6c4c335a8cf49d27eb8e4a55470ea2818f589344c339ae991b6d070`. Nenhuma diretriz foi reescrita ou declarada implementada.

## 8. Decisão final e continuidade

**Encerrar a EC-05 como investigação delimitada, AJUSTAR e conformidade metodológica parcial.** As quatro subetapas têm entregas rastreáveis, decisões pendentes foram resolvidas prospectivamente e os dois painéis existentes reproduziram previsões/controles sem falha inesperada. As ressalvas impedem homologação integral e alegações gerais; não invalidam a conservação contextual, revisão e recomposição simbólicas observadas nas condições dadas. Não há bloqueio científico substantivo que obrigue manter esta investigação aberta somente para corrigir retroativamente seus artefatos.

O resultado **não demonstra aprendizagem neural, compreensão geral, formação autônoma de engramas ou consciência artificial**. Também não demonstra credibilidade aprendida, descoberta de contextos, investigação autônoma, resistência à interferência compartilhada, transferência estrutural ou superioridade sobre P2/recuperação forte. Aprovação das diretrizes A1–A7 é conceitual e permanece distinta de competência.

Pendências para investigações futuras: instrumentação uniforme de trabalho/memória e custos de erro; fiscalização adversarial independente de oposição/isolamento; extensão prospectiva de proveniência, versões, continuidade e histórico se exigida pelo novo escopo. Não são etapas iniciadas nem pré-condição administrativa para toda auditoria futura; uma alegação que dependa dessas garantias exige resolvê-las primeiro.

Próxima etapa nominal do plano: **EC-06 — Raciocínio com evidências**, primeira subetapa **auditar inferência existente**. Não foi iniciada nem autorizada automaticamente pelo encerramento. Esta delegação termina com parecer/decisões/indexação documentais e commit/push autorizados; aguardar nova autorização antes de qualquer continuidade.
