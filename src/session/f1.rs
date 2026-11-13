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

mod composed;

/// Saída estruturada do relatório publicado, sem reinterpretar decisões cognitivas.
pub fn format_composed_report(report: &crate::integration::composed::GoalReport) -> String {
    composed::render(report)
}

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
    /// Consulta MP06 imutável; somente o log externo transitório cresce.
    pub fn query_three(
        &self,
        context: &str,
        from: &str,
        to: &str,
        authorization: crate::integration::query::PathAuthorization,
        log: &mut crate::integration::query::PathEpisodeLog,
    ) -> Result<crate::integration::query::PathQueryReport, String> {
        let q = self.statement(context, from, to, "+")?;
        let a = self.authorized.ok_or("regra sem autorização explícita")?;
        let id = EpisodeId::new(log.entries().len() as u32 + 1).map_err(err)?;
        QueryCoordinator::new(self.memory(), self.metadata(), a, &self.sources)
            .map_err(err)?
            .query_three(id, q, self.at, authorization, 64, log)
            .map_err(err)
    }
    /// Portas internas de leitura para o episódio composto; preservam fontes e regra reais.
    pub(crate) fn composed_coordinator(&self) -> Result<QueryCoordinator<'_>, String> {
        QueryCoordinator::new(
            self.memory(),
            self.metadata(),
            self.authorized.ok_or("regra base não autorizada")?,
            &self.sources,
        )
        .map_err(err)
    }
    pub(crate) fn composed_statement(
        &self,
        scope: Scope,
        from: NodeId,
        to: NodeId,
    ) -> Result<Statement, String> {
        if self.scope != Some(scope) {
            return Err("escopo composto incompatível".into());
        }
        self.statement(
            &scope.context.get().to_string(),
            &from.get().to_string(),
            &to.get().to_string(),
            "+",
        )
    }
    /// Checkpoint F1 v1 inalterado; checksum não autentica origem.
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
    let mut composed_cli = composed::Adapter::default();
    writeln!(output, "{}", composed::HELP)?;
    let mut path_log = crate::integration::query::PathEpisodeLog::default();
    let mut path_authorization = None;
    let mut concept_goal: Option<crate::curriculum::ConceptGoal> = None;
    let mut concept_goal_report: Option<crate::curriculum::ConceptGoalReport> = None;
    writeln!(output,"Objetivo conceitual transitório: objetivo9 ID BITS; avaliacoes-objetivo9 PRE POS (independentes e sem rótulos); preparar/fixar plano9 e permissoes9; acompanhar9 CONSULTAS EXPERIENCIAS (máximos5/2); historico-objetivo9. Objetivo original é reconsultado separadamente; restauração não retoma supervisão.")?;
    let mut teaching_builder: Option<(NodeId, Scope, Vec<crate::curriculum::Lesson>)> = None;
    let mut teaching_plan: Option<crate::curriculum::TeachingPlan> = None;
    let mut teaching_assessment: Option<crate::curriculum::PedagogyAssessment> = None;
    let mut teaching_permissions: Option<crate::curriculum::PedagogyPermissions> = None;
    let mut teaching_report: Option<crate::curriculum::PedagogyReport> = None;
    writeln!(output,"MP09 transitório: plano9; licao9 ID RECIBO EXEMPLAR BITS ROTULO(+|-) FONTE EVIDENCIA; avaliacoes9 PRE1 PRE2 POS1 POS2 (sem rótulos); fixar9; permissoes9 CONTEXTO AVALIAR(sim|nao) APRESENTAR(sim|nao) ADM_IDS(separados por vírgula)|-; ensinar9; historico9. Sem retomada automática, ações externas ou acurácia presumida.")?;
    let mut regulation_permissions: Option<
        crate::integration::investigation::RegulationPermissions,
    > = None;
    let mut regulation_report: Option<crate::integration::investigation::RegulationReport> = None;
    writeln!(output,"MP08 transitório: permissoes8 CONTEXTO AVALIAR(sim|nao) MP06(sim|nao) INVESTIGAR(sim|nao) FONTE_ID|- ADMISSAO_ID|-; regular8 ORIGEM DESTINO CONSULTAS EVENTOS; historico8. Prepare/fixe a fonte7 antes da pergunta. Sem ações externas ou retomada automática.")?;
    let mut source_builder: Option<(SourceId, Vec<Information>)> = None;
    let mut fixed_source: Option<crate::integration::investigation::LocalSource> = None;
    let mut investigation: Option<crate::integration::investigation::Investigation> = None;
    writeln!(output,"MP07 transitório: fonte7 ID / evidencia7 CONTEXTO ORIGEM DESTINO +|- EVIDENCIA|- observacao|hipotese / fixar-fonte7 (antes da pergunta); investigar7 ORIGEM DESTINO; permitir-investigacao7 / pedido7 / permitir-fonte7 / consultar-fonte7 / interpretar7 / avaliar7 / permitir-admissao7 / admitir7 ID / recusar7 / concluir7 / historico7. Fonte e permissões não persistem; restaurar limpa MP07. Ações externas proibidas.")?;
    writeln!(output,"MP06: autorizar3 CONTEXTO / consultar3 CONTEXTO ORIGEM DESTINO / historico3. Até 3 episódios transitórios, 3 premissas, 64 eventos. Autorização/log MP06 não fazem parte do checkpoint F1 v1; restaurar os limpa.")?;
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
            [op, ..] if composed::handles(op) => {
                if concept_goal.is_some()
                    || concept_goal_report.is_some()
                    || teaching_builder.is_some()
                    || teaching_plan.is_some()
                    || teaching_assessment.is_some()
                    || teaching_permissions.is_some()
                    || teaching_report.is_some()
                    || regulation_permissions.is_some()
                    || regulation_report.is_some()
                    || investigation.is_some()
                    || source_builder.is_some()
                    || fixed_source.is_some()
                {
                    Err("episódio paralelo; restaure antes da composição".into())
                } else if line.chars().any(char::is_control) {
                    Err("linha inválida".into())
                } else {
                    composed_cli.execute(&mut session, &p)
                }
            }
            ["ajuda"] => Ok(format!("{}{}", HELP, composed::HELP)),
            [op, ..]
                if composed_cli.active()
                    && !matches!(
                        *op,
                        "estado" | "historico" | "ajuda" | "salvar" | "restaurar"
                    ) =>
            {
                Err("composição fixada/encerrada; comandos cognitivos paralelos bloqueados".into())
            }
            ["historico-objetivo9"] => concept_goal_report
                .as_ref()
                .map(|r| format!("ObjetivoConceitual {r:?}"))
                .ok_or_else(|| "sem supervisão de objetivo conceitual".into()),
            [op, ..]
                if concept_goal_report.is_some()
                    && !matches!(
                        *op,
                        "estado" | "historico" | "ajuda" | "salvar" | "restaurar"
                    ) =>
            {
                Err("objetivo conceitual encerrado; sem retomada ou ensino automático".into())
            }
            ["objetivo9", id, vector] => (|| -> Result<String, String> {
                if concept_goal.is_some()
                    || teaching_report.is_some()
                    || regulation_report.is_some()
                    || investigation.is_some()
                {
                    return Err("objetivo fixado ou episódio paralelo".into());
                }
                let (concept, scope) =
                    crate::curriculum::PedagogyHost::pedagogy_identity(&session)?;
                concept_goal = Some(crate::curriculum::ConceptGoal {
                    id: EpisodeId::new(num(id)?).map_err(err)?,
                    concept,
                    scope,
                    observation: Observation::new(bits(vector)?).map_err(err)?,
                    criterion: crate::curriculum::GoalCriterion::SupportedByCurrentEvidence,
                });
                Ok("Objetivo identificado e sem rótulo esperado".into())
            })(),
            ["avaliacoes-objetivo9", pre, post] => (|| -> Result<String, String> {
                if teaching_assessment.is_some()
                    || teaching_plan.is_some()
                    || teaching_builder.is_none()
                {
                    return Err("avaliações fixadas ou plano ausente".into());
                }
                teaching_assessment = Some(crate::curriculum::PedagogyAssessment::new(
                    &[Observation::new(bits(pre)?).map_err(err)?],
                    &[Observation::new(bits(post)?).map_err(err)?],
                )?);
                Ok("Avaliações independentes fixadas, sem rótulos; objetivo não incluído".into())
            })(),
            ["acompanhar9", queries, experiences] => (|| -> Result<String, String> {
                if teaching_report.is_some()
                    || regulation_report.is_some()
                    || investigation.is_some()
                {
                    return Err("episódio paralelo".into());
                }
                let goal = concept_goal.ok_or("defina objetivo9")?;
                let permissions = teaching_permissions
                    .as_ref()
                    .ok_or("declare permissoes9")?
                    .clone();
                let budget = crate::curriculum::ConceptGoalBudget {
                    queries: num(queries)? as usize,
                    experiences: num(experiences)? as usize,
                };
                if budget.queries > 5 || budget.experiences > 2 {
                    return Err("teto de objetivo conceitual excedido".into());
                }
                let plan = teaching_plan.take().ok_or("fixe plano9")?;
                let assessment = teaching_assessment.take().ok_or("fixe avaliações")?;
                let r = crate::curriculum::supervise_concept_goal(
                    &mut session,
                    goal,
                    plan,
                    assessment,
                    permissions,
                    budget,
                )?;
                let text = format!("ObjetivoConceitual {r:?}");
                concept_goal_report = Some(r);
                Ok(text)
            })(),
            ["historico9"] => teaching_report
                .as_ref()
                .map(|r| format!("MP09 {r:?}"))
                .ok_or_else(|| "sem relatório MP09".into()),
            [op, ..]
                if teaching_report.is_some()
                    && !matches!(
                        *op,
                        "estado" | "historico" | "ajuda" | "salvar" | "restaurar"
                    ) =>
            {
                Err("episódio MP09 encerrado; sem reapresentação automática".into())
            }
            ["plano9"] => (|| -> Result<String, String> {
                if teaching_builder.is_some()
                    || teaching_plan.is_some()
                    || regulation_report.is_some()
                    || investigation.is_some()
                {
                    return Err("plano fixado ou episódio paralelo".into());
                }
                let (id, scope) = crate::curriculum::PedagogyHost::pedagogy_identity(&session)?;
                teaching_builder = Some((id, scope, vec![]));
                Ok("Plano didático em preparação; nenhuma experiência apresentada".into())
            })(),
            ["licao9", id, receipt, exemplar, vector, label, src, ev] => {
                (|| -> Result<String, String> {
                    let (_, _, lessons) = teaching_builder
                        .as_mut()
                        .ok_or("prepare plano9 antes de fixar")?;
                    if lessons.len() >= 3 {
                        return Err("teto de três experiências".into());
                    }
                    let attributes = Observation::new(bits(vector)?)
                        .map_err(err)?
                        .complete()
                        .map_err(err)?;
                    let l = crate::curriculum::Lesson {
                        id: record(id)?,
                        receipt: num(receipt)?,
                        exemplar: node(exemplar)?,
                        attributes,
                        label: if polarity(label)? == Polarity::Positive {
                            Label::Positive
                        } else {
                            Label::Negative
                        },
                        source: source(src)?,
                        evidence: evidence(ev)?.ok_or("evidência didática ausente")?,
                    };
                    lessons.push(l);
                    Ok("Experiência didática preparada; não admitida".into())
                })()
            }
            ["avaliacoes9", pre1, pre2, post1, post2] => (|| -> Result<String, String> {
                if teaching_assessment.is_some()
                    || teaching_plan.is_some()
                    || teaching_builder.is_none()
                {
                    return Err("avaliações já fixadas ou plano ausente".into());
                }
                let obs = |s: &str| -> Result<Observation, String> {
                    Observation::new(bits(s)?).map_err(err)
                };
                teaching_assessment = Some(crate::curriculum::PedagogyAssessment::new(
                    &[obs(pre1)?, obs(pre2)?],
                    &[obs(post1)?, obs(post2)?],
                )?);
                Ok("Observações de avaliação separadas; nenhum gabarito recebido".into())
            })(),
            ["fixar9"] => (|| -> Result<String, String> {
                if teaching_plan.is_some() || teaching_assessment.is_none() {
                    return Err("plano fixado ou avaliações ausentes".into());
                }
                let (id, scope, lessons) = teaching_builder.as_ref().ok_or("prepare plano9")?;
                teaching_plan = Some(crate::curriculum::TeachingPlan::new(*id, *scope, lessons)?);
                teaching_builder = None;
                Ok("Plano e avaliações fixados antes da execução".into())
            })(),
            ["permissoes9", ctx, eval, present, ids] => (|| -> Result<String, String> {
                if teaching_permissions.is_some() || teaching_plan.is_none() {
                    return Err("permissões fixadas ou plano ausente".into());
                }
                let scope = session.scope_in(ctx)?;
                let flag = |s: &str| match s {
                    "sim" => Ok(true),
                    "nao" => Ok(false),
                    _ => Err("use sim ou nao".to_string()),
                };
                let admissions = if *ids == "-" {
                    vec![]
                } else {
                    ids.split(',').map(record).collect::<Result<Vec<_>, _>>()?
                };
                if admissions.len() > 3
                    || admissions
                        .iter()
                        .enumerate()
                        .any(|(i, a)| admissions[..i].contains(a))
                {
                    return Err("autorizações duplicadas ou excessivas".into());
                }
                teaching_permissions = Some(crate::curriculum::PedagogyPermissions {
                    evaluation: flag(eval)?.then_some(scope),
                    presentation: flag(present)?.then_some(scope),
                    admissions,
                });
                Ok("Avaliação, apresentação e admissões explicitamente separadas".into())
            })(),
            ["ensinar9"] => (|| -> Result<String, String> {
                if concept_goal.is_some() {
                    return Err("use acompanhar9 para preservar a supervisão do objetivo".into());
                }
                if regulation_report.is_some() || investigation.is_some() {
                    return Err("episódio paralelo".into());
                }
                let permissions = teaching_permissions
                    .as_ref()
                    .ok_or("declare permissoes9")?
                    .clone();
                let plan = teaching_plan.take().ok_or("fixe plano9")?;
                let assessment = teaching_assessment.take().ok_or("fixe avaliações")?;
                let report =
                    crate::curriculum::teach_concept(&mut session, plan, assessment, permissions)?;
                let text = format!("MP09 {report:?}");
                teaching_report = Some(report);
                Ok(text)
            })(),
            ["historico8"] => regulation_report
                .as_ref()
                .map(|r| format!("MP08 {r:?}"))
                .ok_or_else(|| "sem relatório MP08".into()),
            ["permissoes8", ctx, eval, path, inv, source_id, admission_id] => {
                (|| -> Result<String, String> {
                    if regulation_permissions.is_some()
                        || regulation_report.is_some()
                        || investigation.is_some()
                    {
                        return Err("permissões MP08 já fixadas ou episódio iniciado".into());
                    }
                    let scope = session.scope_in(ctx)?;
                    let flag = |s: &str| match s {
                        "sim" => Ok(true),
                        "nao" => Ok(false),
                        _ => Err("use sim ou nao".to_string()),
                    };
                    let evaluation = flag(eval)?.then_some(scope);
                    let mp06 = flag(path)?.then_some(
                        crate::integration::query::PathAuthorization::inclusion_three(scope),
                    );
                    let investigation_grant = flag(inv)?.then_some(scope);
                    let source_grant = if *source_id == "-" {
                        None
                    } else {
                        Some((scope, source(source_id)?))
                    };
                    let admission = if *admission_id == "-" {
                        None
                    } else {
                        Some((scope, record(admission_id)?))
                    };
                    regulation_permissions =
                        Some(crate::integration::investigation::RegulationPermissions {
                            evaluation,
                            mp06,
                            investigation: investigation_grant,
                            source: source_grant,
                            admission,
                        });
                    Ok(
                        "Concessões explícitas fixadas separadamente; ações externas proibidas"
                            .into(),
                    )
                })()
            }
            ["regular8", from, to, queries, events] => (|| -> Result<String, String> {
                if regulation_report.is_some() || investigation.is_some() {
                    return Err("um episódio supervisor por sessão".into());
                }
                let permissions = regulation_permissions.ok_or("declare permissoes8")?;
                let scope = session.scope.ok_or("sem escopo")?;
                let q = session.statement(&scope.context.get().to_string(), from, to, "+")?;
                let budget = crate::integration::investigation::RegulationBudget {
                    queries: num(queries)? as usize,
                    events: num(events)? as usize,
                };
                if budget.queries > 3 || budget.events > 96 {
                    return Err("orçamento MP08 excede teto".into());
                }
                let source = fixed_source.take().ok_or("fixe fonte7 antes da pergunta")?;
                let result = crate::integration::investigation::regulate(
                    &mut session,
                    source,
                    q,
                    permissions,
                    budget,
                )?;
                let text = format!("MP08 {result:?}");
                regulation_report = Some(result);
                Ok(text)
            })(),
            [op, ..]
                if regulation_report.is_some()
                    && !matches!(
                        *op,
                        "estado" | "historico" | "ajuda" | "salvar" | "restaurar"
                    ) =>
            {
                Err("supervisão MP08 encerrada; sem repetição ou troca automática".into())
            }
            ["fonte7", id] => (|| -> Result<String, String> {
                if investigation.is_some() || fixed_source.is_some() || source_builder.is_some() {
                    return Err("fonte já definida ou investigação iniciada".into());
                }
                source_builder = Some((source(id)?, vec![]));
                Ok("Fonte em preparação; nenhuma consulta executada".into())
            })(),
            ["evidencia7", ctx, from, to, pol, ev, nat] => (|| -> Result<String, String> {
                if investigation.is_some() || fixed_source.is_some() {
                    return Err("fonte imutável".into());
                }
                let (id, rows) = source_builder.as_mut().ok_or("defina fonte7")?;
                if rows.len() >= 8 {
                    return Err("limite da fonte".into());
                }
                let info = session.info(&[ctx, from, to, pol, &id.get().to_string(), ev, nat])?;
                rows.push(info);
                Ok("Registro explícito preparado antes da pergunta".into())
            })(),
            ["fixar-fonte7"] => (|| -> Result<String, String> {
                if investigation.is_some() || fixed_source.is_some() {
                    return Err("fonte já fixada".into());
                }
                let (id, rows) = source_builder.as_ref().ok_or("defina fonte7")?;
                fixed_source = Some(crate::integration::investigation::LocalSource::new(
                    *id, rows,
                )?);
                source_builder = None;
                Ok("Fonte local fixada e imutável".into())
            })(),
            ["investigar7", from, to] => (|| -> Result<String, String> {
                if investigation.is_some() {
                    return Err("um episódio MP07 por sessão".into());
                }
                let ctx = session.scope.ok_or("sem escopo")?.context.get().to_string();
                let q = session.statement(&ctx, from, to, "+")?;
                let source = fixed_source
                    .take()
                    .ok_or("fixe a fonte antes da pergunta")?;
                let mut r = crate::integration::investigation::Investigation::new(source);
                let result = r.begin(&mut session, q);
                investigation = Some(r);
                result.map(|r| format!("MP07 inicial {r:?}"))
            })(),
            ["permitir-investigacao7"] => investigation
                .as_mut()
                .ok_or_else(|| "sem investigação".into())
                .and_then(|r| r.allow_investigation())
                .map(|_| {
                    "Investigação autorizada; fonte/admissão não autorizadas por implicação".into()
                }),
            ["pedido7"] => investigation
                .as_mut()
                .ok_or_else(|| "sem investigação".into())
                .and_then(|r| r.derive(&mut session))
                .map(|r| format!("Pedido MP07 {r:?}")),
            ["permitir-fonte7"] => investigation
                .as_mut()
                .ok_or_else(|| "sem investigação".into())
                .and_then(|r| r.allow_source(&session))
                .map(|_| "Acesso à fonte autorizado; sem admissão implícita".into()),
            ["consultar-fonte7"] => investigation
                .as_mut()
                .ok_or_else(|| "sem investigação".into())
                .and_then(|r| {
                    r.fetch(&mut session)?;
                    Ok(r.history())
                }),
            ["interpretar7"] => investigation
                .as_mut()
                .ok_or_else(|| "sem investigação".into())
                .and_then(|r| r.interpret(&mut session))
                .map(|_| "Retorno interpretado".into()),
            ["avaliar7"] => investigation
                .as_mut()
                .ok_or_else(|| "sem investigação".into())
                .and_then(|r| {
                    r.assess(&mut session)?;
                    Ok(r.history())
                }),
            ["permitir-admissao7"] => investigation
                .as_mut()
                .ok_or_else(|| "sem investigação".into())
                .and_then(|r| r.allow_admission())
                .map(|_| "Admissão explicitamente autorizada".into()),
            ["admitir7", id] => investigation
                .as_mut()
                .ok_or_else(|| "sem investigação".into())
                .and_then(|r| r.admit(&mut session, record(id)?))
                .map(|_| "Uma observação incorporada explicitamente".into()),
            ["recusar7"] => investigation
                .as_mut()
                .ok_or_else(|| "sem investigação".into())
                .and_then(|r| r.deny_admission())
                .map(|_| "Admissão recusada; memória preservada".into()),
            ["concluir7"] => investigation
                .as_mut()
                .ok_or_else(|| "sem investigação".into())
                .and_then(|r| r.finish(&mut session))
                .map(|r| format!("MP07 final {r:?}")),
            ["historico7"] => investigation
                .as_ref()
                .map(|r| r.history())
                .ok_or_else(|| "sem investigação".into()),
            ["externa7"] => Err("ações externas proibidas em MP07".into()),
            [op, ..]
                if investigation.is_some()
                    && !matches!(
                        *op,
                        "estado" | "historico" | "ajuda" | "salvar" | "restaurar"
                    ) =>
            {
                Err(
                    "durante MP07 use os atos supervisionados; comandos paralelos bloqueados"
                        .into(),
                )
            }
            ["historico3"] => Ok(format!("Histórico MP06 {:?}", path_log.entries())),
            ["autorizar3", ctx] => (|| -> Result<String, String> {
                let scope = session.scope_in(ctx)?;
                if session.authorized.map(|a| a.scope()) != Some(scope) {
                    return Err("escopo sem autorização base".into());
                }
                if path_authorization.is_some() {
                    return Err("autorização MP06 já fixada".into());
                }
                path_authorization =
                    Some(crate::integration::query::PathAuthorization::inclusion_three(scope));
                Ok("Composição de até três premissas autorizada explicitamente; estado MP06 transitório".into())
            })(),
            ["consultar3", ctx, from, to] => path_authorization
                .ok_or_else(|| "sem autorização explícita MP06".to_string())
                .and_then(|a| session.query_three(ctx, from, to, a, &mut path_log))
                .map(|r| format!("MP06 {r:?}")),
            ["salvar", path] => session
                .save(Path::new(path))
                .map(|_| "Checkpoint F1 salvo; criação exclusiva".into()),
            ["restaurar", path] => Session::load(Path::new(path)).map(|s| {
                session = s;
                composed_cli = composed::Adapter::default();
                path_log = crate::integration::query::PathEpisodeLog::default();
                path_authorization = None;
                concept_goal = None;
                concept_goal_report = None;
                teaching_builder = None;
                teaching_plan = None;
                teaching_assessment = None;
                teaching_permissions = None;
                teaching_report = None;
                regulation_permissions = None;
                regulation_report = None;
                investigation = None;
                fixed_source = None;
                source_builder = None;
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

/// Porta MP07: atos M3 diretos, sem reexecutar consultas históricas da sessão.
/// O journal recebe os mesmos comandos F1 v1 somente após sucesso do ato.
impl Session {
    fn mp07_step(&self) -> Result<u64, String> {
        if self.journal.len() >= MAX_OPERATIONS {
            return Err("capacidade de operações excedida".into());
        }
        self.at.checked_add(1).ok_or("tempo excedido".into())
    }
    fn mp07_record(&mut self, command: String, at: u64) {
        debug_assert!(command.len() <= MAX_LINE_BYTES);
        self.at = at;
        self.journal.push(command);
    }
}
impl crate::integration::investigation::AcquisitionHost for Session {
    fn regulation_paths(
        &self,
        query: Statement,
        authorization: crate::integration::query::PathAuthorization,
        budget: usize,
    ) -> Result<crate::integration::query::PathQueryReport, String> {
        QueryCoordinator::new(
            self.memory(),
            self.metadata(),
            self.authorized.ok_or("regra não autorizada")?,
            &self.sources,
        )
        .map_err(err)?
        .query_three(
            EpisodeId::new(3).map_err(err)?,
            query,
            self.at.checked_add(1).ok_or("tempo excedido")?,
            authorization,
            budget,
            &mut crate::integration::query::PathEpisodeLog::default(),
        )
        .map_err(err)
    }

    fn start(&mut self, q: Statement) -> Result<crate::integration::query::QueryReport, String> {
        if Some(q.scope()) != self.scope || q.polarity() != Polarity::Positive {
            return Err("consulta MP07 fora do escopo".into());
        }
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
        let at = self.mp07_step()?;
        let mut memory = EngramStore::new(8).map_err(err)?;
        for r in self.memory.iter() {
            memory.record(r).map_err(err)?;
        }
        let mut core = CognitiveEpisode::new(
            memory,
            &self.metadata,
            &self.symbols,
            self.authorized.ok_or("regra sem autorização explícita")?,
            &self.sources,
        )
        .map_err(err)?;
        let report = core
            .evaluate_initial(q, EpisodeId::new(1).map_err(err)?, at)
            .map_err(err)?
            .clone();
        self.acquisition = Some(core);
        self.mp07_record(
            format!("iniciar {} {}", q.subject().get(), q.object().get()),
            at,
        );
        Ok(report)
    }
    fn request(&mut self) -> Result<InformationRequest, String> {
        let at = self.mp07_step()?;
        let r = self
            .acquisition
            .as_mut()
            .ok_or("sem episódio")?
            .derive_request(RequestId::new(1).map_err(err)?, at)
            .map_err(err)?;
        self.mp07_record("pedido".into(), at);
        Ok(r)
    }
    fn receive(&mut self, p: FeedbackPackage) -> Result<(), String> {
        let request = self
            .acquisition
            .as_ref()
            .ok_or("sem episódio")?
            .request()
            .ok_or("sem pedido")?;
        p.check_link(request).map_err(err)?;
        let FeedbackContent::One(i) = p.content else {
            return Err("MP07 aceita uma candidata".into());
        };
        let polarity = if i.statement.polarity() == Polarity::Positive {
            "+"
        } else {
            "-"
        };
        let s = self.statement(
            &i.statement.scope().context.get().to_string(),
            &i.statement.subject().get().to_string(),
            &i.statement.object().get().to_string(),
            polarity,
        )?;
        if s != i.statement {
            return Err("IncompatibleScope: retorno não representável em F1 v1".into());
        }
        if p.envelope_source != i.declared_source {
            return Err("fonte do envelope divergente".into());
        }
        let at = self.mp07_step()?;
        if i.received_at != at || p.returned_at != at {
            return Err("instante de retorno incompatível".into());
        }
        let source = i.declared_source.ok_or("fonte ausente")?;
        let evidence = i
            .evidence
            .map_or_else(|| "-".into(), |e| e.get().to_string());
        let command = format!(
            "retorno {} {} {} {} {} {} {} {} {}",
            p.id.get(),
            p.request.get(),
            i.statement.scope().context.get(),
            i.statement.subject().get(),
            i.statement.object().get(),
            polarity,
            source.get(),
            evidence,
            if i.nature == DeclaredNature::Observation {
                "observacao"
            } else {
                "hipotese"
            }
        );
        self.acquisition
            .as_mut()
            .unwrap()
            .receive(p, at)
            .map_err(err)?;
        self.mp07_record(command, at);
        Ok(())
    }
    fn interpret(&mut self) -> Result<(), String> {
        let at = self.mp07_step()?;
        self.acquisition
            .as_mut()
            .ok_or("sem episódio")?
            .interpret(at)
            .map_err(err)?;
        self.mp07_record("interpretar-retorno".into(), at);
        Ok(())
    }
    fn assess(&mut self) -> Result<Vec<crate::integration::acquisition::AssessedItem>, String> {
        let at = self.mp07_step()?;
        let result = self
            .acquisition
            .as_mut()
            .ok_or("sem episódio")?
            .assess_admissibility(at)
            .map_err(err)?
            .to_vec();
        self.mp07_record("avaliar-retorno".into(), at);
        Ok(result)
    }
    fn incorporate(&mut self, id: EngramId) -> Result<(), String> {
        let at = self.mp07_step()?;
        self.acquisition
            .as_mut()
            .ok_or("sem episódio")?
            .incorporate(&[id], at)
            .map_err(err)?;
        self.mp07_record(format!("admitir-retorno {}", id.get()), at);
        Ok(())
    }
    fn reassess(&mut self) -> Result<crate::integration::query::QueryReport, String> {
        let at = self.mp07_step()?;
        let r = self
            .acquisition
            .as_mut()
            .ok_or("sem episódio")?
            .reassess(EpisodeId::new(2).map_err(err)?, at)
            .map_err(err)?
            .clone();
        self.mp07_record("reavaliar".into(), at);
        Ok(r)
    }
    fn query_readonly(
        &self,
        q: Statement,
    ) -> Result<crate::integration::query::QueryReport, String> {
        QueryCoordinator::new(
            self.memory(),
            self.metadata(),
            self.authorized.ok_or("regra não autorizada")?,
            &self.sources,
        )
        .map_err(err)?
        .query(
            EpisodeId::new(2).map_err(err)?,
            q,
            self.at.checked_add(1).ok_or("tempo excedido")?,
            &mut EpisodeLog::default(),
        )
        .map_err(err)
    }
    fn next_at(&self) -> Result<u64, String> {
        self.at.checked_add(1).ok_or("tempo excedido".into())
    }
    fn knows_source(&self, id: SourceId) -> bool {
        self.sources.contains(&id)
    }
    fn opposition(
        &self,
        q: Statement,
        i: Information,
    ) -> Vec<crate::integration::query::BoundRecord> {
        self.metadata()
            .iter()
            .filter_map(|m| {
                let r = self.memory().get(m.id).ok()?;
                let s = m.information.statement;
                let c = i.statement;
                if m.state != EpistemicState::AdmittedObservation
                    || m.disposition.is_some()
                    || r.status() != RecordStatus::Observed
                    || r.observed_at() > self.at
                    || check_observation(m.information, q.scope(), &self.sources, self.at).is_err()
                {
                    return None;
                }
                if (s.subject() == c.subject()
                    && s.object() == c.object()
                    && s.polarity() != c.polarity())
                    || (s.subject() == q.subject()
                        && s.object() == q.object()
                        && s.polarity() == Polarity::Negative)
                {
                    Some(crate::integration::query::BoundRecord {
                        engram: r,
                        metadata: *m,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}
impl crate::integration::investigation::sealed::Sealed for Session {}

impl crate::curriculum::pedagogy_sealed::Sealed for Session {}
impl crate::curriculum::PedagogyHost for Session {
    fn pedagogy_events(&self) -> usize {
        self.concepts.as_ref().map_or(0, |c| c.trace().len())
    }

    fn pedagogy_existing(&self) -> Result<Vec<crate::cognition::concepts::Attributes>, String> {
        Ok(self
            .concepts
            .as_ref()
            .ok_or("sem conceito")?
            .examples()
            .map_err(err)?
            .iter()
            .map(|e| e.example.attributes)
            .collect())
    }

    fn pedagogy_identity(&self) -> Result<(NodeId, Scope), String> {
        let c = self.concepts.as_ref().ok_or("sem conceito")?;
        let scope = self.scope.ok_or("sem escopo")?;
        if c.models().len() != 1 || c.models()[0].scope() != scope {
            return Err("MP09 requer um contexto".into());
        }
        Ok((c.concept().id(), scope))
    }
    fn pedagogy_version(&self) -> Result<crate::learning::concepts::HypothesisVersion, String> {
        Ok(self
            .concepts
            .as_ref()
            .ok_or("sem conceito")?
            .models()
            .first()
            .ok_or("sem modelo")?
            .current()
            .clone())
    }
    fn pedagogy_query(
        &self,
        episode: EpisodeId,
        observation: Observation,
    ) -> Result<crate::integration::concepts::ConceptReport, String> {
        self.concepts
            .as_ref()
            .ok_or("sem conceito")?
            .query(
                episode,
                self.scope.ok_or("sem escopo")?,
                observation,
                self.at,
                &mut crate::integration::concepts::ConceptEpisodeLog::default(),
            )
            .map_err(err)
    }
    fn pedagogy_capacity(&self, lessons: usize) -> Result<(), String> {
        let c = self.concepts.as_ref().ok_or("sem conceito")?;
        let scope = self.scope.ok_or("sem escopo")?;
        let steps = lessons.checked_mul(5).ok_or("orçamento inválido")?;
        let end = self.at.checked_add(steps as u64).ok_or("tempo excedido")?;
        if self.journal.len() + steps > MAX_OPERATIONS
            || c.trace().len() + steps > 128
            || c.memory().len() + lessons > 16
            || c.models()[0].versions().len() + lessons > 32
            || !scope.validity.contains(self.at)
            || !scope.validity.contains(end)
        {
            return Err(
                "reserva conservadora de operações/eventos/estado/validade MP09 indisponível"
                    .into(),
            );
        }
        Ok(())
    }
    fn pedagogy_present(
        &mut self,
        lesson: crate::curriculum::Lesson,
    ) -> Result<crate::integration::concepts::Admission, String> {
        let scope = self.scope.ok_or("sem escopo")?;
        let at = self.mp07_step()?;
        let concept = self.concepts.as_ref().ok_or("sem conceito")?.concept().id();
        let vector: String = lesson
            .attributes
            .values()
            .iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect();
        let raw = IncomingExample {
            receipt: ReceiptId::new(lesson.receipt).map_err(err)?,
            teacher_episode: EpisodeId::new(lesson.receipt).map_err(err)?,
            concept,
            exemplar: lesson.exemplar,
            attributes: lesson.attributes.values().map(|b| Some(u8::from(b))),
            label: lesson.label,
            scope,
            source: Some(lesson.source),
            evidence: Some(lesson.evidence),
            nature: DeclaredNature::Observation,
            observed_at: at,
            sent_at: at,
        };
        self.concepts
            .as_mut()
            .unwrap()
            .receive(raw, at)
            .map_err(err)?;
        self.mp07_record(
            format!(
                "exemplo {} {} {} {} {} {} {} observacao",
                lesson.receipt,
                lesson.exemplar.get(),
                scope.context.get(),
                vector,
                if lesson.label == Label::Positive {
                    "+"
                } else {
                    "-"
                },
                lesson.source.get(),
                lesson.evidence.get()
            ),
            at,
        );
        let at = self.mp07_step()?;
        self.concepts.as_mut().unwrap().interpret(at).map_err(err)?;
        self.mp07_record("interpretar-exemplo".into(), at);
        let at = self.mp07_step()?;
        let admission = self
            .concepts
            .as_mut()
            .unwrap()
            .assess_admissibility(at)
            .map_err(err)?;
        self.mp07_record("avaliar-exemplo".into(), at);
        Ok(admission)
    }
    fn pedagogy_admit(
        &mut self,
        id: EngramId,
    ) -> Result<crate::learning::concepts::HypothesisVersion, String> {
        // Incorporação já reserva evento/versão de revisão no núcleo; reserva também journal/tempo.
        if self.journal.len() + 2 > MAX_OPERATIONS
            || !self
                .scope
                .ok_or("sem escopo")?
                .validity
                .contains(self.at.checked_add(2).ok_or("tempo excedido")?)
        {
            return Err("admissão/revisão sem reserva".into());
        }
        let at = self.mp07_step()?;
        self.concepts
            .as_mut()
            .ok_or("sem conceito")?
            .incorporate(id, at)
            .map_err(err)?;
        self.mp07_record(format!("admitir-exemplo {}", id.get()), at);
        let at = self.mp07_step()?;
        let version = self
            .concepts
            .as_mut()
            .unwrap()
            .revise(at)
            .map_err(err)?
            .clone();
        self.mp07_record("revisar".into(), at);
        Ok(version)
    }
}
