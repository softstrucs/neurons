//! F3-01. Coordenador aditivo; não altera a composição F2 ou as regras MP06/H23.
//! Uma consulta e uma conferência, sem ensino, admissão, persistência ou reset.
use super::composed::SyntheticRegistry;
use super::composed::{
    Binding, BindingOrigin, Dependencies, IdentityAssurance, Permissions, Report, Status, Usage,
    Verification,
};
use super::query::{PathAuthorization, PathEpisodeLog};
use crate::cognition::concepts::Label;
use crate::cognitive_contracts::{EpisodeId, Refusal};
use crate::curriculum::PedagogyHost;
use crate::metacognition::concepts::{Abstention, ConceptDecision};
use crate::metacognition::{AbstainReason, Assessment};
use crate::session::f1::Session;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    pub conceptual_queries: usize,
    pub relational_queries: usize,
    pub neural_events: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            conceptual_queries: 2,
            relational_queries: 2,
            neural_events: 64,
        }
    }
}

/// Autoridade recebida fica ligada a um objetivo e a um vínculo imutáveis.
/// Não há Clone, reset ou importação de relatório como autoridade.
pub struct Episode {
    objective: EpisodeId,
    binding: Binding,
    permissions: Permissions,
    limits: Limits,
    usage: Usage,
    started: bool,
    finished: bool,
    issued: Option<Report>,
}
impl Episode {
    pub fn new(
        objective: EpisodeId,
        binding: Binding,
        permissions: Permissions,
        limits: Limits,
    ) -> Result<Self, String> {
        if limits.conceptual_queries > 2
            || limits.relational_queries > 2
            || limits.neural_events > 64
        {
            return Err("limites F3-01 excedidos".into());
        }
        Ok(Self {
            objective,
            binding,
            permissions,
            limits,
            usage: Usage::default(),
            started: false,
            finished: false,
            issued: None,
        })
    }
    pub fn usage(&self) -> Usage {
        self.usage
    }
    fn allowed(&self, registry: &SyntheticRegistry) -> bool {
        self.binding.origin == BindingOrigin::ExternalSyntheticObservation
            && registry.record() == &self.binding
            && self.permissions.conceptual == Some(self.binding.scope)
            && self.permissions.relational
                == Some(PathAuthorization::inclusion_three(self.binding.scope))
            && self.permissions.composition.as_ref() == Some(&self.binding)
    }
    fn empty(&self, registry: &SyntheticRegistry) -> Report {
        Report {
            objective: self.objective,
            binding: self.binding.clone(),
            assurance: None,
            permissions: self.permissions.clone(),
            conceptual: None,
            relational: None,
            dependencies: Dependencies {
                registry_revision: registry.revision(),
                state: vec![],
            },
            usage: self.usage,
            status: Status::Denied,
            reason: "F3-01: identidade, escopo ou concessões incompatíveis".into(),
        }
    }
    /// Relatório inicial é provisório: sustentação final exige verify no mesmo episódio.
    pub fn query(
        &mut self,
        session: &Session,
        registry: &SyntheticRegistry,
    ) -> Result<Report, String> {
        if self.started {
            return Err("F3-01: consulta já consumida".into());
        }
        self.started = true;
        let mut report = self.empty(registry);
        if self.allowed(registry) {
            if self.limits != Limits::default() {
                report.status = Status::Interrupted;
                report.reason =
                    "F3-01: sem reserva integral 2/2/64 para consulta e conferência".into();
            } else {
                report = self.calculate(session, registry)?;
            }
        }
        self.issued = Some(report.clone());
        Ok(report)
    }
    /// Única conferência. Recálculo cobra consultas/eventos no orçamento original.
    /// Recusa/erro também consome a oportunidade; nunca há nova tentativa implícita.
    pub fn verify(
        &mut self,
        session: &Session,
        registry: &SyntheticRegistry,
        report: &Report,
    ) -> Result<Verification, String> {
        if !self.started || self.finished {
            return Err("F3-01: conferência indisponível/consumida".into());
        }
        self.finished = true;
        if self.issued.as_ref() != Some(report) {
            return Ok(Verification::Rejected);
        }
        if !self.allowed(registry) {
            return Ok(Verification::Denied);
        }
        if report.dependencies.state.is_empty() {
            return Ok(if report.status == Status::Interrupted {
                Verification::Interrupted
            } else {
                Verification::Denied
            });
        }
        if registry.revision() != report.dependencies.registry_revision
            || session.encode() != report.dependencies.state
        {
            return Ok(Verification::ReevaluationRequired);
        }
        let fresh = self.calculate(session, registry)?;
        if fresh.status == Status::Interrupted {
            return Ok(Verification::Interrupted);
        }
        // Usage cresce por definição; todos os campos semânticos/auditoria são confrontados.
        let mut expected = report.clone();
        expected.usage = fresh.usage;
        Ok(if expected == fresh {
            Verification::Verified(fresh.status)
        } else {
            Verification::Rejected
        })
    }
    fn calculate(
        &mut self,
        session: &Session,
        registry: &SyntheticRegistry,
    ) -> Result<Report, String> {
        let mut out = self.empty(registry);
        if !self.allowed(registry)
            || session.pedagogy_identity().ok() != Some((self.binding.concept, self.binding.scope))
        {
            return Ok(out);
        }
        let statement = match session.composed_statement(
            self.binding.scope,
            self.binding.relational_concept,
            self.binding.destination,
        ) {
            Ok(q) => q,
            Err(e) => {
                out.reason = e;
                return Ok(out);
            }
        };
        let coordinator = match session.composed_coordinator() {
            Ok(c) => c,
            Err(e) => {
                out.reason = e;
                return Ok(out);
            }
        };
        if self.usage.conceptual_queries >= self.limits.conceptual_queries
            || self.usage.relational_queries >= self.limits.relational_queries
            || self.usage.neural_events + 32 > self.limits.neural_events
        {
            out.status = Status::Interrupted;
            out.reason = "F3-01: reserva esgotada".into();
            return Ok(out);
        }
        self.usage.conceptual_queries += 1;
        let conceptual = session.pedagogy_query(self.objective, self.binding.observation)?;
        self.usage.relational_queries += 1;
        let relational = coordinator
            .query_three(
                self.objective,
                statement,
                session.logical_time(),
                self.permissions.relational.unwrap(),
                32,
                &mut PathEpisodeLog::default(),
            )
            .map_err(|e| format!("{e:?}"))?;
        self.usage.neural_events += relational
            .review
            .as_ref()
            .map_or(0, |r| r.inference.processed_events);
        let proof = relational.proof.as_ref();
        let independent = conceptual
            .consulted
            .iter()
            .all(|c| c.metadata.evidence != self.binding.evidence)
            && proof.is_none_or(|p| {
                p.premises.iter().enumerate().all(|(i, r)| {
                    let ev = r.metadata.information.evidence;
                    ev != Some(self.binding.evidence)
                        && conceptual
                            .consulted
                            .iter()
                            .all(|c| ev != Some(c.metadata.evidence))
                        && p.premises[..i]
                            .iter()
                            .all(|prev| prev.metadata.information.evidence != ev)
                })
            });
        let opposed = conceptual.decision
            == ConceptDecision::Abstain(Abstention::ContradictoryEvidence)
            || relational
                .review
                .as_ref()
                .is_some_and(|r| !r.conflicts.is_empty());
        let limited = relational.refusal == Some(Refusal::Monitor(AbstainReason::ResourceLimited))
            || relational
                .review
                .as_ref()
                .is_some_and(|r| r.assessment == Assessment::ResourceLimited)
            || self.usage.neural_events > self.limits.neural_events;
        let exact = proof.is_some_and(|p| p.premises.len() == 2);
        let valid = proof.is_some_and(|p| {
            let ids = p.premises.iter().map(|r| r.engram.id()).collect::<Vec<_>>();
            coordinator
                .verify_path(statement, session.logical_time(), &ids)
                .is_ok_and(|v| v == *p)
        });
        out.status = if !independent {
            Status::Denied
        } else if opposed {
            Status::Contradictory
        } else if limited {
            Status::Interrupted
        } else if proof.is_some() && (!exact || !valid) {
            Status::Denied
        } else if relational
            .refusal
            .is_some_and(|r| r != Refusal::Monitor(AbstainReason::MissingEvidence))
        {
            Status::Denied
        } else if conceptual.decision == ConceptDecision::Supported(Label::Positive)
            && exact
            && valid
            && relational.refusal.is_none()
        {
            Status::Supported
        } else {
            Status::Insufficient
        };
        out.assurance = Some(IdentityAssurance::SyntheticRegistryMatch);
        out.reason = format!(
            "F3-01 {:?}: conjunção provisória, sem admissão factual",
            out.status
        );
        out.conceptual = Some(conceptual);
        out.relational = Some(relational);
        out.dependencies.state = session.encode();
        out.usage = self.usage;
        Ok(out)
    }
}
