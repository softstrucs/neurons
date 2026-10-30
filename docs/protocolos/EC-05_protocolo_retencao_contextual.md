# EC-05 — Protocolo prospectivo de retenção contextual

## 1. Estatuto e hipótese congelada

Base 38d5325b76b4694113a300e7b10424fa42404ef5, devs limpa e sincronizada. A delegação “Retomar avaliação de retenção contextual” aprova D1/D2 e autoriza sua operacionalização e testes isolados. Este documento completa a [preparação](EC-05_preparacao_retencao_contextual.md), que permanece histórica. Não modifica EC-03.3/R1, EC-04 ou o diagnóstico EC-05. Subetapa do plano v0.2: **avaliar retenção sem misturar versões de contexto**. Não iniciar etapas seguintes.

H: P1 conserva evidências e decisões históricas, aplica apenas suporte elegível na configuração consultada e isola conflito somente quando outra prova é suficiente e independente das dependências comprometidas. P2 recompõe sob demanda, sem dependência da manutenção de P1. É teste de regras programadas sobre informações recebidas, não aprendizagem neural, descoberta de contextos ou credibilidade aprendida.

A conferência documental anterior à primeira execução explicitou a regra própria de c2 no caso2 (sem herança); nenhum resultado foi consultado para essa correção de completude. Congelar os bytes consistentes deste protocolo por SHA-256 antes dos testes; congelar mecanismos/avaliador/fixtures antes do primeiro final. Registrar resultados sem modificar expectativas após observar final. Hash atual ou commit posterior não prova pré-registro independente.

## 2. Objetos, identidade e D2

Identidade contextual é a dupla `(id lógico u32, versão u16)`, ambos positivos. Declarar versão1 sem predecessora; versão n>1 exige predecessora existente `(mesmo id,n−1)`. Rejeitar duplicação, salto, predecessor de outro contexto e versão sem predecessor. Sucessão é um evento explícito; não nasce de época, nome ou recência. Configurações e fatos anteriores permanecem armazenados. Consultas nomeiam a configuração exata; consulta histórica usa também o corte/época originais. A versão corrente é a última sucessora declarada na sequência, não autorização para migrar suporte antigo.

Fato: Key(id,versão), configuração de origem, extremos, polaridade, fonte, observação, validade [start,end), hipótese. Revisão exige predecessora consecutiva, mesma configuração de origem e nova observação; a versão anterior fica histórica. Regra: Key, configuração de origem e intervalo. Relação L positiva compõe somente dois fatos evidenciais a→b→c, entidades distintas; negativos não compõem; provas derivadas não são premissas de outra prova. Fonte: ID e origem imediata; raiz aponta para si, origem desconhecida é None. Origens só apontam para fonte já registrada, portanto sem ciclo. Identidades de observação podem ser compartilhadas para representar o mesmo suporte recebido; IDs diferentes não comprovam independência real.

Continuidade é uma operação **DeclareApplicable(kind, key, from, to, declaration)**: somente fato ou regra existente, configuração from já aplicável, to sucessora imediata declarada de from, declaração externa positiva identificável. A declaração afirma aplicabilidade nesse laboratório; não é inferida da resposta correta. Conservar referência à declaração e sua ordem. Não renovar validade, estado, observação ou versão do objeto; não autorizar outra configuração por transitividade implícita. Para terceira versão seria necessária nova declaração a partir da segunda. Fonte é metadado global de origem, não evidência contextual herdada; fato/regra precisam de aplicabilidade própria. Nenhuma declaração é produzida durante avaliação.

Sem declaração, suporte antigo não se aplica à sucessora. Suporte de ambas as versões pode coexistir no diário, mas uma prova usa somente fatos/regra explicitamente aplicáveis à configuração consultada. Declaração ausente/indeterminada impede a prova (Unknown), sem inventar negação. A associação de versões e as declarações são fornecidas externamente, não contexto aprendido.

## 3. Dependências essenciais, D1 e decisões

Toda prova depende essencialmente de duas versões exatas de fatos, suas observações, cadeias de origem das fontes e uma versão da regra; aplicabilidade e validade de cada elemento também são necessárias. As duas premissas e sua composição são registradas; não há inferências intermediárias ocultas ou recursão neste ensaio. Compartilhar contexto, extremos ou regra válida não invalida a independência operacional por si só. A alegação é separação **relativa às dependências comprometidas**, não independência estatística/causal de fontes reais.

Estados declarados de fonte/fato/regra: Active, Suspended, Retracted, Contested. Retracted é terminal para a versão/ID; Set em outro estado exige alvo existente e não retirado. Contested é diagnóstico externo explícito de dependência comprometida, não peso de credibilidade. Uma origem ancestral Contested compromete as vias que a usam; uma regra comum Contested compromete ambas. Origens desconhecidas impedem validar o suporte e geram Indeterminate; não são tratadas como raízes independentes.

Oposição admissível: negativo não hipotético, versão corrente no corte, configuração aplicável, tempo válido e origem/estado verificáveis. Negativo oposto à conclusão bloqueia todas as vias. Para oposição a premissa, marcar as observações de todas as premissas positivas admissíveis com os mesmos extremos como comprometidas, conservando IDs positivos/negativos. Vias que usam essas observações deixam de sustentar; isso alcança outro fato que compartilhe a observação. Não suspender implicitamente toda a fonte: divergência num relato não demonstra que todos os seus relatos sejam falsos. Compartilhar uma origem ativa e identificável é permitido quando o conflito é apenas de premissa e não compromete a origem; explicitar essa diferença no cenário 14.

Uma alternativa sustenta somente se ambas as premissas e a regra forem correntes, ativas, temporariamente válidas, aplicáveis à mesma configuração, não hipotéticas, completamente rastreáveis e sem dependência comprometida. Fonte comum Contested, regra comum Contested ou observação comprometida excluem a alternativa. Se houver conflito e uma via potencial de origem indeterminada, abster-se integralmente como Indeterminate: não demonstrou separação. Se não há conflito e há prova completamente válida, uma via indeterminada adicional não elimina o suporte válido.

P1 registra provas somente em Refresh explícito (todas as combinações tipadas/configurações explicitamente aplicáveis, inclusive provas depois invalidáveis); Query revalida imediatamente todas as dependências, sem Refresh implícito. P2 enumera diretamente fatos/regra do diário permitido em cada Query, independentemente do Refresh. Ambas usam os mesmos fatos e declarações. Uma via de topologia válida mas não materializada não produz prova em P1; pode produzir em P2. Isso não autoriza P1 a usar uma prova antiga inválida.

Decisão: validar entrada/corte; oposição direta → Conflict; conflito com origem indeterminada → Indeterminate; caso contrário, prova elegível → Supported com todas as provas válidas. Sem prova, motivo principal: Indeterminate, Conflict, Suspended, Retracted, Expired, Stale, Unknown. Guardar todas as causas dos caminhos examinados, inclusive Conflict de via isolada quando outra sustenta. Hipóteses não geram caminhos. Sem topologia materializada, P1 retorna Unknown; não inventa Stale ou Expired. Erros são Result::Err, separados de Unknown e nunca conclusão negativa. Ordens de motivos são apresentação diagnóstica, não escolha da fonte verdadeira.

## 4. Sequência e painel congelado

Cada cenário começa independente do vazio. Notação c=(53,1), d=(67,1), c2=(53,2); X/Y/Z/W=11/23/37/41. Fontes 1..4 são raízes ativas, salvo alteração explícita. Regra r=(401,1) em c; rd=(402,1) em d, ambas [1,10). A=(201,1):+XY, fonte1, observação101; B=(202,1):+YZ, fonte2, observação102. Alternativa C=(203,1):+XW, fonte3, obs103; D=(204,1):+WZ, fonte4, obs104. E/F em d têm IDs205/206, fontes3/4, obs105/106, mesmos extremos de A/B. Intervalo [1,10), hipótese=false, origem c salvo indicação. Revisão A2=(201,2), obs111, conteúdo igual, c; na validade começa em3. Novo conteúdo em c2 usa IDs211/212, obs121/122. Negativo N=(207,1), fonte5 raiz declarada antes, obs107, c, [1,10). Fonte5 não é fonte de uma premissa positiva. Época da consulta é1 salvo cenário4. Aquisição base: contextos c/d; fontes em ordem; regras r/rd; A/B; Refresh. Alternativa inclui C/D antes de Refresh. Dados da tabela são do avaliador; modelos recebem apenas operações e consulta.

Em cada linha a primeira consulta assinalada h salva seu corte/resposta para recuperação histórica posterior. Cada History é uma consulta adicional no mesmo corte/época da marca h. Resultados abaixo são previsões, não medições. S Supported; U Unknown; C Conflict; I Indeterminate; P Suspended; R Retracted; E Expired; T Stale.

| Caso / rastreabilidade | Operações e consultas exatamente nesta ordem | P1 | P2 |
|---|---|---|---|
| 1 Contextos distintos / D2 | Base com somente A em c e B em d; q(c); q(d); novo fato +YZ em c ID212 obs122, q(c); Refresh, q(c). | U,U,U,S | U,U,S,S |
| 2 Sucessão sem herança / D2 | Base, q(c)h; declarar c2 e regra (403,1) exclusiva de c2 [1,10), q(c2); fato211 +XY c2, Refresh, q(c2); fato212 +YZ c2, q(c2); Refresh, q(c2); History h. | S,U,U,U,S,S | S,U,U,S,S,S |
| 3 Revisão / manutenção | Base q(c)h; revisar A2, q; Refresh,q; History h. | S,T,S,S | S,S,S,S |
| 4 Validade / [start,end) | Base com A até3; q época2 h; q época3; revisar A2 [3,10), q época3; Refresh,q época3; History h. | S,E,E,S,S | S,E,S,S,S |
| 5 Suspensão/retratação | Base q h; suspender fonte1,q; Refresh,q; retomar fonte1+Refresh,q; retratar A,q; tentar retomar A (Terminal); History h. | S,P,P,S,R,S | igual |
| 6 Via separável / D1 | Base alternativa q h; adquirir N=-XY,q; History h. | S,S,S | igual |
| 7 Observação comum / D1 | Alternativa com C.obs=101; q h; N=-XY,q; History h. | S,C,S | igual |
| 8 Regra comum comprometida / D1 | Base alternativa q h; r Contested,q; History h. | S,C,S | igual |
| 9 Origem comum comprometida / D1 | Fonte3 tem origem1; base alternativa q h; fonte1 Contested,q; History h. | S,C,S | igual |
| 10 Origem indeterminada / D1 | Fonte3 sem origem; base alternativa q h; N=-XY,q; History h. | S,I,S | igual |
| 11 Oposição à conclusão | Base alternativa q h; N=-XZ,q; History h. | S,C,S | igual |
| 12 Continuidade declarada / D2 | Base q h; declarar c2,q(c2); DeclareApplicable A e B c→c2 declarações501/502,q(c2); declarar r aplicável c2, declaração503,q(c2); Refresh,q(c2); History h. | S,U,U,U,S,S | S,U,U,S,S,S |
| 13 Contexto não afetado | Base com E/F em d; q(c)h1; q(d)h2; suspender fonte1; q(c); q(d); Refresh,q(d); History h1; History h2. | S,S,P,S,S,S,S | igual |
| 14 Origem comum não comprometida | Fonte3 origem1; base alternativa q; N=-XY,q h; fonte1 Contested,q; History h. | S,S,C,S | igual |
| 15 Alternativa insuficiente | C hipótese=true; base alternativa q h; N=-XY,q; History h. | S,C,S | igual |
| 16 Erros atômicos | Pré-estado só c/d e fontes/regras; versão c3 com predecessora c1; declaração de r de c para d; Source1 duplicada; Query no corte futuro. | Invalid,Invalid,Duplicate,InvalidCut; estado intacto | igual |

A consulta q é +L(X,Z), configuração c, época1 e corte corrente salvo indicação. Configuração d no caso1 tem só B e não pode compor. A declaração r aplicável à c2 não concede evidência de fatos. Origens e declarações são metadados fornecidos, não rótulos de resposta. Um novo fato/contexto não usa IDs de categoria como entrada de decisão.

Por executor e repetição: 63 consultas, das quais15 históricas; cinco erros verificados (um caso5 e quatro caso16): **68 verificações**. Ordem global: P1 cenários1..16, P2 cenários1..16; repetir a sequência global inteira do vazio. Final: **252 consultas +20 erros =272 verificações**, 60 consultas históricas incluídas nas252; cinco diferenças de decisão P1/P2 por repetição (1/2/3/4/12), dez no total de126 comparações. Zero eventos neuronais. Não confundir teste Cargo, consulta, verificação de erro e amostra independente.

## 5. Desenvolvimento, validação, independência e fiscalização

Desenvolvimento: casos1/3/6/12/16, offset10000, uma sequência:17 consultas+4 erros por executor,42 verificações. Validação:2/4/7/9/13/15, offset20000, uma sequência:27 consultas por executor,54 verificações. Final: todos os casos, offset30000, duas sequências globais. Somar offset a todos os IDs de contexto, entidade, objeto, observação, fonte e declaração, não a versões, tempos ou número de caso. Conhecimento dos tipos de cenário/previsões é declarado; mudança de IDs não demonstra novidade estrutural ou avaliação independente de equipe. Não usar final para corrigir o mecanismo. Se final falhar, registrar e interromper sem ajustar.

Arquivos P1/P2 distintos; compartilhar apenas tipos passivos, trait e fixtures externas. P2 terá replay e seleção próprios, sem chamar validação/decisão de P1. Testes de operações inválidas usam exclusivamente entradas que ambos rejeitam e verificam atomicidade. Candidato não recebe caso, gabarito, motivo esperado ou resposta do outro executor. Query recebe &self e consultas não entram no diário de aquisição. Comparar clones antes/depois, decisões/provas/causas/dependências e registros integrais entre repetições. P1/P2 podem diferir em causas/provas materializadas pré-Refresh; comparar evidências onde decisões Supported e dados disponíveis forem equivalentes, sem exigir igualdade de artefatos internos inválidos de cache.

Fiscalizar referências de provas Supported: extremos, premissas, regra, versão, configuração, declaração quando necessária e lista de dependências. O avaliador compara provas nos pares Supported/Supported e o manifesto congelado, sem devolver informação aos modelos. Negativos de conflito/origem/hipótese/sucessão são ponta a ponta, não triagem textual. Não há texto natural ou circuitos. P2 é recuperação/recomposição forte, não lookup reduzido. Ausência de manutenção é sondada nas diferenças previstas; nenhum mecanismo neural exige ablação. Uma regra fixa/resposta sempre S falha nos negativos; abstenção constante não satisfaz cobertura prevista.

## 6. Métricas, sucesso e interrupção

Publicar por caso/condição: consultas, S, abstenções por motivo, erros técnicos, conformidade à tabela, respostas indevidas, falsas abstenções, histórico, conservação em d, provas e divergências P1/P2. Precisão de S sem S é indefinida; abstenções conformes não são acertos relacionais. Definir falsa abstenção frente à política da própria condição; diferenças pré-Refresh também mostram perda de cobertura frente ao controle, sem esconder ambas as leituras.

Sucesso local: toda tabela prevista, histórico15/15 por executor/repetição, operações rejeitadas sem mutação, consultas somente leitura, provas suficientes e nenhuma mistura de configuração. Erro de decisão sob execução válida falsifica o comportamento esperado; erro de implementação, contagem, orçamento, vazamento ou divergência inexplicada interrompe e torna o painel inconclusivo. Não alterar expectativas para obter sucesso. Retenção aqui é conservação simbólica após intervenções; não retenção em sinapses compartilhadas, compreensão ou aprendizagem autônoma.

## 7. Recursos e compatibilidade

Rust std, novos testes/suportes somente, sem alterações de produção, checkpoint, dependências ou protocolos históricos. Por trajetória:64 fatos/observações,64 provas materializadas,8 fontes,4 regras,8 configurações,128 declarações/dependências de prova e128 operações; 8192 inspeções/pares por chamada. Contadores instrumentados podem ser parciais quanto a cópias/validação e isso deve ser reportado, nunca eficiência geral garantida. Armazenamento vivo/fixtures por condição <=256 KiB incluindo repetições coexistentes, RSS isolado <256 MiB, painel <=30 s, /home >=3 GiB.

Guardas de tempo/disco/RSS nas fronteiras; monitor externo do painel com tempo monotônico, /proc e pico wait4 se disponível. Dados contados por capacidades reais das estruturas e logs; declarar lacunas do alocador/harness, sem confundir RSS com teto de dados. Não instalar instrumentos ou forçar compilação limpa. Interromper excesso observado. Registrar crescimento de target e recursos da regressão separados do painel. Não certificar instrumentação histórica por estas medidas.

Fundamentos originais, complemento A1–A7 e planos preservados. As diretrizes não se tornam capacidades demonstradas por este painel. Ao concluir, documentar resultado restrito e aguardar revisão; EC-06 não é iniciada.
