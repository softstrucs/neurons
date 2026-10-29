# EC-05 — Resultados do diagnóstico local de mudança e conflito

## 1. Escopo e parecer

Base `b5995bfba23510e56244ebbb9439687735d42dce`, branch devs inicialmente limpa, HEAD/origin/devs/referência remota coincidentes. A delegação aprovou expressamente EC-03.3-R1 como contrato diagnóstico local e autorizou protocolo prospectivo, sondas isoladas, execução, documentação e versionamento. Isso resolveu a pendência da preparação anterior, sem promover o contrato a política geral e sem importar a recência EC-04.

**Subetapa “testar mudança ambiental e conflito” concluída no painel delimitado. Parecer: resultados funcionais conformes às previsões; conclusão científica AJUSTAR pelas limitações de escopo/instrumentação preservadas.** Não se certifica implementação integral de proveniência, contexto ou aprendizagem geral. A etapa EC-05 completa não está encerrada: política geral de revisão/abstenção e avaliação posterior de retenção não foram iniciadas.

O [protocolo operacional](protocolos/EC-05_protocolo_diagnostico_contextual.md), especialmente sua matriz §4, contém regras, estados, operações, consultas, controles e critérios congelados antes da implementação. As fontes obrigatórias foram examinadas: plano v0.2 integral, Fundamento, modelagem/preparação EC-05, protocolos EC-03.3 original/R1, relatórios e auditorias EC-03/04, decisões e código correspondente. Todos os artefatos históricos permanecem inalterados, incluindo os documentos que registravam a pendência na data de sua redação.

## 2. O que foi implementado

Somente orquestração, fixtures e fiscalização de evidências novas na infraestrutura de testes. Os executores `provenance_candidate.rs`, `provenance_symbolic.rs`, `provenance_literal.rs` e os DTOs históricos foram reutilizados **sem alteração de bytes**. Nenhum código de produção, arquitetura, dependência, checkpoint ou mecanismo cognitivo foi modificado.

- `tests/support/ec05_cases.rs`: oito trajetórias determinísticas para sete cenários A–G (duas trajetórias D), em três namespaces separados D/V/F. Expõe operações e consultas; expectativas são apenas do avaliador.
- `tests/ec05_context_change.rs`: quatro testes Cargo, preparação D/V, preflight de manifestos e painel final de duas sequências globais completas. Preserva respostas completas, operações, causas, provas, evidências, conflitos, cortes, épocas e trabalho retornado.
- `tests/support/ec05_evidence.rs`: fiscaliza provas/referências pelo prefixo de diário autorizado, sem chamar ou modificar a decisão de nenhum executor. Confere versões correntes, estado de fonte/fato/regra, intervalo, contexto, polaridade, conectividade, observação e completude de provas elegíveis quando a decisão exige prova. Também confere os conjuntos de conflito/evidência. Não é professor interno nem quarto mecanismo de decisão.

As falhas de Query são Result::Err, como no suporte histórico; InvalidCut não vira Unknown. O registro novo marca `cost_known=false` nessas rejeições: zeros nos campos numéricos são ausência de Work retornado, **não alegação de trabalho nulo**. Nenhuma informação do avaliador é devolvida aos modelos.

## 3. Cronologia e integridade

O protocolo foi escrito e seu SHA-256 guardado antes de criar os testes. Preflight verificou repetição dos manifestos, quantidades de sondas por trajetória, três diferenças esperadas e tetos de operações/fontes/regras. Desenvolvimento passou, depois validação passou; a instrumentação de transporte de custo foi concluída ainda em preparação, e D/V foram repetidos antes de F. Não se alteraram regras ou expectativas do protocolo. Código e manifestos foram congelados por hash antes da primeira execução final. **Nenhum código/protocolo/fixture foi modificado depois de observar F.**

D/V têm as mesmas estruturas do final, com IDs distintos: é preparação de conformidade, não prova de generalização ou independência estrutural. O autor conhece os resultados previstos. São dados sintéticos fornecidos, não aquisição autônoma de semântica.

| Laboratório | Verificações | Resultado |
|---|---:|---|
| Desenvolvimento, uma sequência |258|Todas conformes|
| Validação, uma sequência |258|Todas conformes|
| Final, duas sequências globais |516|Todas conformes|

Cada manifesto canônico é a representação Debug determinística da lista de operações/consultas/expectativas; tamanho 16112 bytes por laboratório. Não é formato persistente de conhecimento nem garantia de portabilidade entre versões de compilador. Hashes foram recalculados pelo preflight existente e os arquivos congelados conferidos após execução. Inventário inicial protegeu 112 arquivos rastreados; apenas o acréscimo autorizado em decisões modifica arquivo existente nesta entrega.

## 4. Resultados observados por cenário

Legenda: U Unknown, S Supported, C Conflict, E Expired, T Stale, P Suspended, R Retracted. As sequências observadas abaixo **coincidiram com as previsões congeladas**, inclusive diferenças do literal. Candidato e simbólico têm as mesmas sequências. Cada sequência é por repetição; ambas foram comparadas integralmente, com custos/erros/comandos também iguais dentro de cada condição.

| Cenário | Sondas por repetição | Candidato/simbólico observados | Literal observado | Evidência/limite |
|---|---:|---|---|---|
| A — nova informação ambiental |5|U,S,T,T,S|U,S,T,T,S|A2 muda X→Y para X→W; ausência da ligação W→Z impede recomposição; depois C1 permite prova A2/C1. Mudança recebida externamente, não percebida autonomamente.|
| B — contextos distintos |4|U,S,U,U|U,S,U,U|Negativa em d não contamina c; d sem composição positiva e contexto e vazio são Unknown, não conclusão negativa.|
| C — oposição |6|U,S,C,C,S,S|U,S,C,C,S,S|Oposição ativa exposta antes/depois de Refresh; retirada do negativo remove conflito corrente, sem eleger fonte vencedora.|
| D — validade, via única |7|U,S,S,E,E,E,S|U,S,S,E,E,S,S|Época3 exclui A1 [1,3); literal já recompõe com A2 antes de Refresh.|
| D — via alternativa |4|U,S,S,S|U,S,S,S|Duas provas iniciais; após expiração somente via W sustenta, conservando causa Expired da via antiga.|
| E — suspensão/revisão |10|U,S,P,P,S,T,S,P,P,S|U,S,P,P,S,S,S,P,P,S|Fonte/regra suspensas imediatamente inelegíveis; retomada sondada após Refresh; revisão conserva diferença prevista do literal.|
| F — contexto não afetado |7|U,S,S,P,S,P,S|U,S,S,P,S,P,S|Dois contextos no mesmo diário, fontes/regras distintas; d conserva decisão e evidências nas duas sondas após mudança em c. Corte global real é registrado e só normalizado na comparação do conteúdo.|
| G — histórico |8|U,S,T,S,R,S,S,U|U,S,S,S,R,S,S,U|H1/H2 reproduzem Answer completo, inclusive versões, causas e referências; corte0 permanece vazio; adicionalmente, corte futuro rejeitado.|

F contém duas comparações de conservação por condição/repetição: **12/12** nas três condições principais e duas repetições. Esse é controle local de contexto, não estudo longitudinal de retenção nem resistência à interferência em pesos compartilhados. Em G, três consultas históricas por condição/repetição: **18/18** no total (12 Supported nos dois cortes registrados e 6 Unknown no corte vazio). History refere-se somente ao replay que a interface existente oferece, não recuperação universal por identidade.

Foram fiscalizadas **168 ocorrências de prova** devolvidas pelas três condições nas duas repetições, inclusive provas positivas conservadas sob decisão Conflict. Nenhuma referência inelegível/cross-contexto foi aceita pelo avaliador. Uma prova sob Conflict não autoriza resposta Supported nem resolve a oposição. Não se executou um ataque novo contra o verificador; conformidade nessas trajetórias não prova ausência de bugs em todas as entradas possíveis.

## 5. Contagens e comparação de controles

| Unidade do painel final | Previsto | Observado |
|---|---:|---:|
| Consultas principais (51 ×3 ×2) |306|306|
| Destas, históricas |18|18|
| InvalidCut, sem mutação (1 ×3 ×2) |6|6|
| Consultas das duas ablações (51 ×2 ×2) |204|204|
| Total de consultas |510|510|
| Total consultas + erros verificados |516|516|
| Eventos de Circuit/Engine |0|0|

Não confundir consultas com verificações de erro ou com testes Cargo. A suíte inteira repete os testes existentes, inclusive este painel; seus resultados não são somados às 516 verificações de uma execução final.

Decisões **por condição e repetição**, denominador51:

| Condição |S|U|C|E|T|P|R|Unavailable|
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Candidato |24|11|2|3|4|6|1|0|
| Simbólico independente |24|11|2|3|4|6|1|0|
| Literal forte |27|11|2|2|2|6|1|0|
| Sem aquisição |0|51|0|0|0|0|0|0|
| Informação indisponível |0|0|0|0|0|0|0|51|

Candidato/simbólico: cobertura24/51 (~47,06%), precisão entre respostas sustentadas24/24 no contrato. Literal: cobertura27/51 (~52,94%), precisão 27/27. Nas três condições principais, decisões conformes51/51 mais InvalidCut1/1 por repetição; nenhuma resposta indevida, falsa abstenção **frente às expectativas específicas do contrato** ou erro inesperado. Isso não transforma as abstenções em acertos relacionais nem declara ótima a política de Refresh.

Candidato/simbólico coincidiram nas 102/102 respostas completas das duas repetições (custos são diferentes). Literal coincidiu em 96/102 e diferiu em 6, exatamente D/E/G pré-Refresh de cada repetição. São três respostas sustentadas adicionais por repetição, mostrando que a recuperação literal é mais pronta nessas sondas. Não se exige vitória do candidato nem concordância artificial.

Sem aquisição teve 102 Unknown e indisponibilidade 102 Unavailable nas duas repetições; zero Supported. São ablações deliberadas de informação, não comparação justa de superioridade algorítmica. A ablação vazia consulta corte0, embora se registre o corte solicitado ao harness; não é recuperação de história adquirida. A indisponibilidade é produzida pelo adaptador externo que bloqueia acesso ao conteúdo, não detecção cognitiva interna. Ambas as ablações têm precisão entre respostas indefinida, não 100%.

## 6. Custos e recursos

Os custos abaixo são o **Work nominal retornado pelos suportes históricos**, por condição/repetição. Não certificam contagem uniforme de todas as visitas, rejeições ou alocações. Diferença de custo não sustenta comparação científica de eficiência enquanto essa ressalva permanecer.

| Condição |Leituras aquisição|Pares aquisição|Leituras consulta|Pares consulta|Máximo nominal por chamada|Bytes dos registros da sequência|
|---|---:|---:|---:|---:|---:|---:|
| Candidato |13125|257|25992|17|1471|25384|
| Simbólico |35078|650|64959|778|4324|25384|
| Literal |7642|328|25411|889|1381|25912|
| Sem aquisição |0|0|51|0|1|20568|
| Indisponível |13125|257|0|0|1471|20619|

Máximo nominal 4324 <8192. InvalidCut tem custo desconhecido porque Result::Err não devolve Work; a flag cost_known impede interpretar zeros de transporte como zero trabalho. As consultas não mudaram o estado, verificado por comparação integral Debug antes/depois, sem compartilhar essa fiscalização com o mecanismo.

Memória: guardas somam capacidades dos registros/fixtures/diários novos, respostas salvas e estados coexistentes via bytes(), mais reserva transitória declarada de 16 KiB. Pico contabilizado final **239964 bytes** (~234,34KiB), abaixo de 256 KiB; preparação 172865 bytes. Inclui os registros das cinco primeiras condições coexistentes com a repetição corrente. Métodos bytes() históricos contêm reservas e não equivalem à medição integral do alocador; versões de metadados/fontes também continuam parciais. Conformidade fina de memória e trabalho permanece **parcialmente verificável**, sem excesso observado. Não extrapolar o resultado para os tetos de capacidade não exercitados.

Painel isolado em Rust 1.88/Cargo 1.88: **0,219562s internos**, **0,221280s de parede**. wait4 reportou **7840KiB** (~7,66MiB); /proc amostrado a cada 3 ms reportou máximo 1564 KiB. wait4 mede o escopo do processo lançado (incluindo fase fork/exec e contabilização de filhos aguardados), não memória exclusiva de objetos Rust ou soma simultânea de toda árvore; a amostragem sozinha pode perder picos. Os dois números não são intercambiáveis. Nenhum limite RSS 256 MiB/tempo 30 s/disco foi acionado. /home mínimo observado pelo monitor isolado: **4630990848 bytes**, acima de 3 GiB.

Verificações e regressões: cargo fmt --check, cargo check --offline, cargo check --offline --all-targets, cargo test --offline -- --test-threads=1 e git diff --check passaram. **136 testes aprovados, 0 falhos/ignorados**, incluindo os 132 anteriores e os quatro novos. A suíte levou 2,921218 s. Maior pico acumulado de um filho na sequência de verificações/compilação/testes:125928KiB; não é soma de processos ou pico exclusivo do painel. A primeira compilação de preparação não teve medição RSS isolada; não se forçou rebuild limpo.

Target: 55415100 → 57618855 bytes, crescimento 2203755 bytes (~2,10MiB). /home após verificações: 4631986176 bytes (~4,31GiB). Nenhuma instalação, dependência externa, thread própria, serviço ou modificação de produção. O estado final do disco será reconfirmado após commit/push.

## 7. Conformidades, limitações e alcance

| Aspecto |Parecer|
|---|---|
| Adoção de contrato e regras dos sete cenários |Conforme na matriz local e nas trajetórias executadas; nenhuma política EC-04 incorporada.|
| Separação aquisição/consulta/avaliador |Conforme nos fluxos inspecionados: somente Op/Query/cut entram nos modelos; categorias/expectativas não entram.|
| Fiscalização de provas e causas |Provas/evidências/conflitos completos comparados nos fluxos; causas preservadas nas respostas. Não é autenticação da fonte ou validação universal do próprio fiscal.|
| Conservação contextual e histórico |Conformes no escopo delimitado; não substituem History geral nem teste de retenção longitudinal.|
| Controles e repetibilidade |Duas sequências globais exatas; separação de código dos três executores, mesmos fatos e limites nominais; mesmos pressupostos/autor/avaliador limitam independência científica.|
| Trabalho e memória fina |Parcial: instrumentação histórica incompleta e reservas conservadas, rejeição sem Work. Nenhum excesso observado autoriza declarar completa a unidade de custo.|
| Fonte/proveniência/revisão geral |Parcial/inexistente fora do contrato: fonte é identificável, não autenticada; modo Received é convenção, History por corte não é API geral por versão; revisão de regras não foi ampliada.|
| Recursos de execução medidos |Guardas e medidas de RSS/tempo/disco atendidas nesta reprodução, com escopos e limitações declarados.|

Esses resultados são funcionamento verificável de inferência e manutenção programadas sobre relatos externos. A mudança de A depende de informação/Revise fornecidos; a suspensão e retirada também são comandos externos. Não se demonstraram descoberta ambiental autônoma, credibilidade aprendida de fontes, compreensão textual, aprendizagem neural, aquisição autônoma de significado, resistência à interferência compartilhada ou transferência estrutural. O bloqueio A02.5 não é resolvido por este painel simbólico.

Não houve falha funcional, divergência inesperada ou inconclusão de execução neste painel. As limitações científicas/de instrumentação não são resultados positivos e mantêm a conclusão **AJUSTAR**, sem apagar as ressalvas EC-03/04. Nenhum protocolo histórico foi corrigido ou apresentado como comprovado integralmente. A conclusão da subetapa não autoriza integração em produção.

Próxima subetapa oficial: **definir política de revisão e abstenção**, ainda não iniciada e dependente de nova autorização. A política geral não foi definida por esta adoção local; tampouco se iniciou a avaliação posterior de retenção.

## 8. Custódia e artefatos

SHA-256 do protocolo foi registrado antes de implementar; o mesmo hash permaneceu até o fim. Hashes congelados antes do primeiro final (os relatórios de resultados são posteriores):

```text
85fce7c77521e80bc4a139abc82bd60e1371b51392270c000b79086cedce8e18  docs/protocolos/EC-05_protocolo_diagnostico_contextual.md
4b5ef08323e7951fe83644affc3ad34baa973eeefe5198d08947901478ada9e5  tests/ec05_context_change.rs
7bac7d901199475933b70cd898a485a7f5c774c5901a6f7cfbe290607b429073  tests/support/ec05_cases.rs
efcc3755317cadc0647530bbee15f4b6b0e8222e32fb85d87c1901fdca3ef360  tests/support/ec05_evidence.rs
9d5f3b908e3f77817c0693f35d17c59ae882cdb796513736af05bfa5c82b3bb2  tests/support/provenance_types.rs
491975413ff2e6ba48bce040461484d85ace5492c3f70f6643269794429862a3  tests/support/provenance_candidate.rs
e66af4eca951f4f1a9f5d99a58b424e061b47dbcac9cd24b6a16e751cc7ff814  tests/support/provenance_symbolic.rs
87b4012f8631ae535ad2dd45a02d38c559c23235ea9e15c5a76a67ddf44a5990  tests/support/provenance_literal.rs
```

Manifestos por partição (lista canônica, incluindo expectativas exclusivamente do avaliador):

```text
MANIFEST part=100000 bytes=16112 sha256=48da043a76901d3f0775c85f5e6a714e7364b1be5ce364146e944c79d37f5f16
MANIFEST part=200000 bytes=16112 sha256=4f997d7a0cf3809d0811b2a9697c065e0d7fda479327bb0859850df11c889f11
MANIFEST part=300000 bytes=16112 sha256=363bb56743031bcea1384cbedcf027f9d6d7901bc201b0b85bd85c83bbcd2746
```

O protocolo, os testes e os manifestos foram congelados na ordem declarada; o commit posterior arquiva esses bytes, não constitui prova independente de pré-registro histórico. Registros completos de comandos/consultas/respostas ficam em Trace e são comparados antes de o teste terminar; stdout expõe respostas observadas auditáveis. Não há persistência nova desses estados no produto.

Arquivos desta entrega: protocolo EC-05 novo; teste ec05_context_change e dois suportes ec05_cases/ec05_evidence novos; este relatório novo; acréscimo em docs/decisions.md. Seis arquivos no escopo autorizado. Commit e confirmação de push são fornecidos na entrega, evitando autorreferência.
