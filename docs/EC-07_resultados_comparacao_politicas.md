# EC-07 — Resultados delimitados da comparação de políticas

## Parecer

**AJUSTAR; comparação com seleção aleatória genuína não comprovada.** O [protocolo prospectivo](protocolos/EC-07_comparacao_politicas.md) foi escrito antes do teste, e desenvolvimento/validação foram executados antes do primeiro final. O painel reproduziu as previsões determinísticas, mas a condição denominada aleatória usa `(seed-1) % n`, sem gerador aleatório ou distribuição probabilística de sementes. É uma sequência balanceada de índices fixos, não evidência de seleção aleatória propriamente dita. O próprio protocolo a denomina pseudoaleatória didática; essa denominação não supre a exigência de aleatoriedade. Não corrigir ou substituir os dados após observar o painel. É necessária comparação prospectiva apropriada antes de considerar concluída a comparação solicitada.

As políticas são código externo de teste, não mecanismo de investigação autônoma no núcleo. As previsões sobre discriminação são fornecidas diretamente às três políticas; não representam informação aprendida ou utilidade calibrada. O avaliador sintético conhece a classe; `choose` recebe apenas ações públicas, orçamento, política e semente, não alvo ou identificador de perfil. Ambiente usa o alvo somente após escolha. Controle e candidato compartilham autoria, gerador, representação e pressupostos; não há independência de implementação ou população externa.

## Painel e resultados

Sete perfis V/F/A/N/B/P/U, dois alvos, quatro sementes e duas repetições: **112 casos por política**, 336 decisões das três condições e 112 do controle sem evidência. Cada linha abaixo apresenta resultados em ambas as repetições somadas, com 16 casos por perfil e condição. Valores entre parênteses são intervenções executadas; cada uma custa uma unidade.

| Perfil | Ativa: corretas (execuções) | Sequência por semente: corretas (execuções) | Fixa: corretas (execuções) | Sem evidência |
|---|---:|---:|---:|---:|
| V: discriminante ID2 | 16 (16) | 4 (16) | 0 (16) | 0 (0) |
| F: discriminante ID0 | 16 (16) | 8 (16) | 16 (16) | 0 (0) |
| A: discriminante ID1 | 16 (16) | 4 (16) | 0 (16) | 0 (0) |
| N: não informativo | 0 (0) | 0 (16) | 0 (16) | 0 (0) |
| B: orçamento insuficiente | 0 (0) | 0 (0) | 0 (0) | 0 (0) |
| P: útil não autorizada | 0 (0) | 0 (16) | 0 (16) | 0 (0) |
| U: sinal inconclusivo | 0 (16) | 0 (16) | 0 (16) | 0 (0) |

Todas as demais decisões foram abstenções, não erros: ativa 64/112, sequência 96/112, fixa 96/112, controle 112/112. Nenhuma resposta incorreta, violação de custo ou seleção de intervenção proibida nos casos executados. Em V/A, abster-se por selecionar opção não informativa perde oportunidade de discriminação; é abstenção apropriada ao sinal recebido, não acerto de identificação. Em U, a ativa escolhe opção esperada informativa mas se abstém diante de `?`. Em P, nenhuma condição executa a ação proibida. Não houve fonte externa ou ação real.

Total de execuções/custo em unidades: ativa 64, sequência 96, fixa 96 e controle 0. Não são comparações de tempo de CPU ou trabalho de seleção. A ativa obteve 48/112 respostas corretas, a sequência 16/112 e a fixa 16/112, mas os casos foram construídos para revelar dependência da posição e a ativa recebe previsão exata de discriminação. Fixa empatou ativa no perfil F. Nenhum resultado comprova vantagem geral, seleção aleatória estatística ou ganho de conhecimento após reinício.

## Repetibilidade e verificações

Os registros de decisões, alvos, sementes, escolhas, sinais e custos são comparados integralmente entre as duas repetições; o teste imprime agregados por perfil e condição. O resultado foi reproduzido no teste isolado; regressão reexecutou o painel e não se soma à contagem acima. Desenvolvimento e validação passaram, com mesmos perfis e sementes diferentes; não há separação de topologias nem avaliação de generalização.

`cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check` passaram; **146 testes aprovados, zero falhos ou ignorados**, incluindo três novos. Três avisos históricos de código não usado na EC-04. Painel isolado `wait4`: 0,004366 s de parede e RSS máximo 7.472 KiB, que pode incluir base do lançador; guarda interna de 30 s e RSS 256 MiB passou. `/home` livre inicialmente 4.624.068.608 bytes e após painel 4.651.057.152 bytes, acima de 3 GiB nas observações; não é monitoramento contínuo. Estimativa de tamanho de linhas não inclui todos os vetores e alocações transitórias: teto fino de 256 KiB por condição não foi integralmente medido, sem excesso observado. `target` após construção: 62.457.227 bytes; crescimento nesta execução não foi medido.

Hashes registrados antes de F e novamente conferidos: protocolo `60d9a08a9a3841d771afa79644d6d5ee968fe98bb94d7d28c7e062fa77e5dd28`; teste `05b14f75164ba81489ff899b479d259f0149fe06de11890a3cf9178f210258a6`. Não há prova independente de pré-registro temporal. A1–A8 são diretrizes futuras preservadas; o painel não demonstra competência cognitiva generalizável A8, aprendizagem, descoberta ou participação neural.

## Pendência

Não iniciar automaticamente a subetapa seguinte. Recomenda-se nova autorização para protocolo de comparação com seleção aleatória real ou PRNG definido, sementes/geração e controles independentes, mantendo os resultados deste painel históricos. A próxima subetapa nominal do plano é **medir ganho informacional e de competência**, não iniciada; depende de uma comparação metodologicamente adequada e de avaliação específica.
