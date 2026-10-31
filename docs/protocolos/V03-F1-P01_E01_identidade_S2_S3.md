# V03-F1-P01 — Emenda proposta E01: identidade de S2/S3

> **Atualização posterior:** por autorização humana expressa na formalização de D01, E01 está **APROVADA METODOLOGICAMENTE PARA V03-F1-P01**. O texto abaixo preserva a redação e o estado da proposta histórica; suas referências à decisão pendente descrevem o momento original. A aprovação não autoriza F nem muda dados, IDs, quotas ou expectativas. Ver [decisões pré-finais](../V03-F1-P01_decisoes_metodologicas_pre_finais_APROVADAS.md).

Base `f0dcb9e94fcaf91ec2a06db8e5b9772edf695ae0`. **PROPOSTA, NÃO APROVADA, F NÃO AUTORIZADO.** Preserva-se integralmente o [protocolo original](V03-F1-P01_confiabilidade_experimental_inicial_PROPOSTA.md) e seus critérios; esta emenda não altera dados, IDs, quotas ou expectativas.

A expressão “mesmo estado de S2” em S3 deve ser lida como **mesma estrutura e distribuição de sinais/contextos, após normalização bijetiva dos IDs específicos de cada caso**, não identidade literal de todos os bytes. O algoritmo já fixa IDs por número `n`: S2 e S3 têm IDs e evidências diferentes, embora representem a mesma relação positiva em A e negação em B. Cada estado é construído independentemente. A normalização compara campos, sinais, contexto e metadados com mapa de IDs, não altera as entradas do núcleo.

Esta equivalência é suficiente para testar, em cada caso separado, se a consulta não toma a negação em B como conflito da resposta em A e se B sem fato positivo abstém-se. Não prova comportamento de duas consultas sobre o mesmo objeto ou checkpoint; uma comparação pareada no estado literalmente idêntico seria controle adicional útil, mas não indispensável a essa alegação restrita. Nenhuma equivalência estrutural prova independência estatística ou estrutural entre D/V/F.

A interpretação requer decisão humana ao apreciar o protocolo final. Se se desejar alegar conservação de identidade literal ou efeito da mudança de consulta sobre a mesma instância, será necessário controle específico prospectivo e autorização apropriada; não se presume essa conclusão. A emenda não aprova painel ou mudança de critério.
