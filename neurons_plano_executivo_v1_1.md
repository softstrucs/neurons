# Neurons — Plano Executivo de Implementação

**Documento:** `NEURONS_PLANO_EXECUTIVO.md`  
**Versão:** 1.1 — edição consolidada para Codex local  
**Repositório:** `softstrucs/neurons` (privado)  
**Branch-alvo:** `devs`  
**Linguagem principal:** Rust  
**Ambiente-alvo inicial:** Debian 12, CPU convencional, 11 GiB RAM, cerca de 6,5 GiB livres em `/home` (valor informado; medir novamente antes da execução)  
**Situação:** plano de implementação, não alegação de capacidades cognitivas já demonstradas.

## 0. Mandato e princípios inegociáveis

Neurons é uma pesquisa e desenvolvimento de um **análogo cerebral artificial** em software: uma rede de **neurônios, sinapses e engramas artificiais** capazes de interagir e, progressivamente, demonstrar **cognição**, **metacognição** e **personalidade funcional**. A meta de longo prazo inclui questionar, explorar, estudar, memorizar, aprender, verificar/provar afirmações quando possível, construir conhecimento, decidir e agir sob permissões. Não declarar tais capacidades sem evidências. Não presumir consciência.

**ARQ-001 (aprovada):** arquitetura neuromórfica **orientada a eventos (EDA)**, inicialmente executada localmente em CPU, com núcleo independente da interface e de servidor HTTP.  
**TEC-001 (aprovada):** Rust como linguagem principal; Python apenas se demonstrada necessidade instrumental.  
**DF-001 (aprovada):** domínios de aprendizagem progressivos e integrados: (1) Linguagens e comunicação; (2) Matemática e raciocínio lógico; (3) Ciências naturais; (4) Computação e pensamento computacional. Domínio não significa pacote de conteúdo já carregado nem módulo neural separado.  
**Estratégia:** menor núcleo verificável primeiro; modularidade **LEGO** via componentes pequenos e contratos explícitos; expansão somente por necessidade e testes.  
**Escopo excluído:** não utilizar LLM como substituto oculto do raciocínio do núcleo. Não introduzir, na primeira versão, GPU, Docker, Kubernetes, bancos externos, serviços distribuídos, navegador, agentes ou servidor web.

## 1. Protocolo operacional para o Codex

1. Trabalhar **somente** no clone existente de `softstrucs/neurons`, na branch `devs`. Verificar `pwd`, `git remote -v`, `git status --short`, `git branch --show-current`, `git log -1 --oneline` antes de escrever. Se branch não for `devs`, parar e informar; não trocar de branch automaticamente se houver alterações locais. Se a árvore estiver suja, preservar tudo, apresentar o diagnóstico e limitar mudanças aos arquivos autorizados; nunca sobrescrever conteúdo.
2. Ler `README.md`, `Cargo.toml`, `AGENTS.md` e estruturas já existentes, se houver. Respeitar instruções de repositório compatíveis com este documento. Não criar outra estrutura antes de entender a existente.
3. Executar **uma etapa por vez**. Ao concluir, mostrar arquivos criados/alterados, comandos de teste, resultados, uso de disco e memória, limitações, e **parar para revisão** antes da etapa seguinte. Falhas de teste impedem conclusão da etapa.
4. Não fazer `git push`, merge, criação de PR, exclusão de arquivos, instalação de sistema ou downloads volumosos sem autorização explícita. Commits locais pequenos e descritivos somente se autorizados; não alterar `main`.
5. Nenhuma dependência de crate externa na linha de base sem justificar benefício e custo; usar `std` inicialmente. Evitar `cargo install` de ferramentas opcionais. Nenhum processo em background permanente.
6. Não apresentar simulação de eventos, tabela de respostas ou roteamento fixo como prova de raciocínio, aprendizagem ou metacognição. Todo relatório deve distinguir **implementado**, **testado**, **observado** e **ainda hipotético**.
7. Manter APIs pequenas e documentação objetiva. Preferir testes determinísticos, seeds fixas quando necessário e saídas reproduzíveis.

## 2. Restrições operacionais e orçamentos

Os dados informados foram: Debian 12 (`bookworm`), `/home` de 79 GB com cerca de **6,5 GB disponíveis** e 92% de ocupação; **11 GiB RAM**, cerca de **3,7 GiB disponíveis** na leitura; swap 7,8 GiB, com uso de 2,8 GiB. `rustup`, `cargo` e compilador C já estão presentes. **Não reinstalar Rust** sem evidência de necessidade.

- **Preservação de disco:** orçamento preferencial de trabalho de **até 1 GiB** para `target/`, código e resultados de experimentos iniciais; alerta em 1 GiB, parada preventiva se o espaço livre de `/home` for **inferior a 3 GiB**. Esses limites são operacionais ajustáveis, não medições de consumo futuro.
- **Preservação de RAM:** primeira versão deve ter meta de **até 256 MiB de RSS** nos testes regulares, com testes limitados em número de eventos e células. Interromper cargas que provoquem swap excessivo. Em medições, registrar memória real e não inferir a partir do tamanho de estruturas.
- Configuração leve em `Cargo.toml` (se não houver política conflitante): `[profile.dev] debug = 0; incremental = false` e `[profile.release] debug = 0; incremental = false; strip = "debuginfo"`. Aplicar somente após validar compatibilidade do projeto.
- Usar comandos como `df -h "$HOME"`, `free -h`, `rustc --version`, `cargo --version`, `du -sh target 2>/dev/null`, `/usr/bin/time -v` **se disponível**. Não baixar benchmark pesado.
- Logs pequenos com rotação/limite; fixtures mínimas, sem corpus grande e sem gravação irrestrita de eventos.
- As verificações de capacidade são **portões de qualidade**; se ambiente diferir, registrar e ajustar sem descaracterizar o projeto.

## 3. Arquitetura inicial — LEGO, sem microserviços

**Um único pacote Cargo (`neurons`)**, organizado por módulos com contratos explícitos. Não criar várias crates, layers artificiais nem um framework/plugin system antes de necessidade comprovada.

```text
neurons/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .gitignore
├── AGENTS.md                       # instruções locais do Codex, se pertinente
├── docs/
│   ├── architecture.md
│   ├── experiments.md
│   └── decisions.md
├── src/
│   ├── main.rs                     # CLI mínima; nunca conter lógica neural
│   ├── lib.rs                      # API pública do núcleo
│   ├── event.rs                    # tipos e ordem de eventos
│   ├── neuron.rs                   # unidade e estado funcional
│   ├── synapse.rs                  # ligação e influência
│   ├── engram.rs                   # codificação/recuperação experimental
│   ├── network.rs                  # composição e endereçamento
│   ├── engine.rs                   # agendador de eventos e evolução
│   ├── learning.rs                 # regras candidatas, plugáveis
│   ├── cognition.rs                # tarefas/hipóteses e avaliação
│   ├── metacognition.rs            # monitoramento, incerteza e revisão
│   └── personality.rs              # política comportamental, não consciência
└── tests/
    ├── interactions.rs
    ├── memory.rs
    ├── learning.rs
    ├── cognition.rs
    └── metacognition.rs
```

**Atenção:** essa árvore é uma **meta evolutiva**, não algo a gerar integralmente na Etapa 01. Criar somente módulos necessários à etapa corrente. Um módulo é uma peça LEGO se for testável isoladamente, possuir entradas/saídas explícitas, invariantes verificáveis e substituição possível sem editar todas as outras peças. Rust `mod`, `struct`, `enum`, `trait` (somente quando justificado), ownership e testes são suficientes no início.

### Contratos conceituais mínimos (a validar, não definições científicas finais)

- **Evento:** identificadores compactos de destino/origem (quando aplicável), tipo, carga útil pequena, instante lógico e sequência de desempate; referências a contexto, hipótese e evidência quando a tarefa exigir. Confiança somente com semântica definida e testável. Comunicação interna preferencialmente por números/IDs e referências, **não por textos longos**. Processamento determinístico por `(instante_lógico, sequência)`; limites contra laços de propagação.
- **Neurônio:** unidade computacional **ativa e leve**, com identificador, entradas de eventos, estado interno compacto, processamento local, saída de eventos e, quando experimentalmente justificados, memória local limitada, adaptação incremental e monitoramento local. Esses elementos são responsabilidades funcionais candidatas, não obrigação de implementar seis subsistemas no primeiro protótipo. Não pressupor raciocínio completo ou metacognição individual. Entrada: evento; saída: mudança de estado e zero ou mais eventos.
- **Sinapse:** conexão origem-destino, intensidade/transformação limitada e condição de propagação; adaptação apenas quando houver regra testada. Não criar autoaprendizagem implícita.
- **Engrama:** hipótese operacional de registro recuperável de experiência/associação, com procedência e verificação, sem supor que seja uma célula isolada ou um arquivo específico. Escolher representação **durante experimentação**, comparando alternativas simples.
- **Motor:** recebe estímulos, ordena eventos, processa mudanças de estado, aplica limites de eventos/tempo lógico/memória e reporta observações. O motor não deve embutir regras específicas de todos os neurônios.
- **Metacognição:** observa dados acessíveis (ações, resultados, limites, incerteza declarada), verifica critérios e sugere manutenção, revisão ou abstenção; não reivindicar acesso perfeito ao estado interno.
- **Personalidade:** características estáveis de interação (por exemplo, clareza, curiosidade, cautela epistêmica), controladas por parâmetros e separadas dos critérios de verdade; não mascarar falhas com respostas persuasivas.

**Fronteiras:** UI/CLI é adaptador; HTTP futuro é outro adaptador; persistência é substituível; os quatro domínios são conteúdo e tarefas experimentais, não quatro servidores. O motor pode rodar embutido no processo local, sem servidor neural separado.

## 3.1. Modelo funcional e organização coletiva — hipóteses a validar

A arquitetura aprovada é **neuromórfica orientada a eventos (EDA)**. Como **candidata de implementação**, investigar uma composição híbrida: neurônios leves, ativos e limitados, articulados por sinapses, engramas e circuitos funcionais que possam realizar operações mais complexas. **Essa distribuição não está aprovada como modelo definitivo**; comparar alternativas quando experimentos justificarem. Não fixar desde já equações de ativação, STDP/Hebb, número de neurônios ou formato dos engramas.

- **Responsabilidades locais candidatas:** receber estímulos; atualizar estado/contexto; executar operações simples; manter histórico curto quando útil; adaptar parâmetros/regras sob controle experimental; observar sinais locais de erro/limite; emitir eventos. Implementar apenas o necessário para cada hipótese e medir custo incremental de cada responsabilidade.
- **Responsabilidades coletivas candidatas:** perceber, questionar, investigar, relacionar conceitos, formular e confrontar hipóteses, aprender, recuperar conhecimento, avaliar incerteza, revisar conclusões e selecionar ações. Não criar uma classe de neurônio por verbo; experimentar **circuitos funcionais** e mecanismos compartilhados, sem presumir uma camada central obrigatória.
- **Comunicação:** preferir eventos estruturados compactos, como `(origem, destino, tipo, contexto_id, referencia_id, instante_logico)`; campos de hipótese, evidência e confiança são **opcionais e versionáveis**. Nunca assumir que a presença de um campo `confianca` significa calibração estatística ou compreensão.
- **Conhecimento e evidência:** diferenciar observação, hipótese, evidência favorável/contrária, conclusão provisória, desconhecido e erro identificado. Definir regras verificáveis de proveniência, recuperação, revisão e não confundir repetição de um fato com confirmação.
- **Aprendizagem:** exigir mudança de comportamento causada por experiência, testada contra controles sem adaptação, dados não vistos e testes de retenção/interferência. Separar memória de aprendizagem e ambas de generalização.
- **Metacognição:** observar resultados e limites acessíveis, sinalizar conflito/insuficiência, revisar estratégia ou abster-se quando apropriado. Pode envolver mecanismos locais e coletivos; não pressupor consciência nem introspecção perfeita.
- **Autonomia cognitiva:** permitir ciclos internos limitados de questionamento, exploração e revisão, **sem dependência de intervenção humana a cada inferência**. Impor orçamento de eventos, condição de parada e distinção entre raciocínio interno e ações externas, estas sujeitas a autorização.
- **Eficiência CPU:** processamento esparso, sem varredura integral contínua da rede; estado compacto; endereçamento por IDs; memória compartilhada/referenciada quando útil; agendamento limitado e aprendizagem incremental. Avaliar o ganho com benchmarks, pois eventos e roteamento também têm custo.

**Regra de atribuição de capacidades:** para cada teste, declarar se o resultado depende de uma unidade, de sinapses, de memória, de circuitos, de lógica auxiliar ou de regras pré-configuradas. Um conjunto de respostas roteadas não é evidência de raciocínio emergente. Não declarar capacidades além do que os controles demonstrarem.

## 4. Etapas executivas sequenciais

### Etapa 00 — Auditoria segura do clone e do ambiente

**Entregáveis:** relatório resumido em `docs/decisions.md` ou no terminal, sem alterar arquivos existentes desnecessariamente. Verificar repo/branch, versões Rust, estado, espaço, RAM, ferramentas de compilação e conteúdo do repo. **Não instalar nada.** Se faltarem toolchains, apresentar opções sem executá-las.  
**Aceite:** repositório correto, branch `devs`, árvore de trabalho conhecida, `cargo --version` funcional, estimativa de custo de compilação.  
**Parada:** branch divergente, dados locais importantes não avaliados, `/home` com menos de 3 GiB livres.

### Etapa 01 — Bootstrap Rust mínimo

**Fazer:** aproveitar o repositório; inicializar pacote apenas se não existir `Cargo.toml`; criar `src/main.rs`, `src/lib.rs`, `.gitignore`, README técnico curto. CLI `neurons --help`, `neurons doctor` sem rede nem dados externos; `doctor` informa versão/estado de ambiente sem prometer diagnóstico completo. Evitar crates externas.  
**Aceite:** `cargo fmt --check` (se `rustfmt` já instalado), `cargo check`, `cargo test`, `cargo run -- --help`, com footprint registrado. Se `rustfmt` faltar, registrar e não instalar automaticamente.

### Etapa 02 — Contratos e motor de eventos determinístico

**Fazer:** tipos `Event`, `NeuronId`, `SynapseId`, erro explícito, fila determinística e ciclo finito. Estabelecer representação compacta, IDs e campos opcionais de contexto/referência, sem transportar texto livre entre neurônios como protocolo interno padrão. Testar ordem estável, eventos simultâneos, destinos inexistentes, propagação cíclica, limite de eventos e descarte/erro controlado. Preferir biblioteca padrão (`BinaryHeap`, `VecDeque`, `BTreeMap` conforme testes) antes de bibliotecas externas.  
**Aceite:** reprodutibilidade exata com mesmas entradas; processamento termina sob limites; nada fica em loop infinito; medições básicas documentadas.

### Etapa 03 — Neurônio e sinapse artificiais mínimos

**Fazer:** primeiro modelo operacional de unidade **ativa** com entrada, estado, operação local e emissão de eventos, mais influência conectiva; no máximo poucas dezenas de neurônios. Documentar quais responsabilidades locais candidatas ficam de fora e por quê. Especificar explicitamente entradas, estado, saídas, saturação e invariantes. **Não assumir pulsos biologicamente fiéis, Hebb, STDP ou dinâmica específica como aprovadas.** Comparar modelos apenas se necessário à hipótese.  
**Aceite:** registrar estado e eventos por unidade, custo por ativação e efeito da comunicação; estímulo em A pode alterar B por ligação configurada; sem ligação não há propagação; alterações de peso têm efeito observável quando ativadas; execução idêntica sob mesmo estado inicial e eventos.

### Etapa 04 — Engramas: hipótese mínima de memória

**Fazer:** implementar um experimento de codificação e recuperação de associações/experiências com referência à origem, contexto, evidências e estado de validação. Comparar armazenamento local limitado versus referência a memória compartilhada somente se relevante ao experimento. Registrar distinção entre `observado`, `hipótese` e `validado`. Memória pode estar distribuída ou em estrutura auxiliar candidata; justificar escolha sem apresentar analogia como fato. Persistência em disco é opcional nesta etapa.  
**Aceite:** recuperação contextual reproduzível; teste de associação correta versus distrator; teste de interferência e ausência de memória; repetição não implica automaticamente verdade.

### Etapa 05 — Aprendizagem experimental, não memorização de respostas

**Fazer:** introduzir **uma** regra adaptativa explícita e reversível (local, sináptica ou associativa, conforme hipótese), com modo controle sem atualização. Medir também custo de CPU/RAM da atualização incremental. Estabelecer treino/teste distintos, casos novos da mesma classe, pré e pós-treino, métricas e regressão. Separar mudança de parâmetros da prova de generalização.  
**Aceite:** evidência de alteração por experiência, resultado acima do controle em tarefa predefinida (se demonstrado), custos medidos; caso negativo relatado como negativo. Se não houver ganho, não seguir alegando aprendizagem funcional.

### Etapa 06 — Primeiro ciclo cognitivo verificável

**Fazer:** estabelecer tarefa limitada e significativa (por exemplo, inferência simples entre relações aprendidas), com hipóteses, referências a evidências e resposta verificável; testar participação coordenada de mais de uma unidade/circuito e distinguir isso de lógica central rígida; comparar contra tabela fixa, aleatoriedade e mera recordação. Capacidade exigida: recuperar informações pertinentes, operar uma transformação/inferência definida e justificar externamente quais entradas/estados observáveis sustentaram o resultado.  
**Aceite:** protocolo com exemplos não vistos durante ajuste; correção e generalização mensuradas; distinguir resolução real na tarefa de simples consulta de associação.

### Etapa 07 — Primeiro ciclo metacognitivo verificável

**Fazer:** mecanismo observável (local e/ou coletivo, conforme experimento) que compara saída com critérios observáveis, identifica informação ausente, registra incerteza/erro, solicita nova tentativa, reformula hipótese ou se abstém quando necessário. Incluir um ciclo interno autônomo com orçamento e parada determinísticos. Introduzir tarefas com evidência contraditória e dados insuficientes. Medir falsos alarmes e falhas de detecção.  
**Aceite:** decisões de revisão/abstenção variam adequadamente quando o contexto muda e superam controle sem monitoramento; nunca afirmar autoinspeção ilimitada.

### Etapa 08 — Personalidade funcional inicial e interface textual

**Fazer:** definir pequeno conjunto de disposições comportamentais explícitas e consistentes na forma de interação, mantendo conteúdo factual e verificações independentes; CLI de sessão curta. Não introduzir gerador linguístico sofisticado ou LLM. A primeira interface pode usar vocabulário/gramática restritos e isso deverá ser informado.  
**Aceite:** comportamento reproduzível sob configurações iguais; mudança de estilo não muda resultados de testes de verdade/correção; personalidade não substitui metacognição.

### Etapa 09 — Domínios fundamentais como microcurrículos experimentais

**Fazer:** criar tarefas pequenas, integradas e mensuráveis para: linguagem/comunicação (símbolos e relações), matemática/lógica (comparação, relações, inferência), ciências naturais (observações e hipóteses em ambiente simplificado) e computação (sequências, estados e regras). Aumentar complexidade só após evidência de transferência, não por volume de dados.  
**Aceite:** ao menos um protocolo auditável por domínio; separação entre conhecimento fornecido, adquirido, recuperado e competência demonstrada; teste de retenção/interferência e tentativa de transferência entre tarefas.

### Etapa 10 — Robustez e eficiência em CPU

**Fazer:** perfis controlados e crescentes (p.ex. 10, 100, 1.000 unidades, apenas se memória permitir), comparações de latência/eventos por segundo, **custo por evento e por atualização**, RSS, crescimento de memória, custo de engramas e recuperação. Comparar ativação esparsa com varredura integral apenas quando existir baseline seguro. Evitar geração excessiva de arquivos. Otimizações somente para gargalos medidos.  
**Aceite:** benchmark reproduzível, orçamento de recursos respeitado em perfil inicial, documentação de limites, testes de regressão completos. Não confundir escala com inteligência.

### Etapa 11 — Persistência e continuidade opcional

**Fazer:** somente se necessário, salvar/restaurar estado com versão, integridade e tratamento de corrupção; arquivo local pequeno antes de DB. Testar retomada de eventos, limites e consistência.  
**Aceite:** estado recuperado igual em casos definidos; falha de arquivo não corrompe silenciosamente a sessão; limites de disco; nenhuma gravação ilimitada.

### Etapa 12 — Acesso por API e evolução web (ADIADA)

**Pré-condição:** núcleo local comprovadamente útil, contratos estabilizados e espaço/recursos suficientes.  
**Fazer:** criar adaptador HTTP opcional separado da lógica neural; autenticação, limites de acesso, validação de entrada, operações autorizadas, logging mínimo; HTTPS no ponto de exposição adequado. Escolher framework só nesta etapa, comparar custo e manutenção.  
**Aceite:** mesmo núcleo atende CLI e API sem duplicação; testes de segurança, limites de recursos e regressão. **Não executar esta etapa agora.**

## 5. Experimento E001 — composição, interação, memória e autoavaliação

**Pergunta norteadora:** qual é a menor composição de neurônios, sinapses e engramas artificiais interagindo que demonstra habilidades cognitivas e metacognitivas verificáveis e progressivamente melhoráveis? A personalidade funcional também é objetivo, porém não deve ser confundida com prova cognitiva.

**Hipóteses distintas:**
- H1: eventos causam mudanças de estado e propagação inter-neuronal observáveis (Etapas 02–03).
- H2: experiências deixam registros recuperáveis com discriminação entre condições (Etapa 04).
- H3: uma regra explícita de adaptação altera desempenho em tarefa não idêntica aos exemplos de treino (Etapa 05).
- H4: rede e mecanismos de avaliação realizam inferência delimitada e detectam pelo menos algumas falhas/incertezas (Etapas 06–07).
- H5: um ciclo interno limitado consegue formular ou revisar hipótese diante de evidência contraditória, sem intervenção humana a cada passo (Etapas 06–07).

**Controles mínimos:** mesmo problema sem sinapses, sem memória, sem adaptação, sem monitoramento e com regras de resposta fixa (conforme hipótese); ordem de eventos reproduzível; dados de teste não presentes no treino; logs auditáveis. **Resultado negativo é informação científica válida.**

**Métricas:** acerto em conjunto de teste; generalização e retenção; taxas de detecção de erro/abstenção adequada; eventos processados, custo médio por evento, latência, RAM máxima, disco consumido, eventos por unidade e frequência de ativações. Registrar versões do código e condições do teste. Não fixar limiares científicos arbitrários antes de conhecer baselines; exigir comparação explícita e dados brutos resumidos.

## 6. Governança, revisão e critérios de conclusão

Cada etapa entrega: (a) código mínimo; (b) testes verdes; (c) evidência e limites; (d) custos de CPU/RAM/disco; (e) decisão **prosseguir / ajustar / descartar**. O Codex deve **parar no fim de cada etapa**. Não registrar decisões científicas apenas porque código compilou. Sem commits na branch principal; PR futuro somente após revisão do usuário.

**Definition of Done (DoD) por etapa:** código compila em Rust estável já instalado; testes relevantes executados; invariantes documentados; recursos monitorados; `.gitignore` exclui `target/`, temporários e dados sensíveis; nenhum segredos embutidos; relatórios curtos; nenhuma implementação futura antecipada sem autorização.

## 7. Prompt de delegação inicial ao Codex (executar apenas Etapa 00)

> Você é o Codex trabalhando localmente no clone privado `/neurons`, branch `devs`. Leia integralmente `NEURONS_PLANO_EXECUTIVO.md` e siga suas restrições. **Execute somente a Etapa 00 — Auditoria segura do clone e do ambiente.** Não instale pacotes, não crie commits, não faça push, não modifique arquivos, não escolha modelos neuronais e não inicie a Etapa 01. Confirme branch, remote, `git status`, estrutura existente, Rust/Cargo, RAM e disco do sistema Debian 12. Apresente diagnóstico conciso, riscos, recomendações para próximo passo e **pare aguardando revisão**.

---

**Observação:** esta versão consolida os requisitos e ajustes da sessão e do material complementar fornecido pelo usuário. Propostas de implementação continuam hipóteses até aprovação e validação experimental.
