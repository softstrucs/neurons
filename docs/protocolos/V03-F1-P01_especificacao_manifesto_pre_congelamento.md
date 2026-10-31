# V03-F1-P01 — Especificação de manifesto pré-congelamento

Estado **ESPECIFICAÇÃO NÃO CONGELADA**; não é manifesto materializado, aprovação nem autorização de F. O [protocolo histórico](V03-F1-P01_confiabilidade_experimental_inicial_PROPOSTA.md), [E01 aprovada](V03-F1-P01_E01_identidade_S2_S3.md) e [D01–D04](../V03-F1-P01_decisoes_metodologicas_pre_finais_APROVADAS.md) prevalecem na respectiva ordem histórica; nenhuma hipótese, quota ou critério é alterado aqui.

## Geração e entradas

O algoritmo é determinístico e sem semente: para base simbólica `b` da partição conforme seção 9 do protocolo, `x=b+101`, `y=b+102`, contextos `A=b+1`, `B=b+2`; para `n=1..6`, registro positivo `b+10n`, negativo `b+10n+1` quando aplicável, evidência `b+500+10n`. Registros têm origem `Some(x)` e tempo 7; negativos sem evidência. Não materializar a base reservada final. D e V usam suas bases históricas distintas, com mesmas topologias; diferenças de IDs não são independência estrutural.

| Classe | Entrada e consulta | Expectativa prospectiva |
|---|---|---|
| S1 | Positivo validado com evidência em A; consulta A | Resposta direta com ID e contexto. |
| S2 | Positivo validado em A, negativo observado em B; consulta A | Resposta direta em A. |
| S3 | Estrutura de S2, IDs próprios; consulta B | Abstenção por ausência de suporte; equivalência estrutural E01, não identidade de bytes. |
| S4 | Positivo validado e negativo observado em A; consulta A | Abstenção por contradição, ID negativo na trilha. |
| S5 | Positivo observado sem `EvidenceId` em A; consulta A | Resposta direta; referência ausente marcada incompleta pelo avaliador. |
| S6 | Memória vazia; consulta A | Abstenção por ausência de suporte. |

Por caso: estado novo, `Engine(8,8)`, memória capacidade 8, `TwoHopTask(8,8,8)` e monitor de uma tentativa, sem alternativas exceto negativo de API; duas consultas no pai, uma no filho restaurado com configuração fornecida. Três repetições: 18 casos, 54 consultas válidas, 18 comparações no pai e 18 entre processos, 36 no total. Três chamadas inválidas adicionais com contexto repetido, sem mutação de checkpoint; total 57 verificações de bytes. Filho deve identificar-se e devolver relatório e bytes verificáveis.

Em cada repetição, adulterar separadamente cópia da trilha S1: contexto, ID e remoção de referência. Seis trocas rejeitadas e três remoções classificadas incompletas; essas trilhas não são decisões do núcleo. Critérios prospectivos: 54/54 consultas coerentes com referência válida, 36/36 comparações integrais iguais e 57/57 checkpoints sem mutação; divergência com referência válida é negativa local. Ausência de referência para S5 é registrada sem atribuir erro à resposta.

## Interrupção, invalidação e recursos

Interromper por filho não verificado, referência/verificador inválido, variante fora do contrato sem tratamento, serialização inválida, mudança pós-congelamento, vazamento, insuficiência de recurso indispensável, limite atingido ou `/home` com menos de 3 GiB livres. Não excluir resultados desfavoráveis; apenas falhas técnicas previamente previstas, registradas como inconclusivas, sem substituição. Timeout é evento operacional, não falha científica. A referência de dados vivos e RSS é tratada em D04; sua fiscalização permanece dependente do [relatório operacional](../V03-F1-P01_fechamento_operacional_pre_congelamento.md).

**Atualização prospectiva:** [D04-E02](V03-F1-P01_D04-E02_dados_vivos_APROVADA.md) mantém 256 KiB de dados vivos como restrição não verificada e não indispensável à alegação local. [D04-E03](V03-F1-P01_D04-E03_memoria_cgroup_APROVADA.md) substitui, somente neste protocolo, o critério operacional RSS por **256 MiB de memória cobrada ao cgroup v2**, sujeito a contenção efetiva validada. RSS é complementar; o ambiente anterior não foi validado. Permanecem tempo inferior a 30 s e pelo menos 3 GiB livres em `/home` imediatamente antes da execução. Esta atualização não altera o manifesto científico nem autoriza congelamento ou F.

Antes de qualquer painel autorizado, registrar especificação e manifesto algorítmico, versão e hashes dos instrumentos, referência, esquema, configuração, compilador, critérios, ambiente, responsáveis e momento de congelamento; conferir separação das partições e não consultar resultado final para ajustar nada. Hashes da base atual estão no relatório operacional; eles não constituem congelamento. Gerador atual só aceita D/V; habilitar F exige autorização posterior específica e validação sem executar F até a autorização correspondente. Nenhum resultado ou fixture F foi gerado aqui.
