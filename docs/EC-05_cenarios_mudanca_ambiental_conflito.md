# EC-05 — Testar mudança ambiental e conflito
## Cenários e verificação de suficiência da especificação

### 1. Resultado desta delegação

**Preparação documental concluída; implementação e execução experimental interrompidas antes de começar, por decisão científica pendente.** Não há resultado experimental novo nem conclusão de robustez contextual. A segunda subetapa oficial da EC-05 permanece pendente de execução; não se avançou para “definir política de revisão e abstenção” ou “avaliar retenção sem misturar versões de contexto”.

Base: `af42f9c3de1fb8ec2c0eab992a2bf779d8fde718`, branch devs, HEAD e origin/devs coincidentes, referência remota conferida e árvore inicialmente limpa. Autorização atual permite definir cenários, verificar regras disponíveis e, se faltar decisão substantiva, documentá-la e parar antes de implementar/executar, versionando os artefatos efetivamente concluídos. É essa alternativa que foi aplicada.

Lidos integralmente o plano cognitivo v0.2, a modelagem EC-05 e o Fundamento aprovado. Consultados decisões, auditorias EC-03.5/EC-04, protocolo EC-03.3-R1 e contratos correspondentes de engramas, monitoramento, proveniência e indução EC-04. Cabeçalhos e previsões históricos não foram alterados. EC-03/04 permanecem encerradas como investigações delimitadas, com AJUSTAR/conformidade parcial e suas ressalvas.

Este documento **não é protocolo congelado**, nova política, autorização adicional ou implementação. Cenários são definidos em termos de condições observáveis e sondas; números concretos, calendários e escolhas pendentes não foram inventados. Recomendações abaixo não são decisões aprovadas nem resultados.

### 2. Regras que já existem, e onde se aplicam

| Mecanismo/documento | Regra disponível | Limite para esta investigação |
|---|---|---|
| EngramStore/Cue, src/engram.rs | Recuperação por igualdade de contexto/origem/padrão; duplicação de ID rejeitada; múltiplos registros da pista são ambíguos. | Não fornece revisão temporal, obsolescência ou distinção geral de contradição; observed_at não é validade. |
| Monitor, src/metacognition.rs | Assessments delimitados de falta de evidência/conflito/limite; revisão de consulta entre contextos fornecidos. | Não revisa a memória nem aprende contextos; não é política EC-05 de validade/revisão de afirmações. |
| EC-03.3-R1 §§3–5 | Contexto exato, validade [start,end), versões/corte, suspensão/retratação, Refresh; oposição ativa dá Conflict sem vencedor por recência/ID/fonte. | Contrato explicitamente local ao laboratório L de dois passos. Adoção para as novas sondas EC-05 ainda não foi declarada. |
| EC-03.3-R1 §§5–6 e implementação | Sem conflito, prova elegível permite Supported; sem prova, causas históricas seguem Suspended, Retracted, Expired, Stale, Unknown. Literal recompõe sem Refresh. | Essa precedência é convenção histórica, não política geral aprovada de EC-05; erros de execução são parcialmente transportados por Result. |
| EC-04, protocolo §§2–3 e candidato | Episódios rotulados por contexto; maior sufixo consistente em H; Query por unanimidade; log conservado. | Não tem validade temporal geral nem decisão Conflict; não resolve conflitos de fontes. Revisão é uma hipótese programada de recência, diferente de EC-03. |
| Modelagem EC-05 §§4–5/8 | Distingue versões, tempo, validade, corte, contexto e suporte; explicitamente não promove políticas anteriores nem fixa precedência geral. | Serve ao desenho e à interpretação, não fornece o oráculo operacional completo das novas interações. |

Não falta infraestrutura para toda observação possível: os painéis anteriores já têm protocolos próprios e podem ser reproduzidos sob autorização pertinente. O impedimento não é alegar que EC-03 não tenha regra de conflito, nem exigir que todo código anterior seja corrigido. É decidir **o objeto e o contrato de validade da nova investigação** sem transformar automaticamente um laboratório anterior em política da EC-05. Repetir os painéis antigos não resolveria essa decisão nem demonstraria cobertura nova.

### 3. Cenários definidos para a subetapa

Todos distinguem estado anterior, mudança recebida, consulta e registro posterior. A consulta não adquire, não dá Refresh implicitamente nem recebe resposta esperada; suportes, versões, corte e motivos precisam ser comparáveis. As letras abaixo são identificadores documentais, não fixtures, respostas codificadas ou estratos materializados.

| Cenário | Pré-condição e intervenção a observar | Sondas verificáveis | Expectativa já fundamentada / ponto ainda aberto |
|---|---|---|---|
| A — Mudança ambiental recebida | Afirmação antes sustentada; chega observação posterior incompatível, mantendo a identidade do contexto. | Decisão antes/depois do recebimento e de eventual manutenção; quais versões continuam como suporte. | Distinguir mudança instrumentada do ambiente de relato que alega mudança. Ainda é necessário fixar se o diagnóstico usa oposição entre relatos independentes ou substituição versionada declarada. Não escolher recência como resposta correta. |
| B — Afirmações em contextos diferentes | Conteúdos com validade delimitada em contextos distintos, incluindo polaridades potencialmente opostas. | Consultar cada contexto separadamente e consultar um contexto sem suporte; inspecionar origem das premissas. | Diferença de contexto não é conflito automaticamente. Igualdade exata é mecanismo existente; congelar namespaces/identidades e semântica das consultas antes do teste. |
| C — Evidências conflitantes | Relatos evidenciais opostos para a mesma relação, contexto e época, ambos ativos e de identidades independentes. | Decisão e referências de ambos; após retirada explícita de um relato, identificar suporte restante sem apagar histórico. | EC-03 prevê Conflict antes da retirada; esse é o contrato local recomendado, não uma regra nova já adotada para todo Neurons. Não eleger fonte/ID/voto vencedor. |
| D — Validade temporal | Uma via tem intervalo declarado; a época consultada alcança sua fronteira; considerar separadamente uma via alternativa ainda válida. | Aplicabilidade da via em cada época; diferença entre validade temporal e tempo de registro; causas/provas utilizadas. | EC-03 usa intervalo semiaberto e preserva prova alternativa elegível. Precisa ser explicitamente o contrato do ensaio, inclusive Refresh e precedência quando causas coexistem. |
| E — Revisão ou suspensão | Nova versão de evidência ou suspensão explícita de fonte/regra, sem apagar registros anteriores. | Sondas antes/depois da manutenção e retomada permitida; localizar estado do suporte, não apenas booleano de saída. | Stale/Suspended e posterior recomposição estão definidos em EC-03; o literal pode responder Supported antes de Refresh. Não exigir concordância artificial. Revisão de critério EC-04 é operação diferente. |
| F — Contexto não afetado | Dois conjuntos de afirmações e suportes identificados; modificar somente um. | Comparar decisões/referências do outro antes/depois e verificar se realmente não partilha dependências alteradas. | Preservação é justificável se dependências são separadas. Não exigir invariância se fonte/regra compartilhada também perdeu validade; não chamar isso de resistência neural à interferência. |
| G — Histórico | Registrar a decisão anterior, corte e época; depois revisar/suspender/retratar suporte corrente. | Replay no corte/época antigos; consulta corrente separada; comparar referências, motivos e conteúdo que a API realmente expõe. | EC-03 permite replay delimitado. Não se espera recuperação geral por identidade/versão de todos os objetos, inexistente na API; EC-04 não oferece esse histórico completo. |

Informação insuficiente deve aparecer também em B/G e nas consultas sem aquisição/suporte: falta de dado não vira falsidade. Os cenários D/E distinguem evidência expirada de evidência substituída e de fonte temporariamente indisponível. Não se geraram conteúdos, alvos finais ou sequências concretas para esses cenários.

### 4. Distinções e expectativas admissíveis

| Termo | Evidência necessária para utilizar a descrição | Inferência proibida |
|---|---|---|
| Mudança observada | Registro do ambiente instrumentado ou recebimento explicitamente identificado, com condição anterior/posterior. | Chamar todo relato mais recente de verdade ou descoberta autônoma. |
| Contradição efetiva no modelo | Oposição sobre conteúdo/relação comparável, escopo temporal/contextual compatível e suportes admissíveis. | Confundir IDs diferentes, contextos diferentes ou observações históricas não simultâneas com conflito corrente. |
| Diferença legítima de contexto | Suportes e consultas referidos a condições distintas, sem transferência implícita. | Generalizar conteúdo só pela igualdade de símbolos ou nomes. |
| Informação insuficiente | Falta de suporte, contexto não descrito, acesso indisponível ou história não recuperável explicitamente discriminados. | Pontuar abstenção como previsão de classe correta ou ocultar erro como Unknown. |
| Evidência obsoleta | Versão anterior identificada e relação de substituição verificável, segundo contrato escolhido. | Usar observed_at ou versão do arquivo como prova de validade atual. |
| Revisão justificada | Mudança interna ligada às experiências/operações autorizadas, com efeito posterior e histórico conservado no alcance disponível. | Confundir comando externo Revise com aprendizagem autônoma ou selecionar nova regra pelo gabarito final. |

Expectativas locais de EC-03 conhecidas: Conflict não é falsidade; Expired não é ausência total de qualquer outra prova; Stale antes de Refresh pode coexistir com Supported do literal forte; suspensão/retirada não apagam a decisão histórica. Esses resultados não são convertidos em expectativas universais. Nos pontos ainda abertos da tabela, **não se definiu um resultado vencedor**.

### 5. Controles, métricas e condições de avaliação a congelar

Controles documentalmente pertinentes, sem execução ou instanciação nesta entrega:

- **Recuperação literal forte:** recompor diretamente das evidências válidas, sem depender do Refresh do candidato. Deve receber os mesmos fatos, contextos, versões e acesso autorizado ao diário; não reutilizar o literal T-C incompleto da EC-04 como controle de validade.
- **Executor simbólico independente:** cálculo próprio das provas e estados no contrato escolhido, sem chamar atualização, validação ou decisão do candidato. Distinguir independência de código de pressupostos comuns.
- **Sem aquisição / informação indisponível:** ablações explícitas, com saídas e denominadores separados. Não chamar falta deliberada de informação de competição equivalente.
- **Sem manutenção/atualização:** conservar estado anterior após a mudança, explicitando quais registros novos ficam disponíveis e quais operações foram suprimidas. Não inventar um ramo congelado que receba informação adicional ou treinamento auxiliar oculto.
- **Controle estável/contexto não afetado:** observar mudança indevida de decisões e referências, sem retirar dependências compartilhadas necessárias para o teste.

Métricas propostas, com denominadores a publicar por cenário, componente, fase e motivo:

| Métrica | Definição verificável | Cuidado necessário |
|---|---|---|
| Correção de estado | Decisões compatíveis com o contrato pré-aprovado / consultas elegíveis. | Sem contrato não existe acurácia interpretável para a interação nova. |
| Cobertura e erros explícitos | Respostas sustentadas / consultas; respostas sustentadas sem suporte admissível contadas separadamente. | Abstenções não são previsões positivas/negativas. |
| Abstenção qualificada | Contagens por motivo e falsas abstenções diante de prova corrente elegível. | Preservar Unknown, Conflict, Expired, Stale e falha de execução sem colapsá-los. |
| Proveniência | Provas com versões/premissas/contextos verificáveis / provas devolvidas; referências quebradas ou cruzadas. | Presença de ID não basta; verificar as premissas pelo controle permitido. |
| Conservação contextual | Consultas do contexto realmente não afetado com decisão e referências conservadas / sondas desse contexto. | Separar isolamento de estados de resistência a interferência compartilhada; não iniciar o estudo posterior de retenção geral. |
| Histórico | Consultas por corte reproduzindo a decisão registrada e as referências recuperáveis / consultas históricas suportadas. | Falta de API geral é limitação, não recuperação integral presumida. |
| Mudança/revisão | Mudanças de estado e classificações antes/depois, ligadas a operações/evidências; erros corrigidos e introduzidos. | Não definir um limiar de confirmação/recência ou escolher uma política para melhorar pontuação. |
| Reprodutibilidade/custo | Igualdade das sequências, decisões, motivos, referências e trabalho; eventos, tempo, dados vivos e RSS com escopo declarado. | Não afirmar equivalência de custo com Work parcial ou repetições globais não iguais. |

**Falha funcional**, quando houver protocolo: contrariar uma regra explicitamente adotada (por exemplo, usar premissa de outro contexto ou suporte retirado como corrente), promover hipótese isolada a prova, apagar história exigida ou modificar estado em consulta. Diferenças previstas candidato/literal não são falhas. Resultado negativo válido permanece válido, sem ajustar expectativas.

**Inconclusão/interrupção:** contrato não escolhido, política de prioridade não determinada para a sonda, oráculo ambíguo, vazamento, referência que não se consegue fiscalizar, instrumento insuficiente para a alegação, estouro de recurso ou necessidade de alterar arquitetura/código de produção. Erro de execução não vira Unknown, classe negativa ou acerto.

**Êxito:** só poderá ser definido operacionalmente após resolver o contrato e congelar painel, resultados por sonda, cobertura exigida e guardas. Não se inventa percentual de aprovação, número de casos ou repetição por analogia com as 500 verificações EC-03 ou 14560 consultas EC-04. Esses números são históricos, não quotas EC-05.

### 6. Verificação de suficiência e decisão científica indispensável

As regras são suficientes para entender e, sob seus próprios protocolos, reproduzir os laboratórios antigos. **Não estão suficientemente adotadas/especificadas para executar o novo painel EC-05 como avaliação de uma competência contextual comum.** O plano §7 exige contrato operacional completo; a modelagem EC-05 §§4–5 declara explicitamente que as políticas de EC-03/04 não são promovidas automaticamente e que a política geral pertence à subetapa posterior.

O ponto substantivo é a interpretação de uma observação incompatível mais recente: **conflito entre relatos ativos**, **substituição versionada declarada** e **episódio para indução por recência** são intervenções diferentes. Escolher silenciosamente uma delas muda o que se considera revisão, a informação dada ao sistema e o gabarito. EC-03 mantém oposição entre relatos elegíveis sem vencedor; EC-04 pode descartar restrições antigas do sufixo. Isso não é uma contradição entre implementações, pois seus objetos e contratos são distintos, mas impede exigir uma resposta comum sem decisão de escopo.

**Decisão indispensável proposta ao responsável:** adotar ou não o contrato diagnóstico local **EC-03.3-R1 para as novas sondas**, com seu vocabulário L de dois passos, contexto exato, validade semiaberta, oposição sem vencedor, versões/corte e Refresh explícito, preservando o literal forte e sem interpretar comandos externos de revisão como aprendizagem. Essa é a recomendação para uma investigação mínima sem arquitetura nova. A aprovação deve deixar claro que se testa conformidade/limites de um mecanismo programado, e não se define a política geral EC-05. A revisão de critérios EC-04 permanece evidência complementar, não executor equivalente ao de relações.

Se o objetivo for, em vez disso, comparar ou integrar as duas formas de revisão sob um resultado contextual comum, será necessária decisão científica adicional sobre identidade dos objetos, semântica da mudança e critério de referência; isso não está aprovado e pode antecipar a terceira subetapa. Não se recomenda fazê-lo implicitamente.

A adoção recomendada **não foi assumida**. Depois de decisão explícita, a mesma subetapa poderá ter seu painel operacional fechado prospectivamente: dados, estados, ordem de mudanças/Refresh/consultas, sondas pré/pós, histórico, controles, denominadores, orçamento e expectativas. Esses detalhes de execução não são tratados como microaprovações: deverão formar um único contrato completo antes da implementação. Não se exige reformar a arquitetura nem reconstruir o projeto para isso.

### 7. Ressalvas preservadas e recursos

EC-03: metadados de fonte e History parciais, revisão de regra incompleta, contagem de trabalho/memória parcial e controle literal mais pronto em S1/S5 antes de Refresh. EC-04: literal T-C incompleto; contexto/versão T-R e histórico incompletos; motivos/H incompletos; trabalho/memória parciais; repetições intercaladas e custo extra do ramo congelado; independência científica parcial dos controles. Nenhuma dessas lacunas foi corrigida ou convertida em evidência positiva nesta entrega.

A conservação em B da EC-04 foi obtida em estados separados; não demonstra retenção em parâmetros compartilhados. Os 132 testes, 14560 consultas finais, 3120 validações, 280 eventos e 12 manifestos são resultados históricos da auditoria EC-04, não testes realizados agora. O experimento EC-03 não passa a ser aprendizagem neural por ser reutilizável.

Permanecem as restrições locais: /home >=3 GiB e RSS isolado <256 MiB para eventual painel. Tetos específicos de eventos, tempo, dados e trabalho precisam ser fixados no contrato adotado e medidos com guardas suficientes; o limite fino antigo não é certificação de instrumento atual. Nenhum painel foi iniciado para medir recursos; não há RSS ou tempo experimental novo a relatar. /home observado antes da documentação: 4644024320 bytes (~4,33 GiB); target medido para confirmar ausência de crescimento documental.

### 8. Verificações, arquivos e estado de conclusão

Verificações documentais: branch/HEAD/árvore e referência remota, leitura das fontes requeridas, busca do contrato aplicável, distinção entre regras locais e política geral, cobertura dos sete temas solicitados, rastreabilidade de controles/métricas, links e whitespace. Hashes dos arquivos rastreados foram conservados em /tmp para verificar preservação; somente acréscimo em decisões pode alterar um arquivo existente.

Somente este documento e docs/decisions.md são artefatos autorizados concluídos. **Zero testes experimentais implementados/executados nesta entrega; sem cargo test, regressão cognitiva ou geração de dados finais.** Não há alegação de êxito experimental. A interrupção se aplica à implementação/execução, não impede o versionamento documental expressamente autorizado no procedimento 12.

Próximo passo: decisão sobre o contrato diagnóstico recomendado e retomada desta mesma subetapa, com protocolo operacional completo antes de testes. Não avançar para definir política geral de revisão/abstenção ou avaliação posterior de retenção. Commit, push e estado final do Git são informados após o versionamento; não constituem aprovação das decisões pendentes.
