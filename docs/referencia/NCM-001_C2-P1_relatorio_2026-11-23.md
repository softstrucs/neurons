# NCM-001/C2-P1 — Correção focal da classificação de evidência CPF

Data: 23/11/2026. Mandato: implementação controlada autorizada pelo responsável, com base no ZIP de evidências P0 e no checkpoint `6515675`. **Critério técnico atendido no recorte: nenhum dos 14 falsos positivos permaneceu; oito atribuições indevidas de conhecimento P0 e três fallbacks adicionais corrigidos; controles preservados.** Nenhuma competência nova declarada, nenhuma mudança operacional do Runtime.

## 1. Entrada, autorização e integridade

Entrada `devs`, HEAD `651567553cb4b23b9edd48f55c53bc08364144cc`; índice vazio. `git ls-remote origin refs/heads/devs` confirmou o mesmo hash antes da execução. ZIP `NEURONS_NCM-001_C2-P0_EVIDENCIAS.zip` conferido por SHA-256 `5310218514b66402039febc1d70c82295b76dcd7d66442a743b7190f4edd0445`, CRC de todas as entradas aprovado. Cinco artefatos P0 iguais ao commit; 69 dependências P0 com hashes idênticos. Inventário de entrada: 902 arquivos não ignorados, 15 rastreados modificados e 544 não rastreados.

Controle Mestre, Plano Mestre/MP09/F1/F2, Visão aprovada, PIFI, contratos aplicáveis e limites C0/C1/P0 mantêm precedência e restrições. A autorização atual libera somente este ajuste C2-P1 e suas verificações; os estados históricos de espera por autorização em P0 foram preservados e complementados, não apagados. EC-10 R3/suportes, EDF/R2-F, E13, AXN temporal e onedell não foram executados, alterados ou ativados.

## 2. Testes de conformidade antes da correção

Novo arquivo público `tests/cpf_evidence_conformance.rs`, independente dos diagnósticos históricos: não importa/include P0 e não reescreve suas asserções. O cenário usa os mesmos dados públicos para comparabilidade; a construção da Session e as expectativas de conformidade são próprias. As expectativas são fixadas pelo vínculo objetivo/observação/escopo e não calculadas pelo método candidato.

Classificação anterior à execução: **seguro no recorte sintético**. Usa Session/MP09, consultas imutáveis e clones de relatórios, sem leitura de datasets, rede, subprocessos ou E/S de arquivos no alvo novo. Cópia seletiva de 74 arquivos (73 do conjunto C1 e teste novo), sem outros testes/instrumentos reservados; Cargo offline/locked e CARGO_HOME/target/TMPDIR próprios, variáveis herdadas NEURONS_*, CARGO_* e RUST* removidas.

Comando focal antes e depois, com exatamente os mesmos bytes do teste:

```sh
cargo test --offline --locked --test cpf_evidence_conformance -- --nocapture --test-threads=1
```

**Antes, produção intacta P0: 9 aprovados, 17 falhos, zero ignorados/filtrados, retorno 101.** Todas as 17 falhas foram `assert_eq` de CpfEvidence por vínculo incorreto: 14 casos P0 e três fallbacks. Compilação e montagem dos casos funcionaram; não foi uma falha por símbolo ausente, ambiente ou expectativa mutante. Hash do teste congelado antes/depois consta do manifesto.

## 3. Correção efetivamente aplicada

Somente o trecho de `ConceptGoalReport::cpf_evidence()` em `src/curriculum.rs` foi alterado em relação à árvore local de entrada. [Diff local aplicado](NCM-001_C2-P1_diff_aplicado_2026-11-23.patch): substitui nove linhas por quinze, sem formatação global ou modificação de blocos alheios.

```rust
let matches_goal = |q: &GoalQuery| {
    q.goal == self.goal
        && q.report.observation == self.goal.observation
        && q.report.scope == self.goal.scope
};
let initial = self.initial.as_ref().filter(|q| matches_goal(q));
let final_query = self.final_query.as_ref().filter(|q| matches_goal(q));
let knowledge_query = if self.final_query.is_some() {
    final_query
} else {
    initial
};
let supported = knowledge_query.is_some_and(|q| goal_supported(&q.report));
```

Scope é comparado integralmente: catálogo, contexto, ID/versão da regra e validade. Para aprendizagem, as opções inicial/final agora só existem após validar o vínculo interno. Para conhecimento, uma final presente mas inválida não cai silenciosamente na inicial. Sem final, a resposta inicial válida já sustentada continua possível.

Guardas preservadas literalmente: execução Completed, resultado Resolved, suporte/consulted/versão/critérios não vazios, TeachOnce, insuficiência inicial, suporte final, revisão crescente, alguma lição vinculada admitida/revisada. `competence_demonstrated=false`. Nenhuma alteração em permissões, autonomia, política de ação, estado da Session, H23, MP09, verificador científico, formatos ou limites. A verificação de igualdade do estado da Session integra os testes.

## 4. Matriz dos 23 casos P0 atualizada

C/A = conhecimento sustentado / aprendizagem evidenciada, 1=true, 0=false. Competência foi false em todos os casos, antes e depois.

| Caso original P0 | Antes C/A | Depois C/A | Esperado C/A |
|---|---|---|---|
| `guard_final_goal_id` | 0/0 | 0/0 | 0/0 |
| `guard_initial_goal_id` | 1/0 | 1/0 | 1/0 |
| `guard_revision` | 1/0 | 1/0 | 1/0 |
| `guard_outcome` | 0/0 | 0/0 | 0/0 |
| `guard_execution` | 0/0 | 0/0 | 0/0 |
| `positive_taught` | 1/1 | 1/1 | 1/1 |
| `real_other_observation` | 1/1 | 0/0 | 0/0 |
| `real_other_context` | 1/1 | 0/0 | 0/0 |
| `initial_observation` | 1/1 | 1/0 | 1/0 |
| `initial_catalog` | 1/1 | 1/0 | 1/0 |
| `initial_context` | 1/1 | 1/0 | 1/0 |
| `initial_rule_id` | 1/1 | 1/0 | 1/0 |
| `initial_rule_version` | 1/1 | 1/0 | 1/0 |
| `initial_validity` | 1/1 | 1/0 | 1/0 |
| `final_observation` | 1/1 | 0/0 | 0/0 |
| `final_catalog` | 1/1 | 0/0 | 0/0 |
| `final_context` | 1/1 | 0/0 | 0/0 |
| `final_rule_id` | 1/1 | 0/0 | 0/0 |
| `final_rule_version` | 1/1 | 0/0 | 0/0 |
| `final_validity` | 1/1 | 0/0 | 0/0 |
| `control_original_after_mutations` | 1/1 | 1/1 | 1/1 |
| `control_already_supported` | 1/0 | 1/0 | 1/0 |
| `control_no_teaching` | 0/0 | 0/0 | 0/0 |


Os 12 campos internos foram alterados separadamente (observação + catálogo/contexto/regra/versão/validade, na inicial e na final); os dois transplantes usam relatórios reais de outra observação/contexto. Nos seis casos com inicial incompatível, a final válida conserva conhecimento, mas deixa de evidenciar aprendizagem. Nos oito com final incompatível, conhecimento e aprendizagem ficam false.

Três casos adicionais de fallback, com inicial genuinamente sustentada e final presente inválida: `fallback_final_observation`, `fallback_final_scope`, `fallback_final_goal`. Antes C/A=1/0 nos três; depois C/A=0/0. O controle `control_already_supported` mantém C/A=1/0 quando não há final.

**Depois: 26 aprovados, zero falhas/ignorados/filtrados.** Os 23 casos P0 e três adicionais constam nominalmente do [CSV completo](NCM-001_C2-P1_matriz_2026-11-23.csv). Não se executou o diagnóstico P0 contra o código corrigido; ele continua evidência histórica do defeito e permanece byte a byte intacto.

## 5. Regressão pública C1 e avisos

Os três alvos atuais e suas dependências mantiveram hashes C1/P0, exceto o currículo corrigido. Segurança revisitada: dados sintéticos, subprocessos locais e arquivos temporários próprios nos testes CLI; worker com variáveis herdadas saneadas; nenhum dado/instrumento reservado ou chamada E13. O novo classificador não cria chamadas ao Runtime.

Executor público existente, na cópia seletiva:

```sh
python3 scripts/test_public.py --build --target runtime_cli --target concept_goal_supervision --target development_runtime --target-dir /tmp/neurons-ncm001-c2-p1/target-working
```

| Alvo atual local | Aprovados | Falhas/ignorados/filtrados |
|---|---:|---|
| runtime_cli | 33 | 0/0/0 |
| concept_goal_supervision | 20 | 0/0/0 |
| development_runtime | 29 | 0/0/0 |
| Total regressão | 82 | 0/0/0 |

Com os 26 novos, **108 aprovações no conjunto local delimitado**. Há 12 avisos distintos dead_code preexistentes (3 AXN, 9 neural_priority), repetidos pelas compilações. As exclusões defensivas dos dois probes históricos não correspondem a testes filtrados aqui: eles não existem nos alvos selecionados. Não houve suíte global, testes internos cfg(test), painéis ou generalização científica.

## 6. Isolamento produtivo para publicação e reprodução limpa

Descoberta importante: **CpfEvidence e seu método ainda não existiam no HEAD**, embora estivessem na árvore local auditada. Portanto publicar só uma edição de poucas linhas sobre o HEAD deixaria um patch sem objeto. Foi preparado e revisado integralmente um recorte coerente: somente declaração CpfEvidence e impl corrigida, inseridas no currículo publicado, além do teste novo e evidências. São 66 linhas adicionadas à versão Git; não incluem o verificador CPF-I03/GeneralizationResult nem outras mudanças históricas.

O [diff de publicação](NCM-001_C2-P1_diff_publicacao_2026-11-23.patch) é diferente do diff local e documenta essa primeira introdução versionada da superfície necessária. Nada mais do currículo HEAD muda nesse candidato. O restante do arquivo local continua preservado, fora do índice. O staging produtivo deve usar exclusivamente esse recorte, nunca `git add src/curriculum.rs` sobre a árvore misturada.

Antes de preparar o índice, foi montada uma segunda cópia nova com **66 arquivos**, todos extraídos dos objetos Git HEAD, exceto o currículo com esse recorte e o novo teste. Nenhum dos oito fontes produtivos não rastreados foi transportado ou necessário. CARGO_HOME/target/TMPDIR novamente novos, sem binários anteriores, mesmos Linux/Rust/Cargo1.88.0 e std, sem crates externas.

Resultado do candidato versionável: **26 novos +75 regressões =101 aprovados, zero falhas/ignorados/filtrados**, sem avisos. As versões dos três alvos já no Git têm 17 concept_goal_supervision, 28 development_runtime e 30 runtime_cli; as sete adições locais dessas suítes não foram publicadas por arrasto. Por isso 75 não contradiz as 82 regressões atuais locais: os conjuntos de arquivos de teste são explicitamente diferentes. Manifestos identificam hashes dos dois conjuntos.

Depfiles confirmaram 71 dependências Rust no conjunto local (67 produtivas+4 alvos) e 63 no candidato limpo (59 produtivas+4 alvos). Nenhuma dependência EC-10 R3, teste interno reservado ou fonte inesperada. A cópia limpa é extraída seletivamente dos objetos Git, não instalação em outro equipamento nem clone remoto integral. Ela demonstra que **o recorte publicável pode ser reproduzido sem os oito fontes locais**; um clone que contenha o checkpoint final pode executar esses quatro alvos nominais offline/locked no ambiente descrito. Não autoriza suíte global nem significa que a árvore local inteira ou todas as capacidades C1/E13/AXN estejam publicadas.

## 7. Evidências e critérios de aceite

[Manifesto](NCM-001_C2-P1_manifesto_2026-11-23.json): hashes antes/depois do currículo, pre-registro, teste congelado, matrizes e duas seleções de fontes, comandos/retornos/tempos e integridade. [Log integral](NCM-001_C2-P1_validacao_2026-11-23.log): antes, depois, regressão local e duas verificações do candidato limpo. Os logs são concatenados com identificação de fase, apenas quebras vazias no EOF normalizadas; originais temporários identificados por hash.

Tempos de parede de processo, incluindo compilação/preparação pelo Cargo: antes9,704s; depois10,151s; regressão local3,331s; conformidade limpa9,559s; regressão limpa2,850s. Perfis/cache e recompilações diferem: não são benchmark comparativo. Timeout não acionado; RSS/CPU por fase não certificados.

| Critério | Evidência | Resultado |
|---|---|---|
| Falhas na versão anterior pelos motivos esperados | 17 asserts de vínculo, nenhum erro de compilação | Atendido |
| Eliminar 14 falsos positivos P0 | Matriz nominal, learning=false nos 14 | Atendido |
| Eliminar oito atribuições indevidas e fallback | Knowledge=false nos oito e nos três adicionais | Atendido |
| Manter conhecimento/aprendizagem válidos e guardas | Controles novos e 82 regressões atuais | Atendido no recorte |
| Não declarar competência nova | Campo false em todos os casos | Atendido |
| Preservar estado/Runtime/permissões | Diff focal e comparação Session.encode | Atendido no recorte |
| Separar publicação de histórico não revisado | Recorte66 linhas e cópia limpa101 | Atendido antes do staging; conferir índice exato |
| Preservar diagnósticos/protocolos | Hashes das 902 entradas, exceções autorizadas identificadas | Atendido com atualização documental aditiva |

## 8. Arquivos, integridade, riscos e continuidade

Alterações próprias: trecho de `src/curriculum.rs`; novo `tests/cpf_evidence_conformance.rs`; relatório/manifesto/matriz/log, dois diffs produtivos e patch operacional C2-P1 em `docs/referencia/`. Controle Mestre e progresso receberam somente acréscimos, com seus bytes anteriores preservados e hashes registrados; documentos integrais misturados permanecem locais, acréscimos publicados em patch.

Arquivos de entrada não relacionados permanecem idênticos. As oito fontes não rastreadas seguem intactas e fora do staging; embora ainda sejam necessárias à árvore local completa, não são dependências do recorte Git validado. A seção CPF-I03 do currículo permanece local, sem alteração. Pré-publicação exige conferir exatamente os bytes do currículo candidato e do teste congelado, lista nominal do índice, whitespace, hashes, segredos e remoto sem divergência. Hash efetivo e push confirmado do checkpoint serão informados na entrega final, sem autorreferência circular neste relatório.

Riscos remanescentes: a checagem de vínculo não autentica relatórios nem valida toda semântica possível de versões, provas e permissões. CPF-001/E10 integrais, retenção prolongada, transferência ampla, causalidade neural, segurança de produção, backup/restauração/rollback independentes e gates onedell não são resolvidos por este ajuste. EC-10 R3 continua com conflito científico preservado; EDF/R2-F sem contrato recuperado; E13 e integração temporal AXN fora do mandato.

**Próxima etapa recomendada:** revisão do checkpoint isolado e sua reprodução nominal, depois preparar a menor prova end-to-end CPF supervisionada com evidência de aquisição, avaliação inédita, controle sem ensino, retenção em processo novo e concessões novas. Reutilizar o cérebro existente; não ampliar H23, habilitar autonomia ou iniciar experimento científico/operacional novo por extensão de C2-P1. Nenhuma nova competência ou fechamento global é inferido da correção do classificador.
