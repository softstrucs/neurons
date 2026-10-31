# V03-F1-P01 — Decisões metodológicas pré-finais aprovadas

## Identificação, autorização e fontes

Base `78efa46630efec8aceb49c843b016e0c9282404f`, `devs`. A autorização humana expressa nesta atividade aprova D01–D04 nos limites abaixo. É uma formalização documental: não autoriza alterar instrumentos ou núcleo, realizar novos experimentos, preparar resultados finais nem executar F. O plano v0.3 e o protocolo histórico permanecem intactos.

Fontes: [plano v0.3 aprovado](planos/Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_APROVADO.md), [protocolo proposto](protocolos/V03-F1-P01_confiabilidade_experimental_inicial_PROPOSTA.md), [E01](protocolos/V03-F1-P01_E01_identidade_S2_S3.md), [validação preliminar](V03-F1-P01_validacao_preliminar_instrumentos.md), [auditoria pré-final](V03-F1-P01_auditoria_prontidao_pre_final.md) e [decisões históricas](decisions.md). A inspeção dos instrumentos confirma apenas contratos e versões presentes; não é nova validação experimental.

## D01 — E01 e identidade S2/S3

E01 está **APROVADA METODOLOGICAMENTE PARA V03-F1-P01**. S2 e S3 representam estados construídos separadamente e estruturalmente equivalentes, com identificadores distintos normalizados por bijeção correspondente. Isso basta para o contraste contextual delimitado por caso: resposta em A com suporte e abstenção em B com negação isolada. Não se exige identidade literal de todos os bytes para essa alegação restrita. Não se demonstra comportamento de duas consultas sobre o mesmo objeto ou checkpoint literalmente idêntico; alegação mais forte exige controle próprio. IDs, cenários, quotas e resultados previstos permanecem inalterados.

## D02 — Referência simbólica

Aceita-se condicionadamente a referência de lógica separada para a alegação local. A inspeção registrada na auditoria não identificou chamada ao `Monitor::review`, `IntegratedState::review`, `TwoHopTask` como oráculo, função de validação do candidato ou uso de sua decisão como gabarito; não se identificou dependência circular de decisão que invalide esse recorte. A referência importa o tipo canônico `Fields` e compartilha especificação e manifesto. Sua lógica e dados de entrada são limitados ao contrato documentado, inclusive ao suporte direto; o defeito de contagem de fatos extras foi corrigido e registrado em V4.

| Dimensão | Estado |
|---|---|
| Independência da lógica de decisão | Delimitadamente demonstrada por inspeção e controles D/V. |
| Independência de implementação | Parcial: cálculo separado, tipo canônico e pressupostos compartilhados. |
| Independência de dados | Não demonstrada; manifesto comum à comparação. |
| Independência de especificação | Não demonstrada; critérios e semântica compartilhados. |
| Independência de autoria | Não demonstrada. |
| Revisão externa independente | Não realizada. |

A ausência de auditor externo não é, por si só, bloqueio absoluto à alegação local. Antes de F, deve existir revisão crítica documentada dos pressupostos, dados compartilhados, controles e possíveis erros comuns; não se simula revisão independente. Concordância não prova correção universal ou autenticidade da fonte. `EvidenceId` identifica referência; não a autentica. Referência ausente permanece distinta de referência inválida.

## D03 — Cobertura semântica

Aceitam-se esquema e validador **no domínio delimitado das seis classes S1–S6**, sem exigir validação universal para a futura investigação restrita. Há correspondência exaustiva dos tipos e campos públicos conhecidos na compilação, 79 valores públicos exercitados sinteticamente e comparação integral dos relatórios entre processos nas classes D/V; isso não significa que todas as combinações sejam alcançáveis ou semanticamente validadas. `parse_wire` é estrutural, não validador semântico universal; a comparação por modelo e a referência restrita detectam campos e valores inesperados no seu domínio.

Se aparecer variante não exercida em F, aplicar o contrato congelado: não aceitá-la silenciosamente, interromper ou marcar inconclusivo conforme o caso, sem adaptação retrospectiva. Uma variante explicitamente conhecida e coberta pelo contrato deve ser tratada conforme esse contrato; uma combinação sem referência verificável não pode ser homologada. Sem extrapolação para todas as topologias, variantes, configurações, condições de erro ou segurança geral do parser.

## D04 — Recursos e limites prospectivos

Os seguintes limites operacionais conservadores são aprovados prospectivamente para eventual interrupção, sujeitos a confirmação prévia do ambiente e a mecanismo de fiscalização apropriado; não são resultados de conformidade:

| Recurso | Limite / condição | Estado da medição e da fiscalização |
|---|---|---|
| Tempo total do painel | Inferior a 30 s; interromper ao atingir o limite. | Relógio monotônico e tempo externo medidos em D/V; método de fiscalização de F deve ser registrado. |
| RSS | Limite operacional de 256 MiB. | Método e escopo por processo/árvore devem ser explicitados; amostras D/V não garantem pico real. Se não houver fiscalização adequada à segurança operacional, F fica impedido até definição de mecanismo suficiente. |
| Dados por condição | Referência de 256 KiB. | Dados vivos totais não medidos; bytes de checkpoint não substituem essa grandeza. Antes de F: **(1)** instrumentação confiável com definição operacional de dados vivos, **ou (2)** decisão metodológica humana explícita retirando esse teto dos critérios indispensáveis à alegação local, mantendo-o como restrição não verificada e sem afirmar conformidade. Nenhuma alternativa é escolhida aqui. |
| Espaço livre em `/home` | Pelo menos 3 GiB antes de executar. | Verificação pontual futura obrigatória; observação histórica não garante disponibilidade posterior. |

Limite aprovado, medida obtida com método declarado e grandeza não demonstrada são estados distintos. Pico RSS amostrado, soma de picos, pico simultâneo de árvore e memória acumulada não são equivalentes; médias ou amostras não provam máximos garantidos. Os controles de recursos não são aprovação de F.

## Contrato científico consolidado

Se o protocolo for congelado e autorizado, os controles forem válidos e os resultados sustentarem cada conclusão, poderão ser relatadas **apenas no domínio avaliado**: reprodutibilidade de relatórios públicos; consistência formal com referência simbólica; isolamento contextual nas classes avaliadas; identificação de referências ausentes; sensibilidade às adulterações efetivamente aplicadas; e preservação dos bytes do checkpoint coberto. Identidade S2/S3 é estrutural, não literal. Comparar repetição de topologias não demonstra generalização estatística.

Resultado positivo exige os critérios prospectivos e controles satisfeitos; divergência com referência válida é resultado negativo local; referência inválida ou ausente quando indispensável, falha de instrumento, vazamento ou recurso indispensável não verificado produz resultado inconclusivo ou **NÃO AVALIÁVEL**, conforme a condição. Não se exclui caso por resultado desfavorável nem se ajusta o método após observar F.

São proibidas alegações de aprendizagem cognitiva, generalização, retenção neuronal, autonomia operacional, permissões globais integradas, autenticidade de fontes, correção universal do Monitor, independência científica absoluta, homologação integral de F1 ou competência A8. Não se confunde ausência de demonstração com impossibilidade geral.

## Condicionantes C01–C05

- **C01:** dependências são específicas da alegação, sem exigir conclusão integral de outras frentes.
- **C02:** não aplicável enquanto não se alegar contribuição causal neuronal ou sináptica; se tal alegação surgir, controles e ablações apropriados são obrigatórios.
- **C03:** permissões integradas e persistência neuronal são não avaliáveis neste recorte; não se inferem pela restauração dos estados cobertos.
- **C04:** restauração de registros não demonstra retenção cognitiva, aprendizagem, transferência, cumulatividade ou especialização.
- **C05:** hipóteses, critérios, controles, instrumentos e manifestos devem ser congelados prospectivamente antes de F. **Ainda não concluído operacionalmente.**

## Requisitos de congelamento e pendências operacionais

Revisar criticamente pressupostos e controles; verificar integridade e versões dos instrumentos, configuração e ambiente; resolver os requisitos de recursos da D04; preservar separação D/V/F, critérios de parada e exclusão e histórico V1–V4. Preparar manifesto final sem consultar resultados, sem gerar ou habilitar F nesta atividade. Antes de eventual execução, uma autorização específica deverá abranger qualquer modificação necessária para habilitar F, e o congelamento deverá registrar conteúdo, versões, hashes, responsáveis e momento antes do painel, sem ajuste retrospectivo. Tal autorização não é concedida aqui.

Referências SHA-256 na base documental, para rastreabilidade e não como congelamento ou prova de revisão independente:

| Arquivo | SHA-256 |
|---|---|
| Protocolo original | `e5722f99a7ff13b5040d9f71299847903b3c245c9c20f6109fb3a0fd22cf5286` |
| `tests/v03_f1_p01_instruments.rs` | `4a2c759a22b990f7c8d1086dd82d302849931dbecafcaa7a68bc1fd5a9890ae9` |
| `tests/v03_f1_p01_schema.rs` | `5426befa9eb92c27984e26d33fa624f740734b3a48db60f98e061136fd995651` |
| `tests/support/v03_f1_p01/canonical.rs` | `6242a13d4bcae538582a690aa99ba55bede5620e2c36797b75948a7d91bbdd71` |
| `tests/support/v03_f1_p01/reference.rs` | `e8f92cd56b86ebb78e0af1ccbe0c3a6644fa3c3bc213f7379cde762c38d9e10a` |

## Matriz de prontidão

| Requisito | Estado |
|---|---|
| D01–D04 e E01 | Formalizadas; E01 aprovada metodologicamente. |
| Referência e limites científicos | Delimitados; independências e erros comuns explicitados. |
| Revisão crítica documentada para F | Pendente; não exige necessariamente auditor externo, mas exige registro específico. |
| Recursos e fiscalização | Pendente: RSS adequado, alternativa de dados vivos e confirmação do ambiente. |
| Integridade, versões e hashes | Identificados na base; reconfirmação e congelamento prospectivo pendentes. |
| D/V/F | D/V historicamente executados; gerador atual aceita apenas D/V. F não gerado nem consultado. |
| Manifesto final | Pendente de preparação autorizada sem consultar resultados. |
| Parada e exclusão | Critérios prospectivos definidos no protocolo; aplicação operacional e fiscalização devem ser confirmadas. |
| Congelamento e autorização específica de F | Pendentes; C05 não concluído. |

**Parecer: PENDENTE DE PRÉ-REQUISITOS OPERACIONAIS.** O registro documental está formalizado e pode ser submetido à apreciação humana; isso não equivale a estar pronto para executar. Não se declara **PRONTO PARA SUBMISSÃO À DECISÃO HUMANA** como liberação operacional enquanto faltarem os requisitos identificados; **NÃO APTO** seria cabível se uma dependência indispensável permanecesse inviável ou um controle fosse invalidado.

## Decisões humanas futuras e estado oficial

Decidir explicitamente a alternativa para os 256 KiB se não houver medição confiável; apreciar o mecanismo de fiscalização de RSS e os recursos; autorizar separadamente eventual alteração dos instrumentos, preparação/congelamento e execução F conforme o escopo de cada ato. Nenhuma decisão futura é presumida. Histórico EC-10 R3 não executado e EC-11 H1 parcial, limites da persistência neuronal e demais conclusões anteriores permanecem preservados.

Verificações de regressão: `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` aprovados; **190 testes aprovados, zero falhos e um ignorado** (`r3_frozen_final` da EC-10). A suíte repetiu testes históricos e D/V, sem constituir execução do painel F deste protocolo. Três avisos históricos de método não utilizado na EC-04 persistem.

**D01–D04 formalizadas; E01 aprovada metodologicamente; protocolo final não congelado nem aprovado para execução; F não autorizado nem executado; F1 sem homologação experimental final; nenhuma competência cognitiva homologada.**
