# NEURONS
## Plano Executivo de Evolução Cognitiva, Metacognitiva e Pedagógica Contínua

**Identificador:** PEC-N — versão 0.1 (proposta para revisão)  
**Formato:** Markdown, documento de planejamento e rastreabilidade  
**Base conceitual:** *NEURONS — Fundamento de Formação e Desenvolvimento Cognitivo, Metacognitivo e Pedagógico Contínuo* (12 princípios)  
**Repositório:** `softstrucs/neurons`  
**Branch de trabalho:** `devs`  
**Último HEAD confirmado por relatório do Codex:** `1a313b3706d426143a70d208557f7625bfac68ac`  
**Plano preexistente:** `neurons_plano_executivo_v1_1.md` — preservado, sem substituição ou alteração  
**Estado documental:** PROPOSTA — requer revisão e aprovação explícita antes de adoção no repositório  
**Data de elaboração:** 2026-10-28

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
- **Demonstrado:** competência sustentada por testes independentes e critérios definidos antes da execução.
- **Hipótese:** proposta não confirmada.
- **Não demonstrado:** ausência de evidência suficiente, não necessariamente impossibilidade.

A recuperação literal de informação não prova compreensão. Acerto não prova generalização. Coatividade não prova contribuição causal. Expectativa tabular não prova aprendizagem neural distribuída. Comportamento metacognitivo programado não prova consciência subjetiva.

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
- H1, experimentos anteriores e o Plano Executivo v1.1 permanecem intactos salvo autorização expressa.

## 3. Inventário de situação inicial e histórico preservado

**Fonte de estado:** relatórios de execução apresentados pelo Codex e revisões desta conversa. O estado Git e os resultados devem ser reconfirmados pelo Codex ao iniciar trabalho novo.

| Identificador | Estado documentado | Resultado e limite |
|---|---|---|
| EC-00 | Auditoria conceitual realizada | Compatibilidade parcial do Fundamento com a infraestrutura; não constitui implementação integral. |
| EC-01 | Investigação analítica encerrada | H1 preservada, não implementada; associação induzida externamente não prova crédito causal. |
| EC-02.1 | Diagnóstico causal concebido | Contraste por bloqueio de sinapse, com condições necessária e redundante. |
| EC-02.2 | Protocolo operacional congelado | Dois cenários, intervenções e controle simbólico independente. |
| EC-02.3 | Implementada, testada e versionada | 40 reproduções, 118 testes aprovados, distinção local entre coatividade e dependência contrafactual; commit `1a313b3706d426143a70d208557f7625bfac68ac`. Não prova aprendizagem causal. |
| EC-02.4 | Investigação conceitual concluída | Proposta de aquisição, retenção, utilização e revisão de expectativa causal contextual; sem implementação. |
| EC-02.4-R1 | Protocolo experimental especificado | Protocolo congelado analiticamente; sem execução ou implementação. |
| EC-02.4-R2 | **Pendente, não autorizada neste documento** | Implementação experimental isolada proposta; exige aprovação explícita. |
| A03 e Etapa 12 API/web do plano v1.1 | Não autorizadas por este plano | Permanecem fora do escopo. |

### 3.1 Registro de referências

- Fundamento: documento de 12 princípios apresentado pelo responsável pelo projeto.
- EC-02.3: `tests/causal_diagnostic.rs`, `tests/support/causal_diagnostic.rs`, `tests/support/causal_symbolic.rs`.
- EC-02.4-R1: relatório analítico apresentado em 2026-10-28, com seções A–M e protocolo P0–P5. **Seu conteúdo integral deve ser preservado como anexo/protocolo separado antes da implementação**; este plano não o substitui nem altera.
- Plano preexistente: `neurons_plano_executivo_v1_1.md`.

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

**Situação:** investigação analítica encerrada; H1 preservada, não implementada.  
**Princípios:** 2, 4, 9, 11.

**Subetapas:** comparar coatividade, associação e dependência causal; manter hipóteses congeladas; documentar resultados negativos; decidir experimentos subsequentes sem alterar H1 retroativamente.

**Aceitação:** controles que impeçam atribuir aprendizagem a perturbação externa ou recompensa entregue sem crédito causal.

### EC-02 — Diagnóstico e aprendizagem de expectativas causais locais

**Situação:** EC-02.3 concluída e versionada; EC-02.4-R1 especificada; EC-02.4-R2 pendente.  
**Princípios:** 2, 4, 5, 8, 9, 11.

**Subetapas e ordem preservada:**

1. **EC-02.1:** definir contraste de dependência contrafactual local — concluída analiticamente.
2. **EC-02.2:** congelar intervenções, previsões e controles — concluída.
3. **EC-02.3:** implementar diagnóstico independente — concluída e versionada.
4. **EC-02.4:** investigar aquisição/utilização de informação causal — concluída analiticamente.
5. **EC-02.4-R1:** congelar protocolo de expectativa contextual — concluída analiticamente.
6. **EC-02.4-R2:** implementar e executar o protocolo isolado — **pendente de autorização**.
7. **EC-02.4-R3 (proposta):** auditoria independente dos resultados, controles, vazamentos e limites — não iniciada.
8. **EC-02.4-R4 (proposta):** consolidação documental e eventual versionamento, com autorização separada — não iniciada.

**Contrato mínimo de EC-02.4-R1, a preservar sem alterações:**

- Quatro neurônios e quatro sinapses, parâmetros do circuito EC-02.3.
- Aquisição por pares factual/bloqueio com observações brutas, sem rótulo causal fornecido ao componente.
- Estado de expectativa por chave opaca: `last_contrast`, `streak`, `confirmed`, contagem e proveniência; confirmação após dois contrastes consecutivos concordantes.
- Política programada: `confirmed=0` → permitir; `confirmed=1` → proteger; sem confirmação → investigar.
- 16 condições finais: quatro necessárias estáveis, quatro dispensáveis estáveis, quatro alteradas e quatro desconhecidas.
- Fases P0, P1, P2, P3, P4, P5; avaliação sempre antes de disponibilizar seu resultado à aquisição.
- Controles: sem atualização, informação indisponível, recuperação literal, sempre permitir, sempre proteger, sempre investigar, resumo simbólico independente e executor causal simbólico independente.
- Previsões do componente principal, por P0–P5: acertos operacionais `4,16,16,12,12,16` de 16; cobertura explícita `0,12,12,12,8,12` de 16; erros explícitos `0,0,0,4,0,0`; abstenções `16,4,4,4,8,4`.
- Por sequência: 136 pares; duas execuções independentes por par e dois executores (circuito e simbólico) → 544 reproduções; repetição integral → 1.088 reproduções.
- Tetos: 8 eventos por reprodução, 8.704 eventos agregados, 20 entradas de expectativa, 256 KiB de dados por sequência, RSS isolado <256 MiB, painel isolado ≤30 s, `/home` livre ≥3 GiB.
- Se houver discrepância entre este resumo e o protocolo original EC-02.4-R1, **o protocolo original prevalece e o Codex deve interromper para esclarecimento**.

**Aceitação de EC-02.4-R2:** previsões e controles reproduzidos; atualização somente por pares válidos; P3 registra erros após mudança; P4 suspende confirmação; P5 revisa; resultados e limitações documentados. Igualar ou perder para recuperação literal impede alegar superioridade ou aprendizagem de mecanismos causais gerais.

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
**Situação:** não iniciada / em análise / aprovada / implementada / validada / encerrada
**Autorização:** [registro explícito]
**Princípios:** [números]
**Dependências:** [identificadores e evidências]
**Hipótese ou questão:** [falsificável]
**Escopo permitido:** [arquivos e operações]
**Escopo proibido:** [alterações e afirmações não autorizadas]
**Protocolo congelado:** [arquivo, versão e hash, se aplicável]
**Sequência de trabalho:** [passos enumerados]
**Controles:** [baselines, ablações, comparações independentes]
**Métricas e previsões:** [critérios quantitativos]
**Recursos e interrupção:** [limites]
**Entregáveis:** [código, testes, relatório]
**Critérios de aceitação:** [evidência necessária]
**Limitações científicas:** [o que não prova]
**Estado Git e versionamento:** [branch, HEAD, alterações, autorização]
**Decisão de avanço:** [aprovação humana posterior]
```

## 8. Fluxo operacional de uma etapa

**P1 — Auditoria somente leitura** → **P2 — Hipótese e protocolo congelados** → **P3 — Implementação isolada autorizada** → **P4 — Testes e controles** → **P5 — Revisão científica** → **P6 — Versionamento especificamente autorizado** → **P7 — Registro no plano e autorização da etapa seguinte**.

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

### 9.1 Registro inicial

| Item | Estado |
|---|---|
| PEC-N v0.1 | Elaborado para revisão; **não incorporado ao repositório** |
| Fundamento de 12 princípios | Referência conceitual; adoção formal ao plano v1.1 não presumida |
| EC-02.3 | Versionada no commit informado `1a313b3706d426143a70d208557f7625bfac68ac` |
| EC-02.4-R1 | Protocolo analítico apresentado, sem implementação |
| EC-02.4-R2 | Aguardando autorização específica |
| Próximo ato recomendado | Revisar/aprovar este plano, preservar protocolo EC-02.4-R1 integralmente e somente depois autorizar implementação |

## 10. Instrução inicial de adoção para Codex CLI — somente após aprovação humana

```text
CODEX — NEURONS — ADOÇÃO DOCUMENTAL DO PEC-N v0.1

Objetivo: incorporar ao repositório, como novo documento Markdown,
o Plano Executivo de Evolução Cognitiva, Metacognitiva e Pedagógica
Contínua aprovado pelo responsável pelo projeto.

1. Conferir branch devs, HEAD, árvore limpa e documentos existentes.
2. Receber o conteúdo integral aprovado de
   neurons_plano_executivo_evolucao_cognitiva_v0_1.md.
3. Verificar nomes/numeração para evitar conflito com o Plano v1.1.
4. Criar exclusivamente o documento autorizado no local acordado.
5. Preservar integralmente o Fundamento e o protocolo EC-02.4-R1
   como referências separadas; não reescrevê-los nem substituí-los.
6. Não modificar código, testes, APIs ou outros documentos.
7. Executar verificações documentais e git diff --check.
8. Relatar arquivo criado, diferenças, HEAD e estado Git.
9. Não executar commit, push, merge nem EC-02.4-R2.
10. Aguardar revisão e autorização separada de versionamento.

Interromper se houver divergência de referência, arquivos inesperados
ou necessidade de alterar documento preexistente.
```

## 11. Critérios de qualidade e conservação

- O plano é um **índice de execução**, não substitui relatórios, protocolos completos, arquivos-fonte ou commits.
- Os protocolos congelados devem ser guardados integralmente com versão e, quando possível, hash antes de executar experimentos.
- Cada resultado experimental deve referenciar seu protocolo exato e o commit de implementação.
- Evidências negativas, limitações e hipóteses rejeitadas são parte permanente do histórico.
- A incorporação ao repositório e qualquer atualização do plano dependem de autorização humana.
- O plano deve permanecer útil mesmo se uma hipótese científica for falsificada.

---

**Encerramento da versão 0.1:** documento preparado para revisão. Nenhuma nova etapa foi implementada ou autorizada por sua elaboração.
