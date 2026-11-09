//! CIC-I2: instrumentos de engenharia. Nenhum dado V/F ou executor do painel.
pub mod audit;
pub mod controls;
pub mod custody;
pub mod plan;
pub mod recording;
use crate::cognition::concepts::{Attribute, Attributes, Criterion, Label, Observation};
use crate::cognitive_contracts::Scope;
use crate::integration::concepts::BoundExample;
use crate::metacognition::concepts::{Abstention, ConceptDecision};

/// Referência de implementação própria; compartilha somente contratos públicos.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceRule {
    Constant(bool),
    Single(usize),
    Pair(usize, usize),
}
impl ReferenceRule {
    pub(crate) fn public(self) -> Criterion {
        match self {
            Self::Constant(b) => Criterion::constant(b),
            Self::Single(i) => Criterion::attribute(Attribute::ALL[i]),
            Self::Pair(i, j) => {
                Criterion::conjunction(Attribute::ALL[i], Attribute::ALL[j]).unwrap()
            }
        }
    }
    pub(crate) fn label(self, x: Attributes) -> Label {
        let v = x.values();
        Label::from_bool(match self {
            Self::Constant(b) => b,
            Self::Single(i) => v[i],
            Self::Pair(i, j) => v[i] && v[j],
        })
    }
}
pub(crate) fn rules() -> Vec<ReferenceRule> {
    let mut r = vec![
        ReferenceRule::Constant(false),
        ReferenceRule::Constant(true),
    ];
    r.extend((0..6).map(ReferenceRule::Single));
    for i in 0..6 {
        for j in i + 1..6 {
            r.push(ReferenceRule::Pair(i, j));
        }
    }
    r
}
pub(crate) fn fit(examples: &[BoundExample]) -> Vec<ReferenceRule> {
    rules()
        .into_iter()
        .filter(|r| {
            examples
                .iter()
                .all(|e| r.label(e.example.attributes) == e.example.label)
        })
        .collect()
}
pub(crate) fn conflict(examples: &[BoundExample]) -> bool {
    examples.iter().enumerate().any(|(i, b)| {
        examples[..i].iter().any(|a| {
            a.example.attributes == b.example.attributes && a.example.label != b.example.label
        })
    })
}
pub(crate) fn unanimous(h: &[ReferenceRule], x: Observation) -> ConceptDecision {
    if h.is_empty() {
        return ConceptDecision::Abstain(Abstention::ModelClassInadequate);
    }
    let completions = x.completions();
    let mut labels = vec![];
    for c in completions {
        let first = h[0].label(c);
        if h.iter().any(|r| r.label(c) != first) {
            return ConceptDecision::Abstain(Abstention::InsufficientEvidence);
        }
        labels.push(first);
    }
    if labels.iter().all(|l| *l == labels[0]) {
        ConceptDecision::Supported(labels[0])
    } else {
        ConceptDecision::Abstain(Abstention::IncompleteAttributes)
    }
}
pub(crate) fn reference(
    scope: Scope,
    registered: &[Scope],
    examples: &[BoundExample],
    x: Observation,
) -> ConceptDecision {
    if !registered.contains(&scope) {
        return ConceptDecision::Abstain(Abstention::NoModel);
    }
    if conflict(examples) {
        return ConceptDecision::Abstain(Abstention::ContradictoryEvidence);
    }
    if examples.is_empty() {
        return ConceptDecision::Abstain(Abstention::NoEvidence);
    }
    unanimous(&fit(examples), x)
}

#[cfg(test)]
mod tests;
