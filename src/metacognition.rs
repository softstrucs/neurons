//! Monitoramento delimitado de resultados e evidências acessíveis da tarefa de dois passos.

use crate::cognition::{CognitionError, Hypothesis, InferenceReport, Outcome, Proof, TwoHopTask};
use crate::engram::{Cue, EngramId, EngramStore, RecordStatus};
use crate::event::NodeId;

const MAX_ATTEMPTS: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReviewError {
    InvalidAttemptBudget,
    TooManyCandidates,
    InvalidCandidateContext,
    DuplicateCandidateContext,
    InvalidHypothesis(CognitionError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Assessment {
    Sufficient,
    MissingEvidence,
    ContradictoryEvidence,
    ResourceLimited,
    PropagationFailure,
    InvalidEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AbstainReason {
    MissingEvidence,
    ContradictoryEvidence,
    ResourceLimited,
    PropagationFailure,
    InvalidEvidence,
    ReviewBudget,
    MultipleSupportedContexts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RevisionReason {
    MissingEvidence,
    ContradictoryEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceRefs {
    Direct(EngramId),
    TwoHop { first: EngramId, second: EngramId },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Conflict {
    NegativeConclusion {
        negative: EngramId,
    },
    NegativePremise {
        positive: EngramId,
        negative: EngramId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    Respond {
        hypothesis: Hypothesis,
        evidence: EvidenceRefs,
        revision: Option<RevisionReason>,
    },
    Abstain(AbstainReason),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AttemptLog {
    pub hypothesis: Hypothesis,
    pub inference: Option<InferenceReport>,
    pub evaluation_error: Option<CognitionError>,
    pub assessment: Assessment,
    pub evidence: Option<EvidenceRefs>,
    pub conflict: Option<Conflict>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewReport {
    pub original: Hypothesis,
    pub decision: Decision,
    pub attempts: Vec<AttemptLog>,
}

impl ReviewReport {
    pub fn attempt_count(&self) -> usize {
        self.attempts.len()
    }
}

/// Examina apenas relatórios de inferência e registros acessíveis. Contextos
/// alternativos são fornecidos pelo experimento, não descobertos pela rede.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Monitor {
    task: TwoHopTask,
    max_attempts: usize,
}

impl Monitor {
    pub fn new(task: TwoHopTask, max_attempts: usize) -> Result<Self, ReviewError> {
        if max_attempts == 0 || max_attempts > MAX_ATTEMPTS {
            return Err(ReviewError::InvalidAttemptBudget);
        }
        Ok(Self { task, max_attempts })
    }

    /// Executa no máximo `max_attempts` avaliações, sem recursão. Uma revisão
    /// só é aceita quando exatamente um contexto candidato sustenta a resposta.
    pub fn review(
        self,
        store: &EngramStore,
        original: Hypothesis,
        candidate_contexts: &[u32],
    ) -> Result<ReviewReport, ReviewError> {
        Hypothesis::new(original.context, original.from, original.to)
            .map_err(ReviewError::InvalidHypothesis)?;
        if candidate_contexts.len() > MAX_ATTEMPTS {
            return Err(ReviewError::TooManyCandidates);
        }
        for (index, context) in candidate_contexts.iter().copied().enumerate() {
            if context == 0 || context == original.context {
                return Err(ReviewError::InvalidCandidateContext);
            }
            if candidate_contexts[..index].contains(&context) {
                return Err(ReviewError::DuplicateCandidateContext);
            }
        }

        let mut attempts = Vec::new();
        let mut selected: Option<(Hypothesis, EvidenceRefs)> = None;
        let mut revision_reason = None;
        let mut saw_contradiction = false;
        let mut saw_resource_limit = false;
        let mut saw_propagation_failure = false;
        let mut saw_invalid_evidence = false;

        for context in std::iter::once(original.context).chain(candidate_contexts.iter().copied()) {
            if attempts.len() == self.max_attempts {
                return Ok(ReviewReport {
                    original,
                    decision: Decision::Abstain(AbstainReason::ReviewBudget),
                    attempts,
                });
            }
            let hypothesis = Hypothesis {
                context,
                from: original.from,
                to: original.to,
            };
            let log = self.assess(store, hypothesis);
            let assessment = log.assessment;
            let evidence = log.evidence;
            attempts.push(log);

            if context == original.context {
                match assessment {
                    Assessment::Sufficient => {
                        let Some(evidence) = evidence else {
                            return Ok(ReviewReport {
                                original,
                                decision: Decision::Abstain(AbstainReason::InvalidEvidence),
                                attempts,
                            });
                        };
                        return Ok(ReviewReport {
                            original,
                            decision: Decision::Respond {
                                hypothesis,
                                evidence,
                                revision: None,
                            },
                            attempts,
                        });
                    }
                    Assessment::MissingEvidence => {
                        revision_reason = Some(RevisionReason::MissingEvidence);
                    }
                    Assessment::ContradictoryEvidence => {
                        revision_reason = Some(RevisionReason::ContradictoryEvidence);
                        saw_contradiction = true;
                    }
                    Assessment::ResourceLimited => {
                        return Ok(ReviewReport {
                            original,
                            decision: Decision::Abstain(AbstainReason::ResourceLimited),
                            attempts,
                        });
                    }
                    Assessment::PropagationFailure => {
                        return Ok(ReviewReport {
                            original,
                            decision: Decision::Abstain(AbstainReason::PropagationFailure),
                            attempts,
                        });
                    }
                    Assessment::InvalidEvidence => {
                        return Ok(ReviewReport {
                            original,
                            decision: Decision::Abstain(AbstainReason::InvalidEvidence),
                            attempts,
                        });
                    }
                }
            } else {
                match assessment {
                    Assessment::Sufficient => {
                        if selected.is_some() {
                            return Ok(ReviewReport {
                                original,
                                decision: Decision::Abstain(
                                    AbstainReason::MultipleSupportedContexts,
                                ),
                                attempts,
                            });
                        }
                        selected = evidence.map(|refs| (hypothesis, refs));
                    }
                    Assessment::ContradictoryEvidence => saw_contradiction = true,
                    Assessment::ResourceLimited => saw_resource_limit = true,
                    Assessment::PropagationFailure => saw_propagation_failure = true,
                    Assessment::InvalidEvidence => saw_invalid_evidence = true,
                    Assessment::MissingEvidence => {}
                }
            }
        }

        let decision = if saw_resource_limit {
            Decision::Abstain(AbstainReason::ResourceLimited)
        } else if saw_propagation_failure {
            Decision::Abstain(AbstainReason::PropagationFailure)
        } else if saw_invalid_evidence {
            Decision::Abstain(AbstainReason::InvalidEvidence)
        } else if let Some((hypothesis, evidence)) = selected {
            Decision::Respond {
                hypothesis,
                evidence,
                revision: revision_reason,
            }
        } else if saw_contradiction {
            Decision::Abstain(AbstainReason::ContradictoryEvidence)
        } else {
            Decision::Abstain(AbstainReason::MissingEvidence)
        };
        Ok(ReviewReport {
            original,
            decision,
            attempts,
        })
    }

    fn assess(self, store: &EngramStore, hypothesis: Hypothesis) -> AttemptLog {
        match self.task.evaluate(store, hypothesis) {
            Ok(inference) => {
                let evidence = match inference.outcome {
                    Outcome::DirectRecall(id) => Some(EvidenceRefs::Direct(id)),
                    Outcome::Inferred(proof) => Some(EvidenceRefs::TwoHop {
                        first: proof.first_fact,
                        second: proof.second_fact,
                    }),
                    _ => None,
                };
                let conflict =
                    evidence.and_then(|_| find_conflict(store, hypothesis, inference.outcome));
                let assessment = match inference.outcome {
                    Outcome::DirectRecall(_) | Outcome::Inferred(_) if conflict.is_some() => {
                        Assessment::ContradictoryEvidence
                    }
                    Outcome::DirectRecall(_) | Outcome::Inferred(_) => Assessment::Sufficient,
                    Outcome::Unsupported => Assessment::MissingEvidence,
                    Outcome::InconclusiveBudget => Assessment::ResourceLimited,
                    Outcome::InconclusivePropagation(_) => Assessment::PropagationFailure,
                };
                AttemptLog {
                    hypothesis,
                    inference: Some(inference),
                    evaluation_error: None,
                    assessment,
                    evidence,
                    conflict,
                }
            }
            Err(error) => AttemptLog {
                hypothesis,
                inference: None,
                evaluation_error: Some(error),
                assessment: if error == CognitionError::FactLimitExceeded {
                    Assessment::ResourceLimited
                } else {
                    Assessment::InvalidEvidence
                },
                evidence: None,
                conflict: None,
            },
        }
    }
}

fn find_conflict(
    store: &EngramStore,
    hypothesis: Hypothesis,
    outcome: Outcome,
) -> Option<Conflict> {
    if let Some(negative) = find_negative(store, hypothesis.context, hypothesis.from, hypothesis.to)
    {
        return Some(Conflict::NegativeConclusion { negative });
    }
    if let Outcome::Inferred(Proof {
        via,
        first_fact,
        second_fact,
        ..
    }) = outcome
    {
        if let Some(negative) = find_negative(store, hypothesis.context, hypothesis.from, via) {
            return Some(Conflict::NegativePremise {
                positive: first_fact,
                negative,
            });
        }
        if let Some(negative) = find_negative(store, hypothesis.context, via, hypothesis.to) {
            return Some(Conflict::NegativePremise {
                positive: second_fact,
                negative,
            });
        }
    }
    None
}

fn find_negative(store: &EngramStore, context: u32, from: NodeId, to: NodeId) -> Option<EngramId> {
    store
        .iter()
        .filter(|record| {
            let cue: Cue = record.cue();
            cue.context() == context
                && cue.source() == from
                && cue.pattern() > 0
                && cue.pattern() as u32 == to.get()
                && record.value() == -1
                && record.status() != RecordStatus::Hypothesis
        })
        .map(|record| record.id())
        .min()
}
