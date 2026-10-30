# EC-07 — Resultado complementar do controle pseudoaleatório R1

## Parecer

**AJUSTAR**, com comparação reproduzida no painel sintético e limitações estatísticas e científicas. O controle anterior usava `(seed-1)%n`, sequência determinística sem PRNG; permanece registrado como comparação incompleta. O [protocolo R1](protocolos/EC-07_controle_aleatorio_R1.md) foi redigido antes do teste e não altera os [resultados históricos](EC-07_resultados_comparacao_politicas.md). SplitMix64 e rejeição de módulo substituem prospectivamente esse controle. Vetores conhecidos para semente zero passaram, assim como cobertura das posições para n=2 e n=3 entre sementes 1–32; cobertura não prova uniformidade estatística nem independência dos fluxos. Sementes fixas são distintas, não amostra aleatória independente.

O selecionador recebe candidatos, custo, autorização e previsões para H0/H1, sem alvo, perfil ou resultado futuro; os três compartilham os mesmos dados públicos e orçamento. O ambiente consulta o alvo apenas após escolha. Assertivas verificam permissões, orçamento e ausência de mutação dos candidatos. O gabarito é comparado após a decisão. Previsões discriminantes exatas são fornecidas, e os controles compartilham gerador, autoria e pressupostos: não são uma validação independente de conhecimento ou utilidade informacional.

## Resultados

Sete perfis × dois alvos × 32 sementes × duas repetições = 896 casos por política. As três condições somam **2.688 decisões**; controle sem evidência adiciona 896. Cada valor abaixo soma as duas repetições, com 128 casos por perfil/condição.

| Perfil | Ativa corretas | PRNG corretas | Fixa corretas | Resultado negativo/abstenção |
|---|---:|---:|---:|---|
| V, ID2 | 128 | 36 | 0 | Demais abstêm após sinal ausente. |
| F, ID0 | 128 | 40 | 128 | Fixa iguala ativa. |
| A, ID1 | 128 | 52 | 0 | Demais abstêm após sinal ausente. |
| N, sem discriminação | 0 | 0 | 0 | Todas abstêm. |
| B, orçamento insuficiente | 0 | 0 | 0 | Nenhuma executa intervenção. |
| P, ação útil proibida | 0 | 0 | 0 | Nenhuma executa ação proibida. |
| U, sinal inconclusivo | 0 | 0 | 0 | Todas abstêm. |

Agregados: ativa **384/896** corretas, PRNG **128/896**, fixa **128/896**, controle **0/896**; nenhuma resposta errada ou violação observada. Abstenções: 512, 768, 768 e 896, respectivamente. Intervenções/custo: 512, 768, 768 e 0 unidades, respectivamente. Abstenção após escolher ação não informativa é adequada ao sinal, mas representa oportunidade perdida, não identificação correta. As duas repetições são iguais e não aumentam independência.

Para cada semente, a posição PRNG entre três elegíveis (0/1/2) foi:

```text
1:2 2:1 3:0 4:1 5:2 6:2 7:0 8:1
9:1 10:1 11:0 12:0 13:1 14:2 15:2 16:2
17:0 18:1 19:0 20:0 21:1 22:2 23:1 24:0
25:0 26:1 27:1 28:1 29:2 30:2 31:0 32:1
```

Esta tabela determina o resultado por semente nos três perfis: posição 2 acerta V, 0 acerta F e 1 acerta A; ativa acerta os três e fixa apenas F. Cada semente PRNG acerta exatamente um dos três perfis, ativa três e fixa um. Contagens de posições 0/1/2 são 10/13/9 de 32. Ativa supera a PRNG em 32/32 sementes no agregado V/F/A; por perfil empata nas sementes que escolhem a posição correta e supera nas demais. Não há casos inferiores neste painel. Alvos, perfis e repetições reutilizam sorteios e não são observações independentes; sem intervalo de confiança ou extrapolação populacional.

## Verificações e limites

Vetor de referência e desenvolvimento (101–104) e validação (201–204) passaram antes do painel final. Protocolo SHA-256 antes de F: `e3982fa6e45e1a510c35d55c4c5164f7754802a4106ee0218fc0ff0562b9a3fa`; teste `12357d960dfc809a381c8a539efda7b08b2f86eb5164fa58c797f6bdfcc06b25`. Essas anotações não constituem pré-registro independente. Registros completos por caso foram emitidos em `/tmp/neurons_ec07_r1.log` nesta execução, não arquivados como prova histórica. As duas repetições compararam registros integralmente.

Passaram `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check`: **150 testes aprovados, zero falhos/ignorados**. Painel isolado: 0,014396 s de parede e RSS máximo `wait4` 7.400 KiB, podendo incluir base do lançador; guardas internas de 30 s, 256 MiB e 3 GiB passaram. `/home` observado inicialmente 4.646.289.408 bytes e após painel 4.642.004.992 bytes; não é monitoração contínua. O limite fino de 256 KiB por condição não foi instrumentado integralmente, portanto permanece não verificado. Custos de CPU e trabalho das políticas não foram equalizados ou medidos em unidade comum.

A1–A8 preservadas. Seleção programada de previsão fornecida não demonstra aprendizagem persistente, descoberta autônoma, generalização estrutural, compreensão ou vantagem neural. A correção metodológica da condição PRNG está demonstrada no escopo funcional; a aceitação global EC-07 e ganho de competência permanecem não demonstrados. Próxima subetapa nominal **medir ganho informacional e de competência**, não iniciada.
