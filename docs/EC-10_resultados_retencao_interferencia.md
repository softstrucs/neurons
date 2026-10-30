# EC-10 — Resultado diagnóstico de retenção e revisão simbólica

## Parecer e não conformidade

Base `8c54697e2575c7c3d1505630a046b5ff7cc437fb`. **AJUSTAR; avaliação final independente não demonstrada.** O [protocolo](protocolos/EC-10_retencao_interferencia.md) foi redigido antes do teste, mas repete deliberadamente os mesmos vetores, critérios e gerador entre desenvolvimento, validação e F, mudando apenas IDs de contexto. Isso **não satisfaz a separação de exemplos entre desenvolvimento, validação e final solicitada**. As assertivas e contagens abaixo são diagnósticas; não constituem avaliação final independente, generalização estrutural ou demonstração do aceite EC-10. Nenhum dado ou critério foi alterado após observar F; correção exige novo protocolo e conjunto independente, sem reescrever este resultado.

O ensaio reutiliza protótipo simbólico isolado EC-04, não mecanismo de aprendizagem do núcleo. O estado da instância muda por episódios com rótulos fornecidos; máscaras são acessíveis somente ao avaliador e rótulos finais não entram no treino. Estados A e B ficam em contextos internos distintos, sem identidade de pesos compartilhados. O intervalo é real >=25 ms antes de B, verificado por relógio monotônico, mas seu valor exato não foi impresso; não é retenção de longa duração. Não há reinício nem persistência em disco do protótipo.

## Resultados por repetição

Duas repetições produziram escores iguais; cada fase abaixo é por repetição. Há 32 vetores reservados nas fases 0–3 e oito na fase 4; quatro condições, total **1.088 consultas** nas duas repetições. Os mesmos vetores reaparecem nas reavaliações, sem serem reapresentados como treino.

| Fase | Candidato e indutor independente | Literal | Sem aquisição / revisão congelada |
|---|---|---|---|
| Antes de A, 32 | 0 acertos, 0 erros, 32 abstenções | 0/0/32 | 0/0/32 |
| Após treino A, 32 | 32 acertos, 0 erros, 0 abstenções | 0/0/32 | 0/0/32 sem aquisição |
| Após intervalo e treino B, 32 | 32/0/0 | 0/0/32 | 0/0/32 sem aquisição |
| Após revisão A, referência antiga, 32 | 28 acertos, **4 divergências**, 0 abstenções | 0/0/32 | 32/0/0 congelado |
| Após revisão A, referência revisada, 8 | 8/0/0 | 0/0/8 | 4 acertos, 4 erros, 0 abstenções congelado |

Retenção pareada após B: **32/32** respostas anteriormente corretas preservadas por repetição, zero decisões alteradas. Isto cobre estado na mesma instância, intervalo curto e B em contexto separado; não demonstra retenção após reinício ou resistência à interferência entre pesos compartilhados. Após revisão, quatro respostas divergem do critério antigo; no novo critério oito de oito consultas revisadas estão corretas, contra quatro de oito no estado congelado. A perda em referência antiga é consequência observada sob revisão deliberada de critério e não prova interferência espontânea. Estado público do candidato passa versão 32/start 0 para versão 48/start 29, com uma hipótese em ambas fases; o controle independente apresenta os mesmos agregados e decisões. Consulta não altera estado (`clone` comparado).

O literal recebe os mesmos episódios acessíveis mas apenas busca igualdade de vetor; nenhum dos vetores consultados está cadastrado naquele contexto, por isso se abstém. O controle sem aquisição não recebe os episódios; o congelado não recebe revisão e portanto **não tem a mesma informação após revisão**. O indutor simbólico independente recebe as mesmas entradas e concordou nas decisões, com pressupostos e autor compartilhados. Teste compara diretamente somente `Outcome`; versões, início e quantidade de hipóteses aparecem como agregados iguais, mas não são comparados campo a campo por consulta. Não existe aqui controle relacional de composição: a tarefa é classificação em família de critérios. Nenhuma superioridade neural, transferência entre tarefas/domínios ou cumulatividade foi demonstrada.

## Rastreabilidade, recursos e limites

D e V passaram antes de F, porém reutilizam os mesmos dados do painel diagnóstico, conforme não conformidade acima. A comparação de renomeação D/V/F prevista no protocolo **não é assertada entre execuções**; apenas as duas repetições F com mesmos IDs são comparadas. A contagem de trabalho é a fornecida pelos componentes EC-04 e conserva suas limitações de instrumentação, não sendo custo uniforme ou causalmente equivalente. O teste não calcula um identificador de evidência independente nem autentica referência externa.

Protocolo SHA-256 antes de F `6d35829da5e46f829f918e3e6b1ce2e8a17be6e92d9590ae1e88053c713451a8`; teste `ad36e3429f5d6dacd31593711f2651d7985db3546196b77438ef888afdd3108d`. Isso não comprova pré-registro independente. Painel isolado 0,242922 s de parede e RSS máximo `wait4` 7.480 KiB, incluindo possível base do lançador; guardas internas de tempo/RSS/disco passaram. /home observado inicialmente 4.628.205.568 bytes e após painel 4.625.395.712 bytes; sem amostragem contínua. Teto fino de 256 KiB por condição não medido integralmente. Saída agregada em `/tmp/neurons_ec10_final.log` não é arquivo de custódia independente.

Passaram `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check`: **166 testes aprovados**, zero falhos/ignorados. A suíte inclui três duplicações históricas EC-08 e um teste incorporado do gerador EC-04 no novo alvo; resultados de regressão não são novas observações independentes. Três avisos históricos EC-04 de código não usado.

A1/A2 orientam memória e proveniência, A3/A4 pedagogia e investigação real, A5/A6 idiomas, A7 rigor e A8 competência generalizável futura; nenhuma é integralmente demonstrada aqui. EC-10 permanece aberta. Necessário novo conjunto verdadeiramente separado, controles de renomeação verificáveis e avaliação de retenção/interferência apropriada antes de alegações amplas; não iniciar outra etapa automaticamente.
