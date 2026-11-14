//! Testes funcionais públicos próprios. O oráculo fica apenas neste executável.
use neurons::cognition::concepts::{Attributes, Label, Observation};
use neurons::cognitive_contracts::*;
use neurons::curriculum::{Lesson, PedagogyPermissions};
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::{Event, EventKind, NodeId};
use neurons::integration::composed::*;
use neurons::integration::experiential::*;
use neurons::integration::query::PathAuthorization;
use neurons::metacognition::concepts::{ConceptDecision, ConceptStrategy, PedagogicalDiagnosis};
use neurons::session::f1::Session;
use std::process::Command;

fn n(v: u32) -> NodeId {
    NodeId::new(v).unwrap()
}
fn eid(v: u32) -> EpisodeId {
    EpisodeId::new(v).unwrap()
}
fn scope() -> Scope {
    Scope {
        catalog: CatalogId::new(391).unwrap(),
        context: ContextId::new(341).unwrap(),
        rule: RuleRef {
            id: RuleId::new(351).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 10000).unwrap(),
    }
}
fn attributes(bits: &str) -> Attributes {
    Attributes::new(
        bits.as_bytes()
            .try_into()
            .map(|a: [u8; 6]| a.map(|v| v - b'0'))
            .unwrap(),
    )
    .unwrap()
}
fn obs(bits: &str) -> Observation {
    attributes(bits).into()
}
fn apply(s: &mut Session, c: &str) {
    s.execute(c).unwrap_or_else(|e| panic!("{c}: {e}"));
}
fn relation(s: &mut Session, id: u32, from: u32, to: u32, sign: &str) {
    for c in [
        format!(
            "receber {id} 341 {from} {to} {sign} 8 {} observacao",
            id + 4000
        ),
        "interpretar".into(),
        "avaliar".into(),
        "admitir".into(),
    ] {
        apply(s, &c);
    }
}
fn base(relations: bool) -> Session {
    let mut s = Session::new();
    for c in [
        "config 391 341 351 1 0 10000",
        "simbolo 3101 c",
        "simbolo 3103 b",
        "simbolo 3105 d",
        "autorizar 341",
        "conceito 3501 opaco",
        "exemplo 3401 3601 341 111111 + 7 4401 observacao",
        "interpretar-exemplo",
        "avaliar-exemplo",
        "admitir-exemplo 3401",
        "revisar",
    ] {
        apply(&mut s, c);
    }
    if relations {
        relation(&mut s, 3111, 3101, 3103, "+");
        relation(&mut s, 3112, 3103, 3105, "+");
    }
    apply(&mut s, "agora 100");
    s
}
fn binding(bits: &str) -> Binding {
    Binding {
        id: eid(3801),
        item: n(39010),
        observation: obs(bits),
        concept: n(3501),
        relational_concept: n(3101),
        destination: n(3105),
        scope: scope(),
        source: SourceId::new(9).unwrap(),
        evidence: EvidenceId::new(49901).unwrap(),
        origin: BindingOrigin::ExternalSyntheticObservation,
    }
}
fn lessons() -> Vec<Lesson> {
    [(3402, "011111"), (3403, "100000"), (3404, "111110")]
        .iter()
        .map(|(id, bits)| Lesson {
            id: EngramId::new(*id).unwrap(),
            receipt: *id,
            exemplar: n(id + 1000),
            attributes: attributes(bits),
            label: Label::Negative,
            source: SourceId::new(7).unwrap(),
            evidence: EvidenceId::new(id + 1000).unwrap(),
        })
        .collect()
}
fn source() -> LocalExperiences {
    LocalExperiences::new(SourceId::new(7).unwrap(), n(3501), scope(), &lessons()).unwrap()
}
fn authority(b: &Binding) -> Authority {
    Authority {
        observation: true,
        query: Permissions {
            conceptual: Some(scope()),
            relational: Some(PathAuthorization::inclusion_three(scope())),
            composition: Some(b.clone()),
        },
        investigate: true,
        source: Some(SourceId::new(7).unwrap()),
        pedagogy: PedagogyPermissions {
            evaluation: Some(scope()),
            presentation: Some(scope()),
            admissions: lessons().iter().map(|l| l.id).collect(),
        },
    }
}
fn event(s: &Session, b: &Binding) -> Event {
    let payload = 1 + attributes_from(b)
        .values()
        .iter()
        .enumerate()
        .fold(0, |x, (i, v)| x | ((i32::from(*v)) << i));
    Event::new(
        None,
        b.concept,
        EventKind::Stimulus,
        payload,
        s.logical_time(),
    )
}
fn attributes_from(b: &Binding) -> Attributes {
    b.observation.complete().unwrap()
}
fn run(s: &mut Session, a: Authority, source: &mut LocalExperiences) -> CycleReport {
    let b = binding("110001");
    let e = event(s, &b);
    Cycle::new(eid(39001), b.clone(), a, Budget::default())
        .run(
            s,
            e,
            &SyntheticRegistry::new(b, 1).unwrap(),
            source,
            obs("010101"),
        )
        .unwrap()
}
fn learned() -> Session {
    let mut s = base(true);
    let r = run(&mut s, authority(&binding("110001")), &mut source());
    assert_eq!(r.result, Status::Supported);
    s
}
fn evaluate(s: &Session, bits: &str) -> Report {
    let b = binding(bits);
    let mut e = neurons::integration::composed_two::Episode::new(
        eid(39003),
        b.clone(),
        authority(&b).query,
        Default::default(),
    )
    .unwrap();
    let registry = SyntheticRegistry::new(b, 1).unwrap();
    let r = e.query(s, &registry).unwrap();
    assert_eq!(
        e.verify(s, &registry, &r).unwrap(),
        Verification::Verified(r.status)
    );
    r
}

#[test]
fn integrated_cycle_has_real_gap_question_experience_proof_and_versions() {
    let mut s = base(true);
    let initial = s.encode();
    let facts = s.memory().iter().collect::<Vec<_>>();
    let r = run(&mut s, authority(&binding("110001")), &mut source());
    assert_eq!(r.initial.status, Status::Insufficient);
    assert_eq!(r.diagnosis, PedagogicalDiagnosis::ExperienceNeeded);
    assert_eq!(r.strategy, ConceptStrategy::TeachOnce);
    let q = r.question.as_ref().unwrap();
    assert_eq!(q.objective, r.objective);
    assert_eq!(q.version, 1);
    assert!(q.remaining_hypotheses > 1);
    assert_eq!(r.source_calls, 1);
    assert!(r.committed);
    let t = r.teaching.as_ref().unwrap();
    // MP09 seleciona os dois exemplos informativos; o terceiro é redundante.
    assert_eq!(t.lessons.len(), 2);
    assert!(t.lessons.iter().all(|l| l.admitted));
    assert_eq!(t.final_version.number, 3);
    assert_eq!(
        r.final_verification,
        Some(Verification::Verified(Status::Supported))
    );
    let final_report = r.final_report.as_ref().unwrap();
    assert_eq!(final_report.binding, r.initial.binding);
    assert_eq!(final_report.objective, r.objective);
    assert_eq!(
        final_report
            .relational
            .as_ref()
            .unwrap()
            .proof
            .as_ref()
            .unwrap()
            .premises
            .iter()
            .map(|p| p.engram.id().get())
            .collect::<Vec<_>>(),
        vec![3111, 3112]
    );
    assert_eq!(s.memory().iter().collect::<Vec<_>>(), facts);
    assert_ne!(s.encode(), initial);
    assert_eq!(
        (
            r.usage.conceptual_queries,
            r.usage.relational_queries,
            r.usage.neural_events,
            r.usage.conceptual_events
        ),
        (6, 4, 12, 10)
    );
    println!("ciclo: {:?} -> {:?}; fonte={}, ensino={}, consultas={}/{}, eventos neurais={}, versão=1->3",r.initial.status,r.result,r.source_calls,t.lessons.len(),r.usage.conceptual_queries,r.usage.relational_queries,r.usage.neural_events);
}
#[test]
fn no_experience_control_cannot_learn() {
    let mut s = base(true);
    let bytes = s.encode();
    let mut a = authority(&binding("110001"));
    a.investigate = false;
    let mut src = source();
    let r = run(&mut s, a, &mut src);
    assert_eq!(r.result, Status::Insufficient);
    assert!(!src.consumed());
    assert_eq!(r.source_calls, 0);
    assert_eq!(s.encode(), bytes);
}
#[test]
fn already_supported_avoids_teaching() {
    let mut s = learned();
    let bytes = s.encode();
    let mut src = source();
    let r = run(&mut s, authority(&binding("110001")), &mut src);
    assert_eq!(r.strategy, ConceptStrategy::Respond);
    assert!(!src.consumed());
    assert_eq!(s.encode(), bytes);
}
#[test]
fn missing_relation_cannot_trigger_conceptual_teaching() {
    let mut s = base(false);
    let bytes = s.encode();
    let mut src = source();
    let r = run(&mut s, authority(&binding("110001")), &mut src);
    assert_eq!(r.result, Status::Insufficient);
    assert!(!src.consumed());
    assert_eq!(s.encode(), bytes);
}
#[test]
fn opposition_stops_before_experience() {
    let mut s = base(true);
    relation(&mut s, 3113, 3101, 3105, "-");
    let bytes = s.encode();
    let mut src = source();
    let r = run(&mut s, authority(&binding("110001")), &mut src);
    assert_eq!(r.result, Status::Contradictory);
    assert!(!src.consumed());
    assert_eq!(s.encode(), bytes);
}
#[test]
fn independent_query_permissions_denied_before_cognitive_calls() {
    for field in 0..3 {
        let mut s = base(true);
        let bytes = s.encode();
        let mut a = authority(&binding("110001"));
        match field {
            0 => a.query.conceptual = None,
            1 => a.query.relational = None,
            _ => a.query.composition = None,
        };
        let mut src = source();
        let r = run(&mut s, a, &mut src);
        assert_eq!(r.usage.conceptual_queries, 0);
        assert_eq!(r.usage.relational_queries, 0);
        assert!(!src.consumed());
        assert_eq!(s.encode(), bytes);
    }
}
#[test]
fn independent_investigation_and_pedagogical_authority() {
    for field in 0..5 {
        let mut s = base(true);
        let bytes = s.encode();
        let mut a = authority(&binding("110001"));
        match field {
            0 => a.source = None,
            1 => a.investigate = false,
            2 => a.pedagogy.evaluation = None,
            3 => a.pedagogy.presentation = None,
            _ => a.pedagogy.admissions.clear(),
        };
        let mut src = source();
        let r = run(&mut s, a, &mut src);
        assert_eq!(r.source_calls, 0);
        assert!(!src.consumed());
        assert_eq!(s.encode(), bytes);
    }
}
#[test]
fn insufficient_budget_never_starts_partial_cycle() {
    for field in 0..4 {
        let mut s = base(true);
        let bytes = s.encode();
        let b = binding("110001");
        let a = authority(&b);
        let e = event(&s, &b);
        let registry = SyntheticRegistry::new(b.clone(), 1).unwrap();
        let mut budget = Budget::default();
        match field {
            0 => budget.conceptual_queries -= 1,
            1 => budget.relational_queries -= 1,
            2 => budget.neural_events -= 1,
            _ => budget.conceptual_events -= 1,
        };
        let mut src = source();
        assert!(Cycle::new(eid(39001), b, a, budget)
            .run(&mut s, e, &registry, &mut src, obs("010101"))
            .is_err());
        assert!(!src.consumed());
        assert_eq!(s.encode(), bytes);
    }
}
#[test]
fn malformed_or_stale_event_and_denied_observation() {
    for field in 0..4 {
        let mut s = base(true);
        let bytes = s.encode();
        let b = binding("110001");
        let mut a = authority(&b);
        let e = match field {
            0 => Event::new(None, b.concept, EventKind::Stimulus, 0, 100),
            1 => Event::new(
                None,
                b.concept,
                EventKind::Stimulus,
                event(&s, &b).payload(),
                99,
            ),
            2 => Event::new(
                Some(n(4)),
                b.concept,
                EventKind::Stimulus,
                event(&s, &b).payload(),
                100,
            ),
            _ => {
                a.observation = false;
                event(&s, &b)
            }
        };
        let registry = SyntheticRegistry::new(b.clone(), 1).unwrap();
        let mut src = source();
        assert!(Cycle::new(eid(39001), b, a, Budget::default())
            .run(&mut s, e, &registry, &mut src, obs("010101"))
            .is_err());
        assert_eq!(s.encode(), bytes);
        assert!(!src.consumed());
    }
}
#[test]
fn held_out_inputs_match_external_rule_without_evaluation_labels_in_candidate() {
    let s = learned();
    let bytes = s.encode();
    // Chave exclusivamente externa: conjunção de dois atributos, não entregue ao núcleo.
    for bits in ["101001", "111001", "110000", "010101"] {
        assert!(!["111111", "011111", "100000", "111110"].contains(&bits));
        let truth = bits.as_bytes()[0] == b'1' && bits.as_bytes()[5] == b'1';
        let r = evaluate(&s, bits);
        assert_eq!(
            r.conceptual.unwrap().decision,
            ConceptDecision::Supported(Label::from_bool(truth))
        );
        assert_eq!(
            r.status,
            if truth {
                Status::Supported
            } else {
                Status::Insufficient
            }
        );
    }
    assert_eq!(s.encode(), bytes);
}
#[test]
fn source_cannot_be_reused_as_fresh_authority() {
    let mut src = source();
    let mut first = base(true);
    run(&mut first, authority(&binding("110001")), &mut src);
    assert!(src.consumed());
    let mut second = base(true);
    let bytes = second.encode();
    let r = run(&mut second, authority(&binding("110001")), &mut src);
    assert_eq!(r.source_calls, 0);
    assert_eq!(second.encode(), bytes);
}
#[test]
fn teaching_evaluation_overlap_is_rejected_before_fetch() {
    let mut s = base(true);
    let bytes = s.encode();
    let b = binding("110001");
    let e = event(&s, &b);
    let registry = SyntheticRegistry::new(b.clone(), 1).unwrap();
    let mut src = source();
    assert!(
        Cycle::new(eid(39001), b.clone(), authority(&b), Budget::default())
            .run(&mut s, e, &registry, &mut src, obs("100000"))
            .is_err()
    );
    assert_eq!(s.encode(), bytes);
    assert!(!src.consumed());
}
#[test]
fn wrong_source_cannot_be_substituted() {
    let mut s = base(true);
    let bytes = s.encode();
    let mut rows = lessons();
    for l in &mut rows {
        l.source = SourceId::new(8).unwrap();
    }
    let mut src =
        LocalExperiences::new(SourceId::new(8).unwrap(), n(3501), scope(), &rows).unwrap();
    let r = run(&mut s, authority(&binding("110001")), &mut src);
    assert_eq!(r.source_calls, 0);
    assert_eq!(s.encode(), bytes);
}
#[test]
fn deterministic_trace() {
    let mut a = base(true);
    let mut b = base(true);
    let x = run(&mut a, authority(&binding("110001")), &mut source());
    let y = run(&mut b, authority(&binding("110001")), &mut source());
    assert_eq!(x, y);
    assert_eq!(a.encode(), b.encode());
}

#[test]
fn irrelevant_experience_does_not_fabricate_progress() {
    let mut s = base(true);
    let bytes = s.encode();
    let mut redundant = lessons()[0];
    redundant.attributes = attributes("111111");
    redundant.label = Label::Positive;
    let mut src =
        LocalExperiences::new(SourceId::new(7).unwrap(), n(3501), scope(), &[redundant]).unwrap();
    let r = run(&mut s, authority(&binding("110001")), &mut src);
    assert_eq!(r.source_calls, 1);
    assert_eq!(r.result, Status::Insufficient);
    assert!(r.teaching.unwrap().lessons.is_empty());
    assert_eq!(r.usage.conceptual_events, 0);
    assert_eq!(s.encode(), bytes);
}

#[test]
fn replay_cannot_hide_previous_cognitive_queries_in_the_budget() {
    let mut s = base(true);
    apply(&mut s, "consultar 341 3101 3105");
    let before = s.encode();
    let b = binding("110001");
    let e = event(&s, &b);
    let mut src = source();
    let error = Cycle::new(eid(39001), b.clone(), authority(&b), Budget::default())
        .run(
            &mut s,
            e,
            &SyntheticRegistry::new(b, 1).unwrap(),
            &mut src,
            obs("010101"),
        )
        .unwrap_err();
    assert!(error.contains("histórico sem consultas"));
    assert!(!src.consumed());
    assert_eq!(s.encode(), before);
}

#[test]
fn final_independence_failure_discards_staged_learning() {
    let mut s = base(true);
    let before = s.encode();
    let mut rows = lessons();
    rows[0].evidence = EvidenceId::new(7111).unwrap(); // evidência da primeira premissa
    let mut src =
        LocalExperiences::new(SourceId::new(7).unwrap(), n(3501), scope(), &rows).unwrap();
    let r = run(&mut s, authority(&binding("110001")), &mut src);
    assert!(src.consumed());
    assert_eq!(r.source_calls, 1);
    assert!(r
        .teaching
        .as_ref()
        .unwrap()
        .lessons
        .iter()
        .any(|l| l.admitted));
    assert!(!r.committed);
    assert_eq!(r.result, Status::Interrupted);
    assert_eq!(r.final_report.unwrap().status, Status::Denied);
    assert_eq!(s.encode(), before);
}

#[test]
fn learner_is_not_hardcoded_to_a_particular_attribute_pair() {
    // Família pública de controles funcionais: índices só geram experiências
    // e gabaritos no teste; não são parâmetros do coordenador/aprendiz.
    for i in 0..6 {
        for j in i + 1..6 {
            let mut s = base(true);
            let mut goal = [0u8; 6];
            goal[i] = 1;
            goal[j] = 1;
            let mut b = binding("110001");
            b.observation = Attributes::new(goal).unwrap().into();
            let mut rows = lessons()[..2].to_vec();
            for (row, index) in rows.iter_mut().zip([i, j]) {
                let mut bits = [1u8; 6];
                bits[index] = 0;
                row.attributes = Attributes::new(bits).unwrap();
            }
            let mut src =
                LocalExperiences::new(SourceId::new(7).unwrap(), n(3501), scope(), &rows).unwrap();
            let event = event(&s, &b);
            let r = Cycle::new(eid(39001), b.clone(), authority(&b), Budget::default())
                .run(
                    &mut s,
                    event,
                    &SyntheticRegistry::new(b, 1).unwrap(),
                    &mut src,
                    obs("000000"),
                )
                .unwrap();
            assert_eq!(r.initial.status, Status::Insufficient);
            assert_eq!(r.result, Status::Supported);
            let extra = (0..6).find(|k| *k != i && *k != j).unwrap();
            goal[extra] = 1;
            for positive in [true, false] {
                if !positive {
                    goal[i] = 0;
                }
                let bits: String = goal.iter().map(|v| char::from(b'0' + *v)).collect();
                let got = evaluate(&s, &bits);
                assert_eq!(
                    got.conceptual.unwrap().decision,
                    ConceptDecision::Supported(Label::from_bool(positive))
                );
            }
        }
    }
}
#[test]
fn restored_process_worker() {
    let Some(path) = std::env::var_os("NEURONS_FUNCTIONAL_CHECKPOINT") else {
        return;
    };
    let mut s = Session::load(std::path::Path::new(&path)).unwrap();
    let bytes = s.encode();
    assert_eq!(evaluate(&s, "101001").status, Status::Supported);
    // Conhecimento restaurado não fornece concessão composta transitória.
    let mut denied = authority(&binding("110001"));
    denied.query.composition = None;
    let r = run(&mut s, denied, &mut source());
    assert_eq!(r.usage.conceptual_queries, 0);
    assert_eq!(s.encode(), bytes);
    println!("retencao-em-novo-processo: sustentado; autoridade transitória negada");
}
#[test]
fn retention_in_genuinely_new_process() {
    let s = learned();
    let dir = std::env::temp_dir().join(format!(
        "neurons-functional-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("knowledge.f1");
    s.save(&path).unwrap();
    let child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "restored_process_worker", "--nocapture"])
        .env("NEURONS_FUNCTIONAL_CHECKPOINT", &path)
        .output()
        .unwrap();
    assert!(
        child.status.success(),
        "{}{}",
        String::from_utf8_lossy(&child.stdout),
        String::from_utf8_lossy(&child.stderr)
    );
    assert!(String::from_utf8_lossy(&child.stdout).contains("retencao-em-novo-processo"));
    println!("{}", String::from_utf8_lossy(&child.stdout));
}
