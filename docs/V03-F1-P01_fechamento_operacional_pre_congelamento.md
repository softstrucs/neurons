# V03-F1-P01 — Fechamento operacional pré-congelamento

Base `db80da697966dc0582ab1abcdd7be929030691aa`, `devs`, inicialmente sincronizada e limpa. Autorização humana limitada a revisão, fiscalização, tentativa de medição, especificação e D/V. Fontes: [plano aprovado](planos/Neurons_Plano_Executivo_Evolucao_Cognitiva_v0_3_APROVADO.md), [protocolo histórico](protocolos/V03-F1-P01_confiabilidade_experimental_inicial_PROPOSTA.md), [E01](protocolos/V03-F1-P01_E01_identidade_S2_S3.md), [V1–V3](V03-F1-P01_validacao_preliminar_instrumentos.md), [auditoria V4](V03-F1-P01_auditoria_prontidao_pre_final.md), [D01–D04](V03-F1-P01_decisoes_metodologicas_pre_finais_APROVADAS.md) e [decisões](decisions.md). Nenhum F gerado, consultado ou executado; gerador permanece limitado a D/V. Não houve revisão externa independente.

## Revisão crítica da referência

| Pressuposto | Evidência e controle disponível | Risco de erro comum / limitação residual |
|---|---|---|
| Separação de decisão | `reference.rs` calcula expectativas a partir do manifesto e registros públicos; não chama review, TwoHopTask ou validador do candidato. | Compartilha `Fields`, dados e especificação; autoria e revisão externa independentes não demonstradas. |
| Contextos e sinais | Filtros exatos contexto/origem/destino, sinais ±1; S2/S3 e S4. | Mesma semântica formal dos dois lados; E01 é equivalência estrutural e não estado literalmente idêntico. |
| IDs e referência | Comparação dos campos dos registros, contexto e ID alterados em D/V. | ID não autentica origem; ordenação e IDs únicos pressupostos. |
| `facts_loaded` | V4 soma positivos válidos do contexto inclusive relação adicional; controle adulterado rejeitado. | Contagem é fórmula compartilhada de contrato e limitada aos manifestos Observed/Validated, não generaliza a estados arbitrários. |
| Evidência ausente/divergente | S5 e remoção da evidência produzem incompleto; valor divergente gera inválido. | Não mede autenticidade de fonte; dados de entrada e pressupostos comuns podem errar juntos. |
| Conflito e abstenção | Sem positivo: MissingEvidence; positivo e negativo no mesmo contexto: ContradictoryEvidence; classes S3/S4/S6. | Outras topologias, múltiplas tentativas e configurações não cobertas. |
| Adulterações | D/V exercitam troca de contexto/ID e remoção de referência; campo ausente/extra e contagem alterada. | Sensibilidade pontual não prova ausência de falso negativo geral; parser estrutural não é validador semântico universal. |

A revisão crítica é documental e executada na mesma área de trabalho. Não usa resultado do candidato como gabarito. Não identificou dependência circular de decisão no recorte, nem justifica alegação de independência absoluta; verificação humana apropriada dos pressupostos continua possível e nenhuma foi inventada.

## Fiscalização operacional

O novo auxiliar `tests/support/v03_f1_p01/watchdog.py` aceita somente D, V e dois controles próprios; não habilita F nem IDs finais. Usa relógio monotônico desde antes de iniciar o processo de teste até sua saída, incluindo auxiliares do grupo de processos; ultrapassado o limite, envia SIGKILL ao grupo, espera o principal e registra `timeout`, distinto de falha científica. Interrupção é abrupta, adequada aos testes locais sem arquivos persistentes próprios; a árvore precisa ser verificada após interrupção e processos que alterem grupo não são cobertos. O processo supervisor e sua própria memória ficam fora do grupo.

RSS soma `VmRSS` observado em `/proc` para membros do grupo, com intervalo solicitado de 2 ms mais duração variável da varredura; registra máximo **amostrado**, não pico garantido. Detectado excesso, encerra o grupo e registra `rss_sample_limit`. Descendentes que escapem do grupo, desapareçam entre leituras ou vivam menos que uma varredura podem ser perdidos; RSS compartilhado pode ser contado mais de uma vez e a soma não é memória privada. A soma inclui principal e filhos observáveis, não o supervisor. A verificação não constitui contenção rígida nem prova de pico simultâneo verdadeiro. A montagem cgroup v2 local é somente leitura e `/sys/fs/cgroup` não é gravável; não houve alteração global ou pedido de privilégios. **BLOQUEIO OPERACIONAL:** esta instrumentação isolada não garante fiscalização adequada do teto RSS de 256 MiB para execução final. Uma contenção ou fiscalização suficiente, com escopo e falhas documentados, é pré-requisito pendente.

Controles locais: timeout sintético de 0,15 s interrompido em 0,153419 s, código −9; limite sintético de 8 MiB disparou em RSS amostrado 8.642.560 bytes, 0,010030 s, código −9. Esses controles mostram sensibilidade nas condições testadas, não limite garantido ou resultado científico.

## D/V e espaço

Sem modificar o instrumento de inferência ou a referência, D e V foram repetidos sob supervisor auxiliar: cada partição exibiu 54 consultas principais, 36 comparações e nove adulterações; controles adicionais V4 também permaneceram no teste. D: 28.256 µs internos, 0,030104 s externos, RSS simultâneo amostrado 3.039.232 bytes; V: 28.115 µs, 0,031427 s, 2.695.168 bytes. Até dois processos simultâneos foram observados em cada amostra; isso não conta todos os filhos de vida curta. Checkpoint máximo exibido 272 bytes, não dados vivos. Histórico V1–V4 está preservado: o supervisor é instrumento adicional, não substitui aquelas versões. D/V não são F.

`df -B1 /home` indicou 4.098.715.648 bytes disponíveis nesta verificação, acima de 3 GiB. Conferir novamente imediatamente antes de eventual execução; abaixo de 3 GiB impede início. Nenhum arquivo fora do escopo do projeto foi modificado intencionalmente para essa verificação.

## Dados vivos

Para o teto por condição, seria necessário contabilizar simultaneamente estado vivo do Engine e EngramStore, registros, configuração do Monitor, estruturas do candidato, referência e manifesto, relatórios, buffers/serialização, temporários e cópias coexistentes em todos os processos pertinentes, com regras explícitas para compartilhamento e alocação. Excluir binário, bibliotecas, pilhas, alocador ou supervisor só seria aceitável com definição operacional justificada; nenhuma dessas exclusões transforma RSS ou checkpoint em dados vivos.

As interfaces públicas não expõem alocações e vida de cada objeto, temporários ou heap dos descendentes. Contagem de bytes serializados e RSS não mede esse conjunto; um alocador instrumentado no harness não garante contabilização completa entre processos nem das alocações internas sem alterações adicionais e definição validada. **NÃO MENSURÁVEL com os instrumentos atuais** de forma confiável e não invasiva. Não se afirma conformidade com 256 KiB. Alternativas D04 continuam abertas: instrumento confiável e definição operacional, ou decisão humana explícita de retirar o teto dos critérios indispensáveis à alegação local, mantendo restrição não verificada. Nenhuma alternativa foi aprovada aqui.

## Manifesto, versões e condicionantes

A [especificação pré-congelamento](protocolos/V03-F1-P01_especificacao_manifesto_pre_congelamento.md) registra algoritmo, classes, configuração, quotas, controles e interrupções sem materializar F. SHA-256: supervisor `689c1febad2c560ec9b14e27f2d45654d867e73974d180eedcc0cb4ca738b0b7`; instrumento D/V `4a2c759a22b990f7c8d1086dd82d302849931dbecafcaa7a68bc1fd5a9890ae9`; esquema sintético `5426befa9eb92c27984e26d33fa624f740734b3a48db60f98e061136fd995651`; canônico `6242a13d4bcae538582a690aa99ba55bede5620e2c36797b75948a7d91bbdd71`; referência `e8f92cd56b86ebb78e0af1ccbe0c3a6644fa3c3bc213f7379cde762c38d9e10a`. São identificadores de versão, não congelamento.

C01 dependências por alegação; C02 não aplicável sem alegação neuronal causal; C03 permissões integradas e persistência neuronal não avaliáveis; C04 registros restaurados não provam retenção cognitiva ou transferência; C05 congelamento prospectivo permanece pendente. D01–D04 e E01 preservadas. Não se demonstram cognição geral, autonomia, autenticidade de fonte ou A8.

## Matriz de prontidão

| Requisito | Estado | Motivo |
|---|---|---|
| Revisão crítica e separação do oráculo | ATENDIDO COM RESTRIÇÕES | Pressupostos e riscos comuns documentados; sem revisão externa independente. |
| Tempo e interrupção | ATENDIDO COM RESTRIÇÕES | Monotônico e controle positivo; grupo não cobre descendentes que escapem. |
| Fiscalização RSS | BLOQUEADO | Cgroup não gravável; amostragem não garante o teto. |
| Dados vivos por condição | NÃO MENSURÁVEL | Sem contabilidade integral confiável; alternativa D04 depende de decisão humana. |
| Espaço em disco | ATENDIDO COM RESTRIÇÕES | Verificado pontualmente; repetir antes de execução. |
| Manifesto e critérios | ATENDIDO COM RESTRIÇÕES | Especificação preparada, não materializada ou congelada. |
| D/V e integridade | ATENDIDO COM RESTRIÇÕES | Repetidos com supervisor; mesma topologia e especificação. |
| F, congelamento e autorização | PENDENTE | F não habilitado, congelamento e execução não autorizados. |

**Parecer: C. BLOQUEADO POR IMPEDIMENTO OPERACIONAL.** Antes de qualquer decisão sobre F, resolver fiscalização RSS e tratamento dos dados vivos, verificar novamente recursos e integridade, formalizar congelamento prospectivo e obter autorização humana específica. O bloqueio não altera decisões históricas nem autoriza implementação adicional. As verificações de regressão não são novos experimentos F.

Verificações: `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` aprovados; **190 testes aprovados, zero falhos, um ignorado** (`r3_frozen_final` EC-10). A suíte repetiu D/V e testes históricos, não F deste protocolo; há três avisos históricos de método não utilizado na EC-04.
