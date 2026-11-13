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

/// Dados previamente fixados; nenhuma chave da avaliação independente é recebida.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeachingRequest {
    pub lesson: Lesson,
    pub before: Observation,
    pub after: Observation,
    pub permissions: PedagogyPermissions,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GoalDiagnosis {
    Supported,
    ConceptualInsufficiency,
    RelationalInsufficiency,
    Opposition,
    NegativeClassification,
    ContractBlocked,
    PermissionMissing,
    ResourceLimited,
    StaleReport,
    VerificationFailed,
    NoApplicableLesson,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GoalOutcome {
    Supported,
    Partial,
    Abstention,
    Interrupted,
}
/// Relatório transitório de uma única execução; não é autoridade persistível.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoalReport {
    pub objective: EpisodeId,
    pub binding: Binding,
    pub permissions: Permissions,
    pub teaching: Option<TeachingRequest>,
    pub limits: Limits,
    pub initial: Option<Report>,
    pub diagnosis: GoalDiagnosis,
    pub strategy: crate::metacognition::concepts::ConceptStrategy,
    pub action: Option<PedagogyReport>,
    pub action_started: bool,
    pub action_failure: Option<String>,
    /// Prefixo aceito pela Session durante a tentativa, inclusive em falha.
    pub accepted_operations: Vec<String>,
    pub final_query: Option<Report>,
    pub verification: Option<Verification>,
    pub usage: Usage,
    pub outcome: GoalOutcome,
    pub history: Vec<String>,
    pub reason: String,
}
impl Episode {
    /// Diagnóstico sem nova consulta ou alteração do conhecimento. Recibos e
    /// dependências são conferidos antes de interpretar a insuficiência.
    pub fn diagnose(
        &mut self,
        session: &Session,
        registry: &SyntheticRegistry,
        report: &Report,
        permissions: &Permissions,
    ) -> GoalDiagnosis {
        use crate::metacognition::concepts::{diagnose_pedagogy, PedagogicalDiagnosis};
        use GoalDiagnosis::*;
        if !self.allowed(registry, permissions) {
            return if registry.record != self.binding {
                ContractBlocked
            } else {
                PermissionMissing
            };
        }
        // Recusas anteriores à emissão não possuem recibo/dependências.
        if report.status == Status::Denied && report.dependencies.state.is_empty() {
            return ContractBlocked;
        }
        if report.status == Status::Interrupted && report.dependencies.state.is_empty() {
            return ResourceLimited;
        }
        match self.check_current(session, registry, report, permissions) {
            Verification::Current => {}
            Verification::ReevaluationRequired => return StaleReport,
            Verification::Denied => return PermissionMissing,
            Verification::Interrupted => return ResourceLimited,
            _ => return VerificationFailed,
        }
        match report.status {
            Status::Denied => return ContractBlocked,
            Status::Interrupted => {
                return if report
                    .relational
                    .as_ref()
                    .and_then(|r| r.proof.as_ref())
                    .is_some_and(|p| p.premises.len() != 1)
                {
                    ContractBlocked
                } else {
                    ResourceLimited
                };
            }
            Status::Contradictory => return Opposition,
            _ => {}
        }
        let (Some(concept), Some(relation)) = (&report.conceptual, &report.relational) else {
            return ContractBlocked;
        };
        if relation.refusal.is_some_and(|refusal| {
            !matches!(refusal, Refusal::Monitor(AbstainReason::MissingEvidence))
        }) {
            return ContractBlocked;
        }
        // Reutiliza o verificador MP06, sem procurar outra prova e sem consulta.
        let valid_relation = relation.refusal.is_none()
            && relation
                .review
                .as_ref()
                .is_none_or(|r| r.conflicts.is_empty())
            && relation.proof.as_ref().is_some_and(|proof| {
                proof.premises.len() == 1
                    && session.composed_coordinator().is_ok_and(|coordinator| {
                        let ids = proof
                            .premises
                            .iter()
                            .map(|r| r.engram.id())
                            .collect::<Vec<_>>();
                        coordinator
                            .verify_path(relation.query, relation.at, &ids)
                            .is_ok_and(|p| p == *proof)
                    })
            });
        if !valid_relation {
            return RelationalInsufficiency;
        }
        if concept.decision == ConceptDecision::Supported(Label::Negative) {
            return NegativeClassification;
        }
        match diagnose_pedagogy(std::slice::from_ref(concept)) {
            PedagogicalDiagnosis::ExperienceNeeded => ConceptualInsufficiency,
            PedagogicalDiagnosis::AlreadySupported if report.status == Status::Supported => {
                Supported
            }
            _ => ContractBlocked,
        }
    }

    /// Consome um Episode ainda não usado. Não há reset, segunda experiência ou
    /// acesso mutável paralelo à Session durante o ciclo. APIs manuais F1 continuam
    /// externas: isto não é isolamento contra um chamador hostil.
    pub fn supervise(
        mut self,
        session: &mut Session,
        registry: &SyntheticRegistry,
        permissions: Permissions,
        teaching: Option<TeachingRequest>,
    ) -> GoalReport {
        use crate::metacognition::concepts::{select_concept_strategy, ConceptStrategy};
        use GoalDiagnosis::*;
        let mut out = GoalReport {
            objective: self.objective,
            binding: self.binding.clone(),
            permissions,
            teaching,
            limits: self.limits,
            initial: None,
            diagnosis: ContractBlocked,
            strategy: ConceptStrategy::Abstain,
            action: None,
            action_started: false,
            action_failure: None,
            accepted_operations: vec![],
            final_query: None,
            verification: None,
            usage: self.usage,
            outcome: GoalOutcome::Interrupted,
            history: vec![
                "Objetivo/vínculo fixos; evidência sintética não autentica identidade real".into(),
            ],
            reason: String::new(),
        };
        let journal_start = session.history().len();
        let result = (|| -> Result<(), String> {
            if self.usage != Usage::default() || !self.issued.is_empty() {
                return Err("supervisão requer episódio novo; orçamento nunca é reiniciado".into());
            }
            let initial = self.query(session, registry, &out.permissions)?;
            out.diagnosis = self.diagnose(session, registry, &initial, &out.permissions);
            out.initial = Some(initial.clone());
            out.history.push(format!(
                "Consulta inicial e diagnóstico {:?}, sem mutação cognitiva",
                out.diagnosis
            ));
            match out.diagnosis {
                Supported => {
                    out.strategy =
                        select_concept_strategy(initial.conceptual.as_ref().unwrap(), false, true);
                }
                ConceptualInsufficiency => {
                    let Some(request) = &out.teaching else {
                        out.diagnosis = NoApplicableLesson;
                        out.outcome = GoalOutcome::Abstention;
                        return Ok(());
                    };
                    let p = &request.permissions;
                    if p.evaluation != Some(self.binding.scope)
                        || p.presentation != Some(self.binding.scope)
                        || !p.admissions.contains(&request.lesson.id)
                        || p.admissions.len() > 3
                        || p.admissions
                            .iter()
                            .enumerate()
                            .any(|(i, id)| p.admissions[..i].contains(id))
                    {
                        out.diagnosis = PermissionMissing;
                        return Err(
                            "avaliação/apresentação/admissão pedagógicas não autorizadas".into(),
                        );
                    }
                    let known = session.pedagogy_existing()?;
                    let probes = [request.before, request.after];
                    let overlaps = probes.contains(&self.binding.observation)
                        || self
                            .binding
                            .observation
                            .completions()
                            .contains(&request.lesson.attributes)
                        || known
                            .iter()
                            .chain(std::iter::once(&request.lesson.attributes))
                            .any(|a| probes.iter().any(|o| o.completions().contains(a)));
                    TeachingPlan::new(self.binding.concept, self.binding.scope, &[request.lesson])?;
                    PedagogyAssessment::new(&[request.before], &[request.after])?;
                    let informative = curriculum::choose_lesson(
                        &session.pedagogy_version()?,
                        &[request.lesson],
                        &[],
                    )
                    .is_some();
                    // A mesma política de fontes da sessão; não concede autoridade nova.
                    let known_source = super::investigation::AcquisitionHost::knows_source(
                        session,
                        request.lesson.source,
                    );
                    if overlaps
                        || !informative
                        || !known_source
                        || request.lesson.evidence == self.binding.evidence
                        || initial
                            .relational
                            .as_ref()
                            .unwrap()
                            .consulted
                            .iter()
                            .any(|r| {
                                r.metadata.information.evidence == Some(request.lesson.evidence)
                            })
                    {
                        out.diagnosis = NoApplicableLesson;
                        out.outcome = GoalOutcome::Abstention;
                        return Ok(());
                    }
                    let capacity = self.usage.conceptual_queries + 4
                        <= self.limits.conceptual_queries
                        && self.usage.relational_queries + 2 <= self.limits.relational_queries
                        && self.usage.neural_events + 64 <= self.limits.neural_events
                        && self.usage.conceptual_events + 5 <= 5
                        && session.pedagogy_capacity(1).is_ok();
                    out.strategy = select_concept_strategy(
                        initial.conceptual.as_ref().unwrap(),
                        true,
                        capacity,
                    );
                    if out.strategy != ConceptStrategy::TeachOnce {
                        out.diagnosis = ResourceLimited;
                        return Err(
                            "sem reserva agregada para ensino, reconsulta e verificação".into()
                        );
                    }
                    out.history.push(format!(
                        "Seleção única {:?} anterior à experiência",
                        out.strategy
                    ));
                    out.action_started = true;
                    let attempted = self.teach_once(
                        session,
                        request.lesson,
                        request.before,
                        request.after,
                        p.clone(),
                    );
                    let mut admitted = false;
                    let mut refused = false;
                    match attempted {
                        Ok(action) => {
                            admitted = action.lessons.iter().any(|r| r.admitted);
                            refused =
                                action.leak_detected || action.lessons.iter().any(|r| !r.admitted);
                            out.history.push(format!(
                                "Experiência: {} apresentações; versão {}→{}",
                                action.lessons.len(),
                                action.initial_version.number,
                                action.final_version.number
                            ));
                            out.action = Some(action);
                        }
                        Err(error) => {
                            out.reason = format!("Tentativa pedagógica interrompida: {error}");
                            out.action_failure = Some(error);
                            out.history.push(
                                "Tentativa falhou; prefixo autorizado e reservas preservados"
                                    .into(),
                            );
                        }
                    }
                    // Mesmo objetivo, inclusive quando MP09 não apresenta experiência.
                    out.final_query = Some(self.query(session, registry, &out.permissions)?);
                    out.history
                        .push("Reconsulta do objetivo original após a tentativa pedagógica".into());
                    if refused {
                        out.reason =
                            "Experiência recusada; reconsulta não autoriza declarar sucesso".into();
                    }
                    // Partial exige admissão/revisão real, não apenas tentativa de ensino.
                    if admitted {
                        out.outcome = GoalOutcome::Partial;
                    }
                }
                Opposition | NegativeClassification | RelationalInsufficiency => {
                    out.outcome = GoalOutcome::Abstention;
                    return Ok(());
                }
                ContractBlocked => {
                    return Err("contrato/identidade/elegibilidade incompatíveis".into())
                }
                PermissionMissing => return Err("consulta composta não autorizada".into()),
                ResourceLimited => {
                    out.strategy = ConceptStrategy::Stop;
                    return Err("consulta sem orçamento/reserva".into());
                }
                _ => return Err("integridade/atualidade da consulta não demonstrada".into()),
            }
            let final_report = out.final_query.as_ref().unwrap_or(&initial);
            let verification = self.verify(session, registry, final_report, &out.permissions)?;
            out.verification = Some(verification);
            out.history
                .push(format!("Conferência final {:?}", verification));
            out.outcome = match verification {
                Verification::Verified(Status::Supported) if out.reason.is_empty() => {
                    GoalOutcome::Supported
                }
                Verification::Verified(Status::Insufficient)
                    if out.outcome == GoalOutcome::Partial =>
                {
                    GoalOutcome::Partial
                }
                Verification::Verified(Status::Insufficient | Status::Contradictory)
                    if out.reason.is_empty() =>
                {
                    GoalOutcome::Abstention
                }
                _ => GoalOutcome::Interrupted,
            };
            if !matches!(verification, Verification::Verified(_)) {
                out.diagnosis = match verification {
                    Verification::ReevaluationRequired => StaleReport,
                    Verification::Denied => PermissionMissing,
                    Verification::Interrupted => ResourceLimited,
                    _ => VerificationFailed,
                };
            }
            Ok(())
        })();
        if let Err(error) = result {
            out.outcome = GoalOutcome::Interrupted;
            out.reason = error;
        }
        out.usage = self.usage;
        out.accepted_operations = session.history()[journal_start..].to_vec();
        if out.reason.is_empty() {
            out.reason = format!(
                "{:?}; sustentação provisória no recorte sintético, sem promoção factual",
                out.outcome
            );
        }
        out.history.push(format!(
            "Encerrado {:?}; consumo {:?}",
            out.outcome, out.usage
        ));
        out
    }
}
