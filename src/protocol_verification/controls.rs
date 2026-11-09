//! Controles recebem apenas fatos e configuração públicos. Seleção simbólica própria.
use super::*;
use crate::cognition::{Hypothesis, Outcome, TwoHopTask};
use crate::cognitive_contracts::{DeclaredNature, EpistemicState, Polarity, MAX_RECORDS};
use crate::engram::{EngramId, RecordStatus};
use crate::integration::query::VerifiedProof;

/// Filtro do instrumento, implementado sem QueryCoordinator/check_observation/Monitor.
/// Divergência estrutural é erro; registros corretamente identificados mas inelegíveis são excluídos.
pub fn public_records(input: &PublicInput<'_>) -> Result<Vec<BoundRecord>, VerificationIssue> {
    if input.memory.len() > MAX_RECORDS
        || input.metadata.len() > MAX_RECORDS
        || input.known_sources.len() > MAX_RECORDS
    {
        return Err(VerificationIssue::Bounds);
    }
    if input.memory.len() != input.metadata.len() {
        return Err(VerificationIssue::Metadata);
    }
    for (i, m) in input.metadata.iter().enumerate() {
        if input.metadata[..i].iter().any(|old| old.id == m.id) {
            return Err(VerificationIssue::Duplicate);
        }
        if input.memory.get(m.id).is_err() {
            return Err(VerificationIssue::Metadata);
        }
    }
    let mut selected = Vec::new();
    for engram in input.memory.iter() {
        let m = *input
            .metadata
            .iter()
            .find(|m| m.id == engram.id())
            .ok_or(VerificationIssue::Metadata)?;
        let i = m.information;
        let s = i.statement;
        let target = i32::try_from(s.object().get()).map_err(|_| VerificationIssue::Metadata)?;
        if engram.cue().context() != s.scope().context.get()
            || engram.cue().source() != s.subject()
            || engram.cue().pattern() != target
            || engram.value()
                != if s.polarity() == Polarity::Positive {
                    1
                } else {
                    -1
                }
            || engram.evidence() != i.evidence
            || engram.observed_at() < i.received_at
        {
            return Err(VerificationIssue::Metadata);
        }
        if m.disposition.is_some()
            || m.state != EpistemicState::AdmittedObservation
            || engram.status() != RecordStatus::Observed
            || i.nature != DeclaredNature::Observation
            || i.evidence.is_none()
            || !i
                .declared_source
                .is_some_and(|source| input.known_sources.contains(&source))
            || s.scope() != input.query.scope()
            || !s.scope().validity.contains(input.at)
            || !s.scope().validity.contains(i.received_at)
            || !s.scope().validity.contains(engram.observed_at())
            || i.received_at > input.at
            || engram.observed_at() > input.at
        {
            continue;
        }
        selected.push(BoundRecord {
            engram,
            metadata: m,
        });
    }
    Ok(selected)
}
fn gate(input: &PublicInput<'_>) -> Option<Class> {
    if !input.query.scope().validity.contains(input.at) {
        Some(Class::InvalidEvidence)
    } else if input.authorization.scope() != input.query.scope() {
        Some(Class::OutsideRule)
    } else if input.query.polarity() != Polarity::Positive {
        Some(Class::InvalidEvidence)
    } else {
        None
    }
}
fn endpoints(record: BoundRecord) -> (crate::event::NodeId, crate::event::NodeId) {
    let s = record.metadata.information.statement;
    (s.subject(), s.object())
}
fn negative(records: &[BoundRecord], from: crate::event::NodeId, to: crate::event::NodeId) -> bool {
    records.iter().any(|r| {
        endpoints(*r) == (from, to)
            && r.metadata.information.statement.polarity() == Polarity::Negative
    })
}
fn positives(records: &[BoundRecord]) -> Result<Vec<BoundRecord>, VerificationIssue> {
    let mut positive: Vec<_> = records
        .iter()
        .copied()
        .filter(|r| r.metadata.information.statement.polarity() == Polarity::Positive)
        .collect();
    positive.sort_by_key(|r| {
        let (a, b) = endpoints(*r);
        (a, b, r.engram.id())
    });
    if positive
        .windows(2)
        .any(|p| endpoints(p[0]) == endpoints(p[1]))
    {
        return Err(VerificationIssue::Duplicate);
    }
    Ok(positive)
}

pub fn evaluate(mechanism: Mechanism, input: &PublicInput<'_>) -> ControlReport {
    let mut report = ControlReport {
        mechanism,
        query: input.query,
        at: input.at,
        class: Class::InvalidEvidence,
        proof: None,
        raw: None,
        eligible: vec![],
    };
    if let Some(class) = gate(input) {
        report.class = class;
        return report;
    }
    let Ok(records) = public_records(input) else {
        return report;
    };
    report.eligible = records.clone();
    let Ok(positive) = positives(&records) else {
        return report;
    };
    match mechanism {
        Mechanism::Literal => {
            if let Some(r) = positive
                .iter()
                .find(|r| endpoints(**r) == (input.query.subject(), input.query.object()))
            {
                report.class = Class::Provisional;
                report.proof = Some(EvidenceRefs::Direct(r.engram.id()));
            } else {
                report.class = Class::NoAssociation;
            }
        }
        Mechanism::Symbolic => {
            // Enumeração limitada de no máximo 8×8 pares, sem motor ou Monitor.
            let direct = positive
                .iter()
                .find(|r| endpoints(**r) == (input.query.subject(), input.query.object()));
            let mut refs = direct.map(|r| EvidenceRefs::Direct(r.engram.id()));
            if refs.is_none() {
                'pairs: for first in &positive {
                    for second in &positive {
                        let (x, y) = endpoints(*first);
                        let (v, z) = endpoints(*second);
                        if first.engram.id() != second.engram.id()
                            && x == input.query.subject()
                            && y == v
                            && z == input.query.object()
                            && y != x
                            && y != z
                        {
                            refs = Some(EvidenceRefs::TwoHop {
                                first: first.engram.id(),
                                second: second.engram.id(),
                            });
                            break 'pairs;
                        }
                    }
                }
            }
            match refs {
                None => report.class = Class::MissingEvidence,
                Some(refs) => {
                    report.proof = Some(refs);
                    report.class = match check_proof(input, refs, false) {
                        Ok(_) => Class::Provisional,
                        Err(VerificationIssue::Opposition) => Class::Conflict,
                        Err(_) => Class::InvalidEvidence,
                    };
                }
            }
        }
        Mechanism::NoPropagation | Mechanism::Unmonitored => {
            let mut projection = EngramStore::new(MAX_RECORDS).unwrap();
            for r in records {
                projection
                    .record(r.engram)
                    .expect("IDs distintos e limite verificados");
            }
            let h = Hypothesis::new(
                input.query.scope().context.get(),
                input.query.subject(),
                input.query.object(),
            )
            .unwrap();
            let task = TwoHopTask::new(8, 16, 32).unwrap();
            let result = if mechanism == Mechanism::NoPropagation {
                task.evaluate_without_synapses(&projection, h)
            } else {
                task.evaluate(&projection, h)
            };
            if let Ok(raw) = result {
                report.class = match raw.outcome {
                    Outcome::DirectRecall(id) => {
                        report.proof = Some(EvidenceRefs::Direct(id));
                        Class::Provisional
                    }
                    Outcome::Inferred(p) => {
                        report.proof = Some(EvidenceRefs::TwoHop {
                            first: p.first_fact,
                            second: p.second_fact,
                        });
                        Class::Inferred
                    }
                    Outcome::Unsupported => Class::Unsupported,
                    Outcome::InconclusiveBudget => Class::ResourceLimited,
                    Outcome::InconclusivePropagation(_) => Class::PropagationFailure,
                };
                report.raw = Some(raw);
            }
        }
        Mechanism::Candidate => {} // Não substitui o candidato por um controle.
    }
    report
}

/// Fiscal independente: verifica referências contra conteúdo real, não apenas presença de IDs.
/// `unmonitored` permite relatar cadeia bruta em conflito, sem chamá-la epistemicamente admissível.
pub fn check_proof(
    input: &PublicInput<'_>,
    refs: EvidenceRefs,
    unmonitored: bool,
) -> Result<VerifiedProof, VerificationIssue> {
    if gate(input).is_some() {
        return Err(VerificationIssue::InvalidScope);
    }
    let records = public_records(input)?;
    positives(&records)?;
    super::history::no_conclusions(&input.memory.iter().collect::<Vec<_>>())?;
    let get = |id: EngramId| -> Result<BoundRecord, VerificationIssue> {
        let r = *records
            .iter()
            .find(|r| r.engram.id() == id)
            .ok_or(VerificationIssue::InvalidProof)?;
        if r.metadata.information.statement.polarity() != Polarity::Positive {
            return Err(VerificationIssue::InvalidProof);
        }
        Ok(r)
    };
    let proof = match refs {
        EvidenceRefs::Direct(id) => {
            let r = get(id)?;
            if r.metadata.information.statement != input.query {
                return Err(VerificationIssue::InvalidProof);
            }
            VerifiedProof::Direct(r)
        }
        EvidenceRefs::TwoHop { first, second } => {
            let a = get(first)?;
            let b = get(second)?;
            let (x, y) = endpoints(a);
            let (v, z) = endpoints(b);
            if first == second
                || x != input.query.subject()
                || y != v
                || z != input.query.object()
                || y == x
                || y == z
            {
                return Err(VerificationIssue::InvalidProof);
            }
            if !unmonitored && (negative(&records, x, y) || negative(&records, v, z)) {
                return Err(VerificationIssue::Opposition);
            }
            VerifiedProof::TwoHop {
                first: a,
                second: b,
                via: y,
            }
        }
    };
    if !unmonitored && negative(&records, input.query.subject(), input.query.object()) {
        return Err(VerificationIssue::Opposition);
    }
    Ok(proof)
}
