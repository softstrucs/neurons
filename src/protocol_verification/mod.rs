//! M4: preparação e verificação local do M0. Não contém executor da matriz M5.
//! Rótulos, extensões e expectativas pertencem ao harness/avaliador, nunca ao candidato.
use crate::cognition::InferenceReport;
use crate::cognitive_contracts::SourceId;
use crate::cognitive_contracts::Statement;
use crate::engram::EngramStore;
use crate::integration::query::{BoundRecord, RecordMetadata, RuleAuthorization};
use crate::metacognition::EvidenceRefs;

pub mod controls;
pub mod evaluator;
pub mod fixtures;
pub mod history;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mechanism {
    Candidate,
    Literal,
    Symbolic,
    NoPropagation,
    Unmonitored,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Class {
    Provisional,
    MissingEvidence,
    Conflict,
    OutsideRule,
    NoAssociation,
    Unsupported,
    Inferred,
    InvalidEvidence,
    ResourceLimited,
    PropagationFailure,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationIssue {
    Bounds,
    Metadata,
    Duplicate,
    InvalidScope,
    InvalidProof,
    Opposition,
    StoredConclusion,
    Mutation,
    History,
    Timing,
    Leakage,
    UnexpectedClass,
    UnexpectedMemory,
    UnexpectedProof,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Unavailable {
    MissingDecision,
    OutsideRecut,
    ReferenceUnavailable,
    ResourceInstrumentUnavailable,
    BudgetExhausted,
    PropagationFailure,
    InvalidEvidence,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Verdict {
    Pass,
    Fail(VerificationIssue),
    NotEvaluable(Unavailable),
}
impl Verdict {
    pub fn is_pass(&self) -> bool {
        matches!(self, Self::Pass)
    }
}

pub struct PublicInput<'a> {
    pub memory: &'a EngramStore,
    pub metadata: &'a [RecordMetadata],
    pub authorization: RuleAuthorization,
    pub known_sources: &'a [SourceId],
    pub query: Statement,
    pub at: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlReport {
    pub mechanism: Mechanism,
    pub query: Statement,
    pub at: u64,
    pub class: Class,
    pub proof: Option<EvidenceRefs>,
    pub raw: Option<InferenceReport>,
    pub eligible: Vec<BoundRecord>,
}

/// Contagens prospectivas de M0 §8.2; são plano, não resultados nem loop de execução.
pub const PLANNED_EVALUATIONS: [(Mechanism, usize); 5] = [
    (Mechanism::Candidate, 42),
    (Mechanism::Symbolic, 42),
    (Mechanism::Literal, 42),
    (Mechanism::NoPropagation, 12),
    (Mechanism::Unmonitored, 12),
];
/// Recuperações iniciais F1/F2 propostas, fora das 150 avaliações; não executadas como painel.
pub const PLANNED_LITERAL_CHECKS: usize = 28;

#[cfg(test)]
mod tests;

/// CIC-I2: instrumentos conceituais separados do candidato, sem executor final.
pub mod conceptual;
