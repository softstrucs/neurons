# EC-04-R1 — Contrato do gerador determinístico

Revisão prospectiva autorizada para complementar `EC-04_protocolo_controles_memorizacao.md`, preservado integralmente. Aplicar o original integralmente, com os esclarecimentos abaixo. Nenhuma tarefa, alvo, quota, partição, controle, critério, previsão ou limite do original é alterado. Este documento não contém exemplos finais materializados. A aprovação da revisão e execução isolada decorre exclusivamente da delegação EC-04-R1 + retomada da implementação.

## Domínios e precedência

Em cada construção, `U={0,...,255}`. Ordenar cada domínio por `rank(x)=(197*x+101) mod 256` crescente, com desempate por x. Um valor atribuído não pode aparecer em outro slot da mesma construção. P1 e P2 são, respectivamente, os dois literais na ordem textual do alvo inicial; P3 é o literal acrescentado. Usar os predicados `P1`, `P2`, `P3`, com valor booleano de satisfação do literal, não o valor bruto do bit.

Classes de domínio: `P=P1∧P2`; `N1=¬P1∧P2`; `N2=P1∧¬P2`; `N3=¬P1∧¬P2`; `T=P∧P3`; `F=P∧¬P3`; `O=¬P`; `W1=¬P1∧P2∧P3`; `W2=P1∧¬P2∧P3`. Para B, P3 não existe e `P` é o único domínio positivo. As restrições de domínio são estáticas; disjunção e quotas são verificadas contra a atribuição parcial.

Slots abaixo têm ordem total indicada. Em cada grupo de slots simples com o mesmo domínio, valores devem ter rank estritamente crescente. O grupo `O-livre` da revisão exclui as duas testemunhas; seus quatro valores são exatamente os quatro menores elegíveis não usados no instante desse grupo. O complemento de um par mantém os bits relevantes e inverte todos os demais. Primeiro elemento de cada par deve ter rank menor que o complemento; os primeiros elementos dos quatro pares iniciais devem ter rank crescente. A segunda posição é obrigatória, sem ramificação, uma vez atribuída a primeira.

## Construção do contexto A — 120 slots

Ordem, com posições começando em 1 e par explícito como duas posições consecutivas:

1. Aquisição inicial (32): quatro pares positivos P, posições 1–8; N1×8, N2×8, N3×8, posições 9–32. Os pares completam exatamente quatro T e quatro F.
2. Validação inicial (16): T×2, F×2, N1×4, N2×4, N3×4.
3. Final inicial (32): T×4, F×4, N1×8, N2×8, N3×8.
4. Aquisição de revisão (16): par T, posições 81–82, complementar nas cinco propriedades fora de P1/P2/P3; F×8; W1×1; W2×1; O-livre×4, posições 93–96. As testemunhas W1/W2 são distintas dos demais valores e atendem ao domínio O.
5. Validação de revisão (8): T×1, F×4, O×3.
6. Final de revisão (16): T×2, F×8, O×6.

Todos os 120 vetores são distintos. Os quatro pares da aquisição inicial são complementares nas seis propriedades fora de P1/P2, o que implica uma posição T e uma F por par. Para o par de revisão, as duas posições satisfazem T e são complementares nas outras cinco propriedades. A regra original dos quatro menores O-livre aplica-se depois das duas testemunhas; não importa se outros elementos de O também poderiam ser testemunhas.

## Construção do contexto B — 80 slots

1. Aquisição inicial (32): quatro pares positivos P, posições 1–8, complementares nas seis propriedades fora de P1/P2; N1×8, N2×8, N3×8.
2. Validação inicial (16): P×4, N1×4, N2×4, N3×4.
3. Final inicial (32): P×8, N1×8, N2×8, N3×8.

Todos os 80 vetores são distintos. Não há slots de revisão em B. As sondas de retenção são as quatro primeiras posições positivas e as 12 primeiras negativas do final B quando ordenadas por rank, conforme original; são consultas sobre dados já reservados, não slots adicionais.

## Busca e orçamento

Executar busca em profundidade na precedência de slots acima. Cada slot ramificado percorre seu domínio estático em ordem de rank; antes de atribuir um candidato, inclusive o segundo membro forçado, contabilizar uma expansão. Uma tentativa que falha por disjunção, ordem, quota ou restrição conta. Verificações que não atribuem valor não contam expansão. Após uma tentativa inválida, continuar com o próximo candidato do mesmo slot. Se o segundo de um par violar qualquer restrição, retroceder ao primeiro e explorar seu próximo candidato; nunca escolher outro complemento.

Aplicar somente podas que comprovem, pela cardinalidade dos domínios ainda livres e pelas quotas remanescentes, impossibilidade de completar a atribuição parcial. Uma verificação de poda não atribui slot e não conta expansão; candidatos descartados antes de serem tentados não são expansões. Ao esgotar o domínio de um slot, desfazer suas atribuições e retroceder ao último slot ramificado. Em conclusão completa, devolver a primeira atribuição válida da ordem lexicográfica definida, sem embaralhar ou selecionar por desempenho.

Um contador global por construção/contexto começa em zero; não reiniciar após retrocesso ou poda. Até 2.000.000 tentativas podem ser realizadas; antes da tentativa 2.000.001, interromper com Inconclusive(DataConstructionBudget), sem aceitar atribuição parcial. Esgotamento completo da árvore abaixo do teto retorna Inconclusive(DataConstructionUnsatisfiable). Falha de quota, duplicação, par ou identificabilidade no preflight retorna Inconclusive(InvalidManifest). Nenhum desses resultados constitui falha da hipótese cognitiva. Sem modificação automática dos alvos, quotas, prioridade ou orçamento.

T-R/T-C seguem diretamente fórmulas e ordens do original; não usam essa busca e não têm slots conceituais. Materialização e consulta dos exemplos finais permanecem proibidas durante esta revisão documental. A execução autorizada posterior deve congelar e verificar os hashes dos manifestos e mecanismos antes da primeira avaliação final, respeitar as 14.560 consultas finais e 3.120 validações associadas e registrar os custos reais.
