//! Consulta composta transitória. Confiança sintética explícita, sem identidade real.
//! Classificação nunca é premissa relacional ou fato incorporado.
use super::concepts::ConceptReport;
use super::query::{PathAuthorization, PathEpisodeLog, PathQueryReport};
use crate::cognition::concepts::{Label, Observation};
use crate::cognitive_contracts::{EpisodeId, Refusal, Scope, SourceId};
use crate::curriculum::{
    self, Lesson, PedagogyAssessment, PedagogyHost, PedagogyPermissions, PedagogyReport,
    TeachingPlan,
};
use crate::engram::EvidenceId;
use crate::event::NodeId;
use crate::metacognition::concepts::{Abstention, ConceptDecision};
use crate::metacognition::{AbstainReason, Assessment};
use crate::session::f1::Session;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingOrigin {
    ExternalSyntheticObservation,
    /// Não suportado: impede que a composição forneça sua própria identidade/evidência.
    ComposedReport(EpisodeId),
}
/// Identidade/atributos declarados, que ainda precisam coincidir com a fonte confiada.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Binding {
    pub id: EpisodeId,
    pub item: NodeId,
    pub observation: Observation,
    pub concept: NodeId,
    pub relational_concept: NodeId,
    pub destination: NodeId,
    pub scope: Scope,
    pub source: SourceId,
    pub evidence: EvidenceId,
    pub origin: BindingOrigin,
}
/// Registro imutável fornecido fora do adaptador. Não contém gabaritos.
/// A confiança nesta tabela é uma premissa do experimento, não autenticação real.
#[derive(Debug)]
pub struct SyntheticRegistry {
    record: Binding,
    revision: u32,
}
impl SyntheticRegistry {
    pub fn new(record: Binding, revision: u32) -> Result<Self, String> {
        if revision == 0
            || record.origin != BindingOrigin::ExternalSyntheticObservation
            || record.observation.complete().is_err()
            || record.relational_concept == record.destination
        {
            return Err("registro sintético inválido ou derivado de composição".into());
        }
        Ok(Self { record, revision })
    }
    pub fn record(&self) -> &Binding {
        &self.record
    }
    pub fn revision(&self) -> u32 {
        self.revision
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permissions {
    pub conceptual: Option<Scope>,
    pub relational: Option<PathAuthorization>,
    /// Concessão exata do vínculo, independente das duas consultas.
    pub composition: Option<Binding>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Supported,
    Insufficient,
    Contradictory,
    Denied,
    Interrupted,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityAssurance {
    SyntheticRegistryMatch,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dependencies {
    pub registry_revision: u32,
    /// Comparação conservadora do estado lógico, incluindo versões, fatos,
    /// metadados, autorizações base e tempo. Não é assinatura ou autorização.
    pub state: Vec<u8>,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Usage {
    pub conceptual_queries: usize,
    pub relational_queries: usize,
    pub neural_events: usize,
    pub conceptual_events: usize,
    pub teaching_attempts: usize,
    pub revision_attempts: usize,
    /// Reservas mantidas em erro de ensino: não são consultas confirmadas.
    pub uncertain_conceptual_reservations: usize,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    pub conceptual_queries: usize,
    pub relational_queries: usize,
    pub neural_events: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            conceptual_queries: 8,
            relational_queries: 3,
            neural_events: 96,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Report {
    pub objective: EpisodeId,
    pub binding: Binding,
    pub assurance: Option<IdentityAssurance>,
    pub permissions: Permissions,
    pub conceptual: Option<ConceptReport>,
    pub relational: Option<PathQueryReport>,
    pub dependencies: Dependencies,
    pub usage: Usage,
    pub status: Status,
    pub reason: String,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Verification {
    /// Somente atualidade/origem do relatório; não substitui verify.
    Current,
    Verified(Status),
    ReevaluationRequired,
    Rejected,
    Denied,
    Interrupted,
}
/// Um único objetivo/vínculo por episódio. Contadores e recibos não são persistidos.
/// Sem Clone: a API não oferece checkpoint/reset/reembolso do orçamento.
pub struct Episode {
    objective: EpisodeId,
    binding: Binding,
    limits: Limits,
    usage: Usage,
    last_concept_events: usize,
    issued: Vec<Report>,
}
impl Episode {
    pub fn new(session: &Session, objective: EpisodeId, binding: Binding) -> Self {
        Self::with_limits(session, objective, binding, Limits::default())
            .expect("limites constantes válidos")
    }
    pub fn with_limits(
        session: &Session,
        objective: EpisodeId,
        binding: Binding,
        limits: Limits,
    ) -> Result<Self, String> {
        if limits.conceptual_queries > 8
            || limits.relational_queries > 3
            || limits.neural_events > 96
        {
            return Err("limites excedem episódio composto".into());
        }
        Ok(Self {
            objective,
            binding,
            limits,
            usage: Usage::default(),
            last_concept_events: session.pedagogy_events(),
            issued: vec![],
        })
    }
    pub fn usage(&self) -> Usage {
        self.usage
    }
    fn observe_events(&mut self, session: &Session) -> Result<(), String> {
        let now = session.pedagogy_events();
        let delta = now
            .checked_sub(self.last_concept_events)
            .ok_or("estado conceitual retrocedeu; novo episódio explícito necessário")?;
        self.last_concept_events = now;
        self.usage.conceptual_events += delta;
        if self.usage.conceptual_events > 10 {
            return Err("mais de dez eventos conceituais novos".into());
        }
        Ok(())
    }
    fn allowed(&self, registry: &SyntheticRegistry, p: &Permissions) -> bool {
        self.binding.origin == BindingOrigin::ExternalSyntheticObservation
            && registry.record == self.binding
            && p.conceptual == Some(self.binding.scope)
            && p.relational == Some(PathAuthorization::inclusion_three(self.binding.scope))
            && p.composition.as_ref() == Some(&self.binding)
    }
    fn report(&self, registry: &SyntheticRegistry, p: &Permissions) -> Report {
        Report {
            objective: self.objective,
            binding: self.binding.clone(),
            assurance: None,
            permissions: p.clone(),
            conceptual: None,
            relational: None,
            dependencies: Dependencies {
                registry_revision: registry.revision,
                state: Vec::new(),
            },
            usage: self.usage,
            status: Status::Denied,
            reason: "Permissão, identidade sintética ou vínculo incompatível".into(),
        }
    }
    pub fn query(
        &mut self,
        session: &Session,
        registry: &SyntheticRegistry,
        p: &Permissions,
    ) -> Result<Report, String> {
        let mut out = self.report(registry, p);
        if !self.allowed(registry, p) {
            return Ok(out);
        }
        if let Err(e) = self.observe_events(session) {
            out.status = Status::Interrupted;
            out.reason = e;
            out.usage = self.usage;
            return Ok(out);
        }
        if session.pedagogy_identity().ok() != Some((self.binding.concept, self.binding.scope)) {
            return Ok(out);
        }
        // Pré-validação usa símbolos/configuração e autorização BASE reais da sessão.
        let relation = match session.composed_statement(
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
        if self.usage.conceptual_queries + self.usage.uncertain_conceptual_reservations
            >= self.limits.conceptual_queries
            || self.usage.relational_queries >= self.limits.relational_queries
            || self.usage.neural_events + 32 > self.limits.neural_events
        {
            out.status = Status::Interrupted;
            out.reason = "Sem reserva compartilhada para duas consultas e teto neural32".into();
            out.usage = self.usage;
            return Ok(out);
        }
        out.assurance = Some(IdentityAssurance::SyntheticRegistryMatch);
        self.usage.conceptual_queries += 1;
        let conceptual = match session.pedagogy_query(self.objective, self.binding.observation) {
            Ok(c) => c,
            Err(e) => {
                out.reason = e;
                out.usage = self.usage;
                return Ok(out);
            }
        };
        self.usage.relational_queries += 1;
        // API MP06 existente já aceita orçamento: não altera seu teto geral64.
        let relational = coordinator
            .query_three(
                self.objective,
                relation,
                session.logical_time(),
                p.relational.unwrap(),
                32,
                &mut PathEpisodeLog::default(),
            )
            .map_err(|e| format!("{e:?}"))?;
        self.usage.neural_events += relational
            .review
            .as_ref()
            .map_or(0, |r| r.inference.processed_events);
        let opposition = conceptual.decision
            == ConceptDecision::Abstain(Abstention::ContradictoryEvidence)
            || relational
                .review
                .as_ref()
                .is_some_and(|r| !r.conflicts.is_empty());
        let independent = relational.proof.as_ref().is_none_or(|proof| {
            proof.premises.iter().all(|r| {
                let ev = r.metadata.information.evidence;
                ev != Some(self.binding.evidence)
                    && conceptual
                        .consulted
                        .iter()
                        .all(|c| ev != Some(c.metadata.evidence))
            })
        }) && conceptual
            .consulted
            .iter()
            .all(|c| c.metadata.evidence != self.binding.evidence);
        let one_premise = relational
            .proof
            .as_ref()
            .is_none_or(|p| p.premises.len() == 1);
        // Confere referências contra o coordenador, sem procurar prova substituta.
        let proof_valid = relational.proof.as_ref().is_some_and(|proof| {
            let ids = proof
                .premises
                .iter()
                .map(|r| r.engram.id())
                .collect::<Vec<_>>();
            coordinator
                .verify_path(relation, session.logical_time(), &ids)
                .is_ok_and(|p| p == *proof)
        });
        let limited = relational
            .review
            .as_ref()
            .is_some_and(|r| r.assessment == Assessment::ResourceLimited)
            || relational.refusal == Some(Refusal::Monitor(AbstainReason::ResourceLimited));
        out.status = if !independent {
            Status::Denied
        } else if opposition {
            Status::Contradictory
        } else if limited || !one_premise {
            Status::Interrupted
        } else if relational.refusal == Some(Refusal::RuleOutsideContext) {
            Status::Denied
        } else if conceptual.decision == ConceptDecision::Supported(Label::Positive)
            && proof_valid
            && relational.refusal.is_none()
        {
            Status::Supported
        } else {
            Status::Insufficient
        };
        out.reason=match out.status {
            Status::Denied=>"Autorização base ou independência das evidências não demonstrada",
            Status::Contradictory=>"Oposição conceitual/relacional preservada; composição recusada",
            Status::Interrupted=>"Limite neural ou recorte de uma premissa impede conclusão",
            Status::Supported=>"Conjunção provisória sustentada; não cria fato x em D nem autentica identidade real",
            Status::Insufficient=>"Falta suporte positivo em uma das condições",
        }.into();
        out.conceptual = Some(conceptual);
        out.relational = Some(relational);
        out.usage = self.usage;
        // Recibo privado detecta adulteração inclusive de campos de auditoria.
        // NÃO serve como prova da classificação: verify reexecuta os dois núcleos.
        out.dependencies.state = session.encode();
        self.issued.push(out.clone());
        Ok(out)
    }
    pub fn check_current(
        &mut self,
        session: &Session,
        registry: &SyntheticRegistry,
        r: &Report,
        p: &Permissions,
    ) -> Verification {
        if !self.issued.contains(r) {
            return Verification::Rejected;
        }
        if !self.allowed(registry, p) {
            return if registry.record != r.binding
                || registry.revision != r.dependencies.registry_revision
            {
                Verification::ReevaluationRequired
            } else {
                Verification::Denied
            };
        }
        if self.observe_events(session).is_err() {
            return Verification::Interrupted;
        }
        if registry.revision != r.dependencies.registry_revision
            || session.encode() != r.dependencies.state
        {
            Verification::ReevaluationRequired
        } else {
            Verification::Current
        }
    }
    pub fn verify(
        &mut self,
        session: &Session,
        registry: &SyntheticRegistry,
        r: &Report,
        p: &Permissions,
    ) -> Result<Verification, String> {
        match self.check_current(session, registry, r, p) {
            Verification::Current => {}
            other => return Ok(other),
        }
        // Recálculo a partir do estado admitido, não dos resultados do relatório.
        let fresh = self.query(session, registry, p)?;
        if fresh.status == Status::Interrupted {
            return Ok(Verification::Interrupted);
        }
        let equivalent = fresh.objective == r.objective
            && fresh.binding == r.binding
            && fresh.assurance == r.assurance
            && fresh.permissions == r.permissions
            && fresh.conceptual == r.conceptual
            && fresh.relational == r.relational
            && fresh.dependencies == r.dependencies
            && fresh.status == r.status
            && fresh.reason == r.reason;
        Ok(if equivalent {
            Verification::Verified(fresh.status)
        } else {
            Verification::Rejected
        })
    }
    /// Uma experiência MP09. Reserva no mesmo orçamento as duas avaliações1+1.
    pub fn teach_once(
        &mut self,
        session: &mut Session,
        lesson: Lesson,
        pre: Observation,
        post: Observation,
        p: PedagogyPermissions,
    ) -> Result<PedagogyReport, String> {
        if self.usage.teaching_attempts != 0
            || self.usage.conceptual_queries + self.usage.uncertain_conceptual_reservations + 2
                > self.limits.conceptual_queries
        {
            return Err("ensino ou consultas excedem episódio".into());
        }
        if session.pedagogy_identity()? != (self.binding.concept, self.binding.scope)
            || p.evaluation != Some(self.binding.scope)
        {
            return Err("avaliação de ensino não autorizada/fora de escopo".into());
        }
        if pre == self.binding.observation
            || post == self.binding.observation
            || self
                .binding
                .observation
                .completions()
                .contains(&lesson.attributes)
        {
            return Err("objetivo não pode ser ensino/avaliação didática".into());
        }
        let plan = TeachingPlan::new(self.binding.concept, self.binding.scope, &[lesson])?;
        let assessment = PedagogyAssessment::new(&[pre], &[post])?;
        self.observe_events(session)?;
        self.usage.teaching_attempts += 1;
        self.usage.uncertain_conceptual_reservations += 2;
        let result = curriculum::teach_concept(session, plan, assessment, p);
        if let Ok(r) = &result {
            self.usage.uncertain_conceptual_reservations -= 2;
            self.usage.conceptual_queries += r.initial.len() + r.posterior.len();
        }
        self.observe_events(session)?;
        result
    }
    /// Único desafio de revisão explícita, pelos mesmos atos MP09/F1.
    pub fn revise_once(
        &mut self,
        session: &mut Session,
        lesson: Lesson,
        p: PedagogyPermissions,
    ) -> Result<(), String> {
        if self.usage.revision_attempts != 0
            || p.presentation != Some(self.binding.scope)
            || !p.admissions.contains(&lesson.id)
            || session.pedagogy_identity()? != (self.binding.concept, self.binding.scope)
        {
            return Err("revisão não autorizada, repetida ou fora do escopo".into());
        }
        self.observe_events(session)?;
        session.pedagogy_capacity(1)?;
        self.usage.revision_attempts += 1;
        let result = (|| {
            if session.pedagogy_present(lesson)? != super::concepts::Admission::Admissible {
                return Err("revisão não admissível".into());
            }
            session.pedagogy_admit(lesson.id)?;
            Ok(())
        })();
        self.observe_events(session)?;
        result
    }
}
