//! F1: adaptador local limitado. Não recebe avaliador, partições ou regra conceitual alvo.
//! O checkpoint v1 reexecuta somente comandos lógicos admitidos, nunca comandos de E/S.
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Read, Write};
use std::path::Path;

use crate::cognition::concepts::{ConceptIdentity, IncomingExample, Label, Observation, ReceiptId};
use crate::cognitive_contracts::*;
use crate::engram::{Cue, Engram, EngramId, EngramStore, EvidenceId, RecordStatus};
use crate::event::NodeId;
use crate::integration::acquisition::CognitiveEpisode;
use crate::integration::concepts::{ConceptCoordinator, ConceptEpisodeLog, Limits};
use crate::integration::query::{QueryCoordinator, RecordMetadata, RuleAuthorization};

pub const MAX_OPERATIONS: usize = 128;
pub const MAX_LINE_BYTES: usize = 256;
pub const MAX_CHECKPOINT_BYTES: usize = 40_000;
pub const HELP: &str = "F1 — sessão cognitiva local limitada; fontes 7 e 8 são declarações sintéticas, não autenticadas.
config CATALOGO CONTEXTO REGRA VERSAO INICIO FIM
simbolo ID NOME (máximo 5; sem definição conceitual alvo)
autorizar CONTEXTO (autoriza apenas a regra declarada nesse escopo)
receber ID CONTEXTO ORIGEM DESTINO +|- FONTE EVIDENCIA|- observacao|hipotese
interpretar / avaliar / admitir (receber e avaliar não inserem fatos)
consultar CONTEXTO ORIGEM DESTINO (até 3 consultas livres)
iniciar ORIGEM DESTINO / pedido (um episódio de aquisição, um pedido e um retorno)
retorno FEEDBACK PEDIDO CONTEXTO ORIGEM DESTINO +|- FONTE EVIDENCIA|- observacao|hipotese
interpretar-retorno / avaliar-retorno / admitir-retorno ID [ID]
reavaliar / consultar-final ORIGEM DESTINO
conceito ID NOME / contexto-conceitual CONTEXTO
exemplo RECIBO EXEMPLAR CONTEXTO BITS ROTULO(+|-) FONTE EVIDENCIA|- observacao|hipotese
interpretar-exemplo / avaliar-exemplo / admitir-exemplo ID / revisar
classificar CONTEXTO BITS (6 bits; no máximo um ?)
agora INSTANTE / estado / historico / ajuda / salvar CAMINHO / restaurar CAMINHO / sair
Limites: 128 operações lógicas aceitas, 256 bytes/linha, 8 fatos, 16 exemplos, 32 versões, 128 eventos conceituais, 80 consultas conceituais; sem rede ou linguagem natural.
Persistência F1 v1: configuração, proveniência, pendências, fatos, aquisição, hipóteses, versões e histórico lógico. Exclui circuitos, pesos, potenciais, estado do processo e estilo legado. Não é continuidade neural integral.
";

fn err(e: impl std::fmt::Debug) -> String {
    format!("{e:?}")
}
fn num(s: &str) -> Result<u32, String> {
    s.parse().map_err(|_| "inteiro inválido".into())
}
fn time(s: &str) -> Result<u64, String> {
    s.parse().map_err(|_| "instante inválido".into())
}
fn node(s: &str) -> Result<NodeId, String> {
    NodeId::new(num(s)?).map_err(err)
}
fn record(s: &str) -> Result<EngramId, String> {
    EngramId::new(num(s)?).map_err(err)
}
fn source(s: &str) -> Result<SourceId, String> {
    SourceId::new(num(s)?).map_err(err)
}
fn evidence(s: &str) -> Result<Option<EvidenceId>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(EvidenceId::new(num(s)?).map_err(err)?))
    }
}
fn nature(s: &str) -> Result<DeclaredNature, String> {
    match s {
        "observacao" => Ok(DeclaredNature::Observation),
        "hipotese" => Ok(DeclaredNature::Hypothesis),
        _ => Err("natureza inválida".into()),
    }
}
fn polarity(s: &str) -> Result<Polarity, String> {
    match s {
        "+" => Ok(Polarity::Positive),
        "-" => Ok(Polarity::Negative),
        _ => Err("polaridade inválida".into()),
    }
}
fn bits(s: &str) -> Result<[Option<u8>; 6], String> {
    if s.len() != 6 {
        return Err("seis atributos requeridos".into());
    }
    let mut out = [None; 6];
    for (i, b) in s.bytes().enumerate() {
        out[i] = match b {
            b'0' => Some(0),
            b'1' => Some(1),
            b'?' => None,
            _ => return Err("atributo inválido".into()),
        };
    }
    Observation::new(out).map_err(err)?;
    Ok(out)
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AdmissionStage {
    Received,
    Interpreted,
    Assessed,
    Quarantined,
    Incorporated,
}
#[derive(Clone, Copy, Debug)]
struct Pending {
    id: EngramId,
    information: Information,
    stage: AdmissionStage,
}

/// Não há setter de fatos, hipóteses ou log: mutações passam pelos atos explícitos.
/// `execute` prepara a operação sobre uma reconstrução isolada antes de trocar estado.
pub struct Session {
    scope: Option<Scope>,
    authorized: Option<RuleAuthorization>,
    symbols: Vec<Concept>,
    sources: [SourceId; 2],
    memory: EngramStore,
    metadata: Vec<RecordMetadata>,
    pending: Option<Pending>,
    acquisition: Option<CognitiveEpisode>,
    queries: EpisodeLog,
    concepts: Option<ConceptCoordinator>,
    concept_log: ConceptEpisodeLog,
    at: u64,
    journal: Vec<String>,
}
impl Session {
    pub fn new() -> Self {
        Self {
            scope: None,
            authorized: None,
            symbols: vec![],
            sources: [SourceId::new(7).unwrap(), SourceId::new(8).unwrap()],
            memory: EngramStore::new(8).unwrap(),
            metadata: vec![],
            pending: None,
            acquisition: None,
            queries: EpisodeLog::default(),
            concepts: None,
            concept_log: ConceptEpisodeLog::default(),
            at: 0,
            journal: vec![],
        }
    }
    pub fn memory(&self) -> &EngramStore {
        self.acquisition
            .as_ref()
            .map_or(&self.memory, |e| e.memory())
    }
    pub fn metadata(&self) -> &[RecordMetadata] {
        self.acquisition
            .as_ref()
            .map_or(&self.metadata, |e| e.metadata())
    }
    pub fn concepts(&self) -> Option<&ConceptCoordinator> {
        self.concepts.as_ref()
    }
    pub fn history(&self) -> &[String] {
        &self.journal
    }
    pub fn logical_time(&self) -> u64 {
        self.at
    }
    pub fn snapshot(&self) -> String {
        format!("tempo={} escopo={:?} autorizacao={:?}\nsimbolos={:?}\nfatos={:?}\nmetadados={:?}\npendente={:?}\nconsultas={:?}\naquisicao={:?}\ntrace_aquisicao={:?}\nquarentena={:?}\nmodelos={:?}\nexemplos={:?}\ntrace_conceitual={:?}\nconsultas_conceituais={:?}\noperacoes={:?}",self.at,self.scope,self.authorized,self.symbols,self.memory().iter().collect::<Vec<_>>(),self.metadata(),self.pending,self.queries.entries(),self.acquisition.as_ref().map(|a|a.phase()),self.acquisition.as_ref().map(|a|a.trace()),self.acquisition.as_ref().map(|a|a.quarantine()),self.concepts.as_ref().map(|c|c.models()),self.concepts.as_ref().map(|c|c.examples()),self.concepts.as_ref().map(|c|c.trace()),self.concept_log.entries(),self.journal)
    }
    /// A projeção factual permite verificar que consultas não promovem suas conclusões.
    pub fn factual_snapshot(&self) -> String {
        format!(
            "{:?}|{:?}|{:?}|{:?}",
            self.memory().iter().collect::<Vec<_>>(),
            self.metadata(),
            self.concepts.as_ref().map(|c| c.examples()),
            self.concepts.as_ref().map(|c| c.models())
        )
    }
    pub fn execute(&mut self, line: &str) -> Result<String, String> {
        if line.len() > MAX_LINE_BYTES || line.chars().any(char::is_control) {
            return Err("linha inválida".into());
        }
        let canonical = line.split_whitespace().collect::<Vec<_>>().join(" ");
        match canonical.as_str() {
            "ajuda" => return Ok(HELP.into()),
            "estado" => return Ok(self.snapshot()),
            "historico" => return Ok(format!("{:?}\n{}", self.journal, self.snapshot())),
            _ => {}
        }
        if self.journal.len() >= MAX_OPERATIONS {
            return Err("capacidade de operações excedida".into());
        }
        let mut staged = Self::replay(&self.journal)?;
        let result = staged.apply(&canonical)?;
        staged.journal.push(canonical);
        *self = staged;
        Ok(result)
    }
    fn replay(journal: &[String]) -> Result<Self, String> {
        let mut state = Self::new();
        for line in journal {
            state.apply(line)?;
            state.journal.push(line.clone());
        }
        Ok(state)
    }
    fn scope_in(&self, context: &str) -> Result<Scope, String> {
        let mut s = self.scope.ok_or("configuração ausente")?;
        s.context = ContextId::new(num(context)?).map_err(err)?;
        Ok(s)
    }
    fn statement(
        &self,
        context: &str,
        from: &str,
        to: &str,
        pol: &str,
    ) -> Result<Statement, String> {
        let f = node(from)?;
        let t = node(to)?;
        let f = self
            .symbols
            .iter()
            .find(|c| c.node() == f)
            .ok_or("símbolo de origem ausente")?;
        let t = self
            .symbols
            .iter()
            .find(|c| c.node() == t)
            .ok_or("símbolo de destino ausente")?;
        Statement::inclusion(f, t, polarity(pol)?, self.scope_in(context)?).map_err(err)
    }
    fn info(&self, p: &[&str]) -> Result<Information, String> {
        Ok(Information {
            statement: self.statement(p[0], p[1], p[2], p[3])?,
            declared_source: Some(source(p[4])?),
            evidence: evidence(p[5])?,
            nature: nature(p[6])?,
            received_at: self.at,
        })
    }
    fn apply(&mut self, line: &str) -> Result<String, String> {
        self.at = self.at.checked_add(1).ok_or("tempo excedido")?;
        let p: Vec<_> = line.split_whitespace().collect();
        match p.as_slice() {
            ["config", cat, ctx, rule, version, start, end] => {
                if self.scope.is_some() {
                    return Err("configuração já fixada".into());
                }
                let s = Scope {
                    catalog: CatalogId::new(num(cat)?).map_err(err)?,
                    context: ContextId::new(num(ctx)?).map_err(err)?,
                    rule: RuleRef {
                        id: RuleId::new(num(rule)?).map_err(err)?,
                        version: RuleVersion::new(num(version)?).map_err(err)?,
                    },
                    validity: Validity::new(time(start)?, time(end)?).map_err(err)?,
                };
                if !s.validity.contains(self.at) {
                    return Err("OutsideValidity".into());
                }
                self.scope = Some(s);
                Ok(format!("Escopo declarado {s:?}"))
            }
            ["simbolo", id, name] => {
                if self.acquisition.is_some() || self.memory.len() > 0 || self.symbols.len() >= 5 {
                    return Err("catálogo fixado ou capacidade excedida".into());
                }
                let id = node(id)?;
                if self.symbols.iter().any(|c| c.node() == id) {
                    return Err("Duplicate".into());
                }
                self.symbols.push(
                    Concept::new(
                        id,
                        self.scope.ok_or("configuração ausente")?.catalog,
                        name,
                        "símbolo declarado; sem verificação externa",
                    )
                    .map_err(err)?,
                );
                Ok("Símbolo cadastrado".into())
            }
            ["autorizar", ctx] => {
                if self.authorized.is_some() || self.acquisition.is_some() {
                    return Err("autorização já fixada".into());
                }
                let s = self.scope_in(ctx)?;
                if Some(s) != self.scope {
                    return Err("escopo não configurado".into());
                }
                self.authorized = Some(RuleAuthorization::inclusion_in(s));
                Ok("Regra explicitamente autorizada apenas no escopo configurado".into())
            }
            ["receber", id, rest @ ..] if rest.len() == 7 => {
                if self.acquisition.is_some()
                    || self.pending.is_some_and(|p| {
                        matches!(
                            p.stage,
                            AdmissionStage::Received
                                | AdmissionStage::Interpreted
                                | AdmissionStage::Assessed
                        )
                    })
                {
                    return Err("recepção pendente ou memória vinculada ao episódio".into());
                }
                self.pending = Some(Pending {
                    id: record(id)?,
                    information: self.info(rest)?,
                    stage: AdmissionStage::Received,
                });
                Ok("Informação recebida; não admitida".into())
            }
            ["interpretar"] => {
                let p = self.pending.as_mut().ok_or("sem recepção")?;
                if p.stage != AdmissionStage::Received {
                    return Err("fase inválida".into());
                }
                p.stage = AdmissionStage::Interpreted;
                Ok("Afirmação tipada interpretada; não admitida".into())
            }
            ["avaliar"] => {
                let p = self.pending.ok_or("sem recepção")?;
                if p.stage != AdmissionStage::Interpreted {
                    return Err("fase inválida".into());
                }
                let check = check_observation(
                    p.information,
                    self.scope.ok_or("configuração ausente")?,
                    &self.sources,
                    self.at,
                )
                .and_then(|_| {
                    if self.metadata.iter().any(|m| {
                        m.id == p.id
                            || m.information.statement == p.information.statement
                            || m.information.evidence == p.information.evidence
                    }) {
                        Err(ContractError::Duplicate)
                    } else {
                        Ok(())
                    }
                });
                let mut next = p;
                next.stage = if check.is_ok() {
                    AdmissionStage::Assessed
                } else {
                    AdmissionStage::Quarantined
                };
                self.pending = Some(next);
                Ok(format!("Admissibilidade {check:?}; nenhuma incorporação"))
            }
            ["admitir"] => {
                let p = self.pending.ok_or("sem recepção")?;
                if p.stage != AdmissionStage::Assessed {
                    return Err("admissão não autorizada nesta fase".into());
                }
                check_observation(
                    p.information,
                    self.scope.ok_or("configuração ausente")?,
                    &self.sources,
                    self.at,
                )
                .map_err(err)?;
                let s = p.information.statement;
                let target = i32::try_from(s.object().get()).map_err(err)?;
                self.memory
                    .record(
                        Engram::new(
                            p.id,
                            Cue::new(s.scope().context.get(), s.subject(), target).map_err(err)?,
                            if s.polarity() == Polarity::Positive {
                                1
                            } else {
                                -1
                            },
                            None,
                            self.at,
                            p.information.evidence,
                            RecordStatus::Observed,
                        )
                        .map_err(err)?,
                    )
                    .map_err(err)?;
                self.metadata.push(RecordMetadata {
                    id: p.id,
                    information: p.information,
                    state: EpistemicState::AdmittedObservation,
                    disposition: None,
                });
                self.pending = Some(Pending {
                    stage: AdmissionStage::Incorporated,
                    ..p
                });
                Ok(format!("Observação incorporada explicitamente {:?}", p.id))
            }
            ["consultar", ctx, from, to] => {
                let statement = self.statement(ctx, from, to, "+")?;
                let authorization = self.authorized.ok_or("regra sem autorização explícita")?;
                let id = EpisodeId::new(self.queries.entries().len() as u32 + 1).map_err(err)?;
                // Apenas log externo é mutado; coordenador mantém empréstimo imutável factual.
                let report = if let Some(a) = &self.acquisition {
                    QueryCoordinator::new(a.memory(), a.metadata(), authorization, &self.sources)
                        .map_err(err)?
                        .query(id, statement, self.at, &mut self.queries)
                        .map_err(err)?
                } else {
                    QueryCoordinator::new(
                        &self.memory,
                        &self.metadata,
                        authorization,
                        &self.sources,
                    )
                    .map_err(err)?
                    .query(id, statement, self.at, &mut self.queries)
                    .map_err(err)?
                };
                Ok(format!(
                    "Decisão {:?}\nprova={:?}\nconsultados={:?}\nexcluidos={:?}",
                    report.episode, report.proof, report.consulted, report.excluded
                ))
            }
            ["iniciar", from, to] => {
                if self.acquisition.is_some()
                    || self.pending.is_some_and(|p| {
                        !matches!(
                            p.stage,
                            AdmissionStage::Incorporated | AdmissionStage::Quarantined
                        )
                    })
                {
                    return Err("episódio ou admissão pendente".into());
                }
                let context = self
                    .scope
                    .ok_or("configuração ausente")?
                    .context
                    .get()
                    .to_string();
                let q = self.statement(&context, from, to, "+")?;
                let mut mem = EngramStore::new(8).map_err(err)?;
                for r in self.memory.iter() {
                    mem.record(r).map_err(err)?;
                }
                let mut a = CognitiveEpisode::new(
                    mem,
                    &self.metadata,
                    &self.symbols,
                    self.authorized.ok_or("regra sem autorização explícita")?,
                    &self.sources,
                )
                .map_err(err)?;
                let report = a
                    .evaluate_initial(q, EpisodeId::new(1).map_err(err)?, self.at)
                    .map_err(err)?
                    .clone();
                self.acquisition = Some(a);
                Ok(format!(
                    "Inicial {:?}\nprova={:?}",
                    report.episode, report.proof
                ))
            }
            ["pedido"] => Ok(format!(
                "Pedido {:?}",
                self.acquisition
                    .as_mut()
                    .ok_or("sem episódio")?
                    .derive_request(RequestId::new(1).map_err(err)?, self.at)
                    .map_err(err)?
            )),
            ["retorno", feedback, request, rest @ ..] if rest.len() == 7 => {
                let information = self.info(rest)?;
                let a = self.acquisition.as_mut().ok_or("sem episódio")?;
                let req = a.request().ok_or("sem pedido")?;
                a.receive(
                    FeedbackPackage {
                        id: FeedbackId::new(num(feedback)?).map_err(err)?,
                        request: RequestId::new(num(request)?).map_err(err)?,
                        episode: req.episode,
                        envelope_source: information.declared_source,
                        returned_at: self.at,
                        content: FeedbackContent::One(information),
                    },
                    self.at,
                )
                .map_err(err)?;
                Ok("Retorno recebido; não admitido".into())
            }
            ["interpretar-retorno"] => {
                self.acquisition
                    .as_mut()
                    .ok_or("sem episódio")?
                    .interpret(self.at)
                    .map_err(err)?;
                Ok("Retorno interpretado".into())
            }
            ["avaliar-retorno"] => Ok(format!(
                "Admissibilidade {:?}",
                self.acquisition
                    .as_mut()
                    .ok_or("sem episódio")?
                    .assess_admissibility(self.at)
                    .map_err(err)?
            )),
            ["admitir-retorno", ids @ ..] if ids.len() <= 2 => {
                let ids = ids
                    .iter()
                    .map(|s| record(s))
                    .collect::<Result<Vec<_>, _>>()?;
                self.acquisition
                    .as_mut()
                    .ok_or("sem episódio")?
                    .incorporate(&ids, self.at)
                    .map_err(err)?;
                Ok(format!("Incorporação explícita {ids:?}"))
            }
            ["reavaliar"] => {
                let r = self
                    .acquisition
                    .as_mut()
                    .ok_or("sem episódio")?
                    .reassess(EpisodeId::new(2).map_err(err)?, self.at)
                    .map_err(err)?;
                Ok(format!("Reavaliação {:?}\nprova={:?}", r.episode, r.proof))
            }
            ["consultar-final", from, to] => {
                let ctx = self
                    .scope
                    .ok_or("configuração ausente")?
                    .context
                    .get()
                    .to_string();
                let q = self.statement(&ctx, from, to, "+")?;
                let r = self
                    .acquisition
                    .as_mut()
                    .ok_or("sem episódio")?
                    .query_reserved(q, EpisodeId::new(3).map_err(err)?, self.at)
                    .map_err(err)?;
                Ok(format!(
                    "Consulta posterior {:?}\nprova={:?}",
                    r.episode, r.proof
                ))
            }
            ["conceito", id, name] => {
                if self.concepts.is_some() {
                    return Err("conceito já fixado".into());
                }
                let scope = self.scope.ok_or("configuração ausente")?;
                let mut c = ConceptCoordinator::new(
                    ConceptIdentity::new(node(id)?, scope.catalog, name).map_err(err)?,
                    &self.sources,
                    Limits::default(),
                )
                .map_err(err)?;
                c.register_context(scope, self.at).map_err(err)?;
                self.concepts = Some(c);
                Ok("Conceito opaco criado; H23 inicial, sem regra alvo".into())
            }
            ["contexto-conceitual", ctx] => {
                let s = self.scope_in(ctx)?;
                self.concepts
                    .as_mut()
                    .ok_or("sem conceito")?
                    .register_context(s, self.at)
                    .map_err(err)?;
                Ok("Contexto conceitual registrado".into())
            }
            ["exemplo", receipt, exemplar, ctx, vector, label, src, ev, nat] => {
                let scope = self.scope_in(ctx)?;
                let at = self.at;
                let c = self.concepts.as_mut().ok_or("sem conceito")?;
                c.receive(
                    IncomingExample {
                        receipt: ReceiptId::new(num(receipt)?).map_err(err)?,
                        teacher_episode: EpisodeId::new(num(receipt)?).map_err(err)?,
                        concept: c.concept().id(),
                        exemplar: node(exemplar)?,
                        attributes: bits(vector)?,
                        label: if polarity(label)? == Polarity::Positive {
                            Label::Positive
                        } else {
                            Label::Negative
                        },
                        scope,
                        source: Some(source(src)?),
                        evidence: evidence(ev)?,
                        nature: nature(nat)?,
                        observed_at: at,
                        sent_at: at,
                    },
                    at,
                )
                .map_err(err)?;
                Ok("Exemplo recebido; não admitido".into())
            }
            ["interpretar-exemplo"] => Ok(format!(
                "Interpretação {:?}",
                self.concepts
                    .as_mut()
                    .ok_or("sem conceito")?
                    .interpret(self.at)
                    .map_err(err)?
            )),
            ["avaliar-exemplo"] => Ok(format!(
                "Admissibilidade {:?}",
                self.concepts
                    .as_mut()
                    .ok_or("sem conceito")?
                    .assess_admissibility(self.at)
                    .map_err(err)?
            )),
            ["admitir-exemplo", id] => {
                let id = record(id)?;
                self.concepts
                    .as_mut()
                    .ok_or("sem conceito")?
                    .incorporate(id, self.at)
                    .map_err(err)?;
                Ok(format!(
                    "Exemplo incorporado explicitamente {id:?}; revisão pendente"
                ))
            }
            ["revisar"] => Ok(format!(
                "Revisão {:?}",
                self.concepts
                    .as_mut()
                    .ok_or("sem conceito")?
                    .revise(self.at)
                    .map_err(err)?
            )),
            ["classificar", ctx, vector] => {
                let s = self.scope_in(ctx)?;
                let obs = Observation::new(bits(vector)?).map_err(err)?;
                let id =
                    EpisodeId::new(self.concept_log.entries().len() as u32 + 1).map_err(err)?;
                let r = self
                    .concepts
                    .as_ref()
                    .ok_or("sem conceito")?
                    .query(id, s, obs, self.at, &mut self.concept_log)
                    .map_err(err)?;
                Ok(format!("Classificação {r:?}"))
            }
            ["agora", at] => {
                let at = time(at)?;
                if at < self.at {
                    return Err("tempo regressivo".into());
                }
                self.at = at;
                Ok(format!("Instante lógico {at}"))
            }
            _ => Err("comando F1 inválido; use ajuda".into()),
        }
    }
    /// Formato aditivo distinto dos checkpoints legados. Checksum detecta corrupção,
    /// não autoria. Não aceita comandos de E/S ou estado fornecido sem revalidação.
    pub fn encode(&self) -> Vec<u8> {
        let body = self.journal.join("\n");
        let body = body.as_bytes();
        let mut out = b"NRNF1\0\0\0".to_vec();
        out.extend(1u32.to_le_bytes());
        out.extend((body.len() as u32).to_le_bytes());
        out.extend(checksum(body).to_le_bytes());
        // Também detecta divergência do estado derivado em versões futuras do núcleo.
        out.extend(checksum(self.snapshot().as_bytes()).to_le_bytes());
        out.extend(body);
        out
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > MAX_CHECKPOINT_BYTES {
            return Err("checkpoint excede limite".into());
        }
        if bytes.len() < 32 || &bytes[..8] != b"NRNF1\0\0\0" {
            return Err("formato F1 inválido; checkpoint legado não importado".into());
        }
        if u32::from_le_bytes(bytes[8..12].try_into().unwrap()) != 1 {
            return Err("versão F1 não suportada".into());
        }
        let size = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
        let body = &bytes[32..];
        if body.len() != size
            || checksum(body) != u64::from_le_bytes(bytes[16..24].try_into().unwrap())
        {
            return Err("checkpoint corrompido".into());
        }
        let s = std::str::from_utf8(body).map_err(err)?;
        let lines: Vec<_> = if s.is_empty() {
            vec![]
        } else {
            s.split('\n').map(str::to_string).collect()
        };
        if lines.len() > MAX_OPERATIONS
            || lines.iter().any(|l| {
                l.is_empty()
                    || l.len() > MAX_LINE_BYTES
                    || l.chars().any(char::is_control)
                    || *l != l.split_whitespace().collect::<Vec<_>>().join(" ")
            })
        {
            return Err("journal inválido".into());
        }
        let state = Self::replay(&lines)?;
        if state.encode() != bytes {
            return Err("checkpoint não canônico ou estado reconstruído incompatível".into());
        }
        Ok(state)
    }
    /// Criação exclusiva: nunca sobrescreve arquivo anterior, inclusive symlink.
    /// Escreve staging exclusivo e publica por hard-link atômico, sem substituir destino.
    pub fn save(&self, path: &Path) -> Result<(), String> {
        let name = path
            .file_name()
            .ok_or("destino inválido")?
            .to_string_lossy();
        let temp = path.with_file_name(format!("{name}.f1-pending"));
        let bytes = self.encode();
        if bytes.len() > MAX_CHECKPOINT_BYTES {
            return Err("checkpoint excede limite".into());
        }
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)
            .map_err(err)?;
        let result = (|| -> io::Result<()> {
            f.write_all(&bytes)?;
            f.sync_all()?;
            fs::hard_link(&temp, path)?;
            Ok(())
        })();
        // Só remove staging criado por esta chamada, nunca entrada preexistente.
        let cleanup = fs::remove_file(&temp);
        result.map_err(err)?;
        cleanup.map_err(err)?;
        Ok(())
    }
    pub fn load(path: &Path) -> Result<Self, String> {
        let m = fs::symlink_metadata(path).map_err(err)?;
        if !m.is_file() || m.len() > MAX_CHECKPOINT_BYTES as u64 {
            return Err("arquivo F1 inválido ou excedido".into());
        }
        let mut bytes = vec![];
        fs::File::open(path)
            .map_err(err)?
            .take((MAX_CHECKPOINT_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(err)?;
        Self::decode(&bytes)
    }
}
impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    })
}

pub fn run<R: BufRead, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    let mut session = Session::new();
    writeln!(output, "{HELP}")?;
    output.flush()?;
    for _ in 0..256 {
        let mut bytes = vec![];
        let count = input
            .by_ref()
            .take((MAX_LINE_BYTES + 1) as u64)
            .read_until(b'\n', &mut bytes)?;
        if count == 0 {
            return Ok(());
        }
        if count > MAX_LINE_BYTES {
            writeln!(output, "Erro: linha excede limite; sessão encerrada")?;
            return Ok(());
        }
        let line = match std::str::from_utf8(&bytes) {
            Ok(s) => s.trim_end_matches(['\n', '\r']),
            Err(_) => {
                writeln!(output, "Erro: UTF-8 inválido")?;
                continue;
            }
        };
        let p: Vec<_> = line.split_whitespace().collect();
        let result = match p.as_slice() {
            ["sair"] => return Ok(()),
            ["salvar", path] => session
                .save(Path::new(path))
                .map(|_| "Checkpoint F1 salvo; criação exclusiva".into()),
            ["restaurar", path] => Session::load(Path::new(path)).map(|s| {
                session = s;
                "Checkpoint F1 restaurado".into()
            }),
            _ => session.execute(line),
        };
        match result {
            Ok(text) => writeln!(output, "{text}")?,
            Err(e) => writeln!(output, "Erro: {e}")?,
        }
        output.flush()?;
    }
    writeln!(output, "Limite de entradas atingido; sessão encerrada")
}
