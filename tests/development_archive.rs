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
fn eid(v: u32) -> EpisodeId {
    EpisodeId::new(v).unwrap()
}
fn archive_pair() -> (EpisodeArchive, Session) {
    let (h, s) = history();
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &s).unwrap();
    let mut second = runtime();
    let mut w = world(30, &[(30, 11, 40)]);
    second.intervene(&mut w, &request(3, 11, 20)).unwrap();
    a.insert(eid(2), &second, &s).unwrap();
    (a, s)
}
fn task(from: i32, to: i32) -> PlanningGoal {
    PlanningGoal {
        to,
        ..goal(from).task
    }
}
fn query(g: PlanningGoal, action: u32) -> TransitionQuery {
    TransitionQuery {
        world: g.world,
        revision: g.revision,
        context: g.context,
        source: g.source,
        from: g.from,
        action,
    }
}
fn new_tickets() -> Vec<Intervention> {
    vec![request(10, 7, 32), request(11, 9, 50), request(12, 11, 70)]
}
fn retrieve(r: &mut Runtime, a: &EpisodeArchive, s: &Session, g: PlanningGoal) -> RetrievedMemory {
    r.retrieve_context(a, s, g, RetrievalBudget::default())
        .unwrap()
}
#[test]
fn multiple_episodes_enable_a_novel_three_action_goal_with_provenance() {
    let (a, s) = archive_pair();
    let bytes = a.encode();
    let mut r = runtime();
    let g = task(999, 40);
    let memory = retrieve(&mut r, &a, &s, g);
    assert_eq!(memory.status(), RetrievalStatus::Complete);
    assert_eq!(memory.entries().len(), 3);
    assert_eq!(memory.scans(), 5);
    assert_eq!(memory.entries()[2].origins, vec![eid(2)]);
    assert_eq!(memory.entries()[2].after.event.payload(), 40);
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30), (30, 11, 40)]);
    let objective = PursuitGoal {
        task: g,
        max_actions: 3,
        ..goal(999)
    };
    let out = r
        .pursue_with_consequences(
            &memory,
            &s,
            &mut w,
            objective,
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &new_tickets(),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(out.goal, objective);
    assert_eq!(r.experiences().len(), 7);
    assert_eq!(
        r.action_trials()
            .iter()
            .map(|t| t.action)
            .collect::<Vec<_>>(),
        vec![7, 9, 11]
    );
    assert_eq!(a.encode(), bytes);
    assert!(r.planning_scans() <= 32);
    assert!(r.attention_scans() <= 32);
}
#[test]
fn without_relevant_experiences_the_same_goal_is_not_solved() {
    let (h, s) = history();
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &s).unwrap();
    let mut r = runtime();
    let g = task(999, 40);
    let memory = retrieve(&mut r, &a, &s, g);
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30), (30, 11, 40)]);
    let out = r
        .pursue_with_consequences(
            &memory,
            &s,
            &mut w,
            PursuitGoal {
                task: g,
                max_actions: 3,
                ..goal(999)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &new_tickets(),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::InsufficientEvidence);
    assert_eq!(w.action_count(), 0);
    assert!(out.prefix_alternative.unwrap().question.is_some());
}
#[test]
fn conflicting_evidence_is_preserved_across_episodes_without_majority_vote() {
    let (h, s) = history();
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &s).unwrap();
    a.insert(eid(2), &h, &s).unwrap(); // Mesma experiência, duas origens, não duas provas.
    let mut contrary = runtime();
    let mut w = world(20, &[(20, 9, 77)]);
    contrary.intervene(&mut w, &request(3, 9, 20)).unwrap();
    a.insert(eid(3), &contrary, &s).unwrap();
    let mut r = runtime();
    let g = task(20, 30);
    let m = retrieve(&mut r, &a, &s, g);
    assert_eq!(m.entries().len(), 3);
    assert_eq!(m.entries()[0].origins, vec![eid(1), eid(2)]);
    let p = r.predict_retrieved(&s, query(g, 9), &m).unwrap();
    assert_eq!(p.status, PredictionStatus::Conflicting);
    assert_eq!(
        p.outcomes.iter().map(|o| o.to).collect::<Vec<_>>(),
        vec![30, 77]
    );
    let p = r
        .plan_retrieved(&s, g, PlanningBudget::default(), &m)
        .unwrap();
    assert_eq!(p.reason, PlanReason::ContradictoryEvidence);
    assert!(p.plan.is_none());
}
#[test]
fn resource_limited_lookup_cannot_hide_later_opposition() {
    let (a, s) = archive_pair();
    for budget in [
        RetrievalBudget {
            scans: 1,
            entries: 8,
        },
        RetrievalBudget {
            scans: 32,
            entries: 1,
        },
    ] {
        let mut r = runtime();
        let g = task(20, 40);
        let m = r.retrieve_context(&a, &s, g, budget).unwrap();
        assert_eq!(m.status(), RetrievalStatus::ResourceLimited);
        assert!(r
            .plan_retrieved(&s, g, PlanningBudget::default(), &m)
            .is_err());
    }
}
#[test]
fn retrieval_consumes_attention_shared_with_action_selection() {
    let (a, s) = archive_pair();
    let mut r = runtime_with(
        access(),
        Limits {
            attention_scans: 4,
            ..Default::default()
        },
    );
    let g = task(20, 40);
    let m = retrieve(&mut r, &a, &s, g);
    assert_eq!(m.status(), RetrievalStatus::ResourceLimited);
    assert_eq!(r.attention_scans(), 4);
    let m = retrieve(&mut r, &a, &s, g);
    assert_eq!(m.scans(), 0);
    assert_eq!(m.status(), RetrievalStatus::ResourceLimited);
}
#[test]
fn source_revision_and_time_are_not_interchangeable() {
    let (a, s) = archive_pair();
    for kind in 0..3 {
        let mut r = runtime();
        let mut g = task(20, 40);
        match kind {
            0 => g.revision = 2,
            1 => g.source = SourceId::new(9).unwrap(),
            _ => g.world = 700,
        }
        let m = retrieve(&mut r, &a, &s, g);
        assert!(m.entries().is_empty());
        assert_eq!(m.omitted(), 3);
        let p = r
            .plan_retrieved(&s, g, PlanningBudget::default(), &m)
            .unwrap();
        assert_eq!(p.reason, PlanReason::InsufficientEvidence);
    }
    let (h, s) = history();
    let mut a = EpisodeArchive::new();
    let mut late = runtime();
    let mut w = world(30, &[(30, 11, 40)]);
    late.intervene(&mut w, &request(3, 11, 200)).unwrap();
    a.insert(eid(1), &h, &s).unwrap();
    a.insert(eid(2), &late, &s).unwrap();
    let mut r = runtime();
    let m = retrieve(&mut r, &a, &s, task(20, 40));
    assert_eq!(m.entries().len(), 2);
    assert_eq!(m.omitted(), 1);
}
#[test]
fn changed_session_invalidates_a_previously_retrieved_view() {
    let (a, mut s) = archive_pair();
    let mut r = runtime();
    let g = task(20, 40);
    let m = retrieve(&mut r, &a, &s, g);
    s.execute("agora 106").unwrap();
    assert!(r
        .plan_retrieved(&s, g, PlanningBudget::default(), &m)
        .is_err());
    let refreshed = retrieve(&mut r, &a, &s, g);
    assert!(r
        .plan_retrieved(&s, g, PlanningBudget::default(), &refreshed)
        .unwrap()
        .plan
        .is_some());
}
#[test]
fn earlier_concept_versions_are_references_not_silently_merged_models() {
    let (a, mut s) = archive_pair();
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
    let before = s.encode();
    let mut r = runtime();
    let m = retrieve(&mut r, &a, &s, task(20, 40));
    assert_eq!(m.status(), RetrievalStatus::Complete);
    assert_eq!(m.versions()[0].relation, VersionRelation::Ancestor);
    assert_eq!(m.versions()[0].version.number, 1);
    assert_eq!(s.pedagogy_version().unwrap().number, 2);
    assert_eq!(s.encode(), before);
}
#[test]
fn incompatible_concept_versions_block_automatic_use() {
    let (h, s) = history();
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &s).unwrap();
    let mut alternate = session();
    for c in [
        "exemplo 601 950 61 000001 + 7 9999 observacao",
        "interpretar-exemplo",
        "avaliar-exemplo",
        "admitir-exemplo 601",
        "revisar",
    ] {
        alternate.execute(c).unwrap();
    }
    let mut other = runtime();
    other.inherit_consequence_contract(&h).unwrap();
    let mut w = world(30, &[(30, 11, 40)]);
    other.intervene(&mut w, &request(3, 11, 20)).unwrap();
    a.insert(eid(2), &other, &alternate).unwrap();
    let mut r = runtime();
    let g = task(20, 40);
    let m = retrieve(&mut r, &a, &s, g);
    assert_eq!(m.status(), RetrievalStatus::ConflictingVersions);
    assert!(m
        .versions()
        .iter()
        .any(|v| v.relation == VersionRelation::Incompatible));
    assert!(r
        .plan_retrieved(&s, g, PlanningBudget::default(), &m)
        .is_err());
}
#[test]
fn local_identity_collisions_are_reported_instead_of_renumbering_evidence() {
    let (h, s) = history();
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &s).unwrap();
    let mut other = runtime();
    let mut w = world(20, &[(20, 9, 77)]);
    other.intervene(&mut w, &request(2, 9, 20)).unwrap();
    a.insert(eid(2), &other, &s).unwrap();
    let mut r = runtime();
    let g = task(20, 30);
    let m = retrieve(&mut r, &a, &s, g);
    assert_eq!(m.status(), RetrievalStatus::AmbiguousIdentity);
    assert!(r.predict_retrieved(&s, query(g, 9), &m).is_err());
}
#[test]
fn read_permission_is_required_and_irrelevant_grants_stay_consumed() {
    let (a, s) = archive_pair();
    let mut denied = access();
    denied.read_knowledge = false;
    let mut r = runtime_with(denied, Limits::default());
    assert!(r
        .retrieve_context(&a, &s, task(20, 40), RetrievalBudget::default())
        .is_err());
    assert_eq!(r.attention_scans(), 0);
    let mut r = runtime();
    let g = task(999, 40);
    let m = retrieve(&mut r, &a, &s, g);
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30), (30, 11, 40)]);
    let mut reused = new_tickets();
    reused[0].permit.id = 1001;
    assert!(r
        .pursue_with_consequences(
            &m,
            &s,
            &mut w,
            PursuitGoal {
                task: g,
                max_actions: 3,
                ..goal(999)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &reused
        )
        .is_err());
    assert_eq!(w.action_count(), 0);
}
#[test]
fn archive_roundtrip_is_canonical_bounded_and_corruption_is_rejected() {
    let (a, s) = archive_pair();
    let bytes = a.encode();
    let ports = vec![(eid(1), sensors()), (eid(2), sensors())];
    let restored = EpisodeArchive::decode(&bytes, &ports).unwrap();
    assert_eq!(restored.encode(), bytes);
    for pos in [0, 8, 24, bytes.len() - 1] {
        let mut changed = bytes.clone();
        changed[pos] ^= 1;
        assert!(EpisodeArchive::decode(&changed, &ports).is_err());
    }
    for len in [0, 8, bytes.len() - 1] {
        assert!(EpisodeArchive::decode(&bytes[..len], &ports).is_err());
    }
    assert!(EpisodeArchive::decode(&bytes, &[]).is_err());
    let (h, _) = history();
    let mut full = EpisodeArchive::new();
    for id in 1..=4 {
        full.insert(eid(id), &h, &s).unwrap();
    }
    assert!(full.insert(eid(5), &h, &s).is_err());
    let old = full.encode();
    full.insert(eid(1), &h, &s).unwrap();
    assert_eq!(full.encode(), old);
    let mut changed = runtime();
    let mut w = world(10, &[(10, 7, 99)]);
    changed.intervene(&mut w, &request(30, 7, 30)).unwrap();
    assert!(full.insert(eid(1), &changed, &s).is_err());
    assert_eq!(full.encode(), old);
}
#[test]
fn recovery_also_feeds_supervised_acquisition_and_replanning() {
    let mut a = EpisodeArchive::new();
    let mut s = session();
    let mut tail = runtime();
    let mut w = world(20, &[(20, 9, 30)]);
    tail.intervene(&mut w, &request(2, 9, 10)).unwrap();
    a.insert(eid(1), &tail, &s).unwrap();
    let mut last = runtime();
    let mut w = world(30, &[(30, 11, 40)]);
    last.intervene(&mut w, &request(3, 11, 20)).unwrap();
    a.insert(eid(2), &last, &s).unwrap();
    let mut r = runtime();
    let g = task(999, 40);
    let m = retrieve(&mut r, &a, &s, g);
    let c = contract(&s);
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30), (30, 11, 40)]);
    let grant = PredictiveRevision {
        record: EngramId::new(501).unwrap(),
        receipt: 901,
        assessment: PedagogyAssessment::new(
            &[Attributes::new([1, 0, 1, 0, 1, 0]).unwrap().into()],
            &[Attributes::new([0, 1, 0, 1, 0, 1]).unwrap().into()],
        )
        .unwrap(),
        permissions: PedagogyPermissions {
            evaluation: Some(c.scope),
            presentation: Some(c.scope),
            admissions: vec![EngramId::new(501).unwrap()],
        },
    };
    let out = r
        .pursue_learning_goal(
            &m,
            &mut s,
            &mut w,
            PursuitGoal {
                task: g,
                max_actions: 3,
                ..goal(999)
            },
            c,
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &new_tickets(),
            Some(grant),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(out.revision.unwrap().pedagogy.final_version.number, 1);
    assert!(r
        .plan_retrieved(&s, g, PlanningBudget::default(), &m)
        .is_err()); // Requer nova recuperação após revisão.
}
#[test]
fn archive_is_recovered_in_a_new_process_with_fresh_execution_authority() {
    let (a, s) = archive_pair();
    let dir = std::env::temp_dir().join(format!("neurons-archive-{}", std::process::id()));
    assert!(!dir.exists());
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(dir.join("archive.bin"), a.encode()).unwrap();
    std::fs::write(dir.join("session.bin"), s.encode()).unwrap();
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "archive_restore_worker", "--nocapture"])
        .env("NEURONS_ARCHIVE_RESTORE", &dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    std::fs::remove_file(dir.join("archive.bin")).unwrap();
    std::fs::remove_file(dir.join("session.bin")).unwrap();
    std::fs::remove_dir(dir).unwrap();
}
#[test]
fn archive_restore_worker() {
    let Some(dir) = std::env::var_os("NEURONS_ARCHIVE_RESTORE") else {
        return;
    };
    let dir = std::path::PathBuf::from(dir);
    let bytes = std::fs::read(dir.join("archive.bin")).unwrap();
    let s = Session::decode(&std::fs::read(dir.join("session.bin")).unwrap()).unwrap();
    let a = EpisodeArchive::decode(&bytes, &[(eid(1), sensors()), (eid(2), sensors())]).unwrap();
    let mut denied = runtime_with(Access::default(), Limits::default());
    assert!(denied
        .retrieve_context(&a, &s, task(888, 40), RetrievalBudget::default())
        .is_err());
    let mut r = runtime();
    let g = task(888, 40);
    let m = retrieve(&mut r, &a, &s, g);
    let mut w = world(888, &[(888, 7, 20), (20, 9, 30), (30, 11, 40)]);
    let out = r
        .pursue_with_consequences(
            &m,
            &s,
            &mut w,
            PursuitGoal {
                task: g,
                max_actions: 3,
                ..goal(888)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &new_tickets(),
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(a.encode(), bytes);
}

#[test]
fn identical_version_numbers_and_criteria_do_not_hide_changed_provenance() {
    let (h, s) = history();
    let mut fork = session();
    fork.pedagogy_present(neurons::curriculum::Lesson {
        id: EngramId::new(501).unwrap(),
        receipt: 901,
        exemplar: n(999),
        attributes: Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
        label: neurons::cognition::concepts::Label::Positive,
        source: source(),
        evidence: EvidenceId::new(4003).unwrap(),
    })
    .unwrap();
    fork.pedagogy_admit(EngramId::new(501).unwrap()).unwrap();
    assert_eq!(
        fork.pedagogy_version().unwrap(),
        s.pedagogy_version().unwrap()
    );
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &fork).unwrap();
    let mut r = runtime();
    let g = task(20, 30);
    let memory = retrieve(&mut r, &a, &s, g);
    assert_eq!(memory.status(), RetrievalStatus::ConflictingVersions);
    assert!(r
        .plan_retrieved(&s, g, PlanningBudget::default(), &memory)
        .is_err());
}
#[test]
fn irrelevant_episodes_cannot_restore_consumed_authority() {
    let (mut a, s) = archive_pair();
    let mut other = runtime();
    let mut w = World::new(700, 1, sensors()[0].clone(), 1, &[(1, 7, 2)]).unwrap();
    let mut req = request(90, 7, 20);
    req.permit.world = 700;
    other.intervene(&mut w, &req).unwrap();
    a.insert(eid(3), &other, &s).unwrap();
    let mut r = runtime();
    let g = task(999, 40);
    let memory = retrieve(&mut r, &a, &s, g);
    assert_eq!(memory.omitted(), 1);
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30), (30, 11, 40)]);
    let mut req = new_tickets();
    req[0].permit.id = 1090;
    assert!(r
        .pursue_with_consequences(
            &memory,
            &s,
            &mut w,
            PursuitGoal {
                task: g,
                max_actions: 3,
                ..goal(999)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &req
        )
        .is_err());
    assert_eq!(w.action_count(), 0);
}
#[test]
fn persistent_archive_does_not_overwrite_existing_files() {
    let (a, _s) = archive_pair();
    let path =
        std::env::temp_dir().join(format!("neurons-archive-save-{}.bin", std::process::id()));
    assert!(!path.exists());
    a.save(&path).unwrap();
    let bytes = std::fs::read(&path).unwrap();
    assert!(a.save(&path).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    let restored =
        EpisodeArchive::load(&path, &[(eid(1), sensors()), (eid(2), sensors())]).unwrap();
    assert_eq!(restored.encode(), bytes);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn retrieved_memory_drives_a_question_from_an_actual_remaining_gap() {
    let (archive, session) = archive_pair();
    let mut r = runtime();
    let goal = task(40, 99);
    let memory = retrieve(&mut r, &archive, &session, goal);
    let out = r
        .plan_retrieved(&session, goal, PlanningBudget::default(), &memory)
        .unwrap();
    assert_eq!(out.reason, PlanReason::InsufficientEvidence);
    let question = out.question.unwrap();
    assert_eq!(question.goal, goal);
    assert_eq!(question.observed_trials, 3);
    assert!(out.plan.is_none());
}
