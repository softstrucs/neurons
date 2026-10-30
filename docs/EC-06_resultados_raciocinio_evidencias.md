# EC-06 — Resultados delimitados de raciocínio com evidências

## Escopo e parecer

Base `5a3974ba2b572cbf8a257e9a97e2314812770511`, devs inicialmente limpa e sincronizada. [Protocolo prospectivo](protocolos/EC-06_protocolo_raciocinio_evidencias.md) elaborado e identificado por SHA-256 antes da implementação. O teste isolado `tests/ec06_evidence.rs` usa `EngramStore`, `TwoHopTask` e `Monitor` existentes, sem alterar produção. Desenvolvimento e validação executados antes do primeiro final; protocolo e teste congelados por hash antes do final e intactos após a regressão.

**Resultado funcional: previsões observadas nos quatro perfis; conclusão científica AJUSTAR, com conformidade metodológica parcial.** Este é ensaio de recuperação/composição e detecção programada de oposição direta. Não há evidência de aquisição da regra, aprendizagem neural de raciocínio, descoberta autônoma, compreensão geral, AGI ou transferência estrutural. A EC-06 completa não é certificada por este painel.

Escolhidas tarefas EC06-R/C2/I e oposição direta de EC06-X; EC06-G cobre somente IDs/consultas diferentes com topologias conhecidas. Exclusões justificadas no protocolo: tipos gerais, isolamento de vias conflitantes, versões/validade de contexto e múltiplos passos não são contratos gerais do núcleo. Os resultados históricos das EC-03/04/05 não são modificados. Regra local de dois passos é fornecida pelo protocolo; não extrapolá-la a qualquer predicado ou domínio.

## Dados, método e controles

Memórias independentes de até três relatos sintéticos Observed por cenário, origem declarada como nó de partida, tempo0 e EvidenceId ausente. Nenhuma conclusão positiva x→z é cadastrada nos perfis de composição C ou conflito X. O avaliador gera consulta x→z; a proposição consultada é entrada legítima, mas rótulo de resultado correto não é fornecido aos decisores. O código gerador tem categorias previsíveis e seu autor conhece o protocolo: a inspeção confirma que candidato e controles não recebem o índice/classe; não há cegueira de autoria ou garantia estatística de ausência de correlação nos IDs.

A regra ρ2 e sua identidade estão no protocolo; o núcleo devolve referências dos dois fatos e intermediário, **não devolve ID/versão da regra**. Contextos são IDs fornecidos, não há versão ou validade temporal no ensaio. Origem declarada não é fonte autenticada. A fiscalização consulta registros e verifica IDs, contexto, polaridade, conectividade, intermediário, origem e ativação; no conflito verifica a referência da negativa e a prova positiva associada. A prova verifica o contrato formal, não verdade externa.

Condições: Monitor/TwoHopTask; controle simbólico independente com mesmos fatos/consulta e enumeração própria; recuperação literal positiva com mesmos registros; ablação `evaluate_without_synapses`. O literal não faz composição. A ablação não usa Monitor, portanto não é controle equivalente de detecção de conflito; efeito neural é interpretado nos perfis C/I. Informação é igual no candidato e simbólico, com pressupostos, tipos de fatos, protocolo, autor e avaliador comuns. Decisão e gabarito são separados no código; consultas não alteram registros.

D e V: oito cenários cada, dois de cada perfil; gerador e verificações repetidos antes do final. F:24 cenários, seis por perfil, duas sequências globais com memórias novas. Gerador Fisher–Yates e offsets declarados; fixtures repetidas deterministicamente, com IDs diferentes entre D/V/F. As mesmas quatro topologias ocorrem nas três partições: novidade é de consulta e IDs, não de estrutura, regra ou domínio. Não houve ajuste depois de observar F. Hashes atuais comprovam identidade dos arquivos, não pré-registro independente de autoria ou tempo.

## Resultados finais: duas repetições

| Perfil | Consultas por condição | Monitor | Simbólico | Literal positiva | Ablação |
|---|---:|---|---|---|---|
| R direta |12|12 Direct, referência válida|12 Direct, mesma referência|12 Direct|12 Direct, zero eventos|
| C dois passos |12|12 Inferred, prova válida|12 Inferred, mesma prova|12 ausências|12 Unsupported|
| I insuficiente |12|12 MissingEvidence|12 MissingEvidence|12 ausências|12 Unsupported|
| X oposição direta |12|12 ContradictoryEvidence, negativa identificada|12 Conflict|12 ausências positivas|12 Unsupported; sem Monitor|

Monitor e controle simbólico coincidiram nas **48/48 decisões** e nas referências dos **24/24 retornos sustentados**; todas as provas verificadas correspondiam aos fatos do contrato. As **24/24 abstenções** do Monitor nos perfis I/X foram adequadas; zero respostas indevidas ou falsas abstenções frente ao painel. Literal e ablação registraram12 diretas e36 ausências/Unsupported cada, segundo seus contratos próprios. As12 ausências do literal em C são perda de cobertura de composição, não prova de superioridade neural; em X o literal não diagnostica a oposição.

Contagem:24 cenários ×4 condições ×2 repetições = **192 consultas**. Duas sequências produziram registros iguais de caso, contexto, fatos, decisões, referências, conflito e eventos; o gerador também foi chamado duas vezes e comparado. Os registros detalhados são emitidos pelo teste e guardados em `/tmp/ec06_experiment_final.log` nesta execução; esse log não é artefato versionado nem prova independente de cronologia. Aquisição D/V/F foi sintética; não há atualização por avaliação nem teste de persistência após reinício. A igualdade dos registros da memória antes/depois das consultas demonstra não mutação nesse escopo, não consolidação de conhecimento.

Eventos: candidato **108** (48 em C,24 em I,36 em X, zero R); ablação **36** (um por cenário não direto); literal/simbólico zero. Total observado **144**, previsão144. Remover sinapses remove a propagação no mecanismo testado, mas o controle simbólico equivalente resolve os mesmos problemas; não se demonstra necessidade ou superioridade neural geral.

## Verificações e recursos

Desenvolvimento e validação passaram; nenhum resultado final foi utilizado para alterar os arquivos congelados. `cargo fmt --check`, `cargo check --offline`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check` passaram. **143 testes aprovados, zero falhos/ignorados**, incluindo os140 anteriores e três novos; a regressão executa novamente o painel existente e suas consultas não são somadas ao painel isolado.

Painel isolado:0,002500 s internos,0,013697 s de parede; wait4 reportou7708 KiB, amostragem /proc a cada1ms 1272 KiB. Wait4 inclui processo lançado/fork-exec e possível base do lançador; amostragem pode perder picos. Monitor externo de30s/RSS256MiB/disco3GiB não acionou; mínimo amostrado em /home4663906304 bytes. Após regressão,4658925568 bytes livres. O maior RSS acumulado entre filhos dos comandos de verificação foi110060 KiB; não é pico exclusivo do painel nem soma simultânea dos processos. A compilação inicial não teve medição isolada.

O teste impõe estimativa <=256 KiB com capacidades dos vetores e reserva de32 KiB; não mede integralmente alocações transitórias de Circuit/Monitor, clones de fatos nas linhas e alocador. **Teto fino de dados: não comprovado integralmente**, sem excesso observado. Eventos têm contagem exata no painel, mas visitas, operações de busca e CPU dos controles não possuem unidade uniforme; equivalência de orçamento efetivo e eficiência relativa não foram demonstradas. RSS/tempo não substituem essas medições. `target/` ao fim da regressão:61215773 bytes; crescimento nesta retomada não foi medido a partir de uma linha de base inicial e permanece desconhecido.

## Limitações e custódia

Cenários pequenos, quatro topologias, regras e oposição fornecidas externamente; sem negativos sobre premissa, conflitos entre vias alternativas, critérios de tipos heterogêneos, validade temporal, múltiplos passos ou orçamento insuficiente no painel. Esses temas não foram inferidos dos resultados; permanecem para eventual protocolo autorizado. Não se afirma revisão autônoma, retenção após reinício, aquisição de significado ou generalização entre domínios. Candidato usa circuito e busca simbólica de prova; o controle mostra que a regra formal também é resolvida simbolicamente. R e C têm resultados diferentes por desenho; não comparar literal e inferência como se fossem mecanismos de mesma capacidade.

Arquivos congelados antes do primeiro final e conferidos após regressão:

```text
1888d90392351ee10d189671918f22287f07a83b7f608620945b1124f9dc3e68  docs/protocolos/EC-06_protocolo_raciocinio_evidencias.md
9b260c48d873cf62fdf0c47218844ded4c50c3702c20a33618d44329258cbdc1  tests/ec06_evidence.rs
```

Nenhum manifesto separado com hash foi arquivado antes do final: o gerador e parâmetros estão no teste/protocolo congelados, e a repetição compara fixtures em memória. Essa limitação impede apresentar hash posterior como prova independente de pré-registro. Diretrizes A1–A8, planos e protocolos anteriores permanecem inalterados.

Próximo ato: revisão científica desta execução e decisão sobre o encerramento delimitado da EC-06. A próxima etapa nominal do plano é EC-07 — Questionamento e investigação orientada; não foi iniciada nem autorizada por este relatório.
