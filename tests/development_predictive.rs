//! Integração funcional pública; nenhuma chave de avaliação é entrada do candidato.
use neurons::cognition::concepts::Attributes;
use neurons::cognitive_contracts::*;
use neurons::curriculum::{PedagogyAssessment, PedagogyHost, PedagogyPermissions};
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::{Event, EventKind, NodeId};
use neurons::integration::{development::*, synaptic::SynapticEpisode};
use neurons::session::f1::Session;
fn n(v: u32) -> NodeId {
    NodeId::new(v).unwrap()
}
fn source() -> SourceId {
    SourceId::new(7).unwrap()
}
fn sensors() -> Vec<Sensor> {
    [Encoding::Scalar, Encoding::BooleanAttributes]
        .into_iter()
        .enumerate()
        .map(|(i, encoding)| Sensor {
            id: i as u32 + 1,
            source: source(),
            context: 61,
            input: n(101),
            encoding,
        })
        .collect()
}
fn access() -> Access {
    Access {
        record: true,
        explore: true,
        read_knowledge: true,
        learn_concepts: true,
        sensors: vec![1, 2],
        ..Default::default()
    }
}
fn runtime_with(a: Access, limits: Limits) -> Runtime {
    let mut neural = SynapticEpisode::new();
    for c in [
        "novo 41 61 81 1 0 3 2 16 64",
        "neuronio 101 1 4",
        "neuronio 103 2 4",
        "sinapse 11 101 103 0 1",
    ] {
        neural.execute(c).unwrap();
    }
    Runtime::new(neural, &sensors(), a, limits).unwrap()
}
fn runtime() -> Runtime {
    runtime_with(access(), Limits::default())
}
fn session() -> Session {
    let mut s = Session::new();
    for c in [
        "config 91 61 81 1 0 1000",
        "conceito 601 consequencia",
        "agora 100",
    ] {
        s.execute(c).unwrap();
    }
    s
}
fn contract(s: &Session) -> ConsequenceContract {
    ConsequenceContract {
        concept: n(601),
        scope: s.pedagogy_identity().unwrap().1,
        world: 501,
        revision: 1,
        source: source(),
        action: 7,
        target: 20,
    }
}
fn world(from: i32, table: &[(i32, u32, i32)]) -> World {
    World::new(501, 1, sensors()[0].clone(), from, table).unwrap()
}
fn reading(id: u32, bits: [u8; 6], at: u64) -> Reading {
    let payload = 1 + bits
        .iter()
        .enumerate()
        .map(|(i, b)| i32::from(*b) << i)
        .sum::<i32>();
    Reading {
        id: 100 + id,
        item: n(501),
        sensor: 2,
        source: source(),
        evidence: EvidenceId::new(3000 + id).unwrap(),
        event: Event::new(None, n(101), EventKind::Stimulus, payload, at),
    }
}
fn request(id: u32, action: u32, at: u64) -> Intervention {
    Intervention {
        id,
        permit: ActionPermit {
            id: 1000 + id,
            world: 501,
            revision: 1,
            context: 61,
            source: source(),
            action,
        },
        before_id: 2 * id,
        after_id: 2 * id + 1,
        before_evidence: EvidenceId::new(4000 + 2 * id).unwrap(),
        after_evidence: EvidenceId::new(4001 + 2 * id).unwrap(),
        at,
    }
}
fn history() -> (Runtime, Session) {
    let mut r = runtime();
    let mut s = session();
    let c = contract(&s);
    let mut w = world(10, &[(10, 7, 20)]);
    let frame = r
        .observe_attributes(&w, 7, reading(1, [1, 0, 0, 0, 0, 0], 0))
        .unwrap();
    r.intervene(&mut w, &request(1, 7, 2)).unwrap();
    let assessment = PedagogyAssessment::new(
        &[Attributes::new([1, 0, 1, 0, 1, 0]).unwrap().into()],
        &[Attributes::new([0, 1, 0, 1, 0, 1]).unwrap().into()],
    )
    .unwrap();
    let report = r
        .teach_consequences(
            &mut s,
            c,
            &[(frame, 1, EngramId::new(501).unwrap(), 901)],
            assessment,
            PedagogyPermissions {
                evaluation: Some(c.scope),
                presentation: Some(c.scope),
                admissions: vec![EngramId::new(501).unwrap()],
            },
        )
        .unwrap();
    assert_eq!(report.pedagogy.final_version.number, 1);
    let mut w = world(20, &[(20, 9, 30)]);
    r.intervene(&mut w, &request(2, 9, 10)).unwrap();
    (r, s)
}
fn goal(from: i32) -> PursuitGoal {
    PursuitGoal {
        objective: EpisodeId::new(77).unwrap(),
        max_actions: 2,
        task: PlanningGoal {
            world: 501,
            revision: 1,
            context: 61,
            source: source(),
            from,
            to: 30,
        },
    }
}
fn tickets() -> Vec<Intervention> {
    vec![request(10, 7, 32), request(11, 9, 50)]
}
fn run(r: &mut Runtime, h: &Runtime, s: &Session, w: &mut World, from: i32) -> PredictivePursuit {
    r.pursue_with_consequences(
        h,
        s,
        w,
        goal(from),
        contract(s),
        reading(9, [1, 1, 1, 0, 0, 0], 30),
        &tickets(),
    )
    .unwrap()
}
#[test]
fn novel_objective_uses_learned_prefix_and_observed_tail() {
    let (h, s) = history();
    let old = h.checkpoint(&s).unwrap();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    let before = s.encode();
    let out = run(&mut r, &h, &s, &mut w, 999);
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(out.choice, PredictiveChoice::ConceptualPrefix);
    assert_eq!(out.goal, goal(999));
    assert_eq!(out.review.as_ref().unwrap().agreement, Some(true));
    assert_eq!(
        out.observed_alternative.as_ref().unwrap().reason,
        PlanReason::InsufficientEvidence
    );
    assert_eq!(
        out.prefix_alternative
            .as_ref()
            .unwrap()
            .plan
            .as_ref()
            .unwrap()
            .steps()[0]
            .id,
        2
    );
    assert_eq!(out.execution.as_ref().unwrap().trials[0].action, 9);
    assert_eq!(
        r.action_trials()
            .iter()
            .map(|t| t.action)
            .collect::<Vec<_>>(),
        vec![7, 9]
    );
    assert_eq!(r.experiences().len(), 5);
    assert_eq!(s.encode(), before);
    assert_eq!(h.checkpoint(&s).unwrap(), old);
    assert!(s.memory().is_empty());
    assert!(!out.justification.is_empty());
    assert_eq!(r.cognitive_resources().conceptual_queries, 2);
    assert!(r.planning_scans() <= 32);
}
#[test]
fn fully_observed_shorter_alternative_wins_without_unnecessary_prefix() {
    let (mut h, s) = history();
    let mut sample = world(999, &[(999, 5, 30)]);
    h.intervene(&mut sample, &request(3, 5, 20)).unwrap();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30), (999, 5, 30)]);
    let mut requests = tickets();
    requests.push(request(12, 5, 32));
    let out = r
        .pursue_with_consequences(
            &h,
            &s,
            &mut w,
            goal(999),
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &requests,
        )
        .unwrap();
    assert_eq!(out.choice, PredictiveChoice::ObservedRoute);
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(r.action_trials().len(), 1);
    assert_eq!(r.action_trials()[0].action, 5);
}
#[test]
fn equivalent_two_step_routes_prefer_fully_observed_evidence() {
    let (h, s) = history();
    let mut r = runtime();
    let mut w = world(10, &[(10, 7, 20), (20, 9, 30)]);
    let out = run(&mut r, &h, &s, &mut w, 10);
    assert_eq!(out.choice, PredictiveChoice::ObservedRoute);
    assert_eq!(out.reason, PredictiveReason::Reached);
}
#[test]
fn missing_tail_authorization_denies_before_the_first_action() {
    let (h, s) = history();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    let out = r
        .pursue_with_consequences(
            &h,
            &s,
            &mut w,
            goal(999),
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets()[..1],
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Denied);
    assert_eq!(w.action_count(), 0);
    assert!(r.action_trials().is_empty());
}
#[test]
fn permissions_are_not_inherited_from_the_memory_runtime() {
    let (h, s) = history();
    for denied in 0..3 {
        let mut a = access();
        match denied {
            0 => a.explore = false,
            1 => a.read_knowledge = false,
            _ => a.record = false,
        }
        let mut r = runtime_with(a, Limits::default());
        let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
        assert!(r
            .pursue_with_consequences(
                &h,
                &s,
                &mut w,
                goal(999),
                contract(&s),
                reading(9, [1, 1, 1, 0, 0, 0], 30),
                &tickets()
            )
            .is_err());
        assert_eq!(w.action_count(), 0);
        assert!(r.experiences().is_empty());
    }
}
#[test]
fn full_plan_reserve_is_required_before_any_action() {
    let (h, s) = history();
    for kind in 0..3 {
        let mut limits = Limits::default();
        match kind {
            0 => limits.observations = 4,
            1 => limits.neural_events = 64,
            _ => limits.attention_scans = 1,
        }
        let mut r = runtime_with(access(), limits);
        let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
        let out = run(&mut r, &h, &s, &mut w, 999);
        assert_eq!(out.reason, PredictiveReason::ResourceLimited);
        assert_eq!(w.action_count(), 0);
        assert_eq!(r.action_attempts(), 0);
    }
}
#[test]
fn wrong_prediction_is_retained_and_unreachable_result_is_not_success() {
    let (h, s) = history();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 77), (20, 9, 30)]);
    let out = run(&mut r, &h, &s, &mut w, 999);
    assert_eq!(out.reason, PredictiveReason::Diverged);
    assert_eq!(out.review.as_ref().unwrap().agreement, Some(false));
    assert_eq!(out.observed, 77);
    assert_eq!(out.goal.task.to, 30);
    assert_eq!(w.action_count(), 1);
    assert_eq!(r.action_trials()[0].to, 77);
    assert!(out.replanning.is_some());
}
#[test]
fn divergence_replans_from_actual_state_and_uses_an_authorized_alternative() {
    let (mut h, s) = history();
    let mut sample = world(77, &[(77, 11, 30)]);
    h.intervene(&mut sample, &request(3, 11, 20)).unwrap();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 77), (20, 9, 30), (77, 11, 30)]);
    let mut requests = tickets();
    requests.push(request(12, 11, 50));
    let out = r
        .pursue_with_consequences(
            &h,
            &s,
            &mut w,
            goal(999),
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &requests,
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(out.review.as_ref().unwrap().agreement, Some(false));
    assert_eq!(
        out.replanning
            .as_ref()
            .unwrap()
            .plan
            .as_ref()
            .unwrap()
            .goal()
            .from,
        77
    );
    assert_eq!(
        r.action_trials()
            .iter()
            .map(|t| t.action)
            .collect::<Vec<_>>(),
        vec![7, 11]
    );
    assert_eq!(out.goal, goal(999));
}
#[test]
fn conflicting_memory_suspends_application_despite_positive_h23() {
    let (mut h, s) = history();
    let mut sample = world(10, &[(10, 7, 21)]);
    h.intervene(&mut sample, &request(3, 7, 20)).unwrap();
    let mut r = runtime();
    let mut w = world(10, &[(10, 7, 20), (20, 9, 30)]);
    let out = run(&mut r, &h, &s, &mut w, 10);
    assert_eq!(out.reason, PredictiveReason::ConflictingEvidence);
    assert_eq!(w.action_count(), 0);
}
#[test]
fn no_experience_control_does_not_receive_an_answer_from_the_environment() {
    let mut h = runtime();
    let s = session();
    let mut sample = world(20, &[(20, 9, 30)]);
    h.intervene(&mut sample, &request(2, 9, 10)).unwrap();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    let out = run(&mut r, &h, &s, &mut w, 999);
    assert_eq!(out.reason, PredictiveReason::InsufficientEvidence);
    assert_eq!(w.action_count(), 0);
}
#[test]
fn hidden_outcome_changes_observation_but_not_initial_action_choice() {
    for destination in [20, 77, -4] {
        let (h, s) = history();
        let mut r = runtime();
        let mut w = world(999, &[(999, 7, destination), (20, 9, 30)]);
        let out = run(&mut r, &h, &s, &mut w, 999);
        assert_eq!(out.choice, PredictiveChoice::ConceptualPrefix);
        assert_eq!(r.action_trials()[0].action, 7);
        assert_eq!(r.action_trials()[0].to, destination);
        assert_eq!(out.reason == PredictiveReason::Reached, destination == 20);
    }
}
#[test]
fn checkpoint_and_fresh_episode_apply_newly_acquired_transition_knowledge() {
    let (h, s) = history();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    assert_eq!(
        run(&mut r, &h, &s, &mut w, 999).reason,
        PredictiveReason::Reached
    );
    let bytes = r.checkpoint(&s).unwrap();
    let (memory, s) = Runtime::restore(&bytes, &sensors(), Access::default()).unwrap();
    let mut fresh = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    let out = fresh
        .pursue_with_consequences(
            &memory,
            &s,
            &mut w,
            goal(999),
            contract(&s),
            reading(19, [1, 1, 1, 0, 0, 0], 30),
            &[request(20, 7, 32), request(21, 9, 50)],
        )
        .unwrap();
    assert_eq!(out.choice, PredictiveChoice::ObservedRoute);
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(memory.checkpoint(&s).unwrap(), bytes);
}
#[test]
fn restoration_in_another_process_requires_new_permissions_and_reaches_novel_goal() {
    let (h, s) = history();
    let path = std::env::temp_dir().join(format!("neurons-predictive-{}.bin", std::process::id()));
    assert!(!path.exists());
    std::fs::write(&path, h.checkpoint(&s).unwrap()).unwrap();
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "predictive_restore_worker", "--nocapture"])
        .env("NEURONS_PREDICTIVE_RESTORE", &path)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    std::fs::remove_file(path).unwrap();
}
#[test]
fn predictive_restore_worker() {
    let Some(path) = std::env::var_os("NEURONS_PREDICTIVE_RESTORE") else {
        return;
    };
    let bytes = std::fs::read(path).unwrap();
    let (h, s) = Runtime::restore(&bytes, &sensors(), Access::default()).unwrap();
    let mut r = runtime();
    let mut w = world(888, &[(888, 7, 20), (20, 9, 30)]);
    let out = run(&mut r, &h, &s, &mut w, 888);
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(out.choice, PredictiveChoice::ConceptualPrefix);
    assert_eq!(h.checkpoint(&s).unwrap(), bytes);
}

#[test]
fn authorized_longer_route_is_used_when_shortcut_lacks_permission() {
    let (mut h, s) = history();
    let mut sample = world(999, &[(999, 5, 30)]);
    h.intervene(&mut sample, &request(3, 5, 20)).unwrap();
    let mut r = runtime();
    let mut w = world(999, &[(999, 5, 30), (999, 7, 20), (20, 9, 30)]);
    let out = run(&mut r, &h, &s, &mut w, 999);
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(out.choice, PredictiveChoice::ConceptualPrefix);
    assert_eq!(
        r.action_trials()
            .iter()
            .map(|t| t.action)
            .collect::<Vec<_>>(),
        vec![7, 9]
    );
}
#[test]
fn consumed_memory_grant_cannot_be_reused_in_a_new_runtime() {
    let (h, s) = history();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    assert!(r
        .pursue_with_consequences(
            &h,
            &s,
            &mut w,
            goal(999),
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &[request(1, 7, 32), request(11, 9, 50)]
        )
        .is_err());
    assert_eq!(w.action_count(), 0);
    assert!(r.experiences().is_empty());
}
#[test]
fn known_route_divergence_also_replans_without_a_second_planner() {
    let (mut h, s) = history();
    let mut sample = world(77, &[(77, 11, 30)]);
    h.intervene(&mut sample, &request(3, 11, 20)).unwrap();
    let mut r = runtime();
    let mut w = world(10, &[(10, 7, 77), (20, 9, 30), (77, 11, 30)]);
    let mut requests = tickets();
    requests.push(request(12, 11, 50));
    let out = r
        .pursue_with_consequences(
            &h,
            &s,
            &mut w,
            goal(10),
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &requests,
        )
        .unwrap();
    assert_eq!(out.choice, PredictiveChoice::ObservedRoute);
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(
        out.previous_execution.as_ref().unwrap().reason,
        ExecutionReason::Diverged
    );
    assert_eq!(
        out.replanning
            .as_ref()
            .unwrap()
            .plan
            .as_ref()
            .unwrap()
            .goal()
            .from,
        77
    );
    assert_eq!(
        r.action_trials()
            .iter()
            .map(|t| t.action)
            .collect::<Vec<_>>(),
        vec![7, 11]
    );
}
#[test]
fn insufficient_attributes_do_not_make_a_hypothetical_route_executable() {
    let (h, s) = history();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    let out = r
        .pursue_with_consequences(
            &h,
            &s,
            &mut w,
            goal(999),
            contract(&s),
            reading(9, [0, 1, 1, 0, 0, 0], 30),
            &tickets(),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::InsufficientEvidence);
    assert_eq!(w.action_count(), 0);
}

#[test]
fn denied_state_sensor_cannot_be_bypassed_by_attribute_sensor() {
    let (h, s) = history();
    let mut a = access();
    a.sensors = vec![2];
    let mut r = runtime_with(a, Limits::default());
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    assert!(r
        .pursue_with_consequences(
            &h,
            &s,
            &mut w,
            goal(999),
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets()
        )
        .is_err());
    assert!(r.experiences().is_empty());
    assert_eq!(w.action_count(), 0);
}
#[test]
fn retained_counterexample_blocks_the_same_false_prefix_in_a_new_episode() {
    let (h, s) = history();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 77), (20, 9, 30)]);
    let out = run(&mut r, &h, &s, &mut w, 999);
    assert_eq!(out.reason, PredictiveReason::Diverged);
    let bytes = r.checkpoint(&s).unwrap();
    let (memory, s) = Runtime::restore(&bytes, &sensors(), Access::default()).unwrap();
    let mut fresh = runtime();
    let mut w = world(999, &[(999, 7, 77), (20, 9, 30)]);
    let out = fresh
        .pursue_with_consequences(
            &memory,
            &s,
            &mut w,
            goal(999),
            contract(&s),
            reading(19, [1, 1, 1, 0, 0, 0], 30),
            &[request(20, 7, 32), request(21, 9, 50)],
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::ConflictingEvidence);
    assert_eq!(w.action_count(), 0);
    assert_eq!(out.forecast.unwrap().transitions().outcomes[0].to, 77);
}

fn revision(
    s: &Session,
    record: u32,
    receipt: u32,
    before: [u8; 6],
    after: [u8; 6],
) -> PredictiveRevision {
    let c = contract(s);
    PredictiveRevision {
        record: EngramId::new(record).unwrap(),
        receipt,
        assessment: PedagogyAssessment::new(
            &[Attributes::new(before).unwrap().into()],
            &[Attributes::new(after).unwrap().into()],
        )
        .unwrap(),
        permissions: PedagogyPermissions {
            evaluation: Some(c.scope),
            presentation: Some(c.scope),
            admissions: vec![EngramId::new(record).unwrap()],
        },
    }
}
#[test]
fn insufficient_goal_learns_from_authorized_experience_then_finishes_the_plan() {
    let mut h = runtime();
    let mut s = session();
    let mut sample = world(20, &[(20, 9, 30)]);
    h.intervene(&mut sample, &request(2, 9, 10)).unwrap();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    let c = contract(&s);
    let grant = revision(&s, 501, 901, [1, 0, 1, 0, 1, 0], [0, 1, 0, 1, 0, 1]);
    let out = r
        .pursue_learning_goal(
            &h,
            &mut s,
            &mut w,
            goal(999),
            c,
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets(),
            Some(grant),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(out.choice, PredictiveChoice::SupervisedExperience);
    assert_eq!(
        out.revision
            .as_ref()
            .unwrap()
            .pedagogy
            .initial_version
            .number,
        0
    );
    assert_eq!(
        out.revision.as_ref().unwrap().pedagogy.final_version.number,
        1
    );
    assert_eq!(
        out.revised_query
            .as_ref()
            .unwrap()
            .version
            .as_ref()
            .unwrap()
            .number,
        1
    );
    assert_eq!(
        out.forecast
            .as_ref()
            .unwrap()
            .report()
            .version
            .as_ref()
            .unwrap()
            .number,
        0
    );
    assert_eq!(out.goal, goal(999));
    assert_eq!(r.experiences().len(), 5);
    assert_eq!(r.cognitive_resources().conceptual_queries, 4);
    assert!(s.memory().is_empty());
    assert_eq!(w.state(), 30);
}
#[test]
fn pedagogical_permission_is_required_before_exploratory_acquisition() {
    let mut h = runtime();
    let mut s = session();
    let mut sample = world(20, &[(20, 9, 30)]);
    h.intervene(&mut sample, &request(2, 9, 10)).unwrap();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    let c = contract(&s);
    let mut grant = revision(&s, 501, 901, [1, 0, 1, 0, 1, 0], [0, 1, 0, 1, 0, 1]);
    grant.permissions.admissions.clear();
    let before = s.encode();
    let out = r
        .pursue_learning_goal(
            &h,
            &mut s,
            &mut w,
            goal(999),
            c,
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets(),
            Some(grant),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Denied);
    assert_eq!(w.action_count(), 0);
    assert_eq!(s.encode(), before);
}
#[test]
fn authorized_negative_experience_refines_h23_and_replans_a_new_goal() {
    let (mut h, mut s) = history();
    let mut sample = world(77, &[(77, 11, 30)]);
    h.intervene(&mut sample, &request(3, 11, 20)).unwrap();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 77), (20, 9, 30), (77, 11, 30)]);
    let mut requests = tickets();
    requests.push(request(12, 11, 50));
    let c = contract(&s);
    let grant = revision(&s, 502, 902, [0, 1, 0, 1, 0, 0], [1, 1, 0, 1, 0, 0]);
    let out = r
        .pursue_learning_goal(
            &h,
            &mut s,
            &mut w,
            goal(999),
            c,
            reading(9, [0, 0, 0, 0, 0, 0], 30),
            &requests,
            Some(grant),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(out.choice, PredictiveChoice::SupervisedExperience);
    assert_eq!(
        out.revision
            .as_ref()
            .unwrap()
            .pedagogy
            .initial_version
            .number,
        1
    );
    assert_eq!(
        out.revision.as_ref().unwrap().pedagogy.final_version.number,
        2
    );
    assert_eq!(
        out.revised_query.as_ref().unwrap().decision,
        neurons::metacognition::concepts::ConceptDecision::Supported(
            neurons::cognition::concepts::Label::Negative
        )
    );
    assert_eq!(
        r.action_trials()
            .iter()
            .map(|t| t.action)
            .collect::<Vec<_>>(),
        vec![7, 11]
    );
    let bytes = r.checkpoint(&s).unwrap();
    let (memory, s) = Runtime::restore(&bytes, &sensors(), Access::default()).unwrap();
    let mut fresh = runtime();
    let mut w = world(999, &[(999, 7, 77), (77, 11, 30)]);
    let out = fresh
        .pursue_with_consequences(
            &memory,
            &s,
            &mut w,
            goal(999),
            c,
            reading(19, [0, 0, 0, 0, 0, 0], 30),
            &[request(20, 7, 32), request(21, 11, 50)],
        )
        .unwrap();
    assert_eq!(out.choice, PredictiveChoice::ObservedRoute);
    assert_eq!(out.reason, PredictiveReason::Reached);
}

#[test]
fn refuted_prediction_is_not_forced_into_incompatible_h23_family() {
    let (h, mut s) = history();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 77), (20, 9, 30)]);
    let c = contract(&s);
    let grant = revision(&s, 502, 902, [0, 1, 0, 1, 0, 0], [1, 1, 0, 1, 0, 0]);
    let out = r
        .pursue_learning_goal(
            &h,
            &mut s,
            &mut w,
            goal(999),
            c,
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets(),
            Some(grant),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Diverged);
    assert_eq!(out.review.as_ref().unwrap().agreement, Some(false));
    assert_eq!(out.observed, 77);
    assert_eq!(w.action_count(), 1);
    let report = out.revision.unwrap();
    assert_eq!(report.pedagogy.initial_version.number, 1);
    assert_eq!(report.pedagogy.final_version.number, 1);
    assert!(report.pedagogy.lessons.is_empty());
    assert!(out.failure.is_some());
    assert_eq!(r.action_trials()[0].to, 77);
    assert!(out.revised_query.is_none());
    assert!(r.cognitive_resources().conceptual_queries <= 7);
}
#[test]
fn refuted_prediction_without_pedagogy_authority_preserves_model_and_observed_failure() {
    for denied in 0..4 {
        let (h, mut s) = history();
        let before = s.encode();
        let mut a = access();
        if denied == 0 {
            a.learn_concepts = false;
        }
        let mut r = runtime_with(a, Limits::default());
        let mut w = world(999, &[(999, 7, 77), (20, 9, 30)]);
        let c = contract(&s);
        let mut grant = revision(&s, 502, 902, [0, 1, 0, 1, 0, 0], [1, 1, 0, 1, 0, 0]);
        match denied {
            1 => grant.permissions.evaluation = None,
            2 => grant.permissions.presentation = None,
            3 => grant.permissions.admissions.clear(),
            _ => {}
        }
        let out = r
            .pursue_learning_goal(
                &h,
                &mut s,
                &mut w,
                goal(999),
                c,
                reading(9, [1, 1, 1, 0, 0, 0], 30),
                &tickets(),
                Some(grant),
            )
            .unwrap();
        assert_eq!(out.reason, PredictiveReason::Diverged);
        assert!(out.revision.is_none());
        assert!(out.failure.is_some());
        assert_eq!(s.encode(), before);
        assert_eq!(w.action_count(), 1);
    }
}
#[test]
fn correct_prediction_does_not_trigger_unnecessary_feedback_teaching() {
    let (h, mut s) = history();
    let before = s.encode();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    let c = contract(&s);
    let grant = revision(&s, 502, 902, [0, 1, 0, 1, 0, 0], [1, 1, 0, 1, 0, 0]);
    let out = r
        .pursue_learning_goal(
            &h,
            &mut s,
            &mut w,
            goal(999),
            c,
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets(),
            Some(grant),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert!(out.revision.is_none());
    assert_eq!(s.encode(), before);
}

fn observed_feedback(authorized: bool) -> (Runtime, Session, PredictivePursuit, World) {
    let (mut h, mut s) = history();
    let mut sample = world(999, &[(999, 7, 77)]);
    h.intervene(&mut sample, &request(3, 7, 20)).unwrap();
    let mut r = runtime();
    let mut w = world(999, &[(999, 7, 77)]);
    let c = contract(&s);
    let grant = revision(&s, 502, 902, [0, 1, 0, 1, 0, 0], [1, 1, 0, 1, 0, 0]);
    let mut g = goal(999);
    g.task.to = 77;
    let out = r
        .pursue_learning_goal(
            &h,
            &mut s,
            &mut w,
            g,
            c,
            reading(9, [0, 0, 0, 0, 0, 0], 30),
            &tickets(),
            authorized.then_some(grant),
        )
        .unwrap();
    (r, s, out, w)
}
#[test]
fn achieved_observed_route_can_still_learn_from_conceptual_uncertainty() {
    let (r, _s, out, w) = observed_feedback(true);
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(out.choice, PredictiveChoice::ObservedRoute);
    assert_eq!(out.review.as_ref().unwrap().agreement, None);
    assert_eq!(w.action_count(), 1);
    let trained = out.revision.unwrap();
    assert_eq!(trained.pedagogy.final_version.number, 2);
    assert!(trained
        .pedagogy
        .lessons
        .iter()
        .any(|l| l.admitted && l.lesson.label == neurons::cognition::concepts::Label::Negative));
    assert!(out.revised_query.is_none());
    assert!(r.cognitive_resources().conceptual_queries <= 7);
}

#[test]
fn same_observed_route_without_feedback_grant_leaves_concept_unchanged() {
    let (_, s, out, w) = observed_feedback(false);
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(w.action_count(), 1);
    assert_eq!(s.pedagogy_version().unwrap().number, 1);
    assert!(out.revision.is_none());
}
#[test]
fn feedback_learning_is_retained_for_unseen_attributes_after_new_process() {
    let (r, s, _, _) = observed_feedback(true);
    let path = std::env::temp_dir().join(format!("neurons-feedback-{}.bin", std::process::id()));
    assert!(!path.exists());
    let bytes = r.checkpoint(&s).unwrap();
    std::fs::write(&path, &bytes).unwrap();
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "feedback_restore_worker", "--nocapture"])
        .env("NEURONS_FEEDBACK_RESTORE", &path)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    std::fs::remove_file(path).unwrap();
}
#[test]
fn feedback_restore_worker() {
    let Some(path) = std::env::var_os("NEURONS_FEEDBACK_RESTORE") else {
        return;
    };
    let bytes = std::fs::read(path).unwrap();
    let (mut old, s) = Runtime::restore(&bytes, &sensors(), Access::default()).unwrap();
    assert_eq!(s.pedagogy_version().unwrap().number, 2);
    let w = world(1234, &[(1234, 7, 55)]);
    let attributes = reading(19, [0, 1, 0, 1, 1, 0], 50);
    let (baseline, baseline_session) = history();
    let mut control = runtime();
    control.inherit_consequence_contract(&baseline).unwrap();
    let control_frame = control.observe_attributes(&w, 7, attributes).unwrap();
    let control_report = control
        .forecast_consequence(
            &baseline_session,
            contract(&baseline_session),
            &control_frame,
            EpisodeId::new(88).unwrap(),
        )
        .unwrap();
    assert!(matches!(
        control_report.report().decision,
        neurons::metacognition::concepts::ConceptDecision::Abstain(_)
    ));
    assert!(old.observe_attributes(&w, 7, attributes).is_err());
    let mut fresh = runtime();
    fresh.inherit_consequence_contract(&old).unwrap();
    let frame = fresh.observe_attributes(&w, 7, attributes).unwrap();
    let forecast = fresh
        .forecast_consequence(&s, contract(&s), &frame, EpisodeId::new(88).unwrap())
        .unwrap();
    assert_eq!(
        forecast.report().decision,
        neurons::metacognition::concepts::ConceptDecision::Supported(
            neurons::cognition::concepts::Label::Negative
        )
    );
    assert_eq!(forecast.report().version.as_ref().unwrap().number, 2);
    assert_eq!(old.checkpoint(&s).unwrap(), bytes);
}
