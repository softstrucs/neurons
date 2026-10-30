# EC-10 — Resultado R1: partições e renomeação

## Parecer e limitação substantiva

**AJUSTAR; correção parcial, revisão e transferência genuína não avaliáveis neste painel.** O [protocolo R1](protocolos/EC-10_particoes_renomeacao_R1.md) e teste foram congelados antes do primeiro F. A não conformidade anterior de reutilização literal de vetores entre D/V/F foi corrigida para este ensaio, mas uma verificação pós-F revelou que **0/6 episódios de revisão e 0/8 consultas reservadas têm rótulos diferentes entre os critérios antigo e revisado**. Não há testemunha que exija mudar de critério. O aumento de desempenho após os seis episódios é compatível com prática adicional consistente e **não demonstra revisão, recuperação após revisão ou interferência**. Não alterar dados após o resultado; novo protocolo e conjunto com testemunhas independentes seriam necessários.

O histórico [diagnóstico anterior](EC-10_resultados_retencao_interferencia.md) permanece inalterado. Este R1 usa suporte simbólico EC-04, não aprendizagem do núcleo. Estruturas gerais e família de conjunções são fornecidas; F tem critérios de aridade dois/três versus um/dois em D/V, mas aridades semelhantes já ocorreram historicamente na EC-04: novidade estrutural global não demonstrada.

## Partições e referência

Gerador histórico dos dois critérios do diagnóstico anterior cobre 122 vetores distintos de u8; seus complementares são 134. D e V usam 32 vetores cada, F os 70 restantes, dos quais os primeiros 32 são utilizados. Em cada partição, 12 treino A, oito reserva A, seis treino B e seis episódios adicionais A são disjuntos. O teste verifica disjunção entre partições e exclusão dos conjuntos completos do diagnóstico imediatamente anterior. Isso não prova exclusão de todos os exemplos EC-04 de toda a história. As reservas são reutilizadas em fases pré/pós, nunca adquiridas. A função de referência usa regras declaradas, independente do código de indução, mas mesma autoria/pressupostos; não é referência externa independente de especialista.

D e V executaram antes de F; protocolo SHA-256 `7ae20739a9602c5611cc262e27948d30fcb49e0d029bf5e0e5ef15c349451b12` e teste `df358e583843fce2ce622f8be1578a634d49a611fe97092b7675ed67f789e1dc` foram registrados antes de F. O procedimento constrói as partições em memória também em D/V, mas só entrega à execução a partição correspondente; não comprova cegueira de autoria ou pré-registro independente. Sementes não se aplicam. Nenhum método ou dado foi alterado após observar F.

## Resultados por repetição

Cinco fases × oito consultas × sete condições × duas repetições = **560 consultas**. As repetições são idênticas, não amostras independentes. Valores abaixo são por repetição; formato acertos/erros/abstenções.

| Fase e referência | Candidato, renomeado e indutor | Literal | Sem aquisição | Congelado | Regra fixa p7 |
|---|---:|---:|---:|---:|---:|
| Antes de A, antiga | 0/0/8 | 0/0/8 | 0/0/8 | 0/0/8 | 2/6/0 |
| Após 12 episódios A, antiga | 6/0/2 | 0/0/8 | 0/0/8 | 0/0/8 | 2/6/0 |
| Após intervalo e seis episódios B, antiga | 6/0/2 | 0/0/8 | 0/0/8 | 0/0/8 | 2/6/0 |
| Após seis episódios adicionais A, antiga | 8/0/0 | 0/0/8 | 0/0/8 | 6/0/2 | 2/6/0 |
| Após episódios adicionais A, revisada | 8/0/0 | 0/0/8 | 0/0/8 | 6/0/2 | 2/6/0 |

Os oito rótulos de avaliação antiga e revisada são idênticos; os seis rótulos de aquisição adicional também não divergem entre critérios. Assim a melhoria de 6/8 para 8/8 frente ao congelado é um resultado em mais episódios da mesma condição, não evidência de adaptação a critério alterado. `FixedP7` é regra preexistente com informação e regra diferentes; seu desempenho não é comparação justa de transferência. Literal recebe episódios, mas não compõe/induz regra. Controle sem aquisição não incorpora treino. Congelado recebe os episódios posteriores no invólucro de controle, mas **não os aplica ao estado do candidato**; essa condição não é equivalente em atualização e informação efetivamente usada.

Após A: versão 12, início 0, sete hipóteses; após B permanece igual. Após episódios adicionais: versão 18, início 0, três hipóteses. Antes/depois de B, **6/6** respostas anteriormente corretas foram mantidas por repetição; duas abstenções permanecem. O intervalo real >=25 ms foi verificado, mas tempo exato não foi impresso. B é outro contexto interno, com estados separados e sem pesos compartilhados: isso demonstra coexistência no escopo, não resistência à interferência compartilhada. Não há reinício ou persistência em disco; retenção após reinício e longitudinal longa não avaliadas.

## Renomeação, controles e limites

Contextos 7→70, 8→80 e episódios id→id+1000 mantêm valores, tipos, ordem e rótulos; o teste verifica inversa para cada episódio e campos acessíveis. Candidato original e renomeado coincidiram em **80/80 consultas pareadas** (cinco fases × oito × duas repetições), comparando `Answer` completo e `Work` por consulta. O controle simbólico `Independent` recebeu os mesmos episódios e consultas e coincidiu em `Answer` nas mesmas 80 consultas; seu trabalho foi registrado em agregados, não assertado por consulta. Consultas não alteraram os estados comparados por clone. O mapeamento não renomeia propriedades semânticas dos bits, regras ou valores; a tarefa não contém grafo relacional. `Answer` não expõe IDs de evidência, portanto não se verificou bijeção de proveniência interna nem igualdade do estado privado entre instâncias; eventos neurais não se aplicam. Igualdade parcial de trabalho não comprova igualdade de tempo ou de todas as operações.

Transferência genuína é **não avaliável**: não há domínio/tarefa semanticamente nova com referência externa independente e contraste causal adequado. Candidato e controle de indução compartilham autoria e família; nenhum resultado comprova superioridade neural. A comparação de composição de relações/topologias não é executada porque o painel é de classificação vetorial; exige outro ensaio, não deve ser inferida deste.

## Verificações e recursos

D/V passaram antes de F. `cargo fmt --check`, `cargo check --offline --all-targets`, `cargo test --offline -- --test-threads=1` e `git diff --check` passaram: **170 testes aprovados**, zero falhos/ignorados. A contagem inclui três testes históricos duplicados por EC-08 e dois testes de preflight EC-04 incluídos nos alvos EC-10; não são observações independentes novas. Três avisos históricos EC-04 de métodos não usados.

Painel isolado 0,085766 s de parede e RSS `wait4` 7.392 KiB, incluindo possível base do lançador. Guardas internas de 30 s, 256 MiB e /home >=3 GiB passaram. /home observado inicial 4.623.659.008 e após painel 4.620.853.248 bytes; sem monitoração contínua. Teto fino de 256 KiB por condição não medido integralmente. Contadores `Work` do EC-04 têm limitações históricas; sem medição comparável de custo total. Agregados de F em `/tmp/neurons_ec10_partition_r1.log` não são arquivo de custódia independente.

A1/A2 orientam memória/proveniência, A3/A4 formação/investigação, A5/A6 idiomas, A7 rigor e A8 competência generalizável futura; preservadas sem capacidades presumidas. EC-10 permanece aberta; é necessário protocolo prospectivo com testemunhas de revisão, referências independentes e controles adequados para transferência/interferência antes de alegações correspondentes. Não iniciar etapa posterior.
