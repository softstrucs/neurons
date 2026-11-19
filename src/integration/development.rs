//! Runtime PIFI: introspecção, percepção, memória e portas cognitivas limitadas.
//! Intervenções/planejamento usam experiências observadas, sem alterar protocolos.
//! Sensores são interfaces sintéticas instaladas, não hardware autodetectado.
use super::synaptic::{Config, SynapticEpisode, UpdateRecord};
use super::{composed, experiential, investigation};
use crate::circuit::CircuitSnapshot;
use crate::cognition::concepts::Observation;
use crate::cognitive_contracts::{
    Concept, DeclaredNature, EpisodeId, Information, Polarity, SourceId, Statement,
};
use crate::curriculum::{
    self, ConceptGoal, ConceptGoalBudget, ConceptGoalReport, PedagogyAssessment,
    PedagogyPermissions, TeachingPlan,
};
use crate::engram::{Cue, Engram, EngramId, EngramStore, EvidenceId, RecordStatus};
use crate::event::{Event, EventKind, NodeId};
use crate::learning::Feedback;
use crate::session::f1::Session;
use crate::synapse::SynapseId;

mod actions;
mod archive;
pub use archive::{
    CatalogCost, ConditionStatus, ConditionedKnowledge, EntityCondition, EpisodeArchive,
    EpisodeCatalog, EpisodeMemory, EvidenceProfile, RecoveredTransition, RecoveredVersion,
    RetrievalBudget, RetrievalStatus, RetrievedMemory, SearchCoverage, SearchLimit, SelectiveQuery,
    SelectiveSearch, SnapshotStorage, VersionRelation,
};
mod checkpoint;
mod consequences;
mod discrimination;
mod epistemic;
pub use discrimination::{
    DiscriminatingInvestigation, DiscriminatingOption, InvestigationOffer, InvestigationResult,
    OperationalInvestigation, ProbeBudget, ProbeSelection, ProbeStatus, ProbeTeaching,
};
mod predictive;
pub use actions::{
    ActionPermit, ActionTrial, Execution, ExecutionReason, Intervention, OutcomeEvidence, Plan,
    PlanOutcome, PlanReason, PlanningBudget, PlanningGoal, Prediction, PredictionStatus, Pursuit,
    PursuitGoal, PursuitReason, PursuitStep, TransitionQuery, TransitionQuestion, World,
};
pub use consequences::{
    AttributeFrame, ConsequenceContract, ConsequenceForecast, ConsequenceReview,
    ConsequenceTraining,
};
pub use epistemic::{
    ConsequenceSample, EpistemicDiagnosis, EpistemicQuestion, EpistemicReport, EpistemicRevision,
};
pub use predictive::{PredictiveChoice, PredictivePursuit, PredictiveReason, PredictiveRevision};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Access {
    pub inspect: bool,
    pub record: bool,
    pub read_knowledge: bool,
    pub explore: bool,
    pub adapt: bool,
    pub learn_concepts: bool,
    pub investigate_relations: bool,
    pub sensors: Vec<u32>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    pub records: usize,
    pub observations: usize,
    pub discoveries: usize,
    /// Teto perceptivo MP05; consultas cognitivas têm reservas separadas.
    pub neural_events: usize,
    pub attention_scans: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            records: 32,
            observations: 8,
            discoveries: 4,
            neural_events: 128,
            attention_scans: 32,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sensor {
    pub id: u32,
    pub source: SourceId,
    pub context: u32,
    pub input: NodeId,
    pub encoding: Encoding,
}
/// Contagem lógica, não limite de CPU/RAM. Cada fase ocorre no máximo uma vez.
/// Tetos compostos: 8 consultas conceituais, 9 relacionais e 288 eventos
/// cognitivos, além dos 128 perceptivos MP05; nenhum teto interno é ampliado.
/// Reservas continuam consumidas quando a operação termina sem relatório.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CognitiveResources {
    pub reserved_conceptual_queries: usize,
    pub reserved_relational_queries: usize,
    pub reserved_neural_events: usize,
    pub conceptual_queries: usize,
    pub relational_queries: usize,
    pub neural_events: usize,
    pub conceptual_events: usize,
    pub operations_without_report: usize,
}
impl CognitiveResources {
    fn reserve(&mut self, concepts: usize, relations: usize, events: usize) {
        self.reserved_conceptual_queries += concepts;
        self.reserved_relational_queries += relations;
        self.reserved_neural_events += events;
        self.operations_without_report += 1;
    }
    fn report(
        &mut self,
        concepts: usize,
        relations: usize,
        events: usize,
        conceptual_events: usize,
    ) {
        self.conceptual_queries += concepts;
        self.relational_queries += relations;
        self.neural_events += events;
        self.conceptual_events += conceptual_events;
        self.operations_without_report -= 1;
    }
}
/// Semântica de entrada explícita; um número escalar não é uma relação.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Encoding {
    Scalar,
    BooleanAttributes,
    Inclusion,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SensorState {
    pub sensor: Sensor,
    pub connected: bool,
    pub permitted: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KnowledgeState {
    pub facts: usize,
    pub examples: usize,
    pub versions: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Discovery {
    pub at: u64,
    pub configuration: Option<Config>,
    pub circuit: Option<CircuitSnapshot>,
    pub sensors: Vec<SensorState>,
    pub knowledge: Option<KnowledgeState>,
    pub access: Access,
    pub limits: Limits,
    pub free_records_before: usize,
    pub diagnostic_records: Vec<EngramId>,
    pub cognitive_resources: CognitiveResources,
    pub perceptual_events: u64,
    pub observations_remaining: usize,
    pub action_attempts: usize,
    pub action_reserved_events: usize,
    pub planning_queries: usize,
    pub planning_scans: usize,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Reading {
    pub id: u32,
    pub item: NodeId,
    pub sensor: u32,
    pub source: SourceId,
    pub evidence: EvidenceId,
    pub event: Event,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Change {
    First,
    Stable,
    Changed { previous: i32 },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Experience {
    pub reading: Reading,
    pub record: EngramId,
    pub change: Change,
    pub neural_events: usize,
    /// Observação temporal, sem alegação de causalidade ou fato validado.
    pub nature: RecordStatus,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttentionGoal {
    pub sensors: Vec<u32>,
    pub prefer_change: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attention {
    pub selected: Option<u32>,
    pub scanned: usize,
    pub reason: &'static str,
}

/// Dados observáveis sem chave de respostas, callback ou ações externas.
pub struct SyntheticEnvironment {
    readings: Vec<Reading>,
}
impl SyntheticEnvironment {
    pub fn new(readings: &[Reading]) -> Result<Self, String> {
        if readings.len() > 8
            || readings
                .iter()
                .enumerate()
                .any(|(i, r)| r.id == 0 || readings[..i].iter().any(|p| p.id == r.id))
        {
            return Err("ambiente excedido ou identidade repetida".into());
        }
        Ok(Self {
            readings: readings.to_vec(),
        })
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FeedbackRequest {
    pub id: u32,
    pub experience: u32,
    pub source: SourceId,
    pub evidence: EvidenceId,
    pub synapse: SynapseId,
    pub signal: Feedback,
    pub at: u64,
}

/// O objetivo e as concessões vêm do chamador; o runtime não inventa autoridade.
pub struct ConceptualTask {
    pub objective: EpisodeId,
    pub binding: composed::Binding,
    pub authority: experiential::Authority,
    pub budget: experiential::Budget,
    pub transfer_probe: Observation,
}

pub struct RelationalTask {
    pub query: Statement,
    pub permissions: investigation::RegulationPermissions,
    pub budget: investigation::RegulationBudget,
}

/// Objetivo conceitual independente: não exige que relações ainda desconhecidas
/// sejam admitidas para que se possa aprender um conceito (MP09 existente).
pub struct LessonTask {
    pub goal: ConceptGoal,
    pub plan: TeachingPlan,
    pub assessment: PedagogyAssessment,
    pub permissions: PedagogyPermissions,
    pub budget: ConceptGoalBudget,
}
pub struct VerificationTask {
    pub objective: EpisodeId,
    pub binding: composed::Binding,
    pub permissions: composed::Permissions,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedObservation {
    pub report: composed::Report,
    pub verification: composed::Verification,
    pub usage: composed::Usage,
}

/// Uma instância finita, sem I/O implícita ou concessões restauradas pelo runtime.
pub struct Runtime {
    neural: SynapticEpisode,
    memory: EngramStore,
    sensors: Vec<Sensor>,
    access: Access,
    limits: Limits,
    discoveries: Vec<Discovery>,
    experiences: Vec<Experience>,
    attention_scans: usize,
    at: u64,
    conceptual_attempted: bool,
    relational_attempted: bool,
    verification_attempted: bool,
    consequence_pending: Option<ConsequenceForecast>,
    consequence_contract: Option<ConsequenceContract>,
    cognitive_resources: CognitiveResources,
    discoveries_used: usize,
    action_trials: Vec<ActionTrial>,
    action_permits: Vec<u32>,
    action_reserved_events: usize,
    planning_queries: usize,
    planning_scans: usize,
}
impl Runtime {
    pub fn new(
        neural: SynapticEpisode,
        sensors: &[Sensor],
        access: Access,
        limits: Limits,
    ) -> Result<Self, String> {
        if limits.records == 0
            || limits.records > 32
            || limits.observations > 8
            || limits.discoveries > 4
            || limits.neural_events > 128
            || limits.attention_scans > 32
            || sensors.len() > 8
            || sensors.iter().enumerate().any(|(i, s)| {
                s.id == 0
                    || s.context == 0
                    || s.context == u32::MAX
                    || sensors[..i].iter().any(|p| p.id == s.id)
            })
            || access.sensors.len() > 8
            || access
                .sensors
                .iter()
                .enumerate()
                .any(|(i, s)| *s == 0 || access.sensors[..i].contains(s))
        {
            return Err("limites ou interfaces inválidos".into());
        }
        let at = neural.circuit().and_then(|c| c.logical_time()).unwrap_or(0);
        Ok(Self {
            neural,
            memory: EngramStore::new(limits.records).map_err(|e| format!("{e:?}"))?,
            sensors: sensors.to_vec(),
            access,
            limits,
            discoveries: vec![],
            experiences: vec![],
            attention_scans: 0,
            at,
            conceptual_attempted: false,
            relational_attempted: false,
            verification_attempted: false,
            consequence_pending: None,
            consequence_contract: None,
            cognitive_resources: CognitiveResources::default(),
            discoveries_used: 0,
            action_trials: vec![],
            action_permits: vec![],
            action_reserved_events: 0,
            planning_queries: 0,
            planning_scans: 0,
        })
    }
    pub fn neural(&self) -> &SynapticEpisode {
        &self.neural
    }
    pub fn memory(&self) -> &EngramStore {
        &self.memory
    }
    pub fn experiences(&self) -> &[Experience] {
        &self.experiences
    }
    pub fn discoveries(&self) -> &[Discovery] {
        &self.discoveries
    }
    pub fn attention_scans(&self) -> usize {
        self.attention_scans
    }
    pub fn cognitive_resources(&self) -> CognitiveResources {
        self.cognitive_resources
    }
    pub fn discoveries_used(&self) -> usize {
        self.discoveries_used
    }
    /// Envelope operacional aditivo. Não serializa Access, planos ou supervisores
    /// do runtime. Os codecs internos preservam seus campos históricos de
    /// configuração/concessão; eles não habilitam o runtime sem Access novo.
    /// Relatórios transitórios de descoberta não são serializados.
    pub fn checkpoint(&self, knowledge: &Session) -> Result<Vec<u8>, String> {
        checkpoint::encode(self, knowledge)
    }
    /// Interfaces e autoridade são sempre fornecidas novamente pelo chamador.
    pub fn restore(
        bytes: &[u8],
        sensors: &[Sensor],
        access: Access,
    ) -> Result<(Self, Session), String> {
        checkpoint::decode(bytes, sensors, access)
    }
    fn snapshot(&self) -> Result<Option<CircuitSnapshot>, String> {
        self.neural
            .circuit()
            .map(|c| c.snapshot().map_err(|e| format!("{e:?}")))
            .transpose()
    }
    fn connected(&self, sensor: &Sensor) -> bool {
        self.neural
            .config()
            .is_some_and(|c| c.context == sensor.context)
            && self
                .neural
                .circuit()
                .is_some_and(|c| c.neuron_state(sensor.input).is_some())
    }
    fn room(&self, records: usize) -> Result<(), String> {
        if !self.access.record || self.memory.len() + records > self.memory.capacity() {
            Err("memória não autorizada ou sem capacidade".into())
        } else {
            Ok(())
        }
    }
    fn next_record(&self) -> EngramId {
        EngramId::new(self.memory.len() as u32 + 1).expect("memória limitada a 32")
    }

    /// Consulta o circuito real. Configuração não é conhecimento adquirido.
    pub fn discover(&mut self, knowledge: Option<&Session>) -> Result<Discovery, String> {
        if !self.access.inspect || self.discoveries_used >= self.limits.discoveries {
            return Err("inspeção negada ou limite de descobertas".into());
        }
        self.room(4)?;
        let circuit = self.snapshot()?;
        let values = [
            circuit.as_ref().map_or(0, |c| c.neurons.len()),
            circuit.as_ref().map_or(0, |c| c.synapses.len()),
            self.neural.circuit().map_or(0, |c| c.queue_len()),
            self.experiences.len(),
        ];
        let mut report = Discovery {
            at: self.at,
            configuration: self.neural.config().cloned(),
            circuit,
            sensors: self
                .sensors
                .iter()
                .map(|s| SensorState {
                    sensor: s.clone(),
                    connected: self.connected(s),
                    permitted: self.access.sensors.contains(&s.id),
                })
                .collect(),
            knowledge: if self.access.read_knowledge {
                knowledge.map(|s| KnowledgeState {
                    facts: s.memory().len(),
                    examples: s.concepts().map_or(0, |c| c.metadata().len()),
                    versions: s
                        .concepts()
                        .map_or(0, |c| c.models().iter().map(|m| m.versions().len()).sum()),
                })
            } else {
                None
            },
            access: self.access.clone(),
            limits: self.limits,
            free_records_before: self.memory.capacity() - self.memory.len(),
            diagnostic_records: vec![],
            cognitive_resources: self.cognitive_resources,
            perceptual_events: self
                .neural
                .circuit()
                .map_or(0, |c| c.engine_metrics().dispatched),
            observations_remaining: self
                .limits
                .observations
                .saturating_sub(self.experiences.len())
                .min(8usize.saturating_sub(self.neural.experiences().len()))
                .min(
                    self.limits.neural_events.saturating_sub(
                        self.neural
                            .circuit()
                            .map_or(0, |c| c.engine_metrics().dispatched as usize)
                            + self.action_reserved_events,
                    ) / 16,
                ),
            action_attempts: self.action_permits.len(),
            action_reserved_events: self.action_reserved_events,
            planning_queries: self.planning_queries,
            planning_scans: self.planning_scans,
        };
        // Contexto interno reservado ao diagnóstico; não se admite fato de domínio.
        for (i, value) in values.iter().enumerate() {
            let id = self.next_record();
            let record = Engram::new(
                id,
                Cue::new(u32::MAX, NodeId::new(1).unwrap(), i as i32 + 1)
                    .map_err(|e| format!("{e:?}"))?,
                *value as i32,
                None,
                self.at,
                None,
                RecordStatus::Observed,
            )
            .map_err(|e| format!("{e:?}"))?;
            self.memory.record(record).map_err(|e| format!("{e:?}"))?;
            report.diagnostic_records.push(id);
        }
        self.discoveries.push(report.clone());
        self.discoveries_used += 1;
        Ok(report)
    }

    pub fn observe(&mut self, reading: Reading) -> Result<Experience, String> {
        self.room(1)?;
        let sensor = self
            .sensors
            .iter()
            .find(|s| s.id == reading.sensor)
            .ok_or("sensor ausente")?;
        if !self.access.sensors.contains(&sensor.id)
            || !self.connected(sensor)
            || sensor.source != reading.source
            || reading.id == 0
            || reading.event.origin().is_some()
            || reading.event.kind() != EventKind::Stimulus
            || reading.event.destination() != sensor.input
            || (sensor.encoding == Encoding::BooleanAttributes
                && !(1..=64).contains(&reading.event.payload()))
            || (sensor.encoding == Encoding::Inclusion
                && (reading.event.payload() == 0 || reading.event.payload() == i32::MIN))
            || reading.event.logical_time() < self.at
            || self.experiences.iter().any(|e| e.reading.id == reading.id)
            || self.experiences.len() >= self.limits.observations
        {
            return Err("observação não autorizada, inválida ou esgotada".into());
        }
        let consumed = self
            .neural
            .circuit()
            .ok_or("circuito ausente")?
            .engine_metrics()
            .dispatched as usize;
        // MP05: DAG de quatro arestas, até 16 eventos por experiência.
        if consumed + self.action_reserved_events + 16 > self.limits.neural_events {
            return Err("sem reserva neural integral".into());
        }
        let previous = self
            .experiences
            .iter()
            .rev()
            .find(|e| e.reading.sensor == reading.sensor && e.reading.item == reading.item);
        let change = match previous.map(|p| p.reading.event.payload()) {
            None => Change::First,
            Some(p) if p == reading.event.payload() => Change::Stable,
            Some(previous) => Change::Changed { previous },
        };
        // Reusa o replay/validação MP05. Nenhum codec ou topologia é alterado.
        let mut staged = SynapticEpisode::decode(&self.neural.encode()?)?;
        staged.execute(&format!(
            "experiencia {} {} {} {} {}",
            reading.id,
            sensor.context,
            sensor.input.get(),
            reading.event.payload(),
            reading.event.logical_time()
        ))?;
        staged.execute("executar")?;
        let latest = staged.experiences().last().ok_or("experiência ausente")?;
        if !latest.valid || !latest.complete {
            return Err("propagação incompleta; estágio não publicado".into());
        }
        let dispatched = staged
            .circuit()
            .ok_or("circuito ausente")?
            .engine_metrics()
            .dispatched as usize;
        let neural_events = dispatched
            .checked_sub(consumed)
            .ok_or("contador neural regressivo")?;
        if dispatched > self.limits.neural_events {
            return Err("orçamento neural excedido".into());
        }
        let record = self.next_record();
        let engram = Engram::new(
            record,
            Cue::new(sensor.context, sensor.input, 1).map_err(|e| format!("{e:?}"))?,
            reading.event.payload(),
            None,
            reading.event.logical_time(),
            Some(reading.evidence),
            RecordStatus::Observed,
        )
        .map_err(|e| format!("{e:?}"))?;
        self.memory.record(engram).map_err(|e| format!("{e:?}"))?;
        self.neural = staged;
        self.at = self
            .neural
            .circuit()
            .and_then(|c| c.logical_time())
            .unwrap_or(reading.event.logical_time());
        let experience = Experience {
            reading,
            record,
            change,
            neural_events,
            nature: RecordStatus::Observed,
        };
        self.experiences.push(experience.clone());
        Ok(experience)
    }

    /// Atenção por objetivo explícito e mudança observada, não por acerto futuro.
    pub fn explore(
        &mut self,
        environment: &SyntheticEnvironment,
        goal: &AttentionGoal,
    ) -> Result<(Attention, Option<Experience>), String> {
        if !self.access.explore || goal.sensors.is_empty() || goal.sensors.len() > 8 {
            return Err("exploração não autorizada ou objetivo inválido".into());
        }
        let scans = environment.readings.len();
        if self.attention_scans + scans > self.limits.attention_scans {
            return Err("orçamento de atenção esgotado".into());
        }
        self.attention_scans += scans;
        let selected = environment
            .readings
            .iter()
            .filter(|r| {
                goal.sensors.contains(&r.sensor)
                    && self.access.sensors.contains(&r.sensor)
                    && r.event.logical_time() >= self.at
                    && !self.experiences.iter().any(|e| e.reading.id == r.id)
                    && self
                        .sensors
                        .iter()
                        .any(|s| s.id == r.sensor && s.source == r.source && self.connected(s))
            })
            .max_by_key(|r| {
                let changed = self
                    .experiences
                    .iter()
                    .rev()
                    .find(|e| e.reading.sensor == r.sensor && e.reading.item == r.item)
                    .is_some_and(|e| e.reading.event.payload() != r.event.payload());
                (
                    goal.prefer_change && changed,
                    std::cmp::Reverse(r.event.logical_time()),
                    std::cmp::Reverse(r.id),
                )
            })
            .copied();
        let attention = Attention {
            selected: selected.map(|r| r.id),
            scanned: scans,
            reason: if selected.is_some() {
                "objetivo autorizado; mudança/ordem temporal"
            } else {
                "sem observação elegível"
            },
        };
        let experience = selected.map(|r| self.observe(r)).transpose()?;
        Ok((attention, experience))
    }

    /// Porta de engenharia: feedback explícito, não avaliação causal científica.
    /// O runtime não escolhe o sinal nem cria a concessão MP05 da fonte.
    pub fn feedback(&mut self, request: FeedbackRequest) -> Result<UpdateRecord, String> {
        if !self.access.adapt
            || !self
                .experiences
                .iter()
                .any(|e| e.reading.id == request.experience)
        {
            return Err("adaptação negada ou experiência ausente".into());
        }
        let context = self.neural.config().ok_or("circuito ausente")?.context;
        let signal = match request.signal {
            Feedback::Positive => "+",
            Feedback::Negative => "-",
        };
        self.neural.execute(&format!(
            "feedback {} {} {} {} {} {} {} {}",
            request.id,
            request.experience,
            context,
            request.source.get(),
            request.evidence.get(),
            request.synapse.get(),
            signal,
            request.at
        ))?;
        self.at = self.at.max(request.at);
        self.neural
            .updates()
            .last()
            .cloned()
            .ok_or("alteração ausente".into())
    }

    /// Um ciclo conceitual por instância, vinculado a uma observação efetiva.
    /// Seus orçamentos de consulta são os do coordenador experiencial, separados
    /// do teto perceptivo MP05; não se reembolsam episódios consumidos.
    pub fn learn(
        &mut self,
        session: &mut Session,
        reading_id: u32,
        task: ConceptualTask,
        registry: &composed::SyntheticRegistry,
        source: &mut experiential::LocalExperiences,
    ) -> Result<experiential::CycleReport, String> {
        if !self.access.learn_concepts || self.conceptual_attempted {
            return Err("aprendizagem conceitual negada ou episódio consumido".into());
        }
        self.conceptual_attempted = true;
        self.cognitive_resources.reserve(6, 4, 128);
        let r = self
            .experiences
            .iter()
            .find(|e| e.reading.id == reading_id)
            .ok_or("observação de aprendizagem ausente")?
            .reading;
        let payload = task
            .binding
            .observation
            .complete()
            .map_err(|e| format!("{e:?}"))?
            .encode();
        let sensor = self
            .sensors
            .iter()
            .find(|s| s.id == r.sensor)
            .ok_or("sensor ausente")?;
        if task.binding.item != r.item
            || task.binding.source != r.source
            || task.binding.evidence != r.evidence
            || payload != r.event.payload()
            || sensor.context != task.binding.scope.context.get()
            || sensor.encoding != Encoding::BooleanAttributes
            || r.event.logical_time() != session.logical_time()
        {
            return Err("vínculo conceitual não corresponde à observação registrada".into());
        }
        let event = Event::new(
            None,
            task.binding.concept,
            EventKind::Stimulus,
            payload,
            r.event.logical_time(),
        );
        let report =
            experiential::Cycle::new(task.objective, task.binding, task.authority, task.budget)
                .run(session, event, registry, source, task.transfer_probe)?;
        self.cognitive_resources.report(
            report.usage.conceptual_queries,
            report.usage.relational_queries,
            report.usage.neural_events,
            report.usage.conceptual_events,
        );
        Ok(report)
    }

    /// Sensor relacional estruturado: item é sujeito, payload assinado é destino
    /// (+ inclusão observada, - oposição). É interpretação de uma leitura,
    /// não descoberta de causalidade nem declaração de verdade por ID da fonte.
    /// MP07/MP08 decidem lacuna, busca e admissão; o runtime não insere fatos.
    pub fn investigate(
        &mut self,
        session: &mut Session,
        readings: &[u32],
        task: RelationalTask,
    ) -> Result<investigation::RegulationReport, String> {
        if !self.access.investigate_relations || self.relational_attempted {
            return Err("investigação relacional negada ou consumida".into());
        }
        self.relational_attempted = true;
        self.cognitive_resources.reserve(0, 3, 96);
        let scope = task.query.scope();
        let mut information = vec![];
        let mut source = None;
        if readings.is_empty() || readings.len() > 8 {
            return Err("investigação exige uma a oito observações".into());
        }
        for (index, id) in readings.iter().enumerate() {
            if readings[..index].contains(id) {
                return Err("observação relacional repetida".into());
            }
            let r = self
                .experiences
                .iter()
                .find(|e| e.reading.id == *id)
                .ok_or("observação relacional ausente")?
                .reading;
            let sensor = self
                .sensors
                .iter()
                .find(|s| s.id == r.sensor)
                .ok_or("sensor ausente")?;
            if sensor.context != scope.context.get()
                || sensor.encoding != Encoding::Inclusion
                || !scope.validity.contains(r.event.logical_time())
                || r.event.logical_time() > session.logical_time()
                || source.is_some_and(|s| s != r.source)
            {
                return Err("contexto, instante ou fonte relacional incompatível".into());
            }
            source = Some(r.source);
            let payload = r.event.payload();
            let destination = NodeId::new(payload.unsigned_abs()).map_err(|e| format!("{e:?}"))?;
            let subject = Concept::new(r.item, scope.catalog, "observado", "identidade do sensor")
                .map_err(|e| format!("{e:?}"))?;
            let object = Concept::new(
                destination,
                scope.catalog,
                "observado",
                "identidade do sensor",
            )
            .map_err(|e| format!("{e:?}"))?;
            let statement = Statement::inclusion(
                &subject,
                &object,
                if payload > 0 {
                    Polarity::Positive
                } else {
                    Polarity::Negative
                },
                scope,
            )
            .map_err(|e| format!("{e:?}"))?;
            information.push(Information {
                statement,
                declared_source: Some(r.source),
                evidence: Some(r.evidence),
                nature: DeclaredNature::Observation,
                received_at: r.event.logical_time(),
            });
        }
        let source = investigation::LocalSource::new(source.ok_or("fonte ausente")?, &information)?;
        let report =
            investigation::regulate(session, source, task.query, task.permissions, task.budget)?;
        self.cognitive_resources
            .report(0, report.queries, report.events, 0);
        Ok(report)
    }

    pub fn teach_observed(
        &mut self,
        session: &mut Session,
        reading_id: u32,
        task: LessonTask,
    ) -> Result<ConceptGoalReport, String> {
        if !self.access.learn_concepts || self.conceptual_attempted {
            return Err("aprendizagem conceitual negada ou episódio consumido".into());
        }
        self.conceptual_attempted = true;
        self.cognitive_resources.reserve(5, 0, 0);
        let reading = self
            .experiences
            .iter()
            .find(|e| e.reading.id == reading_id)
            .ok_or("observação ausente")?
            .reading;
        let sensor = self
            .sensors
            .iter()
            .find(|s| s.id == reading.sensor)
            .ok_or("sensor ausente")?;
        if sensor.encoding != Encoding::BooleanAttributes
            || sensor.context != task.goal.scope.context.get()
            || task
                .goal
                .observation
                .complete()
                .map_err(|e| format!("{e:?}"))?
                .encode()
                != reading.event.payload()
            || reading.event.logical_time() > session.logical_time()
        {
            return Err("objetivo conceitual incompatível com percepção".into());
        }
        let report = curriculum::supervise_concept_goal(
            session,
            task.goal,
            task.plan,
            task.assessment,
            task.permissions,
            task.budget,
        )?;
        self.cognitive_resources
            .report(report.queries, 0, 0, report.concept_events);
        Ok(report)
    }

    /// Uma consulta+conferência F3-01, após as experiências. Relatórios antigos
    /// não são reutilizados como atuais. Sem escrita, ensino ou concessões novas.
    pub fn verify_observed(
        &mut self,
        session: &Session,
        reading_id: u32,
        task: VerificationTask,
        registry: &composed::SyntheticRegistry,
    ) -> Result<VerifiedObservation, String> {
        if !self.access.read_knowledge || self.verification_attempted {
            return Err("verificação negada ou consumida".into());
        }
        self.verification_attempted = true;
        self.cognitive_resources.reserve(2, 2, 64);
        let reading = self
            .experiences
            .iter()
            .find(|e| e.reading.id == reading_id)
            .ok_or("observação ausente")?
            .reading;
        let sensor = self
            .sensors
            .iter()
            .find(|s| s.id == reading.sensor)
            .ok_or("sensor ausente")?;
        if sensor.encoding != Encoding::BooleanAttributes
            || sensor.context != task.binding.scope.context.get()
            || reading.item != task.binding.item
            || reading.source != task.binding.source
            || reading.evidence != task.binding.evidence
            || task
                .binding
                .observation
                .complete()
                .map_err(|e| format!("{e:?}"))?
                .encode()
                != reading.event.payload()
            || reading.event.logical_time() > session.logical_time()
        {
            return Err("vínculo de verificação incompatível com percepção".into());
        }
        let mut episode = super::composed_two::Episode::new(
            task.objective,
            task.binding,
            task.permissions,
            Default::default(),
        )?;
        let report = episode.query(session, registry)?;
        let verification = episode.verify(session, registry, &report)?;
        let usage = episode.usage();
        self.cognitive_resources.report(
            usage.conceptual_queries,
            usage.relational_queries,
            usage.neural_events,
            0,
        );
        Ok(VerifiedObservation {
            report,
            verification,
            usage,
        })
    }
}
