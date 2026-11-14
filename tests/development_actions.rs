//! Intervenções funcionais públicas próprias, sem acesso a painéis reservados.
use neurons::cognitive_contracts::SourceId;
use neurons::engram::{EvidenceId, RecordStatus};
use neurons::event::NodeId;
use neurons::integration::development::*;
use neurons::integration::synaptic::SynapticEpisode;
use neurons::session::f1::Session;
fn n(id: u32) -> NodeId {
    NodeId::new(id).unwrap()
}
fn sensor() -> Sensor {
    Sensor {
        id: 1,
        source: SourceId::new(7).unwrap(),
        context: 61,
        input: n(101),
        encoding: Encoding::Scalar,
    }
}
fn access() -> Access {
    Access {
        inspect: true,
        record: true,
        read_knowledge: true,
        explore: true,
        sensors: vec![1],
        ..Default::default()
    }
}
fn runtime() -> Runtime {
    runtime_with_limits(Limits::default())
}
fn runtime_with_limits(limits: Limits) -> Runtime {
    let mut neural = SynapticEpisode::new();
    for c in [
        "novo 41 61 81 1 0 3 2 16 64",
        "neuronio 101 1 4",
        "neuronio 103 2 4",
        "sinapse 11 101 103 0 1",
    ] {
        neural.execute(c).unwrap();
    }
    Runtime::new(neural, &[sensor()], access(), limits).unwrap()
}
fn world(start: i32, rules: &[(i32, u32, i32)]) -> World {
    World::new(501, 1, sensor(), start, rules).unwrap()
}
fn request(id: u32, action: u32, at: u64) -> Intervention {
    Intervention {
        id,
        permit: ActionPermit {
            id: 100 + id,
            world: 501,
            revision: 1,
            context: 61,
            source: SourceId::new(7).unwrap(),
            action,
        },
        before_id: 2 * id - 1,
        after_id: 2 * id,
        before_evidence: EvidenceId::new(2000 + id * 2).unwrap(),
        after_evidence: EvidenceId::new(2001 + id * 2).unwrap(),
        at,
    }
}
fn goal(from: i32, to: i32) -> PlanningGoal {
    PlanningGoal {
        world: 501,
        revision: 1,
        context: 61,
        source: SourceId::new(7).unwrap(),
        from,
        to,
    }
}
fn acquired() -> Runtime {
    let mut runtime = runtime();
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    runtime.intervene(&mut env, &request(1, 7, 0)).unwrap();
    runtime.intervene(&mut env, &request(2, 9, 3)).unwrap();
    runtime
}
#[test]
fn intervention_records_real_state_change_and_neural_perception() {
    let mut runtime = runtime();
    let mut env = world(10, &[(10, 7, 20)]);
    assert!(runtime.action_trials().is_empty());
    let r = runtime.intervene(&mut env, &request(1, 7, 0)).unwrap();
    assert_eq!((r.from, r.to, r.action), (10, 20, 7));
    assert_eq!(env.state(), 20);
    assert_eq!(env.action_count(), 1);
    assert_eq!(runtime.memory().get(r.before).unwrap().value(), 10);
    assert_eq!(runtime.memory().get(r.after).unwrap().value(), 20);
    assert_eq!(runtime.experiences().len(), 2);
    assert!(runtime.experiences().iter().all(|e| e.neural_events > 0));
    assert!(runtime
        .memory()
        .iter()
        .all(|r| r.status() == RecordStatus::Observed));
    assert_eq!(runtime.action_attempts(), 1);
    assert_eq!(runtime.action_reserved_events(), 0);
}
#[test]
fn insufficient_model_asks_for_observations_without_receiving_world_rules() {
    let mut runtime = runtime();
    let outcome = runtime
        .plan(goal(10, 30), PlanningBudget::default())
        .unwrap();
    assert_eq!(outcome.reason, PlanReason::InsufficientEvidence);
    assert!(outcome.plan.is_none());
    let question = outcome.question.unwrap();
    assert_eq!(question.goal, goal(10, 30));
    assert_eq!(question.observed_trials, 0);
}
#[test]
fn acquired_transitions_support_a_new_composed_goal_and_authorized_execution() {
    let mut learned = acquired();
    let mut control = runtime();
    assert!(control
        .plan(goal(10, 30), PlanningBudget::default())
        .unwrap()
        .plan
        .is_none());
    let result = learned
        .plan(goal(10, 30), PlanningBudget::default())
        .unwrap();
    assert_eq!(result.reason, PlanReason::Supported);
    let plan = result.plan.unwrap();
    assert_eq!(
        plan.steps().iter().map(|t| t.action).collect::<Vec<_>>(),
        vec![7, 9]
    );
    let mut task = world(10, &[(10, 7, 20), (20, 9, 30)]);
    let execution = learned.execute_plan(&mut task, &plan, &[request(3, 7, 6), request(4, 9, 9)]);
    assert_eq!(execution.reason, ExecutionReason::Reached);
    assert_eq!(execution.trials.len(), 2);
    assert_eq!(execution.observed, 30);
    assert_eq!(task.action_count(), 2);
    assert_eq!(learned.experiences().len(), 8);
}
#[test]
fn planning_is_not_a_table_of_answers_by_domain() {
    for (start, middle, end, a, b) in [(2, 8, 21, 11, 12), (-8, 0, 3, 4, 6), (41, 37, 99, 2, 1)] {
        let mut runtime = runtime();
        let rules = [(start, a, middle), (middle, b, end)];
        let mut env = world(start, &rules);
        runtime.intervene(&mut env, &request(1, a, 0)).unwrap();
        runtime.intervene(&mut env, &request(2, b, 3)).unwrap();
        let result = runtime
            .plan(goal(start, end), PlanningBudget::default())
            .unwrap();
        let plan = result.plan.unwrap();
        assert_eq!(
            plan.steps().iter().map(|s| s.action).collect::<Vec<_>>(),
            vec![a, b]
        );
        let mut fresh = world(start, &rules);
        let r = runtime.execute_plan(&mut fresh, &plan, &[request(3, a, 6), request(4, b, 9)]);
        assert_eq!(r.reason, ExecutionReason::Reached);
        assert_eq!(fresh.state(), end);
    }
}
#[test]
fn denied_or_incompatible_permit_cannot_intervene() {
    for field in 0..5 {
        let mut runtime = runtime();
        let mut env = world(10, &[(10, 7, 20)]);
        let mut r = request(1, 7, 0);
        match field {
            0 => r.permit.world = 999,
            1 => r.permit.revision = 2,
            2 => r.permit.context = 62,
            3 => r.permit.source = SourceId::new(8).unwrap(),
            _ => r.permit.id = 0,
        };
        let before = runtime.neural().state();
        assert!(runtime.intervene(&mut env, &r).is_err());
        assert_eq!(env.state(), 10);
        assert_eq!(env.action_count(), 0);
        assert!(runtime.memory().is_empty());
        assert_eq!(runtime.neural().state(), before);
    }
}
#[test]
fn failed_intervention_retains_reservation_without_mutating_world_or_memory() {
    let mut runtime = runtime();
    let mut env = world(10, &[]);
    let before = runtime.neural().state();
    assert!(runtime.intervene(&mut env, &request(1, 7, 0)).is_err());
    assert_eq!(runtime.action_reserved_events(), 32);
    assert_eq!(runtime.action_attempts(), 1);
    assert_eq!(runtime.neural().state(), before);
    assert!(runtime.memory().is_empty());
    assert_eq!(env.action_count(), 0);
    assert!(runtime.intervene(&mut env, &request(1, 7, 0)).is_err());
    assert_eq!(runtime.action_attempts(), 1);
}
#[test]
fn all_attempts_are_bounded_even_if_environment_never_responds() {
    let mut runtime = runtime();
    let mut env = world(10, &[]);
    for id in 1..=4 {
        assert!(runtime.intervene(&mut env, &request(id, 7, 0)).is_err());
    }
    assert_eq!(runtime.action_reserved_events(), 128);
    assert_eq!(runtime.action_attempts(), 4);
    assert!(runtime.intervene(&mut env, &request(5, 7, 0)).is_err());
    assert_eq!(runtime.action_attempts(), 4);
}
#[test]
fn insufficient_plan_budget_is_not_missing_knowledge() {
    let mut runtime = acquired();
    let r = runtime
        .plan(
            goal(10, 30),
            PlanningBudget {
                steps: 1,
                scans: 32,
            },
        )
        .unwrap();
    assert_eq!(r.reason, PlanReason::ResourceLimited);
    assert!(r.question.is_none());
    let r = runtime
        .plan(goal(10, 30), PlanningBudget { steps: 4, scans: 1 })
        .unwrap();
    assert_eq!(r.reason, PlanReason::ResourceLimited);
}
#[test]
fn missing_or_duplicate_execution_authority_is_rejected_before_any_action() {
    for field in 0..3 {
        let mut runtime = acquired();
        let plan = runtime
            .plan(goal(10, 30), PlanningBudget::default())
            .unwrap()
            .plan
            .unwrap();
        let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
        let mut r = vec![request(3, 7, 6), request(4, 9, 9)];
        match field {
            0 => {
                r.pop();
            }
            1 => r[1].permit.id = r[0].permit.id,
            _ => r[1].id = r[0].id,
        };
        let count = runtime.experiences().len();
        let result = runtime.execute_plan(&mut env, &plan, &r);
        assert_eq!(result.reason, ExecutionReason::DeniedOrStale);
        assert_eq!(env.action_count(), 0);
        assert_eq!(runtime.experiences().len(), count);
    }
}
#[test]
fn changed_environment_interrupts_plan_and_conflicting_transition_blocks_reuse() {
    let mut runtime = acquired();
    let plan = runtime
        .plan(goal(10, 30), PlanningBudget::default())
        .unwrap()
        .plan
        .unwrap();
    // Mudança não anunciada no ambiente: a previsão não é tomada como verdade.
    let mut env = world(10, &[(10, 7, 99), (99, 9, 30)]);
    let result = runtime.execute_plan(&mut env, &plan, &[request(3, 7, 6), request(4, 9, 9)]);
    assert_eq!(result.reason, ExecutionReason::Diverged);
    assert_eq!(result.trials.len(), 1);
    assert_eq!(env.action_count(), 1);
    assert_eq!(env.state(), 99);
    let fresh = runtime
        .plan(goal(10, 30), PlanningBudget::default())
        .unwrap();
    assert_eq!(fresh.reason, PlanReason::ContradictoryEvidence);
    assert!(fresh.plan.is_none());
}
#[test]
fn stale_model_or_world_revision_cannot_execute_an_old_plan() {
    let mut runtime = acquired();
    let plan = runtime
        .plan(goal(10, 30), PlanningBudget::default())
        .unwrap()
        .plan
        .unwrap();
    let mut env = World::new(501, 2, sensor(), 10, &[(10, 7, 20), (20, 9, 30)]).unwrap();
    assert_eq!(
        runtime
            .execute_plan(&mut env, &plan, &[request(3, 7, 6), request(4, 9, 9)])
            .reason,
        ExecutionReason::DeniedOrStale
    );
    assert_eq!(env.action_count(), 0);
    let mut other = world(30, &[(30, 1, 10)]);
    runtime.intervene(&mut other, &request(3, 1, 6)).unwrap();
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    assert_eq!(
        runtime
            .execute_plan(&mut env, &plan, &[request(4, 7, 9), request(5, 9, 12)])
            .reason,
        ExecutionReason::DeniedOrStale
    );
}
#[test]
fn resource_check_precedes_multi_step_execution() {
    let mut runtime = acquired();
    let mut env = world(30, &[(30, 1, 10)]);
    runtime.intervene(&mut env, &request(3, 1, 6)).unwrap();
    let plan = runtime
        .plan(goal(10, 30), PlanningBudget::default())
        .unwrap()
        .plan
        .unwrap();
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    let result = runtime.execute_plan(&mut env, &plan, &[request(4, 7, 9), request(5, 9, 12)]);
    assert_eq!(result.reason, ExecutionReason::ResourceLimited);
    assert_eq!(env.action_count(), 0);
}

#[test]
fn action_checkpoint_preserves_model_and_requires_fresh_permissions() {
    let runtime = acquired();
    let bytes = runtime.checkpoint(&Session::new()).unwrap();
    assert_eq!(&bytes[..8], b"NRNPIFI2");
    let (mut denied, knowledge) = Runtime::restore(&bytes, &[sensor()], Access::default()).unwrap();
    assert_eq!(denied.action_trials(), runtime.action_trials());
    assert_eq!(denied.action_attempts(), 2);
    assert_eq!(denied.checkpoint(&knowledge).unwrap(), bytes);
    assert!(denied
        .plan(goal(10, 30), PlanningBudget::default())
        .is_err());
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    assert!(denied.intervene(&mut env, &request(3, 7, 6)).is_err());
    assert_eq!(env.action_count(), 0);
    let (mut permitted, _) = Runtime::restore(&bytes, &[sensor()], access()).unwrap();
    let plan = permitted
        .plan(goal(10, 30), PlanningBudget::default())
        .unwrap()
        .plan
        .unwrap();
    assert!(
        permitted.intervene(&mut env, &request(1, 7, 6)).is_err(),
        "concessão antiga não pode ser reutilizada"
    );
    let r = permitted.execute_plan(&mut env, &plan, &[request(3, 7, 6), request(4, 9, 9)]);
    assert_eq!(r.reason, ExecutionReason::Reached);
}
#[test]
fn legacy_checkpoint_is_still_canonical_and_not_silently_upgraded() {
    let runtime = runtime();
    let bytes = runtime.checkpoint(&Session::new()).unwrap();
    assert_eq!(&bytes[..8], b"NRNPIFI1");
    let (restored, s) = Runtime::restore(&bytes, &[sensor()], Access::default()).unwrap();
    assert_eq!(restored.checkpoint(&s).unwrap(), bytes);
}
#[test]
fn failed_action_and_planning_consumption_cannot_be_refunded_by_restart() {
    let mut runtime = runtime();
    let mut env = world(10, &[]);
    assert!(runtime.intervene(&mut env, &request(1, 7, 0)).is_err());
    for _ in 0..4 {
        runtime
            .plan(goal(10, 30), PlanningBudget::default())
            .unwrap();
    }
    let bytes = runtime.checkpoint(&Session::new()).unwrap();
    let (mut restored, s) = Runtime::restore(&bytes, &[sensor()], access()).unwrap();
    assert_eq!(restored.action_reserved_events(), 32);
    assert_eq!(restored.action_attempts(), 1);
    assert_eq!(restored.checkpoint(&s).unwrap(), bytes);
    assert_eq!(
        restored
            .plan(goal(10, 30), PlanningBudget::default())
            .unwrap()
            .reason,
        PlanReason::ResourceLimited
    );
    assert!(restored.intervene(&mut env, &request(1, 7, 0)).is_err());
    assert_eq!(restored.action_attempts(), 1);
}
#[test]
fn checkpoint_preserves_opposition_instead_of_choosing_a_convenient_trial() {
    let mut runtime = acquired();
    let mut changed = world(10, &[(10, 7, 99)]);
    runtime.intervene(&mut changed, &request(3, 7, 6)).unwrap();
    let bytes = runtime.checkpoint(&Session::new()).unwrap();
    let (mut restored, _) = Runtime::restore(&bytes, &[sensor()], access()).unwrap();
    assert_eq!(
        restored
            .plan(goal(10, 30), PlanningBudget::default())
            .unwrap()
            .reason,
        PlanReason::ContradictoryEvidence
    );
}
fn resign(bytes: &mut [u8]) {
    let end = bytes.len() - 8;
    let hash = bytes[..end].iter().fold(0xcbf29ce484222325u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    });
    bytes[end..].copy_from_slice(&hash.to_le_bytes());
}
#[test]
fn valid_checksum_does_not_make_a_forged_action_consequence_valid() {
    let runtime = acquired();
    let mut bytes = runtime.checkpoint(&Session::new()).unwrap();
    let last_trial = bytes.len() - 8 - 11 * 8;
    let to = last_trial + 8 * 8;
    bytes[to..to + 8].copy_from_slice(&999u64.to_le_bytes());
    resign(&mut bytes);
    assert!(Runtime::restore(&bytes, &[sensor()], access()).is_err());
}
#[test]
fn v2_truncations_and_corruption_are_rejected() {
    let runtime = acquired();
    let bytes = runtime.checkpoint(&Session::new()).unwrap();
    for cut in 0..bytes.len() {
        assert!(Runtime::restore(&bytes[..cut], &[sensor()], access()).is_err());
    }
    for index in (0..bytes.len()).step_by((bytes.len() / 64).max(1)) {
        let mut corrupt = bytes.clone();
        corrupt[index] ^= 1;
        assert!(Runtime::restore(&corrupt, &[sensor()], access()).is_err());
    }
}
#[test]
fn action_restore_process_worker() {
    let Some(path) = std::env::var_os("NEURONS_ACTION_RESTORE") else {
        return;
    };
    let bytes = std::fs::read(path).unwrap();
    let (mut runtime, _) = Runtime::restore(&bytes, &[sensor()], access()).unwrap();
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    let executed = runtime
        .pursue(
            &mut env,
            pursuit_goal(2),
            &[request(3, 7, 6), request(4, 9, 9)],
        )
        .unwrap();
    assert_eq!(executed.reason, PursuitReason::Reached);
    assert!(executed.steps.iter().all(|s| !s.exploratory));
    assert_eq!(env.state(), 30);
    assert_eq!(runtime.experiences().len(), 8);
    println!("ações aprendidas restauradas e objetivo composto executado");
}
#[test]
fn learned_action_model_is_applied_in_a_new_process() {
    use std::io::Write;
    let runtime = acquired();
    let dir = std::env::temp_dir().join(format!(
        "neurons-actions-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("runtime.pifi");
    std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
        .unwrap()
        .write_all(&runtime.checkpoint(&Session::new()).unwrap())
        .unwrap();
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "action_restore_process_worker", "--nocapture"])
        .env("NEURONS_ACTION_RESTORE", path)
        .output()
        .unwrap();
    assert!(
        child.status.success(),
        "{}{}",
        String::from_utf8_lossy(&child.stdout),
        String::from_utf8_lossy(&child.stderr)
    );
    assert!(String::from_utf8_lossy(&child.stdout).contains("objetivo composto executado"));
}

#[test]
fn mp05_feedback_is_linked_to_an_action_observation_and_retained_separately() {
    use neurons::learning::Feedback;
    use neurons::synapse::SynapseId;
    let mut neural = SynapticEpisode::new();
    for c in [
        "novo 41 61 81 1 0 3 2 16 64",
        "neuronio 101 1 4",
        "neuronio 103 2 4",
        "sinapse 11 101 103 0 1",
        "autorizar 7 0 1000",
    ] {
        neural.execute(c).unwrap();
    }
    let mut permissions = access();
    permissions.adapt = true;
    let mut runtime = Runtime::new(neural, &[sensor()], permissions, Limits::default()).unwrap();
    let mut env = world(10, &[(10, 7, 20)]);
    let trial = runtime.intervene(&mut env, &request(1, 7, 0)).unwrap();
    let feedback = FeedbackRequest {
        id: 701,
        experience: 2,
        source: SourceId::new(7).unwrap(),
        evidence: EvidenceId::new(9701).unwrap(),
        synapse: SynapseId::new(11).unwrap(),
        signal: Feedback::Positive,
        at: 2,
    };
    let update = runtime.feedback(feedback).unwrap();
    assert_eq!((update.update.before, update.update.after), (0, 2));
    assert_eq!(update.experience, 2);
    assert_eq!(
        runtime.memory().get(trial.after).unwrap().evidence(),
        Some(request(1, 7, 0).after_evidence)
    );
    assert_eq!(runtime.action_trials(), &[trial]);
    assert_eq!(env.state(), 20);
    assert!(runtime
        .memory()
        .iter()
        .all(|m| m.status() == RecordStatus::Observed));
    let bytes = runtime.checkpoint(&Session::new()).unwrap();
    let (mut restored, _) = Runtime::restore(&bytes, &[sensor()], Access::default()).unwrap();
    assert_eq!(restored.neural().updates(), runtime.neural().updates());
    assert_eq!(restored.action_trials(), runtime.action_trials());
    assert!(restored.feedback(feedback).is_err());
    // Verificação de ligação/retencão de um contrato MP05, não aceite causal E03.
}

fn pursuit_goal(actions: usize) -> PursuitGoal {
    PursuitGoal {
        objective: neurons::cognitive_contracts::EpisodeId::new(901).unwrap(),
        task: goal(10, 30),
        max_actions: actions,
    }
}
#[test]
fn active_investigation_questions_explores_learns_and_requeries_same_objective() {
    let mut runtime = runtime();
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    let original = pursuit_goal(2);
    // Ordem dos tickets difere da execução; não se consultam destinos futuros.
    let report = runtime
        .pursue(&mut env, original, &[request(2, 9, 3), request(1, 7, 0)])
        .unwrap();
    assert_eq!(report.goal, original);
    assert_eq!(report.reason, PursuitReason::Reached);
    assert_eq!(report.observed, 30);
    assert_eq!(report.steps.len(), 2);
    assert!(report
        .steps
        .iter()
        .all(|s| s.planning == PlanReason::InsufficientEvidence
            && s.question.is_some()
            && s.exploratory));
    assert_eq!(
        report.steps[0].execution.as_ref().unwrap().reason,
        ExecutionReason::Observed
    );
    assert_eq!(
        report.steps[0].question.as_ref().unwrap().observed_trials,
        0
    );
    assert_eq!(
        report.steps[1].question.as_ref().unwrap().observed_trials,
        1
    );
    assert_eq!(runtime.action_trials().len(), 2);
    assert_eq!(
        runtime
            .plan(goal(10, 30), PlanningBudget::default())
            .unwrap()
            .reason,
        PlanReason::Supported
    );
}
#[test]
fn no_experience_authority_never_fabricates_progress() {
    let mut runtime = runtime();
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    let report = runtime.pursue(&mut env, pursuit_goal(2), &[]).unwrap();
    assert_eq!(report.reason, PursuitReason::NoAuthorizedAction);
    assert_eq!(report.observed, 10);
    assert_eq!(env.action_count(), 0);
    assert!(runtime.action_trials().is_empty());
    assert!(runtime.memory().is_empty());
}
#[test]
fn pursuit_uses_learned_plan_when_evidence_already_exists() {
    let mut runtime = acquired();
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    let report = runtime
        .pursue(
            &mut env,
            pursuit_goal(2),
            &[request(4, 9, 9), request(3, 7, 6)],
        )
        .unwrap();
    assert_eq!(report.reason, PursuitReason::Reached);
    assert_eq!(report.steps.len(), 1);
    assert!(!report.steps[0].exploratory);
    assert!(report.steps[0].question.is_none());
}
#[test]
fn divergent_prediction_is_retained_during_bounded_replanning() {
    let mut runtime = acquired();
    let mut env = world(10, &[(10, 7, 99), (99, 9, 30)]);
    let original = pursuit_goal(2);
    let report = runtime
        .pursue(&mut env, original, &[request(3, 7, 6), request(4, 9, 9)])
        .unwrap();
    assert_eq!(report.goal, original);
    assert_eq!(report.reason, PursuitReason::Reached);
    assert_eq!(report.steps.len(), 2);
    assert_eq!(
        report.steps[0].execution.as_ref().unwrap().reason,
        ExecutionReason::Diverged
    );
    assert_eq!(report.steps[1].local_goal.from, 99);
    assert_eq!(report.steps[1].local_goal.to, 30);
    assert!(report.steps[1].exploratory);
    assert_eq!(
        runtime
            .plan(goal(10, 30), PlanningBudget::default())
            .unwrap()
            .reason,
        PlanReason::ContradictoryEvidence,
        "atingir o objetivo não prova a previsão inicial"
    );
}
#[test]
fn pursuit_does_not_raise_budget_or_look_up_the_right_branch() {
    let mut candidate = runtime();
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    let partial = candidate
        .pursue(
            &mut env,
            pursuit_goal(1),
            &[request(1, 7, 0), request(2, 9, 3)],
        )
        .unwrap();
    assert_eq!(partial.reason, PursuitReason::ResourceLimited);
    assert_eq!(partial.observed, 20);
    assert_eq!(env.action_count(), 1);
    let mut candidate = runtime();
    let mut env = world(10, &[(10, 3, 99), (10, 7, 20), (20, 9, 30)]);
    let stopped = candidate
        .pursue(
            &mut env,
            pursuit_goal(2),
            &[request(1, 3, 0), request(2, 7, 3)],
        )
        .unwrap();
    assert_eq!(stopped.reason, PursuitReason::NoAuthorizedAction);
    assert_eq!(stopped.observed, 99);
    assert_eq!(env.action_count(), 1);
}

#[test]
fn pursuit_cannot_observe_an_unpermitted_interface() {
    let bytes = runtime().checkpoint(&Session::new()).unwrap();
    let mut denied = access();
    denied.sensors.clear();
    let (mut candidate, _) = Runtime::restore(&bytes, &[sensor()], denied).unwrap();
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    assert!(candidate
        .pursue(
            &mut env,
            pursuit_goal(2),
            &[request(1, 7, 0), request(2, 9, 3)]
        )
        .is_err());
    assert_eq!(candidate.planning_scans(), 0);
    assert_eq!(candidate.action_attempts(), 0);
    assert!(candidate.memory().is_empty());
    assert_eq!(env.action_count(), 0);
}

#[test]
fn active_exploration_uses_shared_attention_budget_without_partial_action() {
    let mut limits = Limits::default();
    limits.attention_scans = 3;
    let mut candidate = runtime_with_limits(limits);
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    let outcome = candidate
        .pursue(
            &mut env,
            pursuit_goal(2),
            &[request(1, 7, 0), request(2, 9, 3)],
        )
        .unwrap();
    assert_eq!(outcome.reason, PursuitReason::ResourceLimited);
    assert_eq!(candidate.attention_scans(), 0);
    assert_eq!(candidate.action_attempts(), 0);
    assert_eq!(env.action_count(), 0);
    let mut candidate = runtime();
    let mut env = world(10, &[(10, 7, 20), (20, 9, 30)]);
    assert_eq!(
        candidate
            .pursue(
                &mut env,
                pursuit_goal(2),
                &[request(1, 7, 0), request(2, 9, 3)]
            )
            .unwrap()
            .reason,
        PursuitReason::Reached
    );
    assert_eq!(candidate.attention_scans(), 8);
    let bytes = candidate.checkpoint(&Session::new()).unwrap();
    let (restored, _) = Runtime::restore(&bytes, &[sensor()], Access::default()).unwrap();
    assert_eq!(restored.attention_scans(), 8);
}

#[test]
fn declared_new_world_revision_needs_new_experience_and_preserves_old_conflict() {
    let mut candidate = acquired();
    let mut changed = world(10, &[(10, 7, 99)]);
    candidate
        .intervene(&mut changed, &request(3, 7, 6))
        .unwrap();
    assert_eq!(
        candidate
            .plan(goal(10, 30), PlanningBudget::default())
            .unwrap()
            .reason,
        PlanReason::ContradictoryEvidence
    );
    let updated_goal = PlanningGoal {
        revision: 2,
        ..goal(10, 30)
    };
    let missing = candidate
        .plan(updated_goal, PlanningBudget::default())
        .unwrap();
    assert_eq!(missing.reason, PlanReason::InsufficientEvidence);
    assert_eq!(missing.question.unwrap().observed_trials, 0);
    // A revisão é declarada pelo responsável da fonte, não inferida por IDs.
    let mut revised = World::new(501, 2, sensor(), 10, &[(10, 7, 30)]).unwrap();
    let mut permit = request(4, 7, 9);
    permit.permit.revision = 2;
    candidate.intervene(&mut revised, &permit).unwrap();
    assert_eq!(
        candidate
            .plan(updated_goal, PlanningBudget::default())
            .unwrap()
            .reason,
        PlanReason::Supported
    );
    assert_eq!(
        candidate
            .action_trials()
            .iter()
            .filter(|t| t.revision == 1 && t.from == 10 && t.action == 7)
            .map(|t| t.to)
            .collect::<Vec<_>>(),
        vec![20, 99]
    );
    assert_eq!(candidate.action_trials().len(), 4);
    let bytes = candidate.checkpoint(&Session::new()).unwrap();
    let (mut restored, _) = Runtime::restore(&bytes, &[sensor()], access()).unwrap();
    assert_eq!(
        restored
            .plan(updated_goal, PlanningBudget::default())
            .unwrap()
            .reason,
        PlanReason::Supported
    );
    assert_eq!(restored.action_trials(), candidate.action_trials());
}

fn transition_query(from: i32, action: u32) -> TransitionQuery {
    TransitionQuery {
        world: 501,
        revision: 1,
        context: 61,
        source: SourceId::new(7).unwrap(),
        from,
        action,
    }
}
#[test]
fn contextual_prediction_explains_evidence_without_querying_environment() {
    let mut empty = runtime();
    assert_eq!(
        empty.predict(transition_query(10, 7)).unwrap().status,
        PredictionStatus::Unobserved
    );
    let mut candidate = acquired();
    let memory = candidate.memory().iter().collect::<Vec<_>>();
    let prediction = candidate.predict(transition_query(10, 7)).unwrap();
    assert_eq!(prediction.status, PredictionStatus::Observed);
    assert_eq!(
        prediction.outcomes,
        vec![OutcomeEvidence {
            to: 20,
            trials: vec![1]
        }]
    );
    let other = candidate.predict(transition_query(20, 9)).unwrap();
    assert_eq!(
        other.outcomes,
        vec![OutcomeEvidence {
            to: 30,
            trials: vec![2]
        }]
    );
    let wrong_scope = TransitionQuery {
        context: 62,
        ..transition_query(10, 7)
    };
    assert_eq!(
        candidate.predict(wrong_scope).unwrap().status,
        PredictionStatus::Unobserved
    );
    assert_eq!(candidate.memory().iter().collect::<Vec<_>>(), memory);
}
#[test]
fn more_agreeing_trials_do_not_hide_an_opposing_observation() {
    let mut candidate = acquired();
    let mut repetition = world(10, &[(10, 7, 20)]);
    candidate
        .intervene(&mut repetition, &request(3, 7, 6))
        .unwrap();
    let mut opposed = world(10, &[(10, 7, 99)]);
    candidate
        .intervene(&mut opposed, &request(4, 7, 9))
        .unwrap();
    let report = candidate.predict(transition_query(10, 7)).unwrap();
    assert_eq!(report.status, PredictionStatus::Conflicting);
    assert_eq!(
        report.outcomes,
        vec![
            OutcomeEvidence {
                to: 20,
                trials: vec![1, 3]
            },
            OutcomeEvidence {
                to: 99,
                trials: vec![4]
            }
        ]
    );
}
#[test]
fn prediction_and_its_uncertainty_are_preserved_without_restoring_read_authority() {
    let mut candidate = acquired();
    let before = candidate.predict(transition_query(10, 7)).unwrap();
    let bytes = candidate.checkpoint(&Session::new()).unwrap();
    let (mut denied, _) = Runtime::restore(&bytes, &[sensor()], Access::default()).unwrap();
    assert!(denied.predict(transition_query(10, 7)).is_err());
    let (mut restored, _) = Runtime::restore(&bytes, &[sensor()], access()).unwrap();
    assert_eq!(restored.predict(transition_query(10, 7)).unwrap(), before);
}
#[test]
fn prediction_cannot_bypass_the_shared_query_budget_after_restart() {
    let mut candidate = runtime();
    for _ in 0..4 {
        assert_eq!(
            candidate.predict(transition_query(10, 7)).unwrap().status,
            PredictionStatus::Unobserved
        );
    }
    let bytes = candidate.checkpoint(&Session::new()).unwrap();
    let (mut restored, _) = Runtime::restore(&bytes, &[sensor()], access()).unwrap();
    assert_eq!(
        restored.predict(transition_query(10, 7)).unwrap().status,
        PredictionStatus::ResourceLimited
    );
    assert_eq!(
        restored
            .plan(goal(10, 30), PlanningBudget::default())
            .unwrap()
            .reason,
        PlanReason::ResourceLimited
    );
}
