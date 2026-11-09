//! Filtragem simbólica explícita: não altera pesos, não recebe regra-alvo.
use crate::cognition::concepts::{family, Criterion, Example};
use crate::engram::{EngramId, EvidenceId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExampleEvidence {
    pub record: EngramId,
    pub evidence: EvidenceId,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Elimination {
    pub criterion: Criterion,
    pub because: ExampleEvidence,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HypothesisVersion {
    pub number: u32,
    pub previous: Option<u32>,
    pub at: u64,
    pub criteria: Vec<Criterion>,
    pub candidate: Option<Criterion>,
    pub eliminated: Vec<Elimination>,
    pub support: Vec<ExampleEvidence>,
}
impl HypothesisVersion {
    pub(crate) fn initial(at: u64) -> Self {
        let criteria = family();
        Self {
            number: 0,
            previous: None,
            at,
            candidate: criteria.first().copied(),
            criteria,
            eliminated: vec![],
            support: vec![],
        }
    }
}

/// Chamado somente pelo ato de revisão do coordenador, após incorporação factual.
pub(crate) fn revise(
    previous: &HypothesisVersion,
    example: Example,
    because: ExampleEvidence,
    at: u64,
) -> HypothesisVersion {
    let mut next = HypothesisVersion {
        number: previous.number + 1,
        previous: Some(previous.number),
        at,
        criteria: vec![],
        candidate: None,
        eliminated: vec![],
        support: previous.support.clone(),
    };
    next.support.push(because);
    for criterion in &previous.criteria {
        if criterion.evaluate(example.attributes) == example.label {
            next.criteria.push(*criterion);
        } else {
            next.eliminated.push(Elimination {
                criterion: *criterion,
                because,
            });
        }
    }
    // Filtro estável da família em ordem de complexidade e desempate público.
    next.candidate = next.criteria.first().copied();
    next
}
