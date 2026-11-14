//! Integração funcional sintética, não um novo aceite científico F3.
//! A fonte contém experiências rotuladas, nunca rótulos dos itens de avaliação.
//! Aprendizagem conceitual H23 e propagação neural MP06 permanecem distintas.
use super::composed::{
    Binding, Permissions, Report, Status, SyntheticRegistry, Usage, Verification,
};
use super::composed_two;
use crate::cognition::concepts::Observation;
use crate::cognitive_contracts::{EpisodeId, Scope, SourceId};
use crate::curriculum::{
    teach_concept, Lesson, PedagogyAssessment, PedagogyHost, PedagogyPermissions, PedagogyReport,
    TeachingPlan,
};
use crate::event::{Event, EventKind, NodeId};
use crate::metacognition::concepts::{
    diagnose_pedagogy, select_concept_strategy, ConceptStrategy, PedagogicalDiagnosis,
};
use crate::metacognition::Assessment;
use crate::session::f1::Session;

/// Reserva de dois ciclos F3-01 independentes (inicial/final), mais duas consultas
/// MP09 e até três exemplos x cinco eventos. Nenhum teto interno é ampliado.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Budget {
    pub conceptual_queries: usize,
    pub relational_queries: usize,
    pub neural_events: usize,
    pub conceptual_events: usize,
}
impl Default for Budget {
    fn default() -> Self {
        Self {
            conceptual_queries: 6,
            relational_queries: 4,
            neural_events: 128,
            conceptual_events: 15,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Question {
    pub objective: EpisodeId,
    pub concept: NodeId,
    pub scope: Scope,
    pub version: u32,
    pub remaining_hypotheses: usize,
    pub diagnosis: PedagogicalDiagnosis,
}

/// Fonte finita fixada pelo chamador antes da consulta, sem callback/oráculo.
/// Uma leitura por instância; nenhuma autoridade é serializada.
pub struct LocalExperiences {
    source: SourceId,
    concept: NodeId,
    scope: Scope,
    lessons: Vec<Lesson>,
    consumed: bool,
}
impl LocalExperiences {
    pub fn new(
        source: SourceId,
        concept: NodeId,
        scope: Scope,
        lessons: &[Lesson],
    ) -> Result<Self, String> {
        TeachingPlan::new(concept, scope, lessons)?;
        if lessons.is_empty() || lessons.iter().any(|l| l.source != source) {
            return Err("fonte de experiências vazia/incompatível".into());
        }
        Ok(Self {
            source,
            concept,
            scope,
            lessons: lessons.to_vec(),
            consumed: false,
        })
    }
    pub fn consumed(&self) -> bool {
        self.consumed
    }
    fn fetch(&mut self, question: &Question) -> Result<Vec<Lesson>, String> {
        if self.consumed
            || question.concept != self.concept
            || question.scope != self.scope
            || question.diagnosis != PedagogicalDiagnosis::ExperienceNeeded
        {
            return Err("pedido incompatível ou fonte já consumida".into());
        }
        self.consumed = true;
        Ok(self.lessons.clone())
    }
}

/// Concessões independentes: observar, compor, buscar e admitir exemplos.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authority {
    pub observation: bool,
    pub query: Permissions,
    pub investigate: bool,
    pub source: Option<SourceId>,
    pub pedagogy: PedagogyPermissions,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CycleReport {
    pub objective: EpisodeId,
    pub observation: Event,
    pub binding: Binding,
    pub authority: Authority,
    pub budget: Budget,
    pub capabilities: Vec<&'static str>,
    pub initial: Report,
    pub initial_verification: Verification,
    pub diagnosis: PedagogicalDiagnosis,
    pub strategy: ConceptStrategy,
    pub question: Option<Question>,
    pub source_calls: usize,
    pub teaching: Option<PedagogyReport>,
    pub final_report: Option<Report>,
    pub final_verification: Option<Verification>,
    pub usage: Usage,
    /// Trabalho de reconstrução, distinto de novas experiências/consultas.
    pub replay_operations: usize,
    pub replay_conceptual_events: usize,
    pub committed: bool,
    pub result: Status,
    pub reason: String,
}

/// Uma instância não clonável, consumida pela execução. Conhecimento persiste
/// apenas pelo codec Session F1 existente; este coordenador não tem codec.
pub struct Cycle {
    objective: EpisodeId,
    binding: Binding,
    authority: Authority,
    budget: Budget,
}
impl Cycle {
    pub fn new(
        objective: EpisodeId,
        binding: Binding,
        authority: Authority,
        budget: Budget,
    ) -> Self {
        Self {
            objective,
            binding,
            authority,
            budget,
        }
    }

    pub fn run(
        self,
        session: &mut Session,
        event: Event,
        registry: &SyntheticRegistry,
        source: &mut LocalExperiences,
        transfer_probe: Observation,
    ) -> Result<CycleReport, String> {
        // Reserva completa antes de qualquer consulta, busca ou mutação.
        if self.budget != Budget::default() {
            return Err(
                "reserva funcional requerida: 6/4/128 consultas/eventos e 15 eventos conceituais"
                    .into(),
            );
        }
        let payload = self
            .binding
            .observation
            .complete()
            .map_err(|e| format!("{e:?}"))?
            .encode();
        if !self.authority.observation
            || event.origin().is_some()
            || event.kind() != EventKind::Stimulus
            || event.destination() != self.binding.concept
            || event.payload() != payload
            || event.logical_time() != session.logical_time()
            || registry.record() != &self.binding
        {
            return Err(
                "observação não autorizada, obsoleta ou incompatível com procedência".into(),
            );
        }
        let (initial, initial_verification, usage) = self.read(session, registry)?;
        let diagnosis = initial
            .conceptual
            .as_ref()
            .map_or(PedagogicalDiagnosis::Blocked, |r| {
                diagnose_pedagogy(std::slice::from_ref(r))
            });
        let mut out = CycleReport {
            objective: self.objective,
            observation: event,
            binding: self.binding.clone(),
            authority: self.authority.clone(),
            budget: self.budget,
            capabilities: vec![
                "eventos estruturados",
                "memória/proveniência F1",
                "H23/MP09",
                "prova neural MP06",
                "verificação F3-01",
                "checkpoint F1 v1",
            ],
            result: initial.status,
            initial,
            initial_verification,
            diagnosis,
            strategy: ConceptStrategy::Abstain,
            question: None,
            source_calls: 0,
            teaching: None,
            final_report: None,
            final_verification: None,
            usage,
            replay_operations: 0,
            replay_conceptual_events: 0,
            committed: false,
            reason: "consulta inicial; nenhuma experiência executada".into(),
        };
        if out.initial_verification != Verification::Verified(out.initial.status) {
            out.result = Status::Interrupted;
            out.strategy = ConceptStrategy::Stop;
            out.reason = "consulta inicial não verificada".into();
            return Ok(out);
        }
        if out.initial.status == Status::Supported {
            out.strategy = ConceptStrategy::Respond;
            return Ok(out);
        }
        let relation_sufficient = out.initial.relational.as_ref().is_some_and(|r| {
            r.refusal.is_none()
                && r.proof.as_ref().is_some_and(|p| p.premises.len() == 2)
                && r.review
                    .as_ref()
                    .is_some_and(|v| v.assessment == Assessment::Sufficient)
        });
        if out.initial.status != Status::Insufficient || !relation_sufficient {
            out.reason = "oposição, negação ou lacuna relacional: ensino não elegível".into();
            return Ok(out);
        }
        let can_teach = self.authority.investigate
            && !source.consumed
            && self.authority.source == Some(source.source)
            && source.concept == self.binding.concept
            && source.scope == self.binding.scope
            && self.authority.pedagogy.evaluation == Some(self.binding.scope)
            && self.authority.pedagogy.presentation == Some(self.binding.scope)
            && source
                .lessons
                .iter()
                .all(|l| self.authority.pedagogy.admissions.contains(&l.id));
        let capacity = session.pedagogy_capacity(source.lessons.len()).is_ok();
        out.strategy = select_concept_strategy(
            out.initial.conceptual.as_ref().unwrap(),
            can_teach,
            capacity,
        );
        if out.strategy != ConceptStrategy::TeachOnce {
            out.reason = "diagnóstico, fonte, permissões ou capacidade impedem experiência".into();
            return Ok(out);
        }
        let assessment = PedagogyAssessment::new(&[self.binding.observation], &[transfer_probe])?;
        // Recusa sobreposição antes da busca. Não lê rótulos de avaliação.
        let known = session.pedagogy_existing()?;
        if known
            .iter()
            .copied()
            .chain(source.lessons.iter().map(|l| l.attributes))
            .any(|a| {
                self.binding.observation.completions().contains(&a)
                    || transfer_probe.completions().contains(&a)
            })
        {
            return Err("sobreposição entre experiência e avaliação".into());
        }
        let version = out
            .initial
            .conceptual
            .as_ref()
            .unwrap()
            .version
            .as_ref()
            .ok_or("sem versão")?;
        out.question = Some(Question {
            objective: self.objective,
            concept: self.binding.concept,
            scope: self.binding.scope,
            version: version.number,
            remaining_hypotheses: version.criteria.len(),
            diagnosis,
        });
        // Estágio F1 canônico: falha/contradição não publica aprendizado parcial.
        // Replay de consulta/investigação antiga executaria trabalho cognitivo
        // fora das reservas acima. Só atos de configuração/conhecimento são
        // aceitos no estágio; formatos e replay F1 não são modificados.
        for command in session.history() {
            let verb = command.split_whitespace().next().unwrap_or("");
            if !matches!(
                verb,
                "config"
                    | "simbolo"
                    | "autorizar"
                    | "conceito"
                    | "contexto-conceitual"
                    | "receber"
                    | "interpretar"
                    | "avaliar"
                    | "admitir"
                    | "exemplo"
                    | "interpretar-exemplo"
                    | "avaliar-exemplo"
                    | "admitir-exemplo"
                    | "revisar"
                    | "agora"
            ) {
                return Err(
                    "estágio exige histórico sem consultas/investigações a reexecutar".into(),
                );
            }
        }
        let mut staged = Session::decode(&session.encode())?;
        if staged.snapshot() != session.snapshot() {
            return Err("estado não reproduzível pelo contrato F1".into());
        }
        out.replay_operations = session.history().len();
        out.replay_conceptual_events = staged.pedagogy_events();
        let received = source.fetch(out.question.as_ref().unwrap())?;
        out.source_calls = 1;
        out.usage.teaching_attempts = 1;
        let before = staged.pedagogy_events();
        let teaching = teach_concept(
            &mut staged,
            TeachingPlan::new(source.concept, source.scope, &received)?,
            assessment,
            self.authority.pedagogy.clone(),
        )?;
        out.usage.conceptual_queries += teaching.initial.len() + teaching.posterior.len();
        out.usage.conceptual_events = staged
            .pedagogy_events()
            .checked_sub(before)
            .ok_or("eventos regressivos")?;
        out.teaching = Some(teaching);
        let (final_report, verification, final_usage) = self.read(&staged, registry)?;
        out.usage.conceptual_queries += final_usage.conceptual_queries;
        out.usage.relational_queries += final_usage.relational_queries;
        out.usage.neural_events += final_usage.neural_events;
        if out.usage.conceptual_queries > self.budget.conceptual_queries
            || out.usage.relational_queries > self.budget.relational_queries
            || out.usage.neural_events > self.budget.neural_events
            || out.usage.conceptual_events > self.budget.conceptual_events
        {
            return Err("orçamento agregado excedido; estágio não publicado".into());
        }
        out.result = final_report.status;
        if verification == Verification::Verified(final_report.status)
            && matches!(
                final_report.status,
                Status::Supported | Status::Insufficient
            )
            && session.memory().iter().eq(staged.memory().iter())
            && session.metadata() == staged.metadata()
        {
            *session = staged;
            out.committed = true;
            out.reason = "experiência concluída; objetivo reconsultado e verificado, sem admissão relacional".into();
        } else {
            out.result = Status::Interrupted;
            out.reason = "estágio descartado: verificação, oposição ou alteração factual".into();
        }
        out.final_report = Some(final_report);
        out.final_verification = Some(verification);
        Ok(out)
    }

    fn read(
        &self,
        session: &Session,
        registry: &SyntheticRegistry,
    ) -> Result<(Report, Verification, Usage), String> {
        let mut episode = composed_two::Episode::new(
            self.objective,
            self.binding.clone(),
            self.authority.query.clone(),
            composed_two::Limits::default(),
        )?;
        let report = episode.query(session, registry)?;
        let verification = episode.verify(session, registry, &report)?;
        Ok((report, verification, episode.usage()))
    }
}
