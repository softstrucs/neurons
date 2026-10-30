# EC-09 — Auditoria científica final e encerramento documental delimitado

## Parecer e escopo

Base `09ea050052d6936e749db3f8aeba74b057972f89`, devs inicialmente limpa e sincronizada. A EC-09 pode ser **encerrada como etapa de especificação técnico-científica delimitada, com classificação AJUSTAR e conformidade metodológica parcial**. Não foi identificada contradição que invalide a documentação. O aceite global do plano — **melhora sustentada em ciclos sucessivos e adaptação verificável às lacunas** — permanece **não homologado**: não houve experimento integrado de ciclo pedagógico nesta etapa.

As cinco subetapas são diagnóstico documental (`b048dc2`), pré-requisitos (`6e8ce20`), seleção (`6fdf470`), acompanhamento (`6401ed8`) e progressão (`09ea050`). Foram verificadas quanto à coerência conceitual, não homologadas como contrato executável ou capacidades do núcleo.

## Coerência e rastreabilidade

| Fase e documento | Entrada → saída conceitual | Avaliação e dependência |
|---|---|---|
| [Diagnóstico](EC-09_diagnostico_competencias.md) | Objetivo e evidências → estado, lacuna e limites. | Distingue desempenho de competência adquirida; falta instrumento independente e política integrada de diagnóstico. |
| [Pré-requisitos](EC-09_mapear_pre_requisitos.md) | Objetivo/lacuna → relações, origem, contexto e estado de validação. | Necessidade, facilitação e conveniência são distintas; faltam testes causais e regras operacionais para caminhos condicionais. |
| [Seleção](EC-09_selecionar_experiencias.md) | Lacuna, candidatas, pré-requisitos e restrições → escolha justificada ou abstenção. | Compatível com a fase anterior; elegibilidade, desempate e utilidade são diretrizes, não algoritmo ou prova de efeito. |
| [Acompanhamento](EC-09_acompanhar_pratica_avaliacao.md) | Experiência escolhida/executada e estado anterior → evidências, feedback, revisão e avaliação posterior. | Registro proposto cobre execução, autoria, resultados e limites; não existe ciclo integrado ou consolidação funcional demonstrada. |
| [Progressão](EC-09_progressao_competencia.md) | Critério, evidências e pré-requisitos → decisão contextual, pendência ou suspensão. | Exige critério e referência válidos; progressão pedagógica não altera permissões operacionais. Nenhuma política integrada executada. |

O fluxo identifica competência, estado inicial, lacuna, dependências, escolha e execução, evidências, feedback, revisão, retenção/transferência e decisão. Os documentos admitem caminhos alternativos, competências paralelas e pré-requisitos facilitadores sem bloqueio automático. Conflitos exigem mesmo contexto/versão e não são resolvidos automaticamente por recência; versões e histórico são preservados como requisitos propostos. A distinção entre avaliação de desempenho no escopo e alegação de aquisição persistente evita exigir retenção universal para toda progressão estreita; quando retenção ou transferência não forem critérios, sua ausência deve ser declarada.

Lacunas de integração: não há esquema versionado comum, contratos de entrada/saída implementados, identidade de fontes autenticada, política executável de referência/validade, regras congeladas de escolha e reavaliação, métricas ou custos comuns. O modelo sugere conceitos e controles, mas não determina tratamento operacional completo de conflitos, erros de medição, ciclos condicionais ou limites em todas as situações. Essas indefinições são apropriadas ao escopo documental, não prova de que um ciclo futuro funcionará. A separação entre desenvolvimento, validação e final, isolamento do gabarito e distinção de novidade de itens/combinações/estruturas/domínios são requisitos prospectivos; ainda não existe painel EC-09 que os execute.

## Evidências, controles e critério global

`EngramStore`, persistência técnica, `Microcurriculum`, `TwoHopTask`, `Monitor` e `PlasticityRule` são peças existentes: memória recebe observações, inferência usa regras/fatos fornecidos, monitor classifica alguns resultados e feedback é externo. Armazenamento e recuperação não demonstram aquisição pedagógica. A [EC-04](EC-04_auditoria_cientifica_final.md) demonstrou aquisição/revisão **simbólica delimitada** em família fornecida e casos reservados, com ressalvas de controle literal T-C, proveniência/versionamento, abstenções e instrumentação; não demonstrou ciclo pedagógico autônomo ou superioridade neural. EC-05 mostra revisão contextual delimitada, EC-06 composição com regras fornecidas, EC-07 não mostra ganho persistente causado por investigação, e EC-08 não mostra aprendizado metacognitivo ou calibração probabilística. Avaliadores e roteiros externos não são decisões autônomas do núcleo.

Não foram demonstrados diagnóstico independente integrado, validação causal geral de pré-requisitos, seleção adaptativa de experiências, prática/revisão/consolidação pedagógica integrada, progressão autônoma, retenção funcional pedagógica sustentada após intervalo ou transferência generalizável. Isso não apaga retenções e aquisições restritas anteriormente observadas, nem prova incapacidade universal. Exemplos de pt-BR são conceituais; A01 teve gramática/vocabulário controlados, não domínio geral de português.

Controles prospectivos estão descritos, **não implementados nem executados como ciclo EC-09**: referência independente para diagnóstico; mapeado/sem mapeamento, ordem alternativa e pré-requisito incorreto para dependências; seleção fixa/aleatória válida e sem diagnóstico; sem prática ou prática fixa; feedback ausente/atrasado/incorreto e revisão desativada; avaliação reservada, retenção e transferência; progressão por atividade/memorização versus avaliação apropriada. Exigem equivalência de informação e orçamento quando pertinente, estado inicial, referência e critérios congelados, prevenção de vazamento, medição de erros/abstenções e custos. Nenhum controle documental prova que a condição será tecnicamente viável ou estatisticamente suficiente.

## Diretrizes, reprodução e decisão

A1/A2 orientam memória evolutiva e rastreabilidade; A3/A4 formação pedagógica e comunicação honesta de investigação executada; A5 pt-BR e A6 expansão multilíngue; A7 rigor e preservação de resultados; A8 competência generalizável futura. Os documentos são coerentes com essas diretrizes como objetivos, sem demonstrar sua realização. Fundamento e A1–A8 preservados; não se altera arquitetura modular orientada a eventos.

Passaram `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check`: **162 testes aprovados**, zero falhos/ignorados. A contagem inclui três testes históricos duplicados por `include!` em `tests/ec08_calibration.rs`; a suíte é regressão existente, não experimento novo de EC-09. Há três avisos históricos de métodos EC-04 não utilizados. /home observado inicialmente com 4.629.975.040 bytes livres (>3 GiB); não monitorado continuamente. Não se atribui a regressão à demonstração do critério pedagógico global.

Encerramento autorizado **somente documental e delimitado**: especificações e evidências são rastreáveis, com controles e decisões operacionais ainda pendentes. Nenhum resultado experimental positivo é exigido para este encerramento; a aceitação global segue não homologada. Próxima etapa nominal do plano: **EC-10 — Generalização, transferência e cumulatividade**, não iniciada.
