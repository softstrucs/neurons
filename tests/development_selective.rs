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

use std::collections::{BTreeSet, VecDeque};
#[derive(Clone)]
struct Row {
    episode: u32,
    trial: ActionTrial,
    before: Reading,
    after: Reading,
}
fn capture(id: u32, r: &Runtime) -> Vec<Row> {
    r.action_trials()
        .iter()
        .map(|t| Row {
            episode: id,
            trial: t.clone(),
            before: r
                .experiences()
                .iter()
                .find(|v| v.record == t.before)
                .unwrap()
                .reading,
            after: r
                .experiences()
                .iter()
                .find(|v| v.record == t.after)
                .unwrap()
                .reading,
        })
        .collect()
}
fn eid(v: u32) -> EpisodeId {
    EpisodeId::new(v).unwrap()
}
fn task(from: i32, to: i32) -> PlanningGoal {
    PlanningGoal {
        to,
        ..goal(from).task
    }
}
fn selector(g: PlanningGoal) -> SelectiveQuery {
    SelectiveQuery {
        goal: g,
        conceptual_seed: Some(20),
    }
}
fn corpus(opposition: bool, stale_tail: bool) -> (EpisodeCatalog, Session, Vec<Row>) {
    let (h, s) = history();
    let mut all = capture(1, &h);
    let mut segment = EpisodeArchive::new();
    segment.insert(eid(1), &h, &s).unwrap();
    let mut catalog = EpisodeCatalog::new();
    for id in 2..=40u32 {
        let (from, action, to) = if id == 39 {
            (30, 11, 40)
        } else if id == 40 && opposition {
            (20, 9, 77)
        } else {
            (10000 + id as i32, 3, 10001 + id as i32)
        };
        let mut r = runtime();
        let revision = if id == 39 && stale_tail { 2 } else { 1 };
        let mut w = World::new(
            501,
            revision,
            sensors()[0].clone(),
            from,
            &[(from, action, to)],
        )
        .unwrap();
        let mut req = request(1, action, 20);
        req.permit.id = 20_000 + id;
        req.permit.revision = revision;
        req.before_evidence = EvidenceId::new(30_000 + 2 * id).unwrap();
        req.after_evidence = EvidenceId::new(30_001 + 2 * id).unwrap();
        r.intervene(&mut w, &req).unwrap();
        all.extend(capture(id, &r));
        segment.insert(eid(id), &r, &s).unwrap();
        if segment.len() == 4 {
            catalog
                .insert(id / 4, std::mem::take(&mut segment))
                .unwrap();
        }
    }
    (catalog, s, all)
}
struct Oracle {
    required: BTreeSet<(u32, u32)>,
    closed: BTreeSet<i32>,
    row_visits: usize,
    conflicts: BTreeSet<(i32, u32)>,
}
// Referência exaustiva: percorre a lista bruta emitida pelas intervenções.
// Não consulta catálogo, índices, leitor contextual ou resultado do candidato.
fn exhaustive(rows: &[Row], s: &Session, q: SelectiveQuery) -> Oracle {
    let scope = s.pedagogy_identity().unwrap().1;
    let mut todo = VecDeque::from([q.goal.from]);
    let mut seen = BTreeSet::from([q.goal.from]);
    if let Some(seed) = q.conceptual_seed {
        if seen.insert(seed) {
            todo.push_back(seed)
        }
    }
    let mut result = Oracle {
        required: BTreeSet::new(),
        closed: BTreeSet::new(),
        row_visits: 0,
        conflicts: BTreeSet::new(),
    };
    let mut outcomes = std::collections::BTreeMap::<(i32, u32), BTreeSet<i32>>::new();
    while let Some(state) = todo.pop_front() {
        if state == q.goal.to {
            continue;
        }
        result.closed.insert(state);
        for row in rows {
            result.row_visits += 1;
            let t = &row.trial;
            if t.world != q.goal.world
                || t.revision != q.goal.revision
                || t.context != q.goal.context
                || t.source != q.goal.source
                || t.from != state
                || row.after.event.logical_time() > s.logical_time()
                || !scope.validity.contains(row.before.event.logical_time())
                || !scope.validity.contains(row.after.event.logical_time())
            {
                continue;
            }
            result.required.insert((row.episode, t.id));
            outcomes.entry((t.from, t.action)).or_default().insert(t.to);
            if seen.insert(t.to) {
                todo.push_back(t.to)
            }
        }
    }
    result.conflicts = outcomes
        .into_iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(k, _)| k)
        .collect();
    result
}
fn keys(m: &RetrievedMemory) -> BTreeSet<(u32, u32)> {
    m.entries()
        .iter()
        .flat_map(|e| e.origins.iter().map(move |id| (id.get(), e.trial.id)))
        .collect()
}
fn tickets() -> Vec<Intervention> {
    vec![
        request(100, 7, 32),
        request(101, 9, 50),
        request(102, 11, 70),
    ]
}
fn transition(g: PlanningGoal, action: u32) -> TransitionQuery {
    TransitionQuery {
        world: g.world,
        revision: g.revision,
        context: g.context,
        source: g.source,
        from: g.from,
        action,
    }
}
#[test]
fn selective_recovery_matches_exhaustive_relevant_evidence_and_records_costs() {
    let fixture_started = std::time::Instant::now();
    let (catalog, s, rows) = corpus(false, false);
    let setup = fixture_started.elapsed().as_nanos();
    let mut r = runtime();
    let q = selector(task(999, 40));
    let t = std::time::Instant::now();
    let selected = r
        .select_evidence(&catalog, &s, q, RetrievalBudget::default())
        .unwrap();
    let selection_ns = t.elapsed().as_nanos();
    let t = std::time::Instant::now();
    let reference = exhaustive(&rows, &s, q);
    let oracle_ns = t.elapsed().as_nanos();
    assert!(selected.coverage().complete_for_goal);
    assert_eq!(catalog.cost().episodes, 40);
    assert_eq!(catalog.cost().segments, 10);
    assert!(reference.required.is_subset(&keys(selected.memory())));
    assert_eq!(
        selected
            .coverage()
            .closed_states
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        reference.closed
    );
    assert_eq!(selected.memory().entries().len(), 3);
    assert_eq!(reference.required.len(), 2);
    assert!(
        selected.coverage().index_scans + selected.coverage().materialization_scans
            < reference.row_visits
    );
    println!("SELECTIVE_METRICS episodes={} rows={} required={} recovered={} index_scans={} materialization_scans={} exhaustive_row_visits={} index_build_ns={} fixture_setup_ns={} selection_ns={} oracle_ns={}",
        catalog.cost().episodes,rows.len(),reference.required.len(),keys(selected.memory()).len(),selected.coverage().index_scans,
        selected.coverage().materialization_scans,reference.row_visits,catalog.cost().construction_nanos,setup,selection_ns,oracle_ns);
}
#[test]
fn dispersed_opposition_cannot_be_pruned_as_irrelevant_to_success() {
    let (catalog, s, rows) = corpus(true, false);
    let mut r = runtime();
    let q = selector(task(20, 40));
    let selected = r
        .select_evidence(&catalog, &s, q, RetrievalBudget::default())
        .unwrap();
    let oracle = exhaustive(&rows, &s, q);
    assert!(selected.coverage().complete_for_goal);
    assert!(oracle.required.is_subset(&keys(selected.memory())));
    assert!(oracle.conflicts.contains(&(20, 9)));
    let p = r
        .predict_retrieved(&s, transition(task(20, 40), 9), selected.memory())
        .unwrap();
    assert_eq!(p.status, PredictionStatus::Conflicting);
    assert_eq!(
        p.outcomes.iter().map(|o| o.to).collect::<Vec<_>>(),
        vec![30, 77]
    );
    assert_eq!(
        r.plan_retrieved(
            &s,
            task(20, 40),
            PlanningBudget::default(),
            selected.memory()
        )
        .unwrap()
        .reason,
        PlanReason::ContradictoryEvidence
    );
}
#[test]
fn partial_search_is_not_a_proof_and_can_resume_within_the_same_budget() {
    let (catalog, s, rows) = corpus(true, false);
    let mut r = runtime();
    let q = selector(task(999, 40));
    let mut selected = r
        .select_evidence(
            &catalog,
            &s,
            q,
            RetrievalBudget {
                scans: 4,
                entries: 8,
            },
        )
        .unwrap();
    assert!(!selected.coverage().complete_for_goal);
    assert!(selected.coverage().pending_postings > 0);
    assert!(r
        .plan_retrieved(&s, q.goal, PlanningBudget::default(), selected.memory())
        .is_err());
    let consumed = r.attention_scans();
    r.resume_selection(&s, &mut selected, RetrievalBudget::default())
        .unwrap();
    assert!(selected.coverage().complete_for_goal);
    assert!(r.attention_scans() > consumed);
    assert!(r.attention_scans() <= 32);
    assert!(exhaustive(&rows, &s, q)
        .required
        .is_subset(&keys(selected.memory())));
}
#[test]
fn goal_selective_coverage_is_not_global_coverage() {
    let (catalog, s, _) = corpus(false, false);
    let mut r = runtime();
    let q = selector(task(999, 40));
    let selected = r
        .select_evidence(&catalog, &s, q, RetrievalBudget::default())
        .unwrap();
    assert!(selected.coverage().complete_for_goal);
    assert!(selected.coverage().considered_trials < selected.coverage().context_candidates);
    assert!(r
        .predict_retrieved(&s, transition(task(10002, 10003), 3), selected.memory())
        .is_err());
    assert!(r
        .plan_retrieved(
            &s,
            task(999, 77),
            PlanningBudget::default(),
            selected.memory()
        )
        .is_err());
    assert!(r
        .predict_retrieved(&s, transition(task(40, 40), 3), selected.memory())
        .is_err());
}
#[test]
fn selective_recovery_drives_novel_goal_without_future_environment_answers() {
    let (catalog, s, _) = corpus(false, false);
    let q = selector(task(888, 40));
    let mut r = runtime();
    let selected = r
        .select_evidence(&catalog, &s, q, RetrievalBudget::default())
        .unwrap();
    let mut w = world(888, &[(888, 7, 20), (20, 9, 30), (30, 11, 40)]);
    let out = r
        .pursue_with_consequences(
            selected.memory(),
            &s,
            &mut w,
            PursuitGoal {
                task: q.goal,
                max_actions: 3,
                ..goal(888)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets(),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(r.experiences().len(), 7);
    assert_eq!(out.goal.task, q.goal);
}
#[test]
fn unrelated_version_or_absent_experience_does_not_solve_the_goal() {
    for stale in [false, true] {
        let (catalog, s, rows) = corpus(false, stale);
        let mut r = runtime();
        let mut q = selector(task(999, 40));
        if !stale {
            q.goal.world = 700
        }
        let selected = r
            .select_evidence(&catalog, &s, q, RetrievalBudget::default())
            .unwrap();
        let oracle = exhaustive(&rows, &s, q);
        assert!(selected.coverage().complete_for_goal);
        assert!(oracle.required.is_subset(&keys(selected.memory())));
        let result = r
            .plan_retrieved(&s, q.goal, PlanningBudget::default(), selected.memory())
            .unwrap();
        assert_eq!(result.reason, PlanReason::InsufficientEvidence);
        assert!(result.question.is_some());
    }
}
#[test]
fn old_model_versions_remain_traceable_and_stale_sessions_require_new_search() {
    let (catalog, mut s, _) = corpus(false, false);
    s.pedagogy_present(neurons::curriculum::Lesson {
        id: EngramId::new(502).unwrap(),
        receipt: 902,
        exemplar: n(902),
        attributes: Attributes::new([0, 0, 0, 0, 0, 0]).unwrap(),
        label: neurons::cognition::concepts::Label::Negative,
        source: source(),
        evidence: EvidenceId::new(4999).unwrap(),
    })
    .unwrap();
    s.pedagogy_admit(EngramId::new(502).unwrap()).unwrap();
    let mut r = runtime();
    let q = selector(task(999, 40));
    let mut selected = r
        .select_evidence(
            &catalog,
            &s,
            q,
            RetrievalBudget {
                scans: 1,
                entries: 8,
            },
        )
        .unwrap();
    s.execute("agora 120").unwrap();
    assert!(r
        .resume_selection(&s, &mut selected, RetrievalBudget::default())
        .is_err());
    let mut r = runtime();
    let selected = r
        .select_evidence(&catalog, &s, q, RetrievalBudget::default())
        .unwrap();
    assert!(selected.coverage().complete_for_goal);
    assert_eq!(selected.memory().versions()[0].version.number, 1);
    assert_eq!(
        selected.memory().versions()[0].relation,
        VersionRelation::Ancestor
    );
}
#[test]
fn limited_attention_does_not_turn_partial_coverage_into_success() {
    let (catalog, s, _) = corpus(false, false);
    let mut r = runtime_with(
        access(),
        Limits {
            attention_scans: 5,
            ..Default::default()
        },
    );
    let q = selector(task(999, 40));
    let mut selected = r
        .select_evidence(&catalog, &s, q, RetrievalBudget::default())
        .unwrap();
    assert!(!selected.coverage().complete_for_goal);
    let count = r.attention_scans();
    r.resume_selection(&s, &mut selected, RetrievalBudget::default())
        .unwrap();
    assert_eq!(r.attention_scans(), count);
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30), (30, 11, 40)]);
    assert!(r
        .pursue_with_consequences(
            selected.memory(),
            &s,
            &mut w,
            PursuitGoal {
                task: q.goal,
                max_actions: 3,
                ..goal(999)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets()
        )
        .is_err());
    assert_eq!(w.action_count(), 0);
}
#[test]
fn changing_owner_does_not_reset_the_search_inspection_cap() {
    let s = session();
    let mut archive = EpisodeArchive::new();
    for ep in 0..4u32 {
        let start = ep as i32 * 4;
        let rows: Vec<_> = (0..4).map(|i| (start + i, 3, start + i + 1)).collect();
        let mut w = world(start, &rows);
        let mut r = runtime();
        for j in 0..4u32 {
            let mut req = request(j + 1, 3, j as u64 * 10);
            req.permit.id = 2000 + ep * 4 + j;
            req.before_evidence = EvidenceId::new(8000 + ep * 8 + j * 2).unwrap();
            req.after_evidence = EvidenceId::new(8001 + ep * 8 + j * 2).unwrap();
            r.intervene(&mut w, &req).unwrap();
        }
        archive.insert(eid(ep + 1), &r, &s).unwrap();
    }
    let mut catalog = EpisodeCatalog::new();
    catalog.insert(1, archive).unwrap();
    let mut r = runtime();
    let mut selected = r
        .select_evidence(
            &catalog,
            &s,
            SelectiveQuery {
                goal: task(0, 16),
                conceptual_seed: None,
            },
            RetrievalBudget {
                scans: 1,
                entries: 8,
            },
        )
        .unwrap();
    for _ in 0..40 {
        let mut next = runtime();
        next.resume_selection(
            &s,
            &mut selected,
            RetrievalBudget {
                scans: 1,
                entries: 8,
            },
        )
        .unwrap();
    }
    assert_eq!(selected.coverage().index_scans, 32);
    assert!(!selected.coverage().complete_for_goal);
    assert_eq!(selected.memory().status(), RetrievalStatus::ResourceLimited);
}
#[test]
fn permissions_and_original_archive_limits_remain_in_force() {
    let (catalog, s, _) = corpus(false, false);
    let mut a = access();
    a.read_knowledge = false;
    let mut r = runtime_with(a, Limits::default());
    assert!(r
        .select_evidence(
            &catalog,
            &s,
            selector(task(999, 40)),
            RetrievalBudget::default()
        )
        .is_err());
    assert_eq!(r.attention_scans(), 0);
    let mut r = runtime();
    assert!(r
        .select_evidence(
            &catalog,
            &s,
            selector(task(999, 40)),
            RetrievalBudget {
                scans: 33,
                entries: 8
            }
        )
        .is_err());
    assert!(r
        .select_evidence(
            &catalog,
            &s,
            selector(task(999, 40)),
            RetrievalBudget {
                scans: 32,
                entries: 9
            }
        )
        .is_err());
    let (h, s) = history();
    let mut archive = EpisodeArchive::new();
    for id in 1..=4 {
        archive.insert(eid(id), &h, &s).unwrap();
    }
    assert!(archive.insert(eid(5), &h, &s).is_err());
}
#[test]
fn expansion_covers_an_observed_new_state_without_restoring_budget() {
    let (mut catalog, s, _) = corpus(false, false);
    let mut archive = EpisodeArchive::new();
    let mut r0 = runtime();
    let mut w = world(77, &[(77, 11, 40)]);
    let mut req = request(1, 11, 20);
    req.permit.id = 50001;
    req.before_evidence = EvidenceId::new(50002).unwrap();
    req.after_evidence = EvidenceId::new(50003).unwrap();
    r0.intervene(&mut w, &req).unwrap();
    archive.insert(eid(50), &r0, &s).unwrap();
    catalog.insert(11, archive).unwrap();
    let mut r = runtime();
    let q = selector(task(999, 40));
    let mut selected = r
        .select_evidence(&catalog, &s, q, RetrievalBudget::default())
        .unwrap();
    assert!(r
        .plan_retrieved(
            &s,
            task(77, 40),
            PlanningBudget::default(),
            selected.memory()
        )
        .is_err());
    let spent = r.attention_scans();
    r.extend_selection(&s, &mut selected, 77, RetrievalBudget::default())
        .unwrap();
    assert!(selected.coverage().complete_for_goal);
    assert!(r.attention_scans() > spent);
    assert!(r.attention_scans() <= 32);
    assert!(r
        .plan_retrieved(
            &s,
            task(77, 40),
            PlanningBudget::default(),
            selected.memory()
        )
        .unwrap()
        .plan
        .is_some());
}

#[test]
fn omitted_conceptual_seed_cannot_turn_collateral_records_into_a_plan() {
    let (catalog, s, _) = corpus(false, false);
    let mut r = runtime();
    let g = task(999, 30);
    let selected = r
        .select_evidence(
            &catalog,
            &s,
            SelectiveQuery {
                goal: g,
                conceptual_seed: None,
            },
            RetrievalBudget::default(),
        )
        .unwrap();
    assert!(selected.coverage().complete_for_goal);
    // O snapshot do modelo contém 20->30, mas o índice não conferiu a cobertura de 20.
    assert!(selected
        .memory()
        .entries()
        .iter()
        .any(|e| e.trial.from == 20));
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    let out = r
        .pursue_with_consequences(
            selected.memory(),
            &s,
            &mut w,
            PursuitGoal {
                task: g,
                max_actions: 2,
                ..goal(999)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets(),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::InsufficientEvidence);
    assert!(out.failure.is_some());
    assert_eq!(w.action_count(), 0);
}
#[test]
fn discovered_opposition_is_reported_by_the_existing_metacognitive_pipeline() {
    let (catalog, s, _) = corpus(true, false);
    let mut r = runtime();
    let g = task(999, 40);
    let selected = r
        .select_evidence(&catalog, &s, selector(g), RetrievalBudget::default())
        .unwrap();
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30), (30, 11, 40)]);
    let out = r
        .pursue_with_consequences(
            selected.memory(),
            &s,
            &mut w,
            PursuitGoal {
                task: g,
                max_actions: 3,
                ..goal(999)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets(),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::ConflictingEvidence);
    assert_eq!(w.action_count(), 0);
}

#[test]
fn incompatible_model_in_a_distant_segment_is_not_ignored_by_goal_ranking() {
    let (mut catalog, s, _) = corpus(false, false);
    let (h, _) = history();
    let mut fork = session();
    for c in [
        "exemplo 601 950 61 000001 + 7 9999 observacao",
        "interpretar-exemplo",
        "avaliar-exemplo",
        "admitir-exemplo 601",
        "revisar",
    ] {
        fork.execute(c).unwrap();
    }
    let mut r0 = runtime();
    r0.inherit_consequence_contract(&h).unwrap();
    let mut w = world(4000, &[(4000, 3, 4001)]);
    let mut req = request(1, 3, 20);
    req.permit.id = 50001;
    req.before_evidence = EvidenceId::new(50002).unwrap();
    req.after_evidence = EvidenceId::new(50003).unwrap();
    r0.intervene(&mut w, &req).unwrap();
    let mut archive = EpisodeArchive::new();
    archive.insert(eid(50), &r0, &fork).unwrap();
    catalog.insert(11, archive).unwrap();
    let mut r = runtime();
    let q = selector(task(999, 40));
    let selected = r
        .select_evidence(&catalog, &s, q, RetrievalBudget::default())
        .unwrap();
    assert_eq!(
        selected.memory().status(),
        RetrievalStatus::ConflictingVersions
    );
    assert!(selected
        .memory()
        .versions()
        .iter()
        .any(|v| v.relation == VersionRelation::Incompatible && v.episode == eid(50)));
    assert!(r
        .plan_retrieved(&s, q.goal, PlanningBudget::default(), selected.memory())
        .is_err());
}
#[test]
fn catalogue_persists_as_existing_segments_and_rebuilds_after_process_restart() {
    let (catalog, s, _) = corpus(false, false);
    let dir = std::env::temp_dir().join(format!("neurons-selective-{}", std::process::id()));
    assert!(!dir.exists());
    std::fs::create_dir(&dir).unwrap();
    for id in catalog.segment_ids() {
        catalog
            .segment(id)
            .unwrap()
            .save(&dir.join(format!("{id}.bin")))
            .unwrap();
    }
    std::fs::write(dir.join("session.bin"), s.encode()).unwrap();
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "selective_restore_worker", "--nocapture"])
        .env("NEURONS_SELECTIVE_RESTORE", &dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    for id in catalog.segment_ids() {
        assert_eq!(
            std::fs::read(dir.join(format!("{id}.bin"))).unwrap(),
            catalog.segment(id).unwrap().encode()
        );
        std::fs::remove_file(dir.join(format!("{id}.bin"))).unwrap();
    }
    std::fs::remove_file(dir.join("session.bin")).unwrap();
    std::fs::remove_dir(dir).unwrap();
}
#[test]
fn selective_restore_worker() {
    let Some(dir) = std::env::var_os("NEURONS_SELECTIVE_RESTORE") else {
        return;
    };
    let dir = std::path::PathBuf::from(dir);
    let s = Session::decode(&std::fs::read(dir.join("session.bin")).unwrap()).unwrap();
    let mut catalog = EpisodeCatalog::new();
    for id in 1..=10 {
        let ports: Vec<_> = ((id - 1) * 4 + 1..=id * 4)
            .map(|ep| (eid(ep), sensors()))
            .collect();
        catalog
            .insert(
                id,
                EpisodeArchive::load(&dir.join(format!("{id}.bin")), &ports).unwrap(),
            )
            .unwrap();
    }
    let mut denied = runtime_with(Access::default(), Limits::default());
    let q = selector(task(777, 40));
    assert!(denied
        .select_evidence(&catalog, &s, q, RetrievalBudget::default())
        .is_err());
    let mut r = runtime();
    let selected = r
        .select_evidence(&catalog, &s, q, RetrievalBudget::default())
        .unwrap();
    let mut w = world(777, &[(777, 7, 20), (20, 9, 30), (30, 11, 40)]);
    let out = r
        .pursue_with_consequences(
            selected.memory(),
            &s,
            &mut w,
            PursuitGoal {
                task: q.goal,
                max_actions: 3,
                ..goal(777)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &tickets(),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(r.experiences().len(), 7);
}
fn exhaustive_paths(rows: &[Row], s: &Session, g: PlanningGoal) -> Vec<Vec<(u32, u32)>> {
    fn walk(
        rows: &[Row],
        s: &Session,
        g: PlanningGoal,
        state: i32,
        path: Vec<(u32, u32)>,
        seen: Vec<i32>,
        out: &mut Vec<Vec<(u32, u32)>>,
    ) {
        if state == g.to {
            out.push(path);
            return;
        }
        if path.len() == 4 {
            return;
        }
        let valid = |r: &Row| {
            r.trial.world == g.world
                && r.trial.revision == g.revision
                && r.trial.context == g.context
                && r.trial.source == g.source
                && r.after.event.logical_time() <= s.logical_time()
                && s.pedagogy_identity()
                    .unwrap()
                    .1
                    .validity
                    .contains(r.before.event.logical_time())
                && s.pedagogy_identity()
                    .unwrap()
                    .1
                    .validity
                    .contains(r.after.event.logical_time())
        };
        for r in rows {
            if !valid(r) || r.trial.from != state || seen.contains(&r.trial.to) {
                continue;
            }
            if rows.iter().any(|other| {
                valid(other)
                    && other.trial.from == state
                    && other.trial.action == r.trial.action
                    && other.trial.to != r.trial.to
            }) {
                continue;
            }
            let mut path = path.clone();
            path.push((r.episode, r.trial.id));
            let mut seen = seen.clone();
            seen.push(r.trial.to);
            walk(rows, s, g, r.trial.to, path, seen, out);
        }
    }
    let mut out = vec![];
    walk(rows, s, g, g.from, vec![], vec![g.from], &mut out);
    out
}
#[test]
fn existing_planner_agrees_with_independent_exhaustive_path_enumeration() {
    for opposition in [false, true] {
        let (catalog, s, rows) = corpus(opposition, false);
        let mut r = runtime();
        let q = selector(task(999, 40));
        let selected = r
            .select_evidence(&catalog, &s, q, RetrievalBudget::default())
            .unwrap();
        let g = task(20, 40);
        let reference = exhaustive_paths(&rows, &s, g);
        let result = r
            .plan_retrieved(&s, g, PlanningBudget::default(), selected.memory())
            .unwrap();
        assert_eq!(result.plan.is_some(), !reference.is_empty());
        if let Some(plan) = result.plan {
            let proof: Vec<_> = plan
                .steps()
                .iter()
                .map(|t| {
                    let e = selected.memory().resolve_reference(t.id).unwrap();
                    (e.origins[0].get(), e.trial.id)
                })
                .collect();
            assert!(reference.contains(&proof));
        }
    }
}
