# EC-07 — Ganho informacional e competência: resultado delimitado

## Definição e parecer

Base `8b961748ed5e1a1924d2783ec5cd66598d487002`. [Protocolo prospectivo](protocolos/EC-07_ganho_informacional.md) congela quatro condições com universo fornecido H={0,1}. A incerteza é o número de hipóteses compatíveis, não probabilidade ou entropia. Ganho é a redução de cardinalidade quando o conjunto posterior é não vazio. Contradição que elimina todas as hipóteses é inconclusiva, com ganho **indefinido**, não ganho 2. Razão por custo não é calculada quando custo é zero ou ganho indefinido.

**AJUSTAR**: o cálculo externo reproduz as previsões no painel sintético; não se demonstrou ganho de competência do Neurons. As condições são intervenções fixadas pelo protocolo, não comparações novas das políticas ativa, PRNG e fixa. O avaliador implementa a atualização dos conjuntos a partir de previsões e evidências sintéticas fornecidas; não se atribui essa atualização ao núcleo.

## Resultados efetivamente executados

Desenvolvimento e validação passaram antes do primeiro final. O final compreende 16 IDs × dois alvos × quatro condições × duas repetições = **256 casos**, 64 por condição. Registros de estados, evidências sintéticas e custos são comparados integralmente entre repetições em memória. IDs são novos, mas as mesmas hipóteses e padrões reaparecem; alvos e repetições não são amostras independentes.

| Condição | Casos | Ganho total / definido | Ganho por caso | Custo total | Ganho por custo | Abstenções |
|---|---:|---:|---:|---:|---:|---:|
| Evidência discriminante | 64 | 64 / 64 | 1 | 64 | 1 | 0 |
| Sem evidência | 64 | 0 / 64 | 0 | 0 | indefinido | 64 |
| Evidência repetida | 64 | 64 / 64 | 1; repetição acrescenta 0 | 128 | 0,5 | 0 |
| Contradição | 64 | indefinido / 0 | indefinido | 64 | indefinido | 64 |

A soma impressa como zero no teste para a condição contraditória é soma de nenhum valor definido; **não é um ganho observado de zero**. Em todos os casos contraditórios o conjunto ficou vazio e houve abstenção. As condições partem da mesma informação, mas recebem intervenções diferentes; seus custos e evidências são deliberadamente distintos. Não há fonte externa ou ação real.

A redução H={0,1} para um candidato demonstra apenas filtragem segundo uma regra e sinal programados. Não comprova verdade externa, calibração probabilística, seleção autônoma, atualização persistente ou benefício para competência. EC-04 já registrou aquisição simbólica delimitada em família fornecida, com ressalvas próprias; ela não foi causada ou avaliada por esta intervenção. Não existe aqui estado adquirido persistente, teste posterior inédito ou controle adequado que sustente ganho de competência; esse ganho permanece **não mensurado e não demonstrado neste painel**, sem negar os resultados históricos da EC-04.

## Verificações, recursos e custódia

Passaram `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check`: **153 testes aprovados**, zero falhos ou ignorados. Os testes de desenvolvimento e validação usaram IDs separados; final foi reexecutado pela regressão, sem somar suas consultas. Painel isolado: 0,008793 s de parede e pico `wait4` 7.384 KiB, podendo incluir base do lançador. Guardas internas de 30 segundos, 256 MiB e 3 GiB passaram. `/home` observado no início 4.626.034.688 bytes e após painel 4.622.696.448 bytes; não monitorado continuamente. Limite fino de 256 KiB por condição não medido integralmente; custo computacional da filtragem não foi uniformizado.

Protocolo e teste identificados por SHA-256 antes do primeiro final e conferidos depois: `91495e4c564a3d5b5e0e13433bf646ec69131d56211b988532ec34d875ebb352` e `21ad65d35ee01f18e70141ad31a56b27c168fabd355374b8e5423131051ea197`. Isso não prova pré-registro independente. Registros detalhados são mantidos em memória e comparados no teste, não arquivados individualmente; agregados estão neste relatório. Políticas e resultados R1 permanecem históricos.

A1/A2 orientam memória e proveniência; A3/A4 pedagogia e investigação real; A5/A6 idiomas; A7 rigor; A8 competência generalizável futura. Nenhuma é demonstrada integralmente por esta filtragem. Diretrizes preservadas. Próximo passo: revisão científica da EC-07 e suas limitações, sem iniciar outra etapa.
