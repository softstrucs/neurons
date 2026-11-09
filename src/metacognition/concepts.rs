//! Julgamento conservador para hipóteses conceituais, distinto da prova de dois passos.
use crate::cognition::concepts::{Label, Observation};
use crate::learning::concepts::HypothesisVersion;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Abstention {
    NoModel,
    NoEvidence,
    InsufficientEvidence,
    IncompleteAttributes,
    ContradictoryEvidence,
    ModelClassInadequate,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConceptDecision {
    Supported(Label),
    Abstain(Abstention),
}

pub(crate) fn review(
    version: Option<&HypothesisVersion>,
    contradictory: bool,
    observation: Observation,
) -> ConceptDecision {
    if contradictory {
        return ConceptDecision::Abstain(Abstention::ContradictoryEvidence);
    }
    let Some(version) = version else {
        return ConceptDecision::Abstain(Abstention::NoModel);
    };
    if version.criteria.is_empty() {
        return ConceptDecision::Abstain(Abstention::ModelClassInadequate);
    }
    if version.support.is_empty() {
        return ConceptDecision::Abstain(Abstention::NoEvidence);
    }
    let mut completion_labels = Vec::new();
    for completion in observation.completions() {
        let label = version.criteria[0].evaluate(completion);
        if version
            .criteria
            .iter()
            .any(|c| c.evaluate(completion) != label)
        {
            return ConceptDecision::Abstain(Abstention::InsufficientEvidence);
        }
        completion_labels.push(label);
    }
    if completion_labels.iter().any(|l| *l != completion_labels[0]) {
        ConceptDecision::Abstain(Abstention::IncompleteAttributes)
    } else {
        ConceptDecision::Supported(completion_labels[0])
    }
}
