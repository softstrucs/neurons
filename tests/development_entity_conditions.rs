//! Condições explícitas: entidade observada não é fonte informante.
use neurons::cognition::concepts::{Attributes, Label};
use neurons::cognitive_contracts::*;
use neurons::curriculum::{PedagogyAssessment, PedagogyHost, PedagogyPermissions};
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::{Event, EventKind, NodeId};
use neurons::integration::{development::*, synaptic::SynapticEpisode};
use neurons::metacognition::concepts::ConceptDecision;
use neurons::session::f1::Session;
fn n(id: u32) -> NodeId {
    NodeId::new(id).unwrap()
}
fn src(id: u32) -> SourceId {
    SourceId::new(id).unwrap()
}
fn eid(id: u32) -> EpisodeId {
    EpisodeId::new(id).unwrap()
}
fn sensors(reporter: u32, context: u32) -> Vec<Sensor> {
    [Encoding::Scalar, Encoding::BooleanAttributes]
        .into_iter()
        .enumerate()
        .map(|(i, encoding)| Sensor {
            id: i as u32 + 1,
            source: src(reporter),
            context,
            input: n(101),
            encoding,
        })
        .collect()
}
fn rights() -> Access {
    Access {
        record: true,
        explore: true,
        read_knowledge: true,
        learn_concepts: true,
        sensors: vec![1, 2],
        ..Default::default()
    }
}
fn runtime(reporter: u32, context: u32) -> Runtime {
    let mut neural = SynapticEpisode::new();
    for cmd in [
        format!("novo 41 {context} 81 1 0 3 2 16 64"),
        "neuronio 101 1 4".into(),
        "neuronio 103 2 4".into(),
        "sinapse 11 101 103 0 1".into(),
    ] {
        neural.execute(&cmd).unwrap();
    }
    Runtime::new(
        neural,
        &sensors(reporter, context),
        rights(),
        Limits::default(),
    )
    .unwrap()
}
fn session(context: u32) -> Session {
    let mut s = Session::new();
    for cmd in [
        format!("config 91 {context} 81 1 0 10000"),
        "conceito 601 opaco".into(),
        "agora 100".into(),
    ] {
        s.execute(&cmd).unwrap();
    }
    s
}
fn contract(s: &Session, entity: u32, reporter: u32) -> ConsequenceContract {
    ConsequenceContract {
        concept: n(601),
        scope: s.pedagogy_identity().unwrap().1,
        world: entity,
        revision: 1,
        source: src(reporter),
        action: 7,
        target: 20,
    }
}
fn world(entity: u32, reporter: u32, context: u32, from: i32, to: i32) -> World {
    World::new(
        entity,
        1,
        sensors(reporter, context)[0].clone(),
        from,
        &[(from, 7, to)],
    )
    .unwrap()
}
fn reading(entity: u32, reporter: u32, bits: [u8; 6], serial: u32, id: u32, at: u64) -> Reading {
    Reading {
        id: 100 + serial * 10 + id,
        item: n(entity),
        sensor: 2,
        source: src(reporter),
        evidence: EvidenceId::new(3000 + serial * 10 + id).unwrap(),
        event: Event::new(
            None,
            n(101),
            EventKind::Stimulus,
            1 + bits
                .iter()
                .enumerate()
                .map(|(i, b)| i32::from(*b) << i)
                .sum::<i32>(),
            at,
        ),
    }
}
fn request(
    entity: u32,
    reporter: u32,
    context: u32,
    serial: u32,
    id: u32,
    at: u64,
) -> Intervention {
    let nonce = serial * 10 + id;
    Intervention {
        id,
        permit: ActionPermit {
            id: 1000 + nonce,
            world: entity,
            revision: 1,
            context,
            source: src(reporter),
            action: 7,
        },
        before_id: 500 + nonce * 2,
        after_id: 501 + nonce * 2,
        before_evidence: EvidenceId::new(4000 + 2 * nonce).unwrap(),
        after_evidence: EvidenceId::new(4001 + 2 * nonce).unwrap(),
        at,
    }
}
fn learned(
    entity: u32,
    context: u32,
    reporter: u32,
    axis: usize,
    serial: u32,
) -> (Runtime, Session) {
    let mut positive = [0; 6];
    positive[axis] = 1;
    let negative = if axis == 0 {
        [0; 6]
    } else {
        [1, 0, 0, 0, 0, 0]
    };
    acquired(
        entity,
        context,
        reporter,
        serial,
        &[(positive, 20), (negative, 77)],
    )
}
// A tabela de consequências é material do ambiente de avaliação. O candidato
// recebe somente atributos atuais, depois ActionTrial, nunca o eixo/regra-alvo.
fn acquired(
    entity: u32,
    context: u32,
    reporter: u32,
    serial: u32,
    samples: &[([u8; 6], i32)],
) -> (Runtime, Session) {
    assert!(samples.len() <= 2);
    let mut r = runtime(reporter, context);
    let mut s = session(context);
    let c = contract(&s, entity, reporter);
    let mut frames = vec![];
    for (index, &(bits, actual)) in samples.iter().enumerate() {
        let id = index as u32 + 1;
        let at = (id as u64 - 1) * 10;
        let mut w = world(entity, reporter, context, 10 + id as i32, actual);
        let f = r
            .observe_attributes(&w, 7, reading(entity, reporter, bits, serial, id, at))
            .unwrap();
        r.intervene(
            &mut w,
            &request(entity, reporter, context, serial, id, at + 2),
        )
        .unwrap();
        frames.push((
            f,
            id,
            EngramId::new(500 + serial * 10 + id).unwrap(),
            900 + serial * 10 + id,
        ));
    }
    let permissions = PedagogyPermissions {
        evaluation: Some(c.scope),
        presentation: Some(c.scope),
        admissions: frames.iter().map(|s| s.2).collect(),
    };
    let assessment = PedagogyAssessment::new(
        &[Attributes::new([1, 1, 1, 1, 0, 0]).unwrap().into()],
        &[Attributes::new([0, 0, 1, 1, 1, 0]).unwrap().into()],
    )
    .unwrap();
    let report = r
        .teach_consequences(&mut s, c, &frames, assessment, permissions)
        .unwrap();
    assert_eq!(report.pedagogy.final_version.number, samples.len() as u32);
    (r, s)
}
fn prediction(s: &Session, bits: [u8; 6]) -> ConceptDecision {
    s.pedagogy_query(eid(80), Attributes::new(bits).unwrap().into())
        .unwrap()
        .decision
}
fn task(entity: u32, context: u32, reporter: u32) -> PlanningGoal {
    PlanningGoal {
        world: entity,
        revision: 1,
        context,
        source: src(reporter),
        from: 888,
        to: 20,
    }
}
fn corpus() -> (EpisodeCatalog, Runtime, Session, Runtime, Session) {
    let (a, sa) = learned(501, 61, 7, 0, 1);
    let (b, sb) = learned(502, 61, 7, 1, 2);
    let mut archive = EpisodeArchive::new();
    archive.insert(eid(1), &a, &sa).unwrap();
    archive.insert(eid(2), &b, &sb).unwrap();
    let mut c = EpisodeCatalog::new();
    c.insert(1, archive).unwrap();
    (c, a, sa, b, sb)
}
#[test]
fn observed_entities_can_have_different_learned_bodies_with_same_reporter_and_context() {
    let (_, _, a, _, b) = corpus();
    for bits in [[1, 0, 1, 0, 1, 0], [0, 1, 1, 0, 0, 1]] {
        assert_eq!(
            prediction(&a, bits),
            ConceptDecision::Supported(Label::from_bool(bits[0] == 1))
        );
        assert_eq!(
            prediction(&b, bits),
            ConceptDecision::Supported(Label::from_bool(bits[1] == 1))
        );
    }
    assert_eq!(
        a.pedagogy_identity().unwrap(),
        b.pedagogy_identity().unwrap()
    );
    println!("ENTITY: 4/4 novel results in separately bound models; reporter and Scope identical");
}
#[test]
fn reporter_permutation_preserves_regularities_of_same_entity() {
    let (_, a) = learned(501, 61, 7, 0, 1);
    let (_, b) = learned(501, 61, 8, 0, 1);
    assert_eq!(
        a.pedagogy_version().unwrap().criteria,
        b.pedagogy_version().unwrap().criteria
    );
    assert_ne!(
        a.concepts().unwrap().examples().unwrap()[0].metadata.source,
        b.concepts().unwrap().examples().unwrap()[0].metadata.source
    );
    for bits in [[1, 0, 1, 0, 1, 0], [0, 1, 1, 0, 0, 1]] {
        assert_eq!(prediction(&a, bits), prediction(&b, bits));
    }
}
#[test]
fn one_entity_reported_by_one_informant_can_follow_distinct_contextual_regularities() {
    let (_, a) = learned(501, 61, 7, 0, 1);
    let (_, b) = learned(501, 62, 7, 1, 2);
    assert_eq!(
        prediction(&a, [1, 0, 1, 0, 1, 0]),
        ConceptDecision::Supported(Label::Positive)
    );
    assert_eq!(
        prediction(&b, [1, 0, 1, 0, 1, 0]),
        ConceptDecision::Supported(Label::Negative)
    );
}
#[test]
fn consistent_renaming_of_entities_does_not_change_learned_predicates() {
    let (_, a) = learned(501, 61, 7, 0, 1);
    let (_, b) = learned(951, 61, 7, 0, 1);
    assert_eq!(a.encode(), b.encode());
    assert_eq!(
        prediction(&a, [1, 0, 1, 0, 1, 0]),
        prediction(&b, [1, 0, 1, 0, 1, 0])
    );
}
#[test]
fn wrong_active_session_is_rejected_by_existing_archive_lineage_validation() {
    let (c, _, sa, _, _) = corpus();
    let mut r = runtime(7, 61);
    let selected = r
        .select_evidence(
            &c,
            &sa,
            SelectiveQuery {
                goal: task(502, 61, 7),
                conceptual_seed: Some(20),
            },
            RetrievalBudget::default(),
        )
        .unwrap();
    assert_eq!(
        selected.memory().status(),
        RetrievalStatus::ConflictingVersions
    );
    assert!(r
        .plan_retrieved(
            &sa,
            task(502, 61, 7),
            PlanningBudget::default(),
            selected.memory()
        )
        .is_err());
}
#[test]
fn raw_runtime_and_unrelated_session_are_not_a_proof_of_entity_binding() {
    let (_, _, sa, b, _) = corpus();
    let mut receiver = runtime(7, 61);
    let mut w = world(502, 7, 61, 888, 77);
    let out = receiver
        .pursue_with_consequences(
            &b,
            &sa,
            &mut w,
            PursuitGoal {
                objective: eid(90),
                task: task(502, 61, 7),
                max_actions: 1,
            },
            contract(&sa, 502, 7),
            reading(502, 7, [1, 0, 1, 0, 1, 0], 8, 1, 70),
            &[request(502, 7, 61, 8, 10, 72)],
        )
        .unwrap();
    assert_ne!(out.reason, PredictiveReason::Reached);
    assert_eq!(w.action_count(), 1);
    assert_eq!(out.review.unwrap().agreement, Some(false));
    println!(
        "RAW-PAIR negative control: unrelated Session caused one refuted action; no final success"
    );
}
#[test]
fn unknown_entity_is_not_inferred_from_a_known_informant() {
    let (c, _, sa, _, _) = corpus();
    let mut r = runtime(7, 61);
    let query = task(7, 61, 7);
    let selected = r
        .select_evidence(
            &c,
            &sa,
            SelectiveQuery {
                goal: query,
                conceptual_seed: None,
            },
            RetrievalBudget::default(),
        )
        .unwrap();
    assert!(selected.memory().entries().is_empty());
    assert_eq!(
        r.plan_retrieved(&sa, query, PlanningBudget::default(), selected.memory())
            .unwrap()
            .reason,
        PlanReason::InsufficientEvidence
    );
    assert!(World::new(0, 1, sensors(7, 61)[0].clone(), 10, &[(10, 7, 20)]).is_err());
}
#[test]
fn insufficient_experience_does_not_define_an_entity_specific_regular_rule() {
    let s = session(61);
    assert!(matches!(
        prediction(&s, [1, 0, 1, 0, 1, 0]),
        ConceptDecision::Abstain(_)
    ));
}

fn condition(s: &Session, entity: u32, reporter: u32) -> EntityCondition {
    contract(s, entity, reporter).into()
}
#[test]
fn paired_selection_distinguishes_entities_and_avoids_foreign_session_prediction() {
    let (c, _, sa, b, _) = corpus();
    let mut r = runtime(7, 61);
    let a = r
        .recover_conditioned_knowledge(&c, condition(&sa, 501, 7), RetrievalBudget::default())
        .unwrap();
    let selected = r
        .recover_conditioned_knowledge(&c, condition(&sa, 502, 7), RetrievalBudget::default())
        .unwrap();
    assert_eq!(a.status, ConditionStatus::Ready);
    assert_eq!(selected.status, ConditionStatus::Ready);
    assert_eq!(a.origins, vec![eid(1)]);
    assert_eq!(selected.origins, vec![eid(2)]);
    let sb = selected.session().unwrap();
    assert_eq!(
        prediction(a.session().unwrap(), [1, 0, 1, 0, 1, 0]),
        ConceptDecision::Supported(Label::Positive)
    );
    assert_eq!(
        prediction(sb, [1, 0, 1, 0, 1, 0]),
        ConceptDecision::Supported(Label::Negative)
    );
    let mut w = world(502, 7, 61, 888, 77);
    let out = r
        .pursue_with_consequences(
            &b,
            sb,
            &mut w,
            PursuitGoal {
                objective: eid(90),
                task: task(502, 61, 7),
                max_actions: 1,
            },
            contract(sb, 502, 7),
            reading(502, 7, [1, 0, 1, 0, 1, 0], 8, 1, 70),
            &[request(502, 7, 61, 8, 10, 72)],
        )
        .unwrap();
    assert_eq!(w.action_count(), 0);
    assert_ne!(out.reason, PredictiveReason::Reached);
    assert!(selected.scans <= 32);
    assert_eq!(selected.checked_samples, 2);
    println!("PAIRED selection: foreign-model action count 1 → 0, same entity/attributes/private outcome");
}
#[test]
fn missing_or_unknown_identity_never_uses_informant_as_entity() {
    let (c, _, sa, _, _) = corpus();
    let mut r = runtime(7, 61);
    let mut q = condition(&sa, 501, 7);
    q.observed_entity = None;
    let out = r
        .recover_conditioned_knowledge(&c, q, RetrievalBudget::default())
        .unwrap();
    assert_eq!(out.status, ConditionStatus::MissingIdentity);
    assert!(out.session().is_none());
    for entity in [7, 503] {
        q.observed_entity = Some(n(entity));
        let out = r
            .recover_conditioned_knowledge(&c, q, RetrievalBudget::default())
            .unwrap();
        assert_eq!(out.status, ConditionStatus::UnknownCondition);
        assert!(out.session().is_none());
    }
    q = condition(&sa, 501, 7);
    q.scope.context = ContextId::new(62).unwrap();
    assert_eq!(
        r.recover_conditioned_knowledge(&c, q, RetrievalBudget::default())
            .unwrap()
            .status,
        ConditionStatus::UnknownCondition
    );
    q = condition(&sa, 501, 8);
    assert_eq!(
        r.recover_conditioned_knowledge(&c, q, RetrievalBudget::default())
            .unwrap()
            .status,
        ConditionStatus::UnknownCondition
    );
}
#[test]
fn reporter_and_context_are_separate_axes_in_conditioned_recovery() {
    let (a, sa) = learned(501, 61, 7, 0, 1);
    let (b, sb) = learned(501, 61, 8, 0, 2);
    let (d, sd) = learned(501, 62, 7, 1, 3);
    let mut archive = EpisodeArchive::new();
    for (id, r, s) in [(1, &a, &sa), (2, &b, &sb), (3, &d, &sd)] {
        archive.insert(eid(id), r, s).unwrap();
    }
    let mut c = EpisodeCatalog::new();
    c.insert(1, archive).unwrap();
    for (s, source, expected) in [
        (&sa, 7, Label::Positive),
        (&sb, 8, Label::Positive),
        (&sd, 7, Label::Negative),
    ] {
        let mut r = runtime(source, s.pedagogy_identity().unwrap().1.context.get());
        let out = r
            .recover_conditioned_knowledge(
                &c,
                condition(s, 501, source),
                RetrievalBudget::default(),
            )
            .unwrap();
        assert_eq!(
            prediction(out.session().unwrap(), [1, 0, 1, 0, 1, 0]),
            ConceptDecision::Supported(expected)
        );
    }
}
#[test]
fn removed_entity_condition_keeps_conflicting_model_branches_instead_of_choosing_one() {
    let (a, sa) = learned(501, 61, 7, 0, 1);
    let (b, sb) = learned(501, 61, 7, 1, 2);
    let mut archive = EpisodeArchive::new();
    archive.insert(eid(1), &a, &sa).unwrap();
    archive.insert(eid(2), &b, &sb).unwrap();
    let bytes = archive.encode();
    let mut c = EpisodeCatalog::new();
    c.insert(1, archive).unwrap();
    let mut r = runtime(7, 61);
    let out = r
        .recover_conditioned_knowledge(&c, condition(&sa, 501, 7), RetrievalBudget::default())
        .unwrap();
    assert_eq!(out.status, ConditionStatus::ConflictingModels);
    assert!(out.session().is_none());
    assert_eq!(out.origins, vec![eid(1), eid(2)]);
    assert_eq!(c.segment(1).unwrap().encode(), bytes);
}
#[test]
fn falsely_paired_snapshot_is_not_a_certificate_of_domain_validity() {
    let (_, _, sa, b, _) = corpus();
    let mut archive = EpisodeArchive::new();
    archive.insert(eid(1), &b, &sa).unwrap();
    let mut c = EpisodeCatalog::new();
    c.insert(1, archive).unwrap();
    let mut r = runtime(7, 61);
    let out = r
        .recover_conditioned_knowledge(&c, condition(&sa, 502, 7), RetrievalBudget::default())
        .unwrap();
    assert_eq!(out.status, ConditionStatus::ContraryEvidence);
    assert!(out.session().is_none());
    assert_eq!(out.counterevidence.len(), 2);
    assert!(out.criterion_evaluations <= 23 * 8);
}
#[test]
fn conditioned_recovery_obeys_permissions_and_incomplete_budget_never_yields_session() {
    let (c, _, sa, _, _) = corpus();
    let q = condition(&sa, 501, 7);
    let mut denied = Runtime::new(
        SynapticEpisode::new(),
        &[],
        Access::default(),
        Limits::default(),
    )
    .unwrap();
    assert!(denied
        .recover_conditioned_knowledge(&c, q, RetrievalBudget::default())
        .is_err());
    for scans in [0, 1, 2] {
        let mut r = runtime(7, 61);
        let out = r
            .recover_conditioned_knowledge(&c, q, RetrievalBudget { scans, entries: 8 })
            .unwrap();
        assert_eq!(out.status, ConditionStatus::ResourceLimited);
        assert!(out.session().is_none());
        assert!(out.scans <= scans);
    }
}

#[test]
fn model_selection_uses_proven_ancestry_and_keeps_observation_origins() {
    let (a, current) = learned(501, 61, 7, 0, 1);
    let mut old = session(61);
    old.pedagogy_present(neurons::curriculum::Lesson {
        id: EngramId::new(511).unwrap(),
        receipt: 911,
        exemplar: n(911),
        attributes: Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
        label: Label::Positive,
        source: src(7),
        evidence: EvidenceId::new(4023).unwrap(),
    })
    .unwrap();
    old.pedagogy_admit(EngramId::new(511).unwrap()).unwrap();
    let mut archive = EpisodeArchive::new();
    archive.insert(eid(1), &a, &old).unwrap();
    archive.insert(eid(2), &a, &current).unwrap();
    let mut c = EpisodeCatalog::new();
    c.insert(1, archive).unwrap();
    let mut r = runtime(7, 61);
    let out = r
        .recover_conditioned_knowledge(&c, condition(&current, 501, 7), RetrievalBudget::default())
        .unwrap();
    assert_eq!(out.status, ConditionStatus::Ready);
    assert_eq!(out.session().unwrap().pedagogy_version().unwrap().number, 2);
    assert_eq!(out.origins, vec![eid(1), eid(2)]);
    assert_eq!(out.as_of, Some(current.logical_time()));
}
#[test]
fn entity_condition_and_model_pair_survive_a_new_process_without_authority_restore() {
    let (c, a, sa, b, sb) = corpus();
    let dir = std::env::temp_dir().join(format!(
        "neurons-entity-condition-restore-{}",
        std::process::id()
    ));
    assert!(!dir.exists());
    std::fs::create_dir(&dir).unwrap();
    c.segment(1)
        .unwrap()
        .save(&dir.join("archive.bin"))
        .unwrap();
    std::fs::write(dir.join("active.bin"), b.checkpoint(&sb).unwrap()).unwrap();
    let original = std::fs::read(dir.join("archive.bin")).unwrap();
    let a_before = a.checkpoint(&sa).unwrap();
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "entity_condition_restore_worker", "--nocapture"])
        .env("NEURONS_ENTITY_CONDITION_RESTORE", &dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(std::fs::read(dir.join("archive.bin")).unwrap(), original);
    assert_eq!(a.checkpoint(&sa).unwrap(), a_before);
    std::fs::remove_file(dir.join("archive.bin")).unwrap();
    std::fs::remove_file(dir.join("active.bin")).unwrap();
    std::fs::remove_dir(dir).unwrap();
}
#[test]
fn entity_condition_restore_worker() {
    let Some(dir) = std::env::var_os("NEURONS_ENTITY_CONDITION_RESTORE") else {
        return;
    };
    let dir = std::path::PathBuf::from(dir);
    let bytes = std::fs::read(dir.join("active.bin")).unwrap();
    let (mut denied, s) = Runtime::restore(&bytes, &sensors(7, 61), Access::default()).unwrap();
    let archive = EpisodeArchive::load(
        &dir.join("archive.bin"),
        &[(eid(1), sensors(7, 61)), (eid(2), sensors(7, 61))],
    )
    .unwrap();
    let mut c = EpisodeCatalog::new();
    c.insert(1, archive).unwrap();
    let query = condition(&s, 502, 7);
    assert!(denied
        .recover_conditioned_knowledge(&c, query, RetrievalBudget::default())
        .is_err());
    let mut r = runtime(7, 61);
    let result = r
        .recover_conditioned_knowledge(&c, query, RetrievalBudget::default())
        .unwrap();
    assert_eq!(result.status, ConditionStatus::Ready);
    assert_eq!(result.origins, vec![eid(2)]);
    let sb = result.session().unwrap();
    assert_eq!(
        prediction(sb, [1, 0, 1, 0, 1, 0]),
        ConceptDecision::Supported(Label::Negative)
    );
    assert_eq!(
        prediction(sb, [0, 1, 1, 0, 0, 1]),
        ConceptDecision::Supported(Label::Positive)
    );
    let memory = r
        .select_evidence(
            &c,
            sb,
            SelectiveQuery {
                goal: task(502, 61, 7),
                conceptual_seed: Some(20),
            },
            RetrievalBudget::default(),
        )
        .unwrap();
    assert_eq!(memory.memory().status(), RetrievalStatus::Complete);
    assert!(memory
        .memory()
        .entries()
        .iter()
        .all(|e| e.trial.world == 502));
    assert_eq!(denied.checkpoint(&s).unwrap(), bytes);
}

#[test]
fn entity_identity_does_not_erase_other_conditions_of_validity() {
    let (c, _, sa, _, _) = corpus();
    let base = condition(&sa, 501, 7);
    for changed in 0..5 {
        let mut q = base;
        match changed {
            0 => q.revision = 2,
            1 => q.action = 9,
            2 => q.target = 77,
            3 => q.concept = n(602),
            _ => q.scope.rule.version = RuleVersion::new(2).unwrap(),
        };
        let mut r = runtime(7, 61);
        let out = r
            .recover_conditioned_knowledge(&c, q, RetrievalBudget::default())
            .unwrap();
        assert_eq!(out.status, ConditionStatus::UnknownCondition);
        assert!(out.session().is_none());
    }
}

#[test]
fn compatible_but_unanchored_foreign_model_remains_uncertain() {
    let (_, _, sa, _, _) = corpus();
    let mut r = runtime(7, 61);
    let mut local = session(61);
    let c = contract(&local, 502, 7);
    let mut w = world(502, 7, 61, 10, 20);
    let frame = r
        .observe_attributes(&w, 7, reading(502, 7, [1, 1, 1, 0, 0, 0], 3, 1, 0))
        .unwrap();
    r.intervene(&mut w, &request(502, 7, 61, 3, 1, 2)).unwrap();
    r.teach_consequences(
        &mut local,
        c,
        &[(frame, 1, EngramId::new(531).unwrap(), 931)],
        PedagogyAssessment::new(
            &[Attributes::new([0, 1, 0, 1, 0, 0]).unwrap().into()],
            &[Attributes::new([1, 1, 0, 1, 0, 0]).unwrap().into()],
        )
        .unwrap(),
        PedagogyPermissions {
            evaluation: Some(c.scope),
            presentation: Some(c.scope),
            admissions: vec![EngramId::new(531).unwrap()],
        },
    )
    .unwrap();
    // Modelo A não é refutado por este único ponto, mas suas evidências são de outra entidade.
    let mut archive = EpisodeArchive::new();
    archive.insert(eid(1), &r, &sa).unwrap();
    let mut catalog = EpisodeCatalog::new();
    catalog.insert(1, archive).unwrap();
    let mut reader = runtime(7, 61);
    let out = reader
        .recover_conditioned_knowledge(&catalog, condition(&sa, 502, 7), RetrievalBudget::default())
        .unwrap();
    assert_eq!(out.status, ConditionStatus::UnprovenBinding);
    assert!(out.session().is_none());
}
#[test]
fn opposing_reporters_do_not_become_separate_truths_for_the_same_observed_entity() {
    let (a, sa) = learned(501, 61, 7, 0, 1);
    let (b, sb) = learned(501, 61, 8, 1, 2);
    let mut archive = EpisodeArchive::new();
    archive.insert(eid(1), &a, &sa).unwrap();
    archive.insert(eid(2), &b, &sb).unwrap();
    let mut c = EpisodeCatalog::new();
    c.insert(1, archive).unwrap();
    for (s, reporter) in [(&sa, 7), (&sb, 8)] {
        let mut r = runtime(reporter, 61);
        let out = r
            .recover_conditioned_knowledge(
                &c,
                condition(s, 501, reporter),
                RetrievalBudget::default(),
            )
            .unwrap();
        assert_eq!(out.status, ConditionStatus::ContraryEvidence);
        assert!(out.session().is_none());
        assert!(out.counterevidence.iter().any(|e| e.trial.source == src(7)));
        assert!(out.counterevidence.iter().any(|e| e.trial.source == src(8)));
    }
}

// Experimentos prospectivos de relevância. Nenhum parâmetro do candidato diz
// qual atributo interessa: o eixo só existe no gerador/oráculo deste teste.
fn correlated() -> (Runtime, Session) {
    acquired(501, 61, 7, 1, &[([1, 1, 0, 0, 0, 0], 20), ([0; 6], 77)])
}
fn separate_correlation(
    actual: i32,
    teaching: bool,
    reverse: bool,
) -> (DiscriminatingInvestigation, Session, usize) {
    let (memory, mut s) = correlated();
    let c = contract(&s, 501, 7);
    let mut r = runtime(7, 61);
    let mut redundant = world(501, 7, 61, 777, 20);
    let mut discriminating = world(501, 7, 61, 888, actual);
    let g = teaching.then(|| PredictiveRevision {
        record: EngramId::new(999).unwrap(),
        receipt: 1999,
        assessment: PedagogyAssessment::new(
            &[Attributes::new([0, 1, 1, 0, 0, 1]).unwrap().into()],
            &[Attributes::new([1, 0, 1, 1, 0, 0]).unwrap().into()],
        )
        .unwrap(),
        permissions: PedagogyPermissions {
            evaluation: Some(c.scope),
            presentation: Some(c.scope),
            admissions: vec![EngramId::new(999).unwrap()],
        },
    });
    let mut offers = [
        InvestigationOffer {
            world: &mut redundant,
            attributes: reading(501, 7, [1, 1, 1, 0, 1, 0], 9, 20, 50),
            intervention: request(501, 7, 61, 9, 20, 52),
        },
        InvestigationOffer {
            world: &mut discriminating,
            attributes: reading(501, 7, [1, 0, 0, 1, 0, 1], 9, 21, 50),
            intervention: request(501, 7, 61, 9, 21, 52),
        },
    ];
    if reverse {
        offers.swap(0, 1);
    }
    let out = r
        .investigate_disagreement(
            &memory,
            &mut s,
            c,
            Attributes::new([1, 0, 0, 1, 0, 1]).unwrap(),
            &mut offers,
            g,
        )
        .unwrap();
    assert_eq!(redundant.action_count(), 0);
    assert_eq!(discriminating.action_count(), 1);
    assert!(out.selection_scans <= 4);
    assert!(out.criterion_evaluations <= 4 * 23);
    assert!(r.attention_scans() <= 32);
    (out, s, r.experiences().len())
}

#[test]
fn relevant_attribute_is_learned_for_all_six_axes_without_target_rule_input() {
    use neurons::cognition::concepts::{Attribute, Criterion};
    let mut novel = 0;
    for axis in 0..6 {
        let (_, s) = learned(501, 61, 7, axis, 1);
        assert_eq!(
            s.pedagogy_version().unwrap().criteria,
            vec![Criterion::attribute(Attribute::ALL[axis])]
        );
        for mask in 0..64 {
            let bits = std::array::from_fn(|i| ((mask >> i) & 1) as u8);
            assert_eq!(
                prediction(&s, bits),
                ConceptDecision::Supported(Label::from_bool(bits[axis] == 1))
            );
            let positive = 1 << axis;
            let negative = if axis == 0 { 0 } else { 1 };
            if mask != positive && mask != negative {
                novel += 1;
            }
        }
    }
    assert_eq!(novel, 372);
    println!("RELEVANCE: 384/384 evaluations; 372 outside acquisition; six opaque axes");
}

#[test]
fn perfect_training_correlation_does_not_establish_which_condition_is_relevant() {
    let (_, s) = correlated();
    assert_eq!(s.pedagogy_version().unwrap().criteria.len(), 3);
    for bits in [[1, 0, 1, 0, 1, 0], [0, 1, 0, 1, 0, 1]] {
        assert!(matches!(prediction(&s, bits), ConceptDecision::Abstain(_)));
    }
    let before = s.encode();
    let (_, unexposed) = correlated();
    assert_eq!(unexposed.encode(), before);
}

#[test]
fn discriminating_experience_breaks_spurious_correlation_and_generalizes() {
    let (out, s, observations) = separate_correlation(20, true, false);
    assert_eq!(out.origin.state, EpistemicDiagnosis::InsufficientEvidence);
    assert_eq!(out.selected, Some(211));
    assert_eq!(out.options[0].separation, 0);
    assert!(out.options[1].separation > 0);
    assert_eq!(out.result, InvestigationResult::Learned);
    assert_eq!(out.retained.len(), 1);
    assert_eq!(s.pedagogy_version().unwrap().number, 3);
    assert_eq!(observations, 3);
    let mut novel = 0;
    for mask in 0..64 {
        let bits = std::array::from_fn(|i| ((mask >> i) & 1) as u8);
        assert_eq!(
            prediction(&s, bits),
            ConceptDecision::Supported(Label::from_bool(bits[0] == 1))
        );
        if ![0, 3, 41].contains(&mask) {
            novel += 1;
        }
    }
    assert_eq!(novel, 61);
    println!("DISCRIMINATION: ambiguous A/B/A&B -> A; 61 novel inputs, no hidden-result selection");
}

#[test]
fn concealed_outcome_changes_revision_but_not_investigation_choice() {
    let (positive, a, _) = separate_correlation(20, true, false);
    let (negative, b, _) = separate_correlation(77, true, false);
    assert_eq!(positive.selected, negative.selected);
    assert_eq!(positive.options, negative.options);
    assert_ne!(positive.retained, negative.retained);
    assert!(matches!(
        prediction(&b, [0, 1, 1, 1, 0, 0]),
        ConceptDecision::Abstain(_)
    ));
    assert_ne!(a.pedagogy_version().unwrap(), b.pedagogy_version().unwrap());
    println!("INTERVENTION: same observations/offers, hidden outcome changed only after selection; negative branch remains ambiguous");
}

#[test]
fn offer_order_is_not_a_relevance_oracle() {
    let (a, sa, _) = separate_correlation(20, true, false);
    let (b, sb, _) = separate_correlation(20, true, true);
    assert_eq!(a.selected, b.selected);
    assert_eq!(a.retained, b.retained);
    assert_eq!(sa.encode(), sb.encode());
}

#[test]
fn observation_without_admission_keeps_learning_provisional() {
    let (_, before) = correlated();
    let (out, after, _) = separate_correlation(20, false, false);
    assert_eq!(out.retained.len(), 1);
    assert_eq!(out.result, InvestigationResult::Observed);
    assert!(out.teaching.is_none());
    assert_eq!(before.encode(), after.encode());
    assert!(matches!(
        prediction(&after, [1, 0, 1, 0, 0, 1]),
        ConceptDecision::Abstain(_)
    ));
}

#[test]
fn missing_relevant_condition_abstains_but_missing_irrelevant_condition_does_not() {
    use neurons::cognition::concepts::Observation;
    let (_, s) = learned(501, 61, 7, 0, 1);
    let mut bits = [Some(1), Some(0), Some(1), Some(0), Some(0), Some(1)];
    bits[0] = None;
    let absent = s
        .pedagogy_query(eid(81), Observation::new(bits).unwrap())
        .unwrap();
    assert!(matches!(absent.decision, ConceptDecision::Abstain(_)));
    bits[0] = Some(1);
    bits[5] = None;
    assert_eq!(
        s.pedagogy_query(eid(82), Observation::new(bits).unwrap())
            .unwrap()
            .decision,
        ConceptDecision::Supported(Label::Positive)
    );
}

#[test]
fn no_experience_cannot_select_a_relevant_axis() {
    let s = session(61);
    assert_eq!(s.pedagogy_version().unwrap().criteria.len(), 23);
    for mask in 0..64 {
        assert!(matches!(
            prediction(&s, std::array::from_fn(|i| ((mask >> i) & 1) as u8)),
            ConceptDecision::Abstain(_)
        ));
    }
}

#[test]
fn unavailable_condition_yields_conflict_not_an_invented_context_partition() {
    let (memory, s) = acquired(501, 61, 7, 1, &[([1, 0, 0, 0, 0, 0], 20)]);
    let mut r = runtime(7, 61);
    r.inherit_consequence_contract(&memory).unwrap();
    // O ambiente mudou uma condição que não está em nenhum sensor/Scope.
    // Fornecer um novo ContextId ao candidato resolveria artificialmente a questão.
    let mut w = world(501, 7, 61, 999, 77);
    r.observe_attributes(&w, 7, reading(501, 7, [1, 0, 0, 0, 0, 0], 8, 1, 50))
        .unwrap();
    r.intervene(&mut w, &request(501, 7, 61, 8, 10, 52))
        .unwrap();
    let bytes = s.encode();
    let out = r
        .assess_epistemic(
            &memory,
            &s,
            contract(&s, 501, 7),
            Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
        )
        .unwrap();
    assert_eq!(out.state, EpistemicDiagnosis::ObservationConflict);
    assert_eq!(out.question, Some(EpistemicQuestion::CheckSourceAndContext));
    assert_eq!(out.evidence.len(), 2);
    assert!(out.blocks_application());
    assert_eq!(s.encode(), bytes);
    println!("LIMIT: indistinguishable inputs/opposite effects -> retained conflict, no discovered latent context");
}

#[test]
fn finite_family_can_select_a_spurious_rule_and_must_preserve_its_counterexample() {
    // Oráculo independente XOR(A,B): os dois pontos de aquisição também cabem
    // em A. H23 não contém XOR. A certeza interna não prova verdade universal.
    let (memory, s) = learned(501, 61, 7, 0, 1);
    let bits = [1, 1, 1, 0, 0, 1];
    let actual = if (bits[0] == 1) ^ (bits[1] == 1) {
        20
    } else {
        77
    };
    assert_eq!(
        prediction(&s, bits),
        ConceptDecision::Supported(Label::Positive)
    );
    let mut r = runtime(7, 61);
    let mut w = world(501, 7, 61, 888, actual);
    let out = r
        .pursue_with_consequences(
            &memory,
            &s,
            &mut w,
            PursuitGoal {
                objective: eid(90),
                task: task(501, 61, 7),
                max_actions: 1,
            },
            contract(&s, 501, 7),
            reading(501, 7, bits, 8, 1, 70),
            &[request(501, 7, 61, 8, 10, 72)],
        )
        .unwrap();
    assert_eq!(w.action_count(), 1);
    assert_ne!(out.reason, PredictiveReason::Reached);
    assert_eq!(out.review.unwrap().agreement, Some(false));
    let assessed = r
        .assess_epistemic(
            &memory,
            &s,
            contract(&s, 501, 7),
            Attributes::new(bits).unwrap(),
        )
        .unwrap();
    assert_eq!(assessed.state, EpistemicDiagnosis::ModelExpressiveness);
    assert!(assessed.blocks_application());
    assert_eq!(assessed.evidence.len(), 1);
    assert_eq!(assessed.evidence[0].trial.to, actual);
    assert_eq!(s.pedagogy_version().unwrap().number, 2);
    println!("COUNTEREXAMPLE: H23 learned A, external XOR oracle refutes novel A=B=1; observation retained, no invented family");
}
