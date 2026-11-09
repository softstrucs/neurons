//! Controle transitório MP07 sobre portas do coordenador existente.
//! A fonte recebe apenas o pedido; tabela privada é fixada antes da consulta inicial.
use super::acquisition::{AdmissionVerdict, AssessedItem};
use super::query::{BoundRecord, QueryReport};
use crate::cognitive_contracts::*;
use crate::engram::EngramId;

pub(crate) mod sealed {
    pub trait Sealed {}
}
/// Porta selada: somente adaptadores internos que preservam M1/M2 podem implementá-la.
pub trait AcquisitionHost: sealed::Sealed {
    fn start(&mut self, query: Statement) -> Result<QueryReport, String>;
    fn request(&mut self) -> Result<InformationRequest, String>;
    fn receive(&mut self, package: FeedbackPackage) -> Result<(), String>;
    fn interpret(&mut self) -> Result<(), String>;
    fn assess(&mut self) -> Result<Vec<AssessedItem>, String>;
    fn incorporate(&mut self, id: EngramId) -> Result<(), String>;
    fn reassess(&mut self) -> Result<QueryReport, String>;
    fn query_readonly(&self, query: Statement) -> Result<QueryReport, String>;
    fn next_at(&self) -> Result<u64, String>;
    fn knows_source(&self, id: SourceId) -> bool;
    fn opposition(&self, query: Statement, item: Information) -> Vec<BoundRecord>;
}
/// Não implementa Debug nem expõe catálogo/iterador. IDs não autenticam a fonte.
pub struct LocalSource {
    id: SourceId,
    rows: Vec<Information>,
    calls: usize,
}
impl LocalSource {
    pub fn new(id: SourceId, rows: &[Information]) -> Result<Self, String> {
        if rows.len() > 8 {
            return Err("fonte excede oito registros".into());
        }
        Ok(Self {
            id,
            rows: rows.to_vec(),
            calls: 0,
        })
    }
    fn lookup(&mut self, request: InformationRequest) -> (Option<Information>, bool) {
        self.calls += 1;
        let exact: Vec<_> = self
            .rows
            .iter()
            .filter(|i| {
                i.statement.subject() == request.missing.subject()
                    && i.statement.object() == request.missing.object()
            })
            .copied()
            .collect();
        if exact.len() == 1 {
            return (exact.first().copied(), false);
        }
        if exact.len() > 1 {
            return (None, true);
        }
        // Um retorno sobre o sujeito pode ser irrelevante: admissibilidade é separada.
        let same: Vec<_> = self
            .rows
            .iter()
            .filter(|i| i.statement.subject() == request.missing.subject())
            .copied()
            .collect();
        (
            if same.len() == 1 {
                same.first().copied()
            } else {
                None
            },
            same.len() > 1,
        )
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage {
    Ready,
    Initial,
    Requested,
    Received,
    Interpreted,
    Assessed,
    NoResult,
    Admitted,
    Finished,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceResult {
    HostRefused(FeedbackPackage, String),
    Delivered(FeedbackPackage),
    Unavailable,
    Ambiguous,
    Rejected(Information, ContractError),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceCall {
    pub source: SourceId,
    pub request: InformationRequest,
    pub at: u64,
    pub original: Option<Information>,
    pub result: SourceResult,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Permissions {
    pub investigation: Option<Scope>,
    pub source: Option<(Scope, SourceId)>,
    pub admission: Option<Scope>,
}
/// Só esta interface pode acionar a fonte, depois dos dois controles independentes.
/// Não contém algoritmo de lacuna, inferência ou admissão: todos são delegados.
pub struct Investigation {
    source: LocalSource,
    stage: Stage,
    query: Option<Statement>,
    permissions: Permissions,
    request: Option<InformationRequest>,
    call: Option<SourceCall>,
    assessed: Vec<AssessedItem>,
    opposition: Vec<BoundRecord>,
    reports: Vec<QueryReport>,
    denied: bool,
    request_error: Option<String>,
}
impl Investigation {
    pub fn new(source: LocalSource) -> Self {
        Self {
            source,
            stage: Stage::Ready,
            query: None,
            permissions: Permissions::default(),
            request: None,
            call: None,
            assessed: vec![],
            opposition: vec![],
            reports: vec![],
            denied: false,
            request_error: None,
        }
    }
    pub fn stage(&self) -> Stage {
        self.stage
    }
    pub fn source_calls(&self) -> usize {
        self.source.calls
    }
    pub fn request_value(&self) -> Option<InformationRequest> {
        self.request
    }
    pub fn receipt(&self) -> Option<&SourceCall> {
        self.call.as_ref()
    }
    pub fn processed_events(&self) -> usize {
        self.reports
            .iter()
            .filter_map(|r| r.review.as_ref())
            .flat_map(|r| r.attempts.iter())
            .filter_map(|a| a.inference)
            .map(|r| r.processed_events)
            .sum()
    }
    fn record_report(&mut self, r: QueryReport) -> Result<(), String> {
        let events: usize = r.review.as_ref().map_or(0, |r| {
            r.attempts
                .iter()
                .filter_map(|a| a.inference)
                .map(|r| r.processed_events)
                .sum()
        });
        if self.reports.len() >= 3 || events > 32 || self.processed_events() + events > 96 {
            return Err("orçamento de decisões/eventos MP07".into());
        }
        self.reports.push(r);
        Ok(())
    }
    pub fn reports(&self) -> &[QueryReport] {
        &self.reports
    }
    pub fn permissions(&self) -> &Permissions {
        &self.permissions
    }
    pub fn assessments(&self) -> &[AssessedItem] {
        &self.assessed
    }
    pub fn opposition_records(&self) -> &[BoundRecord] {
        &self.opposition
    }
    pub fn history(&self) -> String {
        format!("stage={:?} permissions={:?} external=prohibited request={:?} request_error={:?} source_calls={} events={} call={:?} assessed={:?} opposition={:?} denied={} reports={:?}",self.stage,self.permissions,self.request,self.request_error,self.source.calls,self.processed_events(),self.call,self.assessed,self.opposition,self.denied,self.reports)
    }
    fn expect(&self, stage: Stage) -> Result<(), String> {
        if self.stage == stage {
            Ok(())
        } else {
            Err(format!("fase MP07 inválida: {:?}", self.stage))
        }
    }
    pub fn begin(
        &mut self,
        host: &mut impl AcquisitionHost,
        query: Statement,
    ) -> Result<QueryReport, String> {
        self.expect(Stage::Ready)?;
        let report = host.start(query)?;
        self.query = Some(query);
        self.stage = if report.episode.decision
            == EpisodeDecision::Abstain(Refusal::Monitor(
                crate::metacognition::AbstainReason::MissingEvidence,
            )) {
            Stage::Initial
        } else {
            Stage::Finished
        };
        self.record_report(report.clone())?;
        Ok(report)
    }
    pub fn allow_investigation(&mut self) -> Result<(), String> {
        self.expect(Stage::Initial)?;
        if self.permissions.investigation.is_some() {
            return Err("autorização já declarada".into());
        }
        self.permissions.investigation = Some(self.query.unwrap().scope());
        Ok(())
    }
    pub fn derive(
        &mut self,
        host: &mut impl AcquisitionHost,
    ) -> Result<InformationRequest, String> {
        self.expect(Stage::Initial)?;
        if self.permissions.investigation != self.query.map(|q| q.scope()) {
            return Err("investigação não autorizada".into());
        }
        let request = match host.request() {
            Ok(r) => r,
            Err(e) => {
                self.request_error = Some(e.clone());
                return Err(e);
            }
        };
        self.request = Some(request);
        self.stage = Stage::Requested;
        Ok(request)
    }
    pub fn allow_source(&mut self, host: &impl AcquisitionHost) -> Result<(), String> {
        self.expect(Stage::Requested)?;
        if !host.knows_source(self.source.id) {
            return Err("fonte não reconhecida pelo host".into());
        }
        if self.permissions.source.is_some() {
            return Err("autorização já declarada".into());
        }
        self.permissions.source = Some((self.query.unwrap().scope(), self.source.id));
        Ok(())
    }
    pub fn external_action(&self) -> Result<(), String> {
        Err("ações externas proibidas em MP07".into())
    }
    pub fn fetch(&mut self, host: &mut impl AcquisitionHost) -> Result<(), String> {
        self.expect(Stage::Requested)?;
        let q = self.query.unwrap();
        if self.permissions.investigation != Some(q.scope())
            || self.permissions.source != Some((q.scope(), self.source.id))
        {
            return Err("consulta à fonte não autorizada".into());
        }
        if self.source.calls != 0 || self.call.is_some() {
            return Err("orçamento de uma consulta esgotado".into());
        }
        let request = self.request.unwrap();
        let at = host.next_at()?;
        if at <= request.requested_at || !q.scope().validity.contains(at) {
            return Err("instante fora de validade".into());
        }
        let (candidate, ambiguous) = self.source.lookup(request);
        let original = candidate;
        let result = match candidate {
            None => {
                self.stage = Stage::NoResult;
                if ambiguous {
                    SourceResult::Ambiguous
                } else {
                    SourceResult::Unavailable
                }
            }
            Some(mut info) => {
                // Valida também a proveniência temporal original, antes de renovar só a recepção.
                if info.received_at > at
                    || !info.statement.scope().validity.contains(info.received_at)
                    || !info.statement.scope().validity.contains(at)
                {
                    self.stage = Stage::NoResult;
                    SourceResult::Rejected(info, ContractError::OutsideValidity)
                } else if info.declared_source != Some(self.source.id) {
                    self.stage = Stage::NoResult;
                    SourceResult::Rejected(info, ContractError::UnknownSource)
                } else {
                    // Apenas o instante de transporte é novo; afirmação/evidência permanecem originais.
                    info.received_at = at;
                    let p = FeedbackPackage {
                        id: FeedbackId::new(1).unwrap(),
                        request: request.id,
                        episode: request.episode,
                        envelope_source: Some(self.source.id),
                        returned_at: at,
                        content: FeedbackContent::One(info),
                    };
                    // A oportunidade permanece consumida mesmo se o host recusar.
                    self.call = Some(SourceCall {
                        source: self.source.id,
                        request,
                        at,
                        original,
                        result: SourceResult::Delivered(p),
                    });
                    self.stage = Stage::NoResult;
                    if let Err(error) = host.receive(p) {
                        self.call = Some(SourceCall {
                            source: self.source.id,
                            request,
                            at,
                            original,
                            result: SourceResult::HostRefused(p, error.clone()),
                        });
                        return Err(error);
                    }
                    self.stage = Stage::Received;
                    SourceResult::Delivered(p)
                }
            }
        };
        self.call = Some(SourceCall {
            source: self.source.id,
            request,
            at,
            original,
            result,
        });
        Ok(())
    }
    pub fn interpret(&mut self, host: &mut impl AcquisitionHost) -> Result<(), String> {
        self.expect(Stage::Received)?;
        host.interpret()?;
        self.stage = Stage::Interpreted;
        Ok(())
    }
    pub fn assess(&mut self, host: &mut impl AcquisitionHost) -> Result<(), String> {
        self.expect(Stage::Interpreted)?;
        self.assessed = host.assess()?;
        if self.assessed.len() > 1 {
            return Err("mais de uma candidata".into());
        }
        if let Some(i) = self.assessed.first() {
            self.opposition = host.opposition(self.query.unwrap(), i.information);
        }
        self.stage = Stage::Assessed;
        Ok(())
    }
    pub fn allow_admission(&mut self) -> Result<(), String> {
        self.expect(Stage::Assessed)?;
        if self.denied || self.permissions.admission.is_some() {
            return Err("admissão recusada ou já autorizada".into());
        }
        self.permissions.admission = Some(self.query.unwrap().scope());
        Ok(())
    }
    pub fn deny_admission(&mut self) -> Result<(), String> {
        self.expect(Stage::Assessed)?;
        self.denied = true;
        Ok(())
    }
    pub fn admit(&mut self, host: &mut impl AcquisitionHost, id: EngramId) -> Result<(), String> {
        self.expect(Stage::Assessed)?;
        if self.denied || self.permissions.admission != self.query.map(|q| q.scope()) {
            return Err("incorporação não autorizada".into());
        }
        if self.assessed.len() != 1 || self.assessed[0].verdict != AdmissionVerdict::Admissible {
            return Err("candidata não admissível".into());
        }
        host.incorporate(id)?;
        self.stage = Stage::Admitted;
        Ok(())
    }
    pub fn finish(&mut self, host: &mut impl AcquisitionHost) -> Result<QueryReport, String> {
        if !matches!(
            self.stage,
            Stage::Initial | Stage::Requested | Stage::NoResult | Stage::Assessed | Stage::Admitted
        ) {
            return Err("fase não finalizável".into());
        }
        if self.reports.len() >= 3 {
            return Err("limite de decisões".into());
        }
        let r = if self.stage == Stage::Admitted {
            host.reassess()?
        } else {
            host.query_readonly(self.query.unwrap())?
        };
        self.record_report(r.clone())?;
        self.stage = Stage::Finished;
        Ok(r)
    }
}
