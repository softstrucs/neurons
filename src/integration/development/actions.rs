//! Intervenções locais finitas e planejamento sobre consequências observadas.
//! A tabela privada do ambiente não é entrada do planejador. Nenhuma transição
//! vira fato relacional ou lei causal por consulta/planejamento.
use super::*;
use std::collections::VecDeque;

pub struct World {
    id: u32,
    revision: u32,
    sensor: Sensor,
    state: i32,
    transitions: Vec<(i32, u32, i32)>,
    actions: usize,
}
impl World {
    pub(super) fn attribute_query(&self, action: u32) -> TransitionQuery {
        TransitionQuery {
            world: self.id,
            revision: self.revision,
            context: self.sensor.context,
            source: self.sensor.source,
            from: self.state,
            action,
        }
    }
    pub fn new(
        id: u32,
        revision: u32,
        sensor: Sensor,
        initial: i32,
        transitions: &[(i32, u32, i32)],
    ) -> Result<Self, String> {
        if id == 0
            || revision == 0
            || sensor.encoding != Encoding::Scalar
            || transitions.len() > 8
            || transitions
                .iter()
                .enumerate()
                .any(|(i, t)| t.1 == 0 || transitions[..i].iter().any(|p| p.0 == t.0 && p.1 == t.1))
        {
            return Err("ambiente de intervenção inválido".into());
        }
        Ok(Self {
            id,
            revision,
            sensor,
            state: initial,
            transitions: transitions.to_vec(),
            actions: 0,
        })
    }
    pub fn state(&self) -> i32 {
        self.state
    }
    pub fn action_count(&self) -> usize {
        self.actions
    }
    /// Controles habilitados observáveis; não revela destinos ou recompensas.
    /// O planejador não recebe esta interface ou a tabela de transições.
    pub fn available_actions(&self) -> Vec<u32> {
        let mut actions = self
            .transitions
            .iter()
            .filter(|t| t.0 == self.state)
            .map(|t| t.1)
            .collect::<Vec<_>>();
        actions.sort_unstable();
        actions
    }
    fn staged(&self) -> Self {
        Self {
            id: self.id,
            revision: self.revision,
            sensor: self.sensor.clone(),
            state: self.state,
            transitions: self.transitions.clone(),
            actions: self.actions,
        }
    }
    fn step(&mut self, action: u32) -> Result<(), String> {
        if self.actions >= 8 {
            return Err("limite de intervenções do ambiente".into());
        }
        let next = self
            .transitions
            .iter()
            .find(|t| t.0 == self.state && t.1 == action)
            .ok_or("consequência indisponível neste estado")?
            .2;
        self.state = next;
        self.actions += 1;
        Ok(())
    }
    fn reading(&self, id: u32, evidence: EvidenceId, at: u64) -> Reading {
        Reading {
            id,
            item: NodeId::new(self.id).unwrap(),
            sensor: self.sensor.id,
            source: self.sensor.source,
            evidence,
            event: Event::new(None, self.sensor.input, EventKind::Stimulus, self.state, at),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionPermit {
    pub id: u32,
    pub world: u32,
    pub revision: u32,
    pub context: u32,
    pub source: SourceId,
    pub action: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Intervention {
    pub id: u32,
    pub permit: ActionPermit,
    pub before_id: u32,
    pub after_id: u32,
    pub before_evidence: EvidenceId,
    pub after_evidence: EvidenceId,
    pub at: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionTrial {
    pub id: u32,
    pub permit: u32,
    pub world: u32,
    pub revision: u32,
    pub context: u32,
    pub source: SourceId,
    pub action: u32,
    pub from: i32,
    pub to: i32,
    pub before: EngramId,
    pub after: EngramId,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanningGoal {
    pub world: u32,
    pub revision: u32,
    pub context: u32,
    pub source: SourceId,
    pub from: i32,
    pub to: i32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanningBudget {
    pub steps: usize,
    pub scans: usize,
}
impl Default for PlanningBudget {
    fn default() -> Self {
        Self {
            steps: 4,
            scans: 32,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanReason {
    Supported,
    AlreadyAtGoal,
    InsufficientEvidence,
    ContradictoryEvidence,
    ResourceLimited,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionQuestion {
    pub goal: PlanningGoal,
    pub observed_trials: usize,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransitionQuery {
    pub world: u32,
    pub revision: u32,
    pub context: u32,
    pub source: SourceId,
    pub from: i32,
    pub action: u32,
}
impl TransitionQuery {
    fn matches(self, t: &ActionTrial) -> bool {
        t.world == self.world
            && t.revision == self.revision
            && t.context == self.context
            && t.source == self.source
            && t.from == self.from
            && t.action == self.action
    }
    fn of(t: &ActionTrial) -> Self {
        Self {
            world: t.world,
            revision: t.revision,
            context: t.context,
            source: t.source,
            from: t.from,
            action: t.action,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeEvidence {
    pub to: i32,
    pub trials: Vec<u32>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PredictionStatus {
    Unobserved,
    Observed,
    Conflicting,
    ResourceLimited,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Prediction {
    pub query: TransitionQuery,
    pub status: PredictionStatus,
    pub outcomes: Vec<OutcomeEvidence>,
    pub scans: usize,
}
/// Prova transitória imutável; não contém permissões de execução.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    goal: PlanningGoal,
    steps: Vec<ActionTrial>,
    dependencies: Vec<ActionTrial>,
}
impl Plan {
    pub fn goal(&self) -> PlanningGoal {
        self.goal
    }
    pub fn steps(&self) -> &[ActionTrial] {
        &self.steps
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanOutcome {
    pub reason: PlanReason,
    pub plan: Option<Plan>,
    pub question: Option<TransitionQuestion>,
    pub scans: usize,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionReason {
    Reached,
    Observed,
    DeniedOrStale,
    ResourceLimited,
    Interrupted,
    Diverged,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Execution {
    pub goal: PlanningGoal,
    pub reason: ExecutionReason,
    pub trials: Vec<ActionTrial>,
    pub observed: i32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PursuitGoal {
    pub objective: EpisodeId,
    pub task: PlanningGoal,
    pub max_actions: usize,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PursuitReason {
    Reached,
    NoAuthorizedAction,
    ContradictoryEvidence,
    ResourceLimited,
    Interrupted,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PursuitStep {
    pub local_goal: PlanningGoal,
    pub planning: PlanReason,
    pub question: Option<TransitionQuestion>,
    pub exploratory: bool,
    pub execution: Option<Execution>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pursuit {
    pub goal: PursuitGoal,
    pub reason: PursuitReason,
    pub steps: Vec<PursuitStep>,
    pub observed: i32,
}

impl Runtime {
    pub fn action_trials(&self) -> &[ActionTrial] {
        &self.action_trials
    }
    pub fn action_attempts(&self) -> usize {
        self.action_permits.len()
    }
    pub fn action_reserved_events(&self) -> usize {
        self.action_reserved_events
    }
    pub fn planning_scans(&self) -> usize {
        self.planning_scans
    }
    pub(super) fn outcomes_in(
        trials: &[ActionTrial],
        query: TransitionQuery,
    ) -> Vec<OutcomeEvidence> {
        let mut outcomes = Vec::<OutcomeEvidence>::new();
        for trial in trials.iter().filter(|t| query.matches(t)) {
            if let Some(outcome) = outcomes.iter_mut().find(|o| o.to == trial.to) {
                outcome.trials.push(trial.id);
            } else {
                outcomes.push(OutcomeEvidence {
                    to: trial.to,
                    trials: vec![trial.id],
                });
            }
        }
        outcomes.sort_by_key(|o| o.to);
        outcomes
    }
    /// Consulta contextual com alternativas e referências reais, sem consultar
    /// o ambiente ou converter frequência em confiança/verdade. Compartilha
    /// quatro consultas e 32 inspeções com o planejamento do mesmo episódio.
    pub fn predict(&mut self, query: TransitionQuery) -> Result<Prediction, String> {
        self.predict_using(query, &self.action_trials.clone())
    }
    pub(super) fn predict_using(
        &mut self,
        query: TransitionQuery,
        trials: &[ActionTrial],
    ) -> Result<Prediction, String> {
        if !self.access.read_knowledge
            || query.world == 0
            || query.revision == 0
            || query.context == 0
            || query.context == u32::MAX
            || query.action == 0
        {
            return Err("consulta de transição não autorizada/inválida".into());
        }
        let mut out = Prediction {
            query,
            status: PredictionStatus::ResourceLimited,
            outcomes: vec![],
            scans: 0,
        };
        if self.planning_queries >= 4 {
            return Ok(out);
        }
        self.planning_queries += 1;
        if self.planning_scans + trials.len() > 32 {
            return Ok(out);
        }
        out.scans = trials.len();
        self.planning_scans += out.scans;
        out.outcomes = Self::outcomes_in(trials, query);
        out.status = match out.outcomes.len() {
            0 => PredictionStatus::Unobserved,
            1 => PredictionStatus::Observed,
            _ => PredictionStatus::Conflicting,
        };
        Ok(out)
    }
    fn stage_intervention(&self) -> Result<Self, String> {
        let mut memory = EngramStore::new(self.memory.capacity()).map_err(|e| format!("{e:?}"))?;
        for record in self.memory.iter() {
            memory.record(record).map_err(|e| format!("{e:?}"))?;
        }
        Ok(Self {
            neural: SynapticEpisode::decode(&self.neural.encode()?)?,
            memory,
            sensors: self.sensors.clone(),
            access: self.access.clone(),
            limits: self.limits,
            discoveries: self.discoveries.clone(),
            experiences: self.experiences.clone(),
            attention_scans: self.attention_scans,
            at: self.at,
            conceptual_attempted: self.conceptual_attempted,
            relational_attempted: self.relational_attempted,
            verification_attempted: self.verification_attempted,
            consequence_pending: self.consequence_pending.clone(),
            consequence_contract: self.consequence_contract,
            cognitive_resources: self.cognitive_resources,
            discoveries_used: self.discoveries_used,
            action_trials: self.action_trials.clone(),
            action_permits: self.action_permits.clone(),
            action_reserved_events: self.action_reserved_events,
            planning_queries: self.planning_queries,
            planning_scans: self.planning_scans,
        })
    }
    pub(super) fn world_observable(&self, world: &World) -> bool {
        self.sensors.contains(&world.sensor)
            && self.access.sensors.contains(&world.sensor.id)
            && self.connected(&world.sensor)
    }
    pub(super) fn permit_matches(&self, world: &World, request: &Intervention) -> bool {
        self.access.explore
            && self.access.record
            && request.id != 0
            && request.permit.id != 0
            && request.permit.action != 0
            && request.permit.world == world.id
            && request.permit.revision == world.revision
            && request.permit.context == world.sensor.context
            && request.permit.source == world.sensor.source
            && self.sensors.contains(&world.sensor)
            && self.access.sensors.contains(&world.sensor.id)
            && self.connected(&world.sensor)
            && !self.action_permits.contains(&request.permit.id)
            && !self.action_trials.iter().any(|t| t.id == request.id)
    }
    pub(super) fn intervention_room(&self, count: usize) -> Result<(), String> {
        self.room(count * 2)?;
        let circuit = self.neural.circuit().ok_or("circuito ausente")?;
        if self.action_permits.len() + count > 4
            || self.experiences.len() + count * 2 > self.limits.observations
            || self.neural.experiences().len() + count * 2 > 8
            || circuit.engine_metrics().dispatched as usize
                + self.action_reserved_events
                + count * 32
                > self.limits.neural_events
        {
            return Err("sem reserva integral de intervenção".into());
        }
        Ok(())
    }
    /// Uma intervenção efetiva com observações antes/depois, registradas pelo
    /// Circuit/MP05 existente. Falha conserva reserva e consumo da concessão.
    /// O commit é apenas de estado em memória, nunca uma operação Git.
    pub fn intervene(
        &mut self,
        world: &mut World,
        request: &Intervention,
    ) -> Result<ActionTrial, String> {
        if !self.permit_matches(world, request)
            || request.at < self.at
            || request.before_id == 0
            || request.after_id == 0
            || request.before_id == request.after_id
            || request.before_evidence == request.after_evidence
            || self.experiences.iter().any(|e| {
                [request.before_id, request.after_id].contains(&e.reading.id)
                    || [request.before_evidence, request.after_evidence]
                        .contains(&e.reading.evidence)
            })
        {
            return Err("intervenção negada, repetida ou incompatível".into());
        }
        self.intervention_room(1)?;
        self.action_permits.push(request.permit.id);
        self.action_reserved_events += 32;
        let mut staged = self.stage_intervention()?;
        staged.action_reserved_events -= 32;
        let mut environment = world.staged();
        let from = environment.state;
        let before = staged.observe(environment.reading(
            request.before_id,
            request.before_evidence,
            request.at,
        ))?;
        environment.step(request.permit.action)?;
        let after_at = staged
            .at
            .checked_add(1)
            .ok_or("tempo de intervenção excedido")?;
        let after = staged.observe(environment.reading(
            request.after_id,
            request.after_evidence,
            after_at,
        ))?;
        let record = ActionTrial {
            id: request.id,
            permit: request.permit.id,
            world: world.id,
            revision: world.revision,
            context: world.sensor.context,
            source: world.sensor.source,
            action: request.permit.action,
            from,
            to: environment.state,
            before: before.record,
            after: after.record,
        };
        staged.action_trials.push(record.clone());
        *self = staged;
        *world = environment;
        Ok(record)
    }

    /// Busca finita sobre experiências efetivas; não recebe o ambiente nem sua
    /// tabela de transições. A conclusão é uma previsão operacional revogável.
    pub fn plan(
        &mut self,
        goal: PlanningGoal,
        budget: PlanningBudget,
    ) -> Result<PlanOutcome, String> {
        self.plan_using(goal, budget, &self.action_trials.clone())
    }
    pub(super) fn plan_using(
        &mut self,
        goal: PlanningGoal,
        budget: PlanningBudget,
        trials: &[ActionTrial],
    ) -> Result<PlanOutcome, String> {
        if !self.access.read_knowledge
            || goal.world == 0
            || goal.revision == 0
            || goal.context == 0
            || goal.context == u32::MAX
            || budget.steps > 4
            || budget.scans > 32
        {
            return Err("planejamento não autorizado ou inválido".into());
        }
        let mut out = PlanOutcome {
            reason: PlanReason::ResourceLimited,
            plan: None,
            question: None,
            scans: 0,
        };
        if self.planning_queries >= 4 {
            return Ok(out);
        }
        self.planning_queries += 1;
        if goal.from == goal.to {
            out.reason = PlanReason::AlreadyAtGoal;
            out.plan = Some(Plan {
                goal,
                steps: vec![],
                dependencies: trials.to_vec(),
            });
            return Ok(out);
        }
        if budget.steps == 0 || budget.scans == 0 {
            return Ok(out);
        }
        let mut queue = VecDeque::from([(goal.from, Vec::<ActionTrial>::new())]);
        let mut visited = vec![goal.from];
        let mut conflict = false;
        let mut depth_limited = false;
        while let Some((state, path)) = queue.pop_front() {
            if path.len() >= budget.steps {
                depth_limited = true;
                continue;
            }
            let mut edges = vec![];
            for t in trials {
                if out.scans >= budget.scans || self.planning_scans >= 32 {
                    return Ok(out);
                }
                out.scans += 1;
                self.planning_scans += 1;
                if t.world != goal.world
                    || t.revision != goal.revision
                    || t.context != goal.context
                    || t.source != goal.source
                    || t.from != state
                {
                    continue;
                }
                let opposed = Self::outcomes_in(trials, TransitionQuery::of(t)).len() > 1;
                if opposed {
                    conflict = true;
                    continue;
                }
                edges.push(t.clone());
            }
            edges.sort_by_key(|t| (t.action, t.id));
            for t in edges {
                let next = t.to;
                let mut proof = path.clone();
                proof.push(t);
                if next == goal.to {
                    out.reason = PlanReason::Supported;
                    out.plan = Some(Plan {
                        goal,
                        steps: proof,
                        dependencies: trials.to_vec(),
                    });
                    return Ok(out);
                }
                if !visited.contains(&next) {
                    visited.push(next);
                    queue.push_back((next, proof));
                }
            }
        }
        out.reason = if conflict {
            PlanReason::ContradictoryEvidence
        } else if depth_limited {
            PlanReason::ResourceLimited
        } else {
            PlanReason::InsufficientEvidence
        };
        if out.reason == PlanReason::InsufficientEvidence {
            out.question = Some(TransitionQuestion {
                goal,
                observed_trials: trials
                    .iter()
                    .filter(|t| {
                        t.world == goal.world
                            && t.revision == goal.revision
                            && t.context == goal.context
                            && t.source == goal.source
                    })
                    .count(),
            });
        }
        Ok(out)
    }
    /// Não cria permissões. Uma divergência preserva a consequência real e
    /// interrompe o restante do plano; ação executada não equivale a objetivo.
    pub fn execute_plan(
        &mut self,
        world: &mut World,
        plan: &Plan,
        requests: &[Intervention],
    ) -> Execution {
        self.execute_plan_using(world, plan, requests, &self.action_trials.clone())
    }
    pub(super) fn execute_plan_using(
        &mut self,
        world: &mut World,
        plan: &Plan,
        requests: &[Intervention],
        trials: &[ActionTrial],
    ) -> Execution {
        let goal = plan.goal;
        let mut out = Execution {
            goal,
            reason: ExecutionReason::DeniedOrStale,
            trials: vec![],
            observed: world.state,
        };
        if plan.dependencies != trials
            || goal.world != world.id
            || goal.revision != world.revision
            || goal.context != world.sensor.context
            || goal.source != world.sensor.source
            || goal.from != world.state
            || requests.len() != plan.steps.len()
            || !self.access.explore
            || !self.access.read_knowledge
            || !self.sensors.contains(&world.sensor)
            || !self.access.sensors.contains(&world.sensor.id)
            || !self.connected(&world.sensor)
        {
            return out;
        }
        if requests
            .iter()
            .zip(&plan.steps)
            .any(|(r, s)| r.permit.action != s.action)
            || !self.valid_requests(world, requests)
        {
            return out;
        }
        if self.intervention_room(requests.len()).is_err() {
            out.reason = ExecutionReason::ResourceLimited;
            return out;
        }
        for (request, step) in requests.iter().zip(&plan.steps) {
            let result = self.intervene(world, request);
            out.observed = world.state;
            let Ok(trial) = result else {
                out.reason = ExecutionReason::Interrupted;
                return out;
            };
            let matches = trial.from == step.from && trial.to == step.to;
            out.trials.push(trial);
            if !matches {
                out.reason = ExecutionReason::Diverged;
                return out;
            }
        }
        out.reason = if world.state == goal.to {
            ExecutionReason::Reached
        } else {
            ExecutionReason::Diverged
        };
        out
    }

    pub(super) fn valid_requests(&self, world: &World, requests: &[Intervention]) -> bool {
        let mut ids = vec![];
        let mut trial_ids = vec![];
        let mut permits = vec![];
        let mut evidence = vec![];
        for request in requests {
            if !self.permit_matches(world, request)
                || request.before_id == request.after_id
                || request.at < self.at
                || trial_ids.contains(&request.id)
                || request.before_evidence == request.after_evidence
                || permits.contains(&request.permit.id)
                || [request.before_id, request.after_id].iter().any(|id| {
                    *id == 0
                        || ids.contains(id)
                        || self.experiences.iter().any(|e| e.reading.id == *id)
                })
                || [request.before_evidence, request.after_evidence]
                    .iter()
                    .any(|id| {
                        evidence.contains(id)
                            || self.experiences.iter().any(|e| e.reading.evidence == *id)
                    })
            {
                return false;
            }
            ids.extend([request.before_id, request.after_id]);
            trial_ids.push(request.id);
            permits.push(request.permit.id);
            evidence.extend([request.before_evidence, request.after_evidence]);
        }
        true
    }
    /// Lacuna -> pergunta -> controle disponível/autorizado -> intervenção ->
    /// memória -> reconsulta. Não escolhe ações por seus destinos ocultos.
    pub fn pursue(
        &mut self,
        world: &mut World,
        goal: PursuitGoal,
        requests: &[Intervention],
    ) -> Result<Pursuit, String> {
        if !self.access.explore
            || !self.access.read_knowledge
            || !self.access.record
            || !self.sensors.contains(&world.sensor)
            || !self.access.sensors.contains(&world.sensor.id)
            || !self.connected(&world.sensor)
            || goal.max_actions > 4
            || requests.len() > 4
            || goal.task.world != world.id
            || goal.task.revision != world.revision
            || goal.task.context != world.sensor.context
            || goal.task.source != world.sensor.source
            || goal.task.from != world.state
        {
            return Err("objetivo/autoridade de investigação ativa incompatível".into());
        }
        let mut out = Pursuit {
            goal,
            reason: PursuitReason::ResourceLimited,
            steps: vec![],
            observed: world.state,
        };
        let initial_attempts = self.action_permits.len();
        loop {
            out.observed = world.state;
            if world.state == goal.task.to {
                out.reason = PursuitReason::Reached;
                return Ok(out);
            }
            let used = self.action_permits.len() - initial_attempts;
            if used >= goal.max_actions || out.steps.len() >= 4 {
                return Ok(out);
            }
            let local_goal = PlanningGoal {
                from: world.state,
                ..goal.task
            };
            let planning = self.plan(local_goal, PlanningBudget::default())?;
            let mut step = PursuitStep {
                local_goal,
                planning: planning.reason,
                question: planning.question,
                exploratory: false,
                execution: None,
            };
            match planning.reason {
                PlanReason::ResourceLimited => {
                    out.steps.push(step);
                    return Ok(out);
                }
                PlanReason::ContradictoryEvidence => {
                    out.reason = PursuitReason::ContradictoryEvidence;
                    out.steps.push(step);
                    return Ok(out);
                }
                PlanReason::AlreadyAtGoal => {
                    out.reason = PursuitReason::Reached;
                    out.steps.push(step);
                    return Ok(out);
                }
                PlanReason::Supported => {
                    let plan = planning.plan.ok_or("plano ausente")?;
                    if plan.steps.len() > goal.max_actions - used {
                        out.steps.push(step);
                        return Ok(out);
                    }
                    let scans = requests.len() * plan.steps.len();
                    if self.attention_scans + scans > self.limits.attention_scans {
                        out.steps.push(step);
                        return Ok(out);
                    }
                    self.attention_scans += scans;
                    let mut chosen = vec![];
                    let mut permits = vec![];
                    for transition in &plan.steps {
                        let request = requests
                            .iter()
                            .filter(|r| {
                                r.permit.action == transition.action
                                    && self.permit_matches(world, r)
                                    && r.at >= self.at
                                    && !permits.contains(&r.permit.id)
                            })
                            .min_by_key(|r| r.id);
                        let Some(request) = request else {
                            out.reason = PursuitReason::NoAuthorizedAction;
                            out.steps.push(step);
                            return Ok(out);
                        };
                        permits.push(request.permit.id);
                        chosen.push(request.clone());
                    }
                    let execution = self.execute_plan(world, &plan, &chosen);
                    let reason = execution.reason;
                    step.execution = Some(execution);
                    out.steps.push(step);
                    match reason {
                        ExecutionReason::Reached => {
                            out.observed = world.state;
                            out.reason = PursuitReason::Reached;
                            return Ok(out);
                        }
                        ExecutionReason::Diverged | ExecutionReason::Observed => continue,
                        ExecutionReason::ResourceLimited => return Ok(out),
                        ExecutionReason::DeniedOrStale => {
                            out.reason = PursuitReason::NoAuthorizedAction;
                            return Ok(out);
                        }
                        ExecutionReason::Interrupted => {
                            out.reason = PursuitReason::Interrupted;
                            out.observed = world.state;
                            return Ok(out);
                        }
                    }
                }
                PlanReason::InsufficientEvidence => {
                    step.exploratory = true;
                    // Compartilha o orçamento atencional existente; o tamanho
                    // da interface não revela os destinos das transições.
                    let scans = world.transitions.len() + requests.len();
                    if self.attention_scans + scans > self.limits.attention_scans {
                        out.steps.push(step);
                        return Ok(out);
                    }
                    self.attention_scans += scans;
                    let enabled = world.available_actions();
                    let request = requests
                        .iter()
                        .filter(|r| {
                            enabled.contains(&r.permit.action)
                                && self.permit_matches(world, r)
                                && r.at >= self.at
                        })
                        .min_by_key(|r| r.id);
                    let Some(request) = request else {
                        out.reason = PursuitReason::NoAuthorizedAction;
                        out.steps.push(step);
                        return Ok(out);
                    };
                    if self.intervention_room(1).is_err() {
                        out.steps.push(step);
                        return Ok(out);
                    }
                    match self.intervene(world, request) {
                        Ok(trial) => {
                            step.execution = Some(Execution {
                                goal: local_goal,
                                reason: if world.state == local_goal.to {
                                    ExecutionReason::Reached
                                } else {
                                    ExecutionReason::Observed
                                },
                                trials: vec![trial],
                                observed: world.state,
                            })
                        }
                        Err(_) => {
                            out.reason = PursuitReason::Interrupted;
                            out.steps.push(step);
                            return Ok(out);
                        }
                    }
                    out.steps.push(step);
                }
            }
        }
    }
}
