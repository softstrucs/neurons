# EC-04 — Implementação experimental isolada

## Estatuto e custódia

Base `7e6bf338ec987d538bfd2036cd2e640a57068ad8`, branch devs. Implementação isolada autorizada; protocolo original preservado. A revisão prospectiva EC-04-R1 completa somente o contrato do gerador. Esta entrega não é a auditoria científica posterior nem encerra a EC-04 inteira.

Foram preservados e integrados os arquivos locais preexistentes. Nenhum arquivo de produção, dependência, CLI, checkpoint ou plano foi alterado. Rust/Cargo 1.88.0, std, execução offline e serial dos testes. Não há threads próprias, serviço ou arquitetura cognitiva nova.

## Implementação e dados

T-R usa memória exata de pares e controle independente BTreeMap. T-C reutiliza EngramStore/TwoHopTask, com seis fatos, oito nós, max_facts=8, fila=32 e 128 eventos/consulta; os dois controles de composição percorrem adjacências e pares de fatos, respectivamente. Apenas T-C do candidato utiliza circuitos; sua regra é fornecida.

T-A/T-V usam modelos em arquivos separados: candidato mantém o espaço de versões e sufixo consistente; indução independente enumera hipóteses e sufixos por implementação própria, sem importar o candidato. DTOs compartilhados são passivos. O ramo sem atualização T-V copia deliberadamente o estado do candidato antes da revisão, como previsto. Contextos A/B conservam estados separados. Professor e funções de pontuação ficam no avaliador; Query recebe somente contexto/vetor. Episódios avaliativos não são oferecidos à aquisição. A flag acquisition recusa episódios identificados como avaliativos; não é autenticação contra um chamador malicioso.

Os 12 manifestos passaram em quotas, domínios, pares, disjunção, testemunhas, ordem, repetibilidade e identificabilidade (um critério inicial e um revisado, quando aplicável). Os vetores finais foram materializados somente pelo gerador do avaliador; não foram adquiridos como respostas. Manifestos são sequências binárias em memória: um byte por vetor, ordem dos slots; os limites das partições são congelados em R1. Seus hashes, abaixo, não publicam gabaritos. Gabaritos são calculados somente pelo professor/avaliador, nunca fornecidos na consulta.

| Contexto | Slots | Expansões |
|---|---:|---:|
| Desenvolvimento A/B | 120/80 | 1833/798 |
| Validação A/B | 120/80 | 1817/770 |
| Final 1 A/B | 120/80 | 1833/770 |
| Final 2 A/B | 120/80 | 1853/770 |
| Final 3 A/B | 120/80 | 1801/770 |
| Final 4 A/B | 120/80 | 1830/770 |

Cada tentativa de atribuição, inclusive rejeitada ou obrigatória, incrementa o contador global; sem reinicialização no retrocesso. Máximo observado 1853, abaixo de 2.000.000 por construção. Nenhuma quota foi relaxada.

Desenvolvimento: 2384 chamadas, sendo 168 T-R, 128 T-C e 2088 conceituais. Validação de implementação: 2320, sendo 168, 64 e 2088. As colunas internas final_queries dos laboratórios D/V significam sondas reservadas desses laboratórios, não avaliação final F. Código foi congelado após D/V e antes do primeiro painel F; os hashes foram novamente conferidos após a execução. Não houve ajuste após F. As reproduções posteriores da mesma sequência foram regressão e medição isolada, não novos dados ou ajuste. Grafos de validação T-C são reutilizados nas validações associadas, conforme protocolo.

## Contagens observadas

| Unidade | Previsto em duas repetições | Observado |
|---|---:|---:|
| T-R final, sete condições | 224 | 224 |
| T-C final, oito condições | 512 | 512 |
| T-A final, nove condições e A0/A1 | 9216 | 9216 |
| T-V final, nove condições e V0/V1 | 4608 | 4608 |
| Total final | 14560 | 14560 |
| Validações associadas | 3120 | 3120 |
| Total painel | 17680 | 17680 |
| Eventos de circuito T-C | 280 | 280 |

São chamadas, incluindo controles e repetições, não observações estatisticamente independentes nem testes Cargo. O painel compara integralmente os registros compactos das duas repetições, incluindo versões, sufixos e referências registradas. Query é imutável e sua execução também é comparada com clone anterior. Nenhuma inconclusão registrada.

## Resultados por repetição

T-R: candidato, literal e memória independente acertaram 8/8 valores presentes e reconheceram 8/8 ausências. Sem aquisição, sem atualização e sempre Absent reconheceram oito ausências e perderam oito valores presentes. Sempre Value(0): zero acertos, oito valores errados e oito respostas indevidas em ausências.

T-C: candidato, executor independente e regra programada obtiveram 16 provas e 16 Unsupported corretos (32/32). Literal, sem aquisição, sem atualização e sempre Unsupported obtiveram somente os 16 Unsupported e perderam as 16 provas. Sempre Supported sem prova: 0/32, sem converter coincidência booleana em prova. Referências às premissas são registradas. Memória direta não tem a capacidade de composição; não há alegação de superioridade informacionalmente equivalente contra ela.

| Condição | T-A A1 acertos/256 | Erros | Abstenções | T-V A V1 acertos/64 | Erros | Abstenções | B V1 acertos/64 |
|---|---:|---:|---:|---:|---:|---:|---:|
| Candidato | 256 | 0 | 0 | 64 | 0 | 0 | 64 |
| Indução independente | 256 | 0 | 0 | 64 | 0 | 0 | 64 |
| Literal | 0 | 0 | 256 | 0 | 0 | 64 | 0 |
| Sempre positivo | 64 | 192 | 0 | 8 | 56 | 0 | 16 |
| Sempre negativo | 192 | 64 | 0 | 56 | 8 | 0 | 48 |
| Sempre abster-se | 0 | 0 | 256 | 0 | 0 | 64 | 0 |
| Sem aquisição | 0 | 0 | 256 | 0 | 0 | 64 | 0 |
| Sem atualização | 0 | 0 | 256 | 32 | 32 | 0 | 64 |
| Regra fixa +p7 | 110 | 146 | 0 | 29 | 35 | 0 | 24 |

Candidato/indução: A0 tem 32 abstenções por contexto, 256 no conjunto de oito contextos. A1 tem 8/8 positivos e 24/24 negativos em cada contexto, cobertura 100%, acurácia balanceada 1, ganho 1 contra sem atualização. Literal não recupera vetores novos; suas abstenções não são acertos de classe.

T-V A: V0 tem 2/2 positivos e 6/14 negativos por bloco, oito erros por bloco, acurácia balanceada 5/7. V1 tem 2/2 e 14/14, cobertura 100%, ganho 2/7 frente ao ramo congelado. Foram corrigidos os oito erros por bloco, sem novos erros. B conserva 16/16 decisões e referências por bloco entre V0/V1. Isso decorre de separação de estados por contexto, não demonstra resistência à interferência em pesos compartilhados. Candidato e indução independente coincidiram nos registros comparáveis, sem superioridade do candidato.

Os critérios de desempenho foram verificados individualmente nos blocos, não somente na média. As tabelas detalhadas abaixo incluem classes, erros e abstenções; os resultados da regra fixa foram pontuados sem adaptar sua regra.

## Recursos e regressão

Primeiro painel F: 1,338974 s internos; 1,385866 s incluindo Cargo/lançador. Máximo RSS reportado para filhos do lançador: 25132 KiB (inclui Cargo, não atribuir ao processo experimental isolado). Reprodução do mesmo binário sem Cargo para instrumentação: 1,341145 s de parede, VmHWM amostrado 1688 KiB. Amostragem a cada 2 ms pode perder o último pico; não é medição exata terminal de RSS. Guardas internas leem VmRSS nas fronteiras de condições e blocos. Nenhum limite foi observado excedido; não há prova de monitoramento contínuo entre amostras.

Compilação/verificações/testes: máximo cumulativo de maior filho 120964 KiB, não soma de RAM simultânea nem rebuild limpo. cargo check --all-targets 0,181467 s; suíte 2,474238 s; cache de compilação reutilizado. Não se forçou compilação limpa.

Dados: 120/80 bytes por manifesto A/B, 1200 bytes de vetores nos 12 contextos; rótulos não são armazenados como fixtures finais. Os registros usam estrutura compacta abaixo de 64 bytes. As guardas somam capacidades dos buffers coexistentes com reservas de 64/96 KiB para estados e temporários; passaram sob 256 KiB. Ressalva: reservas conservadoras não equivalem a rastreamento exato de cada alocação/clone/BTreeMap; a conformidade fina de memória permanece parcialmente instrumentada. Work aplica guardas de 4.000.000/8192 unidades; a exaustividade da contagem de todas as visitas, comparações curtas e cópias não foi provada por instrumentação independente. Não declarar conformidade integral desses dois aspectos apenas por aprovação dos testes.

Target: 50.337.935 → 55.415.100 bytes; crescimento 5.077.165 bytes. Espaço disponível após verificações: 4.630.724.608 bytes (~4,31 GiB), acima do mínimo de 3 GiB; guardas antes/durante o painel. Sem novos pacotes ou dependências.

cargo fmt --check, cargo check --offline, cargo check --offline --all-targets, cargo test --offline -- --test-threads=1 e git diff --check passaram. Suíte: 132 aprovados, zero falhos/ignorados, preservados os 124 anteriores. Os oito testes adicionais incluem três integrações do mesmo preflight: não são oito experimentos independentes.

## Limitações e conclusão provisória

Conclusão **AJUSTAR**: implementação e painel concluídos com previsões funcionais reproduzidas; conformidade fina de instrumentação e validação de fronteiras merece a auditoria posterior prevista. Não houve mudança de código após o primeiro final para corrigir desempenho. Esta entrega não certifica genericamente os controles fora dos casos congelados: por exemplo, literal T-C resulta Unsupported nas consultas sem aresta direta, e T-R não inclui entradas conflitantes no painel final. A rejeição adversarial de todas as fronteiras não foi exaustivamente testada.

Duas implementações da indução são separadas, mas feitas na mesma sessão/equipe e sob a mesma especificação; concordância não prova independência científica completa ou equivalência geral. Famílias, atributos, regra de recência, professor e alvos são fornecidos. Proveniência registra intervalo de sufixo/versão, não todo o protocolo EC-03. Não há formação autônoma de conceitos, aprendizagem neural em T-A/T-V, compreensão geral, competência pedagógica ou transferência estrutural demonstradas. T-R é recuperação literal; T-C composição programada; T-A aplicação de critérios adquiridos; T-V revisão contextual delimitada. Ressalvas históricas A01/A02/EC-02/EC-03 preservadas.

Próxima subetapa prevista: avaliação de discriminação, composição e exceções, mediante autorização; não iniciada nesta entrega.

## Integridade congelada antes do primeiro painel final

SHA-256 dos arquivos de protocolo e código (os relatórios de resultados são posteriores):

```text
b089710e839da9e55182a67d8b6c767c5a4bb381d6b16ced34e47c2663aaa0e8  docs/protocolos/EC-04-R1_gerador_deterministico.md
9f76b306187d160a6d8a7086f556ce1a7b682a617f81f415ecec36e347393202  docs/protocolos/EC-04_protocolo_controles_memorizacao.md
f041200d535ba4fbce31837f21b216bcf6fa0f8c4a9112d6d1d90a82efbee1cc  tests/ec04_generator_preflight.rs
b09346b20364ca3978d9e67388533511851aaad5d69e7e3d41c220c5d5f06952  tests/ec04_learning.rs
d7209b12438c9cd0ab66502b038f0da60940f2ad1e63fd42d5962ae57e1e93cc  tests/ec04_panel.rs
e0eb4670cec37db9d332d3a293a6ba0e82230fe7fcc617dd95b861abc4c24c93  tests/support/ec04_candidate.rs
d2a79285ace19018efe95f40af9ec2f4a4dfe4cf3f53193566cb6de13cf277e2  tests/support/ec04_controls.rs
d801540354760c963a221cd883cb586373454104c57d53f2ed8e6ea84fa02d1d  tests/support/ec04_generator.rs
bcc51cc65e0395a83bba6e135d40bef98dda3fd40cf012a91b37e31284b5d6c8  tests/support/ec04_independent.rs
272eda1e518d56249c17bc8833b5e19415d28dfbfde741dc33b9d65042b53702  tests/support/ec04_types.rs
```

## Hashes dos manifestos canônicos por partição

Cada sequência abaixo é derivável deterministicamente do gerador. Não se arquivam respostas finais no relatório. Arquivamento deste relatório não prova pré-registro histórico anterior à delegação.

```text
MANIFEST dev-A acquisition bytes=32 sha256=d002c73079a85f751335674e9b2d2af137b2332c599bc6c319129c45afef7c28
MANIFEST dev-A validation bytes=16 sha256=1dbf0e38a6e7dc5363bd04a298eccb79a7230e10c1bf7c68094d8bb092902949
MANIFEST dev-A final bytes=32 sha256=34f7b69b63be6bc44a422a163732b4e65000fca1995a040e324b4a2dc9d860cd
MANIFEST dev-A revision-acquisition bytes=16 sha256=d0cacfdc1b948b139339f3599f403c822ad13f502df518ab1de85524b7d0750e
MANIFEST dev-A revision-validation bytes=8 sha256=aced04f0b2089d9483f46a7d85454b74630ee3f27a0bdac0ddc1843958e2b932
MANIFEST dev-A revision-final bytes=16 sha256=8709fc41f7228443ca76d078362c0c4da4f2d724aa13de1a1a4aab12c1ca4b4d
MANIFEST dev-B acquisition bytes=32 sha256=78afa725f16b132846bea79ee97426b80e6422d4d0302be075580887eb75a7c0
MANIFEST dev-B validation bytes=16 sha256=ec852d90cbbfe454219e6eebd7e9ec903763bf02ccd6e1c1c10cfdff1ec47cc9
MANIFEST dev-B final bytes=32 sha256=8750624380703d6b4509dcc68db35aeed6172e94cd207de5c336860d39b63223
MANIFEST val-A acquisition bytes=32 sha256=1154adc64ce377dfe4d176669357ed634b32418b90f5e91decf036835eb0195d
MANIFEST val-A validation bytes=16 sha256=fc187a2ff19f920e4dc8cdc83609568da9d39851d3ea4c7a4d0be683bbdc8bd3
MANIFEST val-A final bytes=32 sha256=e7f8656e590a12154b23b667c86ae272efdb96a86a21ec6010cf523abb8e2ff5
MANIFEST val-A revision-acquisition bytes=16 sha256=711e58555ca142474c90c99df4a06172dd4538e108192bb2160c985639ae0d9b
MANIFEST val-A revision-validation bytes=8 sha256=23e23ff680be21348024a271662f271b3f91a2e16d96cb7ad90b381fcb684b2a
MANIFEST val-A revision-final bytes=16 sha256=b20e4a160285d4adb67b6615dc0c8e859224b7f1c670ad9a223423584c0dd2db
MANIFEST val-B acquisition bytes=32 sha256=7a8f6ebcc5dc15c811d5312b76d18b152322eac3575800750f86dce14f11cdd9
MANIFEST val-B validation bytes=16 sha256=864ce928bfa86d67e27050a141e7f471451c211285657a277c1c0ff8457bda04
MANIFEST val-B final bytes=32 sha256=65524ff358c0818f259123784be400a8ffe27495402ffa50c3e2bb7742399cfe
MANIFEST final-1-A acquisition bytes=32 sha256=82c493b884a18c773be8da7e360aece42ef2b4a90aae0412de44487e1af9eb20
MANIFEST final-1-A validation bytes=16 sha256=92f130c9f71cbb72cbbeb659afb6fceca4a5868eff41794df979c5a1c03c252d
MANIFEST final-1-A final bytes=32 sha256=e7fe18c23c50f196df6c352350b86ac984adc3d153c0c7eb299ec5654bd7cd7d
MANIFEST final-1-A revision-acquisition bytes=16 sha256=92d5872edc25012f507ba0aaea4a8eb51b93a4fd555dfeb170a3b2b32f0cf684
MANIFEST final-1-A revision-validation bytes=8 sha256=5b97cd422e8abeb79a552c8637488887e6a88dce123f544cd85811fea8ce7b41
MANIFEST final-1-A revision-final bytes=16 sha256=a2b7b3926510ce36212b13b4bbd6b5b075b74dffc5794d5e7c2e25580ca353b5
MANIFEST final-1-B acquisition bytes=32 sha256=dd393c6e737aa2b2595a811d7036b820b47e290c53447ebbe30b87d6534cca6c
MANIFEST final-1-B validation bytes=16 sha256=dea8a90a37e0771587e57f63b87c50a4ad2e7fc78076ba779d37e76d8d4d5300
MANIFEST final-1-B final bytes=32 sha256=bc45c3e5c0b9d9001a39d3aafe07a33c53a5a0c338edf3f5707ec9ef128718ca
MANIFEST final-2-A acquisition bytes=32 sha256=1d1d3a78833ede6969a18a4452875a46e034a038f360bb3850956ba5d11ebbd4
MANIFEST final-2-A validation bytes=16 sha256=c4fd6cf6fe0eb9d153522403161889280abaa68f5b8bb5fc15a20c9a0fb5df95
MANIFEST final-2-A final bytes=32 sha256=3271abc9cf40dee6ee9465a9383b3ea0932c1120433d0499c659ff53c2b9820b
MANIFEST final-2-A revision-acquisition bytes=16 sha256=83b64b32f1b61d1f34f84981938acde1b686725c66f75cdb49c3bcfae6053117
MANIFEST final-2-A revision-validation bytes=8 sha256=c75f0fe1d849c372d4d0faaa6b276736e94568e5a605bd480534050360f35ee4
MANIFEST final-2-A revision-final bytes=16 sha256=26be84b94c3ca89158b59c829d23dac3ff675ad87fcdef3422fc2e54b4b2dd84
MANIFEST final-2-B acquisition bytes=32 sha256=e393282fbe1d77338db05d4d330a4d372ec3c7771d7b82ff443abcff8254a55a
MANIFEST final-2-B validation bytes=16 sha256=28a3943154d416821667e32b6d29e0f2ec395213c2981922b70ecee12b191291
MANIFEST final-2-B final bytes=32 sha256=4398899a6f90eb90b3afdf7d8ff419243fbea26f287ffce35e12469ca06d7f57
MANIFEST final-3-A acquisition bytes=32 sha256=36ec0ecb72e62a3353536de28b2957dbec727938dd9a882f3d995421d20bfca4
MANIFEST final-3-A validation bytes=16 sha256=fc085efb4577d396a6e67e2a3e63669d50beb80478f4eb97e0642e2165739552
MANIFEST final-3-A final bytes=32 sha256=ae58c008e85b99d88d966122566ef76a8c27dfdfc6488ee45b73ef1e559b5a57
MANIFEST final-3-A revision-acquisition bytes=16 sha256=7399e519c41b86b6f655d2ef08b8692e89f920fe8a3d4265910ed04e1110a06a
MANIFEST final-3-A revision-validation bytes=8 sha256=ab67f495a0aa33c5572195c6a1403ab83d0edc8fe30f77b2f5e38e8c28bca122
MANIFEST final-3-A revision-final bytes=16 sha256=f7749cb79c9d7673e75bc3c3f2d1e45eb3b3de9572fb323d69b348498d23791d
MANIFEST final-3-B acquisition bytes=32 sha256=cd5422716daf1a692073fdf79513a763420a74453a6568b6b8ef540e9e67fe90
MANIFEST final-3-B validation bytes=16 sha256=b2064a307cbf0231454de3c70f6972fcf377882c9e8765a5e7f0b9fe13acf951
MANIFEST final-3-B final bytes=32 sha256=fe537def6cf08bca0d5cca1a72a152ecc775181b4dd54a41ce497f634631e1c8
MANIFEST final-4-A acquisition bytes=32 sha256=cd5422716daf1a692073fdf79513a763420a74453a6568b6b8ef540e9e67fe90
MANIFEST final-4-A validation bytes=16 sha256=b2064a307cbf0231454de3c70f6972fcf377882c9e8765a5e7f0b9fe13acf951
MANIFEST final-4-A final bytes=32 sha256=f993957803dfb94eb92e9d7c979801e7817c7618cc7340beb57a9485caba6ec6
MANIFEST final-4-A revision-acquisition bytes=16 sha256=b41d0015095b80ba75cbf8087025aa0bb034a9d3ea3e519634ded7077130b16b
MANIFEST final-4-A revision-validation bytes=8 sha256=313ac8b99f04f14eea6920c11ddd2b75c26d901ceb5481690999f1f1ea34d0bc
MANIFEST final-4-A revision-final bytes=16 sha256=00f0874884ddaea491763e2973fd05992dc5dfe54f54587f8c03ed488936d92b
MANIFEST final-4-B acquisition bytes=32 sha256=a58629a281ed052ec9f534d3088c0245e48d6cc0d9989e5350106affb1fda920
MANIFEST final-4-B validation bytes=16 sha256=6d98ebcf94e5d11b29a074791d2c8c965d3d8d18364a052e2273347e7e183a18
MANIFEST final-4-B final bytes=32 sha256=1b9924fa6f067ec0843fd5d73a65066cf19d865ac3caad41a6525d2b117ab116
```

## Resultados detalhados por bloco/contexto

| Condição | Bloco | Tarefa | Fase | Contexto | Pos. | Neg. | Acertos pos. | Acertos neg. | Erros | Abstenções | Inconclusões |
|---|---:|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| Candidate | 1 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Candidate | 1 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Candidate | 1 | T-A | A1 | A | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Candidate | 1 | T-A | A1 | B | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Candidate | 1 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| Candidate | 1 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Candidate | 1 | T-V | V1 | A | 2 | 14 | 2 | 14 | 0 | 0 | 0 |
| Candidate | 1 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Candidate | 2 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Candidate | 2 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Candidate | 2 | T-A | A1 | A | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Candidate | 2 | T-A | A1 | B | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Candidate | 2 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| Candidate | 2 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Candidate | 2 | T-V | V1 | A | 2 | 14 | 2 | 14 | 0 | 0 | 0 |
| Candidate | 2 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Candidate | 3 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Candidate | 3 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Candidate | 3 | T-A | A1 | A | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Candidate | 3 | T-A | A1 | B | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Candidate | 3 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| Candidate | 3 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Candidate | 3 | T-V | V1 | A | 2 | 14 | 2 | 14 | 0 | 0 | 0 |
| Candidate | 3 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Candidate | 4 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Candidate | 4 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Candidate | 4 | T-A | A1 | A | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Candidate | 4 | T-A | A1 | B | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Candidate | 4 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| Candidate | 4 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Candidate | 4 | T-V | V1 | A | 2 | 14 | 2 | 14 | 0 | 0 | 0 |
| Candidate | 4 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Literal | 1 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 1 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 1 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 1 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 1 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| Literal | 1 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| Literal | 1 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| Literal | 1 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| Literal | 2 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 2 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 2 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 2 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 2 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| Literal | 2 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| Literal | 2 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| Literal | 2 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| Literal | 3 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 3 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 3 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 3 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 3 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| Literal | 3 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| Literal | 3 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| Literal | 3 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| Literal | 4 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 4 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 4 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 4 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Literal | 4 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| Literal | 4 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| Literal | 4 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| Literal | 4 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| AlwaysPositive | 1 | T-A | A0 | A | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 1 | T-A | A0 | B | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 1 | T-A | A1 | A | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 1 | T-A | A1 | B | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 1 | T-V | V0 | A | 2 | 14 | 2 | 0 | 14 | 0 | 0 |
| AlwaysPositive | 1 | T-V | V0 | B | 4 | 12 | 4 | 0 | 12 | 0 | 0 |
| AlwaysPositive | 1 | T-V | V1 | A | 2 | 14 | 2 | 0 | 14 | 0 | 0 |
| AlwaysPositive | 1 | T-V | V1 | B | 4 | 12 | 4 | 0 | 12 | 0 | 0 |
| AlwaysPositive | 2 | T-A | A0 | A | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 2 | T-A | A0 | B | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 2 | T-A | A1 | A | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 2 | T-A | A1 | B | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 2 | T-V | V0 | A | 2 | 14 | 2 | 0 | 14 | 0 | 0 |
| AlwaysPositive | 2 | T-V | V0 | B | 4 | 12 | 4 | 0 | 12 | 0 | 0 |
| AlwaysPositive | 2 | T-V | V1 | A | 2 | 14 | 2 | 0 | 14 | 0 | 0 |
| AlwaysPositive | 2 | T-V | V1 | B | 4 | 12 | 4 | 0 | 12 | 0 | 0 |
| AlwaysPositive | 3 | T-A | A0 | A | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 3 | T-A | A0 | B | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 3 | T-A | A1 | A | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 3 | T-A | A1 | B | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 3 | T-V | V0 | A | 2 | 14 | 2 | 0 | 14 | 0 | 0 |
| AlwaysPositive | 3 | T-V | V0 | B | 4 | 12 | 4 | 0 | 12 | 0 | 0 |
| AlwaysPositive | 3 | T-V | V1 | A | 2 | 14 | 2 | 0 | 14 | 0 | 0 |
| AlwaysPositive | 3 | T-V | V1 | B | 4 | 12 | 4 | 0 | 12 | 0 | 0 |
| AlwaysPositive | 4 | T-A | A0 | A | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 4 | T-A | A0 | B | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 4 | T-A | A1 | A | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 4 | T-A | A1 | B | 8 | 24 | 8 | 0 | 24 | 0 | 0 |
| AlwaysPositive | 4 | T-V | V0 | A | 2 | 14 | 2 | 0 | 14 | 0 | 0 |
| AlwaysPositive | 4 | T-V | V0 | B | 4 | 12 | 4 | 0 | 12 | 0 | 0 |
| AlwaysPositive | 4 | T-V | V1 | A | 2 | 14 | 2 | 0 | 14 | 0 | 0 |
| AlwaysPositive | 4 | T-V | V1 | B | 4 | 12 | 4 | 0 | 12 | 0 | 0 |
| AlwaysNegative | 1 | T-A | A0 | A | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 1 | T-A | A0 | B | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 1 | T-A | A1 | A | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 1 | T-A | A1 | B | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 1 | T-V | V0 | A | 2 | 14 | 0 | 14 | 2 | 0 | 0 |
| AlwaysNegative | 1 | T-V | V0 | B | 4 | 12 | 0 | 12 | 4 | 0 | 0 |
| AlwaysNegative | 1 | T-V | V1 | A | 2 | 14 | 0 | 14 | 2 | 0 | 0 |
| AlwaysNegative | 1 | T-V | V1 | B | 4 | 12 | 0 | 12 | 4 | 0 | 0 |
| AlwaysNegative | 2 | T-A | A0 | A | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 2 | T-A | A0 | B | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 2 | T-A | A1 | A | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 2 | T-A | A1 | B | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 2 | T-V | V0 | A | 2 | 14 | 0 | 14 | 2 | 0 | 0 |
| AlwaysNegative | 2 | T-V | V0 | B | 4 | 12 | 0 | 12 | 4 | 0 | 0 |
| AlwaysNegative | 2 | T-V | V1 | A | 2 | 14 | 0 | 14 | 2 | 0 | 0 |
| AlwaysNegative | 2 | T-V | V1 | B | 4 | 12 | 0 | 12 | 4 | 0 | 0 |
| AlwaysNegative | 3 | T-A | A0 | A | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 3 | T-A | A0 | B | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 3 | T-A | A1 | A | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 3 | T-A | A1 | B | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 3 | T-V | V0 | A | 2 | 14 | 0 | 14 | 2 | 0 | 0 |
| AlwaysNegative | 3 | T-V | V0 | B | 4 | 12 | 0 | 12 | 4 | 0 | 0 |
| AlwaysNegative | 3 | T-V | V1 | A | 2 | 14 | 0 | 14 | 2 | 0 | 0 |
| AlwaysNegative | 3 | T-V | V1 | B | 4 | 12 | 0 | 12 | 4 | 0 | 0 |
| AlwaysNegative | 4 | T-A | A0 | A | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 4 | T-A | A0 | B | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 4 | T-A | A1 | A | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 4 | T-A | A1 | B | 8 | 24 | 0 | 24 | 8 | 0 | 0 |
| AlwaysNegative | 4 | T-V | V0 | A | 2 | 14 | 0 | 14 | 2 | 0 | 0 |
| AlwaysNegative | 4 | T-V | V0 | B | 4 | 12 | 0 | 12 | 4 | 0 | 0 |
| AlwaysNegative | 4 | T-V | V1 | A | 2 | 14 | 0 | 14 | 2 | 0 | 0 |
| AlwaysNegative | 4 | T-V | V1 | B | 4 | 12 | 0 | 12 | 4 | 0 | 0 |
| AlwaysAbstain | 1 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 1 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 1 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 1 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 1 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 1 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 1 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 1 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 2 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 2 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 2 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 2 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 2 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 2 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 2 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 2 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 3 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 3 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 3 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 3 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 3 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 3 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 3 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 3 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 4 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 4 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 4 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 4 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| AlwaysAbstain | 4 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 4 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 4 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| AlwaysAbstain | 4 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 1 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 1 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 1 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 1 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 1 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 1 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 1 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 1 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 2 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 2 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 2 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 2 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 2 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 2 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 2 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 2 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 3 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 3 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 3 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 3 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 3 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 3 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 3 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 3 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 4 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 4 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 4 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 4 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoAcquisition | 4 | T-V | V0 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 4 | T-V | V0 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 4 | T-V | V1 | A | 2 | 14 | 0 | 0 | 0 | 16 | 0 |
| NoAcquisition | 4 | T-V | V1 | B | 4 | 12 | 0 | 0 | 0 | 16 | 0 |
| NoUpdate | 1 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 1 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 1 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 1 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 1 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| NoUpdate | 1 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| NoUpdate | 1 | T-V | V1 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| NoUpdate | 1 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| NoUpdate | 2 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 2 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 2 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 2 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 2 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| NoUpdate | 2 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| NoUpdate | 2 | T-V | V1 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| NoUpdate | 2 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| NoUpdate | 3 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 3 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 3 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 3 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 3 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| NoUpdate | 3 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| NoUpdate | 3 | T-V | V1 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| NoUpdate | 3 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| NoUpdate | 4 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 4 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 4 | T-A | A1 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 4 | T-A | A1 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| NoUpdate | 4 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| NoUpdate | 4 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| NoUpdate | 4 | T-V | V1 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| NoUpdate | 4 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Independent | 1 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Independent | 1 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Independent | 1 | T-A | A1 | A | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Independent | 1 | T-A | A1 | B | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Independent | 1 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| Independent | 1 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Independent | 1 | T-V | V1 | A | 2 | 14 | 2 | 14 | 0 | 0 | 0 |
| Independent | 1 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Independent | 2 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Independent | 2 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Independent | 2 | T-A | A1 | A | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Independent | 2 | T-A | A1 | B | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Independent | 2 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| Independent | 2 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Independent | 2 | T-V | V1 | A | 2 | 14 | 2 | 14 | 0 | 0 | 0 |
| Independent | 2 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Independent | 3 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Independent | 3 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Independent | 3 | T-A | A1 | A | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Independent | 3 | T-A | A1 | B | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Independent | 3 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| Independent | 3 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Independent | 3 | T-V | V1 | A | 2 | 14 | 2 | 14 | 0 | 0 | 0 |
| Independent | 3 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Independent | 4 | T-A | A0 | A | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Independent | 4 | T-A | A0 | B | 8 | 24 | 0 | 0 | 0 | 32 | 0 |
| Independent | 4 | T-A | A1 | A | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Independent | 4 | T-A | A1 | B | 8 | 24 | 8 | 24 | 0 | 0 | 0 |
| Independent | 4 | T-V | V0 | A | 2 | 14 | 2 | 6 | 8 | 0 | 0 |
| Independent | 4 | T-V | V0 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| Independent | 4 | T-V | V1 | A | 2 | 14 | 2 | 14 | 0 | 0 | 0 |
| Independent | 4 | T-V | V1 | B | 4 | 12 | 4 | 12 | 0 | 0 | 0 |
| FixedP7 | 1 | T-A | A0 | A | 8 | 24 | 4 | 11 | 17 | 0 | 0 |
| FixedP7 | 1 | T-A | A0 | B | 8 | 24 | 5 | 11 | 16 | 0 | 0 |
| FixedP7 | 1 | T-A | A1 | A | 8 | 24 | 4 | 11 | 17 | 0 | 0 |
| FixedP7 | 1 | T-A | A1 | B | 8 | 24 | 5 | 11 | 16 | 0 | 0 |
| FixedP7 | 1 | T-V | V0 | A | 2 | 14 | 1 | 6 | 9 | 0 | 0 |
| FixedP7 | 1 | T-V | V0 | B | 4 | 12 | 3 | 4 | 9 | 0 | 0 |
| FixedP7 | 1 | T-V | V1 | A | 2 | 14 | 1 | 6 | 9 | 0 | 0 |
| FixedP7 | 1 | T-V | V1 | B | 4 | 12 | 3 | 4 | 9 | 0 | 0 |
| FixedP7 | 2 | T-A | A0 | A | 8 | 24 | 4 | 11 | 17 | 0 | 0 |
| FixedP7 | 2 | T-A | A0 | B | 8 | 24 | 4 | 12 | 16 | 0 | 0 |
| FixedP7 | 2 | T-A | A1 | A | 8 | 24 | 4 | 11 | 17 | 0 | 0 |
| FixedP7 | 2 | T-A | A1 | B | 8 | 24 | 4 | 12 | 16 | 0 | 0 |
| FixedP7 | 2 | T-V | V0 | A | 2 | 14 | 1 | 5 | 10 | 0 | 0 |
| FixedP7 | 2 | T-V | V0 | B | 4 | 12 | 2 | 5 | 9 | 0 | 0 |
| FixedP7 | 2 | T-V | V1 | A | 2 | 14 | 1 | 5 | 10 | 0 | 0 |
| FixedP7 | 2 | T-V | V1 | B | 4 | 12 | 2 | 5 | 9 | 0 | 0 |
| FixedP7 | 3 | T-A | A0 | A | 8 | 24 | 5 | 13 | 14 | 0 | 0 |
| FixedP7 | 3 | T-A | A0 | B | 8 | 24 | 0 | 8 | 24 | 0 | 0 |
| FixedP7 | 3 | T-A | A1 | A | 8 | 24 | 5 | 13 | 14 | 0 | 0 |
| FixedP7 | 3 | T-A | A1 | B | 8 | 24 | 0 | 8 | 24 | 0 | 0 |
| FixedP7 | 3 | T-V | V0 | A | 2 | 14 | 1 | 7 | 8 | 0 | 0 |
| FixedP7 | 3 | T-V | V0 | B | 4 | 12 | 0 | 4 | 12 | 0 | 0 |
| FixedP7 | 3 | T-V | V1 | A | 2 | 14 | 1 | 7 | 8 | 0 | 0 |
| FixedP7 | 3 | T-V | V1 | B | 4 | 12 | 0 | 4 | 12 | 0 | 0 |
| FixedP7 | 4 | T-A | A0 | A | 8 | 24 | 0 | 8 | 24 | 0 | 0 |
| FixedP7 | 4 | T-A | A0 | B | 8 | 24 | 4 | 10 | 18 | 0 | 0 |
| FixedP7 | 4 | T-A | A1 | A | 8 | 24 | 0 | 8 | 24 | 0 | 0 |
| FixedP7 | 4 | T-A | A1 | B | 8 | 24 | 4 | 10 | 18 | 0 | 0 |
| FixedP7 | 4 | T-V | V0 | A | 2 | 14 | 0 | 8 | 8 | 0 | 0 |
| FixedP7 | 4 | T-V | V0 | B | 4 | 12 | 2 | 4 | 10 | 0 | 0 |
| FixedP7 | 4 | T-V | V1 | A | 2 | 14 | 0 | 8 | 8 | 0 | 0 |
| FixedP7 | 4 | T-V | V1 | B | 4 | 12 | 2 | 4 | 10 | 0 | 0 |
