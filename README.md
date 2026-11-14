# NEURONS

Núcleo experimental em Rust para neurônios, sinapses, engramas e evolução cognitiva verificável.

**Autoridade científica:** [NEURONS_PLANO_MESTRE.md](NEURONS_PLANO_MESTRE.md).
Execução funcional PIFI-001 autorizada pelo responsável, com [registro operacional](NEURONS_PIFI_001_PROGRESSO.md).
Essa autorização não libera protocolos novos, dados reservados ou publicação Git.

- [Documentação e referências](docs/README.md).
- [Planos executivos históricos, preservados](docs/historico/planos_executivos/README.md).
- [Relatório de recuperação e limites](docs/referencia/RECUPERACAO_ESTRUTURAL.md).

Pacote local sem dependências externas de crates: `cargo check --offline --lib --bin neurons`.
Testes/painéis históricos e dados reservados exigem escopo próprio; não executar `cargo test`
indiscriminadamente: há fixtures reservadas em alvos científicos. A CLI oferece `--help`,
`doctor`, `session`, `session-explain` e **`session-f1`**.

A implementação técnica F1 integra os coordenadores existentes de consulta, aquisição explícita
e revisão conceitual H23. Inicie `cargo run --offline --locked --bin neurons -- session-f1` e
use `ajuda`. Gramática estruturada, fontes sintéticas declaradas e limites finitos; sem linguagem
natural, rede ou compreensão geral demonstrada. Os comandos anteriores mantêm seu comportamento.

`salvar CAMINHO` cria exclusivamente um checkpoint F1 v1; nunca substitui arquivo existente.
`restaurar CAMINHO` restaura fatos, proveniência, pendências, hipóteses, versões e histórico.
Circuitos, pesos e potenciais neuronais não fazem parte desse contrato. Detalhes, comandos de
regressão seguros, limitações e evidências estão no [Plano Mestre](NEURONS_PLANO_MESTRE.md#9-registro-de-progresso-e-próxima-ação).
F1/F2 e F3-01 possuem aceites delimitados com ressalvas registrados no Plano Mestre.
O ciclo experiencial adicional integra diagnóstico, fonte didática local, revisão H23 e
reconsulta verificada; não demonstra plasticidade neural causal ou compreensão geral.

Regressões públicas selecionadas: `python3 scripts/test_public.py --build`.
O executor usa modo offline, dependências bloqueadas e target externo em `/tmp`,
seleciona alvos explícitos e exclui os dois probes reservados. `--list` mostra a seleção.
Os cinco arquivos locais `src/{integration/{query,acquisition,concepts},protocol_verification{,/conceptual}}/tests.rs`
não rastreados pertencem à fronteira científica histórica; não são requisitos para
compilar lib/CLI nem autorização para executar testes internos. Não removê-los.

O módulo `integration::development::Runtime` adiciona inspeção do circuito real,
sensores sintéticos tipados, atenção limitada, experiências rastreáveis e portas
para MP05, MP09, MP07/MP08 e verificação F3-01. Uso e controles executáveis:
`python3 scripts/test_public.py --target development_runtime`.
Sensores escalares não são interpretados automaticamente como relações.

Os checkpoints operacionais `NRNPIFI1/2` encapsulam os codecs F1 v1/MP05 v1 intactos,
observações, memória e consumo de recursos. `Runtime::restore` exige reinstalação
explícita das interfaces e novas permissões. Episódios consumidos não são reabertos;
relatórios transitórios não são restaurados como autoridade. O checksum detecta
corrupção acidental, sem autenticar origem. Esse recorte não encerra E00–E12 nem
constitui validação científica de vantagem neural, causalidade ou generalização.
Os campos históricos de configuração/concessão dos codecs internos são preservados;
eles não substituem as novas permissões do runtime.

`Runtime::intervene` registra observações antes/depois de uma ação sintética,
com concessão vinculada a mundo, versão, contexto, fonte e ação. `Runtime::plan`
usa apenas transições observadas; `Runtime::pursue` liga lacunas, perguntas,
controles disponíveis, exploração autorizada e reconsulta do objetivo original.
A tabela de consequências do ambiente não é entrada do planejador.
`Runtime::predict` apresenta resultados observados, referências e conflitos,
sem votação por maioria ou promoção a lei causal.

São no máximo quatro tentativas de ação, oito experiências perceptivas MP05,
quatro consultas de modelo e 32 inspeções de planejamento por instância.
A seleção de controles compartilha o orçamento atencional existente.
Falhas conservam concessões consumidas/reservas; previsões divergentes ficam
no histórico e suspendem o plano antigo antes de qualquer revisão explícita.
O envelope v2 retém esse modelo e seus limites; imagens v1 continuam canônicas,
sem migração silenciosa. Planos ativos, concessões e o ambiente externo não são
restaurados automaticamente. Testes: `python3 scripts/test_public.py --target development_actions`.

`Runtime::observe_attributes` vincula seis atributos explícitos ao estado atual;
`teach_consequences` deriva exemplos somente de intervenções já observadas e
reutiliza diagnóstico, seleção informativa e revisão MP09/H23. O conceito possui
um significado operacional declarado (ação/destino/contexto), sem regra-alvo
fornecida ao aprendiz. `forecast_consequence` reúne H23 e o modelo de transições;
`execute_forecast` verifica o vínculo/versão, aplica uma concessão fresca e confere
a consequência. Conflitos conhecidos suspendem a aplicação; abstenção e divergência
produzem pedido de revisão, sem apagar evidências ou admitir leis/fatos derivados.

O envelope operacional `NRNPIFI3` acrescenta somente esse vínculo semântico a uma
base v1/v2 intacta. Não restaura previsão ativa, permissões ou direito de reexecução.
Relatórios de comparação são transitórios; observações, transições e versões H23
são retidas. Uma nova instância explicitamente autorizada pode herdar o contrato
e continuar a memória conceitual; isso não renova recursos da instância anterior.
Os atributos ainda vêm de sensores sintéticos declarados. Não se demonstra descoberta
aberta de atributos, causalidade neural ou conclusão integral de E04–E06.
Testes: `python3 scripts/test_public.py --target development_consequences`.

`Runtime::pursue_with_consequences` consulta outro Runtime como memória imutável
de um episódio anterior e reutiliza o mesmo planejador para comparar uma rota
observada com um prefixo H23 seguido de caminho observado. Escolhe entre as duas
alternativas elegíveis por permissão, reserva e comprimento; empate favorece a
rota observada. Não enumera todos os planos possíveis. A consequência real pode
motivar uma reconsulta e um novo plano para o objetivo original.

`pursue_learning_goal` permite, opcionalmente, uma experiência após diagnóstico
real de insuficiência. Exige `PredictiveRevision` com permissões pedagógicas
independentes; rótulos são derivados da ação observada. MP09 seleciona/incorpora
experiências e revisa H23; a consulta anterior fica histórica. Sem autorização,
não há ensino implícito. Relatórios distinguem aquisição, revisão e objetivo atingido.
Instâncias novas exigem sensores, circuito e autoridade fornecidos explicitamente;
a memória de origem não muda e seus recursos/concessões não são reabertos. Manter
os checkpoints de origem necessários; não há consolidação automática de arquivos
de múltiplos episódios ou retenção permanente dos relatórios transitórios.
Testes: `python3 scripts/test_public.py --target development_predictive`.
