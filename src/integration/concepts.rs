//! CIC-I1: integração finita de memória factual, indução e julgamento conservador.
//! Não contém docente, partições, regra-alvo, avaliador ou executor experimental.
use crate::cognition::concepts::*;
use crate::cognitive_contracts::{
    ContextId, DeclaredNature, EpisodeId, EpistemicState, Scope, SourceId,
};
use crate::engram::{Cue, Engram, EngramId, EngramStore, EvidenceId, RecordStatus};
use crate::learning::concepts::{self, ExampleEvidence, HypothesisVersion};
use crate::metacognition::concepts::{self as judgment, ConceptDecision};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Refusal {
    Format(InputError),
    UnknownSource,
    MissingEvidence,
    IncompatibleScope,
    HypothesisNotObservation,
    Duplicate,
    OutsideValidity,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Admission {
    Admissible,
    Quarantined(Refusal),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Capacity,
    Phase,
    Duplicate,
    OutsideValidity,
    Scope,
    Binding,
    PendingRevision,
    Input(InputError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    pub records: usize,
    pub contexts: usize,
    pub versions: usize,
    pub events: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            records: 16,
            contexts: 16,
            versions: 32,
            events: 128,
        }
    }
}
impl Limits {
    fn check(self) -> Result<(), Error> {
        let max = Self::default();
        if self.records == 0
            || self.records > max.records
            || self.contexts == 0
            || self.contexts > max.contexts
            || self.versions == 0
            || self.versions > max.versions
            || self.events == 0
            || self.events > max.events
        {
            Err(Error::Capacity)
        } else {
            Ok(())
        }
    }
}

/// Apenas metadados: vetor e rótulo são decodificados do EngramStore, não duplicados aqui.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExampleMetadata {
    pub id: EngramId,
    pub concept: crate::event::NodeId,
    pub scope: Scope,
    pub source: SourceId,
    pub evidence: EvidenceId,
    pub receipt: ReceiptId,
    pub teacher_episode: EpisodeId,
    pub received_at: u64,
    pub incorporated_at: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundExample {
    pub record: Engram,
    pub metadata: ExampleMetadata,
    pub example: Example,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextModel {
    scope: Scope,
    versions: Vec<HypothesisVersion>,
}
impl ContextModel {
    pub fn scope(&self) -> Scope {
        self.scope
    }
    pub fn versions(&self) -> &[HypothesisVersion] {
        &self.versions
    }
    pub fn current(&self) -> &HypothesisVersion {
        self.versions.last().unwrap()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventKind {
    Registered(Scope),
    Received(IncomingExample),
    Interpreted(Example),
    Assessed(Admission),
    Quarantined(Refusal),
    Incorporated(BoundExample),
    Revised { context: ContextId, version: u32 },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceEvent {
    pub at: u64,
    pub kind: EventKind,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Stage {
    Received,
    Interpreted(Example),
    Assessed(Example),
    Incorporated(EngramId),
    Finished,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Pending {
    raw: IncomingExample,
    received_at: u64,
    stage: Stage,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptReport {
    pub episode: EpisodeId,
    pub scope: Scope,
    pub at: u64,
    pub observation: Observation,
    pub version: Option<HypothesisVersion>,
    pub consulted: Vec<BoundExample>,
    pub candidate_predictions: Vec<crate::cognition::concepts::Label>,
    pub decision: ConceptDecision,
    pub state: EpistemicState,
}

/// Log específico CIC; não altera os três episódios permitidos pelo contrato M1.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConceptEpisodeLog {
    entries: Vec<ConceptReport>,
}
impl ConceptEpisodeLog {
    pub fn entries(&self) -> &[ConceptReport] {
        &self.entries
    }
    fn check(&self, id: EpisodeId, at: u64) -> Result<(), Error> {
        if self.entries.len() >= 80 {
            return Err(Error::Capacity);
        }
        if self.entries.iter().any(|r| r.episode == id) {
            return Err(Error::Duplicate);
        }
        if self.entries.last().is_some_and(|r| at < r.at) {
            return Err(Error::OutsideValidity);
        }
        Ok(())
    }
}

pub struct ConceptCoordinator {
    concept: ConceptIdentity,
    sources: Vec<SourceId>,
    limits: Limits,
    memory: EngramStore,
    metadata: Vec<ExampleMetadata>,
    models: Vec<ContextModel>,
    trace: Vec<TraceEvent>,
    pending: Option<Pending>,
}
impl ConceptCoordinator {
    pub fn new(
        concept: ConceptIdentity,
        sources: &[SourceId],
        limits: Limits,
    ) -> Result<Self, Error> {
        limits.check()?;
        if sources.is_empty() || sources.len() > 2 {
            return Err(Error::Capacity);
        }
        if sources
            .iter()
            .enumerate()
            .any(|(i, s)| sources[..i].contains(s))
        {
            return Err(Error::Duplicate);
        }
        Ok(Self {
            concept,
            sources: sources.into(),
            limits,
            memory: EngramStore::new(limits.records).map_err(|_| Error::Capacity)?,
            metadata: vec![],
            models: vec![],
            trace: vec![],
            pending: None,
        })
    }
    pub fn concept(&self) -> &ConceptIdentity {
        &self.concept
    }
    pub fn memory(&self) -> &EngramStore {
        &self.memory
    }
    pub fn metadata(&self) -> &[ExampleMetadata] {
        &self.metadata
    }
    pub fn models(&self) -> &[ContextModel] {
        &self.models
    }
    pub fn trace(&self) -> &[TraceEvent] {
        &self.trace
    }
    fn budget(&self, events: usize, at: u64) -> Result<(), Error> {
        if self.trace.len() + events > self.limits.events {
            return Err(Error::Capacity);
        }
        if self.trace.last().is_some_and(|e| at < e.at) {
            return Err(Error::OutsideValidity);
        }
        Ok(())
    }
    fn version_count(&self) -> usize {
        self.models.iter().map(|m| m.versions.len()).sum()
    }
    fn model_index(&self, scope: Scope) -> Result<usize, Error> {
        self.models
            .iter()
            .position(|m| m.scope == scope)
            .ok_or(Error::Scope)
    }
    fn finished(&self) -> bool {
        self.pending.is_none_or(|p| p.stage == Stage::Finished)
    }
    fn push(&mut self, at: u64, kind: EventKind) {
        self.trace.push(TraceEvent { at, kind });
    }

    /// Autoriza o escopo da tarefa/família pública, nunca uma regra verdadeira do mundo.
    pub fn register_context(&mut self, scope: Scope, at: u64) -> Result<(), Error> {
        self.budget(1, at)?;
        if !self.finished() {
            return Err(Error::Phase);
        }
        if scope.catalog != self.concept.catalog() {
            return Err(Error::Scope);
        }
        if !scope.validity.contains(at) {
            return Err(Error::OutsideValidity);
        }
        if self.models.iter().any(|m| m.scope.context == scope.context) {
            return Err(Error::Duplicate);
        }
        if self.models.len() >= self.limits.contexts || self.version_count() >= self.limits.versions
        {
            return Err(Error::Capacity);
        }
        self.models.push(ContextModel {
            scope,
            versions: vec![HypothesisVersion::initial(at)],
        });
        self.push(at, EventKind::Registered(scope));
        Ok(())
    }

    pub fn receive(&mut self, raw: IncomingExample, at: u64) -> Result<(), Error> {
        self.budget(1, at)?;
        if !self.finished() {
            return Err(Error::Phase);
        }
        if self
            .trace
            .iter()
            .any(|e| matches!(e.kind, EventKind::Received(r) if r.receipt == raw.receipt))
        {
            return Err(Error::Duplicate);
        }
        self.pending = Some(Pending {
            raw,
            received_at: at,
            stage: Stage::Received,
        });
        self.push(at, EventKind::Received(raw));
        Ok(())
    }
    pub fn interpret(&mut self, at: u64) -> Result<Option<Example>, Error> {
        self.budget(1, at)?;
        let mut p = self.pending.ok_or(Error::Phase)?;
        if p.stage != Stage::Received {
            return Err(Error::Phase);
        }
        match p.raw.interpret() {
            Ok(example) => {
                p.stage = Stage::Interpreted(example);
                self.pending = Some(p);
                self.push(at, EventKind::Interpreted(example));
                Ok(Some(example))
            }
            Err(reason) => {
                p.stage = Stage::Finished;
                self.pending = Some(p);
                self.push(at, EventKind::Quarantined(Refusal::Format(reason)));
                Ok(None)
            }
        }
    }

    fn refusal(&self, p: Pending, example: Example, at: u64) -> Result<Option<Refusal>, Error> {
        let raw = p.raw;
        if raw.concept != self.concept.id() || self.model_index(raw.scope).is_err() {
            return Ok(Some(Refusal::IncompatibleScope));
        }
        if !raw.source.is_some_and(|s| self.sources.contains(&s)) {
            return Ok(Some(Refusal::UnknownSource));
        }
        if raw.evidence.is_none() {
            return Ok(Some(Refusal::MissingEvidence));
        }
        if raw.nature != DeclaredNature::Observation {
            return Ok(Some(Refusal::HypothesisNotObservation));
        }
        if raw.observed_at > raw.sent_at
            || raw.sent_at > p.received_at
            || [raw.observed_at, raw.sent_at, p.received_at, at]
                .iter()
                .any(|t| !raw.scope.validity.contains(*t))
        {
            return Ok(Some(Refusal::OutsideValidity));
        }
        for old in self.examples()? {
            if Some(old.metadata.evidence) == raw.evidence {
                return Ok(Some(Refusal::Duplicate));
            }
            if old.metadata.scope == raw.scope {
                if old.example.attributes == example.attributes
                    && old.example.label == example.label
                {
                    return Ok(Some(Refusal::Duplicate));
                }
                if old.example.exemplar == example.exemplar
                    && old.example.attributes != example.attributes
                {
                    return Ok(Some(Refusal::Duplicate));
                }
            }
        }
        Ok(None)
    }
    pub fn assess_admissibility(&mut self, at: u64) -> Result<Admission, Error> {
        self.budget(1, at)?;
        let mut p = self.pending.ok_or(Error::Phase)?;
        let Stage::Interpreted(example) = p.stage else {
            return Err(Error::Phase);
        };
        let admission = match self.refusal(p, example, at)? {
            None => {
                p.stage = Stage::Assessed(example);
                Admission::Admissible
            }
            Some(reason) => {
                p.stage = Stage::Finished;
                Admission::Quarantined(reason)
            }
        };
        self.pending = Some(p);
        self.push(at, EventKind::Assessed(admission));
        Ok(admission)
    }

    pub fn incorporate(&mut self, id: EngramId, at: u64) -> Result<(), Error> {
        // Reserva do evento de revisão e da versão antes de escrever memória factual.
        self.budget(2, at)?;
        let mut p = self.pending.ok_or(Error::Phase)?;
        let Stage::Assessed(example) = p.stage else {
            return Err(Error::Phase);
        };
        if self.refusal(p, example, at)?.is_some() {
            return Err(Error::OutsideValidity);
        }
        if self.memory.len() >= self.limits.records || self.version_count() >= self.limits.versions
        {
            return Err(Error::Capacity);
        }
        if self.memory.get(id).is_ok() {
            return Err(Error::Duplicate);
        }
        let raw = p.raw;
        let record = Engram::new(
            id,
            Cue::new(
                raw.scope.context.get(),
                example.exemplar,
                example.attributes.encode(),
            )
            .map_err(|_| Error::Binding)?,
            example.label.encode(),
            None,
            raw.observed_at,
            raw.evidence,
            RecordStatus::Observed,
        )
        .map_err(|_| Error::Binding)?;
        let metadata = ExampleMetadata {
            id,
            concept: raw.concept,
            scope: raw.scope,
            source: raw.source.unwrap(),
            evidence: raw.evidence.unwrap(),
            receipt: raw.receipt,
            teacher_episode: raw.teacher_episode,
            received_at: p.received_at,
            incorporated_at: at,
        };
        self.memory.record(record).map_err(|_| Error::Binding)?;
        self.metadata.push(metadata);
        p.stage = Stage::Incorporated(id);
        self.pending = Some(p);
        self.push(
            at,
            EventKind::Incorporated(BoundExample {
                record,
                metadata,
                example,
            }),
        );
        Ok(())
    }

    pub fn revise(&mut self, at: u64) -> Result<&HypothesisVersion, Error> {
        self.budget(1, at)?;
        let mut p = self.pending.ok_or(Error::Phase)?;
        let Stage::Incorporated(id) = p.stage else {
            return Err(Error::Phase);
        };
        if !p.raw.scope.validity.contains(at) {
            return Err(Error::OutsideValidity);
        }
        let bound = self
            .examples()?
            .into_iter()
            .find(|b| b.record.id() == id)
            .ok_or(Error::Binding)?;
        let i = self.model_index(p.raw.scope)?;
        let next = concepts::revise(
            self.models[i].current(),
            bound.example,
            ExampleEvidence {
                record: id,
                evidence: bound.metadata.evidence,
            },
            at,
        );
        let version = next.number;
        self.models[i].versions.push(next);
        p.stage = Stage::Finished;
        self.pending = Some(p);
        self.push(
            at,
            EventKind::Revised {
                context: p.raw.scope.context,
                version,
            },
        );
        Ok(self.models[i].current())
    }

    /// Decodificação e vínculo reais; não existe cache paralelo de rótulos de consulta.
    pub fn examples(&self) -> Result<Vec<BoundExample>, Error> {
        if self.metadata.len() != self.memory.len() {
            return Err(Error::Binding);
        }
        self.metadata
            .iter()
            .map(|m| {
                let record = self.memory.get(m.id).map_err(|_| Error::Binding)?;
                if record.status() != RecordStatus::Observed
                    || record.origin().is_some()
                    || record.evidence() != Some(m.evidence)
                    || record.cue().context() != m.scope.context.get()
                    || m.concept != self.concept.id()
                    || self.model_index(m.scope).is_err()
                    || !self.sources.contains(&m.source)
                    || record.observed_at() > m.received_at
                    || m.received_at > m.incorporated_at
                    || [record.observed_at(), m.received_at, m.incorporated_at]
                        .iter()
                        .any(|t| !m.scope.validity.contains(*t))
                {
                    return Err(Error::Binding);
                }
                let example = Example {
                    exemplar: record.cue().source(),
                    attributes: Attributes::decode(record.cue().pattern()).map_err(Error::Input)?,
                    label: Label::decode(record.value()).map_err(Error::Input)?,
                };
                Ok(BoundExample {
                    record,
                    metadata: *m,
                    example,
                })
            })
            .collect()
    }

    /// &self: somente o log externo limitado pode crescer. Nenhum acesso mutável ao núcleo.
    pub fn query(
        &self,
        episode: EpisodeId,
        scope: Scope,
        observation: Observation,
        at: u64,
        log: &mut ConceptEpisodeLog,
    ) -> Result<ConceptReport, Error> {
        log.check(episode, at)?;
        if scope.catalog != self.concept.catalog() {
            return Err(Error::Scope);
        }
        if !scope.validity.contains(at) || self.trace.last().is_some_and(|e| at < e.at) {
            return Err(Error::OutsideValidity);
        }
        if self
            .pending
            .is_some_and(|p| matches!(p.stage, Stage::Incorporated(_)))
        {
            return Err(Error::PendingRevision);
        }
        if self
            .models
            .iter()
            .any(|m| m.scope.context == scope.context && m.scope != scope)
        {
            return Err(Error::Scope);
        }
        let version = self
            .models
            .iter()
            .find(|m| m.scope == scope)
            .map(|m| m.current());
        let consulted: Vec<_> = self
            .examples()?
            .into_iter()
            .filter(|b| b.metadata.scope == scope)
            .collect();
        if consulted
            .iter()
            .any(|b| b.metadata.incorporated_at > at || !b.metadata.scope.validity.contains(at))
        {
            return Err(Error::OutsideValidity);
        }
        let contradictory = consulted.iter().enumerate().any(|(i, b)| {
            consulted[..i].iter().any(|a| {
                a.example.attributes == b.example.attributes && a.example.label != b.example.label
            })
        });
        let decision = judgment::review(version, contradictory, observation);
        let candidate_predictions = version
            .and_then(|v| v.candidate)
            .map(|c| {
                observation
                    .completions()
                    .into_iter()
                    .map(|a| c.evaluate(a))
                    .collect()
            })
            .unwrap_or_default();
        let report = ConceptReport {
            episode,
            scope,
            at,
            observation,
            version: version.cloned(),
            consulted,
            candidate_predictions,
            decision,
            state: match decision {
                ConceptDecision::Supported(_) => EpistemicState::ProvisionalKnowledge,
                _ => EpistemicState::Hypothesis,
            },
        };
        log.entries.push(report.clone());
        Ok(report)
    }
}

#[cfg(test)]
mod tests;
