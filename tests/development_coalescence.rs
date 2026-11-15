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

use std::collections::BTreeSet;
fn eid(v: u32) -> EpisodeId {
    EpisodeId::new(v).unwrap()
}
fn task(from: i32, to: i32) -> PlanningGoal {
    PlanningGoal {
        to,
        ..goal(from).task
    }
}
fn query(g: PlanningGoal) -> SelectiveQuery {
    SelectiveQuery {
        goal: g,
        conceptual_seed: Some(20),
    }
}
fn repeated(mode: SnapshotStorage, count: u32) -> (EpisodeCatalog, Session) {
    let (h, s) = history();
    let mut catalog = EpisodeCatalog::with_storage(mode);
    let mut archive = EpisodeArchive::new();
    let mut segment = 1;
    for id in 1..=count {
        archive.insert(eid(id), &h, &s).unwrap();
        if archive.len() == 4 {
            catalog
                .insert(segment, std::mem::take(&mut archive))
                .unwrap();
            segment += 1;
        }
    }
    if !archive.is_empty() {
        catalog.insert(segment, archive).unwrap();
    }
    (catalog, s)
}
fn select<'a>(
    r: &mut Runtime,
    c: &'a EpisodeCatalog,
    s: &Session,
    g: PlanningGoal,
) -> SelectiveSearch<'a> {
    r.select_evidence(c, s, query(g), RetrievalBudget::default())
        .unwrap()
}
fn outcomes(r: &mut Runtime, s: &Session, g: PlanningGoal, m: &RetrievedMemory) -> BTreeSet<i32> {
    r.predict_retrieved(
        s,
        TransitionQuery {
            world: g.world,
            revision: g.revision,
            context: g.context,
            source: g.source,
            from: g.from,
            action: 9,
        },
        m,
    )
    .unwrap()
    .outcomes
    .iter()
    .map(|o| o.to)
    .collect()
}
fn keys(m: &RetrievedMemory) -> BTreeSet<(u32, u32)> {
    m.entries()
        .iter()
        .flat_map(|e| e.origins.iter().map(move |id| (id.get(), e.trial.id)))
        .collect()
}
#[test]
fn exact_snapshots_share_representation_but_every_origin_remains_resolvable() {
    let (catalog, s) = repeated(SnapshotStorage::Coalesced, 64);
    let cost = catalog.cost();
    assert_eq!(cost.episodes, 64);
    assert_eq!(cost.retained_images, 1);
    assert_eq!(cost.unique_postings, 2);
    assert_eq!(cost.image_bytes, cost.retained_payload_bytes * 64);
    let mut r = runtime();
    let g = task(20, 30);
    let search = select(&mut r, &catalog, &s, g);
    assert!(search.coverage().complete_for_goal);
    let m = search.memory();
    assert_eq!(m.entries().len(), 2);
    for entry in m.entries() {
        assert_eq!(
            entry.origins.iter().map(|id| id.get()).collect::<Vec<_>>(),
            (1..=64).collect::<Vec<_>>()
        );
        assert_eq!(m.resolve_reference(entry.reference), Some(entry));
    }
    let p = m.evidence_profile();
    assert_eq!(
        (
            p.episode_origins,
            p.transition_records,
            p.transition_evidence_ids,
            p.sources,
            p.h23_support_references
        ),
        (64, 2, 4, 1, 1)
    );
    assert_eq!(outcomes(&mut r, &s, g, m), BTreeSet::from([30]));
    let raw = s.encode();
    assert!(r
        .plan_retrieved(&s, g, PlanningBudget::default(), m)
        .unwrap()
        .plan
        .is_some());
    assert_eq!(s.encode(), raw);
}
#[test]
fn finished_queries_have_equivalent_conclusions_and_full_origins_with_or_without_coalescence() {
    let mut results = vec![];
    let mut evidence = vec![];
    for mode in [SnapshotStorage::Coalesced, SnapshotStorage::Separate] {
        let (catalog, s) = repeated(mode, 4);
        let mut r = runtime();
        let g = task(20, 30);
        let search = select(&mut r, &catalog, &s, g);
        assert!(search.coverage().complete_for_goal);
        results.push(outcomes(&mut r, &s, g, search.memory()));
        evidence.push(keys(search.memory()));
        assert!(r
            .plan_retrieved(&s, g, PlanningBudget::default(), search.memory())
            .unwrap()
            .plan
            .is_some());
        assert_eq!(search.memory().evidence_profile().h23_support_references, 1);
        assert_eq!(
            catalog.cost().retained_images,
            if mode == SnapshotStorage::Coalesced {
                1
            } else {
                4
            }
        );
    }
    assert_eq!(results[0], results[1]);
    assert_eq!(evidence[0], evidence[1]);
    assert_eq!(evidence[0].len(), 8);
}
#[test]
fn coalescence_improves_usable_coverage_without_widening_any_query_budget() {
    for mode in [SnapshotStorage::Coalesced, SnapshotStorage::Separate] {
        let (catalog, s) = repeated(mode, 64);
        let mut r = runtime();
        let g = task(20, 30);
        let search = select(&mut r, &catalog, &s, g);
        assert!(r.attention_scans() <= 32);
        assert!(search.coverage().selected_episodes <= 4);
        if mode == SnapshotStorage::Coalesced {
            assert!(search.coverage().complete_for_goal);
            assert_eq!(search.coverage().selected_episodes, 1);
            assert_eq!(
                outcomes(&mut r, &s, g, search.memory()),
                BTreeSet::from([30])
            );
        } else {
            assert!(!search.coverage().complete_for_goal);
            assert_eq!(search.coverage().limit, Some(SearchLimit::SourceSlots));
            assert!(r
                .plan_retrieved(&s, g, PlanningBudget::default(), search.memory())
                .is_err());
        }
    }
}
fn independent_record(id: u32, from: i32, action: u32, to: i32, s: &Session) -> Runtime {
    let mut r = runtime();
    let mut w = world(from, &[(from, action, to)]);
    let mut req = request(1, action, 20);
    req.permit.id = 20_000 + id;
    req.before_evidence = EvidenceId::new(30_000 + id * 2).unwrap();
    req.after_evidence = EvidenceId::new(30_001 + id * 2).unwrap();
    r.intervene(&mut w, &req).unwrap();
    assert!(s.logical_time() > 20);
    r
}
#[test]
fn equal_values_from_repeated_source_with_new_evidence_ids_remain_distinct_observations() {
    let s = session();
    let mut a = EpisodeArchive::new();
    for id in 1..=2 {
        let r = independent_record(id, 20, 9, 30, &s);
        a.insert(eid(id), &r, &s).unwrap();
    }
    let mut c = EpisodeCatalog::new();
    c.insert(1, a).unwrap();
    assert_eq!(c.cost().retained_images, 2);
    let mut r = runtime();
    let g = task(20, 30);
    let selected = select(&mut r, &c, &s, g);
    let p = selected.memory().evidence_profile();
    assert_eq!(
        (
            p.episode_origins,
            p.transition_records,
            p.transition_evidence_ids,
            p.sources
        ),
        (2, 2, 4, 1)
    );
    assert_eq!(
        selected.memory().entries()[0].trial.id,
        selected.memory().entries()[1].trial.id
    );
    assert_ne!(
        selected.memory().entries()[0].reference,
        selected.memory().entries()[1].reference
    );
    let prediction = r
        .predict_retrieved(
            &s,
            TransitionQuery {
                world: g.world,
                revision: g.revision,
                context: g.context,
                source: g.source,
                from: 20,
                action: 9,
            },
            selected.memory(),
        )
        .unwrap();
    assert_eq!(prediction.outcomes.len(), 1);
    assert_eq!(prediction.outcomes[0].trials.len(), 2);
}
#[test]
fn one_opposing_observation_survives_many_positive_storage_replicas() {
    let (mut c, s) = repeated(SnapshotStorage::Coalesced, 32);
    let r0 = independent_record(100, 20, 9, 77, &s);
    let mut a = EpisodeArchive::new();
    a.insert(eid(100), &r0, &s).unwrap();
    c.insert(9, a).unwrap();
    let mut r = runtime();
    let g = task(20, 30);
    let selected = select(&mut r, &c, &s, g);
    assert!(selected.coverage().complete_for_goal);
    assert_eq!(
        outcomes(&mut r, &s, g, selected.memory()),
        BTreeSet::from([30, 77])
    );
    assert_eq!(
        r.plan_retrieved(&s, g, PlanningBudget::default(), selected.memory())
            .unwrap()
            .reason,
        PlanReason::ContradictoryEvidence
    );
    let positive = selected
        .memory()
        .entries()
        .iter()
        .find(|e| e.trial.from == 20 && e.trial.to == 30)
        .unwrap();
    let negative = selected
        .memory()
        .entries()
        .iter()
        .find(|e| e.trial.to == 77)
        .unwrap();
    assert_eq!(positive.origins.len(), 32);
    assert_eq!(negative.origins, vec![eid(100)]);
}
#[test]
fn conflict_conclusions_match_unconsolidated_control_when_both_searches_finish() {
    let mut normalized = vec![];
    for mode in [SnapshotStorage::Coalesced, SnapshotStorage::Separate] {
        let (mut c, s) = repeated(mode, 3);
        let mut a = EpisodeArchive::new();
        let r0 = independent_record(100, 20, 9, 77, &s);
        a.insert(eid(100), &r0, &s).unwrap();
        c.insert(2, a).unwrap();
        let mut r = runtime();
        let g = task(20, 30);
        let search = select(&mut r, &c, &s, g);
        assert!(search.coverage().complete_for_goal);
        normalized.push((
            outcomes(&mut r, &s, g, search.memory()),
            keys(search.memory()),
        ));
        assert_eq!(
            r.plan_retrieved(&s, g, PlanningBudget::default(), search.memory())
                .unwrap()
                .reason,
            PlanReason::ContradictoryEvidence
        );
    }
    assert_eq!(normalized[0], normalized[1]);
}
#[test]
fn storage_replicas_cannot_supply_a_missing_second_premise() {
    let s = session();
    let first = independent_record(1, 20, 9, 30, &s);
    let mut a = EpisodeArchive::new();
    for id in 1..=3 {
        a.insert(eid(id), &first, &s).unwrap();
    }
    let mut c = EpisodeCatalog::new();
    c.insert(1, a).unwrap();
    let mut r = runtime();
    let g = task(20, 40);
    {
        let search = select(&mut r, &c, &s, g);
        assert_eq!(search.memory().evidence_profile().transition_records, 1);
        assert_eq!(
            r.plan_retrieved(&s, g, PlanningBudget::default(), search.memory())
                .unwrap()
                .reason,
            PlanReason::InsufficientEvidence
        );
    }
    let second = independent_record(2, 30, 11, 40, &s);
    let mut a = EpisodeArchive::new();
    a.insert(eid(10), &second, &s).unwrap();
    c.insert(2, a).unwrap();
    let mut fresh = runtime();
    let search = select(&mut fresh, &c, &s, g);
    let plan = fresh
        .plan_retrieved(&s, g, PlanningBudget::default(), search.memory())
        .unwrap()
        .plan
        .unwrap();
    assert_eq!(plan.steps().len(), 2);
    let proofs: Vec<_> = plan
        .steps()
        .iter()
        .map(|p| search.memory().resolve_reference(p.id).unwrap())
        .collect();
    assert_ne!(proofs[0].after.evidence, proofs[1].before.evidence);
    assert_eq!(search.memory().evidence_profile().transition_records, 2);
}
#[test]
fn changing_h23_version_or_session_metadata_prevents_whole_snapshot_merging() {
    let (h, mut s) = history();
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &s).unwrap();
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
    a.insert(eid(2), &h, &s).unwrap();
    let bytes = a.encode();
    let mut c = EpisodeCatalog::new();
    c.insert(1, a).unwrap();
    assert_eq!(c.cost().retained_images, 2);
    assert_eq!(c.segment(1).unwrap().encode(), bytes);
    let mut r = runtime();
    let search = select(&mut r, &c, &s, task(20, 30));
    assert!(search.coverage().complete_for_goal);
    assert_eq!(
        search
            .memory()
            .versions()
            .iter()
            .map(|v| (v.episode.get(), v.version.number, v.relation))
            .collect::<Vec<_>>(),
        vec![
            (1, 1, VersionRelation::Ancestor),
            (2, 2, VersionRelation::Current)
        ]
    );
    let p = search.memory().evidence_profile();
    assert_eq!(p.h23_support_references, 2);
    assert_eq!(p.transition_records, 2);
    assert_eq!(p.episode_origins, 2);
}
#[test]
fn failed_attempt_tombstones_are_not_erased_by_similar_snapshots() {
    let (mut h, s) = history();
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &s).unwrap();
    let mut w0 = world(10, &[(10, 9, 20)]);
    assert!(h.intervene(&mut w0, &request(3, 7, 20)).is_err());
    a.insert(eid(2), &h, &s).unwrap();
    let mut c = EpisodeCatalog::new();
    c.insert(1, a).unwrap();
    assert_eq!(c.cost().retained_images, 2);
    let mut r = runtime();
    let g = task(999, 30);
    let search = select(&mut r, &c, &s, g);
    assert!(search.coverage().complete_for_goal);
    let mut reused = request(100, 7, 32);
    reused.permit.id = 1003;
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    assert!(r
        .pursue_with_consequences(
            search.memory(),
            &s,
            &mut w,
            PursuitGoal {
                task: g,
                max_actions: 2,
                ..goal(999)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &[reused, request(101, 9, 50)]
        )
        .is_err());
    assert_eq!(w.action_count(), 0);
}
#[test]
fn coalesced_catalog_rebuilds_after_new_process_without_restoring_authority() {
    let (c, s) = repeated(SnapshotStorage::Coalesced, 8);
    let dir = std::env::temp_dir().join(format!("neurons-coalescence-{}", std::process::id()));
    assert!(!dir.exists());
    std::fs::create_dir(&dir).unwrap();
    for id in c.segment_ids() {
        c.segment(id)
            .unwrap()
            .save(&dir.join(format!("{id}.bin")))
            .unwrap();
    }
    std::fs::write(dir.join("session.bin"), s.encode()).unwrap();
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "coalescence_restore_worker", "--nocapture"])
        .env("NEURONS_COALESCENCE_RESTORE", &dir)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    for id in c.segment_ids() {
        assert_eq!(
            std::fs::read(dir.join(format!("{id}.bin"))).unwrap(),
            c.segment(id).unwrap().encode()
        );
        std::fs::remove_file(dir.join(format!("{id}.bin"))).unwrap();
    }
    std::fs::remove_file(dir.join("session.bin")).unwrap();
    std::fs::remove_dir(dir).unwrap();
}
#[test]
fn coalescence_restore_worker() {
    let Some(dir) = std::env::var_os("NEURONS_COALESCENCE_RESTORE") else {
        return;
    };
    let dir = std::path::PathBuf::from(dir);
    let s = Session::decode(&std::fs::read(dir.join("session.bin")).unwrap()).unwrap();
    let mut c = EpisodeCatalog::new();
    for id in 1..=2 {
        let ports: Vec<_> = ((id - 1) * 4 + 1..=id * 4)
            .map(|ep| (eid(ep), sensors()))
            .collect();
        c.insert(
            id,
            EpisodeArchive::load(&dir.join(format!("{id}.bin")), &ports).unwrap(),
        )
        .unwrap();
    }
    assert_eq!(c.cost().retained_images, 1);
    let mut denied = runtime_with(Access::default(), Limits::default());
    assert!(denied
        .select_evidence(&c, &s, query(task(999, 30)), RetrievalBudget::default())
        .is_err());
    let mut r = runtime();
    let g = task(999, 30);
    let search = select(&mut r, &c, &s, g);
    assert_eq!(search.memory().evidence_profile().episode_origins, 8);
    let mut w = world(999, &[(999, 7, 20), (20, 9, 30)]);
    let out = r
        .pursue_with_consequences(
            search.memory(),
            &s,
            &mut w,
            PursuitGoal {
                task: g,
                max_actions: 2,
                ..goal(999)
            },
            contract(&s),
            reading(9, [1, 1, 1, 0, 0, 0], 30),
            &[request(100, 7, 32), request(101, 9, 50)],
        )
        .unwrap();
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(search.memory().evidence_profile().h23_support_references, 1);
}
#[test]
fn report_memory_and_processing_costs_without_assuming_latency_improvement() {
    for mode in [SnapshotStorage::Coalesced, SnapshotStorage::Separate] {
        let fixture = std::time::Instant::now();
        let (catalog, s) = repeated(mode, 4);
        let fixture_ns = fixture.elapsed().as_nanos();
        let mut r = runtime();
        let started = std::time::Instant::now();
        let search = select(&mut r, &catalog, &s, task(20, 30));
        let query_ns = started.elapsed().as_nanos();
        assert!(search.coverage().complete_for_goal);
        println!("COALESCENCE_METRICS mode={mode:?} origins={} retained_images={} supplied_bytes={} retained_payload_bytes={} retained_payload_capacity={} unique_postings={} index_scans={} materialization_scans={} index_build_ns={} fixture_ns={} query_ns={}",
            catalog.cost().episodes,catalog.cost().retained_images,catalog.cost().image_bytes,catalog.cost().retained_payload_bytes,catalog.cost().retained_payload_capacity,catalog.cost().unique_postings,
            search.coverage().index_scans,search.coverage().materialization_scans,catalog.cost().construction_nanos,fixture_ns,query_ns);
    }
}

fn overlapping(mode: SnapshotStorage, conflict: bool) -> (EpisodeCatalog, Session) {
    let (mut h, s) = history();
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &s).unwrap();
    let (from, action, to) = if conflict { (20, 9, 77) } else { (30, 11, 40) };
    let mut w = world(from, &[(from, action, to)]);
    h.intervene(&mut w, &request(3, action, 20)).unwrap();
    a.insert(eid(2), &h, &s).unwrap();
    let mut c = EpisodeCatalog::with_storage(mode);
    c.insert(1, a).unwrap();
    (c, s)
}
#[test]
fn partial_overlap_shares_postings_not_snapshots_or_exclusive_observations() {
    let mut controls = vec![];
    for mode in [SnapshotStorage::Coalesced, SnapshotStorage::Separate] {
        let (c, s) = overlapping(mode, false);
        assert_eq!(c.cost().retained_images, 2);
        assert_eq!(
            c.cost().unique_postings,
            if mode == SnapshotStorage::Coalesced {
                3
            } else {
                5
            }
        );
        let mut r = runtime();
        let g = task(20, 40);
        let selected = select(&mut r, &c, &s, g);
        assert!(selected.coverage().complete_for_goal);
        let m = selected.memory();
        assert_eq!(m.entries().len(), 3);
        for e in m.entries() {
            assert_eq!(
                e.origins,
                if e.trial.id == 3 {
                    vec![eid(2)]
                } else {
                    vec![eid(1), eid(2)]
                }
            );
        }
        assert_eq!(m.evidence_profile().h23_support_references, 1);
        let plan = r
            .plan_retrieved(&s, g, PlanningBudget::default(), m)
            .unwrap()
            .plan
            .unwrap();
        assert_eq!(plan.steps().len(), 2);
        controls.push(keys(m));
    }
    assert_eq!(controls[0], controls[1]);
}
#[test]
fn partial_overlap_preserves_conflicts_and_suspends_under_small_budget() {
    for mode in [SnapshotStorage::Coalesced, SnapshotStorage::Separate] {
        let (c, s) = overlapping(mode, true);
        let mut r = runtime();
        let g = task(20, 30);
        let partial = r
            .select_evidence(
                &c,
                &s,
                query(g),
                RetrievalBudget {
                    scans: 1,
                    entries: 8,
                },
            )
            .unwrap();
        assert!(!partial.coverage().complete_for_goal);
        assert!(r
            .plan_retrieved(&s, g, PlanningBudget::default(), partial.memory())
            .is_err());
        let mut fresh = runtime();
        let selected = select(&mut fresh, &c, &s, g);
        assert!(selected.coverage().complete_for_goal);
        assert_eq!(
            outcomes(&mut fresh, &s, g, selected.memory()),
            BTreeSet::from([30, 77])
        );
        assert_eq!(
            fresh
                .plan_retrieved(&s, g, PlanningBudget::default(), selected.memory())
                .unwrap()
                .reason,
            PlanReason::ContradictoryEvidence
        );
    }
}
#[test]
fn overlap_survives_new_process_and_preserves_original_archive_bytes() {
    let (c, s) = overlapping(SnapshotStorage::Coalesced, false);
    let dir = std::env::temp_dir().join(format!("neurons-overlap-{}", std::process::id()));
    assert!(!dir.exists());
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("archive.bin");
    c.segment(1).unwrap().save(&path).unwrap();
    std::fs::write(dir.join("session.bin"), s.encode()).unwrap();
    let before = std::fs::read(&path).unwrap();
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "overlap_restore_worker", "--nocapture"])
        .env("NEURONS_OVERLAP_RESTORE", &path)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(std::fs::read(&path).unwrap(), before);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(dir.join("session.bin")).unwrap();
    std::fs::remove_dir(dir).unwrap();
}
#[test]
fn overlap_restore_worker() {
    let Some(path) = std::env::var_os("NEURONS_OVERLAP_RESTORE") else {
        return;
    };
    let a = EpisodeArchive::load(
        std::path::Path::new(&path),
        &[(eid(1), sensors()), (eid(2), sensors())],
    )
    .unwrap();
    let mut c = EpisodeCatalog::new();
    c.insert(1, a).unwrap();
    assert_eq!(c.cost().unique_postings, 3);
    let s = Session::decode(
        &std::fs::read(
            std::path::Path::new(&path)
                .parent()
                .unwrap()
                .join("session.bin"),
        )
        .unwrap(),
    )
    .unwrap();
    let mut denied = runtime_with(Access::default(), Limits::default());
    assert!(denied
        .select_evidence(&c, &s, query(task(20, 40)), RetrievalBudget::default())
        .is_err());
    let mut r = runtime();
    let g = task(20, 40);
    let selected = select(&mut r, &c, &s, g);
    let plan = r
        .plan_retrieved(&s, g, PlanningBudget::default(), selected.memory())
        .unwrap()
        .plan
        .unwrap();
    assert_eq!(plan.steps().len(), 2);
    assert_eq!(
        selected
            .memory()
            .entries()
            .iter()
            .find(|e| e.trial.id == 3)
            .unwrap()
            .origins,
        vec![eid(2)]
    );
}
