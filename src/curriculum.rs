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
fn choose_lesson(
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
        let mut used = vec![];
        for _ in 0..3 {
            let version = host.pedagogy_version()?;
            let Some((lesson, rejected)) = choose_lesson(&version, &plan.lessons, &used) else {
                break;
            };
            used.push(lesson.id);
            let admission = host.pedagogy_present(lesson)?;
            let mut record = LessonRecord {
                lesson,
                rejected_hypotheses: rejected,
                admissibility: admission,
                admitted: false,
                revision: None,
            };
            if admission == Admission::Admissible && permissions.admissions.contains(&lesson.id) {
                record.revision = Some(host.pedagogy_admit(lesson.id)?);
                record.admitted = true;
            }
            out.history.push(format!(
                "Experiência {:?}: redução prevista {}; admitida={}",
                lesson.id, rejected, record.admitted
            ));
            let continue_teaching = record.admitted;
            out.lessons.push(record);
            if !continue_teaching {
                break;
            } // Preserva a pendência F1 e não tenta outra apresentação após recusa.
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
