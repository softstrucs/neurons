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

**Estado:** proposto, não iniciado. **Autorização de execução:** ausente nesta atividade.

**Objetivo e pré-requisitos:** MP02/MP06 e mecanismo de atualização pertinente MP03/MP05; fontes sintéticas previamente autorizadas.

**Arquivos/módulos:** integration/acquisition.rs, cognition.rs, metacognition.rs, episódios e adaptador de sessão.

**Implementação necessária:** Selecionar pedido/intervenção finita com motivo de lacuna e registrar escolha, execução autorizada, retorno, incorporação e efeito. Evitar simulador de investigação que apenas imprime intenção.

**Testes e evidências:** Ausência/irrelevância/oposição de retorno, limites, uma ação sem mandato recusada; comparar política fixa/literal/aleatória apenas com protocolo apropriado.

**Critério de conclusão:** Traço causal separa decisão de investigar, ação efetuada e mudança de estado; benefício medido somente quando comparável e sustentado.

**Avanço automático no intervalo autorizado:** Sem novo prompt para rotinas de fonte/ação já expressamente delimitadas, mantendo contadores e parada.

**Intervenção humana / interrupção:** Rede, ferramenta ou ação externa não autorizada; novos retries abertos; dados reservados ou fonte confiada por ID.

### MP08 — Metacognição útil e comunicação honesta

**Estado:** proposto, não iniciado. **Autorização de execução:** ausente nesta atividade.

**Objetivo e pré-requisitos:** MP02/MP06; MP07 se regulando investigação real.

**Arquivos/módulos:** metacognition.rs, metacognition/concepts.rs, personality.rs, session.rs.

**Implementação necessária:** Regular resposta, revisão e abstenção sobre evidência acessível, separar estado deliberativo/avaliabilidade/execução quando pertinente sem aprovar política G2. Estilo nunca altera verdade ou autorização.

**Testes e evidências:** Casos positivos, contraditórios, insuficientes e custos; falsos alarmes/falhas de detecção; controle com mesma informação. Calibração probabilística apenas se houver probabilidades justificadas.

**Critério de conclusão:** Decisão e motivo corretos no escopo definido; utilidade incremental não presumida. Sem alegação de autoinspeção perfeita ou consciência.

**Avanço automático no intervalo autorizado:** Prossegue dentro do recorte autorizado sem elevar a capacidade fora do protocolo.

**Intervenção humana / interrupção:** Falta de referência independente, confiança estatística inventada, risco crítico oculto ou opinião persuasiva substituindo evidência.

### MP09 — Pedagogia progressiva e linguagem pt-BR delimitada

**Estado:** proposto, não iniciado. **Autorização de execução:** ausente nesta atividade.

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
