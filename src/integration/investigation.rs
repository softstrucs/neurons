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
    fn regulation_paths(
        &self,
        query: Statement,
        authorization: super::query::PathAuthorization,
        budget: usize,
    ) -> Result<super::query::PathQueryReport, String>;
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

/// MP08: permissões recebidas do chamador; nenhuma delas é inferida de outra.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RegulationPermissions {
    pub evaluation: Option<Scope>,
    pub mp06: Option<super::query::PathAuthorization>,
    pub investigation: Option<Scope>,
    pub source: Option<(Scope, SourceId)>,
    pub admission: Option<(Scope, EngramId)>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RegulationBudget {
    pub queries: usize,
    pub events: usize,
}
impl Default for RegulationBudget {
    fn default() -> Self {
        Self {
            queries: 3,
            events: 96,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegulationEvidence {
    Query(QueryReport),
    Path(super::query::PathQueryReport),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegulationReport {
    pub query: Statement,
    pub permissions: RegulationPermissions,
    pub budget: RegulationBudget,
    pub investigation_history: Option<String>,
    pub diagnostics: Vec<crate::metacognition::RegulationDiagnostic>,
    pub strategy: crate::metacognition::Strategy,
    pub goal: crate::metacognition::GoalResult,
    pub queries: usize,
    pub events: usize,
    pub evaluations: usize,
    pub source_calls: usize,
    pub admitted: bool,
    /// Reserva conservadora indisponível; distinto de limite atingido pelo motor.
    pub reservation_blocked: bool,
    pub evidence: Vec<RegulationEvidence>,
    pub history: Vec<String>,
}
impl RegulationReport {
    fn diagnostic(&mut self, d: crate::metacognition::RegulationDiagnostic) {
        if !self.diagnostics.contains(&d) {
            self.diagnostics.push(d);
        }
    }
    fn observe(&mut self, proof: bool, refusal: Option<Refusal>) {
        use crate::metacognition::{AbstainReason as A, RegulationDiagnostic as D};
        if proof {
            self.diagnostic(D::Supported);
        }
        match refusal {
            None => {
                if !proof {
                    self.diagnostic(D::InvalidEvidence);
                }
            }
            Some(Refusal::Monitor(A::MissingEvidence)) => self.diagnostic(D::MissingEvidence),
            Some(Refusal::Monitor(A::ContradictoryEvidence)) => self.diagnostic(D::Opposition),
            Some(Refusal::Monitor(A::ResourceLimited | A::ReviewBudget)) => {
                self.diagnostic(D::ResourceLimited)
            }
            Some(Refusal::Monitor(A::InvalidEvidence)) => self.diagnostic(D::InvalidEvidence),
            Some(Refusal::Monitor(A::PropagationFailure)) => self.diagnostic(D::PropagationFailure),
            Some(_) => self.diagnostic(D::ContractBlocked),
        }
    }
    fn query(&mut self, r: QueryReport) -> bool {
        let proof = r.proof.is_some() && r.proof_error.is_none();
        let refusal = match r.episode.decision {
            EpisodeDecision::Abstain(r) => Some(r),
            _ => None,
        };
        self.observe(proof, refusal);
        if r.proof_error.is_some() {
            self.diagnostic(crate::metacognition::RegulationDiagnostic::InvalidEvidence);
        }
        if let Some(review) = &r.review {
            for a in &review.attempts {
                use crate::metacognition::{Assessment as A, RegulationDiagnostic as D};
                if a.conflict.is_some() {
                    self.diagnostic(D::Opposition);
                }
                match a.assessment {
                    A::InvalidEvidence => self.diagnostic(D::InvalidEvidence),
                    A::ResourceLimited => self.diagnostic(D::ResourceLimited),
                    A::PropagationFailure => self.diagnostic(D::PropagationFailure),
                    A::ContradictoryEvidence => self.diagnostic(D::Opposition),
                    _ => {}
                }
            }
        }
        self.events += r.review.as_ref().map_or(0, |v| {
            v.attempts
                .iter()
                .filter_map(|a| a.inference)
                .map(|i| i.processed_events)
                .sum()
        });
        self.queries += 1;
        self.evidence.push(RegulationEvidence::Query(r));
        proof
    }
    fn room(&self, b: RegulationBudget) -> bool {
        // Reserva conservadora antes de executar a consulta M3 de teto fixo32.
        self.queries < b.queries && self.events + 32 <= b.events
    }
}
/// Episódio único, sem retomada, repetição automática ou concessão de permissões.
/// A fonte é recebida já fixada antes da pergunta. Só relatórios reais são julgados.
pub fn regulate(
    host: &mut impl AcquisitionHost,
    source: LocalSource,
    query: Statement,
    permissions: RegulationPermissions,
    budget: RegulationBudget,
) -> Result<RegulationReport, String> {
    use crate::metacognition::{select_strategy, GoalResult, RegulationDiagnostic as D, Strategy};
    if budget.queries > 3 || budget.events > 96 {
        return Err("orçamento MP08 excede o teto".into());
    }
    let mut out = RegulationReport {
        query,
        permissions,
        budget,
        investigation_history: None,
        diagnostics: vec![],
        strategy: Strategy::Abstain,
        goal: GoalResult::NotMet,
        queries: 0,
        events: 0,
        evaluations: 0,
        source_calls: 0,
        admitted: false,
        reservation_blocked: false,
        evidence: vec![],
        history: vec!["Permissões recebidas; ações externas proibidas".into()],
    };
    if permissions.evaluation != Some(query.scope()) {
        out.diagnostic(D::PermissionMissing);
        out.diagnostic(D::ContractBlocked);
        out.history
            .push("Avaliação negada; nenhum método cognitivo executado".into());
        return Ok(out);
    }
    if !out.room(budget) {
        out.reservation_blocked = true;
        out.diagnostic(D::ResourceLimited);
        out.strategy = Strategy::StopAtLimit;
        out.history
            .push("Reserva mínima de32 eventos/consulta indisponível".into());
        return Ok(out);
    }
    let mut investigation = Investigation::new(source);
    let first = match investigation.begin(host, query) {
        Ok(r) => r,
        Err(e) => {
            out.diagnostic(D::ContractBlocked);
            out.history.push(format!("Avaliação recusada: {e}"));
            return Ok(out);
        }
    };
    let mut supported = out.query(first);
    // MP06 é uma capacidade separadamente autorizada; não pode ser inferida da avaliação.
    if !supported
        && !out.diagnostics.iter().any(|d| {
            matches!(
                d,
                D::ContractBlocked
                    | D::InvalidEvidence
                    | D::Opposition
                    | D::PropagationFailure
                    | D::ResourceLimited
            )
        })
    {
        if let Some(auth) = permissions.mp06 {
            if out.room(budget) {
                let cap = (budget.events - out.events).min(64);
                match host.regulation_paths(query, auth, cap) {
                    Ok(r) => {
                        supported = r.proof.is_some() && r.refusal.is_none();
                        out.observe(supported, r.refusal);
                        if r.review.as_ref().is_some_and(|v| !v.conflicts.is_empty()) {
                            out.diagnostic(D::Opposition);
                        }
                        out.events += r
                            .review
                            .as_ref()
                            .map_or(0, |r| r.inference.processed_events);
                        out.queries += 1;
                        out.evidence.push(RegulationEvidence::Path(r));
                    }
                    Err(e) => {
                        out.diagnostic(D::ContractBlocked);
                        out.history.push(format!("MP06 recusado: {e}"));
                    }
                }
            } else {
                out.reservation_blocked = true;
                out.diagnostic(D::ResourceLimited);
            }
        }
    }
    out.evaluations = 1;
    out.history
        .push(format!("Diagnóstico inicial {:?}", out.diagnostics));
    let permits = permissions.investigation == Some(query.scope())
        && permissions.source == Some((query.scope(), investigation.source.id))
        && host.knows_source(investigation.source.id);
    let mut unique = false;
    if !supported && select_strategy(&out.diagnostics, false, true, true) == Strategy::Investigate {
        if permits {
            // Copia apenas a concessão explícita já recebida; não cria autoridade.
            investigation.allow_investigation()?;
            match investigation.derive(host) {
                Ok(_) => unique = true,
                Err(e) => {
                    out.diagnostic(D::GapUnavailable);
                    out.history.push(format!("Lacuna não derivável: {e}"));
                }
            }
        } else {
            out.diagnostic(D::PermissionMissing);
        }
    }
    if unique && !out.room(budget) {
        out.reservation_blocked = true;
        out.diagnostic(D::ResourceLimited);
    }
    out.strategy = select_strategy(&out.diagnostics, supported, unique, permits);
    out.history.push(format!(
        "Seleção única {:?}; sem troca automática",
        out.strategy
    ));
    if out.strategy == Strategy::Respond {
        out.goal = GoalResult::Met;
    }
    if out.strategy == Strategy::Investigate {
        let execution = (|| -> Result<(), String> {
            investigation.allow_source(host)?;
            investigation.fetch(host)?;
            if investigation.stage() == Stage::Received {
                investigation.interpret(host)?;
                investigation.assess(host)?;
                if investigation
                    .assessments()
                    .iter()
                    .any(|a| a.verdict != AdmissionVerdict::Admissible)
                {
                    out.diagnostic(D::InvalidEvidence);
                }
                if !investigation.opposition_records().is_empty() {
                    out.diagnostic(D::Opposition);
                }
                if let Some((scope, id)) = permissions.admission {
                    if scope == query.scope()
                        && investigation
                            .assessments()
                            .first()
                            .is_some_and(|a| a.verdict == AdmissionVerdict::Admissible)
                    {
                        investigation.allow_admission()?;
                        investigation.admit(host, id)?;
                        out.admitted = true;
                    }
                }
            }
            Ok(())
        })();
        out.source_calls = investigation.source_calls();
        if matches!(
            investigation.receipt().map(|r| &r.result),
            Some(SourceResult::Rejected(_, _))
        ) {
            out.diagnostic(D::InvalidEvidence);
        }
        if matches!(
            investigation.receipt().map(|r| &r.result),
            Some(SourceResult::HostRefused(_, _))
        ) {
            out.diagnostic(D::ContractBlocked);
        }
        out.history.push(format!(
            "Execução MP07: {:?}; chamadas={}; admissão={}",
            execution, out.source_calls, out.admitted
        ));
        // Consulta final somente nas fases seguras, nunca admissão vazia.
        if execution.is_ok() || investigation.stage() == Stage::NoResult {
            match investigation.finish(host) {
                Ok(r) => {
                    let final_supported = out.query(r);
                    out.evaluations += 1;
                    out.goal = if final_supported
                        && !out.diagnostics.iter().any(|d| {
                            matches!(
                                d,
                                D::Opposition
                                    | D::InvalidEvidence
                                    | D::ContractBlocked
                                    | D::PropagationFailure
                                    | D::ResourceLimited
                            )
                        }) {
                        GoalResult::Met
                    } else if out.source_calls == 1 && execution.is_ok() {
                        GoalResult::Partial
                    } else {
                        GoalResult::NotMet
                    };
                }
                Err(e) => {
                    out.diagnostic(D::ContractBlocked);
                    out.history.push(format!("Reavaliação recusada: {e}"));
                }
            }
        } else {
            out.diagnostic(D::ContractBlocked);
        }
    }
    out.investigation_history = Some(investigation.history());
    out.history.push(format!(
        "Objetivo {:?}; eventos={}; consultas={}",
        out.goal, out.events, out.queries
    ));
    if out.events > budget.events
        || out.queries > budget.queries
        || out.evaluations > 2
        || out.history.len() > 8
    {
        return Err("violação de orçamento supervisor; episódio interrompido".into());
    }
    Ok(out)
}
