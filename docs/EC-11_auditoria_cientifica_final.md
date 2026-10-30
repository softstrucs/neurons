# EC-11 — Auditoria científica final

Base `8bb77307d4c870aa229b1bffbd50bd9ab8158eeb`, branch `devs`. Esta auditoria lê documentos e código e executa regressões existentes, sem novos experimentos, alterações de produção ou reabertura de etapas anteriores. **Parecer: EC-11 encerrada como investigação técnico-científica delimitada; AJUSTAR; conformidade metodológica parcial. Critério global parcialmente atendido no cenário testado, não homologado como confiabilidade longitudinal geral.**

## Plano e rastreabilidade

O [Plano Executivo v0.2](../neurons_plano_executivo_evolucao_cognitiva_v0_2.md) define o objetivo **“Integrar capacidades demonstradas”** e a aceitação **“resultados reproduzíveis e limites explicitados; integração não pode transformar hipóteses em capacidades demonstradas.”** A tabela do plano indica dependências EC-02–10 e painel longitudinal integrado. A conclusão abaixo restringe o alcance a contratos efetivamente exercitados; as capacidades gerais não foram homologadas.

1. **Mapear interfaces e dependências:** o [mapa](EC-11_mapear_interfaces_dependencias.md) localiza produtores, consumidores, estado, testes e lacunas, incluindo motor, circuito, memória, inferência, monitor, feedback, currículo, sessão e persistência. É levantamento útil e delimitado, não prova de completude de interfaces futuras nem painel integrado abrangente.
2. **Integrar apenas mecanismos validados:** a [matriz e relatório](EC-11_integracao_mecanismos_validados.md) classificam contratos funcionais locais A, capacidades com restrições B e capacidades não validadas C. [`IntegratedState`](../src/integration.rs) conecta motor, memória, monitor e codificação existente; não integra capacidades C. A categoria A refere-se ao contrato restrito, não valida cognição geral. Monitor é fornecido pelo chamador na restauração; checkpoint não o serializa.
3. **Regressão, recursos, rastreabilidade, segurança, retenção e autonomia limitada:** o [protocolo](protocolos/EC-11_confiabilidade_integrada.md), [instrumento](../tests/ec11_reliability.rs) e [resultados](EC-11_resultados_confiabilidade_integrada.md) conectam hipóteses, casos, controles e limites. Os hashes atuais coincidem com os registrados antes do painel; não constituem pré-registro externo independente. A avaliação é parcial: o contrato não oferece versões históricas, permissões integradas ou continuidade neuronal para testar essas dimensões.

## Confronto entre protocolo e instrumento

O painel histórico executou duas repetições da mesma configuração, com dois fatos e dois eventos; verificou bytes iguais, resposta com IDs 1/2, evidência 11, abstenção sem suporte e por orçamento, contexto inválido, fila cheia e ordem de cargas 10/20. Checkpoint corrompido e acima do limite foram recusados. O controle de memória vazia absteve-se. Um processo separado leu o arquivo em cada repetição e reexecutou as verificações com monitor configurado pelo chamador. O processo filho é o mesmo binário de testes; não equivale a reiniciar o serviço completo. A referência tem autoria e pressupostos compartilhados.

**Limitação adicional de H1:** o protocolo pede igualdade de bytes e relatório entre repetições. `frozen_panel` compara bytes explicitamente e verifica em cada execução a decisão e evidências esperadas, mas não compara o `ReviewReport` completo entre as duas repetições. O [teste de integração anterior](../tests/ec11_integration.rs) compara relatório completo antes/depois de uma restauração no mesmo processo, o que não supre a comparação integral H1 entre repetições e processos. Logo, a reprodução de decisão e bytes é sustentada, e a igualdade integral de relatório entre repetições é **parcialmente verificada**. Não foi encontrado motivo para invalidar as observações locais que foram efetivamente assertadas.

O auxiliar `child_restore` normalmente retorna sem exercício quando a variável de ambiente está ausente; sua contagem como teste aprovado na suíte não é observação adicional. As duas repetições e os filhos compartilham cenário e topologia; não são amostras independentes. O painel não valida todos os estados inconsistentes nem políticas de autorização. A função de persistência usa checksum para corrupção acidental, não autenticação contra adulteração deliberada.

## Matriz científica

| Dimensão | Classificação | Evidência e limite |
|---|---|---|
| Reprodutibilidade | Demonstrada no escopo para bytes e decisão | Mesmo cenário; igualdade de relatório completo entre repetições parcialmente verificada conforme acima. |
| Persistência de registros e fila | Demonstrada no escopo | Arquivo lido por processo separado, metadados e despacho verificados. |
| Continuidade de inferência | Parcialmente demonstrada | Consulta fixa reconstruída com monitor equivalente fornecido; não envolve aprendizagem ou reinício do serviço completo. |
| `Circuit`, pesos e estado neuronal | Não avaliável após reinício | Checkpoint cobre somente `Engine` e `EngramStore`; nenhum desses estados neuronais é serializado. |
| Rastreabilidade pública | Parcialmente demonstrada | IDs/contexto e evidência acessíveis; histórico, versões sucessivas, retração e proveniência interna não abrangidos. |
| Segurança local | Parcialmente demonstrada | Corrupção simples, tamanho, entrada inválida, fila e não mutação testados; não demonstra segurança geral nem autenticação. |
| Permissões integradas e autonomia operacional | Não avaliável | Sem contrato global integrado de autorização; não se demonstrou rejeição de ações externas não autorizadas por tal política. |
| Autonomia cognitiva e investigação | Não demonstradas | Monitor segue regras e contextos fornecidos, não escolhe investigação autônoma. |
| Confiabilidade longitudinal geral | Não demonstrada | Uma topologia, intervalo e recursos restritos; sem persistência neuronal ou ciclo pedagógico. |

## Recursos e interpretação

Os valores do relatório histórico são checkpoint **376 bytes**, codificação **10–16 µs**, restauração **8–9 µs**, sequência interna **2.389 µs**, aproximadamente **2,4 ms**, e RSS máximo externo **11.208 KiB**. Tempos vêm de relógio monotônico; RSS de `getrusage(RUSAGE_CHILDREN)`, não de isolamento do adaptador. O utilitário `/usr/bin/time` estava indisponível; medição pode incluir instrumentação ou memória herdada, sem amostragem contínua. Valores são pontuais, dependentes do ambiente e componentes envolvidos, sem evidência de custo isolado, escala, ou desempenho geral. A verificação de teto se limita às observações disponíveis.

## Parecer e dependências

As evidências atendem **parcialmente e apenas no cenário restrito** ao critério de resultados reproduzíveis e limites explicitados: há reprodução de bytes, decisões e restauração de registros; H1 integral, cobertura longitudinal e controles abrangentes permanecem incompletos. A integração não converte hipóteses em capacidades demonstradas no relatório. Não homologar confiabilidade geral ou autonomia integrada.

EC-08 não demonstrou investigação autônoma ou metacognição geral; EC-09 não demonstrou aprendizagem pedagógica contínua; EC-10 não demonstrou transferência genuína, cumulatividade, interferência compartilhada nem retenção neuronal após reinício. Ausência de evidência não prova impossibilidade geral. Dependências futuras incluem especificação e validação de estado neuronal persistente se necessário, versões e histórico, avaliação de permissões e segurança, referências independentes, repetição em estruturas diferentes e instrumentação isolada de custos. Não se inicia sua implementação aqui.

A1–A2 orientam memória e proveniência; A3–A4 pedagogia e investigação real; A5–A6 idiomas; A7 rigor; A8 competência generalizável como objetivo. Essas diretrizes não são capacidades demonstradas por esta etapa. Fundamentos, arquitetura orientada a eventos, resultados e auditorias anteriores permanecem preservados.

Não há falha identificada que invalide as observações restritas ou impeça seu encerramento documental. **EC-11 encerrada como investigação técnico-científica delimitada, AJUSTAR, conformidade metodológica parcial; critério global parcialmente atendido no cenário restrito e não homologado para confiabilidade longitudinal geral.** A próxima etapa nominal é EC-12 — Consolidação, especialização futura e questões abertas, não iniciada.

## Verificações desta auditoria

`cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` passaram. Regressão: **185 testes aprovados, zero falhos e um ignorado**; `r3_frozen_final` EC-10 permanece ignorado por quota insuficiente, não aprovado. Inclui auxiliar inerte e duplicações históricas; não é novo experimento. Três avisos históricos EC-04 de código não utilizado. `/home` observado com aproximadamente 4,4 GiB livres, sem monitoração contínua. Links locais e hashes do protocolo e instrumento conferidos.
