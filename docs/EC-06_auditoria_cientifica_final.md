# EC-06 — Auditoria científica final e encerramento delimitado

## Parecer e rastreabilidade

Na base `92391348234109cc4c8e026dd63aaa3e7d41ba3e`, devs estava limpa e sincronizada. A EC-06 pode ser **encerrada como investigação experimental delimitada, com classificação AJUSTAR e conformidade metodológica parcial**. As assertivas do painel foram reproduzidas; suas limitações não invalidam os resultados restritos aos casos avaliados, mas impedem aceite de raciocínio geral ou da aceitação ampla do plano como competência independente já alcançada.

Sequência rastreável: auditoria da inferência (`8978ecd`); catálogo de tarefas (`e0ab6a4`); formalização A8 (`9029092`); distinção de conhecimento fornecido/adquirido (`5a3974b`); [protocolo](protocolos/EC-06_protocolo_raciocinio_evidencias.md), implementação e [resultados](EC-06_resultados_raciocinio_evidencias.md) no commit `9239134`. Os registros históricos relatam protocolo identificado antes da implementação e teste congelado antes da primeira avaliação final. Os hashes atuais conferem com o relatório: protocolo `1888d90392351ee10d189671918f22287f07a83b7f608620945b1124f9dc3e68`; teste `9b260c48d873cf62fdf0c47218844ded4c50c3702c20a33618d44329258cbdc1`. Nenhum manifesto independente anterior ao final foi arquivado; hashes atuais e reprodução não comprovam pré-registro independente nem cronologia por si sós.

## Protocolo, resultados e controles

O arquivo [ec06_evidence.rs](../tests/ec06_evidence.rs) usa memórias novas, fatos fornecidos, `TwoHopTask` e `Monitor`; o oráculo de classe é consultado após as decisões. O gerador é repetido e comparado, e os registros da memória antes e depois das consultas coincidem. O escopo selecionado compreende recuperação direta R, composição de dois passos C, insuficiência I e oposição direta X; o catálogo restante não foi avaliado. O controle simbólico recebe os mesmos fatos, contexto e consulta e implementa enumeração separada. A regra e os pressupostos são compartilhados, assim como autoria, gerador e avaliador: independência de implementação não é independência científica integral.

| Perfil | Consultas por condição nas duas repetições | Monitor e simbólico | Literal | Ablação |
|---|---:|---|---|---|
| R | 12 | 12 respostas diretas | 12 diretas | 12 diretas |
| C | 12 | 12 conclusões com duas referências | 12 ausências | 12 Unsupported |
| I | 12 | 12 abstenções por falta de evidência | 12 ausências | 12 Unsupported |
| X | 12 | 12 abstenções por oposição | 12 ausências positivas | 12 Unsupported, sem Monitor |

F: 24 cenários × quatro condições × duas repetições = **192 consultas**; **144 eventos** incluem 108 no candidato e 36 na ablação. Monitor e simbólico concordaram em **48/48 decisões**; **24/24 respostas sustentadas** compreendem 12 referências diretas e 12 provas de dois passos. As **24/24 abstenções** compreendem 12 por insuficiência e 12 por conflito; não houve respostas indevidas ou falsas abstenções no painel. A referência negativa e a prova positiva de X são verificadas no candidato. Registros, decisões e eventos coincidiram entre as duas repetições.

A ablação mostra dependência local da propagação na implementação de C; não é controle equivalente de conflito, pois não usa Monitor. O controle simbólico resolve os mesmos casos com a mesma informação: eventos neuronais e ablação não demonstram vantagem causal neural específica nem necessidade neural geral. O controle literal não compõe nem diagnostica oposição; menor cobertura em C é esperada pela sua capacidade restrita.

## Conformidade e limitações

| Requisito | Parecer | Evidência e alcance |
|---|---|---|
| Contagens, quatro perfis e previsões | Conforme funcionalmente | Assertivas e registros reproduzidos; não extrapolar a outros perfis. |
| Provas e abstenções | Conforme no painel | IDs, contexto, polaridade e conectividade são verificados; origem é declarada, não autenticada; a regra não é identificada/versionada pelo núcleo. |
| Separação de avaliação e não mutação | Conforme no código examinado | Classe esperada não é passada aos decisores; memórias comparadas antes/depois. Não há autoria cega ou auditoria independente do gerador. |
| Novidade e partições | Parcial | IDs e consultas variam; D/V/F reutilizam as mesmas quatro topologias e regra. Não há teste de nova estrutura ou domínio. |
| Controles e orçamento | Parcial | Simbólico independente em implementação e equivalente quanto aos fatos, com pressupostos comuns; custo de buscas não é uniformizado. Literal e ablação têm capacidades distintas. |
| Recursos | Parcial | Eventos exatos, tempo e RSS observados; estimativa de dados não inclui integralmente alocações transitórias e clones. Teto fino de 256 KiB não comprovado integralmente. |
| Histórico e generalização | Não demonstrados | Sem persistência após reinício, aprendizado de regra, transferência estrutural/interdomínios ou descoberta autônoma. |

A inspeção não encontrou acesso direto ao gabarito nas decisões; categorias previsíveis, mesma autoria e estrutura fixa deixam risco metodológico e não provam ausência universal de vazamento indireto. O verificador de prova é adequado aos casos produzidos; não avalia exaustivamente provas malformadas, múltiplos caminhos, negativas em premissas ou conflitos em vias alternativas. Contexto é identificador fornecido, sem versão ou validade temporal testada. Não se demonstram aprendizagem neural, formação autônoma de engramas, compreensão geral, AGI ou consciência.

## Reprodução e recursos

`cargo fmt --check`, `cargo check --offline`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check` passaram. Suíte: **143 aprovados, zero falhos e zero ignorados**. Desenvolvimento, validação e final da EC-06 passaram separadamente; as execuções adicionais de regressão não são somadas às 192 consultas do painel. `check --all-targets` apresentou três avisos preexistentes de código não utilizado na EC-04.

O painel final isolado medido via `wait4` durou 0,003223 s de parede e teve RSS máximo de 7.492 KiB; o valor pode incluir a base do lançador antes de `exec`. O teste verifica internamente limites de 30 s, RSS 256 MiB e espaço livre de 3 GiB; amostragens de disco não são monitoração contínua. Espaço observado após regressão: 4.648.304.640 bytes; `target` permaneceu em 61.215.773 bytes entre medições. A ferramenta `/usr/bin/time` não estava disponível; a medição substituta acima foi efetivamente executada.

## Diretrizes e pendências

A1/A2 orientam memória evolutiva e proveniência: os registros e referências são passos delimitados, sem memória integrada ou reorganização neural demonstradas. A3/A4 (pedagogia e comunicação) e A5/A6 (idiomas) não foram avaliadas. A7 exige rigor e preserva as ressalvas aqui documentadas. A8 orienta competência generalizável futura; a composição formal com fatos e regra fornecidos não demonstra essa competência. A1–A8 permanecem integralmente preservadas.

Para investigações futuras, permanecem úteis controles com autoria e avaliador mais independentes, topologias e regras realmente inéditas, testes de prova e conflitos em premissas, e medição comparável de trabalho e dados. São pendências, não novos protocolos autorizados. A próxima etapa nominal é **EC-07 — Questionamento e investigação orientada**, não iniciada.
