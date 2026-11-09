//! Fiscal de conteúdo, fases e versões. Não chama revise/review/evaluate do candidato.
use super::{fit, reference, rules, ReferenceRule};
use crate::cognition::concepts::{
    Attributes, ConceptIdentity, Example, IncomingExample, Label, Observation,
};
use crate::cognitive_contracts::{DeclaredNature, EpistemicState, Scope, SourceId};
use crate::engram::{Cue, Engram, RecordStatus};
use crate::integration::concepts::{
    Admission, BoundExample, ConceptCoordinator, ConceptReport, EventKind, ExampleMetadata, Limits,
    Refusal, TraceEvent,
};
use crate::learning::concepts::{Elimination, ExampleEvidence, HypothesisVersion};
use crate::metacognition::concepts::ConceptDecision;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditError {
    Bounds,
    History,
    Timing,
    Binding,
    Scope,
    Criteria,
    Proof,
    Mutation,
    Promotion,
    Leakage,
    OutsideRecut,
    Reserved,
    Missing,
    Duplicate,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelImage {
    pub scope: Scope,
    pub versions: Vec<HypothesisVersion>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicSnapshot {
    pub concept: ConceptIdentity,
    pub memory: Vec<Engram>,
    pub metadata: Vec<ExampleMetadata>,
    pub models: Vec<ModelImage>,
    pub trace: Vec<TraceEvent>,
    pub memory_capacity: usize,
}
impl PublicSnapshot {
    pub fn capture(c: &ConceptCoordinator) -> Self {
        Self {
            concept: c.concept().clone(),
            memory: c.memory().iter().collect(),
            metadata: c.metadata().into(),
            models: c
                .models()
                .iter()
                .map(|m| ModelImage {
                    scope: m.scope(),
                    versions: m.versions().into(),
                })
                .collect(),
            trace: c.trace().into(),
            memory_capacity: c.memory().capacity(),
        }
    }
    pub fn bound(&self) -> Result<Vec<BoundExample>, AuditError> {
        if self.memory.len() != self.metadata.len() {
            return Err(AuditError::Binding);
        }
        self.memory
            .iter()
            .map(|record| {
                let found: Vec<_> = self
                    .metadata
                    .iter()
                    .filter(|m| m.id == record.id())
                    .collect();
                if found.len() != 1 {
                    return Err(AuditError::Binding);
                }
                let m = *found[0];
                if record.status() != RecordStatus::Observed {
                    return Err(AuditError::Promotion);
                }
                if record.origin().is_some()
                    || record.evidence() != Some(m.evidence)
                    || record.cue().context() != m.scope.context.get()
                    || m.concept != self.concept.id()
                    || !(1..=64).contains(&record.cue().pattern())
                {
                    return Err(AuditError::Binding);
                }
                let n = record.cue().pattern() - 1;
                let attributes =
                    Attributes::new(std::array::from_fn(|i| u8::from(n & (1 << i) != 0)))
                        .map_err(|_| AuditError::Binding)?;
                let label = match record.value() {
                    1 => Label::Positive,
                    -1 => Label::Negative,
                    _ => return Err(AuditError::Binding),
                };
                Ok(BoundExample {
                    record: *record,
                    metadata: m,
                    example: Example {
                        exemplar: record.cue().source(),
                        attributes,
                        label,
                    },
                })
            })
            .collect()
    }
    pub fn query_scope(&self, scope: Scope, at: u64) -> Result<(), AuditError> {
        if scope.catalog != self.concept.catalog()
            || self
                .models
                .iter()
                .any(|m| m.scope.context == scope.context && m.scope != scope)
        {
            return Err(AuditError::Scope);
        }
        if !scope.validity.contains(at) || self.trace.last().is_some_and(|e| e.at > at) {
            return Err(AuditError::Timing);
        }
        Ok(())
    }
}
pub struct AuditPolicy {
    concept: ConceptIdentity,
    sources: Vec<SourceId>,
    limits: Limits,
}
impl AuditPolicy {
    pub fn new(
        concept: ConceptIdentity,
        sources: &[SourceId],
        limits: Limits,
    ) -> Result<Self, AuditError> {
        if sources.is_empty()
            || sources.len() > 2
            || sources
                .iter()
                .enumerate()
                .any(|(i, s)| sources[..i].contains(s))
            || limits.records == 0
            || limits.records > 16
            || limits.contexts == 0
            || limits.contexts > 16
            || limits.versions == 0
            || limits.versions > 32
            || limits.events == 0
            || limits.events > 128
        {
            return Err(AuditError::Bounds);
        }
        Ok(Self {
            concept,
            sources: sources.into(),
            limits,
        })
    }
}
fn interpreted(raw: IncomingExample) -> Result<Example, Refusal> {
    use crate::cognition::concepts::InputError;
    if raw.attributes.iter().flatten().any(|v| *v > 1) {
        return Err(Refusal::Format(InputError::InvalidAttribute));
    }
    let unknown = raw.attributes.iter().filter(|a| a.is_none()).count();
    if unknown > 1 {
        return Err(Refusal::Format(InputError::TooManyUnknowns));
    }
    if unknown == 1 {
        return Err(Refusal::Format(InputError::IncompleteExample));
    }
    Ok(Example {
        exemplar: raw.exemplar,
        attributes: Attributes::new(raw.attributes.map(|v| v.unwrap())).unwrap(),
        label: raw.label,
    })
}
fn admission(
    p: &AuditPolicy,
    models: &[ModelImage],
    facts: &[BoundExample],
    raw: IncomingExample,
    example: Example,
    received: u64,
    at: u64,
) -> Admission {
    let reason = if raw.concept != p.concept.id() || !models.iter().any(|m| m.scope == raw.scope) {
        Some(Refusal::IncompatibleScope)
    } else if !raw.source.is_some_and(|s| p.sources.contains(&s)) {
        Some(Refusal::UnknownSource)
    } else if raw.evidence.is_none() {
        Some(Refusal::MissingEvidence)
    } else if raw.nature != DeclaredNature::Observation {
        Some(Refusal::HypothesisNotObservation)
    } else if raw.observed_at > raw.sent_at
        || raw.sent_at > received
        || [raw.observed_at, raw.sent_at, received, at]
            .iter()
            .any(|t| !raw.scope.validity.contains(*t))
    {
        Some(Refusal::OutsideValidity)
    } else if facts.iter().any(|b| {
        Some(b.metadata.evidence) == raw.evidence
            || (b.metadata.scope == raw.scope
                && ((b.example.attributes == example.attributes
                    && b.example.label == example.label)
                    || (b.example.exemplar == example.exemplar
                        && b.example.attributes != example.attributes)))
    }) {
        Some(Refusal::Duplicate)
    } else {
        None
    };
    reason
        .map(Admission::Quarantined)
        .unwrap_or(Admission::Admissible)
}
fn initial(at: u64) -> HypothesisVersion {
    let criteria: Vec<_> = rules().into_iter().map(ReferenceRule::public).collect();
    HypothesisVersion {
        number: 0,
        previous: None,
        at,
        candidate: criteria.first().copied(),
        criteria,
        eliminated: vec![],
        support: vec![],
    }
}
fn next(
    previous: &HypothesisVersion,
    facts: &[BoundExample],
    last: BoundExample,
    at: u64,
) -> HypothesisVersion {
    let criteria: Vec<_> = fit(facts).into_iter().map(ReferenceRule::public).collect();
    let because = ExampleEvidence {
        record: last.record.id(),
        evidence: last.metadata.evidence,
    };
    let eliminated = previous
        .criteria
        .iter()
        .filter(|c| !criteria.contains(c))
        .map(|c| Elimination {
            criterion: *c,
            because,
        })
        .collect();
    let support = facts
        .iter()
        .map(|b| ExampleEvidence {
            record: b.record.id(),
            evidence: b.metadata.evidence,
        })
        .collect();
    HypothesisVersion {
        number: previous.number + 1,
        previous: Some(previous.number),
        at,
        candidate: criteria.first().copied(),
        criteria,
        eliminated,
        support,
    }
}
#[derive(Clone, Copy)]
struct Pending {
    raw: IncomingExample,
    received: u64,
    phase: u8,
    example: Option<Example>,
    last: Option<BoundExample>,
}

/// Prefixos são auditáveis; true significa incorporação ainda não revisada.
pub fn history(s: &PublicSnapshot, p: &AuditPolicy) -> Result<bool, AuditError> {
    if s.concept != p.concept || s.memory_capacity != p.limits.records {
        return Err(AuditError::Binding);
    }
    if s.memory.len() > p.limits.records
        || s.models.len() > p.limits.contexts
        || s.trace.len() > p.limits.events
        || s.models.iter().map(|m| m.versions.len()).sum::<usize>() > p.limits.versions
    {
        return Err(AuditError::Bounds);
    }
    let mut models: Vec<ModelImage> = vec![];
    let mut facts: Vec<BoundExample> = vec![];
    let mut pending: Option<Pending> = None;
    let mut receipts = vec![];
    let mut previous_time = 0;
    for (index, e) in s.trace.iter().enumerate() {
        if e.at < previous_time {
            return Err(AuditError::Timing);
        }
        previous_time = e.at;
        match e.kind.clone() {
            EventKind::Registered(scope) => {
                if pending.is_some()
                    || scope.catalog != p.concept.catalog()
                    || !scope.validity.contains(e.at)
                    || models.iter().any(|m| m.scope.context == scope.context)
                {
                    return Err(AuditError::History);
                }
                if models.len() >= p.limits.contexts
                    || models.iter().map(|m| m.versions.len()).sum::<usize>() >= p.limits.versions
                {
                    return Err(AuditError::Bounds);
                }
                models.push(ModelImage {
                    scope,
                    versions: vec![initial(e.at)],
                });
            }
            EventKind::Received(raw) => {
                if pending.is_some() {
                    return Err(AuditError::History);
                }
                if receipts.contains(&raw.receipt) {
                    return Err(AuditError::Duplicate);
                }
                receipts.push(raw.receipt);
                pending = Some(Pending {
                    raw,
                    received: e.at,
                    phase: 0,
                    example: None,
                    last: None,
                });
            }
            EventKind::Interpreted(example) => {
                let q = pending.as_mut().ok_or(AuditError::History)?;
                if q.phase != 0 || interpreted(q.raw) != Ok(example) {
                    return Err(AuditError::Proof);
                }
                q.example = Some(example);
                q.phase = 1;
            }
            EventKind::Quarantined(reason) => {
                let q = pending.ok_or(AuditError::History)?;
                if q.phase != 0 || interpreted(q.raw) != Err(reason) {
                    return Err(AuditError::Proof);
                }
                pending = None;
            }
            EventKind::Assessed(value) => {
                let q = pending.as_mut().ok_or(AuditError::History)?;
                if q.phase != 1
                    || admission(
                        p,
                        &models,
                        &facts,
                        q.raw,
                        q.example.unwrap(),
                        q.received,
                        e.at,
                    ) != value
                {
                    return Err(AuditError::Proof);
                }
                if value == Admission::Admissible {
                    q.phase = 2;
                } else {
                    pending = None;
                }
            }
            EventKind::Incorporated(b) => {
                let q = pending.as_mut().ok_or(AuditError::History)?;
                if q.phase != 2
                    || admission(
                        p,
                        &models,
                        &facts,
                        q.raw,
                        q.example.unwrap(),
                        q.received,
                        e.at,
                    ) != Admission::Admissible
                {
                    return Err(AuditError::Proof);
                }
                if facts.len() >= p.limits.records
                    || index + 2 > p.limits.events
                    || models.iter().map(|m| m.versions.len()).sum::<usize>() >= p.limits.versions
                {
                    return Err(AuditError::Bounds);
                }
                if facts.iter().any(|a| a.record.id() == b.record.id()) {
                    return Err(AuditError::Duplicate);
                }
                let raw = q.raw;
                let example = q.example.unwrap();
                let pattern = 1 + example
                    .attributes
                    .values()
                    .iter()
                    .enumerate()
                    .fold(0, |n, (i, v)| n + (i32::from(*v) << i));
                let record = Engram::new(
                    b.record.id(),
                    Cue::new(raw.scope.context.get(), raw.exemplar, pattern).unwrap(),
                    if raw.label == Label::Positive { 1 } else { -1 },
                    None,
                    raw.observed_at,
                    raw.evidence,
                    RecordStatus::Observed,
                )
                .unwrap();
                let metadata = ExampleMetadata {
                    id: b.record.id(),
                    concept: raw.concept,
                    scope: raw.scope,
                    source: raw.source.unwrap(),
                    evidence: raw.evidence.unwrap(),
                    receipt: raw.receipt,
                    teacher_episode: raw.teacher_episode,
                    received_at: q.received,
                    incorporated_at: e.at,
                };
                if b != (BoundExample {
                    record,
                    metadata,
                    example,
                }) {
                    return Err(AuditError::Binding);
                }
                facts.push(b);
                q.last = Some(b);
                q.phase = 3;
            }
            EventKind::Revised { context, version } => {
                let q = pending.ok_or(AuditError::History)?;
                if q.phase != 3
                    || q.raw.scope.context != context
                    || !q.raw.scope.validity.contains(e.at)
                {
                    return Err(AuditError::History);
                }
                let m = models
                    .iter_mut()
                    .find(|m| m.scope == q.raw.scope)
                    .ok_or(AuditError::Scope)?;
                let local: Vec<_> = facts
                    .iter()
                    .copied()
                    .filter(|b| b.metadata.scope == m.scope)
                    .collect();
                let v = next(m.versions.last().unwrap(), &local, q.last.unwrap(), e.at);
                if v.number != version {
                    return Err(AuditError::History);
                }
                m.versions.push(v);
                pending = None;
            }
        }
    }
    if s.memory != facts.iter().map(|b| b.record).collect::<Vec<_>>()
        || s.metadata != facts.iter().map(|b| b.metadata).collect::<Vec<_>>()
    {
        return Err(AuditError::Binding);
    }
    if s.models != models {
        return Err(AuditError::Criteria);
    }
    s.bound()?;
    Ok(pending.is_some_and(|q| q.phase == 3))
}

pub fn expected(
    s: &PublicSnapshot,
    scope: Scope,
    observation: Observation,
    at: u64,
) -> Result<ConceptDecision, AuditError> {
    s.query_scope(scope, at)?;
    let local: Vec<_> = s
        .bound()?
        .into_iter()
        .filter(|e| e.metadata.scope == scope)
        .collect();
    Ok(reference(
        scope,
        &s.models.iter().map(|m| m.scope).collect::<Vec<_>>(),
        &local,
        observation,
    ))
}
pub fn query(
    before: &PublicSnapshot,
    after: &PublicSnapshot,
    old_log: &[ConceptReport],
    new_log: &[ConceptReport],
    r: &ConceptReport,
    p: &AuditPolicy,
) -> Result<(), AuditError> {
    if before != after {
        return Err(AuditError::Mutation);
    }
    if history(before, p)? {
        return Err(AuditError::History);
    }
    if old_log.len() >= 80
        || new_log.len() != old_log.len() + 1
        || !new_log.starts_with(old_log)
        || new_log.last() != Some(r)
    {
        return Err(AuditError::History);
    }
    if old_log.iter().any(|a| a.episode == r.episode) {
        return Err(AuditError::Duplicate);
    }
    if old_log.last().is_some_and(|a| a.at > r.at) {
        return Err(AuditError::Timing);
    }
    let current = before
        .models
        .iter()
        .find(|m| m.scope == r.scope)
        .and_then(|m| m.versions.last());
    if r.version.as_ref() != current {
        return Err(AuditError::Criteria);
    }
    let consulted: Vec<_> = before
        .bound()?
        .into_iter()
        .filter(|b| b.metadata.scope == r.scope)
        .collect();
    if consulted != r.consulted {
        return Err(AuditError::Proof);
    }
    if r.decision != expected(before, r.scope, r.observation, r.at)? {
        return Err(AuditError::Proof);
    }
    let state = if matches!(r.decision, ConceptDecision::Supported(_)) {
        EpistemicState::ProvisionalKnowledge
    } else {
        EpistemicState::Hypothesis
    };
    if r.state != state {
        return Err(AuditError::Promotion);
    }
    let candidate = current
        .and_then(|v| v.candidate)
        .and_then(|c| rules().into_iter().find(|r| r.public() == c));
    let predictions: Vec<_> = candidate
        .map(|c| {
            r.observation
                .completions()
                .into_iter()
                .map(|a| c.label(a))
                .collect()
        })
        .unwrap_or_default();
    if r.candidate_predictions != predictions {
        return Err(AuditError::Proof);
    }
    Ok(())
}

pub fn prefix(before: &PublicSnapshot, after: &PublicSnapshot) -> Result<(), AuditError> {
    if !after.trace.starts_with(&before.trace)
        || !after.memory.starts_with(&before.memory)
        || !after.metadata.starts_with(&before.metadata)
    {
        return Err(AuditError::History);
    }
    for old in &before.models {
        if !after
            .models
            .iter()
            .any(|m| m.scope == old.scope && m.versions.starts_with(&old.versions))
        {
            return Err(AuditError::History);
        }
    }
    Ok(())
}
