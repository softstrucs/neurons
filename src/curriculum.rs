//! Microcurrículos relacionais: contextos de domínio, sem semântica ou respostas embutidas.
use crate::cognition::{CognitionError, Hypothesis, TwoHopTask};
use crate::engram::{Cue, Engram, EngramError, EngramId, EngramStore, RecordStatus};
use crate::event::NodeId;
use crate::metacognition::{Monitor, ReviewError, ReviewReport};

/// Cada contexto contém uma única relação definida externamente pelo protocolo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Domain {
    Communication,
    Mathematics,
    NaturalScience,
    Computing,
}

impl Domain {
    pub const fn context(self) -> u32 {
        match self {
            Self::Communication => 901,
            Self::Mathematics => 902,
            Self::NaturalScience => 903,
            Self::Computing => 904,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Polarity {
    Positive,
    Negative,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurriculumError {
    InvalidCapacity,
    InvalidTarget,
    ReflexiveObservation,
    Memory(EngramError),
    Query(CognitionError),
    Review(ReviewError),
}

/// Memória compartilhada limitada; registros são fornecidos como observações.
/// Não há treino implícito, promoção a validado ou gravação das conclusões.
pub struct Microcurriculum {
    memory: EngramStore,
    monitor: Monitor,
}

impl Microcurriculum {
    pub fn new(capacity: usize) -> Result<Self, CurriculumError> {
        if capacity == 0 || capacity > 32 {
            return Err(CurriculumError::InvalidCapacity);
        }
        let task = TwoHopTask::new(32, 64, 128).map_err(CurriculumError::Query)?;
        Ok(Self {
            memory: EngramStore::new(capacity).map_err(CurriculumError::Memory)?,
            monitor: Monitor::new(task, 1).map_err(CurriculumError::Review)?,
        })
    }

    pub fn observe(
        &mut self,
        id: EngramId,
        domain: Domain,
        from: NodeId,
        to: NodeId,
        polarity: Polarity,
    ) -> Result<(), CurriculumError> {
        if to.get() > i32::MAX as u32 {
            return Err(CurriculumError::InvalidTarget);
        }
        if from == to {
            return Err(CurriculumError::ReflexiveObservation);
        }
        let cue =
            Cue::new(domain.context(), from, to.get() as i32).map_err(CurriculumError::Memory)?;
        let value = match polarity {
            Polarity::Positive => 1,
            Polarity::Negative => -1,
        };
        let record = Engram::new(id, cue, value, Some(from), 0, None, RecordStatus::Observed)
            .map_err(CurriculumError::Memory)?;
        self.memory.record(record).map_err(CurriculumError::Memory)
    }

    pub fn evaluate(
        &self,
        domain: Domain,
        from: NodeId,
        to: NodeId,
    ) -> Result<ReviewReport, CurriculumError> {
        let hypothesis =
            Hypothesis::new(domain.context(), from, to).map_err(CurriculumError::Query)?;
        self.monitor
            .review(&self.memory, hypothesis, &[])
            .map_err(CurriculumError::Review)
    }

    /// Evidência acessível para auditoria e controles, sem acesso mutável.
    pub fn memory(&self) -> &EngramStore {
        &self.memory
    }
}

// MP09 utiliza o estado conceitual F1; Microcurriculum legado não é instanciado.
use crate::cognition::concepts::{Attributes, Label, Observation};
use crate::cognitive_contracts::{EpisodeId, Scope, SourceId};
use crate::engram::EvidenceId;
use crate::integration::concepts::{Admission, ConceptReport};
use crate::learning::concepts::HypothesisVersion;
use crate::metacognition::concepts::{diagnose_pedagogy, PedagogicalDiagnosis};

pub(crate) mod pedagogy_sealed {
    pub trait Sealed {}
}
pub trait PedagogyHost: pedagogy_sealed::Sealed {
    fn pedagogy_events(&self) -> usize;
    fn pedagogy_existing(&self) -> Result<Vec<Attributes>, String>;
    fn pedagogy_identity(&self) -> Result<(NodeId, Scope), String>;
    fn pedagogy_version(&self) -> Result<HypothesisVersion, String>;
    fn pedagogy_query(
        &self,
        episode: EpisodeId,
        observation: Observation,
    ) -> Result<ConceptReport, String>;
    fn pedagogy_present(&mut self, lesson: Lesson) -> Result<Admission, String>;
    fn pedagogy_admit(&mut self, id: EngramId) -> Result<HypothesisVersion, String>;
    fn pedagogy_capacity(&self, lessons: usize) -> Result<(), String>;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Lesson {
    pub id: EngramId,
    pub receipt: u32,
    pub exemplar: NodeId,
    pub attributes: Attributes,
    pub label: Label,
    pub source: SourceId,
    pub evidence: EvidenceId,
}
/// Plano fixado antes da execução: somente experiências de ensino, sem gabaritos de prova.
pub struct TeachingPlan {
    concept: NodeId,
    scope: Scope,
    lessons: Vec<Lesson>,
}
impl TeachingPlan {
    pub fn new(concept: NodeId, scope: Scope, lessons: &[Lesson]) -> Result<Self, String> {
        if lessons.len() > 3 {
            return Err("máximo de três experiências".into());
        }
        for (i, l) in lessons.iter().enumerate() {
            if l.receipt == 0
                || lessons[..i].iter().any(|p| {
                    p.id == l.id
                        || p.receipt == l.receipt
                        || p.evidence == l.evidence
                        || p.exemplar == l.exemplar
                })
            {
                return Err("identidade didática inválida ou duplicada".into());
            }
        }
        Ok(Self {
            concept,
            scope,
            lessons: lessons.to_vec(),
        })
    }
}
/// Apenas observações sem rótulo. O supervisor não recebe a chave de correção.
pub struct PedagogyAssessment {
    before: Vec<Observation>,
    after: Vec<Observation>,
}
impl PedagogyAssessment {
    pub fn new(before: &[Observation], after: &[Observation]) -> Result<Self, String> {
        if before.is_empty() || after.is_empty() || before.len() > 2 || after.len() > 2 {
            return Err("cada avaliação exige um ou dois itens".into());
        }
        let all: Vec<_> = before.iter().chain(after).collect();
        if all.iter().enumerate().any(|(i, a)| all[..i].contains(a)) {
            return Err("avaliações devem ser separadas, sem itens repetidos".into());
        }
        Ok(Self {
            before: before.to_vec(),
            after: after.to_vec(),
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PedagogyPermissions {
    pub evaluation: Option<Scope>,
    pub presentation: Option<Scope>,
    pub admissions: Vec<EngramId>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LessonRecord {
    pub lesson: Lesson,
    pub rejected_hypotheses: usize,
    pub admissibility: Admission,
    pub admitted: bool,
    pub revision: Option<HypothesisVersion>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PedagogyReport {
    pub concept: NodeId,
    pub scope: Scope,
    pub permissions: PedagogyPermissions,
    pub diagnosis: PedagogicalDiagnosis,
    pub initial: Vec<ConceptReport>,
    pub posterior: Vec<ConceptReport>,
    pub initial_version: HypothesisVersion,
    pub final_version: HypothesisVersion,
    pub lessons: Vec<LessonRecord>,
    pub history: Vec<String>,
    pub leak_detected: bool,
}
/// Seleção determinística por redução informativa entre hipóteses atuais.
/// Recebe SOMENTE hipóteses e experiências didáticas: nenhuma observação/rótulo de prova.
pub(crate) fn choose_lesson(
    version: &HypothesisVersion,
    lessons: &[Lesson],
    used: &[EngramId],
) -> Option<(Lesson, usize)> {
    lessons
        .iter()
        .filter(|l| !used.contains(&l.id))
        .filter_map(|l| {
            let rejected = version
                .criteria
                .iter()
                .filter(|c| c.evaluate(l.attributes) != l.label)
                .count();
            (rejected > 0 && rejected < version.criteria.len()).then_some((*l, rejected))
        })
        .min_by_key(|(l, n)| (usize::MAX - *n, l.id.get()))
}
/// Episódio único: duas avaliações em lote, até três apresentações/admissões/revisões.
/// Resultado não contém acurácia: correção é medida fora do supervisor por chave separada.
pub fn teach_concept(
    host: &mut impl PedagogyHost,
    plan: TeachingPlan,
    assessment: PedagogyAssessment,
    permissions: PedagogyPermissions,
) -> Result<PedagogyReport, String> {
    if permissions.admissions.len() > 3
        || permissions
            .admissions
            .iter()
            .enumerate()
            .any(|(i, a)| permissions.admissions[..i].contains(a))
    {
        return Err("autorizações de admissão duplicadas ou excessivas".into());
    }
    if host.pedagogy_identity()? != (plan.concept, plan.scope) {
        return Err("conceito/contexto do plano incompatível".into());
    }
    if permissions.evaluation != Some(plan.scope) {
        return Err("avaliação conceitual não autorizada".into());
    }
    let initial_version = host.pedagogy_version()?;
    let mut out = PedagogyReport {
        concept: plan.concept,
        scope: plan.scope,
        permissions: permissions.clone(),
        diagnosis: PedagogicalDiagnosis::Blocked,
        initial: vec![],
        posterior: vec![],
        initial_version: initial_version.clone(),
        final_version: initial_version,
        lessons: vec![],
        history: vec!["Plano fixado; chave de avaliação ausente da interface".into()],
        leak_detected: false,
    };
    // Barreira anterior à execução: nenhum item de ensino pode coincidir com prova.
    let known = host.pedagogy_existing()?;
    if plan
        .lessons
        .iter()
        .map(|l| l.attributes)
        .chain(known)
        .any(|a| {
            assessment
                .before
                .iter()
                .chain(&assessment.after)
                .any(|o| o.completions().contains(&a))
        })
    {
        out.leak_detected = true;
        out.history.push("Sobreposição de ensino/estado prévio com avaliação detectada; bloqueio antes da execução".into());
        return Ok(out);
    }
    host.pedagogy_capacity(plan.lessons.len())?;
    for (i, observation) in assessment.before.iter().enumerate() {
        out.initial.push(host.pedagogy_query(
            EpisodeId::new(i as u32 + 1).map_err(|e| format!("{e:?}"))?,
            *observation,
        )?);
    }
    out.diagnosis = diagnose_pedagogy(&out.initial);
    out.history
        .push(format!("Diagnóstico inicial {:?}", out.diagnosis));
    if out.diagnosis == PedagogicalDiagnosis::ExperienceNeeded
        && permissions.presentation == Some(plan.scope)
    {
        let run = run_teaching(host, &plan, &permissions, 3);
        if let Some(error) = run.failure {
            return Err(error);
        }
        out.lessons = run.records;
        for record in &out.lessons {
            out.history.push(format!(
                "Experiência {:?}: redução prevista {}; admitida={}",
                record.lesson.id, record.rejected_hypotheses, record.admitted
            ));
        }
    } else {
        out.history
            .push("Ensino não executado: diagnóstico/permissão".into());
    }
    for (i, observation) in assessment.after.iter().enumerate() {
        out.posterior.push(
            host.pedagogy_query(
                EpisodeId::new((assessment.before.len() + i + 1) as u32)
                    .map_err(|e| format!("{e:?}"))?,
                *observation,
            )?,
        );
    }
    out.final_version = host.pedagogy_version()?;
    out.history.push(format!(
        "Avaliação posterior concluída; versão {}→{}; correção externa não presumida",
        out.initial_version.number, out.final_version.number
    ));
    if out.history.len() > 8 || out.lessons.len() > 3 {
        return Err("limite supervisor pedagógico".into());
    }
    Ok(out)
}

/// Núcleo de coordenação MP09 compartilhado pelos dois adaptadores; não aprende por si.
struct TeachingRun {
    records: Vec<LessonRecord>,
    failure: Option<String>,
}
fn run_teaching(
    host: &mut impl PedagogyHost,
    plan: &TeachingPlan,
    permissions: &PedagogyPermissions,
    limit: usize,
) -> TeachingRun {
    let mut out = TeachingRun {
        records: vec![],
        failure: None,
    };
    let mut used = vec![];
    for _ in 0..limit {
        let version = match host.pedagogy_version() {
            Ok(v) => v,
            Err(e) => {
                out.failure = Some(e);
                break;
            }
        };
        let Some((lesson, rejected)) = choose_lesson(&version, &plan.lessons, &used) else {
            break;
        };
        used.push(lesson.id);
        let admission = match host.pedagogy_present(lesson) {
            Ok(a) => a,
            Err(e) => {
                out.failure = Some(e);
                break;
            }
        };
        let mut record = LessonRecord {
            lesson,
            rejected_hypotheses: rejected,
            admissibility: admission,
            admitted: false,
            revision: None,
        };
        if admission == Admission::Admissible && permissions.admissions.contains(&lesson.id) {
            match host.pedagogy_admit(lesson.id) {
                Ok(v) => {
                    record.revision = Some(v);
                    record.admitted = true;
                }
                Err(e) => out.failure = Some(e),
            }
        }
        let proceed = record.admitted;
        out.records.push(record);
        if !proceed {
            break;
        }
    }
    out
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GoalCriterion {
    SupportedByCurrentEvidence,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConceptGoal {
    pub id: EpisodeId,
    pub concept: NodeId,
    pub scope: Scope,
    pub observation: Observation,
    pub criterion: GoalCriterion,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConceptGoalResult {
    Resolved,
    Partial,
    Abstention,
    Interrupted,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConceptExecution {
    NotStarted,
    Completed,
    Interrupted,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConceptGoalBudget {
    pub queries: usize,
    pub experiences: usize,
}
impl Default for ConceptGoalBudget {
    fn default() -> Self {
        Self {
            queries: 5,
            experiences: 2,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoalQuery {
    pub goal: ConceptGoal,
    pub report: ConceptReport,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GoalLesson {
    pub goal: ConceptGoal,
    pub record: LessonRecord,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptGoalReport {
    pub goal: ConceptGoal,
    pub permissions: PedagogyPermissions,
    pub budget: ConceptGoalBudget,
    pub initial: Option<GoalQuery>,
    pub final_query: Option<GoalQuery>,
    pub independent_before: Vec<GoalQuery>,
    pub independent_after: Vec<GoalQuery>,
    pub strategy: crate::metacognition::concepts::ConceptStrategy,
    pub outcome: ConceptGoalResult,
    pub execution: ConceptExecution,
    pub lessons: Vec<GoalLesson>,
    pub queries: usize,
    pub concept_events: usize,
    pub history: Vec<String>,
    pub reason: String,
}
fn goal_supported(r: &ConceptReport) -> bool {
    matches!(
        r.decision,
        crate::metacognition::concepts::ConceptDecision::Supported(_)
    ) && !r.consulted.is_empty()
        && r.version
            .as_ref()
            .is_some_and(|v| !v.support.is_empty() && !v.criteria.is_empty())
}
/// Um objetivo, uma escolha e uma tentativa MP09. Chave de correção não existe na API.
pub fn supervise_concept_goal(
    host: &mut impl PedagogyHost,
    goal: ConceptGoal,
    plan: TeachingPlan,
    assessment: PedagogyAssessment,
    permissions: PedagogyPermissions,
    budget: ConceptGoalBudget,
) -> Result<ConceptGoalReport, String> {
    use crate::metacognition::concepts::{select_concept_strategy, ConceptStrategy};
    if budget.queries > 5
        || budget.experiences > 2
        || plan.lessons.len() > 2
        || permissions.admissions.len() > 2
        || permissions
            .admissions
            .iter()
            .enumerate()
            .any(|(i, a)| permissions.admissions[..i].contains(a))
    {
        return Err("teto/autorizações do objetivo inválidos".into());
    }
    if (goal.concept, goal.scope) != (plan.concept, plan.scope)
        || host.pedagogy_identity()? != (goal.concept, goal.scope)
    {
        return Err("objetivo/plano/estado incompatíveis".into());
    }
    let mut out = ConceptGoalReport {
        goal,
        permissions: permissions.clone(),
        budget,
        initial: None,
        final_query: None,
        independent_before: vec![],
        independent_after: vec![],
        strategy: ConceptStrategy::Abstain,
        outcome: ConceptGoalResult::Interrupted,
        execution: ConceptExecution::NotStarted,
        lessons: vec![],
        queries: 0,
        concept_events: 0,
        history: vec![format!(
            "Objetivo {:?} fixado; critério {:?}; sem rótulo esperado",
            goal.id, goal.criterion
        )],
        reason: String::new(),
    };
    let known = host.pedagogy_existing()?;
    let probes: Vec<_> = assessment
        .before
        .iter()
        .chain(&assessment.after)
        .copied()
        .collect();
    if probes.contains(&goal.observation)
        || plan
            .lessons
            .iter()
            .map(|l| l.attributes)
            .chain(known)
            .any(|a| probes.iter().any(|o| o.completions().contains(&a)))
        || plan
            .lessons
            .iter()
            .any(|l| goal.observation.completions().contains(&l.attributes))
    {
        out.reason =
            "Sobreposição ensino/objetivo/avaliações; vazamento potencial bloqueado".into();
        out.history.push(out.reason.clone());
        return Ok(out);
    }
    if permissions.evaluation != Some(goal.scope) {
        out.reason = "Avaliação do objetivo não autorizada".into();
        return Ok(out);
    }
    if budget.queries == 0 {
        out.strategy = ConceptStrategy::Stop;
        out.reason = "Sem orçamento para consulta inicial".into();
        return Ok(out);
    }
    let events_before = host.pedagogy_events();
    let initial = host.pedagogy_query(
        EpisodeId::new(1).map_err(|e| format!("{e:?}"))?,
        goal.observation,
    )?;
    out.queries = 1;
    out.history.push(format!(
        "Objetivo {:?}: diagnóstico {:?}, versão {:?}, antes do ensino",
        goal.id,
        diagnose_pedagogy(std::slice::from_ref(&initial)),
        initial.version.as_ref().map(|v| v.number)
    ));
    let next = choose_lesson(&host.pedagogy_version()?, &plan.lessons, &[]);
    let can_teach = permissions.presentation == Some(goal.scope)
        && next.is_some_and(|(l, _)| permissions.admissions.contains(&l.id));
    let needed = 2 + assessment.before.len() + assessment.after.len();
    let capacity = budget.experiences >= plan.lessons.len()
        && budget.experiences > 0
        && needed <= budget.queries
        && host.pedagogy_capacity(plan.lessons.len()).is_ok();
    out.strategy = select_concept_strategy(&initial, can_teach, capacity);
    out.initial = Some(GoalQuery {
        goal,
        report: initial.clone(),
    });
    out.history.push(format!(
        "Objetivo {:?}: seleção única {:?}",
        goal.id, out.strategy
    ));
    match out.strategy {
        ConceptStrategy::Respond => {
            out.execution = ConceptExecution::Completed;
            out.outcome = if goal_supported(&initial) {
                ConceptGoalResult::Resolved
            } else {
                ConceptGoalResult::Abstention
            };
            out.reason =
                "Resposta inicial já sustentada; nenhuma experiência ou consulta duplicada".into();
        }
        ConceptStrategy::Abstain => {
            out.execution = ConceptExecution::Completed;
            let denied = diagnose_pedagogy(std::slice::from_ref(&initial))
                == PedagogicalDiagnosis::ExperienceNeeded
                && next.is_some()
                && !can_teach;
            if denied {
                out.execution = ConceptExecution::Interrupted;
            }
            out.outcome = if denied {
                ConceptGoalResult::Interrupted
            } else {
                ConceptGoalResult::Abstention
            };
            out.reason = if denied {
                "Apresentação/admissão não autorizada"
            } else {
                "Diagnóstico bloqueante ou nenhuma experiência informativa"
            }
            .into();
        }
        ConceptStrategy::Stop => {
            out.execution = ConceptExecution::Interrupted;
            out.reason="Reserva agregada de consultas/experiências/operações/eventos/validade indisponível".into();
        }
        ConceptStrategy::TeachOnce => {
            // Mesmos conjuntos separados MP09; consulta do objetivo NÃO é um item deles.
            for observation in &assessment.before {
                let r = host.pedagogy_query(
                    EpisodeId::new(out.queries as u32 + 1).map_err(|e| format!("{e:?}"))?,
                    *observation,
                )?;
                out.queries += 1;
                out.independent_before.push(GoalQuery { goal, report: r });
            }
            let run = run_teaching(host, &plan, &permissions, budget.experiences);
            for record in run.records {
                out.history.push(format!(
                    "Objetivo {:?}: experiência {:?}, admitida={}, revisão {:?}",
                    goal.id,
                    record.lesson.id,
                    record.admitted,
                    record.revision.as_ref().map(|v| v.number)
                ));
                out.lessons.push(GoalLesson { goal, record });
            }
            out.concept_events = host.pedagogy_events() - events_before;
            if let Some(e) = run.failure {
                out.reason = format!("Ensino interrompido: {e}");
                out.execution = ConceptExecution::Interrupted;
            } else {
                out.execution = ConceptExecution::Completed;
            }
            for observation in &assessment.after {
                let r = host.pedagogy_query(
                    EpisodeId::new(out.queries as u32 + 1).map_err(|e| format!("{e:?}"))?,
                    *observation,
                )?;
                out.queries += 1;
                out.independent_after.push(GoalQuery { goal, report: r });
            }
            let final_report = host.pedagogy_query(
                EpisodeId::new(out.queries as u32 + 1).map_err(|e| format!("{e:?}"))?,
                goal.observation,
            )?;
            out.queries += 1;
            let progress = final_report.version.as_ref().map(|v| v.number)
                > initial.version.as_ref().map(|v| v.number);
            let refusal = out
                .lessons
                .iter()
                .any(|l| !l.record.admitted && l.record.admissibility == Admission::Admissible);
            if refusal {
                out.execution = ConceptExecution::Interrupted;
            }
            out.outcome = if out.execution == ConceptExecution::Interrupted || refusal {
                ConceptGoalResult::Interrupted
            } else if goal_supported(&final_report) {
                ConceptGoalResult::Resolved
            } else if progress {
                ConceptGoalResult::Partial
            } else {
                ConceptGoalResult::Abstention
            };
            if out.reason.is_empty() {
                out.reason = format!(
                    "Resultado {:?}; versão {:?}→{:?}; ensino executado não implica resolução",
                    out.outcome,
                    initial.version.as_ref().map(|v| v.number),
                    final_report.version.as_ref().map(|v| v.number)
                );
            }
            out.final_query = Some(GoalQuery {
                goal,
                report: final_report,
            });
        }
    }
    out.history.push(format!(
        "Objetivo {:?}: {:?}, consultas={}, eventos conceituais={}",
        goal.id, out.outcome, out.queries, out.concept_events
    ));
    if out.queries > budget.queries
        || out.lessons.len() > 2
        || out.history.len() > 8
        || out.concept_events > 10
    {
        return Err("limite agregado do objetivo violado".into());
    }
    Ok(out)
}
