//! M3: aquisição simbólica assistida em etapas explícitas, sem professor/oráculo interno.
use super::query::{
    BoundRecord, QueryCoordinator, QueryError, QueryReport, RecordMetadata, RuleAuthorization,
};
use crate::cognitive_contracts::{
    check_observation, Concept, ContractError, Disposition, EpisodeDecision, EpisodeId, EpisodeLog,
    EpistemicState, ExchangeLog, FeedbackContent, FeedbackPackage, Information, InformationRequest,
    Polarity, Refusal, Rejection, RequestId, SourceId, Statement, MAX_RECORDS,
};
use crate::engram::{Cue, Engram, EngramError, EngramId, EngramStore, RecordStatus};
use crate::metacognition::{AbstainReason, EvidenceRefs};

pub const MAX_CONCEPTS: usize = 5;
pub const MAX_TRACE: usize = 8;
/// Política sintética declarada: observações identificadas, escopadas e com referência.
/// Não representa autenticação, confirmação de verdade ou validação científica.
pub const ADMISSION_POLICY: &str = "declared-observation/requested-subject/v2";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcquisitionError {
    WrongPhase,
    NoUniqueGap,
    Contract(ContractError),
    Query(QueryError),
    Memory(EngramError),
}
impl From<ContractError> for AcquisitionError {
    fn from(value: ContractError) -> Self {
        Self::Contract(value)
    }
}
impl From<QueryError> for AcquisitionError {
    fn from(value: QueryError) -> Self {
        Self::Query(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase {
    Ready,
    Initial,
    Requested,
    Received,
    Interpreted,
    Assessed,
    Incorporated,
    Reassessed,
    Complete,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmissionVerdict {
    Admissible,
    Quarantined(ContractError),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssessedItem {
    pub information: Information,
    pub verdict: AdmissionVerdict,
}
impl AssessedItem {
    /// Pertinência à lacuna não implica admissibilidade, e vice-versa.
    pub fn addresses(self, request: InformationRequest) -> bool {
        self.information.statement.subject() == request.missing.subject()
            && self.information.statement.object() == request.missing.object()
            && self.information.statement.scope() == request.missing.scope()
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TraceKind {
    Query(EpisodeId),
    Request(InformationRequest),
    Received {
        package: FeedbackPackage,
        envelope_error: Option<ContractError>,
    },
    Interpreted,
    Assessed {
        policy: &'static str,
        items: Vec<AssessedItem>,
    },
    Incorporated {
        ids: Vec<EngramId>,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceEntry {
    pub at: u64,
    pub kind: TraceKind,
    pub before: Vec<BoundRecord>,
    pub after: Vec<BoundRecord>,
}
#[derive(Clone, Copy)]
struct Reception {
    package: FeedbackPackage,
    at: u64,
    error: Option<ContractError>,
}
#[derive(Clone, Copy)]
struct InterpretedItem {
    information: Information,
    error: Option<ContractError>,
}

/// Possui memória, metadados e histórico limitados. Nenhum acesso mutável é exposto.
/// Consulta futura é recebida apenas em `query_reserved`, após reavaliação.
pub struct CognitiveEpisode {
    memory: EngramStore,
    metadata: Vec<RecordMetadata>,
    concepts: Vec<Concept>,
    authorization: RuleAuthorization,
    sources: Vec<SourceId>,
    phase: Phase,
    episodes: EpisodeLog,
    reports: Vec<QueryReport>,
    exchange: ExchangeLog,
    reception: Option<Reception>,
    interpreted: Vec<InterpretedItem>,
    assessed: Vec<AssessedItem>,
    quarantine: Vec<Rejection>,
    trace: Vec<TraceEntry>,
}
impl CognitiveEpisode {
    pub fn new(
        memory: EngramStore,
        metadata: &[RecordMetadata],
        concepts: &[Concept],
        authorization: RuleAuthorization,
        sources: &[SourceId],
    ) -> Result<Self, AcquisitionError> {
        if concepts.is_empty() || concepts.len() > MAX_CONCEPTS || memory.capacity() > MAX_RECORDS {
            return Err(ContractError::Capacity.into());
        }
        for (i, c) in concepts.iter().enumerate() {
            if concepts[..i].iter().any(|old| old.node() == c.node()) {
                return Err(ContractError::Duplicate.into());
            }
        }
        QueryCoordinator::new(&memory, metadata, authorization, sources)?;
        let episode = Self {
            memory,
            metadata: metadata.to_vec(),
            concepts: concepts.to_vec(),
            authorization,
            sources: sources.to_vec(),
            phase: Phase::Ready,
            episodes: EpisodeLog::default(),
            reports: Vec::new(),
            exchange: ExchangeLog::default(),
            reception: None,
            interpreted: Vec::new(),
            assessed: Vec::new(),
            quarantine: Vec::new(),
            trace: Vec::new(),
        };
        for record in &episode.metadata {
            episode.in_catalog(record.information.statement)?;
        }
        Ok(episode)
    }
    pub const fn phase(&self) -> Phase {
        self.phase
    }
    pub fn memory(&self) -> &EngramStore {
        &self.memory
    }
    pub fn metadata(&self) -> &[RecordMetadata] {
        &self.metadata
    }
    pub fn episodes(&self) -> &EpisodeLog {
        &self.episodes
    }
    pub fn reports(&self) -> &[QueryReport] {
        &self.reports
    }
    pub fn trace(&self) -> &[TraceEntry] {
        &self.trace
    }
    pub fn quarantine(&self) -> &[Rejection] {
        &self.quarantine
    }
    pub fn assessments(&self) -> &[AssessedItem] {
        &self.assessed
    }
    pub fn request(&self) -> Option<InformationRequest> {
        self.exchange.request()
    }
    /// Inclui retorno inválido; ExchangeLog M1 conserva apenas o vínculo válido.
    pub fn received(&self) -> Option<FeedbackPackage> {
        self.reception.map(|r| r.package)
    }

    fn expect(&self, phase: Phase) -> Result<(), AcquisitionError> {
        if self.phase != phase {
            return Err(AcquisitionError::WrongPhase);
        }
        Ok(())
    }
    fn check_step(&self, at: u64) -> Result<(), AcquisitionError> {
        if self.trace.len() >= MAX_TRACE {
            return Err(ContractError::Capacity.into());
        }
        if self.trace.last().is_some_and(|entry| entry.at > at) {
            return Err(ContractError::OutsideValidity.into());
        }
        Ok(())
    }
    fn snapshot(&self) -> Vec<BoundRecord> {
        self.memory
            .iter()
            .map(|engram| BoundRecord {
                engram,
                metadata: *self
                    .metadata
                    .iter()
                    .find(|m| m.id == engram.id())
                    .expect("vínculo M2 validado"),
            })
            .collect()
    }
    fn append(&mut self, at: u64, kind: TraceKind, before: Vec<BoundRecord>) {
        self.trace.push(TraceEntry {
            at,
            kind,
            before,
            after: self.snapshot(),
        });
    }
    fn in_catalog(&self, statement: Statement) -> Result<(), ContractError> {
        let from = self
            .concepts
            .iter()
            .find(|c| c.node() == statement.subject())
            .ok_or(ContractError::InvalidFormat)?;
        let to = self
            .concepts
            .iter()
            .find(|c| c.node() == statement.object())
            .ok_or(ContractError::InvalidFormat)?;
        let interpreted = Statement::inclusion(from, to, statement.polarity(), statement.scope())?;
        if interpreted != statement {
            return Err(ContractError::InvalidFormat);
        }
        Ok(())
    }
    fn run_query(
        &mut self,
        query: Statement,
        id: EpisodeId,
        at: u64,
        next: Phase,
    ) -> Result<&QueryReport, AcquisitionError> {
        self.check_step(at)?;
        self.in_catalog(query)?;
        let before = self.snapshot();
        // A instância empresta a memória apenas nesta expressão e é encerrada antes de qualquer escrita.
        let report = QueryCoordinator::new(
            &self.memory,
            &self.metadata,
            self.authorization,
            &self.sources,
        )?
        .query(id, query, at, &mut self.episodes)?;
        self.reports.push(report);
        self.append(at, TraceKind::Query(id), before);
        self.phase = next;
        Ok(self.reports.last().expect("relatório acrescentado"))
    }
    pub fn evaluate_initial(
        &mut self,
        query: Statement,
        id: EpisodeId,
        at: u64,
    ) -> Result<&QueryReport, AcquisitionError> {
        self.expect(Phase::Ready)?;
        self.run_query(query, id, at, Phase::Initial)
    }

    /// Política finita: uma única aresta positiva elegível X→Y sugere a falta Y→Z.
    /// Ambiguidade ou oposição à base impede escolher por conveniência/ID.
    pub fn derive_request(
        &mut self,
        id: RequestId,
        at: u64,
    ) -> Result<InformationRequest, AcquisitionError> {
        if self.exchange.request().is_some() {
            return Err(ContractError::Capacity.into());
        }
        self.expect(Phase::Initial)?;
        self.check_step(at)?;
        let initial = &self.reports[0];
        if initial.episode.decision
            != EpisodeDecision::Abstain(Refusal::Monitor(AbstainReason::MissingEvidence))
        {
            return Err(AcquisitionError::NoUniqueGap);
        }
        if at <= initial.episode.at {
            return Err(ContractError::OutsideValidity.into());
        }
        let query = initial.episode.query;
        let mut candidate = None;
        for record in &initial.consulted {
            let statement = record.metadata.information.statement;
            if statement.polarity() != Polarity::Positive
                || statement.subject() != query.subject()
                || statement.object() == query.object()
            {
                continue;
            }
            let opposed = initial.consulted.iter().any(|r| {
                let other = r.metadata.information.statement;
                other.subject() == statement.subject()
                    && other.object() == statement.object()
                    && other.polarity() == Polarity::Negative
            });
            if opposed {
                continue;
            }
            if candidate.is_some() {
                return Err(AcquisitionError::NoUniqueGap);
            }
            candidate = Some(*record);
        }
        let base = candidate.ok_or(AcquisitionError::NoUniqueGap)?;
        check_observation(base.metadata.information, query.scope(), &self.sources, at)?;
        let from = self
            .concepts
            .iter()
            .find(|c| c.node() == base.metadata.information.statement.object())
            .ok_or(ContractError::InvalidFormat)?;
        let to = self
            .concepts
            .iter()
            .find(|c| c.node() == query.object())
            .ok_or(ContractError::InvalidFormat)?;
        let missing = Statement::inclusion(from, to, Polarity::Positive, query.scope())?;
        let request = InformationRequest {
            id,
            episode: initial.episode.id,
            requested_at: at,
            missing,
            basis: EvidenceRefs::Direct(base.engram.id()),
        };
        let before = self.snapshot();
        self.exchange.record_request(request)?;
        self.append(at, TraceKind::Request(request), before);
        self.phase = Phase::Requested;
        Ok(request)
    }

    /// Recepção consome a única oportunidade mesmo se o envelope estiver inválido.
    /// Nenhuma informação é admitida ou incorporada aqui.
    pub fn receive(&mut self, package: FeedbackPackage, at: u64) -> Result<(), AcquisitionError> {
        if self.reception.is_some() {
            return Err(ContractError::Capacity.into());
        }
        self.expect(Phase::Requested)?;
        self.check_step(at)?;
        let request = self.exchange.request().expect("fase com pedido");
        let error = package.check_link(request).err().or_else(|| {
            if package.returned_at <= request.requested_at
                || package.returned_at > at
                || !request
                    .missing
                    .scope()
                    .validity
                    .contains(package.returned_at)
                || !request.missing.scope().validity.contains(at)
            {
                Some(ContractError::OutsideValidity)
            } else if !package
                .envelope_source
                .is_some_and(|s| self.sources.contains(&s))
            {
                Some(ContractError::UnknownSource)
            } else {
                None
            }
        });
        let before = self.snapshot();
        if error.is_none() {
            self.exchange.record_feedback(package)?;
        }
        self.reception = Some(Reception { package, at, error });
        self.append(
            at,
            TraceKind::Received {
                package,
                envelope_error: error,
            },
            before,
        );
        self.phase = Phase::Received;
        Ok(())
    }

    /// Reconhecimento do vocabulário já tipado; não interpreta texto livre.
    pub fn interpret(&mut self, at: u64) -> Result<(), AcquisitionError> {
        self.expect(Phase::Received)?;
        self.check_step(at)?;
        let received = self.reception.expect("recepção registrada");
        let items = match received.package.content {
            FeedbackContent::Unavailable => vec![],
            FeedbackContent::One(a) => vec![a],
            FeedbackContent::Two(a, b) => vec![a, b],
        };
        let interpreted = items
            .into_iter()
            .map(|information| InterpretedItem {
                error: received
                    .error
                    .or_else(|| self.in_catalog(information.statement).err()),
                information,
            })
            .collect();
        let before = self.snapshot();
        self.interpreted = interpreted;
        self.append(at, TraceKind::Interpreted, before);
        self.phase = Phase::Interpreted;
        Ok(())
    }

    pub fn assess_admissibility(&mut self, at: u64) -> Result<&[AssessedItem], AcquisitionError> {
        self.expect(Phase::Interpreted)?;
        self.check_step(at)?;
        let request = self.exchange.request().expect("pedido registrado");
        let received = self.reception.expect("recepção registrada");
        let mut assessed = Vec::new();
        let mut quarantine = Vec::new();
        if let Some(reason) = received.error {
            if self.interpreted.is_empty() {
                quarantine.push(Rejection {
                    feedback: received.package.id,
                    item: None,
                    disposition: Disposition::Quarantined,
                    reason,
                });
            }
        }
        for (index, item) in self.interpreted.iter().enumerate() {
            let info = item.information;
            let check = (|| -> Result<(), ContractError> {
                if let Some(error) = item.error {
                    return Err(error);
                }
                check_observation(info, request.missing.scope(), &self.sources, at)?;
                if info.received_at != received.at {
                    return Err(ContractError::OutsideValidity);
                }
                let duplicate = self.metadata.iter().any(|m| {
                    m.information.statement == info.statement
                        || info.evidence.is_some() && m.information.evidence == info.evidence
                }) || self.interpreted[..index].iter().any(|old| {
                    old.information.statement == info.statement
                        || info.evidence.is_some() && old.information.evidence == info.evidence
                });
                if duplicate {
                    return Err(ContractError::Duplicate);
                }
                // M4/C1: observação positiva sobre o sujeito pedido pode ser irrelevante.
                // Oposição continua restrita à relação pedida; outros sujeitos são recusados.
                if info.statement.subject() != request.missing.subject()
                    || (info.statement.polarity() == Polarity::Negative
                        && info.statement.object() != request.missing.object())
                {
                    return Err(ContractError::InvalidFormat);
                }
                i32::try_from(info.statement.object().get())
                    .map_err(|_| ContractError::InvalidFormat)?;
                Ok(())
            })();
            let verdict = match check {
                Ok(()) => AdmissionVerdict::Admissible,
                Err(reason) => {
                    quarantine.push(Rejection {
                        feedback: received.package.id,
                        item: Some(info),
                        disposition: Disposition::Quarantined,
                        reason,
                    });
                    AdmissionVerdict::Quarantined(reason)
                }
            };
            assessed.push(AssessedItem {
                information: info,
                verdict,
            });
        }
        let before = self.snapshot();
        self.append(
            at,
            TraceKind::Assessed {
                policy: ADMISSION_POLICY,
                items: assessed.clone(),
            },
            before,
        );
        self.assessed = assessed;
        self.quarantine = quarantine;
        self.phase = Phase::Assessed;
        Ok(&self.assessed)
    }

    /// Único ato que escreve memória factual; lote preparado e validado antes da troca.
    /// IDs são fornecidos explicitamente, sem deduzir significado por aritmética.
    pub fn incorporate(&mut self, ids: &[EngramId], at: u64) -> Result<(), AcquisitionError> {
        self.expect(Phase::Assessed)?;
        self.check_step(at)?;
        let admissible: Vec<_> = self
            .assessed
            .iter()
            .filter(|item| item.verdict == AdmissionVerdict::Admissible)
            .collect();
        if ids.len() != admissible.len() {
            return Err(ContractError::InvalidFormat.into());
        }
        if self.memory.len() + ids.len() > self.memory.capacity()
            || self.memory.len() + ids.len() > MAX_RECORDS
        {
            return Err(ContractError::Capacity.into());
        }
        for (i, id) in ids.iter().enumerate() {
            if self.memory.get(*id).is_ok() || ids[..i].contains(id) {
                return Err(ContractError::Duplicate.into());
            }
        }
        let mut staged =
            EngramStore::new(self.memory.capacity()).map_err(AcquisitionError::Memory)?;
        let mut metadata = self.metadata.clone();
        for record in self.memory.iter() {
            staged.record(record).map_err(AcquisitionError::Memory)?;
        }
        for (id, item) in ids.iter().zip(admissible) {
            let info = item.information;
            let statement = info.statement;
            check_observation(info, self.authorization.scope(), &self.sources, at)?;
            let target = i32::try_from(statement.object().get())
                .map_err(|_| ContractError::InvalidFormat)?;
            let cue = Cue::new(statement.scope().context.get(), statement.subject(), target)
                .map_err(AcquisitionError::Memory)?;
            // Origem relacional não é fonte externa. Fonte permanece nos metadados.
            let engram = Engram::new(
                *id,
                cue,
                if statement.polarity() == Polarity::Positive {
                    1
                } else {
                    -1
                },
                None,
                at,
                info.evidence,
                RecordStatus::Observed,
            )
            .map_err(AcquisitionError::Memory)?;
            staged.record(engram).map_err(AcquisitionError::Memory)?;
            metadata.push(RecordMetadata {
                id: *id,
                information: info,
                state: EpistemicState::AdmittedObservation,
                disposition: None,
            });
        }
        QueryCoordinator::new(&staged, &metadata, self.authorization, &self.sources)?;
        let before = self.snapshot();
        self.memory = staged;
        self.metadata = metadata;
        self.append(at, TraceKind::Incorporated { ids: ids.to_vec() }, before);
        self.phase = Phase::Incorporated;
        Ok(())
    }
    pub fn reassess(&mut self, id: EpisodeId, at: u64) -> Result<&QueryReport, AcquisitionError> {
        // C3 repete o gate sem pedido, retorno ou aquisição fictícios.
        if !(self.phase == Phase::Initial
            && self.reports[0].episode.decision
                == EpisodeDecision::Abstain(Refusal::RuleOutsideContext))
        {
            self.expect(Phase::Incorporated)?;
        }
        self.run_query(self.reports[0].episode.query, id, at, Phase::Reassessed)
    }
    pub fn query_reserved(
        &mut self,
        query: Statement,
        id: EpisodeId,
        at: u64,
    ) -> Result<&QueryReport, AcquisitionError> {
        self.expect(Phase::Reassessed)?;
        self.run_query(query, id, at, Phase::Complete)
    }
}

#[cfg(test)]
mod tests;
