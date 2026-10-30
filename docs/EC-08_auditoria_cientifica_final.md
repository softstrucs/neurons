# EC-08 — Auditoria científica final e encerramento delimitado

## Parecer e sequência

Base `64bdfc35ca9daff0a8da3b4f8a33e5f804d12804`, devs inicialmente limpa e sincronizada. A EC-08 pode ser **encerrada como investigação experimental delimitada, com AJUSTAR e conformidade metodológica parcial**. Não foi identificada falha que invalide as contagens restritas aos painéis. O monitor observa uma inferência efetiva e intervém sobre a decisão conforme regras; isso demonstra utilidade operacional delimitada, sem demonstrar vantagem incremental frente ao controle equivalente nem satisfazer integralmente o critério global do plano para monitoramento útil em geral.

Sequência: [auditoria de monitores](EC-08_auditoria_monitores_existentes.md), commit `7343321`; [estimativas de incerteza e erro](EC-08_estimativas_incerteza_erro.md), `40c6384`; [protocolo de decisões](protocolos/EC-08_decisoes_revisao_investigacao.md) e [resultados](EC-08_resultados_decisoes.md), `acbd0e0`; [protocolo categórico/custos](protocolos/EC-08_calibracao_custo_beneficio.md) e [resultados](EC-08_resultados_calibracao_custo_beneficio.md), `64bdfc3`. As duas primeiras entregas são documentais; as demais são experimentos delimitados. Hashes de protocolos e testes conferem com relatos de congelamento; isso não comprova pré-registro independente.

## Evidências e conformidade

| Item | Evidência reproduzida ou inspeção | Limite |
|---|---|---|
| Processo efetivo | `Monitor::assess` chama `TwoHopTask::evaluate`; relatórios incluem tentativas e eventos. Testes existentes verificam resposta, evidência e falhas. | Processo de dois passos, fatos e critérios programados; não supervisiona cognição geral. |
| Falta, conflito e recursos | `MissingEvidence`, `ContradictoryEvidence`, `ResourceLimited`, `ReviewBudget` e outros motivos são retornados; conflitos referenciam registros. | Ausência de suporte não é falsidade; conflito procura conclusão e premissas da prova selecionada, não todas as alternativas. Não há verificação de verdade externa. |
| Revisão e abstenção | Painel de decisões: 12 sondas, 14 consultas por repetição, duas repetições e duas condições = 56 decisões; concordância **28/28**. Monitor: quatro respostas originais, duas revisões e 22 abstenções; controle igual. | Contextos alternativos e dados são fornecidos. Mudança de resposta não é aprendizagem persistente nem revisão autônoma de estratégia. |
| Investigar | `Decision` expõe responder/abster, sem decisão de investigar. S8–S10 recebem fatos incompletos; permissão e suspeita são metadados externos. | Não se atribuem ao Monitor investigação, seleção de ação, interpretação de permissão ou suspeita. |
| Categorias e referência | Painel de qualidade: 30 decisões por condição, **28/28 avaliáveis** concordantes, dois casos sem referência por condição. Seis respostas, incluindo duas revisões; 22 abstenções avaliáveis. | `sem_referencia` é cópia de caso de falta cuja referência é deliberadamente omitida pelo avaliador; não é evidência de nova topologia. Referência sintética e pressupostos comuns limitam validade externa. |
| Calibração | Interface atual não emite probabilidade. | Calibração probabilística **não mensurável**, não resultado de calibração ruim nem prova de confiança calibrada. Categorias correspondem ao esperado somente nos casos testados. |
| Benefício | Controle recebe fatos, candidatos e orçamento de tentativas equivalentes; mesmas decisões, benefício incremental observado **zero**. | Implementação de controle é separada mas compartilha autoria, regras, casos e referência. Não prova igualdade geral ou inexistência universal de benefício. |
| Custos | Painel reporta 38 tentativas e 36 eventos; tempos de `review` incluem inferência e instrumentação; `symbol` executa operações diferentes. | Custo exclusivo do monitor, memória por condição, avaliador e trabalho uniforme não foram isolados. Não inferir custo-benefício positivo ou negativo definitivo de tempos não equivalentes. |

O controle simbólico opera sobre fatos estruturados equivalentes aos registros observados neste painel, mas não reproduz todas as características de `EngramStore` ou todos os contratos possíveis. O teste de qualidade verifica igualdade de categorias e decisão; referências de resposta são verificadas no painel anterior de decisões, não por um novo avaliador de proveniência independente. S6 é abstenção correta sob orçamento, embora uma resposta pudesse existir sob orçamento maior: perda de oportunidade contrafactual não é erro sob contrato limitado.

Os cenários D/V/F reutilizam topologias e variam IDs; as duas repetições são idênticas. S7 usa evidência adicionada externamente; S11 repete consulta e estado. Não representam amostra independente nem demonstram generalização. Nenhum caso sem referência foi contabilizado como acerto. O resultado histórico de sete casos com controle sem acesso a um contexto alternativo não é evidência de superioridade sob informação equivalente.

## Capacidades e pendências

Demonstrados no escopo: monitoramento por regra de inferência real, detecção de falta de suporte e alguns conflitos, reconhecimento de limites, revisão com contexto fornecido e abstenções reproduzíveis. Não demonstrados: referência independente e erro externo geral, seleção/execução autônoma de investigação, calibração probabilística, benefício incremental no painel, revisão autônoma de estratégia, aprendizagem metacognitiva, autorregulação adaptativa geral, compreensão ou consciência. Nenhum resultado negativo impede encerramento delimitado.

Permanecem para investigação futura, sem implementação autorizada aqui: referências e autoria mais independentes; topologias e contextos inéditos; controle com custos e operações comparáveis; instrumentação isolada; estimativas probabilísticas somente quando justificadas; testes de calibração e consequências em dados adequados; decisões investigativas reais sob autorização e avaliação de persistência. Custos e dados por condição permanecem parcialmente instrumentados.

A1/A2 orientam memória e proveniência; A3/A4 aprendizagem pedagógica e comunicação honesta; A5/A6 pt-BR e expansão multilíngue; A7 rigor; A8 competência generalizável e reconhecimento de limites. Este painel fornece evidência de um componente funcional delimitado, não realização integral dessas diretrizes. Fundamentos e A1–A8 preservados.

## Reprodução e recursos

Reexecutados os dois alvos focais: nove testes passaram (seis no alvo de qualidade, dos quais três são testes históricos incluídos; três no alvo original). Os sumários reproduziram 14 corretas, zero incorretas, um sem referência por repetição, 19 tentativas e 18 eventos por repetição. Os tempos variaram, como esperado, e não fazem parte da igualdade dos registros. Suíte `cargo test --offline -- --test-threads=1`: **162 aprovados, zero falhos ou ignorados**, incluindo três testes históricos duplicados pelo `include!`. Também passaram `cargo fmt --check`, `cargo check --offline --all-targets` e `git diff --check`; três avisos históricos de métodos EC-04 não usados.

Painéis isolados `wait4`: decisões 0,003639 s e 7.400 KiB; qualidade 0,002812 s e 7.416 KiB. RSS pode incluir base do lançador; não isola memória por condição. Guardas internas de tempo/RSS/disco passaram; /home observado inicialmente 4.630.224.896 bytes e após painéis 4.630.167.552 bytes, acima de 3 GiB nas observações, sem monitoramento contínuo. Limite fino de 256 KiB por condição não verificado integralmente. As medições históricas de `review`/`symbol` permanecem no relatório anterior e não são comparações de operações equivalentes.

Encerramento autorizado como investigação delimitada, com pendências, não homologação integral do critério global. Próxima etapa nominal do plano: **EC-09 — Formação pedagógica contínua**, não iniciada.
