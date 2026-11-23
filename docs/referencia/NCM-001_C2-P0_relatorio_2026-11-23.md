# NCM-001/C2-P0 — Diagnóstico de integridade de evidência CPF

Data: 23/11/2026. Mandato: preparação diagnóstica, dados públicos sintéticos, sem correção produtiva. **Lacuna confirmada experimentalmente no classificador `ConceptGoalReport::cpf_evidence()`**: observação/escopo internos incompatíveis podem conservar `learning_evidenced=true`. Não significa conclusão da CPF-001 nem falha demonstrada de autorização do Runtime. **C2-P1 não iniciado e depende de nova autorização.**

## 1. Artefatos C1 recuperados e integridade Git

Os quatro arquivos já estavam disponíveis localmente. Cada um foi comparado byte a byte com `git show 4e231ca:<caminho>`, sem sobrescrita ou nova cópia sobre documentos existentes:

- [Relatório completo C1](NCM-001_C1_revisao_2026-11-23.md).
- [Manifesto de reprodução C1](NCM-001_C1_manifesto_2026-11-23.json).
- [Log de validação C1](NCM-001_C1_regressao_2026-11-23.log).
- [Patch operacional C1](NCM-001_C1_atualizacoes_operacionais_2026-11-23.patch).

Hashes SHA-256 recuperados estão no [manifesto C2-P0](NCM-001_C2-P0_manifesto_2026-11-23.json). Os hashes de evidências internos do manifesto C1 também foram conferidos. Nenhum artefato C0/C1 foi editado.

Entrada: branch `devs`, HEAD e `origin/devs` local em `4e231ca17ee88ff2870c6b91074c934e0172ee5b`; índice vazio. Consulta direta `git ls-remote origin refs/heads/devs` confirmou esse hash publicado. Estado pendente: **15 modificados, 544 não rastreados**; sem divergência remota na entrada. Todas as 73 fontes de C1 e os dois documentos operacionais coincidiram com os respectivos hashes de encerramento C1. Foram inventariados **897 arquivos não ignorados** antes de produzir os artefatos novos.

Fontes normativas mantidas: Controle Mestre, C0/C1, Plano Mestre (especialmente MP09, contratos F1/F2 e restrições §6–7), Visão aprovada, PIFI e critérios transversais, M1/CIC-I1 e parecer CPF-I03. A classificação de um relatório não é autenticação, prova de verdade ou certificação de competência; consulta não promove fatos. As restrições científicas e os bloqueios históricos permanecem vigentes.

## 2. Oito fontes produtivas pendentes

Todas permanecem **não rastreadas**, com os mesmos hashes de C1. São necessárias à compilação da biblioteca atual, não necessariamente ao caminho executado pelo diagnóstico. Nenhuma foi adicionada ao Git.

| Arquivo | Dependências/fronteira | Impedimento à publicação automática |
|---|---|---|
| `src/axon.rs` | Event/NodeId, Neuron/Emission, SynapseId; seleção de saídas chamada por Circuit | Conjunto AXN misturado com trabalho local; separar seleção de condução/política, revisar equivalência e contratos próprios |
| `src/integration/associative.rs` | Synaptic Experience, NeuronSnapshot, NodeId | Experimento associativo/CNF separado; C1/P0 não homologam suas APIs, testes ou alegações |
| `src/integration/cognitive_coordination.rs` | Runtime/discover, agenda e checkpoint do coordenador | Coordenação/E13 com aprovação da proposta não recuperada; persistência e semântica exigem revisão específica |
| `src/integration/development/discrimination/neural_priority.rs` | NodeId e medições/tipos de experiência neural, seleção/contrafactual experimental | N19/seleção experimental não integrada ao fluxo validado; separar engenharia de causalidade/aceite científico |
| `src/integration/e13_bundle.rs` | CognitiveCoordinator, Runtime/Access/Sensor, Session | Envelope E13 depende de proposta e contratos próprios ainda não liberados |
| `src/integration/e13_storage.rs` | Filesystem/OpenOptions/Read/Write, bundle E13, Runtime, Session e coordenador | Persistência/segurança e compatibilidade do envelope dependem de revisão; ativação E13 fora do mandato |
| `src/session/conversation.rs` | Intent/classify/clarification, Runtime CLI, buffers e E/S | Interface local preexistente precisa de revisão/publicação como unidade funcional própria e testes aplicáveis |
| `src/session/intent.rs` | Parser de intenções e esclarecimentos, consumido pela conversação | Parser/contrato textual preexistentes não se tornam entrega aprovada por compilação do diagnóstico |

Há ainda fontes **rastreadas modificadas** em currículo, Circuit/Engine/Neuron, declarações de módulos e Runtime CLI. Publicar apenas oito arquivos não produziria automaticamente uma versão coerente. Clone remoto sozinho continua insuficiente para reproduzir o conjunto local; hashes não são backup. Não se presume conteúdo malicioso nesses oito arquivos: o impedimento é revisão/agrupamento/autorização específicos ainda pendentes.

## 3. Implementação, consumidores e contrato observado

`GoalQuery` contém `goal` e `report`, ambos públicos; o relatório interno expõe sua própria observação e escopo. `cpf_evidence()` verifica execução concluída, objetivo resolvido, igualdade de `GoalQuery.goal`, suporte não vazio, versão crescente, estratégia TeachOnce e alguma lição admitida/revisada. **Não compara `query.report.observation` e `query.report.scope` com `self.goal`.**

Consumidores localizados: testes públicos CPF-I02 em `concept_goal_supervision`, testes de engenharia CPF-I03 e `verify_generalization()` em `curriculum.rs`. Não foi localizado uso do retorno como concessão de autoridade ou decisão de ação no Runtime/CLI. O verificador externo de generalização possui conferências próprias de observação/escopo, não é o mesmo classificador e não foi executado em P0. Não se infere vulnerabilidade dele por este resultado.

Os três testes CPF-I02 preexistentes verificam ensino válido/conhecimento prévio, controles sem aprendizagem e adulterações de IDs, revisões, estratégia e encerramento. Eles **não cobrem** as divergências internas agora reproduzidas. Não foram alterados nem reexecutados em P0: as fontes são idênticas às de C1, cujo resultado 82/0/0/0 é histórico preservado, não nova execução.

## 4. Reprodução focal segura

Artefato Rust [diagnóstico isolado](NCM-001_C2-P0_diagnostico_2026-11-23.rs), mantido em `docs/referencia/`, fora da descoberta automática de `tests/`. Classificado **seguro para execução sintética delimitada antes da execução**: não lê dados externos, não faz rede/E/S de arquivos/subprocessos no teste Rust; usa Session, supervisor MP09 e consultas somente leitura. O compilador/Cargo usa diretórios temporários exclusivos. O diagnóstico não implementa aprendiz alternativo ou uma correção do classificador.

Construção do caso público: conceito opaco 501, contexto 41, catálogo 91, regra 51 v1, validade [0,10000]. Uma experiência inicial `100001 +`; objetivo `100010` sem rótulo fornecido ao supervisor. Plano rotulado fixado antes da execução: `000001 -` e `100000 -`. Avaliações independentes sem rótulos para o supervisor: `110001` antes e `010101` depois. Dados reutilizam o cenário público de engenharia existente, sem partições científicas reservadas.

O supervisor intacto produz TeachOnce, revisão **v1 → v3** e decisão final `Supported(Negative)` para o objetivo. As intervenções ocorrem depois, em **clones dos relatórios**, sem modificar Session, memórias ou modelo do candidato. As comparações de Session.encode antes/depois asseguram que consultas/classificação/intervenções no relatório não alteraram o conhecimento original.

Cópia isolada nova `/tmp/neurons-ncm001-c2-p0/isolated`: 69 arquivos anteriores (67 fontes produtivas e Cargo.toml/Cargo.lock) mais um diagnóstico copiado como `tests/ncm_c2_p0.rs`. Depfiles confirmaram **68 arquivos Rust**: as 67 fontes e somente esse alvo diagnóstico. Testes internos reservados, EC-10 R3/suportes, outros testes/exemplos e painéis não foram copiados. E13/instrumentos permanecem definições compiladas na biblioteca, sem ativação ou chamada pelo diagnóstico.

Comando explícito, com CARGO_HOME/TMPDIR/target novos e variáveis herdadas NEURONS_*, CARGO_* e RUST* removidas:

```sh
cargo test --offline --locked --test ncm_c2_p0 -- --nocapture --test-threads=1
```

Sem dependências externas, instalação ou alteração de Cargo.toml. Timeout externo de 120 s não acionado. Primeira execução incluindo compilação: aproximadamente 9,957 s; rodada final após refinamento de anotação: 0,818 s, com target já compilado. Não são benchmarks equivalentes ou limites RSS/CPU. Doze avisos distintos `dead_code` (3 AXN, 9 prioridade neural), repetidos nas duas rodadas; sem erros de compilação.

## 5. Resultados e interpretação

**Seis testes diagnósticos passaram**, zero falhas/ignorados/filtrados, cobrindo **23 casos distintos**. PASS significa reprodução do comportamento atual documentado, incluindo o defeito; **não significa aprovação funcional dos casos incompatíveis**. O diagnóstico foi pré-fixado para evidenciar o comportamento anterior à correção e não deve ser incorporado como teste verde de conformidade em C2-P1.

| Grupo | Casos | Observação |
|---|---:|---|
| Ensino válido | 1 | Conhecimento e aprendizagem true, competência false; versões v1→v3 e objetivo preservados |
| Conhecimento já sustentado, sem ensino novo | 1 | Conhecimento true, aprendizagem false |
| Controle sem autorização de apresentação | 1 | Nenhuma lição; conhecimento/aprendizagem false; sessão preservada |
| Consulta inicial: observação ou campo do escopo incompatível | 6 | **Aprendizagem indevidamente true**; consulta final válida ainda pode sustentar conhecimento |
| Consulta final: observação ou campo do escopo incompatível | 6 | **Conhecimento e aprendizagem indevidamente true** |
| Relatório real de outra observação ou de outro contexto transplantado | 2 | **Conhecimento e aprendizagem indevidamente true** |
| Relatório original depois das intervenções nos clones | 1 | Continua válido e inalterado |
| Controles de guarda: ID inicial/final, revisão ausente, resultado parcial, execução interrompida | 5 | Aprendizagem false como previsto; conhecimento pode permanecer true quando a consulta final ainda é válida |

Os campos de escopo testados separadamente são catálogo, contexto, ID da regra, versão da regra e validade. Com observação, são seis intervenções por fase, sem trocar o `GoalQuery.goal` externo.

**Confirmação:** 14 falsos positivos de aprendizagem, dos quais oito também são falsos positivos de conhecimento para o objetivo declarado. `competence_demonstrated` permaneceu false em todos os 23 casos. Não houve soma das duas rodadas como 46 casos ou 12 testes independentes.

Contraexemplo mais forte: uma consulta real posterior, na mesma sessão, a `110001` retorna `Supported(Positive)`. Inserir esse relatório dentro do invólucro final do objetivo original `100010` (cuja resposta genuína era negativa) conserva `knowledge_supported=true` e `learning_evidenced=true`. Portanto o defeito não exige inventar uma decisão inválida dentro do relatório substituto: a decisão é válida para **outra observação**, não para o objetivo reivindicado. Um relatório real do contexto 42 transplantado para o invólucro do contexto 41 também foi aceito.

Transparência metodológica: na primeira rodada, `expected_knowledge=false` também foi anotado conservadoramente nos seis casos de consulta **inicial** incompatível. Isso era mais estrito que o necessário: uma consulta final válida pode sustentar conhecimento, mas não provar a aprendizagem por comparação com aquela consulta inicial. Foi refinada somente essa anotação, antes da rodada final. Dados, intervenções, expectativas de aprendizagem e asserções de reprodução do defeito permaneceram iguais. Fonte preliminar/final são identificadas por hash e diff no manifesto; [log completo](NCM-001_C2-P0_validacao_2026-11-23.log) preserva ambas as rodadas. Nenhum critério científico preexistente foi alterado.

Limite da conclusão: inconsistência de **classificação de relatórios fornecidos ao método**, não corrupção automática de memória, escalada de permissão, execução de ação não autorizada ou erro em todas as saídas genuínas do supervisor. Não demonstra generalização, competência ou conclusão integral de CPF-001.

## 6. Proposta mínima C2-P1 — não implementada

1. Validar a ligação completa da consulta usada como evidência: `query.goal == self.goal`, `query.report.observation == self.goal.observation` e `query.report.scope == self.goal.scope` (Scope completo, incluindo catálogo/regra/versão/validade).
2. Para conhecimento, exigir consulta final válida quando ela estiver presente; quando não houver consulta final, permitir o caminho de resposta inicial válida. Evitar que filtrar uma consulta final malformada e usar fallback silencioso esconda a inconsistência.
3. Para aprendizagem, exigir consultas inicial **e** final válidas e vinculadas ao mesmo objetivo, além de preservar os gates existentes de execução, estratégia, revisão, admissão e suporte. Consulta inicial incompatível deve impedir aprendizagem, mesmo com resposta final sustentada.
4. Preservar conhecimento já sustentado sem ensino, controle sem ensino e `competence_demonstrated=false`. Não ampliar escopo para autenticação criptográfica, semântica completa de todos os relatórios, mudança H23/MP09 ou novos formatos.
5. Antes de alterar produção, transformar os contraexemplos públicos pertinentes em testes de conformidade separados, observar a falha no código atual e só então corrigir minimamente, mediante autorização C2-P1. Revalidar o alvo de supervisão e dependentes seguros C1 após a correção. O diagnóstico histórico P0 permanece como evidência, não deve ser reescrito para simular aprovação anterior.

## 7. Rastreabilidade, publicação e retomada

Todos os 897 arquivos de entrada foram reconferidos por SHA-256 depois da execução diagnóstica, sem alteração. Depois, somente Controle Mestre e progresso recebem acréscimos C2-P0; seus conteúdos anteriores permanecem prefixos idênticos. O [patch operacional próprio](NCM-001_C2-P0_atualizacoes_operacionais_2026-11-23.patch) registra apenas esses acréscimos; hashes antes/depois constam do manifesto. Como em C1, os documentos integrais misturados permanecem fora do staging.

Arquivos próprios preparados para publicação nominal: este relatório, manifesto JSON, diagnóstico Rust isolado, log das duas rodadas e patch operacional C2-P0. Revisar conteúdo, segredos, JSON, hashes, formatação somente do diagnóstico, whitespace e aplicação reversa do patch antes do commit. Nenhuma fonte produtiva ou teste preexistente deve entrar no índice. Hash efetivo e confirmação do push são informados na entrega da sessão, sem autorreferência circular no relatório.

Riscos preservados: EC-10 R3 e suportes intactos e não executados; EDF/R2-F sem contrato recuperado; E13 não ativado; nenhuma integração temporal axonal; backup independente/segurança operacional ainda pendentes; onedell não acessado. Regressão C1 e diagnóstico P0 não liberam esses conjuntos nem gates de produção.

**Próximo checkpoint recomendado: NCM-001/C2-P1 — Implementação mínima da validação de vínculo de evidência**, exclusivamente após nova autorização baseada neste relatório. Até lá, preservar o código produtivo atual e tratar `cpf_evidence()` isoladamente como insuficiente para aceitar relatórios com vínculo interno não conferido. Nenhuma implementação prossegue automaticamente ao fim desta sessão.
