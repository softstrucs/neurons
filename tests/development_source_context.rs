//! Experimentos públicos finitos: escopo, procedência e atomicidade de recusa.
//! Regras de referência só constroem World privado; aquisição usa a consequência.
use neurons::cognition::concepts::{
    Attributes, ConceptIdentity, IncomingExample, Label, ReceiptId,
};
use neurons::cognitive_contracts::*;
use neurons::curriculum::{PedagogyAssessment, PedagogyHost, PedagogyPermissions};
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::{Event, EventKind, NodeId};
use neurons::integration::{
    concepts::{
        Admission, ConceptCoordinator, ConceptEpisodeLog, Limits as ConceptLimits, Refusal,
    },
    development::*,
    synaptic::SynapticEpisode,
};
use neurons::metacognition::concepts::ConceptDecision;
use neurons::session::f1::Session;
fn node(v: u32) -> NodeId {
    NodeId::new(v).unwrap()
}
fn source(v: u32) -> SourceId {
    SourceId::new(v).unwrap()
}
fn eid(v: u32) -> EpisodeId {
    EpisodeId::new(v).unwrap()
}
fn scope(ctx: u32) -> Scope {
    Scope {
        catalog: CatalogId::new(91).unwrap(),
        context: ContextId::new(ctx).unwrap(),
        rule: RuleRef {
            id: RuleId::new(81).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 10000).unwrap(),
    }
}
fn sensors(src: u32, ctx: u32) -> Vec<Sensor> {
    [Encoding::Scalar, Encoding::BooleanAttributes]
        .into_iter()
        .enumerate()
        .map(|(i, encoding)| Sensor {
            id: i as u32 + 1,
            source: source(src),
            context: ctx,
            input: node(101),
            encoding,
        })
        .collect()
}
fn rights() -> Access {
    Access {
        read_knowledge: true,
        record: true,
        explore: true,
        learn_concepts: true,
        sensors: vec![1, 2],
        ..Default::default()
    }
}
fn runtime(src: u32, ctx: u32) -> Runtime {
    let mut neural = SynapticEpisode::new();
    for cmd in [
        format!("novo 41 {ctx} 81 1 0 3 2 16 64"),
        "neuronio 101 1 4".into(),
        "neuronio 103 2 4".into(),
        "sinapse 11 101 103 0 1".into(),
    ] {
        neural.execute(&cmd).unwrap();
    }
    Runtime::new(neural, &sensors(src, ctx), rights(), Limits::default()).unwrap()
}
fn reading(src: u32, bits: [u8; 6], id: u32, at: u64) -> Reading {
    Reading {
        id: 100 + id,
        item: node(501),
        sensor: 2,
        source: source(src),
        evidence: EvidenceId::new(3000 + id).unwrap(),
        event: Event::new(
            None,
            node(101),
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
fn request(src: u32, ctx: u32, id: u32, at: u64) -> Intervention {
    Intervention {
        id,
        permit: ActionPermit {
            id: 1000 + id,
            world: 501,
            revision: 1,
            context: ctx,
            source: source(src),
            action: 7,
        },
        before_id: 2 * id,
        after_id: 2 * id + 1,
        before_evidence: EvidenceId::new(4000 + 2 * id).unwrap(),
        after_evidence: EvidenceId::new(4001 + 2 * id).unwrap(),
        at,
    }
}
fn world(src: u32, ctx: u32, from: i32, to: i32) -> World {
    World::new(501, 1, sensors(src, ctx)[0].clone(), from, &[(from, 7, to)]).unwrap()
}
fn session(ctx: u32) -> Session {
    let mut s = Session::new();
    for cmd in [
        format!("config 91 {ctx} 81 1 0 10000"),
        "conceito 601 opaco".into(),
        "agora 100".into(),
    ] {
        s.execute(&cmd).unwrap();
    }
    s
}
fn contract(src: u32, ctx: u32) -> ConsequenceContract {
    ConsequenceContract {
        concept: node(601),
        scope: scope(ctx),
        world: 501,
        revision: 1,
        source: source(src),
        action: 7,
        target: 20,
    }
}
fn learned(src: u32, ctx: u32) -> (Runtime, Session) {
    let mut r = runtime(src, ctx);
    let mut s = session(ctx);
    let c = contract(src, ctx);
    let mut w = world(src, ctx, 10, 20);
    let frame = r
        .observe_attributes(&w, 7, reading(src, [1, 0, 0, 0, 0, 0], 1, 0))
        .unwrap();
    r.intervene(&mut w, &request(src, ctx, 1, 2)).unwrap();
    let assessment = PedagogyAssessment::new(
        &[Attributes::new([0, 1, 0, 1, 0, 1]).unwrap().into()],
        &[Attributes::new([1, 1, 0, 0, 0, 1]).unwrap().into()],
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
    (r, s)
}
fn attempt(
    memory: &Runtime,
    receiver: &mut Runtime,
    s: &Session,
    src: u32,
    ctx: u32,
    w: &mut World,
) -> Result<PredictivePursuit, String> {
    receiver.pursue_with_consequences(
        memory,
        s,
        w,
        PursuitGoal {
            objective: eid(80),
            task: PlanningGoal {
                world: 501,
                revision: 1,
                context: ctx,
                source: source(src),
                from: 888,
                to: 20,
            },
            max_actions: 1,
        },
        contract(src, ctx),
        reading(src, [1, 1, 1, 0, 0, 0], 19, 70),
        &[request(src, ctx, 20, 72)],
    )
}
fn boundary(src: u32, ctx: u32) {
    let (memory, _) = learned(7, 61);
    let (_, s) = learned(src, ctx);
    let mut receiver = runtime(src, ctx);
    let before = receiver.checkpoint(&s).unwrap();
    let mut w = world(src, ctx, 888, 20);
    let result = attempt(&memory, &mut receiver, &s, src, ctx, &mut w);
    println!(
        "BOUNDARY src={src} ctx={ctx} error={} observations={} actions={}",
        result.is_err(),
        receiver.experiences().len(),
        w.action_count()
    );
    assert!(result.is_err());
    assert_eq!(w.action_count(), 0);
    assert!(
        receiver.experiences().is_empty(),
        "recusa incompatível registrou observação antes de validar a memória"
    );
    assert_eq!(receiver.checkpoint(&s).unwrap(), before);
}
#[test]
fn incompatible_source_is_rejected_before_recording() {
    boundary(8, 61)
}
#[test]
fn incompatible_context_is_rejected_before_recording() {
    boundary(7, 62)
}
#[test]
fn incompatible_source_and_context_are_rejected_before_recording() {
    boundary(8, 62)
}
#[test]
fn matching_bindings_still_apply_learned_hypothesis_to_new_state() {
    for (src, ctx) in [(7, 61), (8, 61), (7, 62), (8, 62)] {
        let (m, s) = learned(src, ctx);
        let mut r = runtime(src, ctx);
        let mut w = world(src, ctx, 888, 20);
        let result = attempt(&m, &mut r, &s, src, ctx, &mut w).unwrap();
        assert_eq!(result.reason, PredictiveReason::Reached);
        assert_eq!(w.action_count(), 1);
    }
}
#[derive(Clone)]
struct Sample {
    bits: [u8; 6],
    label: Label,
    source: SourceId,
    context: u32,
    evidence: EvidenceId,
    id: u32,
}
fn observed(id: u32, src: u32, ctx: u32, bits: [u8; 6], positive: bool) -> Sample {
    let mut r = runtime(src, ctx);
    let mut w = world(src, ctx, 1000 + id as i32, if positive { 20 } else { 77 });
    r.observe_attributes(&w, 7, reading(src, bits, id, 10))
        .unwrap();
    let trial = r.intervene(&mut w, &request(src, ctx, id, 12)).unwrap();
    Sample {
        bits,
        label: Label::from_bool(trial.to == 20),
        source: source(src),
        context: ctx,
        evidence: r.experiences().last().unwrap().reading.evidence,
        id,
    }
}
fn dataset(same_context: bool, same_source: bool) -> Vec<Sample> {
    let ctx = if same_context { 61 } else { 62 };
    let src = if same_source { 7 } else { 8 };
    // Dois regimes privados: o primeiro segue A, o segundo B. Os tratamentos
    // agrupam ou separam seus metadados; isso não é descoberta desses regimes.
    // O candidato só recebe labels depois de intervenção e observação reais.
    vec![
        observed(1, 7, 61, [1, 0, 0, 0, 0, 0], true),
        observed(2, 7, 61, [0, 0, 0, 0, 0, 0], false),
        observed(3, src, ctx, [0, 1, 0, 0, 0, 0], true),
        observed(4, src, ctx, [1, 0, 0, 0, 0, 0], false),
    ]
}
fn coordinator(contexts: &[u32]) -> ConceptCoordinator {
    let mut c = ConceptCoordinator::new(
        ConceptIdentity::new(node(601), scope(61).catalog, "opaco").unwrap(),
        &[source(7), source(8)],
        ConceptLimits::default(),
    )
    .unwrap();
    for (i, ctx) in contexts.iter().enumerate() {
        c.register_context(scope(*ctx), i as u64 + 1).unwrap();
    }
    c
}
fn admit(c: &mut ConceptCoordinator, sample: &Sample) -> Admission {
    let at = 100 + sample.id as u64 * 10;
    c.receive(
        IncomingExample {
            receipt: ReceiptId::new(900 + sample.id).unwrap(),
            teacher_episode: eid(900 + sample.id),
            concept: node(601),
            exemplar: node(1000 + sample.id),
            attributes: sample.bits.map(Some),
            label: sample.label,
            scope: scope(sample.context),
            source: Some(sample.source),
            evidence: Some(sample.evidence),
            nature: DeclaredNature::Observation,
            observed_at: 12,
            sent_at: at,
        },
        at,
    )
    .unwrap();
    c.interpret(at + 1).unwrap();
    let admission = c.assess_admissibility(at + 2).unwrap();
    if admission == Admission::Admissible {
        c.incorporate(EngramId::new(sample.id).unwrap(), at + 3)
            .unwrap();
        c.revise(at + 4).unwrap();
    }
    admission
}
fn query(
    c: &ConceptCoordinator,
    ctx: u32,
    bits: [u8; 6],
) -> neurons::integration::concepts::ConceptReport {
    c.query(
        eid(90),
        scope(ctx),
        Attributes::new(bits).unwrap().into(),
        1000,
        &mut ConceptEpisodeLog::default(),
    )
    .unwrap()
}
#[test]
fn contexts_preserve_opposite_knowledge_and_support_unseen_inputs() {
    let mut c = coordinator(&[61, 62]);
    let unseen = [[1, 0, 1, 0, 1, 0], [0, 1, 1, 0, 0, 1]];
    for ctx in [61, 62] {
        for bits in unseen {
            assert!(matches!(
                query(&c, ctx, bits).decision,
                ConceptDecision::Abstain(_)
            ));
        }
    }
    for sample in dataset(false, false) {
        assert_eq!(admit(&mut c, &sample), Admission::Admissible);
    }
    for ctx in [61, 62] {
        for bits in unseen {
            let report = query(&c, ctx, bits);
            assert_eq!(
                report.decision,
                ConceptDecision::Supported(Label::from_bool(
                    bits[if ctx == 61 { 0 } else { 1 }] == 1
                ))
            );
            assert!(report
                .consulted
                .iter()
                .all(|e| e.metadata.scope.context.get() == ctx));
        }
    }
    println!("CONTEXT separated: 4/4 unseen correct; no experience: 4/4 abstentions");
}
#[test]
fn context_removal_with_all_labels_preserved_creates_abstention_not_false_certainty() {
    let mut c = coordinator(&[61]);
    for sample in dataset(true, false) {
        assert_eq!(admit(&mut c, &sample), Admission::Admissible);
    }
    let report = query(&c, 61, [1, 0, 1, 0, 1, 0]);
    assert!(matches!(report.decision, ConceptDecision::Abstain(_)));
    assert!(report.version.unwrap().criteria.is_empty());
    assert_eq!(c.examples().unwrap().len(), 4);
    println!("CONTEXT merged: H23 cannot fit contradictory pooled observations; evidence retained");
}
#[test]
fn changing_source_ids_without_context_or_label_changes_does_not_change_h23() {
    let rows = dataset(false, false);
    let mut a = coordinator(&[61, 62]);
    let mut b = coordinator(&[61, 62]);
    for row in &rows {
        assert_eq!(admit(&mut a, row), Admission::Admissible);
        let mut swapped = row.clone();
        swapped.source = source(if row.source.get() == 7 { 8 } else { 7 });
        assert_eq!(admit(&mut b, &swapped), Admission::Admissible);
    }
    for ctx in [61, 62] {
        for bits in [[1, 0, 1, 0, 1, 0], [0, 1, 1, 0, 0, 1]] {
            let x = query(&a, ctx, bits);
            let y = query(&b, ctx, bits);
            assert_eq!(x.decision, y.decision);
            assert_eq!(x.version.unwrap().criteria, y.version.unwrap().criteria);
        }
    }
    assert_ne!(
        a.examples().unwrap()[0].metadata.source,
        b.examples().unwrap()[0].metadata.source
    );
    println!("SOURCE identity permutation: 4/4 predictions unchanged; provenance differs");
}
#[test]
fn source_identity_alone_does_not_learn_a_source_conditional_rule() {
    let mut c = coordinator(&[61]);
    for row in dataset(true, false) {
        assert_eq!(admit(&mut c, &row), Admission::Admissible);
    }
    let report = query(&c, 61, [1, 0, 1, 0, 1, 0]);
    assert!(matches!(report.decision, ConceptDecision::Abstain(_)));
    assert_eq!(
        report
            .consulted
            .iter()
            .map(|e| e.metadata.source.get())
            .collect::<std::collections::BTreeSet<_>>(),
        std::collections::BTreeSet::from([7, 8])
    );
    println!("SOURCE conditional: not demonstrated; query has scope + attributes, no source input");
}
#[test]
fn one_source_can_supply_different_contexts_without_merging_their_models() {
    let mut c = coordinator(&[61, 62]);
    for row in dataset(false, true) {
        assert_eq!(admit(&mut c, &row), Admission::Admissible);
    }
    assert_eq!(
        query(&c, 61, [1, 0, 1, 0, 1, 0]).decision,
        ConceptDecision::Supported(Label::Positive)
    );
    assert_eq!(
        query(&c, 62, [1, 0, 1, 0, 1, 0]).decision,
        ConceptDecision::Supported(Label::Negative)
    );
}
#[test]
fn source_allowlist_affects_admission_without_claiming_source_truthfulness() {
    for src in [7, 8, 99] {
        let mut c = coordinator(&[61]);
        let row = observed(1, src, 61, [1, 0, 0, 0, 0, 0], true);
        let result = admit(&mut c, &row);
        assert_eq!(
            result,
            if src == 99 {
                Admission::Quarantined(Refusal::UnknownSource)
            } else {
                Admission::Admissible
            }
        );
    }
}

#[test]
fn consistent_sources_can_jointly_refine_a_model_without_becoming_domain_variables() {
    let (m, mut s) = learned(7, 61);
    let input = Attributes::new([0, 1, 1, 0, 0, 1]).unwrap();
    assert!(matches!(
        s.pedagogy_query(eid(80), input.into()).unwrap().decision,
        ConceptDecision::Abstain(_)
    ));
    let sample = observed(2, 8, 61, [0, 0, 0, 0, 0, 0], false);
    assert_eq!(
        s.pedagogy_present(neurons::curriculum::Lesson {
            id: EngramId::new(502).unwrap(),
            receipt: 902,
            exemplar: node(902),
            attributes: Attributes::new(sample.bits).unwrap(),
            label: sample.label,
            source: sample.source,
            evidence: sample.evidence
        })
        .unwrap(),
        Admission::Admissible
    );
    s.pedagogy_admit(EngramId::new(502).unwrap()).unwrap();
    let report = s.pedagogy_query(eid(81), input.into()).unwrap();
    assert_eq!(report.decision, ConceptDecision::Supported(Label::Negative));
    assert_eq!(
        report
            .consulted
            .iter()
            .map(|e| e.metadata.source.get())
            .collect::<std::collections::BTreeSet<_>>(),
        std::collections::BTreeSet::from([7, 8])
    );
    let mut r = runtime(7, 61);
    let mut w = world(7, 61, 888, 20);
    assert_eq!(
        attempt(&m, &mut r, &s, 7, 61, &mut w).unwrap().reason,
        PredictiveReason::Reached
    );
    println!("CONSISTENT sources: unknown→negative after actual source-8 feedback; source-7 binding remains valid");
}
#[test]
fn allowlist_does_not_resolve_contradictions_from_one_known_source() {
    let mut c = coordinator(&[61]);
    for sample in dataset(true, true) {
        assert_eq!(admit(&mut c, &sample), Admission::Admissible);
    }
    assert!(matches!(
        query(&c, 61, [1, 0, 1, 0, 1, 0]).decision,
        ConceptDecision::Abstain(_)
    ));
}
fn session_with_contexts() -> Session {
    let mut s = session(61);
    s.execute("contexto-conceitual 62").unwrap();
    for row in dataset(false, false) {
        let vector: String = row.bits.iter().map(|v| char::from(b'0' + *v)).collect();
        let label = if row.label == Label::Positive {
            "+"
        } else {
            "-"
        };
        for command in [
            format!(
                "exemplo {} {} {} {vector} {label} {} {} observacao",
                900 + row.id,
                1000 + row.id,
                row.context,
                row.source.get(),
                row.evidence.get()
            ),
            "interpretar-exemplo".into(),
            "avaliar-exemplo".into(),
            format!("admitir-exemplo {}", row.id),
            "revisar".into(),
        ] {
            s.execute(&command).unwrap();
        }
    }
    s
}
fn scoped_session_query(
    s: &Session,
    ctx: u32,
    bits: [u8; 6],
) -> neurons::integration::concepts::ConceptReport {
    s.concepts()
        .unwrap()
        .query(
            eid(90),
            scope(ctx),
            Attributes::new(bits).unwrap().into(),
            s.logical_time(),
            &mut ConceptEpisodeLog::default(),
        )
        .unwrap()
}
#[test]
fn checkpoint_preserves_context_models_sources_and_early_refusal_in_new_process() {
    let s = session_with_contexts();
    let (a, sa) = learned(7, 61);
    let (b, sb) = learned(8, 62);
    let dir = std::env::temp_dir().join(format!(
        "neurons-source-context-restore-{}",
        std::process::id()
    ));
    assert!(!dir.exists());
    std::fs::create_dir(&dir).unwrap();
    let bytes = s.encode();
    let ba = a.checkpoint(&sa).unwrap();
    let bb = b.checkpoint(&sb).unwrap();
    for (name, data) in [("contexts.bin", &bytes), ("a.bin", &ba), ("b.bin", &bb)] {
        std::fs::write(dir.join(name), data).unwrap();
    }
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "source_context_restore_worker", "--nocapture"])
        .env("NEURONS_SOURCE_CONTEXT_RESTORE", &dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    for (name, data) in [("contexts.bin", &bytes), ("a.bin", &ba), ("b.bin", &bb)] {
        assert_eq!(std::fs::read(dir.join(name)).unwrap(), *data);
        std::fs::remove_file(dir.join(name)).unwrap();
    }
    std::fs::remove_dir(dir).unwrap();
}
#[test]
fn source_context_restore_worker() {
    let Some(dir) = std::env::var_os("NEURONS_SOURCE_CONTEXT_RESTORE") else {
        return;
    };
    let dir = std::path::PathBuf::from(dir);
    let original = std::fs::read(dir.join("contexts.bin")).unwrap();
    let s = Session::decode(&original).unwrap();
    assert_eq!(s.encode(), original);
    for ctx in [61, 62] {
        for bits in [[1, 0, 1, 0, 1, 0], [0, 1, 1, 0, 0, 1]] {
            let report = scoped_session_query(&s, ctx, bits);
            assert_eq!(
                report.decision,
                ConceptDecision::Supported(Label::from_bool(
                    bits[if ctx == 61 { 0 } else { 1 }] == 1
                ))
            );
            assert!(report
                .consulted
                .iter()
                .all(|e| e.metadata.scope.context.get() == ctx));
            assert!(report
                .consulted
                .iter()
                .all(|e| e.metadata.source.get() == if ctx == 61 { 7 } else { 8 }));
        }
    }
    // MP09 limita-se a um contexto: não transformar suporte do núcleo em API inexistente.
    assert!(s.pedagogy_identity().is_err());
    let (a, sa) = Runtime::restore(
        &std::fs::read(dir.join("a.bin")).unwrap(),
        &sensors(7, 61),
        Access::default(),
    )
    .unwrap();
    let (mut b, sb) = Runtime::restore(
        &std::fs::read(dir.join("b.bin")).unwrap(),
        &sensors(8, 62),
        Access::default(),
    )
    .unwrap();
    let mut receiver = runtime(8, 62);
    let mut w = world(8, 62, 888, 20);
    let before = receiver.checkpoint(&sb).unwrap();
    assert!(attempt(&a, &mut receiver, &sb, 8, 62, &mut w).is_err());
    assert_eq!(receiver.checkpoint(&sb).unwrap(), before);
    assert_eq!(
        attempt(&b, &mut receiver, &sb, 8, 62, &mut w)
            .unwrap()
            .reason,
        PredictiveReason::Reached
    );
    let mut denied_world = world(8, 62, 888, 20);
    assert!(attempt(&a, &mut b, &sb, 8, 62, &mut denied_world).is_err());
    assert_eq!(denied_world.action_count(), 0);
    assert_eq!(
        a.checkpoint(&sa).unwrap(),
        std::fs::read(dir.join("a.bin")).unwrap()
    );
}

#[test]
fn removing_only_context_metadata_reproduces_pooling_without_changing_feedback() {
    let rows = dataset(false, false);
    let mut isolated = coordinator(&[61, 62]);
    let mut pooled = coordinator(&[61]);
    for original in &rows {
        assert_eq!(admit(&mut isolated, original), Admission::Admissible);
        let mut intervention = original.clone();
        intervention.context = 61;
        assert_eq!(intervention.bits, original.bits);
        assert_eq!(intervention.label, original.label);
        assert_eq!(intervention.evidence, original.evidence);
        assert_eq!(intervention.source, original.source);
        assert_eq!(admit(&mut pooled, &intervention), Admission::Admissible);
    }
    assert_eq!(
        rows.iter().map(|r| r.context).collect::<Vec<_>>(),
        vec![61, 61, 62, 62]
    );
    for bits in [[1, 0, 1, 0, 1, 0], [0, 1, 1, 0, 0, 1]] {
        for ctx in [61, 62] {
            assert!(matches!(
                query(&isolated, ctx, bits).decision,
                ConceptDecision::Supported(_)
            ));
            assert!(matches!(
                query(&pooled, 61, bits).decision,
                ConceptDecision::Abstain(_)
            ));
        }
    }
    println!("CONTEXT-only projection intervention: 4 supported scoped results → 4 pooled abstentions, identical feedback");
}
