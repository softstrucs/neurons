# EC-11 — Integração delimitada de mecanismos validados

Base `c324f779b8fe0dd1104c1e2d933c2bd8a31d2920`. A matriz foi estabelecida antes da implementação; A indica validação funcional para o contrato delimitado, B validação parcial sob restrições, C ausência de validação para integração funcional da capacidade proposta. Nenhuma classe certifica cognição geral.

| Mecanismo | Classe, evidência e interface | Estado/dependências e restrições |
|---|---|---|
| `Event`/`Engine` | A: ordem, limites e despacho em [testes](../tests/engine.rs); `enqueue`, `run_cycle` | Fila, tempo e métricas; não valida autonomia ou segurança global. |
| `EngramStore` | A: recuperação contextual e ambiguidades em [testes](../tests/memory.rs); `record`, `recall` | Registros fornecidos, proveniência limitada; armazenamento não é aprendizagem. |
| Checkpoint `Engine` + `EngramStore` | A: round-trip e validações em [testes](../tests/persistence.rs); `encode`/`decode` | Só esses estados, limite 256 KiB, FNV não autentica; não salva `Circuit` ou monitor. |
| `TwoHopTask` e `Monitor` | B: inferência de dois passos e revisão/abstenção delimitadas; [EC-06](EC-06_auditoria_cientifica_final.md), [EC-08](EC-08_auditoria_cientifica_final.md), [testes](../tests/metacognition.rs) | Regras e contextos fornecidos, sem investigação autônoma, probabilidades calibradas ou vantagem geral; configuração do monitor precisa ser fornecida novamente. |
| `Circuit`, neurônios e sinapses | B: propagação em [testes](../tests/cognition.rs); usados pela inferência reconstruída | Atividade sob circuitos programados; estado/pesos não persistidos pelo checkpoint. Não integrar como continuidade neuronal. |
| `PlasticityRule`, currículo, linguagem | B: interfaces e testes locais em [mapa](EC-11_mapear_interfaces_dependencias.md) | Feedback e conteúdo externos; não demonstram ciclo pedagógico, retenção ou competência linguística geral; não incluídos no adaptador. |
| Revisão histórica geral, investigação autônoma, aprendizagem contínua, transferência e cumulatividade | C: [EC-08](EC-08_auditoria_cientifica_final.md), [EC-09](EC-09_auditoria_cientifica_final.md), [EC-10](EC-10_auditoria_cientifica_final.md) | Não demonstradas; não integradas ou simuladas por avaliador externo. R3 final não foi executado. |

## Contrato da integração

A escolha prospectiva é um adaptador mínimo de `Engine`, `EngramStore` e `Monitor` com codificação/decodificação do checkpoint existente. O chamador fornece a configuração do monitor na construção e restauração; ela não é serializada. São expostas referências ao motor e à memória e consulta somente leitura via monitor; checkpoint não toca em `Circuit`, pesos ou estado neuronal. `decode` produz objeto novo somente após validação; erro não modifica o objeto anterior. Sem operações externas implícitas, permissões ampliadas ou persistência de arquivos adicionada. Essa fronteira é integração funcional restrita, não ciclo longitudinal geral.

Invariantes verificáveis: ordem/identidade de eventos na fila restaurada; registros contextuais e IDs de evidência; resultado e abstenção em casos explícitos; consultas não alteram bytes do checkpoint; erro de checkpoint não altera estado original; limites existentes propagam falhas. Não são implementadas versões contextuais, retração, histórico geral, autenticação ou permissões globais. A restauração da memória permite reconstruir inferência programada por consulta, mas não preserva pesos ou circuito. Comparar monitor restaurado pressupõe configuração idêntica do chamador; mudança de configuração não é continuidade garantida.

## Implementação e resultados

[`IntegratedState`](../src/integration.rs) recebe motor, memória e monitor; disponibiliza motor e memória explicitamente, `review` com referência imutável e checkpoint em bytes. A restauração delega a validação integral ao formato existente e recebe novo monitor do chamador. Não há arquivo escrito automaticamente, checkpoint neuronal, migração, autenticação, ou efeito colateral externo. A API não impõe política de permissões ou orçamento global; os limites são os dos componentes subjacentes.

Dois [testes focais](../tests/ec11_integration.rs) passaram: (1) eventos de mesmo tempo mantêm destino, carga e ordem após restauração, fila cheia é rejeitada, IDs de premissa e evidência sobrevivem, contexto ausente gera abstenção e consulta não muda bytes; (2) checkpoint corrompido é recusado sem modificar objeto original e memória vazia conduz à abstenção. São testes de integração com referência explícita de expectativas e critérios programados, não avaliação científica independente de aprendizagem, transferência ou persistência após reinício de processo. A configuração de monitor foi repetida explicitamente nos testes; não é parte do checkpoint. Contexto não é versão temporal; revisão/retração persistente, histórico e permissões permanecem fora do contrato.

Resultado: integração funcional **delimitada, AJUSTAR**, com critérios locais dos testes atendidos e critérios globais da EC-11 pendentes. EC-08 mantém limites de monitor, EC-09 não demonstrou ciclo pedagógico e EC-10 não homologou transferência; A1–A8 permanecem diretrizes futuras. A próxima subetapa nominal é **executar testes de regressão, recursos, rastreabilidade, segurança, retenção e autonomia limitada**, não iniciada como bateria científica nesta execução.

## Verificações e recursos

`cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline` e `git diff --check` passaram. Regressão: **183 aprovados, zero falhos, um ignorado**; inclui dois testes novos focais e duplicações históricas. O ignorado é o painel final R3 da EC-10, bloqueado pela quota, não um resultado aprovado. Três avisos históricos EC-04 de métodos não usados; `/home` observado com aproximadamente 4,3 GiB livres, sem medição contínua ou instrumentação isolada de custo, RSS e tempo da integração.
