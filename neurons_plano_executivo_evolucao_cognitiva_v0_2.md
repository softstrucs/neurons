# NEURONS
## Plano Executivo de Evolução Cognitiva, Metacognitiva e Pedagógica Contínua

**Identificador:** PEC-N — versão 0.2 (proposta documental revisada, pendente de aprovação)

**Formato:** Markdown, documento de planejamento e rastreabilidade

**Base conceitual:** *NEURONS — Fundamento de Formação e Desenvolvimento Cognitivo, Metacognitivo e Pedagógico Contínuo* (12 princípios)

**Repositório:** `softstrucs/neurons`

**Branch de trabalho:** `devs`

**HEAD confirmado na revisão EC-DOC-01:** `dcb3c6a85b606a3bb128643e4e6afc65057fd7b2`

**Plano preexistente:** `neurons_plano_executivo_v1_1.md` — preservado, sem substituição ou alteração

**Estado documental:** PRESENTE LOCALMENTE / PROPOSTA — criação autorizada em EC-DOC-01; conteúdo ainda não aprovado como referência operacional e não versionado nesta revisão

**Data da proposta v0.1:** 2026-10-28

**Data desta revisão v0.2:** 2026-10-29

**Origem preservada:** `neurons_plano_executivo_evolucao_cognitiva_v0_1.md`, sem modificações

> **Diretriz central:** “O Neurons não deverá depender de nascer conhecendo o mundo. Deverá possuir e desenvolver mecanismos verificáveis que lhe permitam aprender progressivamente sobre ele.”

---

## 1. Finalidade e limites

Este plano transforma o Fundamento em uma trajetória **cronológica, auditável, incremental e delegável ao Codex CLI**. Cada etapa tem hipótese, dependências, tarefas delimitadas, evidências exigidas, critérios de aceitação e um ponto de decisão humano.

O Fundamento é uma **diretriz conceitual**, não uma arquitetura computacional demonstrada. Este plano também não constitui prova de que suas capacidades futuras serão alcançadas. Resultados negativos podem exigir revisão da ordem, divisão de etapas ou abandono de hipóteses.

**Aprovar este documento não equivale a autorizar a implementação de todas as etapas.** Cada subetapa exige autorização específica. Não há autorização implícita para alterar o plano v1.1, modificar contratos públicos, realizar commit/push/merge ou iniciar serviços, APIs ou interfaces web.

### 1.1 Vocabulário obrigatório de evidências

- **Implementado:** código ou mecanismo existente, identificado e verificado.
- **Observado:** comportamento efetivamente registrado em teste ou experimento.
- **Adquirido:** estado interno modificado por experiências, com proveniência e controles.
- **Demonstrado no protocolo:** competência sustentada somente no escopo das condições testadas, com critérios definidos antes da execução. Declarar separadamente independência de implementação, de dados, de avaliador e de equipe; uma não implica as demais.
- **Hipótese:** proposta não confirmada.
- **Não demonstrado:** ausência de evidência suficiente, não necessariamente impossibilidade.

A recuperação literal de informação não prova compreensão. Acerto não prova generalização. Coatividade não prova contribuição causal. Expectativa tabular não prova aprendizagem neural distribuída. Comportamento metacognitivo programado não prova consciência subjetiva.

### 1.2 Presença, aprovação e versionamento

- **Presente:** arquivo disponível no diretório local; pode estar não rastreado.
- **Aprovado:** conteúdo explicitamente aceito pelo responsável para o uso declarado; presença ou commit não substituem essa decisão.
- **Versionado:** conteúdo incluído em commit identificado; push e sincronização são verificações separadas.
- Autorizar a criação/revisão de um documento não aprova automaticamente seu conteúdo nem autoriza sua adoção, versionamento ou execução de etapas.

## 2. Governança de execução com Codex CLI

1. **Uma subetapa por autorização.** O Codex lê este plano e os protocolos referenciados, identifica o escopo autorizado e não executa etapas subsequentes por conta própria.
2. **Auditoria antes de alteração.** Conferir branch, HEAD, árvore de trabalho, arquivos relevantes, testes existentes, recursos e contratos.
3. **Protocolo antes de experimento.** Congelar hipótese, entradas, intervenções, controles, previsões, métricas, limites e critérios de interrupção antes da execução final.
4. **Separação de responsabilidades.** O componente experimental não acessa gabaritos, rótulos ocultos, resultados futuros ou configurações reservadas ao avaliador.
5. **Sem ajustes oportunistas.** Divergência com protocolo congelado é registrada como falha, inconclusão ou solicitação de revisão; não se retoca o experimento para obter êxito.
6. **Isolamento inicial.** Preferir testes e suportes experimentais; mudanças no núcleo de produção exigem etapa e autorização próprias, com justificativa.
7. **Sem versionamento automático.** Commit, push e merge somente com autorização posterior e específica.
8. **Relatório obrigatório.** Registrar arquivos, comandos, resultados observados versus previstos, controles, métricas, recursos, limitações, estado Git e recomendação.
9. **Portão de avanço.** A próxima subetapa depende de revisão humana explícita; não presumir aprovação por silêncio.
10. **Rastreabilidade.** Toda afirmação de capacidade deve apontar para relatório, protocolo, teste e commit correspondentes.

### 2.1 Restrições operacionais atuais

- Preferir Rust e biblioteca padrão, execução local, determinística e recursos modestos.
- Não introduzir GPU, serviços, frameworks, banco de dados, Docker ou infraestrutura distribuída sem justificativa e autorização.
- Preservar `/home` com pelo menos **3 GiB livres**; para painéis experimentais atuais, **RSS isolado inferior a 256 MiB**.
- Limites adicionais de tempo, eventos e armazenamento são específicos de cada protocolo; não extrapolar limites da EC-02.4-R1 para outras etapas.
- EC-01/H1, experimentos anteriores e o Plano Executivo v1.1 permanecem intactos salvo autorização expressa.

### 2.2 Nomenclatura qualificada

- `E001/H1`: hipótese de propagação de eventos nas Etapas 02–03 do Plano Executivo v1.1.
- `EC-01/H1`: protocolo exploratório analisado em EC-01-R1/R2; preservado e não implementado. Não é E001/H1.
- `EC-XX`: série experimental deste roteiro; não equivale às Etapas 00–12 do v1.1 nem às séries A01/A02/A03.
- `EC-12`: consolidação científica proposta; a **Etapa 12 do v1.1 — API/web permanece ADIADA**.
- `G1–G7`: fases de governança (§8). `P0–P5`: exclusivamente os pontos de avaliação de EC-02.4-R1/R2.

### 2.3 Resultados negativos e dependências

Uma execução válida com resultado negativo pode encerrar uma hipótese e satisfazer um entregável de diagnóstico; não satisfaz automaticamente uma competência exigida por outra etapa. Falha técnica, ausência de ganho e ausência de evidência devem ser classificadas separadamente.

Antes de avançar, identificar se cada dependência exige: (a) documento disponível; (b) mecanismo existente; ou (c) competência efetivamente demonstrada em condições explicitadas. Se uma competência necessária não foi demonstrada, o avanço correspondente fica bloqueado até revisão e autorização. Uma auditoria de representação não exige presumir que a aprendizagem geral já funciona.

Os bloqueios de elegibilidade da A02.5/R1 e as limitações dos controles da A02-F continuam evidências relevantes; não são eliminados pelo resultado positivo de expectativa simbólica de EC-02.4-R2. Revisar ordem ou dependências requer decisão humana registrada, preservando resultados anteriores.

## 3. Inventário de situação inicial e histórico preservado

**Fonte de estado:** relatórios de execução apresentados pelo Codex e revisões desta conversa. O estado Git e os resultados devem ser reconfirmados pelo Codex ao iniciar trabalho novo.

| Identificador | Estado documentado | Resultado e limite |
|---|---|---|
| EC-00.1 e EC-00.1-R1 | Auditorias realizadas e complementação documental encerrada na sessão | Matriz dos 12 princípios e compatibilidade parcial; lacunas computacionais permanecem. Relatórios integrais ainda não arquivados no repositório. |
| EC-01, EC-01-R1 e EC-01-R2 | Investigação e especificação analíticas encerradas | EC-01/H1 não implementada; parecer APTO COM RESTRIÇÕES para associação, sem identificação de atribuição causal de crédito. |
| EC-02.1 | Diagnóstico causal concebido | Contraste por bloqueio de sinapse, com condição necessária e condição com via alternativa suficiente. |
| EC-02.2 | Protocolo operacional congelado | Dois cenários, intervenções e controle simbólico independente. |
| EC-02.3 | Implementada, testada e versionada | 40 reproduções, 118 testes aprovados, distinção local entre coatividade e dependência contrafactual; commit `1a313b3706d426143a70d208557f7625bfac68ac`. Não prova aprendizagem causal. |
| EC-02.4 | Investigação conceitual concluída | Proposta de aquisição, retenção, utilização e revisão de expectativa causal contextual; sem implementação. |
| EC-02.4-R1 | Protocolo analítico preservado | Especificação implementada e executada pela EC-02.4-R2; seu texto integral permanece pendente de arquivamento no repositório. |
| EC-02.4-R2 | **Concluída e versionada** | Quatro arquivos experimentais, 121 testes aprovados no relatório validado, 1.088 reproduções e 3.808 eventos; commit `dcb3c6a85b606a3bb128643e4e6afc65057fd7b2`. Não autoriza novas execuções ou alterações. |
| A03 e Etapa 12 API/web do plano v1.1 | Não autorizadas por este plano | Permanecem fora do escopo. |

### 3.1 Registro de referências existentes

Documentos locais: `README.md`, `docs/decisions.md`, `neurons_plano_executivo_v1_1.md`, proposta v0.1 preservada e esta proposta v0.2. Na revisão, somente os três primeiros estavam versionados; v0.1 já estava presente e não rastreada. O estado de v0.2 deverá ser novamente conferido após qualquer aprovação/versionamento específico.

Referências externas lidas e disponíveis no ambiente desta revisão:

- `/home/aguiar/Documentos/Docs/Neurons_Fundamento_Cognitivo_Metacognitivo_Pedagogico_Aprovado.md`: texto dos 12 princípios, diretriz conceitual; não presume incorporação formal ao Plano v1.1.
- `/home/aguiar/Documentos/Docs/Neurons_Codex_EC-00.1-R1_Complementacao.md`: instrução de complementação da auditoria; não é o relatório de resultados.

Esses caminhos são locais, externos ao clone e não garantem disponibilidade em outra máquina ou sessão. O PDF `Neurons_Principio_Arquitetural_Cognitivo_Metacognitivo.pdf` foi localizado no mesmo diretório externo, mas não foi usado como fonte nem teve seu conteúdo validado nesta revisão.

### 3.2 Índice de rastreabilidade documental

**PENDENTE** significa texto integral não arquivado no repositório; não significa que a atividade não ocorreu. Relatórios disponíveis nesta conversa não devem ser confundidos com arquivos versionados.

| Série / atividade | Evidência existente e localizável | Situação dos documentos integrais |
|---|---|---|
| Fundamento | Markdown externo indicado em §3.1; matriz EC-00.1-R1 nesta sessão | PENDENTE: referência durável no repositório, sob autorização documental própria |
| EC-00.1 e EC-00.1-R1 | Relatórios desta sessão; infraestrutura em `src/`; instrução externa de complementação | PENDENTE: arquivamento integral dos relatórios; não inventar commit de auditoria |
| EC-01, EC-01-R1 e EC-01-R2 | Relatórios desta sessão; análise de EC-01/H1, sem implementação | PENDENTE: textos integrais da especificação, protocolo e parecer |
| EC-02.1 e EC-02.2 | Relatórios desta sessão; implementação do diagnóstico em EC-02.3 | PENDENTE: proposta e protocolo integrais, não substituídos pelo teste |
| EC-02.3 | `tests/causal_diagnostic.rs`, `tests/support/causal_diagnostic.rs`, `tests/support/causal_symbolic.rs`; commit `1a313b3706d426143a70d208557f7625bfac68ac` | Código versionado; PENDENTE: relatório integral da execução em arquivo |
| EC-02.4 e EC-02.4-R1 | Investigação e protocolo P0–P5 nesta sessão; implementação R2 correspondente | PENDENTE: arquivamento integral dos dois textos |
| EC-02.4-R2 | `tests/causal_expectation.rs`, `tests/support/expectation.rs`, `tests/support/expectation_symbolic.rs`, `tests/support/expectation_locked.rs`; commit `dcb3c6a85b606a3bb128643e4e6afc65057fd7b2` | Concluída e versionada; PENDENTE: relatório integral validado em arquivo |
| A01.1–A01.4, incluindo A01.3-R1 | `docs/decisions.md`, seções A01; `tests/languages*.rs`, `tests/concepts.rs`, `tests/contextual*.rs`, `tests/sentences*.rs` e suportes correspondentes | Registros resumidos versionados; PENDENTE: auditoria A01-F integral e relatórios/protocolos integrais não arquivados |
| A02.1–A02.5, incluindo revisões | `docs/decisions.md`, seções A02; `tests/a02_1_*.rs`, `tests/shared_feedback.rs`, `tests/context_encoding.rs`, `tests/local_plasticity.rs`, `tests/reward_learning.rs`, `tests/activity_selection.rs`, `tests/activity_r1.rs` e suportes | Registros e testes versionados; PENDENTE: relatórios/protocolos integrais não arquivados |
| A02-F | Auditoria científica consolidada nesta sessão; referências a testes e resultados A02 | PENDENTE: arquivamento integral; não há commit próprio de auditoria identificado |
| Auditoria documental da v0.1 | Relatório desta sessão, antecedente da revisão EC-DOC-01 | PENDENTE: relatório integral em arquivo; esta revisão não o substitui |

Âncoras Git verificadas: A01.1 `4e922ce27bfa5a88b97b9a9c6501aa7304ac9482`; A01.2 `2c7bee1f2e6e278b577b17b2d634416dc4c6a47f`; A01.3/R1 `7b0762e939c2a27a49e205b7569a27ab2bd17ab8`; A01.4 `18e86ecd490c90856009e5d82e622531e3ad1889`. A02.1/R1 `b52da15368ed84b3d3601a7099706bf6d7b25aa9`; A02.2 `c00ceff017dabd40da8c897fd43edca9d2589ebc`; A02.3/R1 `26004df8e65d5049a9b3eafdb162718c6ccb9e1b`; A02.4 `09e2175316eff3b119eb521de1074e52f28751bc`; A02.5/R1 `7a9443e9727666d15d54c6b0b9da875e3c0bd973`.

### 3.3 Conservação e limites das referências

O plano é índice, não reconstituição dos protocolos. Não completar textos ausentes a partir de resumos ou do código. Se uma nova tarefa depender de documento integral indisponível, informar a ausência e interromper a parte dependente antes de implementar.

O protocolo EC-02.4-R1 integral está na sessão e prevalece sobre qualquer resumo. Seu arquivamento permanece PENDENTE; arquivar futuramente não constitui prova retroativa de pré-registro independente. Nenhum protocolo ausente é criado por esta revisão.

## 4. Mapa cronológico de etapas e portões

**Regra de numeração:** a série `EC-*` designa a evolução cognitiva experimental, distinta das etapas 00–12 do plano v1.1. Os identificadores EC-00 a EC-02.4 já utilizados são preservados; a numeração futura abaixo é **proposta**, sujeita a aprovação.

| Etapa | Objetivo | Dependência principal | Entregável mínimo | Portão de avanço |
|---|---|---|---|---|
| EC-00 — Diagnóstico de base | Mapear capacidades e lacunas | Código e Fundamento | Auditoria de evidências | Limitações explícitas |
| EC-01 — Mecanismos elementares | Separar associação, ação e crédito | EC-00 | Hipóteses e controles | Não confundir coatividade com causa |
| EC-02 — Informação causal local | Diagnosticar, adquirir e usar expectativas | EC-01 | Painéis contrafactuais e de aquisição | Evidência de atualização e utilização, com limites |
| EC-03 — Representação e memória revisável | Investigar unidades e proveniência | EC-02 e inventário de memória | Comparação de representações | Recuperação, revisão e escopo verificáveis |
| EC-04 — Formação de conceitos | Investigar categorias e relações adquiridas | EC-03 | Experimentos de formação conceitual | Controles contra simples memorização |
| EC-05 — Aprendizagem contextual | Reconhecer validade, exceções e mudança | EC-03/04 | Testes de contexto e revisão | Robustez contextual demonstrada |
| EC-06 — Raciocínio fundamentado | Compor inferências e verificar evidências | EC-04/05 | Tarefas e controles independentes | Inferência não reduzida a respostas cadastradas |
| EC-07 — Investigação e questionamento | Identificar lacunas e selecionar testes | EC-02/05/06 | Experimentos de investigação | Benefício causal de buscar evidências |
| EC-08 — Monitoramento metacognitivo | Monitorar incerteza, erro e estratégias | EC-05/07 | Métricas de monitoramento | Regulação útil e calibrada |
| EC-09 — Formação pedagógica contínua | Diagnóstico, currículo e progressão | EC-04–08 | Ciclo pedagógico experimental | Progressão por competência, não volume |
| EC-10 — Transferência e cumulatividade | Testar situações e domínios novos | EC-06/09 | Avaliações fora da aquisição | Transferência acima de controles |
| EC-11 — Integração e robustez | Integrar capacidades demonstradas | EC-02–10 | Painel longitudinal integrado | Retenção, segurança, reprodutibilidade |
| EC-12 — Consolidação científica | Documentar maturidade e lacunas | EC-11 | Relatório de capacidades e limites | Decisão humana de continuidade |

A ordem é uma **hipótese de engenharia**, não uma afirmação de sequência necessária do desenvolvimento cognitivo. Dependências podem ser revistas após auditoria, sem reescrever resultados anteriores.

## 5. Especificação das etapas

### EC-00 — Diagnóstico de infraestrutura e Fundamento

**Situação:** auditoria realizada; preservar registros.

**Princípios:** 1, 2, 9, 11, 12.

**Subetapas:** inventariar módulos e testes; mapear cada princípio a evidências; registrar lacunas, restrições e riscos; aprovar linha de base.

**Aceitação:** evidências localizáveis, distinção entre implementado e hipotético, nenhuma capacidade presumida.

### EC-01 — Aprendizagem elementar e causalidade da ação

**Situação:** investigação analítica encerrada; EC-01/H1 preservada, não implementada.

**Princípios:** 2, 4, 9, 11.

**Subetapas:** comparar coatividade, associação e dependência causal; manter hipóteses congeladas; documentar resultados negativos; decidir experimentos subsequentes sem alterar EC-01/H1 retroativamente.

**Aceitação:** distinguir atividade induzida, associação por recompensa e atribuição causal de crédito; controles devem sustentar somente a conclusão efetivamente testada. Ausência de crédito causal identificado não deve ser convertida em impossibilidade de aprendizagem associativa delimitada.

### EC-02 — Diagnóstico e aprendizagem de expectativas causais locais

**Situação:** EC-02.3 e EC-02.4-R2 concluídas e versionadas; EC-02.4-R1 preservada como especificação analítica implementada por R2.

**Princípios:** 2, 4, 5, 8, 9, 11.

**Subetapas e ordem preservada:**

1. **EC-02.1:** definir contraste de dependência contrafactual local — concluída analiticamente.
2. **EC-02.2:** congelar intervenções, previsões e controles — concluída.
3. **EC-02.3:** implementar diagnóstico independente — concluída e versionada.
4. **EC-02.4:** investigar aquisição/utilização de informação causal — concluída analiticamente.
5. **EC-02.4-R1:** congelar protocolo de expectativa contextual — concluída analiticamente.
6. **EC-02.4-R2:** protocolo implementado, executado e versionado no commit `dcb3c6a85b606a3bb128643e4e6afc65057fd7b2` — **concluída; não reimplementar por orientação deste plano**.
7. **EC-02.4-R3 (proposta):** auditoria independente dos resultados, controles, vazamentos e limites — não iniciada.
8. **EC-02.4-R4 (proposta):** eventual consolidação documental posterior à auditoria, sob autorização própria — não iniciada. O código R2 já foi versionado; esta proposta não constitui novo versionamento pendente dessa implementação.

**Contrato mínimo de EC-02.4-R1, a preservar sem alterações:**

- Quatro neurônios e quatro sinapses, parâmetros do circuito EC-02.3.
- Aquisição por pares factual/bloqueio com observações brutas, sem rótulo causal fornecido ao componente.
- Estado de expectativa por chave opaca: `last_contrast`, `streak`, `confirmed`, contagem e proveniência; confirmação após dois contrastes consecutivos concordantes.
- Política programada: `confirmed=0` → permitir; `confirmed=1` → proteger; sem confirmação → investigar.
- 16 condições finais: quatro necessárias estáveis, quatro dispensáveis estáveis, quatro alteradas e quatro desconhecidas.
- Pontos de avaliação P0–P5: todas as decisões são registradas antes de produzir seus pares avaliativos. Observações de avaliação **nunca atualizam o aprendiz**, nem posteriormente; somente pares explicitamente destinados à aquisição alteram o resumo. Os controles recebem as mesmas observações permitidas, com as intervenções declaradas.
- Controles: sem atualização, informação indisponível, recuperação literal, sempre permitir, sempre proteger, sempre investigar, resumo simbólico independente e executor causal simbólico independente.
- Previsões do componente principal, por P0–P5: acertos operacionais `4,16,16,12,12,16` de 16; cobertura explícita `0,12,12,12,8,12` de 16; erros explícitos `0,0,0,4,0,0`; abstenções `16,4,4,4,8,4`.
- Por sequência: 136 pares; duas execuções independentes por par e dois executores (circuito e simbólico) → 544 reproduções; repetição integral → 1.088 reproduções.
- Tetos: 8 eventos por reprodução, 8.704 eventos agregados, 20 entradas de expectativa, 256 KiB de dados por sequência, RSS isolado <256 MiB, painel isolado ≤30 s, `/home` livre ≥3 GiB.
- Se houver discrepância entre este resumo e o protocolo original EC-02.4-R1, **o protocolo original prevalece e o Codex deve interromper para esclarecimento**.

**Critérios aplicados à EC-02.4-R2 concluída:** previsões e controles reproduzidos; atualização somente por pares válidos; P3 registra erros após mudança; P4 suspende confirmação; P5 revisa; resultados e limitações documentados. Igualar ou perder para recuperação literal impede alegar superioridade ou aprendizagem de mecanismos causais gerais.

### Registro de execução concluída EC-02.4-R2

Foram criados quatro arquivos: `tests/causal_expectation.rs`, `tests/support/expectation.rs`, `tests/support/expectation_symbolic.rs`, `tests/support/expectation_locked.rs`. Commit de implementação: `dcb3c6a85b606a3bb128643e4e6afc65057fd7b2`.

O relatório validado registrou **121 testes aprovados**, preservando os 118 anteriores, e painel integral de **1.088 reproduções e 3.808 eventos observados**, abaixo do teto de 8.704. Esses resultados históricos não foram reexecutados na revisão documental. Não confundir previsão, teto e medição.

| Avaliação | Correção operacional | Previsões explícitas corretas | Erros explícitos | Abstenções |
|---|---|---|---|---|
| P0 — estado vazio | 4/16 | 0/0 | 0 | 16 |
| P1 — aquisição | 16/16 | 12/12 | 0 | 4 |
| P2 — retenção | 16/16 | 12/12 | 0 | 4 |
| P3 — ambiente alterado | 12/16 | 8/12 | 4 | 4 |
| P4 — primeira contradição | 12/16 | 8/8 | 0 | 8 |
| P5 — segunda evidência | 16/16 | 12/12 | 0 | 4 |

Correção operacional inclui investigar nas quatro condições desconhecidas; essas abstenções não são previsões causais corretas. P3 preservou duas permissões indevidas e duas proteções desnecessárias. Os resumos foram retidos após experiências intermediárias; a primeira evidência contrária suspendeu as quatro confirmações alteradas e a segunda confirmou a revisão.

A **recuperação literal superou o resumo em P4**: 16/16 operacionalmente corretas e 12 previsões corretas, enquanto o resumo ficou em 12/16 e oito previsões corretas. Nos demais pontos, os resultados coincidiram. Sem atualização, informação indisponível e sempre investigar: 4/16 e 16 abstenções em todos os pontos. Sempre permitir/proteger: 6/16 na correção operacional, com erros próprios. Resumo e executor simbólicos independentes reproduziram os registros; a repetição integral coincidiu.

O relatório registrou 162.612 bytes na contabilização conservadora por sequência, RSS isolado reportado pelo lançador de 11.312 KiB e aproximadamente 0,0203 s, sujeitos às limitações de medição e harness. Os limites pertencem ao protocolo original, não são garantias para etapas futuras.

O resultado sustenta aquisição e utilização de expectativa causal local por regras programadas. Não demonstra superioridade sobre memória episódica, descoberta de um mecanismo causal geral ou aprendizagem neural. Uma implementação correta de política determinística não encerra a avaliação científica.

**Não demonstra:** autonomia de escolha de intervenções, generalização a circuitos desconhecidos, aprendizagem neural distribuída, compreensão causal geral ou consciência.

### EC-03 — Representações de experiência, evidência e conhecimento

**Situação:** proposta, não iniciada.

**Princípios:** 2, 4, 8, 9.

**Subetapas:** EC-03.1 auditoria das representações atuais; EC-03.2 comparação entre episódio, contraste, expectativa, relação, competência e representação distribuída; EC-03.3 protocolo de proveniência, validade e revisão; EC-03.4 protótipo isolado se aprovado; EC-03.5 controles e auditoria.

**Aceitação:** preservar contexto, versão, evidência e revisão; demonstrar recuperação e atualização sem perda silenciosa; não impor antecipadamente uma “unidade universal de conhecimento”.

### EC-04 — Formação progressiva de conceitos e relações

**Situação:** proposta.

**Princípios:** 1, 2, 4, 7, 8.

**Subetapas:** auditoria de relações existentes; definição de tarefas com exemplos não cadastrados; protocolo com controles de memorização; implementação isolada; avaliação de discriminação, composição e exceções.

**Aceitação:** evidência de aquisição e aplicação em condições controladas; não confundir associação lexical ou regras codificadas com compreensão.

### EC-05 — Contexto, validade e revisão

**Situação:** proposta.

**Princípios:** 4, 5, 8, 9.

**Subetapas:** modelar escopo de afirmações; testar mudança ambiental e conflito; definir política de revisão e abstenção; avaliar retenção sem misturar versões de contexto.

**Aceitação:** conclusões revisadas mediante evidência, identificação de exceções e ausência de extrapolação indevida.

### EC-06 — Raciocínio com evidências

**Situação:** proposta.

**Princípios:** 1, 4, 7, 8, 9.

**Subetapas:** auditar inferência existente; definir tarefas composicionais; separar conhecimento fornecido e adquirido; testar raciocínio com evidências, controles e problemas não cadastrados.

**Aceitação:** justificativas rastreáveis e resultados em avaliações independentes, com reconhecimento de incerteza.

### EC-07 — Questionamento e investigação orientada

**Situação:** proposta.

**Princípios:** 3, 5, 6, 9.

**Subetapas:** detectar lacunas; formular hipóteses operacionais; selecionar intervenções sob orçamento; comparar investigação ativa com seleção aleatória e política fixa; medir ganho informacional e de competência.

**Aceitação:** demonstração de que a escolha de investigar contribui para aprendizagem posterior, sem depender de perguntas em linguagem natural.

### EC-08 — Metacognição e autorregulação

**Situação:** proposta.

**Princípios:** 5, 6, 9, 11.

**Subetapas:** auditar monitores existentes; definir estimativas de incerteza e erro; testar decisões de revisar, investigar e abster-se; medir calibração e custo-benefício.

**Aceitação:** monitoramento útil sobre processos efetivos, não apenas mensagens ou estados predefinidos.

### EC-09 — Formação pedagógica contínua

**Situação:** proposta.

**Princípios:** 1, 3, 6, 7, 11, 12.

**Subetapas:** diagnosticar competências; mapear pré-requisitos; selecionar experiências; acompanhar prática, avaliação, revisão e consolidação; permitir progressão somente por competência demonstrada.

**Aceitação:** melhora sustentada em ciclos sucessivos e adaptação verificável das experiências às lacunas identificadas.

### EC-10 — Generalização, transferência e cumulatividade

**Situação:** proposta.

**Princípios:** 1, 3, 4, 7, 8, 9, 11.

**Subetapas:** definir novos contextos e tarefas; estabelecer controles de renomeação e recuperação; avaliar transferência genuína, retenção longitudinal e interferência.

**Aceitação:** desempenho acima de controles adequados em situações não usadas na aquisição; relatar fracassos e limites de domínio.

### EC-11 — Integração longitudinal e confiabilidade

**Situação:** proposta.

**Princípios:** 1–9, 11, 12.

**Subetapas:** mapear interfaces e dependências; integrar apenas mecanismos validados; executar testes de regressão, recursos, rastreabilidade, segurança, retenção e autonomia limitada.

**Aceitação:** resultados reproduzíveis e limites explicitados; integração não pode transformar hipóteses em capacidades demonstradas.

### EC-12 — Consolidação, especialização futura e questões abertas

**Situação:** proposta.

**Princípios:** 7, 9, 10, 11, 12.

**Subetapas:** inventariar competências demonstradas; avaliar lacunas; definir pré-requisitos de especialização; registrar questões abertas, incluindo consciência artificial como investigação distinta e não pré-requisito.

**Aceitação:** relatório científico e decisão explícita sobre próximas linhas de pesquisa, sem alegar experiência subjetiva.

## 6. Mapa dos 12 princípios para etapas

| Princípio | Etapas que o investigam principalmente |
|---|---|
| 1. Objetivo fundamental | EC-00, EC-04, EC-06, EC-09–12 |
| 2. Infraestrutura inicial | EC-00–04 |
| 3. Formação pedagógica contínua | EC-07, EC-09, EC-10 |
| 4. Aprendizagem como transformação | EC-01–06, EC-10 |
| 5. Investigação e questionamento | EC-02, EC-05, EC-07, EC-08 |
| 6. Cognição e metacognição integradas | EC-07–09, EC-11 |
| 7. Fundamentos à especialização | EC-04, EC-06, EC-09, EC-10, EC-12 |
| 8. Compreensão contextual | EC-02–06, EC-10 |
| 9. Confiabilidade científica | Todas as etapas |
| 10. Consciência artificial | EC-12 como questão aberta; sem pressupor demonstração |
| 11. Evolução progressiva | Todas as etapas e portões de aprovação |
| 12. Diretriz central | Orientação transversal do plano |

## 7. Modelo obrigatório de subetapa para delegação

```markdown
### EC-XX.Y — [Título]
**Situação:** não iniciada / em análise / implementada / encerrada; registrar aprovação, validação no protocolo e versionamento em campos separados
**Autorização:** [registro explícito]
**Princípios:** [números]
**Dependências:** [identificadores, tipo documental/mecanismo/competência, evidências disponíveis e pendências que bloqueiam a tarefa]
**Hipótese ou questão:** [falsificável]
**Escopo permitido:** [arquivos e operações]
**Escopo proibido:** [alterações e afirmações não autorizadas]
**Protocolo congelado:** [texto integral acessível, localização verificada, versão e hash quando existente; ausência impeditiva deve ser relatada, nunca suprida por invenção]
**Sequência de trabalho:** [passos enumerados]
**Controles:** [baselines, ablações, comparações independentes]
**Métricas e previsões:** [critérios quantitativos]
**Recursos e interrupção:** [limites]
**Entregáveis:** [código, testes, relatório]
**Critérios de aceitação:** [evidência necessária]
**Limitações científicas:** [o que não prova]
**Estado Git e versionamento:** [branch, HEAD, alterações, autorização]
**Decisão de avanço:** [aprovação humana posterior; resultado negativo não satisfaz competência não demonstrada]
```

O modelo acima deve ser integralmente preenchido e revisado antes de qualquer implementação futura. Objetivos e aceites qualitativos das etapas EC-03–EC-12 não são protocolos executáveis. Congelar estados iniciais, dados/partições, sequência, fronteira do avaliador, intervenções, controles, métricas com denominadores, regras de decisão/atualização, orçamento e critérios de sucesso, falha, inconclusão e interrupção. Não deixar decisões científicas para improvisação durante implementação. Auditorias sem implementação devem declarar o escopo de leitura e seus critérios próprios de conclusão.

## 8. Fluxo operacional de uma etapa

**G1 — Auditoria somente leitura** → **G2 — Hipótese e protocolo congelados** → **G3 — Implementação isolada autorizada** → **G4 — Testes e controles** → **G5 — Revisão científica** → **G6 — Versionamento especificamente autorizado** → **G7 — Registro documental autorizado e decisão sobre continuidade**.

G1–G7 são fases de governança, distintas das avaliações P0–P5. Atualizar o plano e autorizar a próxima etapa são atos separados; G7 não concede autorização automática.

Nem toda etapa exige código. Falha experimental não autoriza correção retroativa de protocolo. Um resultado negativo documentado pode encerrar legitimamente uma hipótese.

## 9. Registro de decisões e mudanças

Toda atualização deste plano deve conter:

| Campo | Preenchimento obrigatório |
|---|---|
| Identificador da decisão | `DEC-AAAA-NNN` |
| Data e responsável | Data e aprovação humana |
| Documento afetado | Nome e versão |
| Mudança proposta | Descrição precisa |
| Motivo e evidência | Relatórios, testes e limitações |
| Impacto | Etapas, dependências, contratos e recursos |
| Decisão | Aprovada / rejeitada / pendente |
| Referência Git | Commit, se houver e autorizado |

### 9.1 Estado na revisão v0.2

| Item | Estado |
|---|---|
| PEC-N v0.1 | Presente localmente, não rastreado na verificação inicial; preservado sem alteração. Aprovação operacional não presumida |
| PEC-N v0.2 | Criado sob autorização EC-DOC-01; proposta para revisão, ainda não aprovada como referência operacional nem versionada nesta atividade |
| Fundamento de 12 princípios | Referência conceitual externa; incorporação formal ao Plano v1.1 não presumida |
| EC-02.3 | Concluída e versionada em `1a313b3706d426143a70d208557f7625bfac68ac` |
| EC-02.4-R1 | Especificação preservada; implementada por R2; arquivamento integral PENDENTE |
| EC-02.4-R2 | Concluída e versionada em `dcb3c6a85b606a3bb128643e4e6afc65057fd7b2` |
| Próxima ação recomendada | Revisão e aprovação documental explícita da v0.2; eventual versionamento exige autorização separada. Não executar EC-03 |

### 9.2 Registro desta revisão documental

- **Atividade autorizada:** EC-DOC-01, revisão documental em 2026-10-29, solicitada pelo responsável nesta sessão.
- **Documento criado:** somente `neurons_plano_executivo_evolucao_cognitiva_v0_2.md`.
- **Motivo:** corrigir desatualização histórica, isolamento de avaliações, nomenclatura, pendências documentais e critérios de delegação identificados na auditoria da v0.1.
- **Preservação:** v0.1, Plano v1.1, código, testes, EC-01/H1 e protocolos anteriores não são modificados.
- **Decisão sobre o conteúdo:** PENDENTE de revisão/aprovação. A autorização de redação não é aprovação do plano. Identificador formal DEC, se exigido para adoção, ainda não atribuído pelo responsável; não presumir aprovação.
- **Referência Git da base:** `dcb3c6a85b606a3bb128643e4e6afc65057fd7b2`; nenhum commit desta revisão está autorizado ou informado.

## 10. Instrução de revisão e adoção documental — atos separados

O arquivo v0.2 já está presente por autorização de criação. A próxima ação é revisá-lo. Não criá-lo novamente nem sobrescrevê-lo como se estivesse ausente.

```text
CODEX — NEURONS — REVISÃO DOCUMENTAL DO PEC-N v0.2

Executar somente se especificamente autorizado pelo responsável.

1. Conferir devs, HEAD, estado Git e documentos existentes.
2. Ler neurons_plano_executivo_evolucao_cognitiva_v0_2.md e as
   referências integrais disponíveis; identificar as PENDENTES.
3. Reconhecer v0.1 e v0.2 não rastreados como pendências esperadas
   somente quando assim confirmados pela autorização; preservar ambos.
4. Conferir histórico, nomenclatura e separação aquisição/avaliação.
5. Relatar prontidão e correções propostas sem editar, salvo autorização
   explícita para edição de arquivo identificado.
6. Não modificar código, testes, protocolos ou Plano v1.1.
7. Não reimplementar EC-02.4-R2: concluída e versionada.
8. Não arquivar textos integrais pendentes sem autorização própria;
   não inventar conteúdo, caminho ou hash.
9. Não executar commit, push, merge, EC-03 ou qualquer experimento.
10. Aguardar decisão humana sobre aprovação documental e eventual
    autorização separada de versionamento.

Se houver divergência de branch, referência ou arquivos não abrangidos
pela autorização, preservar o estado e relatar antes de qualquer escrita.
```

A adoção deve registrar conteúdo aprovado, data, responsável e escopo. Versionamento exige instrução específica com arquivos e mensagem de commit. A aprovação documental ou seu versionamento não autoriza execução de EC-03 nem de etapas posteriores.

## 11. Critérios de qualidade e conservação

- O plano é um **índice de execução**, não substitui relatórios, protocolos completos, arquivos-fonte ou commits.
- Os protocolos congelados devem ser guardados integralmente com versão e, quando possível, hash antes de executar experimentos.
- Cada resultado experimental deve referenciar seu protocolo exato e o commit de implementação.
- Evidências negativas, limitações e hipóteses rejeitadas são parte permanente do histórico.
- A incorporação ao repositório e qualquer atualização do plano dependem de autorização humana.
- O plano deve permanecer útil mesmo se uma hipótese científica for falsificada.

---

**Encerramento da versão 0.2:** revisão documental EC-DOC-01 preparada para análise e aprovação explícita. Nenhuma implementação ou nova etapa é autorizada pelo documento. Próximo ato: revisão documental da v0.2, sem execução de EC-03.
