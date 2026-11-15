//! Integração do planejador existente com uma previsão H23 de primeiro passo.
//! Memória de outro episódio é explícita e imutável; recursos não são renovados.
use super::*;
use crate::cognition::concepts::Label;
use crate::curriculum::PedagogyHost;
use crate::integration::concepts::ConceptReport;
use crate::metacognition::concepts::{diagnose_pedagogy, ConceptDecision, PedagogicalDiagnosis};

/// Concessões pedagógicas independentes; nenhuma chave de avaliação ou rótulo.
pub struct PredictiveRevision {
    pub record: EngramId,
    pub receipt: u32,
    pub assessment: PedagogyAssessment,
    pub permissions: PedagogyPermissions,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PredictiveChoice {
    ObservedRoute,
    ConceptualPrefix,
    None,
    SupervisedExperience,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PredictiveReason {
    Reached,
    InsufficientEvidence,
    ConflictingEvidence,
    Denied,
    ResourceLimited,
    Diverged,
    Interrupted,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredictivePursuit {
    pub goal: PursuitGoal,
    pub forecast: Option<ConsequenceForecast>,
    pub observed_alternative: Option<PlanOutcome>,
    pub prefix_alternative: Option<PlanOutcome>,
    pub choice: PredictiveChoice,
    pub justification: String,
    pub review: Option<ConsequenceReview>,
    pub replanning: Option<PlanOutcome>,
    pub execution: Option<Execution>,
    pub previous_execution: Option<Execution>,
    pub failure: Option<String>,
    pub revision: Option<ConsequenceTraining>,
    pub revised_query: Option<ConceptReport>,
    pub exploration: Option<ActionTrial>,
    pub reason: PredictiveReason,
    pub observed: i32,
}
impl Runtime {
    /// Continuação opcional após insuficiência real. Uma experiência, autorização
    /// explícita e revisão pelo currículo existente, sem presumir êxito do ensino.
    pub fn pursue_learning_goal(
        &mut self,
        memory: &impl EpisodeMemory,
        session: &mut Session,
        world: &mut World,
        goal: PursuitGoal,
        contract: ConsequenceContract,
        attributes: Reading,
        requests: &[Intervention],
        revision: Option<PredictiveRevision>,
    ) -> Result<PredictivePursuit, String> {
        let mut out = self.pursue_with_consequences(
            memory, session, world, goal, contract, attributes, requests,
        )?;
        let Some(revision) = revision else {
            return Ok(out);
        };
        if out.reason != PredictiveReason::InsufficientEvidence {
            return Ok(out);
        }
        let forecast = out.forecast.as_ref().ok_or("previsão ausente")?;
        if diagnose_pedagogy(std::slice::from_ref(forecast.report()))
            != PedagogicalDiagnosis::ExperienceNeeded
        {
            return Ok(out);
        }
        if !self.access.learn_concepts
            || self.conceptual_attempted
            || revision.receipt == 0
            || revision.permissions.evaluation != Some(contract.scope)
            || revision.permissions.presentation != Some(contract.scope)
            || revision.permissions.admissions.len() > 3
            || revision
                .permissions
                .admissions
                .iter()
                .enumerate()
                .any(|(i, id)| revision.permissions.admissions[..i].contains(id))
            || !revision.permissions.admissions.contains(&revision.record)
        {
            out.reason = PredictiveReason::Denied;
            return Ok(out);
        }
        session.pedagogy_capacity(1)?;
        if self.intervention_room(goal.max_actions).is_err() {
            out.reason = PredictiveReason::ResourceLimited;
            return Ok(out);
        }
        let chosen = match self.choose_requests(world, &[contract.action], requests) {
            Ok(r) => r,
            Err(reason) => {
                out.reason = reason;
                return Ok(out);
            }
        };
        let frame = forecast.frame().clone();
        let trial = match self.intervene(world, &chosen[0]) {
            Ok(t) => t,
            Err(e) => {
                out.failure = Some(e);
                out.reason = PredictiveReason::Interrupted;
                return Ok(out);
            }
        };
        out.choice = PredictiveChoice::SupervisedExperience;
        out.exploration = Some(trial.clone());
        out.observed = world.state();
        let teaching = self.teach_consequences(
            session,
            contract,
            &[(frame, trial.id, revision.record, revision.receipt)],
            revision.assessment,
            revision.permissions,
        );
        let teaching = match teaching {
            Ok(r) => r,
            Err(e) => {
                out.failure = Some(e);
                out.reason = PredictiveReason::Interrupted;
                return Ok(out);
            }
        };
        let admitted = teaching.pedagogy.lessons.iter().any(|l| l.admitted);
        out.revision = Some(teaching);
        // Segunda consulta da reserva original: original agora é histórico,
        // jamais reutilizado como atual depois da revisão H23.
        let current = session.pedagogy_query(goal.objective, forecast.report().observation)?;
        self.cognitive_resources.conceptual_queries += 1;
        out.revised_query = Some(current);
        if !admitted {
            out.reason = PredictiveReason::InsufficientEvidence;
            return Ok(out);
        }
        out.justification.push_str(" Insuficiência motivou experiência explicitamente autorizada; MP09 decidiu sua incorporação; objetivo original preservado.");
        self.finish_predictive(memory, world, out, requests, goal.max_actions - 1, false)
    }
    pub(super) fn remembered_trials(
        &self,
        memory: &impl EpisodeMemory,
    ) -> Result<Vec<ActionTrial>, String> {
        let mut trials = memory.memory_trials()?;
        for t in &self.action_trials {
            if trials
                .iter()
                .any(|old| old.id == t.id || old.permit == t.permit)
            {
                return Err("identidade de experiência reutilizada entre episódios".into());
            }
            trials.push(t.clone());
        }
        Ok(trials)
    }
    pub(super) fn choose_requests(
        &mut self,
        world: &World,
        actions: &[u32],
        requests: &[Intervention],
    ) -> Result<Vec<Intervention>, PredictiveReason> {
        let scans = actions.len() * requests.len();
        if self.attention_scans + scans > self.limits.attention_scans {
            return Err(PredictiveReason::ResourceLimited);
        }
        self.attention_scans += scans;
        let mut selected: Vec<Intervention> = vec![];
        for action in actions {
            let Some(r) = requests
                .iter()
                .filter(|r| {
                    r.permit.action == *action
                        && self.permit_matches(world, r)
                        && !selected.iter().any(|old| old.permit.id == r.permit.id)
                })
                .min_by_key(|r| r.id)
            else {
                return Err(PredictiveReason::Denied);
            };
            selected.push(r.clone());
        }
        if !self.valid_requests(world, &selected) {
            return Err(PredictiveReason::Denied);
        }
        if self.intervention_room(selected.len()).is_err() {
            return Err(PredictiveReason::ResourceLimited);
        }
        Ok(selected)
    }
    /// Compara duas alternativas por evidência e comprimento; empates favorecem
    /// a rota inteiramente observada. Não recebe atributos de estados futuros,
    /// destinos ocultos ou chave de avaliação. Um prefixo conceitual, até quatro
    /// ações totais e uma reconsulta/revisão do plano, no orçamento já existente.
    pub fn pursue_with_consequences(
        &mut self,
        memory: &impl EpisodeMemory,
        session: &Session,
        world: &mut World,
        goal: PursuitGoal,
        contract: ConsequenceContract,
        attributes: Reading,
        requests: &[Intervention],
    ) -> Result<PredictivePursuit, String> {
        if !self.world_observable(world) {
            return Err("sensor de estado indisponível ou não autorizado".into());
        }
        let current = world.attribute_query(contract.action);
        if !self.access.read_knowledge
            || !self.access.explore
            || !self.access.record
            || goal.max_actions == 0
            || goal.max_actions > 4
            || requests.len() > 4
            || current.world != goal.task.world
            || current.revision != goal.task.revision
            || current.context != goal.task.context
            || current.source != goal.task.source
            || current.from != goal.task.from
            || self.verification_attempted
        {
            return Err("objetivo, consulta ou autoridade de planejamento incompatível".into());
        }
        memory.validate_memory(session, goal.task)?;
        if requests.iter().any(|r| memory.memory_used(r)) {
            return Err("concessão/identidade já consumida no episódio de origem".into());
        }
        let mut out = PredictivePursuit {
            goal,
            forecast: None,
            observed_alternative: None,
            prefix_alternative: None,
            choice: PredictiveChoice::None,
            justification: String::new(),
            review: None,
            replanning: None,
            execution: None,
            previous_execution: None,
            failure: None,
            revision: None,
            revised_query: None,
            exploration: None,
            reason: PredictiveReason::InsufficientEvidence,
            observed: world.state(),
        };
        if goal.task.from == goal.task.to {
            out.reason = PredictiveReason::Reached;
            return Ok(out);
        }
        if self.consequence_contract.is_none() && memory.memory_contract().is_some() {
            self.inherit_consequence_contract(memory)?;
        }
        let trials = self.remembered_trials(memory)?;
        let frame = self.observe_attributes(world, contract.action, attributes)?;
        let forecast = self.forecast_using(session, contract, &frame, goal.objective, &trials)?;
        let direct = self.plan_using(
            goal.task,
            PlanningBudget {
                steps: goal.max_actions,
                scans: 32,
            },
            &trials,
        )?;
        let tail_goal = PlanningGoal {
            from: contract.target,
            ..goal.task
        };
        let tail = if let Err(e) = memory.validate_plan_goal(tail_goal) {
            out.failure = Some(e);
            PlanOutcome {
                reason: PlanReason::InsufficientEvidence,
                plan: None,
                question: Some(TransitionQuestion {
                    goal: tail_goal,
                    observed_trials: trials.len(),
                }),
                scans: 0,
            }
        } else {
            self.plan_using(
                tail_goal,
                PlanningBudget {
                    steps: goal.max_actions - 1,
                    scans: 32,
                },
                &trials,
            )?
        };
        out.forecast = Some(forecast.clone());
        out.observed_alternative = Some(direct.clone());
        out.prefix_alternative = Some(tail.clone());
        let supported = forecast.report().decision == ConceptDecision::Supported(Label::Positive);
        let safe_prefix = supported
            && forecast.transitions().status != PredictionStatus::Conflicting
            && forecast
                .transitions()
                .outcomes
                .iter()
                .all(|o| o.to == contract.target);
        let direct_plan = direct.plan.as_ref().filter(|p| {
            p.steps().first().is_none_or(|s| {
                s.action != contract.action
                    || (forecast.transitions().status != PredictionStatus::Conflicting
                        && match forecast.report().decision {
                            ConceptDecision::Supported(label) => {
                                label == Label::from_bool(s.to == contract.target)
                            }
                            ConceptDecision::Abstain(_) => {
                                diagnose_pedagogy(std::slice::from_ref(forecast.report()))
                                    != PedagogicalDiagnosis::Blocked
                            }
                        })
            })
        });
        let tail_plan = tail.plan.as_ref().filter(|_| safe_prefix);
        let permitted_direct = direct_plan.map(|p| {
            self.choose_requests(
                world,
                &p.steps().iter().map(|s| s.action).collect::<Vec<_>>(),
                requests,
            )
        });
        let permitted_prefix = tail_plan.map(|p| {
            self.choose_requests(
                world,
                &std::iter::once(contract.action)
                    .chain(p.steps().iter().map(|s| s.action))
                    .collect::<Vec<_>>(),
                requests,
            )
        });
        let direct_ok = permitted_direct.as_ref().is_some_and(Result::is_ok);
        let prefix_ok = permitted_prefix.as_ref().is_some_and(Result::is_ok);
        let choose_prefix = prefix_ok
            && (!direct_ok
                || tail_plan.is_some_and(|p| {
                    direct_plan.is_none_or(|d| 1 + p.steps().len() < d.steps().len())
                }));
        let chosen;
        if choose_prefix {
            out.choice = PredictiveChoice::ConceptualPrefix;
            chosen = permitted_prefix.unwrap().unwrap();
            out.justification = "H23 sustentado e sem oposição; prefixo e caminho observado formam a menor alternativa autorizada com reserva integral".into();
        } else if direct_ok {
            out.choice = PredictiveChoice::ObservedRoute;
            chosen = permitted_direct.unwrap().unwrap();
            out.justification = "Caminho inteiramente observado elegível, menor ou empatado; referências preservadas no plano".into();
        } else {
            out.reason = if [permitted_direct.as_ref(), permitted_prefix.as_ref()]
                .into_iter()
                .flatten()
                .any(|r| *r == Err(PredictiveReason::ResourceLimited))
                || direct.reason == PlanReason::ResourceLimited
                || tail.reason == PlanReason::ResourceLimited
            {
                PredictiveReason::ResourceLimited
            } else if permitted_direct.is_some() || permitted_prefix.is_some() {
                PredictiveReason::Denied
            } else if forecast.transitions().status == PredictionStatus::Conflicting
                || (supported
                    && forecast
                        .transitions()
                        .outcomes
                        .iter()
                        .any(|o| o.to != contract.target))
                || direct.reason == PlanReason::ContradictoryEvidence
                || tail.reason == PlanReason::ContradictoryEvidence
            {
                PredictiveReason::ConflictingEvidence
            } else {
                PredictiveReason::InsufficientEvidence
            };
            out.justification =
                "Nenhuma alternativa sustentada; não executar uma hipótese como fato".into();
            self.consequence_pending.take();
            self.cognitive_resources.operations_without_report -= 1;
            return Ok(out);
        }
        if out.choice == PredictiveChoice::ObservedRoute {
            self.consequence_pending.take();
            self.cognitive_resources.operations_without_report -= 1;
            let execution = self.execute_plan_using(world, direct_plan.unwrap(), &chosen, &trials);
            out.observed = world.state();
            out.reason = reason_of(execution.reason);
            if execution.reason == ExecutionReason::Diverged {
                let remaining = goal.max_actions.saturating_sub(execution.trials.len());
                out.previous_execution = Some(execution);
                return self.finish_predictive(memory, world, out, requests, remaining, true);
            }
            out.execution = Some(execution);
            return Ok(out);
        }
        let review = self.execute_forecast(session, world, forecast, &chosen[0]);
        out.observed = world.state();
        let review = match review {
            Ok(r) => r,
            Err(e) => {
                out.failure = Some(e);
                out.reason = PredictiveReason::Interrupted;
                return Ok(out);
            }
        };
        let diverged = review.review_required;
        out.review = Some(review);
        self.finish_predictive(memory, world, out, requests, goal.max_actions - 1, diverged)
    }
    fn finish_predictive(
        &mut self,
        memory: &impl EpisodeMemory,
        world: &mut World,
        mut out: PredictivePursuit,
        requests: &[Intervention],
        remaining_actions: usize,
        diverged: bool,
    ) -> Result<PredictivePursuit, String> {
        let goal = out.goal;
        if world.state() == goal.task.to {
            out.reason = PredictiveReason::Reached;
            return Ok(out);
        }
        // Reconsulta usa o estado REAL, nunca muda o objetivo original e não
        // elimina a previsão falha ou sua experiência contrária.
        let actual = PlanningGoal {
            from: world.state(),
            ..goal.task
        };
        if let Err(e) = memory.validate_plan_goal(actual) {
            out.reason = PredictiveReason::InsufficientEvidence;
            out.failure = Some(e);
            return Ok(out);
        }
        let updated = self.remembered_trials(memory)?;
        let replanning = self.plan_using(
            actual,
            PlanningBudget {
                steps: remaining_actions,
                scans: 32,
            },
            &updated,
        )?;
        out.replanning = Some(replanning.clone());
        let Some(plan) = replanning.plan else {
            out.reason = if diverged {
                PredictiveReason::Diverged
            } else if replanning.reason == PlanReason::ResourceLimited {
                PredictiveReason::ResourceLimited
            } else {
                PredictiveReason::InsufficientEvidence
            };
            return Ok(out);
        };
        let remaining: Vec<u32> = plan.steps().iter().map(|s| s.action).collect();
        let chosen = match self.choose_requests(world, &remaining, requests) {
            Ok(chosen) => chosen,
            Err(reason) => {
                out.reason = reason;
                return Ok(out);
            }
        };
        let execution = self.execute_plan_using(world, &plan, &chosen, &updated);
        out.observed = world.state();
        out.reason = reason_of(execution.reason);
        out.execution = Some(execution);
        Ok(out)
    }
}
fn reason_of(reason: ExecutionReason) -> PredictiveReason {
    match reason {
        ExecutionReason::Reached => PredictiveReason::Reached,
        ExecutionReason::Diverged => PredictiveReason::Diverged,
        ExecutionReason::ResourceLimited => PredictiveReason::ResourceLimited,
        ExecutionReason::DeniedOrStale => PredictiveReason::Denied,
        _ => PredictiveReason::Interrupted,
    }
}
