# EC-05 — Resultados de retenção contextual sob D1/D2

## 1. Escopo, parecer e arquivos

Base 38d5325b76b4694113a300e7b10424fa42404ef5, devs inicialmente limpa; HEAD/origin/devs/referência remota confirmados. Delegação “Retomar avaliação de retenção contextual”. Executada somente a última subetapa literal EC-05 do plano v0.2, **avaliar retenção sem misturar versões de contexto**. [Protocolo prospectivo](protocolos/EC-05_protocolo_retencao_contextual.md), [preparação histórica](protocolos/EC-05_preparacao_retencao_contextual.md) e [proposta histórica de política](EC-05_politica_revisao_abstencao.md) são documentos distintos.

**Resultado funcional: conforme às previsões no painel executado. Parecer científico: AJUSTAR**, por escopo sintético, declarações externas e instrumentação parcial. Subetapa experimental concluída; não se certifica política cognitiva universal ou cumprimento integral de todos os limites finos. A EC-05 pode ser submetida à revisão para encerramento delimitado, preservando ressalvas. Nenhuma etapa posterior foi iniciada.

Criados somente teste/suportes experimentais novos, protocolo e este relatório; acrescidos decisões e índice de fundamentos. Produção, DTOs/executores históricos, checkpoint, planos, Fundamento original, complemento A1–A7 e experimentos anteriores permaneceram inalterados. O novo laboratório não corrige retroativamente os executores EC-03/05, que continuam com bloqueio global por conflito.

## 2. D1 e D2 operacionalizadas

**D1:** prova de dois passos referencia versões exatas de fatos/regra, observações, cadeias de origem das fontes e declarações de aplicabilidade. Conflito de premissa compromete as observações associadas, não transforma automaticamente toda a fonte em falsa. Outra via precisa estar íntegra, suficiente e rastreável. Regra comum ativa e não contestada é admissível; regra comum Contested ou ancestral comum Contested invalida ambas. Compartilhar a observação comprometida também invalida a alternativa. Origem desconhecida não é raiz independente: em presença de conflito impede responder. Isso demonstra separação operacional relativa a dependências declaradas, não independência estatística de fontes reais.

**D2:** Context(id,versão) preserva identidade lógica e sucessão declarada consecutiva; predecessor de outra identidade/salto são rejeitados. Sem herança automática. DeclareApplicable exige referência exata, predecessora aplicável, sucessora imediata e declaração externa identificável. Fato e regra requerem aplicabilidade separada. A declaração não renova validade ou retira suspensão. Versão da representação, versão do contexto, época e corte continuam distintos. Consultas de versões anteriores não servem como suporte implícito da corrente.

P1 mantém provas por Refresh e revalida suporte imediatamente em Query. P2 reconstrói estado e compõe diretamente do diário autorizado, sem chamar P1. Ambas preservam diário; Query recebe &self, não incorpora observações avaliativas. Origens, estados Contested, contextos, regra de composição e continuidade são fornecidos pelo professor externo, não adquiridos autonomamente. P3/recência EC-04 e P4 não participam.

## 3. Cronologia e prevenção de vazamento

A conferência documental de completude explicitou, antes dos testes, uma regra própria na nova configuração do caso2. Sem essa regra a última resposta prevista não teria suporte, pois regras também não são herdadas. Foi completada a sequência de entrada mantendo os resultados previstos e sem consultar uma execução para escolhê-la. O hash inicial de redação 014d012aaf816d0ca56f8cba7ea2b85a8846de6d7de7e0d477f3067a39e65c09 foi superado nessa conferência; não é apresentado como protocolo operacional final. A versão consistente tem o hash abaixo, registrado antes do desenvolvimento. Não houve revisão de regra científica após resultados.

Desenvolvimento: casos1/3/6/12/16, offset10000, **42 verificações** (34 consultas e8 erros). Validação prévia: casos2/4/7/9/13/15, offset20000, **54 consultas**. Ambos passaram na primeira execução desses testes; a expectativa auxiliar de contagem histórica da validação foi conferida antes de executá-la: sete por executor, não seis. Não alterou decisões, dados ou protocolo.

Depois da preparação, manifestos e os seis arquivos de referência foram congelados; executado o final offset30000 em duas sequências globais independentes completas. Nenhum deles foi alterado depois de consultar o final, inclusive após regressão. Manifestos são representações Debug determinísticas, não um formato persistente universal. Mudança de IDs entre partições não é novidade estrutural; autoria, expectativas e pressupostos são conhecidos. Não se afirma avaliação cega de equipe ou pré-registro independente.

Os modelos importam somente DTOs/trait, não o arquivo de fixtures ou o avaliador. Recebem apenas Op, Query e corte. As expectativas são consultadas pelo harness depois do retorno. As consultas não são reapresentadas em aquisição; clones antes/depois fiscalizam não mutação e atomicidade das rejeições. O avaliador verifica referências, fontes/origens, estados, versões, intervalos, contexto, continuidade e dependências de provas sustentadas. A decisão negativa e o isolamento são controlados pelas previsões e P2; o fiscal de provas não é uma terceira implementação completa de oposição/seleção. Independência de implementação não implica independência de pressupostos/equipe.

## 4. Resultados observados versus previstos

Todas as sequências coincidiram com a tabela do protocolo, inclusive diferenças P1/P2. Valores por executor e repetição; ambas as repetições foram iguais nos registros completos comparáveis.

| Caso | Consultas | Supported P1 | Supported P2 | Resultado distintivo observado |
|---|---:|---:|---:|---|
| 1 Contextos distintos |4|1|2|Sem composição c/d; P2 responde antes de Refresh após completar c.|
| 2 Sucessão sem herança |6|3|4|Sem prova mista; suporte integral novo em c2 e regra própria; histórico c1 preservado.|
| 3 Revisão |4|3|4|P1 Stale pré-manutenção; P2 usa A2; histórico conserva A1.|
| 4 Validade |5|3|4|Fronteira temporal respeitada; P2 usa nova evidência antes de Refresh.|
| 5 Suspensão/retratação |6|3|3|Duas Suspended e uma Retracted; tentativa de retomada terminal rejeitada.|
| 6 Conflito localizado |3|3|3|Via W sustenta apesar da via Y conflitante; oposição não é apagada.|
| 7 Observação comum |3|2|2|Alternativa compartilha observação comprometida: Conflict.|
| 8 Regra comum comprometida |3|2|2|Contested em regra essencial impede ambas as vias.|
| 9 Origem comum comprometida |3|2|2|Fonte ancestral Contested impede ambas as vias.|
| 10 Origem desconhecida |3|2|2|Conflito com alternativa não rastreável: Indeterminate.|
| 11 Oposição à conclusão |3|2|2|Nenhuma via contorna oposição direta: Conflict.|
| 12 Continuidade declarada |6|3|4|Declarar somente fatos não basta; regra também precisa de declaração; P2 mais pronta.|
| 13 Contexto não afetado |7|6|6|Mudança em c não elimina suporte de d; histórico de ambos preservado.|
| 14 Origem comum íntegra |4|3|3|Conflito localizado admite W; comprometer origem comum passa a impedir ambas.|
| 15 Hipótese insuficiente |3|2|2|Hipótese não sustenta alternativa; Conflict sem outra prova.|
| 16 Erros |0|0|0|Quatro rejeições atômicas por executor/repetição.|
| **Total** |**63**|**40**|**45**|Mais cinco erros, incluindo a rejeição do caso5.|

Distribuição de decisões **nas duas repetições**, denominador126 por condição:

| Condição |Supported|Indeterminate|Conflict|Suspended|Retracted|Expired|Stale|Unknown|
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| P1 |80|2|12|6|2|4|2|18|
| P2 |90|2|12|6|2|2|0|12|

Cobertura P1:80/126 (~63,49%); P2:90/126 (~71,43%). Precisão entre respostas sustentadas, frente ao contrato avaliado:80/80 e90/90. Nenhuma resposta indevida ou falsa abstenção **em relação à expectativa específica de cada política**, nenhum erro inesperado ou inconclusão de execução. Comparado ao suporte utilizável por P2, P1 tem dez abstenções adicionais pré-manutenção; preservar essa diferença de utilidade, sem mascará-la como equivalência ou defeito automático.

Decisões concordantes:116/126; dez diferenças previstas (cinco por repetição, casos1/2/3/4/12). Evidências das respostas Supported/Supported coincidiram em80/80 comparações de conjuntos de provas e dependências. Vias não sustentadas/materializações históricas não foram artificialmente igualadas. Registro de motivos pode diferir quando P1 ainda não materializou topologia; todos os motivos devolvidos foram preservados entre repetições.

## 5. Contagens, retenção e integridade

| Unidade | Previsto | Observado |
|---|---:|---:|
| Consultas finais,63×2 condições×2 repetições |252|252|
| Erros rejeitados,5×2×2 |20|20|
| Total de verificações finais |272|272|
| Consultas históricas incluídas nas252,15×2×2 |60|60|
| Eventos neuronais |0|0|

Cada executor reproduziu15/15 respostas históricas completas por repetição, com consulta/época/corte, decisão, motivos, provas, conflitos e dependências. Após mudança corrente, evidência posterior não apareceu no corte antigo. Todas as leituras/rejeições comparadas conservaram estado. O caso13 conservou as decisões de d e suas provas sustentadas, fiscalizadas no escopo d; não é resistência à interferência em sinapses ou teste longitudinal natural. O vínculo de sucessão e a aplicabilidade são descrições externas explícitas, não descoberta de identidade contextual.

Repetições compararam integralmente Record por condição (caso, posição, resposta/erro e custo retornado); fixtures/comandos determinísticos conservam a sequência de entradas. Estados iniciais são novos em cada cenário. Os registros completos são mantidos e comparados em memória durante o teste; stdout publica tabelas/contagens e hashes, não um arquivo permanente de todos os estados intermediários. A regressão reexecutou o mesmo painel existente; não somar suas consultas às272 verificações do painel isolado.

## 6. Recursos e regressão

Rust1.88.0/Cargo1.88.0, offline, sem instalação. Painel isolado `target/debug/deps/ec05_retention-b2c5ab7f52975b48 frozen_final_panel --exact --nocapture --test-threads=1`:0,053512 s internos;0,056992 s externos. wait4 reportou8196 KiB; /proc amostrado a cada2 ms reportou1460 KiB. wait4 inclui o escopo do processo lançado/fork-exec e contabilização do sistema; não mede apenas estruturas Rust. Amostragem pode perder picos. Watchdog externo30 s/RSS256 MiB/disco3 GiB não acionou; guardas internas nas fronteiras também passaram. Menor espaço amostrado durante painel:4497006592 bytes.

Pico contabilizado de dados final:170144 bytes, incluindo fixtures, registros coexistentes, clones/estados, respostas salvas e reserva transitória32 KiB. Primeira repetição118008 bytes. **Contabilização parcial:** Snapshot P2 usa BTreeMap e alocações transitórias não são medidas integralmente; a reserva não é auditoria do alocador. Contagem nominal máxima de trabalho60, mas cópias, buscas e comparações internas não são uniformemente instrumentadas. Não se certifica custo fino integral, garantia geral dos tetos8192/256 KiB ou superioridade de eficiência. Não foi observado excesso; RSS/tempo não substituem a demonstração do teto de dados.

| Verificação | Resultado | Tempo externo(s) | Pico reportado KiB |
|---|---|---:|---:|
| cargo fmt --check | passou |0,279602|47452|
| cargo check --offline | passou |0,062439|25228|
| cargo check --offline --all-targets | passou |0,179379|123256|
| cargo test --offline -- --test-threads=1 | **140 aprovados,0 falhos,0 ignorados** |2,995396|87216|
| git diff --check | passou |0,005529|9112|

Preservados os136 testes anteriores; quatro novos: desenvolvimento, validação, preflight e painel final. Os três avisos de método declare não utilizado nos suportes EC-04 são preexistentes e não foram corrigidos. Maior pico nos comandos medidos123256 KiB; a compilação inicial do novo teste e a preparação anterior não tiveram RSS separado, e não se forçou rebuild limpo. Os picos não são soma instantânea da árvore de processos.

Target:57618855→59619598 bytes, crescimento2000743 bytes (~1,91MiB). /home inicial4502945792 bytes; após regressão4496736256 (~4,19GiB), acima3GiB. Conferir novamente após documentação/push. Sem novas dependências, serviços, mecanismos de produção ou alterações dos fundamentos.

## 7. Conformidade e limites científicos

| Requisito | Parecer |
|---|---|
| D1, dependências compartilhadas e isolamento | Conforme nos casos6–11/14/15; regra de comprometimento é fornecida, não descoberta. |
| D2, identidade/sucessão/aplicabilidade | Conforme nos casos1/2/12/16; continuidade declarada, sem herança presumida. |
| História, não mutação e conservação contextual | Conforme nos casos exercitados; replay de decisão não é History universal ou persistência em disco. |
| P2 forte/independência de implementação | Código de replay/join/validação próprio; mesmos dados e sinais; mesma autoria/família/avaliador limitam independência científica. |
| Separação aquisição/avaliação | Conforme nos fluxos; DTOs sem gabarito; contexto/metadados externos não são conhecimentos autonomamente adquiridos. |
| Repetibilidade | Registros determinísticos iguais; não amplia a população ou prova generalização. |
| Trabalho/dados finos | Parcial; lacunas de instrumentação explicitadas, sem certificação integral. |
| RSS/tempo/disco nesta execução | Sem excesso observado, com instrumentos/escopos/guardas declarados. |

Resultados sustentam retenção e revisão **simbólicas delimitadas**, aplicabilidade contextual explícita e abstenções previstas. Não demonstram aprendizagem neural, formação autônoma de engramas, compreensão, credibilidade de fontes aprendida, transferência estrutural, investigação autônoma ou memória integrada em produção. Os engramas, motor de eventos e checkpoint não foram adaptados; zero participação causal neural. A1–A7 continuam diretrizes aprovadas de desenvolvimento, integralmente preservadas, não conquistas deste ensaio.

O teste cobre continuidade até a segunda versão, alguns tipos de dependência e três estados de impedimento; não exaure todas as capacidades, cadeias longas de sucessão, retratação distribuída, ciclos arbitrários, corrupção ou fontes reais. Regras/limiares de precedência permanecem programados. Nenhum resultado histórico é corrigido ou certificado retroativamente. A informação adicional de D1/D2 impede usar este painel como comparação causal isolada com o antigo bloqueio global.

**Próximo ato:** revisão destes resultados e decisão de encerramento delimitado EC-05 com ressalvas. Próxima etapa nominal do plano é EC-06 — Raciocínio com evidências, começando por auditar inferência existente; não foi iniciada nem recebe autorização por este relatório.

## 8. Custódia

Arquivos congelados antes do primeiro final e conferidos intactos depois da regressão:

```text
53f85c47579de84cf767a24b0d12949bd9f6245434a1455a62ef95a2a2c87391  docs/protocolos/EC-05_protocolo_retencao_contextual.md
9660a2925a62f89d3cd54d8b4f1c871fcb23528a12f8453b7e8ca2a5240a40d8  tests/ec05_retention.rs
546f23864dddc70fb8457c6bbf0a2a863e359596ebeaf1190cb0bcfbaaee4681  tests/support/retention_types.rs
3cf7be5e569d1c6122d37144be860c48e67df0e1c651443e27026b8faa469ca7  tests/support/retention_candidate.rs
32a2efb250061eef79e35f4c85fd1914fc80453e12ff4b535575c78eb38ee43e  tests/support/retention_symbolic.rs
d85815644ae4b82d353dd3147856867f408bee342409035b0e3a3d735d867fe6  tests/support/retention_cases.rs
```

Manifestos completos canônicos,24362 bytes cada, preflight determinístico:

```text
offset10000 c958aa7c922fb0c1aed911691e5ed4d37b4f5acb5eaf39921e0028602a1f9f1e
offset20000 5505494cca7f07916b3034f2eb60a80026ad5235eec631ce5b3d8e407d4a604b
offset30000 8ffc62e1576601c4e3838d2e33d9e02139735478a937cb14bbd25a3d9ab54f7d
```

Preflight também materializa as definições completas na faixa de preparação, embora desenvolvimento/validação executem somente os subconjuntos previstos; esses manifestos não são alegação de novidade ou de observação prévia das decisões finais. Resultado final só foi consultado após congelamento dos mecanismos/avaliador. A versão Debug depende do código/compilador declarados. Hashes e commit preservam bytes atuais, não prova independente da cronologia histórica. Commit/push finais constam da entrega após versionamento, evitando autorreferência.
