# NEURONS — Plano Mestre

**Versão:** 1.0, atualização executiva F1 em 2026-11-08. **Estado desta revisão:** mandato explícito de implementação contínua MP01–MP04 e MP11 no recorte F1 executado; F1 formalmente APROVADA COM RESSALVAS pelo responsável em 08/11/2026, exclusivamente no recorte autorizado. Este continua sendo o único Plano Mestre executivo ativo. MP05–MP10, painéis científicos e acesso G2/G7/D/V/F não foram autorizados.

## 1. Visão, finalidade e invariantes

**A primeira meta de produto do NEURONS é entregar F1 — Sessão Cognitiva Local Limitada, funcional, reproduzível e verificável.** F0 é marco intermediário de estabilização; F2 é evolução cognitiva posterior. MP10 — agentes supervisionados — é opcional e não bloqueia F1.

Desenvolver incrementalmente um sistema local modular, orientado a eventos, com neurônios, sinapses e engramas artificiais que possa perceber entradas, preservar experiências, aprender sob evidência, raciocinar, investigar lacunas, regular suas decisões e comunicar-se por CLI/chat. Preservar a evolução intelectual e pedagógica, pt-BR inicial, retenção, revisão e futura coordenação de agentes sob supervisão. Arquitetura computacional inspirada em cérebro não é prova de fidelidade biológica, consciência ou inteligência geral.

Reutilizar um pacote Rust/CPU e contratos pequenos; interfaces são adaptadores do mesmo núcleo. Sem LLM como raciocínio oculto, segundo motor paralelo, GPU, DB, microsserviços, servidor ou agentes externos por conveniência. Cada dependência nova exige benefício/custo e aprovação específica; `Cargo.toml` atual não declara crates externas. Autonomia interna é finita e permissionada; autorização de planejar não é autorização de agir.

Separar: bytes presentes, implementação, compilação, teste funcional, resultado experimental, demonstração científica e autorização. Informação recebida, hipótese, observação admitida, inferência e conhecimento validado são estados distintos. Nenhuma consulta promove inferência, ID de evidência ou afirmação a fato confiável; reconhecimento de fonte não autentica verdade. Proveniência e efeitos precisam ser observáveis.

## 2. Fontes reconciliadas e precedência

Os seis documentos originais estão íntegros no [arquivo histórico](docs/historico/planos_executivos/README.md), com SHA-256 e [mapa de contexto](docs/historico/planos_executivos/LOCALIZACAO_ORIGINAL.json). Não foram considerados inúteis por idade e não são roteiros concorrentes ativos.

| Fonte | Conteúdo incorporado / divergência preservada |
|---|---|
| v1.1 | EDA/Rust/std/CPU, módulos pequenos, neurônios/sinapses/engramas, etapas00–12, CLI, limites e controles. Etapas já implementadas não são reinicializadas. |
| PEC-N v0.1 | Desenvolvimento progressivo,12 princípios, causalidade/memória/conceitos/investigação/pedagogia; EC-02.4-R2 aparece pendente nesse snapshot. |
| PEC-N v0.2 | Diferencia presença/aprovação/versionamento e isola avaliação de aquisição; registra EC-02.4-R2 concluída em dcb3c6a, sem transferir a conclusão a aprendizagem neural. EC03–12 propostas naquela data têm auditorias posteriores. |
| v0.3 proposta/parecer/aprovado condicionado | Sete frentes V03-01..07 e condicionantes C01–C05; aprovação documental é histórica, não autorização executiva. Preservar dependências específicas, contribuição causal, limites de persistência/permissões, retenção/transferência e critérios prospectivos. |
| Decisões, EC03–12, M0–M5, CIC-I1..RC-01 | Evidência mais específica de interfaces e resultados, inclusive AJUSTAR, inconclusões e bloqueios. Não reexecutar painéis para atualizar percentuais. |

O mandato específico vigente delimita o que pode ser feito; este plano aprovado futuramente só organiza a execução. Protocolos científicos e restrições de dados/permissões não são substituídos por este roteiro. Conflito entre resumo e protocolo integral bloqueia a parte dependente. Renumerações qualificadas evitam confundir etapas v1.1, séries EC/V03/CIC, governança G2/G7 e etapas MP deste plano. A arquitetura A é de atestação G2 offline segregada, não substituição da EDA do núcleo.

As regras antigas de parar a cada subetapa são reconciliadas com a autonomia solicitada: uma autorização futura poderá abranger um intervalo explícito de etapas MP e seus testes/limites. Dentro desse intervalo, rotinas reversíveis e avanço com gates satisfeitos dispensam microconfirmações. Fora dele, nenhuma aprovação implícita. Esta regra está incorporada nesta consolidação; não amplia autorizações históricas específicas ou acesso reservado.

## 3. Linha de base real e progresso consolidado

Base observada: repositório `softstrucs/neurons`, `devs`, HEAD5066a3745709d45df6bfd5129abc71acc660f955. A consulta de leitura ao GitHub na recuperação precedente encontrou o mesmo tip em devs:0 commits locais exclusivos e0 remotos exclusivos; não houve nova sincronização nesta consolidação. Isso não significa worktree igual ao remoto: cinco arquivos Rust modificados e centenas de não rastreados incluem partes essenciais M1–M5/CIC. Backup integral externo verificado precedeu toda reorganização. [Relatório da recuperação](docs/referencia/RECUPERACAO_ESTRUTURAL.md).

| Marco / estado | Implementação e evidência | Limite e próximo efeito |
|---|---|---|
| Fundação00–03 v1.1 | `event`, `engine`, `neuron`, `synapse`, `circuit`; fila determinística, soma saturada, propagação e orçamento. | Modelos operacionais simples; não cognição emergente. Não reimplementar bootstrap. |
| Memória/aprendizagem04–05 | EngramStore e PlasticityRule, recuperação exata e ajuste explícito por feedback externo; testes existentes. | Memória, ajuste de peso e generalização não são equivalentes; sem crédito causal autônomo. |
| Cognição/metacognição06–07 | TwoHopTask/Monitor; composição de dois passos e regras conservadoras, evidência/abstenção. | Inferência central auxiliar e controle simbólico explicam comportamento; não vantagem neural. |
| Interface08–09 | `session`, `personality`, `curriculum`, `languages`; gramática curta e vocabulário/configuração. | Sem português livre ou ciclo pedagógico autônomo. |
| Eficiência/persistência10–11 | Perfis pequenos e checkpoint Engine+EngramStore; IntegratedState recompõe Monitor externo. | Não serializa Circuit, pesos/estado neuronal ou toda política; não continuidade cognitiva integral. |
| EC-02..12 | Diagnósticos/suportes experimentais e auditorias; EC04/10 revisão simbólica, EC07/08 controles limitados, EC09 pedagógico documental. | EC03–12 encerramentos AJUSTAR não homologam competência geral. EC10 R3 final ignorado por quota20/28; não corrigir expectativa para passar. |
| M1–M4 | cognitive_contracts, QueryCoordinator, aquisição explícita, provas semânticas e controles separados. | Código local parcialmente não rastreado; não é instalável por clone remoto apenas. |
| M5 histórico | Relatório registra150 avaliações e28 verificações literais, duas repetições; logs brutos preservados. | Sucesso funcional restrito de aquisição simbólica assistida, não formação autônoma/neural. Não reexecutado aqui. |
| CIC-I1/I2 | ConceptCoordinator, H pública23 critérios, versões, exemplos, unanimidade/abstenção; instrumentos/evaluador segregados. | Evidência histórica de engenharia D; nenhuma execução atual de partição. Indução simbólica em família fornecida. |
| CIC-I3..RC-01 | Custódia/G2, auditoria e governança; consumidor apenas sintético; muitos artefatos são propostas. | V bloqueada em G2; F/painel864 não executados/autorizados. Docs aprovados com ressalvas não resolvem11 achados/33 alternativas/14 propostas/20 riscos/P7-RISK01. |
| Recuperação atual MP00 | Backup2.419 entradas, restauração8 amostras, seis planos arquivados em bytes idênticos e índices reorganizados. Compilação lib+CLI offline passou com target em /tmp. | Nenhum teste funcional ou painel reexecutado; Plano Mestre ainda proposto; sem commit/push. |

O total de testes históricos de épocas diferentes não é soma de evidências independentes. Na recuperação precedente foi executado `CARGO_TARGET_DIR=/tmp/neurons-recovery-cargo-check cargo check --offline --lib --bin neurons` para compilação; testes/painéis não foram executados. Não inferir cobertura comportamental dessa compilação.

### 3.1. Preservação durável verificada

O backup histórico original `/tmp/neurons-recovery-backup-8dt15y2x/neurons-integral.tar` foi preservado, sem substituição por um backup do estado atual. Cópia persistente fora do repositório e de diretórios temporários:

`/home/aguiar/Documentos/Trabalho/Softstrucs/Backups/neurons/recuperacao_historica_20261108T195533_431277Z/neurons-integral.tar`

Tamanho: **136069120 bytes**. SHA-256: `d7ff33e5a87d051ea942ab9028c4e4617089bd0ce550264b8aa7a479b9424eb4`. Backup, inventário e oito registros auxiliares de verificação/auditoria (dez arquivos no total) foram copiados com igualdade de hashes. Data e evidências em [PRESERVACAO_DURAVEL.json](docs/referencia/PRESERVACAO_DURAVEL.json). Diretório novo privado; permissões existentes não foram alteradas.

A cópia está em armazenamento persistente, mas `/tmp` e `/home` pertencem ao mesmo disco físico `nvme0n1`, em partições diferentes. **Não há segunda cópia independente verificada em outro dispositivo nesta atividade.** Isso não protege contra falha física do disco: recomenda-se uma segunda cópia verificada em dispositivo distinto ou armazenamento independente. Nenhuma cópia externa adicional foi criada ou presumida. Os documentos antigos continuam descrevendo corretamente a retenção temporária que existia naquela data.

### 3.2. Conjunto fonte e reprodução delimitada

O [manifesto técnico de reprodutibilidade](docs/referencia/REPRODUTIBILIDADE_FONTES.json) identifica **178 arquivos de fonte/configuração**:134 rastreados sem alteração, cinco rastreados modificados e39 essenciais não rastreados. Todos os hashes coincidem com os membros correspondentes do backup histórico agora durável, incluindo os módulos locais ausentes do commit5066a374. Portanto o commit, isoladamente, não representa o conjunto fonte atual. Nenhum arquivo foi adicionado ao índice ou enviado ao GitHub. O inventário efetivo desta consolidação contém1.419 arquivos:233 rastreados existentes (sete modificados),484 não rastreados e702 ignorados; seis origens rastreadas ausentes correspondem aos movimentos preservados da recuperação. São contagens atuais, não reaproveitamento de resumos anteriores.

Uma cópia isolada fora do repositório, com39 entradas de compilação (Cargo.toml/Cargo.lock e37 arquivos Rust de produção), passou em `cargo check --offline --locked --lib --bin neurons` e em `cargo build --offline --locked --lib --bin neurons`, com target temporário separado e toolchain instalado. Biblioteca e binário foram gerados, sem execução do binário. Cinco arquivos de testes internos não foram copiados para esse ensaio de compilação; testes, exemplos, scripts, protocolos e dados não foram executados nem usados como entradas de avaliação. O manifesto preserva a identificação e os hashes de todo o conjunto, distingue artefatos gerados e dependências documentais controladas e descreve como recuperar os membros do backup.

**Resultado demonstrado: integridade do conjunto fonte e reprodução da compilação básica lib/CLI no ambiente identificado.** Não demonstra testes funcionais aprovados, comportamento de sessão, reprodução integral de instrumentos científicos, portabilidade para qualquer plataforma ou conclusão de F1. Toolchain/SO não estão empacotados; dados/evidências restritos, catálogos e contexto confiado continuam sujeitos a seus escopos. Nenhum arquivo ignorado foi necessário para essa compilação; `target/` é reconstruível. Dependências dinâmicas dos testes/instrumentos e seus ambientes não foram exaustivamente exercitados.

**Ressalva de conformidade desta atividade:** uma inspeção estática exibiu trecho de teste interno identificado como fixtures D, fora do limite de acesso da consolidação. A leitura desse material foi interrompida. As fixtures não foram executadas, copiadas para a árvore isolada de compilação ou usadas para ajustar código/expectativas/critérios. Não se declara ausência total de acesso D; a ocorrência consta dos manifestos e deve ser considerada na revisão do próximo intervalo executivo. V/F e fluxos reais G2/G7 não foram executados ou liberados.

## 4. Arquitetura funcional a evoluir

| Responsabilidade | Módulos reais / fronteira |
|---|---|
| Percepção/entrada | `src/event.rs`, `src/session.rs`, `src/languages*.rs`: estímulos e parser estruturado. Sensores/percepção geral não existem como capacidade demonstrada. |
| Dinâmica neuronal | `engine.rs`, `circuit.rs`, `neuron.rs`, `synapse.rs`: eventos, estado/peso, limites e métricas; sem varredura global gratuita ou plasticidade implícita. |
| Memória/proveniência | `engram.rs`, `cognitive_contracts.rs`: fato, hipótese, fonte, evidência, contexto, validade e histórico. `Cue.source` não é origem epistemológica. |
| Raciocínio | `cognition.rs`, `integration/query.rs`: provas e autorização de regra; ampliar apenas com semântica explícita, não transitividade numérica de IDs. |
| Aquisição/revisão | `learning.rs`, `learning/concepts.rs`, `integration/acquisition.rs`, `integration/concepts.rs`: incorporação explícita e hipótese versionada. |
| Metacognição | `metacognition.rs`, `metacognition/concepts.rs`: conflito/insuficiência/abstenção e orçamento; não introspecção perfeita. |
| Integração/sessão | `integration.rs`, `session.rs`, `main.rs`: mesmo núcleo, adaptadores pequenos. CLI legada ainda não usa toda integração M2/M3/CIC. |
| Comunicação/pedagogia | `languages/`, `personality.rs`, `curriculum.rs`: estilo separado de veracidade, currículo futuro por competência real. |
| Estado persistente | `persistence.rs` é incluído como `engine::persistence`, reexportado em lib; manter caminho/API enquanto não autorizada mudança. |
| Avaliação | `protocol_verification/`, suportes em `tests/`, scripts G2: instrumentos separados do candidato; avaliador e gabaritos não entram nas APIs cognitivas. |
| Agentes futuros | Ainda sem framework operacional: só após permissões/contratos de mensagem/cancelamento/recursos específicos. Não criar nomes de papéis para contornar P2. |

Compilar sem crates externas é uma propriedade atual, não regra de que toda necessidade futura será resolvida sem bibliotecas. Nenhuma dependência foi demonstrada supérflua nesta auditoria. Otimizar somente gargalo medido.

## 5. Cronologia, dependências e entregas funcionais

MP00 foi esta recuperação e está concluído no alcance de preservação/reorganização, sem aprovação do plano. Na linha de base da consolidação, MP01–MP11 eram propostas não iniciadas; o registro executivo atual está na seção 9. Cada etapa entrega comportamento/testes; documentos são evidência associada, nunca motivo único para criar uma fase. Fluxo principal: MP01→MP02→MP03/MP04; melhorias de raciocínio e aprendizagem dependem do mecanismo específico, não da conclusão burocrática de toda pesquisa. MP10 agentes é opcional, isolado e nunca requisito para uma versão local útil. MP11 fecha um recorte funcional definido, não exige demonstrar inteligência geral.

### Sequência executiva prioritária para F1

Preservação durável e identificação/reprodução do conjunto fonte → **MP01** (base executável honesta e regressão segura) → **MP02** (sessão perceptiva e episódio integrado) → **MP03/MP04** (conceitos e persistência, respeitando dependências) → **MP11 no recorte F1** (integração, validação e entrega).

MP03 e MP04 admitem trabalhos independentes após MP02 quando não compartilham pré-requisitos ainda não satisfeitos. Persistir hipóteses conceituais depende da definição explícita do estado que a sessão F1 promete restaurar; esse contrato deve ser identificado antes da parte correspondente de MP04. Não se exige persistência de estado que F1 não promete, nem se omite silenciosamente estado prometido. **MP05–MP10 não são requisitos para concluir F1.** F0 estabiliza a base, não substitui a primeira entrega de produto. Cada avanço ainda exige intervalo executivo previamente autorizado; esta sequência não é essa autorização.

### MP01 — Base executável honesta e reprodutível

**Estado:** CONCLUÍDA NO RECORTE F1 — ACEITA COM RESSALVAS pelo responsável em 08/11/2026; entregas futuras além desse recorte não estão concluídas por extensão. **Autorização de execução:** mandato explícito MP01–MP04 e MP11, sem extensão a outras etapas. Evidências atuais na seção 9.

**Objetivo e pré-requisitos:** MP00 preservado; autorização explícita de manutenção funcional; dados sintéticos públicos independentes de D/V/F.

**Arquivos/módulos:** src/main.rs, src/lib.rs, tests/cli.rs e testes básicos já existentes; Cargo.toml apenas se necessidade demonstrada.

**Implementação necessária:** Corrigir doctor e seu teste para relatar os módulos/limites reais, preservar comandos e códigos de saída, consolidar uma lista de regressões seguras. Não reimplementar motor ou alterar critérios científicos.

**Testes e evidências:** cargo check offline lib+bin; testes CLI/engine/interactions/memory/learning/persistence previamente inspecionados para excluir painéis e partições. Saída de doctor não pode dizer que todo núcleo inexiste nem alegar capacidades gerais.

**Critério de conclusão:** Compilação e alvos autorizados passam; limites da sessão e erros de argumentos preservados; inventário do conjunto fonte completo, incluindo novos módulos locais.

**Avanço automático no intervalo autorizado:** Próxima etapa somente se já estiver no intervalo autorizado e a base ficar coerente.

**Intervenção humana / interrupção:** Mudança de API, dependência, falha de teste, referência a dado reservado, necessidade de commit ou painel científico.

### MP02 — Sessão perceptiva e episódio cognitivo integrado utilizável

**Estado:** CONCLUÍDA NO RECORTE F1 — ACEITA COM RESSALVAS pelo responsável em 08/11/2026; entregas futuras além desse recorte não estão concluídas por extensão. **Autorização de execução:** mandato explícito MP01–MP04 e MP11, sem extensão a outras etapas. Evidências atuais na seção 9.

**Objetivo e pré-requisitos:** MP01; escopo de CLI e exemplos sintéticos públicos aprovados; permissão de regra e fonte declaradas.

**Arquivos/módulos:** src/session.rs, integration/query.rs, integration/acquisition.rs, cognitive_contracts.rs, tests/cli.rs; nova cobertura específica de sessão.

**Implementação necessária:** Adaptar CLI ao coordenador existente: receber entrada tipada, contexto/proveniência, consultar, explicitar pedido e admitir feedback por ato separado. Exibir provas, decisões e histórico; nenhuma aquisição automática, acesso externo ou tabela de respostas.

**Testes e evidências:** Cadeia pública positiva, ausência de suporte, oposição, fonte inválida, K2 sem regra e consulta somente leitura. Retorno irrelevante admissível não suporta conclusão indevida.

**Critério de conclusão:** Uma sessão reproduzível mostra antes→pedido→retorno→admissão explícita→reavaliação, sem inserir conclusões inferidas. Gabarito externo não é recebido.

**Avanço automático no intervalo autorizado:** MP03 ou MP04 podem seguir conforme intervalo/pré-requisitos, sem micropermissões para rotinas já aprovadas.

**Intervenção humana / interrupção:** Nova política de investigação, fonte/rede real, quebra do contrato legado ou leitura de partição.

### MP03 — Formação e revisão conceitual limitada acessível na sessão

**Estado:** CONCLUÍDA NO RECORTE F1 — ACEITA COM RESSALVAS pelo responsável em 08/11/2026; entregas futuras além desse recorte não estão concluídas por extensão. **Autorização de execução:** mandato explícito MP01–MP04 e MP11, sem extensão a outras etapas. Evidências atuais na seção 9.

**Objetivo e pré-requisitos:** MP02 e mecanismo CIC-I1 existente; autorização de desenvolvimento sintético separado dos dados D/V/F reais.

**Arquivos/módulos:** cognition/concepts.rs, learning/concepts.rs, metacognition/concepts.rs, integration/concepts.rs, session.rs.

**Implementação necessária:** Expor a versão e a revisão de hipótese com os mecanismos existentes, sem introduzir regra-alvo. Manter família pública H23 até uma questão experimental autorizada exigir mudança. Não refazer CIC-I1/I2 nem avaliador final.

**Testes e evidências:** Exemplos sintéticos independentes das partições, contraexemplo, unanimidade, observação incompleta, oposição, contexto e classe inadequada; verificar histórico/prefixo e não mutação na consulta.

**Critério de conclusão:** Usuário observa mudança de H explicada por experiência admissível, com evidência de eliminação e abstenção conservadora. Nenhuma alegação de compreensão/autonomia/neural.

**Avanço automático no intervalo autorizado:** Continua apenas nas etapas autorizadas, família e entradas dentro do escopo; resultado negativo preservado.

**Intervenção humana / interrupção:** Mudança científica de H/critério, acesso D/V/F, gabarito, painel864 ou aprendizagem neural presumida.

### MP04 — Persistência fiel do estado explicitamente suportado

**Estado:** CONCLUÍDA NO RECORTE F1 — ACEITA COM RESSALVAS pelo responsável em 08/11/2026; entregas futuras além desse recorte não estão concluídas por extensão. **Autorização de execução:** mandato explícito MP01–MP04 e MP11, sem extensão a outras etapas. Evidências atuais na seção 9.

**Objetivo e pré-requisitos:** MP02; MP03 se hipóteses conceituais forem parte do checkpoint requerido. Formato/escopo a decidir antes de alterar bytes.

**Arquivos/módulos:** src/persistence.rs, engine.rs, circuit.rs, integration.rs, integration/concepts.rs, testes de persistência.

**Implementação necessária:** Definir e implementar checkpoint versionado do subconjunto de sessão/circuito/memória/hipóteses realmente necessário, ou declarar exclusão explícita de cada estado. Para continuidade de pesos/neurônios, incluir tais estados em contrato aprovado; monitor não deve ser reconstruído com configuração implícita.

**Testes e evidências:** Round-trip de estado, fila, pesos/potenciais se incluídos, metadados/versões, falha/corrupção, versão antiga, capacidade e atomicidade. Testes em diretório temporário, sem segredos ou credenciais.

**Critério de conclusão:** Não perder estado prometido; restaurar com igualdade verificável e sem promover hipóteses. Formato antigo tratado explicitamente, sem alegar continuidade que não persiste.

**Avanço automático no intervalo autorizado:** Pode encaminhar o recorte F1 a MP11 quando MP01–04 e controles da sessão forem aceitos no escopo; MP04 isoladamente não encerra a entrega F1.

**Intervenção humana / interrupção:** Migração incompatível de API/formato sem autorização, perda silenciosa, persistência de dado reservado, requisito de criptografia real.

### MP05 — Aprendizagem adaptativa com contribuição mecânica identificável

**Estado:** IMPLEMENTADO E VALIDADO somente no intervalo MP05 de adaptação sináptica assistida, auditável e persistente. MP05 integral e F2 não estão concluídas. **Autorização:** mandato específico do responsável para implementação/testes deste recorte; sem commit/push ou outras etapas.

**Objetivo e pré-requisitos:** MP01/MP04 conforme retenção requerida; tarefa e controles prospectivos aprovados para qualquer alegação experimental.

**Arquivos/módulos:** learning.rs, neuron.rs, synapse.rs, circuit.rs e testes sintéticos delimitados, sem substituir o motor.

**Implementação necessária:** Evoluir uma regra explícita reversível e rastrear o que mudou por experiência. Separar atualização sináptica, engrama, regra e hipótese; avaliar contribuição da rede contra controle simbólico com informação equivalente.

**Testes e evidências:** Sem atualização, sem sinapses/adaptação, retenção e contraexemplo em entradas públicas inéditas da tarefa autorizada; recursos medidos quando houver execução autorizada.

**Critério de conclusão:** Atualização/localização/proveniência observáveis e ausência de promoção; ganho só declarado se critério prospectivo e controle o sustentarem. Resultado nulo não vira aprendizagem neural geral.

**Avanço automático no intervalo autorizado:** Avanço por requisito mecânico/funcional satisfeito; competência não demonstrada bloqueia alegação dependente, não auditoria independente.

**Intervenção humana / interrupção:** Execução científica sem protocolo/mandato, ajuste oportunista, nova arquitetura/crate ou extrapolação de resultado simbólico.

### MP06 — Raciocínio fundamentado e regras contextuais extensíveis

**Estado:** IMPLEMENTADO E VALIDADO exclusivamente no primeiro intervalo F2/MP06: inclusão com até três premissas. MP06 além desse recorte e F2 integral não estão concluídas. **Autorização:** mandato explícito do responsável para este intervalo, sem commit/push e sem MP05/MP07–MP10.

**Objetivo e pré-requisitos:** MP02; MP03 somente para raciocínio sobre conceitos adquiridos. Sem depender de vantagem neural ainda ausente.

**Arquivos/módulos:** cognition.rs, integration/query.rs, metacognition.rs e contratos tipados.

**Implementação necessária:** Adicionar apenas formas de composição realmente necessárias, mantendo TwoHopTask quando bastar. Cada regra deve ter semântica, versão, premissas, polaridade, validade e autorização explícitas; consultar não grava conclusões como fatos.

**Testes e evidências:** Provas desconectadas/adulteradas, regra fora de contexto, fatos negativos/expirados e orçamento; controle simbólico independente e literal em tarefa autorizada.

**Critério de conclusão:** Conclusão tem prova verificável ou motivo tipado de abstenção; mesma entrada produz mesmo resultado, sem IDs com significado oculto.

**Avanço automático no intervalo autorizado:** Prossegue se próxima atividade já autorizada e cada dependência específica comprovada.

**Intervenção humana / interrupção:** Nova semântica sem decisão, caminho sem premissas, acesso ao oráculo, gasto ilimitado ou alteração de expectativa científica antiga.

### MP07 — Investigação limitada que realmente produz evidência

**Estado:** IMPLEMENTADO E VALIDADO somente no primeiro intervalo MP07 de investigação supervisionada de uma lacuna relacional em fonte local. MP07 integral e F2 não estão concluídas. **Autorização:** mandato específico de implementação/testes, sem commit/push e sem ampliação de outros intervalos.

**Objetivo e pré-requisitos:** MP02/MP06 e mecanismo de atualização pertinente MP03/MP05; fontes sintéticas previamente autorizadas.

**Arquivos/módulos:** integration/acquisition.rs, cognition.rs, metacognition.rs, episódios e adaptador de sessão.

**Implementação necessária:** Selecionar pedido/intervenção finita com motivo de lacuna e registrar escolha, execução autorizada, retorno, incorporação e efeito. Evitar simulador de investigação que apenas imprime intenção.

**Testes e evidências:** Ausência/irrelevância/oposição de retorno, limites, uma ação sem mandato recusada; comparar política fixa/literal/aleatória apenas com protocolo apropriado.

**Critério de conclusão:** Traço causal separa decisão de investigar, ação efetuada e mudança de estado; benefício medido somente quando comparável e sustentado.

**Avanço automático no intervalo autorizado:** Sem novo prompt para rotinas de fonte/ação já expressamente delimitadas, mantendo contadores e parada.

**Intervenção humana / interrupção:** Rede, ferramenta ou ação externa não autorizada; novos retries abertos; dados reservados ou fonte confiada por ID.

### MP08 — Metacognição útil e comunicação honesta

**Estado:** IMPLEMENTADO E VALIDADO exclusivamente no primeiro intervalo MP08: supervisão determinística de resposta, investigação única, abstenção ou encerramento por limite. MP08 integral e F2 não estão concluídas. **Autorização:** implementação/testes deste recorte; sem commit/push ou outras etapas.

**Objetivo e pré-requisitos:** MP02/MP06; MP07 se regulando investigação real.

**Arquivos/módulos:** metacognition.rs, metacognition/concepts.rs, personality.rs, session.rs.

**Implementação necessária:** Regular resposta, revisão e abstenção sobre evidência acessível, separar estado deliberativo/avaliabilidade/execução quando pertinente sem aprovar política G2. Estilo nunca altera verdade ou autorização.

**Testes e evidências:** Casos positivos, contraditórios, insuficientes e custos; falsos alarmes/falhas de detecção; controle com mesma informação. Calibração probabilística apenas se houver probabilidades justificadas.

**Critério de conclusão:** Decisão e motivo corretos no escopo definido; utilidade incremental não presumida. Sem alegação de autoinspeção perfeita ou consciência.

**Avanço automático no intervalo autorizado:** Prossegue dentro do recorte autorizado sem elevar a capacidade fora do protocolo.

**Intervenção humana / interrupção:** Falta de referência independente, confiança estatística inventada, risco crítico oculto ou opinião persuasiva substituindo evidência.

### MP09 — Pedagogia progressiva e linguagem pt-BR delimitada

**Estado:** IMPLEMENTADO E VALIDADO somente no primeiro intervalo MP09: microepisódio conceitual supervisionado com progresso/controle e retenção F1 v1. MP09 integral, linguagem geral e F2 integral permanecem não concluídas. **Autorização:** implementação/testes deste recorte, sem commit/push ou MP10.

**Objetivo e pré-requisitos:** MP03/MP04/MP08 e aquisição/raciocínio aplicáveis; currículo real não se presume de enum Domain.

**Arquivos/módulos:** curriculum.rs, languages.rs, languages/contextual.rs, languages/sentences.rs, session.rs.

**Implementação necessária:** Conectar diagnóstico→experiência→feedback→revisão→retenção com o mesmo estado. Ampliar interpretação/produção textual restrita por tarefas e medir fenômenos; domínios de linguagem/lógica/ciência/computação são experiências, não quatro redes independentes.

**Testes e evidências:** Pré/pós e experiências intermediárias, retenção/interferência, itens inéditos não isomorfos quando alegada transferência, controles de memorização e conhecimento inicial equivalente.

**Critério de conclusão:** Progresso funcional verificável e limitações linguísticas claras; melhorar quantidade de exemplos não prova generalização. Documentar separadamente transferência positiva/nula/negativa.

**Avanço automático no intervalo autorizado:** Avança por competência específica sustentada; tarefa nova que não dependa de competência ausente pode ser autorizada separadamente.

**Intervenção humana / interrupção:** Dados/corpus reservado, currículo alimentado por gabarito final, alteração de denominador ou pretensão de português geral.

### MP10 — Agentes locais supervisionados, somente se necessários

**Estado:** proposto, não iniciado. **Autorização de execução:** ausente nesta atividade.

**Objetivo e pré-requisitos:** Recurso concreto e decisão humana específica; MP04/MP08 e contratos de mensagem, identidade, cancelamento, orçamento e autorização demonstráveis no recorte.

**Arquivos/módulos:** Adaptadores novos mínimos sobre integration/session; reutilizar memória e regras. Sem microserviços ou segundo núcleo.

**Implementação necessária:** Implementar delegação local com responsabilidades/efeitos explícitos apenas após demonstrar utilidade; um ator não pode elevar privilégios ou compartilhar gabarito/segredo. Não mexer em papéis/zonas/permissões P2.

**Testes e evidências:** Negativas de acesso/delegação, cadeia de responsabilidade, cancelamento, limites, repetição e resultado incorreto; controle monoagente equivalente.

**Critério de conclusão:** Toda ação/delegação rastreável e revogável no escopo; agentes não são pré-requisito de F1 e não demonstram autonomia geral.

**Avanço automático no intervalo autorizado:** Jamais iniciar só porque MP09 terminou: requer autorização específica que já inclua agentes e limites; depois rotinas internas autorizadas são autônomas.

**Intervenção humana / interrupção:** Qualquer privilégio indefinido, infraestrutura/credencial/rede real, acesso experimental reservado ou contorno de deny.

### MP11 — Entrega local utilizável, escalável e reproduzível

**Estado:** CONCLUÍDA NO RECORTE F1 — ACEITA COM RESSALVAS pelo responsável em 08/11/2026; entregas futuras além desse recorte não estão concluídas por extensão. **Autorização de execução:** mandato explícito MP01–MP04 e MP11, sem extensão a outras etapas. Evidências atuais na seção 9.

**Objetivo e pré-requisitos:** Recorte de versão definido; a prioridade executiva inicial é F1 após MP01–MP04. F0 é estabilização intermediária e F2 posterior; não exige MP10 nem consciência/AGI.

**Arquivos/módulos:** main/session/integration, persistência, testes CLI e regressão inspecionada, examples/cpu_profile.rs se benchmarking autorizado.

**Implementação necessária:** Fechar pacote funcional com mesma biblioteca em CLI/chat textual, instalação local documentada, entradas inválidas/limites/retomada, medições CPU/disco/RSS reais no perfil aprovado e gargalos específicos corrigidos se autorizados. API/web é adaptador opcional futuro sob decisão própria.

**Testes e evidências:** Reprodução em cópia isolada com conjunto fonte completo, testes do recorte, cenários de uso e falha, checkpoints se prometidos, custo por evento/atualização e memória com instrumento real.

**Critério de conclusão:** Entrega corresponde exatamente aos comandos/capacidades prometidos, com erros honestos e resultados brutos suficientes. Nenhum ignorado/Fail/NotEvaluable conta como sucesso; limites de escala declarados.

**Avanço automático no intervalo autorizado:** Termina o escopo de versão e aguarda liberação humana para publicação/versionamento; nenhum push/commit automático.

**Intervenção humana / interrupção:** Dependência ausente na cópia limpa, regressão falha, consumo fora de orçamento, requisito operacional não autorizado ou API pública alterada sem decisão.

## 6. Contrato de autonomia do Codex CLI

Cada autorização futura deve identificar os campos abaixo. A ausência de mandato para implementação continua sendo bloqueio real, não motivo para inventar outra rodada documental.

| Campo da autorização executiva | Conteúdo necessário |
|---|---|
| Revisão do Plano Mestre | Versão e referência/hash do documento utilizado. |
| Intervalo MP autorizado | Etapas incluídas/excluídas e dependências; para F1 pode ser MP01–MP04 e MP11 no recorte F1, sem incluir MP05–MP10. |
| Objetivo e versão | Resultado funcional pretendido, prioritariamente F1, e seus limites. |
| Arquivos e APIs | Módulos abrangidos, compatibilidade a preservar e exclusões explícitas. |
| Testes e dados | Alvos, fontes públicas/sintéticas permitidas e painéis/partições proibidos. |
| Recursos | Orçamentos de tempo/eventos/memória/disco e instrumento para verificar o que for exigido. |
| Ações externas | Lista de ações permitidas/proibidas, sem herança de autorização. |
| Avanço automático | Gates e ordem de dependências dentro do intervalo; nenhum salto de requisito. |
| Interrupção | Falhas não resolvidas, dependências ausentes, incompatibilidade de API/arquitetura, acesso reservado, dependência externa nova ou ação irreversível. |
| Entregáveis e aceite | Código/estado/resultado verificável e critérios objetivos da entrega, sem alegações além da evidência. |

Dentro do intervalo previamente autorizado, o Codex poderá ler, implementar, editar reversivelmente, executar os testes permitidos, diagnosticar e corrigir defeitos **dentro do escopo**, registrar resultados e avançar pela ordem das dependências sem novos prompts a cada subetapa. Não exigir árvore limpa nem sobrescrever alterações preexistentes. Não solicitar novamente permissão para rotina já abrangida. Trabalhos independentes podem ser ordenados conforme suas dependências, sem confundir isso com autorização para novas frentes.

Falhas corrigíveis dentro do escopo podem ser tratadas e reverificadas; **falhas não resolvidas** impedem avanço. As referências a falha/teste nos itens MP têm esse alcance: não exigem microautorização para corrigir o defeito já abrangido. Interromper a parte dependente ao encontrar pré-requisito crítico ausente, divergência de protocolo, necessidade de API incompatível ou arquitetura não aprovada, orçamento insuficiente, acesso reservado, nova dependência externa ou ação irreversível não autorizada. Não alterar critérios para fazer o resultado passar, nem repetir painel científico sem mandato. Resultado negativo/inconclusivo preserva o registro e bloqueia somente as conclusões que dependem dele.

**A aprovação do Plano Mestre não constitui autorização automática para executar MP01–MP11.** A ausência de resposta humana não é aprovação. Ao fim do intervalo, ou em bloqueio real fora do que ele permite resolver, apresentar resultado/impedimento e aguardar decisão explícita. Commit, push, novos serviços, chaves reais e acesso G2/G7/D/V/F permanecem atos separados. Esta consolidação não autoriza MP01 nem corrige doctor.

Padrão conservador de recursos do planejamento inicial: guardar ≥3 GiB livres em /home; preferir target/trabalho regular≤1 GiB e processos de teste comuns≤256 MiB RSS quando mensurável. Parar/reavaliar se orçamento não couber; não afirmar conformidade sem instrumento. Valores de protocolos específicos prevalecem em seus escopos e não são reescritos por esse orçamento. Ambiente desta recuperação: rustc/cargo1.88.0, /home≈5,8 GiB livres e /tmp≈15 GiB em leitura pontual, sem garantia contínua.

Nenhum `cargo test` indiscriminado: alguns testes/suportes históricos contêm painéis/dados/protocolos que não são smoke tests comuns. Inspecionar alvos, excluir V/F/D reservados e painéis científicos, registrar ignorados. Não reexecutar EC-10 R3, M0/M5,864 classificações, controles de infraestrutura/systemd/cgroup/D/V/F ou homologação sem autorização específica. Compilar biblioteca/CLI não executa painéis; teste unitário aprovado não é resultado científico geral.

## 7. Limites de custódia e governança preservados

G2_real=NotEvaluable; G7_reserved_access=NotAuthorized; reserved_DVF_access=NotAuthorized. CIC-I6/P8 não iniciado, não é etapa MP e não decorre deste plano. Permissões P2:378 células,46 condicionais e332 deny,22 fluxos; EA/Z4/verify linha195, CD/Z5/custody147, VI/Z5/custody255 e RP/Z4/approve34 continuam deny. Nada aqui altera papéis/zonas/fluxos nem gera chave/certificado/criptografia real.

As11 pendências P4/P5,33 alternativas P5,14 propostas P6,20 critérios/20 riscos P6 e P7-RISK01 mantêm seus estados. Propostas documentais de deliberação/avaliabilidade/execução RC-01 não são política operacional aprovada. Não se exige outra rodada de documentação G2 para corrigir um bug comum de CLI com fontes públicas, mas qualquer trabalho que realmente dependa do acesso bloqueado permanece interrompido. Aprovação de nova etapa funcional não libera partições por renomeá-las ou chamar fixtures reais de sintéticas.

## 8. Versões funcionais e critérios de conclusão

**F1 — Sessão Cognitiva Local Limitada é a primeira entrega funcional obrigatória.** F0 é marco intermediário; F2 é evolução posterior. MP10 continua opcional e não bloqueante. A compilação isolada registrada em§3.2 não é validação funcional.

- **F0 — Base honesta, marco intermediário de estabilização:** MP01 aceito no recorte; biblioteca/CLI compilam, comandos e erros correspondem à implementação, testes básicos autorizados passam, nenhum módulo essencial fica apenas na máquina do autor sem inventário recuperável. Não implica demonstração neural.
- **F1 — Sessão cognitiva local limitada:** MP01–04 e controles da sessão aceitos; receber entrada pública, recuperar evidência, inferir/abster-se, admitir explicitamente experiência, revisar hipótese e restaurar o estado prometido, com provas/histórico/limites. Sem rede/serviço/LLM oculto, gabarito ou promoção implícita. MP11 fecha entrega desse recorte.
- **F2 — Evolução cumulativa delimitada:** partes pertinentes MP05–09 aceitas sob protocolos próprios, retenção/transferência e contribuição de mecanismos avaliadas de forma independente; alegação final limitada a cada resultado. Agentes MP10 são extensão opcional supervisionada, nunca requisito artificial para entregar CLI local.

Uma versão funcional está concluída apenas com: conjunto fonte/configuração identificado; interfaces/limites claros; compilação e testes pertinentes executados com resultados reais; cenários de sucesso/falha e recuperação reproduzíveis; critérios de recursos efetivamente medidos quando exigidos; nenhum bloqueio crítico no recorte; dados/estado preservados; riscos e capacidades não demonstradas explícitos; decisão de aceite humano. Publicação/commit/push permanecem atos separados. Não exigir inteligência geral/consciência nem ocultar ausência de competência indispensável com média de acertos.

### 8.1. Aceitação obrigatória de F1

F1 somente pode ser concluída quando todos os requisitos pertinentes forem sustentados por evidência real, e não pela existência deste texto:

1. O conjunto completo de fontes estiver preservado e for reproduzível.
2. A biblioteca e a CLI compilarem no ambiente definido.
3. Os testes funcionais pertinentes forem executados e aprovados.
4. Uma sessão local permitir entrada, consulta e recuperação de evidências.
5. Inferências apresentarem justificativas verificáveis ou abstenção explícita.
6. A aquisição de novas informações exigir admissão autorizada e verificável.
7. Hipóteses conceituais puderem ser revisadas com histórico e evidências.
8. O estado prometido pela sessão puder ser salvo e restaurado fielmente, com contrato explícito do que está incluído/excluído.
9. Erros, limites, entradas inválidas e interrupções forem tratados.
10. A entrega puder ser reproduzida em uma cópia isolada, incluindo os cenários funcionais autorizados.
11. Nenhuma capacidade não demonstrada for apresentada como comprovada.
12. O responsável aprovar formalmente o resultado funcional.

MP11 consolida esse aceite no recorte F1, sem exigir MP05–MP10 ou promover dados/estados reservados. Critérios científicos históricos permanecem intactos; nenhum denominador, expectativa ou protocolo congelado é ajustado para produzir resultado favorável. A consolidação anterior verificou apenas integridade e compilação. A execução F1 atual acrescentou testes e cenários públicos reproduzíveis, detalhados na seção 9. **Critério 12: aceite humano registrado em 08/11/2026 — APROVADO COM RESSALVAS. O aceite limita-se a F1 e não constitui homologação científica.**

### 8.2. Contrato de conclusão da F2 — Evolução Cognitiva Integrada

**Formalização documental em 2026-11-13; F2 NÃO CONCLUÍDA.** Este contrato delimita a entrega F2 sem reescrever a definição, o aceite ou as seis ressalvas históricas da F1. A designação cumulativa da seção 8 permanece histórica; partes pertinentes MP06–MP09 são avaliadas pelo comportamento requerido abaixo. O recorte MP05 já registrado mantém sua evidência própria, mas sua integração às decisões cognitivas não é requisito F2. Nenhum resultado parcial equivale ao aceite integral.

**Objetivo de entrega:** uma sessão cognitiva local capaz de receber objetivos relacionais ou conceituais-compostos, consultar evidências, diagnosticar insuficiências, selecionar estratégias supervisionadas permitidas, realizar ensino ou investigação delimitados quando aplicáveis, reavaliar o objetivo e encerrar com resultado verificável, respeitando permissões, limites e persistência do conhecimento.

A classificação abaixo usa exclusivamente as demonstrações registradas na seção 9 e a inspeção estática dos contratos no HEAD `7c12e22fa74b1239858a467544008bf44e9a940f`. **Demonstrado** significa satisfeito no recorte público histórico identificado, sem nova execução; **parcial** significa componentes demonstrados, mas integração/validação da entrega ainda ausente; **pendente** significa sem demonstração ou aceite específico. Não transforma relato histórico em nova certificação.

| Nº | Critério verificável de conclusão | Estado e evidência disponível / pendência |
|---|---|---|
| 1 | Núcleo F1 preservado e regressões aprovadas. | Demonstrado no recorte histórico: aceite F1 e 63 regressões, incluídas na revalidação TCR-01 de 176 testes. Nova integração deverá preservar e revalidar essa base. |
| 2 | Consulta relacional com prova verificável ou abstenção. | Demonstrado: MP06, provas de até três premissas, oposição e abstenção; composição atual restringe suporte relacional a uma premissa. |
| 3 | Revisão conceitual supervisionada com histórico e versões. | Demonstrado: MP09, supervisão conceitual e TCR-01; sem promoção da hipótese a verdade. |
| 4 | Investigação delimitada com autorização independente. | Demonstrado: MP07/MP08, permissões distintas de investigar, fonte e admissão; não implica investigação automática do objetivo composto. |
| 5 | Diagnóstico metacognitivo baseado em relatórios reais. | Parcial: MP08 e diagnóstico pedagógico existentes; falta diagnóstico e decisão integrados do objetivo composto. |
| 6 | Consulta composta com vínculo e evidência relacional independente. | Demonstrado: consulta composta e TCR-01, vínculo sintético explícito, dependências e conferência; sem autenticação de identidade real. |
| 7 | Supervisão de objetivo composto com seleção de estratégia, no máximo uma experiência autorizada por episódio e reconsulta do mesmo objetivo. | Pendente: há supervisão de objetivo apenas conceitual e operações compostas manuais; ainda não há supervisor composto integrado. Uma operação pedagógica histórica com duas admissões não satisfaz o limite de uma experiência deste contrato. |
| 8 | Interface mínima integrada na sessão, sem duplicar lógica cognitiva. | Parcial: sessão F1 e comandos MP07–MP09 existentes; composição acessível pela API Rust, sem interface integrada de supervisão composta. |
| 9 | Permissões explícitas, orçamento limitado e interrupção conservadora. | Parcial: contratos individuais demonstrados; falta validar a contabilidade e as recusas no supervisor composto completo. |
| 10 | Ausência de promoção automática de classificações ou composições a fatos. | Demonstrado nos controles MP09/composição/TCR-01; deverá permanecer invariante no novo fluxo. |
| 11 | Persistência fiel do conhecimento prometido, sem restauração automática de autoridade ou supervisores transitórios. | Demonstrado no contrato F1 v1 e extensões transitórias: restauração em novo processo com novas concessões obrigatórias para a composição; preservar as exclusões e autorizações sintéticas base F1 historicamente persistidas. |
| 12 | Testes públicos, controles negativos e avaliação independente. | Parcial: 176 testes históricos, referências e controles TCR-01; matriz da supervisão composta ainda não executada. |
| 13 | Reprodução funcional em ambiente isolado. | Parcial: F1 e intervalos anteriores reproduzidos; falta reprodução da entrega integrada F2. Não presumir durabilidade dos logs temporários nem suficiência universal do clone. |
| 14 | Declarações científicas limitadas às evidências demonstradas. | Demonstrado como limite documental: resultados sintéticos finitos, TCR-01 inválido anterior preservado, sem extrapolação neural/geral; obrigação contínua. |
| 15 | Aceite formal específico da F2 pelo responsável. | Pendente: aceite F1 não transfere aprovação à F2; requer decisão humana específica após apresentação da entrega e pendências. |

**Condição de fechamento:** satisfazer todos os critérios pertinentes na entrega integrada com evidências públicas rastreáveis, controles negativos e reprodução; registrar limitações e aceite F2 específico. Critérios demonstrados continuam sujeitos à regressão quando a integração mudar. Não somar execuções repetidas como evidências independentes. Publicar permanece ato separado.

**Limites expressos:** F2 não exige AGI ou consciência; aprendizagem geral ou irrestrita; linguagem natural livre; planejamento aberto de longo prazo; percepção multimodal geral; autonomia operacional irrestrita; integração obrigatória do MP05 às decisões cognitivas; implementação do MP10; persistência de supervisores ativos entre processos. Esses temas dependem de pesquisa, necessidade demonstrada e autorizações futuras. Não constituem lacunas artificiais para impedir a entrega delimitada, nem capacidades presumidas.

### 8.3. Especificação prospectiva — F2 — Supervisão Limitada de Objetivo Composto

**Estado: PREPARADO, NÃO IMPLEMENTADO NEM VALIDADO.** Nome provisório do próximo intervalo; sua execução exige autorização futura. O mandato atual permite somente editar este Plano Mestre e inspecionar contratos. Não cria política geral de agentes, não amplia autonomia operacional e não introduz conhecimento factual automaticamente.

**Objetivo:** integrar os contratos existentes MP08, MP09 e MP06 para acompanhar um único objetivo composto, desde a consulta inicial até a decisão final, na mesma Session F1 e no mesmo Episode composto. Investigação relacional MP07/MP08 permanece uma capacidade independente já demonstrada; neste próximo intervalo, relação ausente gera abstenção, sem investigar, ensinar para compensar a ausência ou conceder acesso a fonte.

#### 8.3.1. Entrada, sequência e resultados

Entrada fixa antes da consulta: ID do objetivo, Binding completo (item, observação, conceito, entidade relacional, destino, Scope e proveniência), SyntheticRegistry e revisão, Permissions de composição, PedagogyPermissions separadas, orçamento, plano didático previamente fixado e avaliações independentes pré/pós sem rótulos. O objetivo e seu vínculo ficam imutáveis; os rótulos de correção permanecem exclusivamente no avaliador externo. O item objetivo não pode aparecer como ensino ou avaliação pedagógica independente.

1. Validar identidade, vínculo, escopos, concessões e limites; criar um único Episode com orçamento fixo. Sem permissão de consulta, não executar o núcleo nem expor dependências.
2. Executar Episode::query inicial. Conservar o relatório real, suas dependências e Usage; não repetir consulta somente para produzir diagnóstico.
3. Conferir atualidade/origem por check_current e diagnosticar separadamente insuficiência conceitual, ausência relacional, oposição conceitual/relacional, negação de permissão, relatório obsoleto/adulterado e limite/reserva indisponível. Status agregado não substitui os relatórios ConceptReport/PathQueryReport nem suas provas e recusas.
4. Reutilizar diagnose_pedagogy, select_concept_strategy e a precedência select_strategy. O adaptador composto deve aplicar primeiro os impedimentos relacionais e de integridade: suporte conceitual negativo nunca sustenta composição positiva. Não basta a estratégia conceitual Respond para declarar o objetivo composto sustentado.
5. Selecionar ensino somente com relatório inicial íntegro/atual, relação independente elegível com prova suficiente e sem oposição impeditiva, insuficiência conceitual NoEvidence/InsufficientEvidence, uma experiência informativa e permissões de avaliação, apresentação e admissão específica. Inadequação de classe, atributos incompletos, oposição, relação ausente e evidência circular/compartilhada impedem ensino neste recorte. Registrar diagnóstico e escolha antes da ação.
6. Executar no máximo uma experiência por Episode::teach_once, pelos atos MP09 existentes. Uma experiência significa no máximo uma apresentação selecionada, uma admissão e uma revisão; não um lote de duas experiências sob uma chamada. Recusa/falha consome conservadoramente a oportunidade iniciada; sem segunda estratégia, retry, revise_once adicional ou reconstrução de Episode para renovar orçamento.
7. Após ensino executado, reconsultar exatamente o mesmo objetivo e Binding por Episode::query. A revisão torna obsoleto o relatório inicial; não o reutilizar como decisão final. Se houver falha que impeça reconsulta, interromper e explicitar a falta de relatório final, preservando admissões anteriores autorizadas.
8. Conferir relatório final e dependências com Episode::verify, contabilizando sua reexecução; check_current sozinho não comprova semântica. Não responder com relatório adulterado, obsoleto ou sem saldo para a conferência. Quando o objetivo já vier sustentado, verificar o relatório inicial e encerrar sem ensino; ele é a base final, sem reconsulta pedagógica redundante.
9. Encerrar com resultado, motivo, estratégia, execução, relatórios, dependências, permissões, experiência/admissão/versões e consumo. Histórico supervisor transitório limitado a oito registros; relatórios internos mantêm limites próprios. Sem novos atos cognitivos paralelos durante a supervisão ou após fechamento; leitura e salvamento/restauração seguem a sessão existente.

| Resultado de encerramento | Condição de aceitação prospectiva |
|---|---|
| Sustentado | Verification::Verified(Status::Supported), vínculo imutável, classificação positiva sustentada e prova relacional independente suficientes; nenhuma conclusão incorporada automaticamente. |
| Parcial | Experiência admissível produziu revisão efetiva, mas a reconsulta/conferência válida do mesmo objetivo segue Insufficient, sem impedimento crítico. Progresso não é sucesso composto. |
| Abstenção | Evidência insuficiente sem revisão útil, relação ausente, oposição, classificação negativa ou bloqueio semântico; preservar motivo tipado, sem converter ausência em negação factual. |
| Interrompido | Permissão necessária negada, orçamento/reserva insuficiente, erro de execução ou relatório adulterado/obsoleto que impeça decisão final conferida. Preservar Denied/Interrupted/Rejected/ReevaluationRequired subjacentes; não confundir falha de contrato com falta de evidência. |

Esses nomes são resultados do supervisor proposto, não novos valores de Status ou Verification já implementados. Sem troca automática do vínculo para salvar o resultado. Mudança externa de estado/revisão da tabela invalida a decisão dependente e encerra conservadoramente; não abre ciclo de replanejamento.

#### 8.3.2. Dependências, APIs e compatibilidade por inspeção

| Arquivo / contrato existente | Reutilização e limite observado |
|---|---|
| `src/integration/composed.rs`: Binding, SyntheticRegistry, Permissions, Limits, Usage, Report, Verification, Episode | Episode::query, check_current, teach_once, verify e usage; objetivo/vínculo privados, sem reset/Clone/checkpoint. Registro sintético não autentica identidade real. |
| `src/curriculum.rs`: Lesson, TeachingPlan, PedagogyAssessment, PedagogyPermissions, PedagogyReport, teach_concept | teach_once constrói plano com exatamente uma Lesson e avaliações 1+1; seleção e revisão existentes. supervise_concept_goal/ConceptGoalReport são referência para rastreio e encerramento, não substitutos da consulta composta. run_teaching é detalhe interno, não API pública a duplicar. |
| `src/metacognition/concepts.rs`: diagnose_pedagogy, select_concept_strategy | Diagnóstico real e seleção conceitual; ainda não conferem a relação independente. |
| `src/metacognition.rs`: RegulationDiagnostic, Strategy, select_strategy | Precedência determinística MP08; adaptar somente os sinais do relatório composto, sem criar política geral. |
| `src/session/f1.rs`: Session, query_three, encode/decode, save/load; implementação selada PedagogyHost | Uma única memória/hipóteses/journal; composed_coordinator e composed_statement são portas internas pub(crate), não novas APIs externas. Interface futura deve ser adaptador fino do mesmo fluxo. |
| `src/integration/query.rs`: QueryCoordinator::query_three, PathAuthorization, PathQueryReport, verify_path | Prova MP06 já existente; não alterar verify_path nem ampliar semântica. Suporte da composição atual exige relação de uma premissa, apesar de MP06 isolado aceitar até três. |
| `src/integration/investigation.rs`: Investigation e regulate; `src/integration/concepts.rs`: ConceptCoordinator | Preservar investigação permissionada independente e núcleo conceitual; não executar investigação nem criar indutor no próximo intervalo. |
| `tests/f1_session.rs`, `tests/mp06_paths.rs`, `tests/mp08_regulation.rs`, `tests/mp09_pedagogy.rs`, `tests/concept_goal_supervision.rs`, `tests/composed_query.rs`, `tests/tcr01_transfer.rs` | Alvos públicos de referência/regressão futura. reference_verify em composed_query é referência de teste, não API de produção; não importar avaliador/gabarito para o supervisor. Nenhum desses testes foi executado nesta formalização. |

**Compatibilidade de recursos:** Episode aceita no máximo oito consultas conceituais, três relacionais e 96 eventos neurais agregados; cada consulta composta reserva 32 eventos e usa uma consulta de cada tipo. Ensino reserva duas consultas conceituais independentes. Fluxo inicial → ensino 1+1 → reconsulta → verify final requer até cinco consultas conceituais e três relacionais, com reservas neurais de até 96; a relação direta pode consumir zero eventos, sem significar custo nulo. verify reexecuta query: verificar semanticamente também o relatório inicial antes de ensinar elevaria o fluxo a quatro consultas relacionais e não cabe. Para seleção, usar relatório inicial emitido pelo próprio Episode, check_current e conferência da prova relacional existente; reservar verify completo para o relatório que fundamentará o encerramento. Ausência de integridade suficiente interrompe, sem ampliar teto.

Respeitar reservas não devolvidas em erro (uncertain_conceptual_reservations), eventos conceituais separados, no máximo cinco operações/eventos conceituais novos por experiência, 128 operações F1, 128 eventos conceituais, 16 exemplos, 32 versões, oito fatos e validade do escopo. Conferir capacidade antes da ação; prefixo já admitido pode reduzir o saldo. Planejar timeout de processo de 10s e RSS de 256MiB na futura validação isolada, com instrumento externo e interrupção; não são garantias internas das APIs. Nenhuma medição nova foi feita aqui.

**Impedimentos e menor adaptação futura:** as primitivas comportam a sequência, mas não existe supervisor público de objetivo composto ou interface dedicada na sessão. A menor adaptação é um coordenador finito e relatório transitório sobre Episode, com adaptador mínimo na sessão F1, sem outro motor/memória. select_concept_strategy não recebe relação, atualidade ou concessões compostas: será necessário um adaptador que aplique esses bloqueios antes de reutilizar a política existente. teach_once limita uma tentativa pedagógica, mas revise_once possui contador independente e as APIs manuais continuam disponíveis; exclusividade/fechamento devem ser impostos pelo supervisor/adaptador, sem alegar proteção contra chamador hostil. Não é necessário modificar H23, verify_path, formatos ou tetos. Nenhuma dessas adaptações é executada ou autorizada por este texto.

**Persistência:** salvar somente o conhecimento já representável no F1 v1 (exemplos, versões, fatos explicitamente admitidos e proveniência), preservando suas inclusões/exclusões históricas. Não serializar objetivo ativo, Binding/registro transitório, relatórios como autoridade, estratégia, plano, concessões compostas/pedagógicas ou contadores supervisores. Novo processo exige nova configuração e permissões; não retoma ensino ou investigação. Replay de atos admitidos não é nova experiência. Não remover silenciosamente a autorização sintética base historicamente incluída no F1 v1 nem confundi-la com concessões transitórias.

#### 8.3.3. Plano público de validação — resultados ainda não obtidos

Fixar antes da futura execução dados sintéticos públicos, plano de uma experiência, vínculo/relação independente, chaves externas e orçamentos; separar estado inicial equivalente de tratamento/controle. Reutilizar uma referência simbólica independente para conceito, vínculo, relação/oposição e prova, além do verificador de produção cobrado no Episode. Nenhum acesso D/V/F, avaliador reservado ou ajuste posterior para obter sucesso.

| Cenário público prospectivo | Aceitar somente se / rejeitar quando |
|---|---|
| Insuficiente → sustentado | Consulta inicial insuficiente, relação suficiente, seleção anterior ao ensino, uma experiência autorizada, revisão e mesmo objetivo final Verified(Supported). Rejeitar sucesso sem prova ou dependente de outra pergunta. |
| Controle sem ensino | Mesmo estado/objetivo; apresentação negada ou ensino desabilitado; sem admissão/revisão e sem ganho fabricado. Distinguir interrupção por negação de abstenção por insuficiência. |
| Relação ausente | Nenhum ensino/lookup automático nem composição sustentada, mesmo com conceito positivo. Ausência não vira fato negativo. |
| Evidência oposta | Oposição conceitual e relacional em controles separados; abstenção e oposição preservada, sem escolher cadeia conveniente ou ensinar para apagá-la. |
| Permissões negadas | Negar separadamente conceito, relação, composição, avaliação pedagógica, apresentação e admissão por ID. Sem consulta quando negada na entrada; sem experiência quando faltar autorização para ensino; nenhuma concessão implícita. |
| Orçamento insuficiente | Reservas insuficientes antes da consulta, ensino, reconsulta e verify; interrupção sem extrapolar contadores, reiniciar Episode ou reembolsar falhas implicitamente. |
| Relatório adulterado ou obsoleto | Alterar prova, decisão, auditoria, versão, estado ou revisão do registro em casos separados; rejeitar/admitir necessidade de reavaliação, nunca sucesso com relatório antigo. |
| Objetivo e vínculo imutáveis | Conferir todos os campos entre consulta, escolha, ensino, reconsulta e conferência; troca de ID/item/atributos/contexto/destino/proveniência é recusada. |
| Ensino sem promoção factual | No máximo uma admissão conceitual e revisão autorizadas; memória relacional/metadados inalterados, sem Engram da classificação ou composição; bloquear segunda experiência e revise_once paralelo. |
| Reconsulta e encerramento verificáveis | Mesmo objetivo após ensino, relatório final conferido e orçamento inclui verify; cobrir também sustentado inicial sem ensino, parcial após revisão insuficiente, abstenção e interrupção. Nenhum sucesso por mera execução da experiência. |
| Persistência e processo novo | Reter conhecimento prometido e compatibilidade F1 v1/MP05 v1; não retomar supervisor/concessões, não ensinar no load; nova consulta exige autorização. |
| Regressões e reprodução | Revalidar F1, MP06, MP08, MP09, objetivo conceitual, consulta composta e TCR-01; incluir MP07 por dependência MP08 e MP05 apenas como regressão de compatibilidade, sem integrá-lo. Reproduzir em cópia isolada de fontes/testes públicos selecionados. |

**Gate de aceitação futura:** todos os cenários públicos previstos e regressões pertinentes aprovados, referência independente concordante, zero falsos sucessos sustentados, nenhuma promoção factual/ação não autorizada, orçamento e recursos medidos dentro dos limites, reprodução isolada e evidências de prefixo/persistência preservadas. Qualquer divergência de prova, troca de objetivo, segunda experiência, concessão implícita ou estouro não tratado rejeita o intervalo. Resultado parcial/abstenção esperado em controle é comportamento correto, não evidência de sucesso do objetivo. Não contar testes filtrados/ignorados como aprovados; manter probes reservados e os dois probes históricos excluídos. Não executar cargo test indiscriminado. Aceitar o intervalo técnico não conclui F2 nem substitui o aceite do critério 15.

## 9. Registro de progresso e próxima ação

### Execução autorizada até F1 — 2026-11-08

Mandato recebido: MP01, MP02, MP03, MP04 e MP11 exclusivamente F1; autonomia para integração, testes e correções dentro desse intervalo. Branch `devs`, HEAD `5066a3745709d45df6bfd5129abc71acc660f955`, origin `git@github.com:softstrucs/neurons.git`. Worktree já continha modificações e módulos essenciais não rastreados; não foi limpo ou sincronizado. Backup durável histórico conferido novamente com o SHA-256 da seção 3.1. A exposição D anterior continua pendência independente: não houve releitura ou reutilização nesta execução. A cópia de compilação/testes excluiu explicitamente os cinco arquivos de testes reservados identificados no manifesto anterior; nenhum painel, script G2 ou fixture D/V/F foi executado ou consultado.

| Etapa | Alteração e critério técnico demonstrado | Evidência / limite |
|---|---|---|
| MP01 — CONCLUÍDA NO RECORTE F1 — ACEITA COM RESSALVAS | `doctor` descreve núcleo existente e limitações; teste corrigido. Comandos, argumentos e códigos legados preservados. | 32 regressões iniciais aprovadas; ao final, 63 testes públicos aprovados. Fontes necessárias à biblioteca/CLI identificadas em pacote recuperável. |
| MP02 — CONCLUÍDA NO RECORTE F1 — ACEITA COM RESSALVAS | Adaptador aditivo `session-f1` em `session::f1` usa QueryCoordinator, CognitiveEpisode, EpisodeLog, EngramStore, TwoHopTask e Monitor existentes. Entrada tipada, proveniência, recepção/interpretação/avaliação/incorporação separadas, pedido explícito, retorno sintético, reavaliação e consulta posterior. | Provas com premissas reais; testes de suporte, ausência, oposição, fonte inválida, escopo, validade, vínculo e consulta sem alteração factual. Apenas premissa admitida é incorporada; conclusões não viram fatos. Um episódio de aquisição por sessão. |
| MP03 — CONCLUÍDA NO RECORTE F1 — ACEITA COM RESSALVAS | ConceptCoordinator existente exposto por comandos; H23 e seleção/revisão/metacognição originais intactas. Conceito tem nome opaco, sem regra alvo configurada. | Experiências públicas novas, independentes das partições, reduzem H preservando versões, eliminações e evidências. Testes de desacordo, atributos incompletos, contradição, família inadequada, quarentena e leitura sem promoção. Indução simbólica limitada, sem alegação neural/geral. |
| MP04 — CONCLUÍDA NO RECORTE F1 — ACEITA COM RESSALVAS | Checkpoint F1 v1 distinto do formato legado; journal lógico limitado, revalidação determinística, checksum de bytes e do estado reconstruído, criação exclusiva e publicação sem substituir destino. | Round-trip de fase de aquisição pendente, fatos/proveniência, exemplos, hipóteses, versões, decisões e continuação idêntica. Corrupção/truncamento byte a byte, versão/formato, comando de E/S adulterado, falha de caminho, destino ocupado e restauração inválida testados. |
| MP11 (F1) — CONCLUÍDA NO RECORTE F1 — ACEITA COM RESSALVAS | Integração funcional e reprodução em cópia isolada com 40 arquivos de produção/configuração e 8 alvos públicos. | `check`, `build` e 63 testes aprovados; 2 probes filtrados explicitamente. Duas sessões CLI completas idênticas, dois checkpoints de 897 bytes com hashes iguais e restauração idêntica. Aceita pelo responsável em 08/11/2026 com ressalvas; critério 12 registrado no aceite abaixo. |
| MP05–MP10 / CIC-I6/P8 | Não iniciadas nesta atividade. | Sem rede, dependências novas, serviços, G2/G7, acesso reservado, commit ou push. |

### Contrato de produto F1 e uso

Executar `cargo run --offline --locked --bin neurons -- session-f1`; `ajuda` mostra toda a gramática. `session` e `session-explain` continuam sendo os adaptadores legados compatíveis. A integração F1 é um adaptador dos coordenadores existentes, não um segundo motor. Não recebe mundo, gabarito, avaliador ou partições. Os identificadores e nomes públicos dos testes novos não codificam a regra verdadeira de um experimento.

Na F1, `config CATALOGO CONTEXTO REGRA VERSAO INICIO FIM` fixa o escopo; `simbolo ID NOME` declara até cinco símbolos; `autorizar CONTEXTO` autoriza explicitamente a composição de inclusão nesse escopo sintético, sem conceder qualquer permissão P2 ou autorização G2/G7. Fontes 7 e 8 são declaradas pela política sintética local, sem autenticação ou verdade científica. `receber` → `interpretar` → `avaliar` → `admitir` são atos distintos; a avaliação pode colocar a informação em quarentena. `consultar` retorna decisão, prova, premissas com proveniência e exclusões.

`iniciar ORIGEM DESTINO` fixa a primeira consulta do episódio; `pedido` deriva a lacuna a partir de evidência, sem consultar gabarito; `retorno` exige vínculo ao pedido. `interpretar-retorno` → `avaliar-retorno` → `admitir-retorno ID` exige incorporação explícita. `reavaliar` repete a consulta inicial e `consultar-final ORIGEM DESTINO` recebe somente então uma consulta posterior pública (o nome do comando não se refere à partição F). Hipótese, fonte desconhecida, evidência ausente e contexto incompatível não se tornam observações. Oposição permanece registrada, sem escolha arbitrária de fonte.

`conceito ID NOME` cria identidade opaca e H23; `exemplo` → `interpretar-exemplo` → `avaliar-exemplo` → `admitir-exemplo ID` → `revisar` expõe recepção, admissibilidade, atualização e revisão. `classificar CONTEXTO BITS` exige unanimidade conservadora, inclusive nas completações de no máximo um atributo desconhecido. `estado` e `historico` mostram versões, premissas, evidências, estados epistêmicos, recusas e alterações. Uma consulta avança o instante lógico e registra episódio; fatos, H, regras e metadados epistêmicos permanecem iguais. Operações são preparadas numa reconstrução isolada e só substituem o estado vivo após sucesso; falhas preservam o prefixo aceito.

**Limites declarados:** 128 operações lógicas aceitas e 256 entradas por processo, 256 bytes por linha, cinco símbolos, oito fatos, três consultas livres mais as três decisões do único episódio M3, um pedido/retorno, um conceito opaco, até 16 contextos/exemplos, 32 versões e 128 eventos conceituais, 80 consultas conceituais. Capacidade excedida gera erro explícito sem perder o prefixo. `agora` avança tempo lógico monotônico; não é relógio confiável ou instrumentação G2.

**Persistência prometida:** configuração e autorização sintética, catálogo declarado, memória factual e proveniência, pendências e quarentenas semânticas, fase de aquisição, pedidos/retornos/provas, identidade conceitual, exemplos admitidos, H/candidata/versões/eliminações/suportes, logs e tempo lógico. Reconstrução é por replay determinístico dos atos lógicos aceitos, sem executar E/S a partir do arquivo. `salvar CAMINHO` exige caminho novo: nunca substitui arquivo ou staging preexistente; `restaurar CAMINHO` só troca a sessão após validação completa. Formato F1 v1 é aditivo e rejeita o formato legado; mudanças semânticas futuras precisam nova versão ou rejeição por divergência do checksum do estado reconstruído.

**Exclusões:** circuitos, pesos/potenciais/filas neuronais arbitrárias, processo/estilo legado, descritores, orçamento de linhas do processo e texto de erros sintáticos/saída de console não fazem parte do checkpoint. Erros sintáticos são relatados na saída; atos semânticos aceitos e quarentenas ficam no histórico. Checksum não é autenticação, criptografia operacional ou proteção contra adulteração maliciosa. Sistema de arquivos e permissões do SO continuam externos; não há sandbox de segurança, proteção contra administrador hostil ou garantia de durabilidade física após queda de energia. Publicação exige suporte local a hard-link; falha gera erro. Se a publicação ocorrer e a limpeza do staging falhar, o erro não implica inexistência do arquivo publicado. Não se promete continuidade cognitiva integral.

### Verificações e reprodução

[Manifesto F1 de fontes e evidências](docs/referencia/F1_REPRODUTIBILIDADE.json) identifica os 48 arquivos do pacote (40 produção/configuração, 8 testes), hashes, comandos, logs e medições. O pacote inclui módulos essenciais não rastreados e as cinco modificações Rust anteriores, mantidas intactas. Clone remoto sozinho continua insuficiente para reproduzir esta árvore local. Os 48 arquivos do tar foram reconstruídos e comparados por hash, sem copiar fixtures reservadas. Pacote fonte desta entrega preservado em `/home/aguiar/Documentos/Trabalho/Softstrucs/Backups/neurons/F1_20261108_twhs6vgj/F1-source.tar`, 501760 bytes, SHA-256 `72b1541d3748883eefd835870c172a6662fa7ce09bcc615a518e5d939979f565`; 18 arquivos de fonte/logs/transcrições/documentos foram copiados e verificados. Retenção persistente no mesmo disco físico, sem segunda cópia independente demonstrada. Requisitos externos: Rust/Cargo 1.88.0, std e ambiente Linux observado; não há crates novas. Não se demonstrou portabilidade para outros sistemas.

Comandos executados na cópia isolada, com `CARGO_TARGET_DIR` também isolado:

```sh
cargo check --offline --locked --lib --bin neurons
cargo test --offline --locked --test cli --test engine --test interactions --test learning --test memory --test persistence --test personality --test f1_session -- --skip bounded_update_probe --skip bounded_cost_probe_for_activation_and_communication
cargo build --offline --locked --lib --bin neurons
```

Resultado final: **63 aprovados, zero falhas, zero ignorados; dois probes de desempenho não executados**. Distribuição: CLI 3, motor 7, interações 9, aprendizagem legada 6, memória 7, persistência legada 9, personalidade/sessão legada 6, F1 16. Não somar reexecuções intermediárias como testes independentes. Não executar `cargo test --lib` ou uma seleção indiscriminada: os alvos científicos reservados continuam excluídos. Formatação dos cinco arquivos Rust desta entrega verificada com `rustfmt --check --config skip_children=true`. `git diff --check` passou.

Duas sessões CLI públicas: tempos de parede observados no pai de aproximadamente 26,60 ms e 18,51 ms, saídas idênticas, checkpoints de 897 bytes idênticos, restauração em processo novo idêntica. Uma execução adicional instrumentada via `os.wait4` registrou 33,95 ms de parede, 17,689 ms de CPU usuário, 4,422 ms de CPU sistema e pico RSS de 16.964 KiB, incluindo inicialização do processo. Esses números não são benchmark comparativo, memória viva ou garantia de orçamento universal. Heap vivo e alocação física de disco permanecem não avaliados; tamanho lógico do checkpoint foi medido. Nenhum instrumento exigiu serviço ou dependência externa.

### Situação dos critérios de F1 e próxima ação

Critérios 1–11 da seção 8.1 demonstrados no contrato e ambiente delimitados acima: conjunto fonte preservado/reproduzível, biblioteca e CLI compiladas, regressões públicas aprovadas, sessão/evidências/provas/abstenções, aquisição explícita, revisão com histórico, checkpoint fiel ao contrato, erros/limites e cópia isolada. **Critério 12: aceite formal do responsável registrado em 08/11/2026 — APROVADO COM RESSALVAS, conforme decisão explícita recebida nesta atividade documental.** Isso não é homologação científica, aprendizagem autônoma ou demonstração integral de PCF-001/002/003.

Situação executiva: F1 encerrada com ressalvas, como primeira entrega funcional formalmente aceita. F2 permanece não iniciada e não autorizada. Nenhuma nova etapa será iniciada automaticamente após este registro. G2 real permanece `NotEvaluable`; G7 e acesso D/V/F permanecem `NotAuthorized`; permissões P2 e protocolos/resultados históricos permanecem intactos. A ocorrência anterior de exposição D e a necessidade de segunda cópia em dispositivo independente continuam pendências separadas.

### Registro definitivo do aceite humano F1 — 08/11/2026

**F1 — SESSÃO COGNITIVA LOCAL LIMITADA**

- **Implementação:** concluída no recorte autorizado.
- **Validação técnica:** APTO COM RESSALVAS, conforme parecer da validação final anteriormente apresentado.
- **Aceite humano:** APROVADO COM RESSALVAS por decisão explícita do responsável pelo projeto.
- **Data da decisão:** 08/11/2026.
- **Etapas abrangidas:** MP01, MP02, MP03, MP04 e MP11, exclusivamente no recorte F1 — CONCLUÍDAS NO RECORTE F1 — ACEITAS COM RESSALVAS. Entregas futuras dessas etapas não são concluídas por extensão.
- **Base Git:** branch `devs`, HEAD `5066a3745709d45df6bfd5129abc71acc660f955`. Esse HEAD é a base histórica, não contém por si só a implementação F1 ainda não versionada; o conjunto fonte inclui modificações locais e arquivos essenciais não rastreados.
- **Testes relatados:** 63 aprovados, zero falhas, zero ignorados; não houve nova execução ou avaliação funcional nesta atividade de registro.
- **Reprodutibilidade:** demonstrada na validação anterior em cópia isolada com o conjunto fonte completo necessário ao recorte F1, não apenas com o commit remoto.
- **Alcance do aceite:** funcionalidades delimitadas de sessão, evidência, inferência/abstenção, aquisição explícita, revisão conceitual, histórico e persistência, conforme contrato de inclusão/exclusão acima.
- **Situação executiva:** F1 encerrada com ressalvas; primeira entrega funcional formalmente aceita. F2 não iniciada nem autorizada.

**Ressalvas permanentes do aceite:**

1. **Backup independente:** o backup persistente foi verificado, mas não existe cópia verificada em dispositivo físico independente.
2. **Git e distribuição:** arquivos essenciais continuam modificados ou não rastreados; o repositório remoto isoladamente não reproduz F1.
3. **Formatação global:** `cargo fmt --all -- --check` falhou na cópia isolada por ausência deliberada de módulo de testes. A verificação dos cinco arquivos Rust da entrega passou separadamente; a verificação global permanece inconclusiva.
4. **Probes de desempenho:** `bounded_update_probe` e `bounded_cost_probe_for_activation_and_communication` não foram executados e não integram os critérios obrigatórios de F1.
5. **Limites científicos:** o aceite não demonstra inteligência geral, consciência, aprendizagem neural autônoma, transferência geral ou determinismo universal.
6. **Ocorrência de acesso D:** permanece registrado o acesso por inspeção estática indevida anterior; não se declara conformidade integral retroativa nem se reutiliza seu conteúdo. A ocorrência continua como pendência independente.

Essas ressalvas não são apagadas ou convertidas em requisitos retroativamente aprovados pelo aceite. MP00 permanece concluída no escopo de recuperação, com suas ressalvas; MP05–MP10 permanecem não iniciadas no novo intervalo executivo, preservadas as evidências históricas dos mecanismos preexistentes. F2 permanece evolução futura, sem autorização executiva nesta atividade. Este documento continua sendo o único plano executivo ativo.

O [manifesto F1](docs/referencia/F1_REPRODUTIBILIDADE.json) e as referências existentes a comandos, testes, resultados, checkpoints, hashes e limitações são preservados como evidências históricas: seus campos anteriores de aceite pendente não são reescritos retroativamente. O presente registro documenta a decisão humana posterior. A retenção permanente dos caminhos temporários `/tmp/neurons-F1.patch`, `/tmp/neurons-F1-status.txt` e `/tmp/neurons-f1-audit-3j0d0h8f/commands.json` não está comprovada; este aceite não depende exclusivamente deles e não os copia, move, regenera ou altera.

Este registro é exclusivamente documental, sem nova validação funcional, homologação científica ou autorização operacional. G2 real permanece `NotEvaluable`; G7 e D/V/F permanecem `NotAuthorized`; permissões P2 inalteradas; CIC-I6/P8 não iniciada. Nenhuma implementação, etapa seguinte, publicação, commit ou push é autorizada por este aceite.

### F2 — primeiro intervalo MP06: inclusão com até três premissas

**Resultado técnico: IMPLEMENTADO E VALIDADO no recorte autorizado.** Base inicial `devs`, HEAD `afe2bd58c52eeacdbcfe4f39d39ab48a1bc98d8a`, sem mudança de branch, commit ou push. A autorização posterior ao aceite F1 abrange somente composição de inclusão até três premissas; não inicia MP05, MP07, MP08, MP09, MP10 ou CIC-I6/P8. F2 integral permanece não concluída. O aceite F1 e suas seis ressalvas acima permanecem intactos.

**Arquivos:** `src/cognition.rs` compartilha a propagação Circuit/Engine existente com `src/cognition/bounded.rs`; a extração legada TwoHopTask mantém sua semântica. A extensão enumera somente caminhos simples de até três premissas, ordenados por quantidade de premissas e depois pelos IDs em ordem lexicográfica. IDs servem exclusivamente ao desempate, nunca ao significado. A propagação compartilhada registra eventos/ativações, falhas e orçamento; não aprende pesos. Recuperação literal dispensa propagação como no contrato legado. Não há segundo motor, serviço ou dependência externa.

`src/metacognition.rs` acrescenta julgamento de caminhos sem modificar Decision/EvidenceRefs: oposição à conclusão ou a premissa de qualquer cadeia candidata relevante impede a resposta, mesmo quando há alternativa mais curta favorável. Oposição permanece no relatório junto aos registros consultados; uma cadeia conveniente não a oculta. Orçamento/propagação incompletos produzem abstenção. Ausência de cadeia positiva não é convertida em conclusão negativa.

`src/integration/query.rs` acrescenta PathAuthorization, PathProof, PathQueryReport e PathEpisodeLog; QueryCoordinator continua vinculando engramas aos metadados existentes. São tipos aditivos necessários porque EvidenceRefs/EpisodeDecision legados representam no máximo duas premissas; esses enums não foram ampliados incompativelmente. A autorização MP06 é explícita e adicional à autorização base, com escopo/regra/versão/validade coincidentes. Cada premissa é verificada por identidade, conteúdo, polaridade, admissibilidade, proveniência, evidência, contexto e tempo. verify_path rejeita referências ausentes, negativas, repetidas/cíclicas, desconectadas ou comprimentos indevidos; não substitui a prova recebida por outra. Consulta mantém fatos, estados, regras e metadados imutáveis; somente o log externo cresce. Suporte produz conhecimento provisório, nunca validação explícita ou incorporação factual.

**Limites e compatibilidade:** oito fatos, três premissas e no máximo 64 eventos por consulta MP06; orçamento zero ou acima do teto gera abstenção. A enumeração é finita, com teto conservador de 8+64+512 extensões e profundidade três, separada da contagem de eventos do motor. Ciclos podem consumir o orçamento e gerar abstenção mesmo havendo um caminho simples. O limite M1 de três episódios por log é preservado, mais restritivo que o teto autorizado de 32 consultas por execução de teste; os novos testes permanecem abaixo desse teto. A regressão F1 não altera seus limites próprios já aprovados (inclusive exemplos e consultas conceituais); não foram ampliados para MP06.

**Sessão:** `src/session/f1.rs` adiciona `autorizar3 CONTEXTO`, `consultar3 CONTEXTO ORIGEM DESTINO` e `historico3`. A autorização só é aceita em escopo com autorização base e permanece fixada até restauração/encerramento. O relatório mostra prova, alternativas, conflitos, exclusões, contexto, regra, instante, estados e snapshots. Autorização e log MP06 são transitórios, não entram no checkpoint F1 v1 e são limpos explicitamente ao restaurar. Isso é informado no início da sessão; não se promete persistência da extensão. O formato, estado lógico e bytes dos checkpoints F1 permanecem compatíveis. Não há aquisição externa ou alterações implícitas de memória.

**Demonstração pública própria do produto:** observações 11:101→103, 12:103→107 e 13:107→109, catálogo71/contexto61/regra81 versão1. A consulta 101→109 produziu prova [11,12,13]. Após admissão explícita da observação negativa 14:103→107, a mesma consulta absteve-se com ContradictoryEvidence, preservando as alternativas e a oposição. A consulta 109→101 absteve-se com MissingEvidence. Nenhuma conclusão foi cadastrada. O cenário foi repetido em processos novos, com saídas idênticas; isso não estabelece determinismo universal ou vantagem neural.

**Testes e comandos efetivamente executados:** em cópia isolada de fontes permitidas, com CARGO_TARGET_DIR/TMPDIR temporários e exclusão dos cinco módulos de testes reservados já identificados no manifesto F1:

```sh
cargo check --offline --locked --lib --bin neurons
cargo build --offline --locked --bin neurons
cargo test --offline --locked --test cli --test engine --test interactions --test learning --test memory --test persistence --test personality --test f1_session --test mp06_paths -- --skip bounded_update_probe --skip bounded_cost_probe_for_activation_and_communication
```

Resultado final: **76 testes aprovados — 13 novos MP06 e 63 de regressão F1; zero falhas, zero ignorados; dois probes não executados**. Não somar reexecuções intermediárias. `tests/mp06_paths.rs` é público, autossuficiente e não importa fixtures/avaliadores científicos. Cobre provas de 1/2/3 premissas, verificações individuais, adulteração, exclusões, oposição em alternativa não escolhida, desempate e permutação de inserção, ausência de mutação, contexto/regra/tempo, orçamento/ciclos, log/prefixo e compatibilidade de sessão. A referência independente usa multiplicação booleana de matrizes em quatro casos públicos; os controles literal e TwoHopTask permanecem sem suporte para o caso de três premissas. Não é avaliação científica geral.

Um checkpoint F1 de 897 bytes preservado antes deste intervalo, SHA-256 `053cd611f7ed562f41ff1c3fffbad818f9861c70b9bb5d0f4de2837661ff0916`, foi restaurado e salvo pelo binário novo com igualdade byte a byte. Os testes F1 existentes não foram alterados. Formatação dos seis arquivos Rust envolvidos e git diff --check conferidos; a ressalva histórica de formatação global da F1 não foi reaberta.

**Recursos realmente observados:** duas execuções do cenário CLI, medidas com perf_counter_ns no pai e os.wait4 do processo filho Linux: parede 4,914 ms / 4,084 ms; CPU usuário 0 / 3,788 ms; CPU sistema 4,035 / 0 ms; pico RSS incluindo inicialização 15.252 / 15.712 KiB. Cada execução realizou três consultas MP06 e produziu 37.724 bytes de saída. Valores zero refletem a resolução/contabilização observada, não ausência de custo. Não são orçamento universal, heap vivo, vantagem de desempenho ou isolamento comprovado.

Logs brutos, comandos, entradas públicas e medições desta execução estão em `/tmp/neurons-mp06-2m7oaknw/`, especialmente `regression-final.log`, `regression-command.json`, `scenario.input.txt`, `scenario-1.output.txt`, `scenario-2.output.txt`, `old-checkpoint-compatibility.log` e `evidence.json`. Sua retenção permanente não está demonstrada. Resultados e limites essenciais estão registrados neste Plano Mestre; não foi criado novo plano ou protocolo de governança.

**Pendências e encerramento:** log/autorização MP06 não persistentes; ciclos podem impedir conclusão no orçamento; cobertura pública finita, sem generalização científica. Não houve acesso D/V/F, execução G2/G7, instrumentos reservados, novos crates, serviços, instalação, commit ou push. As seis ressalvas F1 permanecem. Encerrado somente o primeiro intervalo MP06, aguardando avaliação do responsável; nenhum próximo intervalo é iniciado automaticamente.

### F2 — intervalo MP05: episódio sináptico assistido e persistente

**Estado: IMPLEMENTADO E VALIDADO no recorte autorizado**, sobre `devs`, HEAD inicial `68dfd7746dba2ffb7e0f03a0f110a9be7ca45128`. Não encerra MP05 integral ou F2. Sem commit/push. O aceite humano F1, suas seis ressalvas e o registro MP06 anterior permanecem literalmente preservados. Nenhuma etapa MP07–MP10, CIC-I6/P8, G2/G7 ou acesso D/V/F foi iniciada.

**Implementação e APIs aditivas:** `src/integration/synaptic.rs` coordena atos explícitos sobre Circuit/Neuron/Synapse/Engine e aplica a PlasticityRule existente. Nenhuma regra neural alternativa ou dependência foi criada. `src/neuron.rs` expõe snapshot somente leitura de identidade, limiar, limite, potencial e contadores; `src/synapse.rs` acrescenta leitura de atraso; `src/engine.rs` expõe tempo lógico sem avançar eventos. `src/circuit.rs` expõe snapshot completo de topologia/estados e bytes do Engine pelo codec legado, sem modificar seu formato. O EngramStore vazio usado internamente pelo codec do motor não é memória factual ou admissão epistêmica. O coordenador não possui acesso mutável à memória F1/MP06.

`src/session/synaptic.rs`, exportado por `src/session.rs` e roteado em `src/main.rs`, oferece `session-mp05`. A sessão F1 e a extensão MP06 permanecem inalteradas. A interface é um adaptador do mesmo núcleo, não outro motor. `novo`, `neuronio`, `sinapse` e `autorizar` configuram explicitamente o episódio; `experiencia` agenda o estímulo sem atualizar peso; `executar` processa o ciclo existente; `feedback` valida e aplica explicitamente a atualização; `reverter` restaura explicitamente o peso anterior da última alteração reversível; `estado`/`historico`, `salvar` e `restaurar` permitem auditoria e retomada. A ajuda da CLI especifica a gramática completa.

**Contrato de feedback:** identidade, experiência mais recente concluída sem erro, circuito/contexto, fonte declarada autorizada, evidência não zero, sinapse correspondente à origem efetivamente ativada, sinal, regra/versão e instante lógico são vinculados antes do ajuste. A origem informante é distinta do neurônio de origem. Fonte declarada não autentica autoria ou verdade. A atribuição à sinapse é externa e assistida, não crédito causal autônomo. Feedback ausente, inválido, fora de contexto, fonte desconhecida, vencido, anterior ao relógio, duplicado ou com experiência/evidência consumida não altera estado/pesos. Uma experiência admite um feedback. Saturação sem mudança de peso também consome o feedback. Erros preservam atomicamente o prefixo aceito; recusas de entrada são mostradas na saída, não adicionadas ao checkpoint como atos aceitos.

O histórico conserva configuração, autorizações sintéticas, experiências e snapshots neuronais antes/depois, ciclos/eventos/falhas, peso anterior/posterior, regra/versão, passo/limites, saturação, fonte/evidência e reversões. Reversão usa o valor anterior registrado, não feedback de sinal oposto, e não apaga consumo ou história. Ela reverte apenas o peso: não desfaz eventos, potenciais ou contadores já observados. As quatro mudanças permitidas incluem reversões; tentativas válidas saturadas sem mudança não contam como alteração, mas ficam registradas e consumidas.

**Limites:** até quatro neurônios, quatro sinapses, fila16, ciclo64, episódio128 eventos, oito experiências e quatro alterações de peso. Até duas fontes autorizadas, oito feedbacks, 256 operações lógicas/512 linhas de sessão, 256 bytes por linha e arquivo64KiB. A topologia é fixada após a primeira experiência. Neste primeiro recorte o coordenador aceita apenas grafos acíclicos: com quatro arestas, cada estímulo percorre no máximo 2^4 trajetos incluindo o inicial; oito experiências respeitam o teto128. O Circuit geral não foi restringido. Ciclos, capacidades, parâmetros inválidos e destinos ausentes são recusados. A fila pode permanecer pendente entre `experiencia` e `executar` ou com ciclo menor; salvar rejeita esse estado sem executar ou descartar eventos. Falha de transmissão preserva registro da experiência inválida e impede feedback sobre ela.

**Persistência:** formato MP05 v1 próprio, magic `NRNMP05`, tamanho e checksums de journal e estado reconstruído, serialização canônica limitada. Inclui configuração, topologia, parâmetros/potenciais/contadores neuronais, pesos/atrasos, relógio/métricas/configuração do motor, autorizações sintéticas com validade original, experiências, feedbacks consumidos e histórico/reversões. Restauração reexecuta em instância nova somente atos lógicos permitidos, usando os construtores e o motor existentes, e compara o estado completo antes de aceitar. Não executa comandos de E/S vindos do arquivo. Formato/versão, corrupção, topologia/histórico inválidos e estado incompatível são rejeitados. Arquivos são publicados por criação exclusiva, sem substituir destino ou staging preexistente; falha após publicação pode deixar o arquivo criado, relatando erro. Não há promessa de durabilidade física após queda de energia.

O checkpoint só é aceito em repouso e não contém memória epistêmica F1, log transitório MP06, descritores de processo ou saída de console. Consumo de feedback e validade são preservados no estado restaurado; não há autenticação criptográfica, proteção global contra rollback/replay de arquivos antigos ou autoridade temporal real. Checksums detectam corrupção, não adversários capazes de reescrever arquivos. Mudanças futuras de semântica exigem versão nova ou rejeição por divergência do estado reconstruído. Os formatos F1 v1 e Engine+EngramStore legados não foram modificados.

**Demonstração pública:** circuito41/contexto61/regra81 v1, neurônios101 (limiar1) e103 (limiar2), sinapse11 inicialmente0, regra [0,3] passo2. A experiência1 com carga1 não ativa103. Feedback201, fonte sintética7 autorizada, evidência1201, ligado à experiência1, altera peso0→2 sem ativação imediata do destino. Reaplicar a mesma carga na experiência2 ativa103; o controle sem atualização permanece sem resposta. A referência simbólica independente reproduz o limiar, sem alegação de superioridade neural. Um segundo feedback positivo satura2→3; reversão explícita restaura2, enquanto feedback negativo de passo2 produziria1. Isso demonstra adaptação sináptica assistida nesta tarefa, não algoritmo novo, aprendizagem autônoma, generalização ou compreensão.

**Verificações realizadas:** cópia isolada das fontes permitidas, sem os cinco módulos de testes reservados da seleção histórica F1; CARGO_TARGET_DIR e TMPDIR isolados. O erro inicial de compilação por ausência de Synapse::delay foi corrigido com getter aditivo somente leitura; nenhuma expectativa de teste foi alterada para forçar aprovação. Comandos finais:

```sh
cargo check --offline --locked --lib --bin neurons
cargo build --offline --locked --bin neurons
cargo test --offline --locked --test cli --test engine --test interactions --test learning --test memory --test persistence --test personality --test f1_session --test mp06_paths --test mp05_synaptic -- --skip bounded_update_probe --skip bounded_cost_probe_for_activation_and_communication
```

Resultado: **91 aprovados — 15 MP05, 13 MP06 e 63 F1; zero falhas, zero ignorados; dois probes não executados**. Testes em `tests/mp05_synaptic.rs` são públicos e autossuficientes. Cobrem consequência funcional contra controle sem atualização/referência simbólica, localização, saturação/no-op consumido, reversão, autorização/validade/vínculos, duplicidade, fila pendente, limites, falhas de propagação, round-trip completo, retenção de potencial neuronal não zero com efeito na resposta seguinte e continuação em processo filho. Corrupção/truncamento byte a byte e journals adulterados com checksum correto são recusados; falhas de caminho e arquivos ocupados não sobrescrevem dados. F1 e MP06 mantiveram testes/fontes próprios; checkpoint F1 público preservado foi restaurado e salvo com bytes idênticos. A formatação aplicável e git diff --check foram verificados; a ressalva global de formatação F1 permanece.

**Recursos observados:** duas execuções CLI públicas com saídas idênticas: parede18,791/7,098ms; CPU usuário0/2,636ms; CPU sistema7,399/2,636ms; pico RSS Linux15.312/15.452KiB incluindo inicialização. Método: perf_counter_ns no pai e os.wait4 do filho. Zero indica a contabilização observada, não ausência de custo. Checkpoint após a primeira atualização201 bytes; após continuação235 bytes. Em processos independentes, restaurar o primeiro e executar a mesma continuação gerou checkpoint final byte a byte igual ao controle não interrompido, SHA-256 `0f7ba1a015de09d42f3ab77c7eb27ecab24860c87a3a16e1b528ae3fced821ff`. As medições não comprovam orçamento universal ou heap vivo.

Logs, entradas, checkpoints públicos e medições estão em `/tmp/neurons-mp05-30fwofty/`, incluindo `regression-final.log`, `regression-command.json`, `evidence.json`, `episode-1.input.txt`, `episode-1.output.txt` e `restored-1.output.txt`. Retenção permanente desses artefatos temporários não está demonstrada. Este Plano Mestre contém o registro essencial; nenhum novo plano ou documento de governança foi criado.

**Encerramento:** implementado somente o episódio MP05 delimitado. Permanecem as limitações de atribuição externa do feedback, topologia acíclica, replay limitado ao formato/semântica e persistência em repouso. Não houve alteração de fatos admitidos, dependências novas, rede, serviços, acesso D/V/F, instrumentos reservados, commit/push ou início de outras etapas. Aguarda avaliação do responsável; MP05 integral e F2 não são declaradas concluídas.

### F2 — primeiro intervalo MP07: investigação supervisionada em fonte local

**Resultado: IMPLEMENTADO E VALIDADO no recorte autorizado.** Base `devs`, HEAD inicial `ee4388d92c5b7a32f78a4cae4b91b7b3f91d4bc1`. Não encerra MP07 integral ou F2, não amplia MP05/MP06/MP03 e não inicia MP08–MP10. O aceite F1, suas seis ressalvas e os registros publicados de MP05/MP06 permanecem literalmente preservados. Sem commit/push, G2/G7, CIC-I6/P8, acesso D/V/F ou instrumentos científicos reservados.

**Arquivos e arquitetura:** novo `src/integration/investigation.rs`, exportado aditivamente em `src/integration.rs`, controla uma única fonte e uma oportunidade de consulta. A porta AcquisitionHost é selada e implementada por Session em `src/session/f1.rs`: delega consulta inicial, derivação do pedido, recepção, interpretação, admissibilidade, incorporação e reavaliação aos atos já existentes de CognitiveEpisode/QueryCoordinator. Não contém outro algoritmo de inferência, lacuna ou admissão. A porta chama os métodos M3 diretamente e só então registra os mesmos comandos no journal F1 v1; não usa o replay transacional de Session.execute para supervisionar o episódio. Assim, consultas históricas não são reexecutadas durante os atos MP07 e o orçamento contabiliza as consultas efetivamente realizadas nesse fluxo. Nenhuma mudança em acquisition.rs ou nas regras científicas congeladas foi necessária.

A fonte LocalSource é uma tabela privada de até oito Information tipadas, sem API de enumeração/Debug ou respostas esperadas. Seu conteúdo é fixado antes da pergunta MP07 e não pode ser alterado durante o episódio. A consulta real lê registros existentes, incrementando contador de acesso; não constrói uma afirmação copiando a lacuna. Retorna a única relação exata, ou, se não houver exata, o único registro sobre o sujeito, que pode ser irrelevante. Múltiplos resultados candidatos geram ambiguidade sem escolher uma fonte/afirmação conveniente. O candidato recebe somente retorno/ausência, não o catálogo completo. O instante de transporte é novo, mas afirmação, evidência e natureza permanecem originais; o registro original é preservado no SourceCall. A validade e a cronologia originais também são verificadas antes de materializar o retorno. Conhecimento de fonte ou EvidenceId não constitui autenticação ou verdade científica.

**Permissões separadas:** autorização de investigação, acesso à fonte e admissão são declaradas em campos distintos, vinculados ao escopo; ações externas são incondicionalmente proibidas. Sem autorização de fonte, lookup não é executado. Permissão de consulta não autoriza incorporar. A fonte deve ainda ser reconhecida pelo host; isso não substitui sua autorização explícita. A admissão recusada prevalece sobre permissão anteriormente declarada. Esses controles são sintéticos e supervisionados, não identidade operacional real ou isolamento adversarial. As APIs manuais legadas F1 não foram revogadas; durante MP07 a CLI bloqueia comandos paralelos que poderiam contornar sua supervisão.

**Fluxo:** consulta inicial deve resultar em MissingEvidence para permitir a investigação. Suporte suficiente ou limite de recursos encerra a tentativa sem chamar fonte; ambiguidade/oposição à base impede derive_request e conserva o motivo. Pedido vem do mecanismo existente: A→B conhecido e A→C sem suporte permitem perguntar B→C. O recorte não amplia a derivação para lacunas de três premissas, conceitos ou aprendizagem sináptica.

Retorno identificado é ligado ao pedido/episódio, interpretado e avaliado pela política atual. Admissibilidade e pertinência permanecem distintas: observação positiva irrelevante pode ser admissível sob a política publicada, mas nunca prova a consulta original. Oposição relevante é exibida antes da admissão e continua disponível ao Monitor na reconsulta. Uma única observação pode ser incorporada mediante ato explícito, sem elevar a ExplicitlyValidated e sem cadastrar a conclusão inferida. Retornos com catálogo/regra/versão não representáveis fielmente na gramática F1 v1 são recusados e registrados como HostRefused, sem reinterpretação silenciosa. Contexto incompatível, hipótese, evidência ausente/duplicada ou validade inválida não produzem admissão indevida.

Ausência, ambiguidade da fonte ou recusa de admissão permitem finalizar por QueryCoordinator em consulta somente leitura. Não se chama incorporate com lote vazio e não se força fase Incorporated. A consulta adicional fica no relatório transitório MP07, preservando o journal/estado epistêmico F1 nessa finalização. Validade temporal ou contratos de envelope podem impedir uma consulta; nesse caso há erro explícito, sem fabricar episódio fora da validade ou contar falha como resposta correta.

**Limites:** um pedido, uma fonte, uma consulta efetiva, um retorno, uma candidata, profundidade1 e nenhum retry automático. A oportunidade fica consumida inclusive em retorno inválido/recusado. Até oito fatos e uma incorporação. Teto de três decisões e96 eventos por episódio, com verificação explícita de até32 eventos em cada relatório. O fluxo mínimo usa duas decisões; o exemplo positivo processou três eventos. O traço CognitiveEpisode conserva o limite existente de oito entradas (fluxo positivo usa sete); os campos transitórios adicionais têm capacidade fixa: três permissões, um pedido, um SourceCall, uma avaliação/oposição limitada aos oito fatos e até três relatórios. Não se amplia o traço M3 nem os orçamentos do motor. A fonte pode conter até oito registros. Não houve instalação, dependência, serviço ou busca externa. O timeout opcional de1s não foi tratado como garantia de desempenho.

**Sessão existente:** `session-f1` acrescenta preparação `fonte7 ID`, `evidencia7 CONTEXTO ORIGEM DESTINO +|- EVIDENCIA|- observacao|hipotese`, `fixar-fonte7` antes da pergunta. O episódio usa `investigar7`, `permitir-investigacao7`, `pedido7`, `permitir-fonte7`, `consultar-fonte7`, `interpretar7`, `avaliar7`, `permitir-admissao7`, `admitir7 ID`, `recusar7`, `concluir7` e `historico7`. `externa7` sempre recusa. A consulta só lê a fonte depois das permissões independentes. Preparação da fonte não altera fatos da sessão.

**Persistência:** fonte, permissões, orçamento e histórico MP07 são transitórios. Somente atos já representáveis (retorno materializado e observação explicitamente admitida, com histórico M3) entram no journal F1 v1 existente. Nenhum comando de consulta à fonte entra no replay. Salvar não promete guardar a investigação integral; restaurar limpa a fonte e todos os controles MP07 e não executa lookup. A reconsulta sem incorporação permanece fora do checkpoint. F1 v1 e MP05 v1 não foram alterados; seus arquivos públicos de897 e235 bytes foram restaurados/salvos com igualdade byte a byte. Checkpoint do cenário MP07, limitado aos atos F1 representáveis, tem312 bytes e também foi restaurado sem retomar permissões ou consultar fonte.

**Demonstração pública:** catálogo71/contexto61/regra81 v1; fato11:101→103 admitido; fonte7 fixada previamente contém evidência1012 para103→107. Pergunta101→107 resulta em MissingEvidence; derive_request formula103→107; após autorização de investigação/fonte ocorre uma única leitura real. A candidata não muda memória. Só após autorização e ato de admissão entra o fato12:103→107. Reconsulta produz TwoHop [11,12], sem inserir101→107. Fonte vazia conserva MissingEvidence; oposição previamente admitida103→107 negativa permanece após a nova observação e produz ContradictoryEvidence, sem escolha arbitrária. Duas execuções positivas em processos novos tiveram saídas e checkpoints idênticos; não demonstram determinismo universal.

**Testes finais:** novo `tests/mp07_investigation.rs`, público e autossuficiente, com15 testes. São verificados lacuna/pedido, consulta real, fonte negada sem execução, independência das permissões, bloqueio de contorno na CLI, suporte prévio, ambiguidade/oposição à base, ausência/irrelevância/invalidade, validade e proveniência temporal original, evidência contraditória, recusa de admissão, ausência de promoção/mutação indevida, capacidade/orçamentos, repetição/isolamento, persistência apenas materializada e não retomada de fonte. A referência simbólica independente recebe somente o fato inicial e as evidências efetivamente entregues; não recebe catálogo completo, gabarito ou consultas futuras.

Comandos executados em cópia isolada de fontes permitidas, com target/TMPDIR temporários e exclusão dos cinco módulos de testes reservados identificados no manifesto F1:

```sh
cargo check --offline --locked --lib --bin neurons
cargo build --offline --locked --bin neurons
cargo test --offline --locked --test cli --test engine --test interactions --test learning --test memory --test persistence --test personality --test f1_session --test mp06_paths --test mp05_synaptic --test mp07_investigation -- --skip bounded_update_probe --skip bounded_cost_probe_for_activation_and_communication
```

**106 aprovados:15 MP07 +15 MP05 +13 MP06 +63 F1; zero falhas, zero ignorados; dois probes não executados.** Não somar reexecuções intermediárias como testes independentes. Formatação dos quatro arquivos Rust envolvidos e git diff --check conferidos. Não houve execução de cargo test indiscriminado ou alteração de expectativas científicas para forçar aprovação.

**Recursos observados no binário final:** duas execuções CLI, três eventos de inferência por episódio,45.101 bytes de saída; parede9.313/7.818ms; CPU usuário6.522/3.151ms; CPU sistema0.000/3.151ms; pico RSS Linux15256/15464KiB incluindo inicialização. Método perf_counter_ns no pai + os.wait4 do filho; zero é a contabilização observada, não ausência de custo. Checkpoints312 bytes, SHA-256 `3b33bfacdcd639d5fb708a95810c44035dfc6d6e573ddfc6c1356c7e383f2e42`, idênticos entre repetições e após restauração. Não são prova de orçamento universal, tempo confiável ou heap vivo.

Evidências temporárias em `/tmp/neurons-mp07-xb_lk3k2/`: `commands.json`, `final-2.log`, `evidence.json`, `verified-1.input.txt`, `verified-1.output.txt`, `unavailable.log`, `opposition.log`. Sua retenção permanente não está demonstrada; resultados/limites essenciais ficam neste Plano Mestre. Nenhum documento de governança novo foi criado.

**Encerramento e riscos:** investigação é relacional, supervisionada e rasa, com fonte sintética definida pelo operador e sem autenticação real. A interface não prova independência adversarial do ambiente. Falta de evidência, contexto/tempo inválido e orçamento impedem conclusão, não autorizam ampliar busca. MP07 integral, persistência integral da investigação, MP08–MP10 e F2 integral permanecem fora deste resultado. As ressalvas F1 e as limitações publicadas de MP05/MP06 continuam abertas. Sem commit/push ou próximo intervalo automático; aguarda revisão do responsável.


### F2 — primeiro intervalo MP08: metacognição e autorregulação delimitadas

**Resultado: IMPLEMENTADO E VALIDADO no recorte autorizado.** Base `devs`, HEAD `ae00e8e5ba3e2ae43eacd5adbfce7113c8452c4d`. MP08 integral e F2 integral não estão concluídas. O aceite F1 e suas seis ressalvas, os registros MP05/MP06/MP07, formatos anteriores e restrições científicas permanecem preservados. Sem commit/push, MP09/MP10, CIC-I6/P8, G2/G7 ou acesso D/V/F.

**Integração:** `src/metacognition.rs` acrescenta RegulationDiagnostic, Strategy, GoalResult e a função pura select_strategy. O supervisor regulate, em `src/integration/investigation.rs`, coordena a Investigation existente pela porta selada AcquisitionHost. `src/session/f1.rs` implementa apenas a consulta adicional MP06 por QueryCoordinator::query_three, com PathAuthorization recebido, e a interface transitória. Novo teste público `tests/mp08_regulation.rs`. Nenhum segundo motor de raciocínio, investigação, plasticidade ou regra científica foi criado; nenhuma dependência externa foi introduzida.

**Política e evidência:** os diagnósticos vêm de QueryReport/PathQueryReport reais e preservam relatórios, provas, conflitos, recusas, exclusões, eventos e permissões recebidas. Precedência determinística: impedimento contratual, invalidez, oposição ou falha de propagação impedem resposta/continuação; depois limite; depois prova verificada; depois investigação somente com insuficiência, lacuna única derivada pelo M3 e permissões suficientes; demais situações mantêm abstenção. Diagnósticos históricos coexistem: MissingEvidence da consulta de dois passos não impede resposta quando MP06 autorizada encontra prova válida de três premissas. Não são probabilidades de confiança nem autoinspeção perfeita.

O relatório inicial é o da própria Investigation::begin, reutilizado para derivação; não se repete a consulta apenas para diagnosticar. MP06 só é consultada, sob autorização própria, quando ainda falta suporte e não há impedimento anterior. Consulta adicional consome contadores reais. O supervisor faz uma seleção, sem troca de estratégia ou investigação automática repetida. A porta M3 continua sem replay histórico durante supervisão.

**Permissões:** evaluation, mp06, investigation, source e admission são campos separados, fornecidos pelo chamador e vinculados ao escopo; fonte inclui SourceId e admissão inclui EngramId explícito. O supervisor somente materializa no MP07 as concessões já recebidas, nunca as inventa ou renova. Sem avaliação, não chama o núcleo; sem investigação/fonte válidas, não chama lookup. Fonte conhecida não equivale a autenticidade. Admissão ausente ou fora de escopo não incorpora a candidata. Ações externas estão incondicionalmente proibidas. Fonte é recebida já fixa antes da pergunta. Controles permanecem sintéticos e não demonstram autenticação ou isolamento operacional adversarial.

**Execução e resultado:** o recorte distingue strategy (Respond/Investigate/Abstain/StopAtLimit) de goal (Met/Partial/NotMet). Met exige prova final suficiente sem impedimento; executar lookup ou admitir observação não basta. Partial significa que a investigação delimitada foi executada/avaliada, mas não respondeu fundamentadamente ao objetivo; inclui ausência/irrelevância e recusa de admissão. Não é sucesso cognitivo ou aprendizagem comprovada. Retornos inválidos/quarentenados, oposição e erros ficam registrados. Sem incorporação, a reconsulta é somente leitura; não se simula admissão vazia. Nenhuma conclusão inferida é incorporada como fato e nenhuma observação é elevada implicitamente a ExplicitlyValidated.

**Limites fiscalizados:** duas avaliações metacognitivas, quatro estratégias possíveis, uma seleção, nenhuma troca, uma investigação/fonte/consulta/candidata, até uma admissão e oito fatos. Até três consultas cognitivas efetivas e96 eventos agregados, sem reiniciar contadores entre fases. Consulta legada/M3 conserva teto32; MP06 recebe no máximo64 e nunca mais que o saldo. Antes de uma consulta M3 reserva-se conservadoramente32 eventos, pois a API publicada não recebe teto menor. Se faltar essa reserva ou slot de consulta, encerra sem executar; reservation_blocked distingue esse bloqueio conservador de ResourceLimited observado no motor. Isso pode impedir uma consulta que hipoteticamente usaria menos eventos; não é alegação de exaustão física. Relatório valida os tetos em runtime. Histórico supervisor tem até oito entradas, além dos relatórios e do traço MP07 com capacidades próprias fixas. Não há escalada de orçamento, retry ou busca externa.

**Sessão:** preparar a fonte pelos comandos MP07 já publicados, antes da pergunta. `permissoes8 CONTEXTO AVALIAR(sim|nao) MP06(sim|nao) INVESTIGAR(sim|nao) FONTE_ID|- ADMISSAO_ID|-` fixa as concessões separadas; `regular8 ORIGEM DESTINO CONSULTAS EVENTOS` executa uma vez; `historico8` apresenta o relatório. O identificador de admissão representa uma autorização explícita antecipada para uma única observação admissível no episódio, não autorização implícita de investigar. Depois do relatório, comandos cognitivos paralelos são bloqueados até restauração/encerramento; leitura e salvamento permanecem disponíveis. Não se revogam APIs manuais F1 nem se declara proteção global contra um chamador hostil.

**Persistência:** histórico, seleção, fonte, autorizações e contadores MP08 são transitórios. Somente atos M3 já representáveis entram no journal F1 v1. Restaurar limpa controles MP08/MP07 e não consulta fonte. Não se promete retomada da supervisão, orçamento ou permissões. Formatos F1 v1 e MP05 v1 não foram alterados: checkpoints públicos anteriores de897/235 bytes foram restaurados e salvos com identidade byte a byte. O checkpoint materializado do episódio MP08 tem326 bytes; não contém o relatório supervisor inteiro.

**Demonstrações:** A→B admitido e consulta A→C geram MissingEvidence, pedido B→C, única leitura da fonte local, avaliação e admissão explícita; reconsulta com prova [11,12] atende o objetivo sem armazenar A→C. Sem admissão, ausência ou relação irrelevante, não há Met. Oposição permanece visível e impede sucesso; orçamento insuficiente encerra sem consultar fonte. Com três premissas já admitidas e PathAuthorization explícita, MP06 responde sem investigar. Duas execuções positivas em processos novos produziram saídas e checkpoints idênticos, sem alegação de determinismo universal.

**Validação pública final:**18 testes novos MP08 +63 F1 +15 MP05 +13 MP06 +15 MP07 = **124 aprovados, zero falhas e zero ignorados**. Dois probes permanecem excluídos, não contados como aprovados: bounded_update_probe e bounded_cost_probe_for_activation_and_communication. Nenhuma bateria reservada foi executada. Os18 testes cobrem política/precedência, lacuna única, permissões independentes, invalidade, oposição, ausência/irrelevância, sucesso versus execução parcial, orçamento agregado inclusive MP06, atomicidade de orçamento inválido, determinismo, restauração sem retomada e compatibilidade da sessão. Referência simbólica independente compara cinco decisões iniciais e seis objetivos finais usando somente fatos efetivamente admitidos/permissões/orçamentos, sem catálogo privilegiado da fonte: zero divergências/falsos sucessos/falhas de sucesso nesses casos finitos, sem estimativa estatística geral.

As duas primeiras rodadas MP08 tiveram12/14 e13/14 aprovações. Falhas de montagem dos testes: tentativa de substituir autorização já fixada, hipótese indevida de que uma reserva32 estaria indisponível com saldo32 e ciclo sem conexão ao alvo que não atingia limite. Corrigidos os cenários de teste conforme contratos existentes; não foram alterados critérios científicos, testes anteriores ou expectativas para ocultar falha de produção. A matriz final ampliada passou integralmente.

Comandos executados em cópia isolada apenas de fontes rastreadas de produção e testes públicos explicitamente selecionados; os módulos de testes reservados não foram copiados ou lidos:

```sh
cargo check --offline --locked --lib --bin neurons
cargo build --offline --locked --bin neurons
cargo test --offline --locked --test cli --test engine --test interactions --test learning --test memory --test persistence --test personality --test f1_session --test mp05_synaptic --test mp06_paths --test mp07_investigation --test mp08_regulation -- --skip bounded_update_probe --skip bounded_cost_probe_for_activation_and_communication
```

Formatação aplicável dos quatro arquivos Rust alterados/criados e git diff --check conferidos, sem declarar resolvida a ressalva global de formatação F1. Evidências e logs em `/tmp/neurons-mp08-9mnq5kky/`: `commands.json`, `regression-final.log`, `evidence.json`, `episode-3.input.txt`, `episode-3.output.txt`, `absent.log`, `opposition.log`, `limit.log`; retenção permanente não demonstrada. Nenhum novo plano ou documento de governança foi criado.

**Recursos observados no binário final:** parede 14.337/5.872ms; CPU usuário 4.172/0.000ms e sistema 0.000/4.796ms; pico RSS Linux 15164/15384KiB incluindo inicialização. Método perf_counter_ns + os.wait4 por processo; valores zero são contabilização observada. Checkpoint326 bytes, SHA-256 `b1a82c3c3217932399f46eade99f8be3572298384363615bdcaa4aa050f6ffe5`. Não são garantias de latência, memória universal ou desempenho geral.

**Encerramento:** somente este intervalo MP08 foi implementado e validado; F2 integral permanece não concluída. Permanecem as ressalvas F1, limitações MP05–MP07, fonte sintética supervisionada e ausência de autenticação real. Sem avanço automático, publicação ou nova fase.


### F2 — primeiro intervalo MP09: aprendizagem conceitual supervisionada

**Resultado: IMPLEMENTADO E VALIDADO no recorte autorizado.** Base `devs`, HEAD `b63361d38f2197283ae6031b55a5f79b0c0c4041`. Não encerra MP09 integral ou F2; não inicia MP10. Preservados aceite F1 e suas seis ressalvas, resultados MP05–MP08, protocolos científicos e proibições D/V/F, G2/G7 e CIC-I6/P8. Sem commit/push.

**Arquivos e reutilização:** `src/curriculum.rs` acrescenta TeachingPlan, Lesson, PedagogyAssessment, PedagogyPermissions, PedagogyReport e teach_concept. A porta PedagogyHost é selada e implementada somente por Session em `src/session/f1.rs`. `src/metacognition/concepts.rs` acrescenta diagnóstico puro a partir de ConceptReport/ConceptDecision reais. Novo teste público `tests/mp09_pedagogy.rs`. O Microcurriculum legado não é instanciado: memória admitida, versões e revisão permanecem exclusivamente no ConceptCoordinator existente. Planos didáticos e relatórios são entradas/rastros transitórios, não outra memória de inferência. Não houve mudança em H23, no indutor, em critérios científicos ou dependências.

**Ciclo e seleção:** uma avaliação inicial em lote identifica ExperienceNeeded por NoEvidence/InsufficientEvidence. Oposição, atributos incompletos ou classe inadequada bloqueiam ensino; suporte já suficiente não exige experiências. O seletor recebe somente a versão atual e até três experiências rotuladas de ensino previamente fixadas. Usa Criterion::evaluate existente para contar hipóteses que cada experiência eliminaria, sem filtrar/armazenar um modelo alternativo. Escolhe redução informativa máxima, preservando pelo menos uma hipótese; desempata por EngramId. A revisão efetiva continua exclusivamente no núcleo, após incorporação. Não usa rótulos ou observações de prova para escolher experiências. Não seleciona automaticamente exemplos que eliminariam toda a família, nem altera H23 para obter sucesso.

**Ensino e avaliação separados:** TeachingPlan contém apenas experiências didáticas; PedagogyAssessment contém observações sem rótulos, em conjuntos inicial/posterior distintos de um ou dois itens cada. Os rótulos de correção existem somente no harness público externo, não na interface do supervisor. Há bloqueio e registro leak_detected antes de qualquer consulta/ensino se um item de avaliação coincidir com ensino ou exemplos já admitidos, inclusive completamentos de atributos incompletos. IDs didáticos duplicados e itens de prova repetidos são recusados. Isso verifica sobreposição explícita e ausência do canal de gabarito nessa interface; não prova isolamento adversarial ou inexistência de qualquer canal externo. Progresso não é calculado/afirmado pelo supervisor sem chave: ele fornece os relatórios reais para correção independente.

**Permissões:** avaliação e apresentação são concessões separadas por Scope; admissão usa lista explícita de até três EngramId autorizados. Apresentar, interpretar e avaliar admissibilidade não autoriza incorporar. Sem concessão do ID selecionado, a apresentação termina sem admissão, não se escolhe outro ID mais conveniente e preservam-se memória/hipóteses. O journal/pendência de recepção pode mudar para registrar a apresentação, como no contrato F1; não se promete que todo estado de recepção permaneça idêntico. Fonte sintética conhecida/evidência identificada não significam autenticidade ou ExplicitlyValidated. Nenhuma ação externa, alteração sináptica ou criação de fatos inferidos.

**Limites:** um conceito/contexto, até três experiências, três admissões/revisões e oito registros supervisores. Uma avaliação inicial e uma posterior em lote, no máximo dois itens cada: até quatro consultas conceituais somente leitura, com IDs de relatório distintos1–4 dentro do episódio. Não há ciclos automáticos nem repetição após recusa. Antes da execução reserva-se conservadoramente capacidade para cinco operações/eventos por experiência: recepção, interpretação, admissibilidade, incorporação e revisão. Respeitam-se os tetos existentes de128 operações F1,128 eventos conceituais,16 exemplos e32 versões, além da validade temporal; o núcleo também reserva revisão antes de incorporar. Reservas conservadoras podem bloquear um plano mesmo que somente parte dele fosse útil. Consultas usam logs transitórios limitados e não alteram memória, modelos, journal ou relógio da sessão. Atos didáticos usam métodos existentes diretamente e registram os mesmos comandos F1 após sucesso, sem replay histórico durante ensino.

**Interface:** depois de criar/configurar o conceito existente, `plano9`, até três `licao9 ID RECIBO EXEMPLAR BITS ROTULO FONTE EVIDENCIA`, `avaliacoes9 PRE1 PRE2 POS1 POS2` e `fixar9` preparam entradas antes da execução. `permissoes9 CONTEXTO AVALIAR(sim|nao) APRESENTAR(sim|nao) ADM_IDS|-` fixa concessões independentes; IDs separados por vírgula. `ensinar9` executa uma vez e `historico9` expõe diagnóstico, permissões, experiências, evidências, admissões, versões e relatórios. Encerrado o episódio, permite leitura/salvamento/restauração, mas bloqueia novos atos cognitivos paralelos até restauração/encerramento. Formatos persistidos não incluem plano, avaliações ou permissões MP09.

**Experimento público fixado:** conceito opaco501, contexto41, catálogo91/regra51 v1. Estado inicial igual no tratamento e controle: exemplo401 `100001 +`, evidência2401, versão1 com quatro hipóteses remanescentes. Avaliação inicial: `100010 -`, `110001 +`; posterior separada: `010101 -`, `100011 +`. Os rótulos após os vetores são a chave EXTERNA do teste, não entrada do supervisor. Ensino disponível antes da execução: exemplo402 `000001 -` e403 `100000 -`, fonte7/evidências2402/2403. Nenhuma dessas observações coincide com os quatro itens de avaliação.

Diagnóstico inicial: InsufficientEvidence no item negativo e suporte no positivo; correção externa1/2. Selecionados402 e403, com reduções previstas2 e1, explicitamente admitidos. Revisões1→2→3, preservando versão0 inicial e todas as versões/evidências anteriores; três exemplos admitidos no total e uma hipótese remanescente. Avaliação posterior2/2. Controle idêntico, com apresentação negada, manteve versão1 e posterior1/2; não recebeu informação adicional. Mesma chave, dois itens por fase, sem alterar denominador ou critério. Referência simbólica separada usa tabelas booleanas por máscara e somente exemplos efetivamente admitidos, sem chamar family/revise/evaluate do núcleo ou receber gabarito futuro. Previsões posteriores coincidiram nesse cenário.

Isso demonstra progresso e retenção delimitados, não generalização: itens são inéditos em relação ao ensino, mas pertencem ao mesmo vocabulário/família finita. Não há teste de transferência não isomorfa, aprendizagem neural, português livre, autonomia pedagógica geral ou ligação entre MP05 e conceitos. Ensino rotulado é atribuído externamente e não autenticado.

**Persistência e repetição:** checkpoint F1 v1 materializa os mesmos atos lógicos já suportados; restauração reconstrói esse estado pelo replay legado, sem executar o plano MP09 ou iniciar novo ensino. Exemplos, proveniência, versões e previsões foram retidos em processo independente. Plano, permissões e relatório supervisor são transitórios e limpos ao restaurar; historico9 informa ausência de relatório. Não se promete retomada pedagógica. Duas execuções finais produziram saídas e checkpoints idênticos; checkpoint419 bytes. Arquivos públicos anteriores F1 v1 (897 bytes) e MP05 v1 (235 bytes) restaurados/salvos byte a byte idênticos. Sem alteração de formatos.

**Testes finais:**15 MP09 +124 anteriores (63 F1,15 MP05,13 MP06,15 MP07,18 MP08) = **139 aprovados; zero falhas, zero ignorados; dois probes excluídos** (bounded_update_probe e bounded_cost_probe_for_activation_and_communication). Os testes MP09 verificam progresso/controle, diagnóstico, seleção canônica, chaves externas, sobreposição, permissões/recusa, fonte inválida, contradição, insuficiência/atributos incompletos, não mutação, versões/evidências, identidade/capacidade, determinismo e restauração em processo independente. Todas as rodadas unitárias MP09 executadas passaram. Um primeiro harness de medição tentou consultas legadas após encerramento; ambas foram corretamente bloqueadas pela CLI. Corrigiu-se o harness para utilizar a avaliação posterior já produzida, sem alterar código/critério ou ocultar as recusas; saída preliminar preservada.

Comandos em cópia isolada somente de produção rastreada e testes públicos previamente selecionados, sem módulos/fixtures reservados:

```sh
cargo check --offline --locked --lib --bin neurons
cargo build --offline --locked --bin neurons
cargo test --offline --locked --test cli --test engine --test interactions --test learning --test memory --test persistence --test personality --test f1_session --test mp05_synaptic --test mp06_paths --test mp07_investigation --test mp08_regulation --test mp09_pedagogy -- --skip bounded_update_probe --skip bounded_cost_probe_for_activation_and_communication
```

Formatação aplicável dos quatro arquivos Rust envolvidos e git diff --check conferidos; permanece a ressalva global F1. Logs/evidências temporários: `/tmp/neurons-mp09-n0q8c39g/commands.json`, `final-2.log`, `evidence.json`, `episode-5.input.txt`, `episode-5.output.txt`, `control.output.txt`, `restore.output.txt`. Retenção permanente desses temporários não demonstrada. Nenhum novo plano ou documento de governança.

**Medições no binário final:** duas execuções CLI em processos novos; parede 20.485/5.114ms; CPU usuário 7.033/3.682ms e sistema 0.000/0.000ms; pico RSS Linux 15288/15496KiB, incluindo inicialização. Método perf_counter_ns + os.wait4; zero reflete contabilização observada. Checkpoint419 bytes, SHA-256 `40dc8a44c914c6a77b53a49b787b847bf9538fbb78079ec9d38163e6d43fbb71`. Sem garantia universal de tempo/memória ou determinismo.

**Encerramento:** concluído tecnicamente somente o primeiro intervalo MP09. F2 integral não concluída; MP10 não iniciada. Sem commit/push, acesso reservado ou ampliação de outros intervalos. Permanecem todas as ressalvas e limitações anteriores.


### F2 — primeiro intervalo de supervisão de objetivo conceitual (MP08/MP09)

**Resultado: IMPLEMENTADO E VALIDADO no recorte autorizado.** Base `devs`, HEAD `ab23612e6d06b49ea95096cc95406dd3211e957e`. Integra autorregulação e ensino conceitual existentes, sem encerrar F2 integral, iniciar MP10 ou ligar conceitos a premissas relacionais/pesos MP05. Preservados aceite F1 e suas seis ressalvas, resultados MP05–MP09, H23 e critérios científicos. Sem commit/push, acesso D/V/F, G2/G7 ou CIC-I6/P8.

**Arquivos e contratos:** `src/curriculum.rs` acrescenta ConceptGoal, GoalCriterion, ConceptGoalBudget, GoalQuery, GoalLesson e ConceptGoalReport; função supervise_concept_goal. Objetivo tipado contém ID não nulo, conceito, Scope, observação original e critério SupportedByCurrentEvidence, sem rótulo esperado. Todos os relatórios de consulta/experiência são vinculados ao mesmo objetivo, inclusive avaliações independentes. Consulta inicial precede qualquer apresentação; diagnóstico e seleção são registrados antes do ensino. `src/metacognition/concepts.rs::select_concept_strategy` adapta ConceptReport/diagnose_pedagogy à precedência pura `metacognition::select_strategy` MP08. Respond/Abstain/Stop conservam significado; a ação exploratória é adaptada a TeachOnce MP09, sem executar MP07 ou transferir concessões relacionais. Não foi criado outro motor, memória ou agente.

A sequência de escolha/apresentação/incorporação/revisão MP09 foi extraída para run_teaching e reutilizada por teach_concept e pelo novo supervisor, conservando a API e o comportamento legado. Escolha didática permanece por redução informativa entre hipóteses atuais, com desempate canônico, sem consultar rótulos de prova. PedagogyHost segue selada, implementada por Session. Acrescentou-se somente leitura da contagem de eventos conceituais para medição agregada. Memória, evidências e hipóteses permanecem no ConceptCoordinator existente. Falhas de admissão são registradas juntamente com as experiências anteriores já autorizadas, sem ocultar execução parcial.

**Escolha e resultados:** suporte suficiente permite responder diretamente em uma consulta, sem ensino ou reconsulta duplicada; nesse caso final_query é None e a base da resolução é o relatório inicial. Insuficiência com experiência informativa e permissões explícitas permite uma única tentativa de ensino. Oposição/inadequação/atributos incompletos mantêm abstenção; ausência de capacidade encerra. São distintos execução (NotStarted/Completed/Interrupted) e objetivo (Resolved/Partial/Abstention/Interrupted). Resolved exige ConceptDecision::Supported e versão com hipóteses/evidências admitidas e registros consultados; é sustentação provisória pelo modelo atual, não acurácia ou verdade externa. Partial exige revisão efetiva sem resolução do item. Recusa de permissão ou falha interrompe, mesmo que uma experiência anterior autorizada tenha produzido progresso.

**Permissões e estado:** avaliação, apresentação e admissões específicas por ID continuam separadas em PedagogyPermissions. Sem autorização de apresentação ou da primeira admissão selecionada, não apresenta experiências e preserva o estado inteiro da sessão. Não troca a experiência por um ID mais conveniente. Se uma segunda admissão for negada, somente a primeira incorporação autorizada permanece: o exemplo recusado pode ter recepção/pendência no journal F1, mas não entra na memória/versões. Fonte desconhecida é quarentenada. Nenhuma classificação é promovida a fato relacional, nenhum feedback positivo concede autorização, nenhuma alteração sináptica ou ação externa é executada.

**Objetivo versus avaliação:** a observação original é consultada inicialmente e reconsultada após revisão com o mesmo ID e conteúdo. Essas duas consultas são distintas dos conjuntos MP09, que continuam separados e não admitem repetição do mesmo item. O objetivo não pode ser inserido nesses conjuntos nem ser uma experiência didática. Também são bloqueadas sobreposições dos itens independentes com ensino/estado já admitido. Rótulos de correção não existem na API do supervisor, seletor ou estado cognitivo: somente o harness externo corrige os resultados. Barreiras detectam sobreposição explícita, não pretendem provar ausência de todos os canais adversariais possíveis.

**Orçamentos fiscalizados:** um objetivo/conceito/contexto, uma decisão e uma tentativa; até duas experiências, admissões e revisões. Até cinco consultas no total: inicial do objetivo + itens independentes iniciais + itens independentes posteriores + reconsulta do objetivo. Consulta inicial é reaproveitada para diagnóstico/seleção. Antes de ensinar, reserva-se o total necessário; conjuntos independentes2+2 exigiriam seis consultas e são bloqueados neste recorte, sem contornar sua separação. Cada conjunto continua com até dois itens; combinações1+1 usam quatro consultas e2+1 usam cinco. Contadores não são reiniciados. Histórico supervisor até oito entradas; máximo de dez eventos conceituais novos, além dos tetos MP09/F1 existentes de eventos, exemplos, versões, operações e validade. Eventos conceituais não são eventos neurais: não se executa inferência relacional/Engine neste episódio e não se amplia o orçamento MP08. Reservas conservadoras podem impedir continuação antes de esgotar fisicamente recursos.

**Interface F1:** `objetivo9 ID BITS` fixa identidade/item sem rótulo. Reutiliza plano9/licao9/fixar9/permissoes9; `avaliacoes-objetivo9 PRE POS` prepara dois conjuntos independentes com um item cada, sem rótulos. `acompanhar9 CONSULTAS EXPERIENCIAS` executa e `historico-objetivo9` apresenta o relatório. Enquanto um objetivo está definido, ensinar9 não pode contornar sua supervisão. Após encerramento, não há repetição automática ou novo ato cognitivo paralelo, preservando leitura/salvamento/restauração. Controles e objetivo ativo são locais ao runner, não novos campos persistidos da Session.

**Experimento público observado:** conceito501/contexto41, catálogo91/regra51 v1; seed401 `100001 +` admitido com evidência2401, versão1. Objetivo9001: classificar `100010`, sem rótulo enviado. Inicial: InsufficientEvidence, quatro hipóteses remanescentes. Plano previamente fixado:402 `000001 -` e403 `100000 -`, fonte7/evidências2402/2403. Seleção TeachOnce anterior às experiências; duas admissões explícitas e revisões1→2→3; hipóteses4→2→1. Reconsulta do mesmo objetivo9001/item100010, com três exemplos/evidências, produz Supported(Negative). Resolved é conferido pelo critério do modelo; o rótulo negativo correto é conhecido somente pelo teste externo.

Controle com mesmo estado/objetivo e apresentação negada não recebeu ensino; estado permaneceu byte a byte igual e nova avaliação externa do objetivo continuou InsufficientEvidence. Avaliações independentes: pré `110001` (chave externa positiva) e pós `010101` (chave externa negativa), ambos corretos no tratamento; controle posterior permaneceu sem suporte. São1/1 por conjunto, não taxa geral de aprendizagem; o avanço central é a resolução do objetivo antes insuficiente. Com apenas a primeira experiência há revisão, mas o objetivo permanece insuficiente: Partial, sem alegar sucesso. Oposição inicial mantém Abstenção; falta de autorização/orçamento produz Interrupted. A referência simbólica independente usa somente os exemplos presentes em consulted de cada relatório, sem catálogo didático completo ou gabarito futuro, e coincide com as decisões nos casos públicos comparados.

**Retenção:** conhecimento usa exclusivamente comandos lógicos F1 já representáveis; formato v1 inalterado. Objetivo ativo, escolha, concessões, plano, avaliações e contadores não são persistidos. Restauração reconstrói exemplos/versões pelo replay legado de dados admitidos, sem executar o plano didático. Em processo novo, historico-objetivo9/acompanhar9 sem nova configuração recusam; depois de definir explicitamente objetivo9002 para o mesmo item100010 e nova autorização somente de avaliação, responde diretamente na versão3, sem ensino. Checkpoint após essa consulta autorizada é idêntico ao restaurado. Não há retomada automática do objetivo9001. F1 v1 público anterior (897 bytes) e MP05 v1 (235 bytes) também foram restaurados e salvos com identidade byte a byte.

**Validação final nova:**17 testes em `tests/concept_goal_supervision.rs` +139 regressões existentes = **156 aprovados, zero falhas, zero ignorados; dois probes excluídos**, bounded_update_probe e bounded_cost_probe_for_activation_and_communication. Não contar reexecuções como testes novos. Casos agrupam identidade/ordem/decisão, resposta direta, permissões/controle, fonte/contexto inválidos, oposição, parcial versus resolução/interrupção, proveniência/versões, barreiras de avaliação, chave externa, orçamento agregado inclusive teto cinco, determinismo, falha de admissão rastreável, persistência e nova autorização em processo independente. Todas as rodadas executadas passaram; não foram alterados testes anteriores, H23 ou critérios para forçar resultado.

Comandos executados em cópia isolada somente de produção rastreada e testes públicos explicitamente permitidos; módulos/fixtures reservados não foram copiados como testes nem executados:

```sh
cargo check --offline --locked --lib --bin neurons
cargo build --offline --locked --bin neurons
cargo test --offline --locked --test cli --test engine --test interactions --test learning --test memory --test persistence --test personality --test f1_session --test mp05_synaptic --test mp06_paths --test mp07_investigation --test mp08_regulation --test mp09_pedagogy --test concept_goal_supervision -- --skip bounded_update_probe --skip bounded_cost_probe_for_activation_and_communication
```

Formatação aplicável dos quatro arquivos Rust envolvidos e git diff --check conferidos, sem declarar resolvida a ressalva global F1. Evidências temporárias em `/tmp/neurons-concept-goal-fkihhj6a/`: `commands.json`, `final-2.log`, `evidence.json`, `episode-1.input.txt`, `episode-1.output.txt`, `control.output.txt`, `control-requery.output.txt`, `restore.output.txt`. Retenção permanente dos temporários não está demonstrada; resultados essenciais ficam neste Plano Mestre. Nenhum documento administrativo ou plano concorrente foi criado.

**Recursos realmente observados:** duas execuções CLI novas, quatro consultas e dez eventos conceituais por episódio; parede 13.769/4.804ms; CPU usuário 3.631/0.000ms e sistema 0.000/3.571ms; pico RSS Linux 15456/15544KiB incluindo inicialização. Método perf_counter_ns + os.wait4; zero reflete contabilização observada. Saídas/checkpoints idênticos entre repetições; checkpoint419 bytes, SHA-256 `40dc8a44c914c6a77b53a49b787b847bf9538fbb78079ec9d38163e6d43fbb71`. Não são garantias universais de latência, memória ou determinismo.

**Limitações e encerramento:** supervisão finita, fontes e autorizações sintéticas, classificação provisória da família existente. Sem aprendizagem neural, transferência ampla, português livre, autoatribuição de autoridade ou integração MP09→MP05/MP06. F2 integral permanece não concluída; MP10 adiada. Encerrado somente este intervalo, sem commit/push ou próxima etapa automática.


### F2 — consulta composta vinculada e transitória (MP09 + MP06), validação de 2026-11-10

**Resultado: IMPLEMENTADO E VALIDADO exclusivamente no intervalo experimental autorizado.** Base `devs`, HEAD `cc63baa3b3bf665360f5a2b081a46b901d3be527`; alterações permanecem na árvore, sem commit/push. Consultadas as seções 1–2, 4, MP06/MP09 da seção 5, restrições das seções 6–8 e contratos/resultados da seção 9. Este registro acrescenta evidências à seção 9, preservando decisões anteriores, aceite F1 e suas seis ressalvas. F2 integral não concluída; MP10 permanece adiada.

**Contrato e semântica:** `src/integration/composed.rs`, exportado por `src/integration.rs`, introduz Binding, SyntheticRegistry, Permissions, Dependencies, Usage/Limits, Report, Verification e Episode. Duas portas internas somente de leitura em `src/session/f1.rs` reutilizam as fontes, regra e escopo reais da Session e o QueryCoordinator existente. A condição composta é exclusivamente a conjunção de classificação conceitual positiva sustentada e relação independente C⊆D elegível, não contradita e verificada. Não afirma automaticamente x∈D, não cria Engram, não usa ConceptReport como premissa e não altera verify_path. Não foi criado outro indutor, memória, motor ou checkpoint.

Binding identifica objetivo, item, observação/atributos, conceito, contexto, entidade relacional, destino e proveniência. SyntheticRegistry é uma tabela imutável cuja confiança é premissa delimitada do experimento, fornecida independentemente do pedido no harness. Coincidência numérica de IDs não estabelece equivalência: conceito501 é vinculado explicitamente à entidade101. Identidade declarada e atributos são confrontados com a tabela autorizada; isso não autentica uma pessoa, sensor ou identidade real. As três permissões (conceitual, relacional e composição exata) são independentes; negação não executa consultas nem revela snapshot de dependências.

**Integridade e atualidade:** query reutiliza ConceptCoordinator e query_three, verify_path e monitor de oposição existentes. Suporte exige classificação positiva, prova de uma premissa neste recorte e ausência de recusa/oposição impeditiva. Evidências compartilhadas entre as dependências e a origem do vínculo são conservadoramente recusadas; origem baseada na própria composição é negada. Report contém julgamentos, versões, provas, permissões, consumo e dependências transitórias. check_current confere recibo exato emitido pelo episódio, vínculo/revisão da tabela e snapshot lógico atual da Session; adulteração é rejeitada. Qualquer mudança relevante representada no snapshot exige reavaliação conservadora, inclusive mudança de versão com classificação igual. Não há invalidação global automática. Os estados Supported/Insufficient/Contradictory/Denied/Interrupted distinguem sustentação, insuficiência, oposição, negação e limite; ReevaluationRequired pertence à conferência posterior de atualidade.

**Conferência:** Episode.verify não aceita somente o recibo: reexecuta os coordenadores sobre o estado admitido, compara resultados e consome o orçamento compartilhado. O verificador público reference_verify em `tests/composed_query.rs` enumera independentemente a família simbólica de 23 hipóteses a partir dos exemplos admitidos, confere atributos/vínculo/versão e calcula relações/oposição com as mesmas evidências elegíveis; também confere a prova MP06. Não recebe gabarito futuro nem usa o resultado composto como evidência. Esta referência de teste é uma computação simbólica finita, sem executar consultas do Engine; sua CPU/tempo são medidos separadamente. Ela não substitui nem contorna as consultas cobradas por Episode.verify em produção. Conferência semântica não equivale a autenticação operacional.

**Experimento observado:** item9010 com atributos `100101`, conceito501/contexto41, vínculo800 e objetivo9001; exemplos iniciais `111111 +` e `000001 -` admitidos, relação101→103 independente já admitida. Consulta inicial: Abstain(InsufficientEvidence), versão2, composição Insufficient. Ensino previamente fixado `100001 +`, com apresentação e admissão explícitas pelos mecanismos MP09: revisão para versão3. Reconsulta: Supported(Positive) e composição Supported, confirmadas por referência e verificador de produção. Memória relacional e metadados permaneceram inalterados. Controle sem ensino permanece insuficiente; controle sem relação também. Revisão adicional autorizada `100000 -`: versão4, classificação ainda positiva; relatório versão3 exige ReevaluationRequired e nova consulta sustenta a conjunção na versão4. Dados e critérios não foram ajustados para obter sucesso.

**Controles públicos aprovados:** oposição conceitual e relacional; permissões negadas independentemente; vínculo/atributos/mapeamento/contexto incompatíveis; relatório conceitual, campos de auditoria e prova adulterados; versão ou revisão da tabela obsoleta; mudança de tempo, elegibilidade e nova oposição; origem circular/evidência compartilhada; classificação negativa; duas premissas fora do recorte; reservas e orçamento excedidos; ausência de promoção factual; determinismo e isolamento. Recusa de ensino/admissão preserva o conhecimento; reservas conservadoras em falhas não são reiniciadas ou devolvidas implicitamente.

**Orçamento efetivamente fiscalizado:** oito consultas conceituais, três relacionais, 32 eventos neurais por consulta e 96 agregados, sem modificar o teto geral MP06 de64. Execução principal, ensino e conferência de produção juntos consumiram cinco consultas conceituais, três relacionais, cinco eventos conceituais e zero neurais: a relação direta foi recuperada sem propagação neural. Zero não comprova custo nulo. No desafio cíclico sintético separado, três consultas atingiram exatamente32 eventos cada, total96, e a quarta foi interrompida sem consumir mais. O desafio de revisão consumiu cinco consultas conceituais, três relacionais e dez eventos conceituais. Contabilidade conceitual permanece separada; limites e reservas abrangem as fases do mesmo Episode.

**Persistência comprovada:** somente conhecimento já representável em F1 v1 é salvo. Relatório, vínculo transitório, permissões de composição e Episode ativo não integram o checkpoint. Duas execuções independentes produziram checkpoints de549 bytes idênticos; restauração em novos processos e nova consulta autorizada preservaram os mesmos bytes, sem ensino ou retomada automática. Sem novas concessões, a consulta retornou Denied com consumo zero; após concessões explícitas, Supported na versão3. O teste de round-trip existente foi estendido para executar também um filho independente, sem aumentar artificialmente a contagem de testes. O checkpoint público F1 anterior de897 bytes (SHA-256 `053cd611f7ed562f41ff1c3fffbad818f9861c70b9bb5d0f4de2837661ff0916`) e MP05 v1 de235 bytes (`0f7ba1a015de09d42f3ab77c7eb27ecab24860c87a3a16e1b528ae3fced821ff`) foram restaurados e salvos com identidade byte a byte. Isso demonstra compatibilidade desses exemplares e preservação do código de formato, não uma prova universal de todos os estados possíveis.

**Validação integral executada em cópia isolada:** 17 testes compostos +156 regressões = **173 aprovados, zero falhas, zero ignorados**. Regressões:63 F1,15 MP05,13 MP06,15 MP07,18 MP08,15 MP09 e17 supervisão de objetivo. bounded_update_probe e bounded_cost_probe_for_activation_and_communication permaneceram excluídos (dois filtrados, não aprovados). Não foram copiados/executados módulos de testes reservados. Comandos reais:

```sh
cargo check --offline --locked --lib --bin neurons
cargo build --offline --locked --bin neurons
cargo test --offline --locked --test cli --test engine --test interactions --test learning --test memory --test persistence --test personality --test f1_session --test mp05_synaptic --test mp06_paths --test mp07_investigation --test mp08_regulation --test mp09_pedagogy --test concept_goal_supervision --test composed_query -- --skip bounded_update_probe --skip bounded_cost_probe_for_activation_and_communication
rustfmt --edition 2021 --check src/integration.rs src/integration/composed.rs src/session/f1.rs tests/composed_query.rs
git diff --check
```

Compilação, formatação aplicável e diff aprovados. Não se declara resolvida a ressalva histórica de formatação global F1. verify_path, H23, critérios congelados, formatos F1 v1/MP05 v1 e testes anteriores permanecem inalterados.

**Recursos medidos:** harness público temporário, mesmos helpers e referência dos testes, processos isolados; perf_counter_ns/os.wait4 e Instant por fase. Duas execuções completas: parede19.407/16.090ms, CPU usuário6.776/11.104ms, sistema3.388/3.701ms, pico RSS15384/15384KiB. Dois processos de restauração, nova consulta e conferência: parede10.717/10.325ms, CPU usuário3.100/4.491ms, sistema6.201/4.491ms, RSS15384/15384KiB. Incluem inicialização e I/O; não são limites garantidos. Na primeira execução, setup2.877ms, consulta inicial0.333ms, ensino0.194ms, reconsulta0.294ms, referência0.292ms, verificação de produção0.535ms, gravação9.425ms. No primeiro processo de retomada, load1.643ms, nova consulta0.683ms, referência0.667ms, verificação1.212ms. CPU/RSS por fase não disponíveis; baseline de inicialização em processos separados foi observado, sem subtração para alegar custo líquido.

**Evidências e limitações:** logs, comandos, relatórios, medições, checkpoints e hashes em `/tmp/neurons-composed-stage3-d06rgch1/` (commands.json, final-2.log, resources.json, run-1.json, restore-1.json, version.json, golden-checkpoints.json); retenção permanente desses temporários não demonstrada. Resultados essenciais ficam neste registro. Demonstra-se uma conjunção transitória delimitada, com tabela sintética confiada e classificação provisória da família existente; não identidade real, verdade absoluta, generalização ampla, aprendizagem neural ou integração MP09→MP05. Consulta composta disponível pela API Rust da Session, sem novo comando CLI dedicado. Não há invalidação global nem retomada automática de concessões/supervisão. Mantidos D/V/F, G2/G7 e CIC-I6/P8 fora do escopo. Encerrada a validação deste intervalo; publicação depende de autorização específica, F2 integral permanece pendente.


### F2 / TCR-01 — transferência conceitual, composição e retenção — 2026-11-10

**Resultado científico: H-TCR-01 sustentada exclusivamente no recorte sintético aprovado, em duas repetições independentes.** Base anterior à publicação: `devs`, HEAD `ef275bde3f6e563d70dd240c6dd04677b3b3dc8e`. Único artefato novo: `tests/tcr01_transfer.rs`; produção, H23, verify_path e formatos F1 v1/MP05 v1 não foram modificados. F2 integral permanece incompleta; MP10 e integração MP05 permanecem fora deste intervalo.

**Hipótese e domínio congelados:** ensino conceitual supervisionado melhora cobertura/correção em itens inéditos utilizados em consultas compostas, sem erros sustentados, conservando o benefício após restauração em outro processo. Regra externa `f(x)=b2∧b5`, seis atributos booleanos; conceito1501, contexto141, catálogo191, regra151 v1; vínculo sintético explícito com entidade relacional1101 e relação independente1101→1103. O avaliador aplica a regra externa; o candidato não recebe a chave dos itens de avaliação. Classificação negativa não é negação relacional; composição sustentada é somente conjunção verificada, sem admissão automática de x∈D.

| Partição | Itens/atributos e rótulos externos |
|---|---|
| P0 — inicial | S01 `111111 +`; S02 `000000 -` |
| P1 — ensino | L01 `101111 -`; L02 `111101 -` |
| P2 — transferência | E01 `010010 +`; E02 `110011 +`; E03 `011110 +`; E04 `111010 +`; E05 `010000 -`; E06 `001010 -`; E07 `101100 -`; E08 `100111 -` |
| Auxiliares MP09 | A01 `101010 -` inicial; A02 `110110 +` posterior; enviados ao ensino sem rótulos |
| P3 — retenção | Reavalia E01–E08 após restauração; não constitui nova amostra independente |

P0/P1/P2/auxiliares são disjuntos; nenhum E01–E08 é admitido. Critérios prospectivos: ganho conceitual correto ≥4/8 e composto correto ≥2/8 sobre C0; zero erros sustentados; relação necessária; autorizações respeitadas; retenção integral; orçamento preservado. Não foram ajustados dados, partições ou critérios após resultados.

**Correção metodológica e controles:** a execução anterior em `/tmp/neurons-tcr01-0txh4qv4/` permanece **experimento inválido**, pois ensinou duas vezes, totalizando quatro admissões novas por repetição. Seus resultados não integram esta confirmação. Na execução corrigida, K0 contém P0 e nenhum fato relacional. Reconstruções equivalentes preparam T inicial e C0 com relação. Uma única operação `teach_concept` em reconstrução independente de K0 admite L01/L02; K1 é materializado e restaurado em dois estados independentes. A relação é admitida somente em T; C1 preserva o mesmo conhecimento conceitual sem relação. C2 nega cada uma das três permissões; C3 restaura T em outro processo e exige novas concessões; C4 avalia separadamente os exemplos ensinados. São estados bifurcados controlados, não uma alegação de sessão única ininterrupta.

**Integridade observada:** T inicial e C0 são byte a byte equivalentes. T e C1 têm iguais identidade conceitual, memória, exemplos/proveniência, modelos, versões, critérios, suporte, eliminações e traço. Admitir a relação em T preserva essa projeção. C1 tem memória e metadados relacionais vazios antes e depois das consultas; T contém a relação independente admitida. O harness registra IDs reais das admissões do relatório pedagógico: **1403 e1404, exatamente duas por repetição**, sem inferência por divisão de contagem de eventos. Replay reconstrói conhecimento histórico e não executa novo ensino; seu custo é registrado separadamente.

**Resultados nas duas repetições:** inicialmente T/C0 abstiveram-se em8/8. Ensino revisou versões2→3→4. T classificou corretamente os oito itens inéditos: positivos E01–E04 e negativos E05–E08. Composição sustentada somente em E01–E04. C0 permaneceu em abstenção8/8. C1 conservou oito classificações corretas e zero composições sustentadas. C2 bloqueou24 tentativas (oito por permissão negada), sem consultas cognitivas. C4 classificou corretamente2/2 exemplos ensinados.

Cobertura conceitual correta T=8/8, C0=0/8; **ganho8/8**. Acurácia entre classificações sustentadas T=8/8; C0 não avaliável por denominador zero. Composições corretas T=4/8, C0=0/8; **ganho4/8**, cobertura dos positivos4/4. Zero erros sustentados conceituais ou compostos. A regra externa e a referência simbólica independente concordaram; referência recalcula hipóteses a partir de exemplos admitidos, confere vínculo, elegibilidade, prova e oposição. Teste adicional rejeita relatório adulterado. Os ganhos ocorrem em P2 e não dependem de contar P1 como transferência.

**Retenção:** C3 reproduziu integralmente decisões, versões e evidências de T, sem novo ensino. Sem concessões novas, consulta negada com consumo zero. Checkpoint corrigido de671 bytes, SHA-256 `c4007f533fb4d37808840caf4b071551c3df6606677052b7a7dc5576431df1e9`, idêntico entre repetições e após restauração. Relatórios compostos, concessões e episódios ativos não são restaurados como autoridade. Exemplares anteriores F1 v1 de897 bytes e MP05 v1 de235 bytes também foram restaurados/regravados com identidade byte a byte nesta publicação.

**Orçamento:** por repetição completa,100 consultas conceituais e96 relacionais, incluindo conferência; zero eventos neurais na relação direta. São48 consultas compostas iniciais e48 conferências de produção, além de duas avaliações didáticas e duas consultas C4. Cada episódio mantém limites8 conceituais/3 relacionais/32 eventos por consulta/96 agregados. Tetos globais120/120/3840 preservados, sem reiniciar contadores entre processos. Uma operação pedagógica, duas admissões novas e dez eventos conceituais novos; inicialização11 eventos. Cinco reconstruções explícitas no processo principal recompõem75 eventos históricos e uma em C3 recompõe21; não são novas experiências. Replays internos de Session::execute não são enumerados como novas admissões; seu custo integra as medições do processo.

**Revalidação para publicação:**176 testes públicos aprovados (173 regressões +3 testes TCR-01), zero falhas e zero ignorados; bounded_update_probe e bounded_cost_probe_for_activation_and_communication continuam excluídos. Compilação offline com dependências bloqueadas, formatação aplicável e whitespace aprovados. Seleção explícita, em cópia isolada de produção/testes públicos, sem módulos ou fixtures reservados:

```sh
cargo check --offline --locked --lib --bin neurons
cargo build --offline --locked --bin neurons
cargo test --offline --locked --test cli --test engine --test interactions --test learning --test memory --test persistence --test personality --test f1_session --test mp05_synaptic --test mp06_paths --test mp07_investigation --test mp08_regulation --test mp09_pedagogy --test concept_goal_supervision --test composed_query --test tcr01_transfer -- --skip bounded_update_probe --skip bounded_cost_probe_for_activation_and_communication --nocapture
rustfmt --edition 2021 --check tests/tcr01_transfer.rs
git diff --check
```

**Recursos da revalidação:** harness usa Rust e supervisor Python3/biblioteca padrão em Linux, com timeout10s e limiar RSS256MiB, incluindo conferência de pico por wait4; nenhum limite acionado. Processo experimental R1/R2: parede89.778/121.364ms, CPU usuário67.741/106.801ms e sistema11.290/4.272ms, pico RSS11364/11512KiB. C3 em outros processos: parede23.497/30.995ms, usuário16.441/12.293ms, sistema4.110/8.195ms, RSS11468/11496KiB. Incluem inicialização e I/O; não são garantias universais. Tempos por consulta, referência, conferência, ensino e replay constam dos artefatos; CPU/RSS não são decompostos por operação.

**Reprodução e limitações:** dados, regra externa, construção de controles e referências permanecem no harness versionável; resultados essenciais ficam neste registro. Evidências brutas da publicação em `/tmp/neurons-tcr01-publish-0y090w29/` (commands.json, validation-2.log, experiment/repeat-1 e repeat-2, CSVs, protocol-validity.json, assessment.json, signatures, checkpoints e resources.json). Retenção permanente desses temporários não comprovada. Os resultados confirmam transferência limitada dentro da família fornecida, em oito itens de um domínio pequeno; duas repetições não aumentam a amostra independente. Vínculo sintético não autentica identidade real. Sem aprendizagem neural, generalização ampla ou autonomia geral. Preservados aceite F1/seis ressalvas, restrições D/V/F, G2/G7, CIC-I6/P8, critérios científicos e checkpoints. A publicação encerra somente TCR-01; não inicia outro intervalo nem conclui F2 integral.


### F2 — formalização do escopo e preparação do próximo intervalo — 2026-11-13

**Mandato exclusivamente documental encerrado:** leitura integral do Plano Mestre, inspeção estática dos contratos de produção permitidos e conferência Git; nenhuma implementação, teste, compilação ou experimento novo. Seções 8.2 e 8.3 acrescentadas; registros anteriores mantidos integralmente. O cabeçalho e os estados históricos de autorização descrevem seus respectivos mandatos, não anulam os intervalos F2 posteriores registrados nesta seção.

**Base verificada:** repositório softstrucs/neurons, branch devs, HEAD `7c12e22fa74b1239858a467544008bf44e9a940f`, igual ao esperado; índice sem diferenças e árvore rastreada inicialmente sem alterações. Inventário inicial: 285 caminhos rastreados, 450 não rastreados e 702 ignorados. TCR-01 está no commit HEAD (Plano Mestre e tests/tcr01_transfer.rs); origin/devs local e consulta somente leitura git ls-remote origin refs/heads/devs confirmam o mesmo hash remoto. Isso verifica publicação do commit, não reexecuta seus 176 testes ou autentica resultados brutos. Não houve fetch, sincronização, commit ou push.

**Resultado:** contrato de conclusão F2 e matriz de 15 critérios registrados; próximo intervalo especificado com APIs existentes, impedimentos mínimos e controles prospectivos. F2 integral permanece NÃO CONCLUÍDA e sem aceite específico. Preservados F1/seis ressalvas, ocorrência histórica de acesso D, experimento TCR-01 inválido anterior, resultados negativos, limitações de logs temporários e critérios científicos congelados. Sem acesso D/V/F, execução CIC-I6/P8 ou G2/G7, probes, alteração de H23/verify_path/checkpoints, início MP10 ou integração MP05. Somente este Plano Mestre pode apresentar diferença rastreada ao fim; índice e conjuntos de caminhos não rastreados/ignorados devem permanecer idênticos. Próxima ação depende de autorização executiva própria; este registro não a concede.
