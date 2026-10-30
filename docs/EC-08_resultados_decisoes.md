# EC-08 — Resultado delimitado de revisão e abstenção

## Parecer e método

Base `40c63847aa12963079de5841ebd8c07807339b62`. [Protocolo prospectivo](protocolos/EC-08_decisoes_revisao_investigacao.md) e `tests/ec08_decisions.rs` usam Monitor/TwoHopTask reais e controle simbólico de implementação separada. **AJUSTAR**: decisões previstas foram reproduzidas para fatos e contextos sintéticos limitados; investigação autônoma não está disponível na interface e não foi demonstrada. Não se avaliou aprendizado metacognitivo ou persistência.

Desenvolvimento e validação ocorreram antes do final; hashes antes de F: protocolo `d81ddc22a39d2ff71f389b80c35817690e94c66a06d8a1c07d46c2b13e5d7f21`, teste `8b81952f890a6c381db19b478c7fed83a644ddcd38f20b7dba35789dda8cfba5`. Os hashes registrados não provam pré-registro independente. D/V/F usam mesmas topologias com IDs diferentes. Gabarito é consultado após decisões; fatos, contextos candidatos e limite de tentativas são os mesmos, e leitura não modifica memória. Controle recebe estrutura simbólica equivalente aos fatos observados, mas pressupostos, autoria e avaliador são comuns; custos de leitura não uniformizados.

## Resultados

12 sondas por repetição, S7 e S11 com duas consultas cada: 14 × duas repetições × duas condições = **56 decisões**, 28 do Monitor e 28 do controle. Concordância **28/28**, todas as decisões corresponderam às expectativas programadas. Por consulta/condição, duas repetições:

| Sonda | Decisão observada do Monitor e controle | Quantidade |
|---|---|---:|
| S0 suporte; S7 depois de evidência fornecida | Resposta no contexto original | 4 |
| S3 contexto alternativo único fornecido | Resposta revisada | 2 |
| S1, S4, S7 antes, S8–S10, S11 primeira e segunda | Abstenção por falta | 16 |
| S2 conflito | Abstenção por contradição | 2 |
| S5 múltiplos contextos | Abstenção por ambiguidade | 2 |
| S6 orçamento de tentativas | Abstenção por limite | 2 |

Total por condição: 4 respostas originais, 2 revisadas e 22 abstenções. Nenhuma resposta ou abstenção divergiu da referência sintética; zero revisões indevidas **segundo os casos definidos**. S0 é resposta sustentada em consulta isolada, não teste longitudinal de manutenção de resposta. S7 compara estados fornecidos externamente; não demonstra aquisição persistente. S11 repete estado e consulta sem evidência nova e conserva abstenção; não cobre todas as revisões possíveis. S8–S10 são metadados do avaliador: Monitor recebe apenas fatos incompletos, sem conhecimento da ação, permissão ou suspeita. Assim não se demonstra decisão de investigar, detecção de permissão ou suspeita de erro. Nenhuma solicitação ou execução investigativa ocorreu; nenhuma ação externa foi realizada.

O teste verifica referências das respostas, decisão, eventos e igualdade de registros entre repetições; monitor processou **36 eventos** (18 por repetição). Registros detalhados e `ReviewReport` foram emitidos em `/tmp/neurons_ec08_decisions.log` nesta execução, não versionados como custódia independente. Referência é a tabela sintética do avaliador, não fonte de verdade externa; não há generalização estrutural ou inferência estatística. Falta/confusão, evidência nova, falso positivo e custo-benefício não são exaustivamente explorados por este painel pequeno.

## Verificações, limites e interpretação

`cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check` passaram: **156 testes aprovados**, zero falhos/ignorados. Há três avisos anteriores de métodos EC-04 não utilizados. Painel isolado: 0,003347 s de parede, RSS máximo `wait4` 7.480 KiB, que pode incluir base do lançador. Guardas internas de 30 s, 256 MiB e /home >=3 GiB passaram; espaço observado inicial 4.634.492.928 bytes e após painel 4.632.866.816 bytes. Sem monitoramento contínuo de disco. Teto de 256 KiB de dados por condição não foi instrumentado integralmente; custo simbólico não foi equiparado a eventos do Monitor.

As referências dos fatos são IDs com origem declarada e tempo zero, não proveniência externa autenticada. Contextos alternativos são fornecidos; o controle tem as mesmas alternativas e orçamento de tentativas, mas compartilha regras e cenários. Sem probabilidades ou calibração. Decisão efetiva e custo pertencem ao Monitor; criação dos fatos, casos e expectativas ao avaliador. Revisar resposta não é adaptação da estratégia nem aprendizagem. A1/A2 orientam memória/proveniência, A3/A4 pedagogia e investigação real, A5/A6 idiomas, A7 rigor e A8 competência generalizável; nenhuma diretriz é demonstrada integralmente por este painel. A1–A8 preservadas.

Subetapa concluída no escopo limitado; próximo passo nominal **medir calibração e custo-benefício**, não iniciado. Necessita de referência independente, política/ação investigativa real quando autorizada, critérios de calibração e custos comparáveis previamente definidos; não se congelam aqui.
