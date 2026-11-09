//! Controles usam somente fatos públicos correntes. Não chamam filtro/julgador CIC-I1.
use super::audit::{AuditError, AuditPolicy, PublicSnapshot};
use super::plan::Mechanism;
use super::{conflict, fit, reference, unanimous, ReferenceRule};
use crate::cognition::concepts::{Label, Observation};
use crate::cognitive_contracts::Scope;
use crate::integration::concepts::BoundExample;
use crate::metacognition::concepts::{Abstention, ConceptDecision};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlReport {
    pub mechanism: Mechanism,
    pub decision: Option<ConceptDecision>,
    pub no_literal_association: bool,
    pub training_errors: Option<usize>,
}
pub struct PublicFacts<'a> {
    registered: Vec<Scope>,
    examples: Vec<BoundExample>,
    scope: Scope,
    observation: Observation,
    _snapshot: &'a PublicSnapshot,
}
impl<'a> PublicFacts<'a> {
    pub fn new(
        snapshot: &'a PublicSnapshot,
        scope: Scope,
        observation: Observation,
        at: u64,
        policy: &AuditPolicy,
    ) -> Result<Self, AuditError> {
        if super::audit::history(snapshot, policy)? {
            return Err(AuditError::History);
        }
        snapshot.query_scope(scope, at)?;
        Ok(Self {
            registered: snapshot.models.iter().map(|m| m.scope).collect(),
            examples: snapshot
                .bound()?
                .into_iter()
                .filter(|e| e.metadata.scope == scope)
                .collect(),
            scope,
            observation,
            _snapshot: snapshot,
        })
    }
}
fn report(mechanism: Mechanism, decision: ConceptDecision) -> ControlReport {
    ControlReport {
        mechanism,
        decision: Some(decision),
        no_literal_association: false,
        training_errors: None,
    }
}
pub fn symbolic(input: &PublicFacts<'_>) -> ControlReport {
    report(
        Mechanism::Symbolic,
        reference(
            input.scope,
            &input.registered,
            &input.examples,
            input.observation,
        ),
    )
}
pub fn literal(input: &PublicFacts<'_>) -> ControlReport {
    if !input.registered.contains(&input.scope) {
        return report(
            Mechanism::Literal,
            ConceptDecision::Abstain(Abstention::NoModel),
        );
    }
    if conflict(&input.examples) {
        return report(
            Mechanism::Literal,
            ConceptDecision::Abstain(Abstention::ContradictoryEvidence),
        );
    }
    let mut labels = vec![];
    for c in input.observation.completions() {
        let Some(e) = input.examples.iter().find(|e| e.example.attributes == c) else {
            return ControlReport {
                mechanism: Mechanism::Literal,
                decision: None,
                no_literal_association: true,
                training_errors: None,
            };
        };
        labels.push(e.example.label);
    }
    report(
        Mechanism::Literal,
        if labels.iter().all(|l| *l == labels[0]) {
            ConceptDecision::Supported(labels[0])
        } else {
            ConceptDecision::Abstain(Abstention::IncompleteAttributes)
        },
    )
}
pub fn trivial(input: &PublicFacts<'_>) -> ControlReport {
    let mechanism = Mechanism::Trivial;
    if !input.registered.contains(&input.scope) {
        return report(mechanism, ConceptDecision::Abstain(Abstention::NoModel));
    }
    if conflict(&input.examples) {
        return report(
            mechanism,
            ConceptDecision::Abstain(Abstention::ContradictoryEvidence),
        );
    }
    if input.examples.is_empty() {
        return report(mechanism, ConceptDecision::Abstain(Abstention::NoEvidence));
    }
    let (errors, index) = (0..6)
        .map(|i| {
            (
                input
                    .examples
                    .iter()
                    .filter(|e| {
                        ReferenceRule::Single(i).label(e.example.attributes) != e.example.label
                    })
                    .count(),
                i,
            )
        })
        .min()
        .unwrap();
    let mut r = report(
        mechanism,
        unanimous(&[ReferenceRule::Single(index)], input.observation),
    );
    r.training_errors = Some(errors);
    r
}

/// Teto privilegiado: só há regra no controle, nunca no candidato ou PublicFacts.
pub struct ProvidedRuleControl {
    scope: Scope,
    rule: ReferenceRule,
}
impl ProvidedRuleControl {
    pub fn new(scope: Scope, rule: ReferenceRule) -> Result<Self, AuditError> {
        if !super::rules().contains(&rule) {
            return Err(AuditError::Criteria);
        }
        Ok(Self { scope, rule })
    }
    pub fn evaluate(
        &self,
        input: &PublicFacts<'_>,
        block: super::plan::Block,
    ) -> Result<ControlReport, AuditError> {
        if input.scope != self.scope || !block.ceiling() {
            return Err(AuditError::OutsideRecut);
        }
        Ok(report(
            Mechanism::Provided,
            unanimous(&[self.rule], input.observation),
        ))
    }
}

pub struct FrozenControl {
    scope: Scope,
    frozen: Option<Vec<ReferenceRule>>,
}
impl FrozenControl {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            frozen: None,
        }
    }
    /// T0 possui exatamente quatro exemplos; não copia H calculado pelo candidato.
    pub fn at_t0(input: &PublicFacts<'_>) -> Result<Self, AuditError> {
        if input.examples.len() != 4 {
            return Err(AuditError::History);
        }
        Ok(Self {
            scope: input.scope,
            frozen: Some(fit(&input.examples)),
        })
    }
    pub fn evaluate(&self, input: &PublicFacts<'_>) -> ControlReport {
        if input.scope != self.scope {
            return report(
                Mechanism::Frozen,
                ConceptDecision::Abstain(Abstention::NoModel),
            );
        }
        if conflict(&input.examples) {
            return report(
                Mechanism::Frozen,
                ConceptDecision::Abstain(Abstention::ContradictoryEvidence),
            );
        }
        report(
            Mechanism::Frozen,
            match &self.frozen {
                Some(h) => unanimous(h, input.observation),
                None => ConceptDecision::Abstain(Abstention::NoEvidence),
            },
        )
    }
}

/// Separado do Pass contratual: abstenção não é classificação correta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Classification {
    Correct,
    Incorrect,
    Abstained,
    AmbiguousTruth,
}
pub fn classification(decision: Option<ConceptDecision>, truth: Option<Label>) -> Classification {
    match decision {
        Some(ConceptDecision::Supported(l)) => match truth {
            Some(t) if l == t => Classification::Correct,
            Some(_) => Classification::Incorrect,
            None => Classification::AmbiguousTruth,
        },
        _ => Classification::Abstained,
    }
}
