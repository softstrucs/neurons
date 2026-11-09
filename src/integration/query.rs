//! M2: consulta somente de leitura, com metadados vinculados à memória existente.
//! A admissão é uma pré-condição declarada pelo chamador; não é produzida aqui.
use crate::cognition::{CognitionError, Hypothesis, Outcome, TwoHopTask};
use crate::cognitive_contracts::{
    check_observation, ContractError, Disposition, Episode, EpisodeDecision, EpisodeId, EpisodeLog,
    EpistemicState, Information, MemorySnapshot, Polarity, Refusal, Scope, SourceId, Statement,
    MAX_EPISODES, MAX_RECORDS,
};
use crate::engram::{Engram, EngramError, EngramId, EngramStore, RecordStatus};
use crate::event::NodeId;
use crate::metacognition::{
    AbstainReason, Decision, EvidenceRefs, Monitor, ReviewError, ReviewReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryError {
    Contract(ContractError),
    MetadataMismatch(EngramId),
    Memory(EngramError),
    Cognition(CognitionError),
    Review(ReviewError),
}
impl From<ContractError> for QueryError {
    fn from(value: ContractError) -> Self {
        Self::Contract(value)
    }
}

/// Metadados públicos preexistentes; não é pacote de feedback ou ato de admissão.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecordMetadata {
    pub id: EngramId,
    pub information: Information,
    pub state: EpistemicState,
    pub disposition: Option<Disposition>,
}
/// Cópia exata do engrama e seus metadados; relatórios não dependem só de IDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundRecord {
    pub engram: Engram,
    pub metadata: RecordMetadata,
}

/// Permissão externa explícita para inclusão de dois passos em um único escopo.
/// Não é derivada dos fatos nem da consulta. K2 não herda autorização de K1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleAuthorization {
    scope: Scope,
}
impl RuleAuthorization {
    pub const fn inclusion_in(scope: Scope) -> Self {
        Self { scope }
    }
    pub const fn scope(self) -> Scope {
        self.scope
    }
    fn allows(self, query: Statement, at: u64) -> bool {
        self.scope == query.scope() && self.scope.validity.contains(at)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExclusionReason {
    NotAdmitted,
    Disposition(Disposition),
    Contract(ContractError),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExcludedRecord {
    pub record: BoundRecord,
    pub reason: ExclusionReason,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProofError {
    MissingPremise,
    IneligiblePremise(EngramId),
    NegativePremise(EngramId),
    DisconnectedChain,
    InconsistentReport,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerifiedProof {
    Direct(BoundRecord),
    TwoHop {
        first: BoundRecord,
        second: BoundRecord,
        via: NodeId,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryReport {
    pub episode: Episode,
    /// None quando o gate recusa antes da inferência.
    pub review: Option<ReviewReport>,
    pub proof: Option<VerifiedProof>,
    pub proof_error: Option<ProofError>,
    /// Projeção fornecida ao monitor, inclusive oposição; não é contagem de leituras.
    pub consulted: Vec<BoundRecord>,
    pub excluded: Vec<ExcludedRecord>,
}

/// Mantém empréstimo imutável de EngramStore durante toda a vida da interface.
/// Não recebe Engine/Circuit, feedback, avaliador, gabarito ou consulta futura.
pub struct QueryCoordinator<'a> {
    memory: &'a EngramStore,
    records: Vec<BoundRecord>,
    authorization: RuleAuthorization,
    known_sources: Vec<SourceId>,
    monitor: Monitor,
}
impl<'a> QueryCoordinator<'a> {
    pub fn new(
        memory: &'a EngramStore,
        metadata: &[RecordMetadata],
        authorization: RuleAuthorization,
        known_sources: &[SourceId],
    ) -> Result<Self, QueryError> {
        if memory.len() > MAX_RECORDS
            || metadata.len() > MAX_RECORDS
            || known_sources.len() > MAX_RECORDS
        {
            return Err(ContractError::Capacity.into());
        }
        if metadata.len() != memory.len() {
            return Err(ContractError::InvalidFormat.into());
        }
        for (i, source) in known_sources.iter().enumerate() {
            if known_sources[..i].contains(source) {
                return Err(ContractError::Duplicate.into());
            }
        }
        for (i, meta) in metadata.iter().enumerate() {
            if metadata[..i].iter().any(|old| old.id == meta.id) {
                return Err(ContractError::Duplicate.into());
            }
            if memory.get(meta.id).is_err() {
                return Err(QueryError::MetadataMismatch(meta.id));
            }
        }
        let mut records = Vec::new();
        // Ordem de inserção da memória é preservada, independentemente da ordem dos metadados.
        for engram in memory.iter() {
            let meta = *metadata
                .iter()
                .find(|m| m.id == engram.id())
                .ok_or(QueryError::MetadataMismatch(engram.id()))?;
            let info = meta.information;
            let statement = info.statement;
            let target = i32::try_from(statement.object().get())
                .map_err(|_| QueryError::MetadataMismatch(meta.id))?;
            let value = match statement.polarity() {
                Polarity::Positive => 1,
                Polarity::Negative => -1,
            };
            if engram.cue().context() != statement.scope().context.get()
                || engram.cue().source() != statement.subject()
                || engram.cue().pattern() != target
                || engram.value() != value
                || engram.evidence() != info.evidence
                || engram.observed_at() < info.received_at
            {
                return Err(QueryError::MetadataMismatch(meta.id));
            }
            records.push(BoundRecord {
                engram,
                metadata: meta,
            });
        }
        let task = TwoHopTask::new(MAX_RECORDS, 16, 32).map_err(QueryError::Cognition)?;
        let monitor = Monitor::new(task, 1).map_err(QueryError::Review)?;
        Ok(Self {
            memory,
            records,
            authorization,
            known_sources: known_sources.to_vec(),
            monitor,
        })
    }

    fn eligibility(
        &self,
        record: BoundRecord,
        query: Statement,
        at: u64,
    ) -> Result<(), ExclusionReason> {
        let meta = record.metadata;
        if let Some(disposition) = meta.disposition {
            return Err(ExclusionReason::Disposition(disposition));
        }
        if meta.state != EpistemicState::AdmittedObservation
            || record.engram.status() != RecordStatus::Observed
        {
            return Err(ExclusionReason::NotAdmitted);
        }
        check_observation(meta.information, query.scope(), &self.known_sources, at)
            .map_err(ExclusionReason::Contract)?;
        if record.engram.observed_at() > at
            || !query.scope().validity.contains(record.engram.observed_at())
        {
            return Err(ExclusionReason::Contract(ContractError::OutsideValidity));
        }
        Ok(())
    }

    /// Verifica somente as referências retornadas; não procura ou gera outra prova.
    fn verify_proof(
        &self,
        query: Statement,
        at: u64,
        refs: EvidenceRefs,
    ) -> Result<VerifiedProof, ProofError> {
        let premise = |id| -> Result<BoundRecord, ProofError> {
            let record = *self
                .records
                .iter()
                .find(|r| r.engram.id() == id)
                .ok_or(ProofError::MissingPremise)?;
            self.eligibility(record, query, at)
                .map_err(|_| ProofError::IneligiblePremise(id))?;
            if record.metadata.information.statement.polarity() != Polarity::Positive {
                return Err(ProofError::NegativePremise(id));
            }
            Ok(record)
        };
        match refs {
            EvidenceRefs::Direct(id) => {
                let record = premise(id)?;
                if record.metadata.information.statement != query {
                    return Err(ProofError::DisconnectedChain);
                }
                Ok(VerifiedProof::Direct(record))
            }
            EvidenceRefs::TwoHop { first, second } => {
                let a = premise(first)?;
                let b = premise(second)?;
                let x = a.metadata.information.statement;
                let y = b.metadata.information.statement;
                if first == second
                    || x.subject() != query.subject()
                    || x.object() != y.subject()
                    || y.object() != query.object()
                    || x.object() == query.subject()
                    || x.object() == query.object()
                {
                    return Err(ProofError::DisconnectedChain);
                }
                Ok(VerifiedProof::TwoHop {
                    first: a,
                    second: b,
                    via: x.object(),
                })
            }
        }
    }

    pub fn query(
        &self,
        id: EpisodeId,
        query: Statement,
        at: u64,
        log: &mut EpisodeLog,
    ) -> Result<QueryReport, QueryError> {
        // Falhas de envelope/log não executam inferência e preservam o histórico.
        if log.entries().iter().any(|e| e.id == id) {
            return Err(ContractError::Duplicate.into());
        }
        if log.entries().len() >= MAX_EPISODES {
            return Err(ContractError::Capacity.into());
        }
        if !query.scope().validity.contains(at) || log.entries().last().is_some_and(|e| e.at > at) {
            return Err(ContractError::OutsideValidity.into());
        }
        let snapshot = MemorySnapshot::new(
            &self
                .records
                .iter()
                .map(|r| r.engram.id())
                .collect::<Vec<_>>(),
        )?;
        let mut report = QueryReport {
            episode: Episode {
                id,
                at,
                query,
                before: snapshot.clone(),
                after: snapshot,
                prior: EpistemicState::Hypothesis,
                posterior: EpistemicState::Hypothesis,
                decision: EpisodeDecision::Abstain(Refusal::RuleOutsideContext),
            },
            review: None,
            proof: None,
            proof_error: None,
            consulted: Vec::new(),
            excluded: Vec::new(),
        };
        if !self.authorization.allows(query, at) {
            // A própria recusa é registrada. Monitor e TwoHopTask não são chamados.
        } else if query.polarity() != Polarity::Positive {
            report.episode.decision =
                EpisodeDecision::Abstain(Refusal::Contract(ContractError::InvalidFormat));
        } else {
            let mut projection = EngramStore::new(MAX_RECORDS).map_err(QueryError::Memory)?;
            for record in self.records.iter().copied() {
                match self.eligibility(record, query, at) {
                    Ok(()) => {
                        projection
                            .record(record.engram)
                            .map_err(QueryError::Memory)?;
                        report.consulted.push(record);
                    }
                    Err(reason) => report.excluded.push(ExcludedRecord { record, reason }),
                }
            }
            let hypothesis =
                Hypothesis::new(query.scope().context.get(), query.subject(), query.object())
                    .map_err(QueryError::Cognition)?;
            let review = self
                .monitor
                .review(&projection, hypothesis, &[])
                .map_err(QueryError::Review)?;
            report.episode.decision = match review.decision {
                Decision::Abstain(reason) => EpisodeDecision::Abstain(Refusal::Monitor(reason)),
                Decision::Respond {
                    hypothesis: actual,
                    evidence,
                    revision,
                } => {
                    let proof = self.verify_proof(query, at, evidence).and_then(|proof| {
                        // As referências e a unidade intermediária devem coincidir com o relatório bruto.
                        let consistent = review
                            .attempts
                            .first()
                            .and_then(|a| a.inference)
                            .is_some_and(|r| {
                                r.hypothesis == hypothesis
                                    && match (r.outcome, proof) {
                                        (
                                            Outcome::DirectRecall(id),
                                            VerifiedProof::Direct(record),
                                        ) => id == record.engram.id(),
                                        (
                                            Outcome::Inferred(p),
                                            VerifiedProof::TwoHop { first, second, via },
                                        ) => {
                                            p.first_fact == first.engram.id()
                                                && p.second_fact == second.engram.id()
                                                && p.via == via
                                                && p.via_activations > 0
                                        }
                                        _ => false,
                                    }
                            });
                        if !consistent || actual != hypothesis || revision.is_some() {
                            return Err(ProofError::InconsistentReport);
                        }
                        Ok(proof)
                    });
                    match proof {
                        Ok(proof) => {
                            report.proof = Some(proof);
                            report.episode.posterior = EpistemicState::ProvisionalKnowledge;
                            EpisodeDecision::Provisional {
                                premises: evidence,
                                rule: query.scope().rule,
                            }
                        }
                        Err(error) => {
                            report.proof_error = Some(error);
                            EpisodeDecision::Abstain(Refusal::Monitor(
                                AbstainReason::InvalidEvidence,
                            ))
                        }
                    }
                }
            };
            report.review = Some(review);
        }
        // O empréstimo imutável protege conteúdo/ordem/status. Nenhum fato é escrito.
        debug_assert!(self.memory.iter().eq(self.records.iter().map(|r| r.engram)));
        log.append(report.episode.clone())?;
        Ok(report)
    }
}

#[cfg(test)]
mod tests;

/// MP06 aditivo: não amplia os enums públicos M1/F1 com variantes incompatíveis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PathAuthorization {
    scope: Scope,
}
impl PathAuthorization {
    /// Ato explícito do chamador, separado da permissão legada de dois passos.
    pub const fn inclusion_three(scope: Scope) -> Self {
        Self { scope }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathProof {
    pub premises: Vec<BoundRecord>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathQueryReport {
    pub id: EpisodeId,
    pub query: Statement,
    pub at: u64,
    pub before: MemorySnapshot,
    pub after: MemorySnapshot,
    pub prior: EpistemicState,
    pub posterior: EpistemicState,
    pub rule: crate::cognitive_contracts::RuleRef,
    pub proof: Option<PathProof>,
    pub alternatives: Vec<PathProof>,
    pub refusal: Option<Refusal>,
    pub review: Option<crate::metacognition::PathReview>,
    pub consulted: Vec<BoundRecord>,
    pub excluded: Vec<ExcludedRecord>,
}
#[derive(Default)]
pub struct PathEpisodeLog {
    entries: Vec<PathQueryReport>,
}
impl PathEpisodeLog {
    pub fn entries(&self) -> &[PathQueryReport] {
        &self.entries
    }
}
impl QueryCoordinator<'_> {
    /// Verifica referências fornecidas contra fatos/metadados reais. Nunca procura prova substituta.
    pub fn verify_path(
        &self,
        query: Statement,
        at: u64,
        ids: &[EngramId],
    ) -> Result<PathProof, ProofError> {
        if query.polarity() != Polarity::Positive {
            return Err(ProofError::InconsistentReport);
        }
        if ids.is_empty() || ids.len() > 3 {
            return Err(ProofError::DisconnectedChain);
        }
        let mut premises = vec![];
        let mut node = query.subject();
        let mut visited = vec![node];
        for id in ids {
            let r = *self
                .records
                .iter()
                .find(|r| r.engram.id() == *id)
                .ok_or(ProofError::MissingPremise)?;
            self.eligibility(r, query, at)
                .map_err(|_| ProofError::IneligiblePremise(*id))?;
            let s = r.metadata.information.statement;
            if s.polarity() != Polarity::Positive {
                return Err(ProofError::NegativePremise(*id));
            }
            if s.subject() != node || visited.contains(&s.object()) {
                return Err(ProofError::DisconnectedChain);
            }
            node = s.object();
            visited.push(node);
            premises.push(r);
        }
        if node != query.object() {
            return Err(ProofError::DisconnectedChain);
        }
        Ok(PathProof { premises })
    }
    pub fn query_three(
        &self,
        id: EpisodeId,
        query: Statement,
        at: u64,
        authorization: PathAuthorization,
        budget: usize,
        log: &mut PathEpisodeLog,
    ) -> Result<PathQueryReport, QueryError> {
        // Conserva limite M1 de três episódios, mais restritivo que o teto 32 do intervalo.
        if log.entries.len() >= MAX_EPISODES {
            return Err(ContractError::Capacity.into());
        }
        if log.entries.iter().any(|e| e.id == id) {
            return Err(ContractError::Duplicate.into());
        }
        if !query.scope().validity.contains(at) || log.entries.last().is_some_and(|e| e.at > at) {
            return Err(ContractError::OutsideValidity.into());
        }
        let snapshot = MemorySnapshot::new(
            &self
                .records
                .iter()
                .map(|r| r.engram.id())
                .collect::<Vec<_>>(),
        )?;
        let mut out = PathQueryReport {
            id,
            query,
            at,
            before: snapshot.clone(),
            after: snapshot,
            prior: EpistemicState::Hypothesis,
            posterior: EpistemicState::Hypothesis,
            rule: query.scope().rule,
            proof: None,
            alternatives: vec![],
            refusal: None,
            review: None,
            consulted: vec![],
            excluded: vec![],
        };
        if authorization.scope != query.scope() || !self.authorization.allows(query, at) {
            out.refusal = Some(Refusal::RuleOutsideContext)
        } else if query.polarity() != Polarity::Positive {
            out.refusal = Some(Refusal::Contract(ContractError::InvalidFormat))
        } else {
            let mut projection = EngramStore::new(MAX_RECORDS).map_err(QueryError::Memory)?;
            for r in self.records.iter().copied() {
                match self.eligibility(r, query, at) {
                    Ok(()) => {
                        projection.record(r.engram).map_err(QueryError::Memory)?;
                        out.consulted.push(r)
                    }
                    Err(reason) => out.excluded.push(ExcludedRecord { record: r, reason }),
                }
            }
            let hyp = Hypothesis::new(query.scope().context.get(), query.subject(), query.object())
                .map_err(QueryError::Cognition)?;
            let review = self
                .monitor
                .review_paths(&projection, hyp, budget)
                .map_err(QueryError::Cognition)?;
            for path in &review.inference.alternatives {
                match self.verify_path(query, at, path) {
                    Ok(p) => out.alternatives.push(p),
                    Err(_) => {
                        out.refusal = Some(Refusal::Monitor(AbstainReason::InvalidEvidence));
                        break;
                    }
                }
            }
            if out.refusal.is_none() {
                use crate::metacognition::Assessment;
                out.refusal = match review.assessment {
                    Assessment::Sufficient => None,
                    Assessment::MissingEvidence => {
                        Some(Refusal::Monitor(AbstainReason::MissingEvidence))
                    }
                    Assessment::ContradictoryEvidence => {
                        Some(Refusal::Monitor(AbstainReason::ContradictoryEvidence))
                    }
                    Assessment::ResourceLimited => {
                        Some(Refusal::Monitor(AbstainReason::ResourceLimited))
                    }
                    Assessment::PropagationFailure => {
                        Some(Refusal::Monitor(AbstainReason::PropagationFailure))
                    }
                    Assessment::InvalidEvidence => {
                        Some(Refusal::Monitor(AbstainReason::InvalidEvidence))
                    }
                };
                if out.refusal.is_none() {
                    out.proof = out.alternatives.first().cloned();
                    if out.proof.is_some() {
                        out.posterior = EpistemicState::ProvisionalKnowledge
                    } else {
                        out.refusal = Some(Refusal::Monitor(AbstainReason::InvalidEvidence))
                    }
                }
            }
            out.review = Some(review);
        }
        debug_assert!(self.memory.iter().eq(self.records.iter().map(|r| r.engram)));
        log.entries.push(out.clone());
        Ok(out)
    }
}
