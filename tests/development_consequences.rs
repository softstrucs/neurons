//! Engenharia sintética prospectiva: aquisição separada dos casos de aplicação.
use neurons::cognition::concepts::{Attributes, Label};
use neurons::cognitive_contracts::*;
use neurons::curriculum::{PedagogyAssessment, PedagogyPermissions};
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::{Event, EventKind, NodeId};
use neurons::integration::{development::*, synaptic::SynapticEpisode};
use neurons::metacognition::concepts::ConceptDecision;
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
fn runtime(a: Access, limits: Limits) -> Runtime {
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
    use neurons::curriculum::PedagogyHost;
    ConsequenceContract {
        concept: n(601),
        scope: s.pedagogy_identity().unwrap().1,
        world: 501,
        revision: 1,
        source: source(),
        action: 7,
        target: 99,
    }
}
fn world(from: i32, to: i32) -> World {
    World::new(501, 1, sensors()[0].clone(), from, &[(from, 7, to)]).unwrap()
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
fn request(id: u32, at: u64) -> Intervention {
    Intervention {
        id,
        permit: ActionPermit {
            id: 1000 + id,
            world: 501,
            revision: 1,
            context: 61,
            source: source(),
            action: 7,
        },
        before_id: id * 2,
        after_id: id * 2 + 1,
        before_evidence: EvidenceId::new(4000 + id * 2).unwrap(),
        after_evidence: EvidenceId::new(4001 + id * 2).unwrap(),
        at,
    }
}
fn observed(
    r: &mut Runtime,
    id: u32,
    bits: [u8; 6],
    from: i32,
    to: i32,
    at: u64,
) -> AttributeFrame {
    let mut w = world(from, to);
    let f = r.observe_attributes(&w, 7, reading(id, bits, at)).unwrap();
    r.intervene(&mut w, &request(id, at + 2)).unwrap();
    f
}
fn assessment() -> PedagogyAssessment {
    PedagogyAssessment::new(
        &[Attributes::new([1, 0, 1, 0, 1, 0]).unwrap().into()],
        &[Attributes::new([0, 1, 0, 1, 0, 1]).unwrap().into()],
    )
    .unwrap()
}
fn permissions(c: ConsequenceContract, ids: &[u32]) -> PedagogyPermissions {
    PedagogyPermissions {
        evaluation: Some(c.scope),
        presentation: Some(c.scope),
        admissions: ids.iter().map(|i| EngramId::new(*i).unwrap()).collect(),
    }
}
fn acquired() -> (Runtime, Session) {
    let mut r = runtime(access(), Default::default());
    let mut s = session();
    let c = contract(&s);
    let f = observed(&mut r, 1, [1, 0, 0, 0, 0, 0], 10, 99, 0);
    let t = r
        .teach_consequences(
            &mut s,
            c,
            &[(f, 1, EngramId::new(501).unwrap(), 901)],
            assessment(),
            permissions(c, &[501]),
        )
        .unwrap();
    assert_eq!(t.pedagogy.final_version.number, 1);
    assert_eq!(t.pedagogy.lessons.iter().filter(|l| l.admitted).count(), 1);
    (r, s)
}
fn predict(r: &mut Runtime, s: &Session, f: &AttributeFrame) -> ConsequenceForecast {
    r.forecast_consequence(s, contract(s), f, EpisodeId::new(77).unwrap())
        .unwrap()
}
#[test]
fn unseen_attributes_and_state_predict_observed_consequence_without_target_table() {
    let (mut r, s) = acquired();
    let c = contract(&s);
    let before = s.encode();
    let mut w = world(4321, 99);
    let f = r
        .observe_attributes(&w, 7, reading(2, [1, 1, 1, 0, 0, 0], 10))
        .unwrap();
    let p = predict(&mut r, &s, &f);
    assert_eq!(
        p.report().decision,
        ConceptDecision::Supported(Label::Positive)
    );
    assert_eq!(p.contract(), c);
    let report = r
        .execute_forecast(&s, &mut w, p.clone(), &request(2, 12))
        .unwrap();
    assert_eq!(report.agreement, Some(true));
    assert!(!report.review_required);
    assert_eq!(report.trial.from, 4321);
    assert_eq!(report.trial.to, 99);
    assert!(r.review_consequence(&s, p, 2).is_err());
    assert_eq!(s.encode(), before);
    assert!(s.memory().is_empty());
    assert_eq!(r.cognitive_resources().conceptual_queries, 4);
    assert!(r.cognitive_resources().reserved_conceptual_queries <= 7);
    assert!(r.experiences().len() <= 8);
}
#[test]
fn control_without_experience_abstains_and_does_not_claim_success() {
    let mut r = runtime(access(), Default::default());
    let s = session();
    let mut w = world(4321, 99);
    let f = r
        .observe_attributes(&w, 7, reading(1, [1, 1, 1, 0, 0, 0], 0))
        .unwrap();
    let p = predict(&mut r, &s, &f);
    assert!(matches!(p.report().decision, ConceptDecision::Abstain(_)));
    r.intervene(&mut w, &request(1, 2)).unwrap();
    let report = r.review_consequence(&s, p, 1).unwrap();
    assert_eq!(report.agreement, None);
    assert!(report.review_required);
}
#[test]
fn unexpected_consequence_preserves_divergence_and_requests_strategy_review() {
    let (mut r, s) = acquired();
    let mut w = world(4321, -8);
    let f = r
        .observe_attributes(&w, 7, reading(2, [1, 1, 1, 0, 0, 0], 10))
        .unwrap();
    let p = predict(&mut r, &s, &f);
    r.intervene(&mut w, &request(2, 12)).unwrap();
    let report = r.review_consequence(&s, p, 2).unwrap();
    assert_eq!(report.agreement, Some(false));
    assert_eq!(report.actual, Label::Negative);
    assert!(report.review_required);
    assert_eq!(r.action_trials().len(), 2);
    assert_eq!(r.memory().get(report.trial.after).unwrap().value(), -8);
}
#[test]
fn two_real_experiences_revise_h23_and_distinguish_unseen_negative_attributes() {
    let mut r = runtime(access(), Default::default());
    let mut s = session();
    let c = contract(&s);
    let a = observed(&mut r, 1, [1, 0, 0, 0, 0, 0], 10, 99, 0);
    let b = observed(&mut r, 2, [0, 0, 0, 0, 0, 0], 20, -8, 10);
    let t = r
        .teach_consequences(
            &mut s,
            c,
            &[
                (a, 1, EngramId::new(501).unwrap(), 901),
                (b, 2, EngramId::new(502).unwrap(), 902),
            ],
            assessment(),
            permissions(c, &[501, 502]),
        )
        .unwrap();
    assert_eq!(t.pedagogy.initial_version.number, 0);
    assert_eq!(t.pedagogy.final_version.number, 2);
    assert_eq!(t.pedagogy.final_version.criteria.len(), 1);
    let w = world(777, 99);
    let f = r
        .observe_attributes(&w, 7, reading(3, [0, 1, 1, 0, 0, 0], 20))
        .unwrap();
    assert_eq!(
        predict(&mut r, &s, &f).report().decision,
        ConceptDecision::Supported(Label::Negative)
    );
}
#[test]
fn independent_permissions_deny_observation_query_and_teaching() {
    for field in 0..4 {
        let mut a = access();
        match field {
            0 => a.explore = false,
            1 => a.record = false,
            2 => a.read_knowledge = false,
            _ => a.learn_concepts = false,
        }
        let mut r = runtime(a, Default::default());
        let mut s = session();
        let c = contract(&s);
        let w = world(10, 99);
        if field < 2 {
            assert!(r
                .observe_attributes(&w, 7, reading(1, [1, 0, 0, 0, 0, 0], 0))
                .is_err());
            assert!(r.experiences().is_empty());
            continue;
        }
        let f = observed(&mut r, 1, [1, 0, 0, 0, 0, 0], 10, 99, 0);
        let before = s.encode();
        if field == 2 {
            assert!(r
                .forecast_consequence(&s, c, &f, EpisodeId::new(77).unwrap())
                .is_err());
        }
        assert!(r
            .teach_consequences(
                &mut s,
                c,
                &[(f, 1, EngramId::new(501).unwrap(), 901)],
                assessment(),
                permissions(c, &[501])
            )
            .is_err());
        assert_eq!(s.encode(), before);
    }
}
#[test]
fn admission_permission_and_presentation_are_independent() {
    for field in 0..3 {
        let mut r = runtime(access(), Default::default());
        let mut s = session();
        let c = contract(&s);
        let f = observed(&mut r, 1, [1, 0, 0, 0, 0, 0], 10, 99, 0);
        let before = s.encode();
        let mut p = permissions(c, &[501]);
        match field {
            0 => p.evaluation = None,
            1 => p.presentation = None,
            _ => p.admissions.clear(),
        }
        assert!(r
            .teach_consequences(
                &mut s,
                c,
                &[(f, 1, EngramId::new(501).unwrap(), 901)],
                assessment(),
                p
            )
            .is_err());
        assert_eq!(s.encode(), before);
    }
}
#[test]
fn mismatched_action_scope_revision_or_temporal_binding_is_rejected() {
    for field in 0..4 {
        let mut r = runtime(access(), Default::default());
        let mut s = session();
        let mut c = contract(&s);
        let f = observed(&mut r, 1, [1, 0, 0, 0, 0, 0], 10, 99, 0);
        match field {
            0 => c.action = 8,
            1 => c.revision = 2,
            2 => c.source = SourceId::new(9).unwrap(),
            _ => c.scope.context = ContextId::new(62).unwrap(),
        }
        let before = s.encode();
        assert!(r
            .teach_consequences(
                &mut s,
                c,
                &[(f, 1, EngramId::new(501).unwrap(), 901)],
                assessment(),
                permissions(c, &[501])
            )
            .is_err());
        assert_eq!(s.encode(), before);
    }
}
#[test]
fn insufficient_neural_reserve_refuses_before_action_and_keeps_world() {
    let mut r = runtime(
        access(),
        Limits {
            neural_events: 16,
            ..Default::default()
        },
    );
    let mut w = world(10, 99);
    r.observe_attributes(&w, 7, reading(1, [1, 0, 0, 0, 0, 0], 0))
        .unwrap();
    let before = r.memory().len();
    assert!(r.intervene(&mut w, &request(1, 2)).is_err());
    assert_eq!(w.state(), 10);
    assert_eq!(w.action_count(), 0);
    assert_eq!(r.memory().len(), before);
}
#[test]
fn checkpoint_retains_evidence_and_versions_but_not_prediction_authority() {
    let (mut r, s) = acquired();
    let w = world(4321, 99);
    let f = r
        .observe_attributes(&w, 7, reading(2, [1, 1, 1, 0, 0, 0], 10))
        .unwrap();
    let p = predict(&mut r, &s, &f);
    let bytes = r.checkpoint(&s).unwrap();
    let (mut restored, s2) = Runtime::restore(&bytes, &sensors(), access()).unwrap();
    assert_eq!(restored.checkpoint(&s2).unwrap(), bytes);
    assert!(restored.review_consequence(&s2, p, 1).is_err());
    assert!(restored
        .forecast_consequence(&s2, contract(&s2), &f, EpisodeId::new(78).unwrap())
        .is_err());
    assert_eq!(restored.action_trials(), r.action_trials());
}
#[test]
fn restored_concept_is_applied_in_another_process_with_fresh_permissions() {
    let (r, s) = acquired();
    let bytes = r.checkpoint(&s).unwrap();
    let path =
        std::env::temp_dir().join(format!("neurons-consequences-{}.bin", std::process::id()));
    assert!(!path.exists());
    std::fs::write(&path, bytes).unwrap();
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "consequence_restore_worker", "--nocapture"])
        .env("NEURONS_CONSEQUENCE_RESTORE", &path)
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
fn consequence_restore_worker() {
    let Some(path) = std::env::var_os("NEURONS_CONSEQUENCE_RESTORE") else {
        return;
    };
    let bytes = std::fs::read(path).unwrap();
    let (mut r, s) = Runtime::restore(&bytes, &sensors(), access()).unwrap();
    let mut w = world(4321, 99);
    let f = r
        .observe_attributes(&w, 7, reading(2, [1, 1, 1, 0, 0, 0], 10))
        .unwrap();
    let p = predict(&mut r, &s, &f);
    assert_eq!(
        p.report().decision,
        ConceptDecision::Supported(Label::Positive)
    );
    r.intervene(&mut w, &request(2, 12)).unwrap();
    assert_eq!(
        r.review_consequence(&s, p, 2).unwrap().agreement,
        Some(true)
    );
    let (mut denied, s) = Runtime::restore(&bytes, &sensors(), Access::default()).unwrap();
    assert!(denied
        .observe_attributes(&w, 7, reading(2, [1, 1, 1, 0, 0, 0], 10))
        .is_err());
    assert!(s.memory().is_empty());
}

#[test]
fn forecast_cannot_be_fabricated_after_observing_the_result() {
    let (mut r, s) = acquired();
    let mut w = world(4321, 99);
    let f = r
        .observe_attributes(&w, 7, reading(2, [1, 1, 1, 0, 0, 0], 10))
        .unwrap();
    r.intervene(&mut w, &request(2, 12)).unwrap();
    assert!(r
        .forecast_consequence(&s, contract(&s), &f, EpisodeId::new(77).unwrap())
        .is_err());
}
#[test]
fn stale_report_is_rejected_after_explicit_concept_revision() {
    use neurons::curriculum::{Lesson, PedagogyHost};
    let (mut r, mut s) = acquired();
    let mut w = world(4321, 99);
    let f = r
        .observe_attributes(&w, 7, reading(2, [1, 1, 1, 0, 0, 0], 10))
        .unwrap();
    let p = predict(&mut r, &s, &f);
    r.intervene(&mut w, &request(2, 12)).unwrap();
    // Ato supervisor externo explícito: invalida a versão usada pela previsão.
    s.pedagogy_present(Lesson {
        id: EngramId::new(502).unwrap(),
        receipt: 902,
        exemplar: n(902),
        attributes: Attributes::new([0, 0, 0, 0, 0, 0]).unwrap(),
        label: Label::Negative,
        source: source(),
        evidence: EvidenceId::new(4999).unwrap(),
    })
    .unwrap();
    s.pedagogy_admit(EngramId::new(502).unwrap()).unwrap();
    assert!(r.review_consequence(&s, p.clone(), 2).is_err());
    assert!(r.review_consequence(&s, p, 2).is_err());
}
#[test]
fn review_of_insufficiency_can_refine_knowledge_in_a_new_authorized_episode() {
    use neurons::curriculum::PedagogyHost;
    let (mut first, mut s) = acquired();
    let mut w = world(20, -8);
    let f = first
        .observe_attributes(&w, 7, reading(2, [0, 0, 0, 0, 0, 0], 10))
        .unwrap();
    let forecast = predict(&mut first, &s, &f);
    assert!(matches!(
        forecast.report().decision,
        ConceptDecision::Abstain(_)
    ));
    first.intervene(&mut w, &request(2, 12)).unwrap();
    assert!(
        first
            .review_consequence(&s, forecast, 2)
            .unwrap()
            .review_required
    );
    // Sem reset de first: outra instância, sensores e autoridade novos, mesma
    // memória conceitual, nova experiência. O episódio anterior continua gasto.
    let mut next = runtime(access(), Default::default());
    next.inherit_consequence_contract(&first).unwrap();
    let c = contract(&s);
    let f = observed(&mut next, 3, [0, 0, 0, 0, 0, 0], 20, -8, 20);
    let review_assessment = PedagogyAssessment::new(
        &[Attributes::new([0, 1, 0, 1, 0, 0]).unwrap().into()],
        &[Attributes::new([1, 1, 0, 1, 0, 0]).unwrap().into()],
    )
    .unwrap();
    let training = next
        .teach_consequences(
            &mut s,
            c,
            &[(f, 3, EngramId::new(502).unwrap(), 902)],
            review_assessment,
            permissions(c, &[502]),
        )
        .unwrap();
    assert_eq!(training.pedagogy.initial_version.number, 1);
    assert_eq!(training.pedagogy.final_version.number, 2);
    assert_eq!(s.pedagogy_version().unwrap().criteria.len(), 1);
    let w = world(800, 0);
    let f = next
        .observe_attributes(&w, 7, reading(4, [0, 1, 1, 0, 0, 0], 30))
        .unwrap();
    assert_eq!(
        predict(&mut next, &s, &f).report().decision,
        ConceptDecision::Supported(Label::Negative)
    );
    assert!(first
        .forecast_consequence(&s, c, &f, EpisodeId::new(78).unwrap())
        .is_err());
}
#[test]
fn unrelated_attribute_frame_cannot_label_an_intervention() {
    let mut r = runtime(access(), Default::default());
    let mut s = session();
    let c = contract(&s);
    let w = world(10, 99);
    let old = r
        .observe_attributes(&w, 7, reading(9, [0, 1, 0, 0, 0, 0], 0))
        .unwrap();
    observed(&mut r, 1, [1, 0, 0, 0, 0, 0], 10, 99, 3);
    let before = s.encode();
    assert!(r
        .teach_consequences(
            &mut s,
            c,
            &[(old, 1, EngramId::new(501).unwrap(), 901)],
            assessment(),
            permissions(c, &[501])
        )
        .is_err());
    assert_eq!(s.encode(), before);
}

#[test]
fn acquired_hypothesis_cannot_be_retargeted_even_after_restore() {
    let (mut r, s) = acquired();
    let w = world(700, 99);
    let f = r
        .observe_attributes(&w, 7, reading(2, [1, 1, 1, 0, 0, 0], 10))
        .unwrap();
    let bytes = r.checkpoint(&s).unwrap();
    assert_eq!(&bytes[..8], b"NRNPIFI3");
    let (mut restored, s2) = Runtime::restore(&bytes, &sensors(), access()).unwrap();
    for r in [&mut r, &mut restored] {
        let mut c = contract(&s2);
        c.target = -8;
        assert!(r
            .forecast_consequence(&s2, c, &f, EpisodeId::new(77).unwrap())
            .is_err());
    }
}
#[test]
fn conflicting_transition_history_blocks_action_despite_positive_h23() {
    let (mut r, s) = acquired();
    observed(&mut r, 2, [1, 1, 0, 0, 0, 0], 10, -8, 10);
    let mut w = world(10, 99);
    let f = r
        .observe_attributes(&w, 7, reading(3, [1, 1, 1, 0, 0, 0], 20))
        .unwrap();
    let p = predict(&mut r, &s, &f);
    assert_eq!(
        p.report().decision,
        ConceptDecision::Supported(Label::Positive)
    );
    assert_eq!(p.transitions().status, PredictionStatus::Conflicting);
    let before = r.action_trials().len();
    assert!(r.execute_forecast(&s, &mut w, p, &request(3, 22)).is_err());
    assert_eq!(w.action_count(), 0);
    assert_eq!(r.action_trials().len(), before);
}
#[test]
fn tampered_consequence_envelope_is_rejected_and_old_formats_stay_distinct() {
    let (r, s) = acquired();
    let bytes = r.checkpoint(&s).unwrap();
    for pos in [0, 8, bytes.len() - 16, bytes.len() - 1] {
        let mut changed = bytes.clone();
        changed[pos] ^= 1;
        assert!(Runtime::restore(&changed, &sensors(), access()).is_err());
    }
    let old = runtime(access(), Default::default())
        .checkpoint(&session())
        .unwrap();
    assert_eq!(&old[..8], b"NRNPIFI1");
    let (r, s) = Runtime::restore(&old, &sensors(), access()).unwrap();
    assert_eq!(r.checkpoint(&s).unwrap(), old);
}
#[test]
fn stale_hypothesis_stops_adapter_before_execution() {
    use neurons::curriculum::{Lesson, PedagogyHost};
    let (mut r, mut s) = acquired();
    let mut w = world(4321, 99);
    let f = r
        .observe_attributes(&w, 7, reading(2, [1, 1, 1, 0, 0, 0], 10))
        .unwrap();
    let p = predict(&mut r, &s, &f);
    s.pedagogy_present(Lesson {
        id: EngramId::new(502).unwrap(),
        receipt: 902,
        exemplar: n(902),
        attributes: Attributes::new([0, 0, 0, 0, 0, 0]).unwrap(),
        label: Label::Negative,
        source: source(),
        evidence: EvidenceId::new(4999).unwrap(),
    })
    .unwrap();
    s.pedagogy_admit(EngramId::new(502).unwrap()).unwrap();
    assert!(r.execute_forecast(&s, &mut w, p, &request(2, 12)).is_err());
    assert_eq!(w.action_count(), 0);
}

#[test]
fn comparison_distinguishes_exact_transition_from_conceptual_target() {
    let (mut r, s) = acquired();
    let mut w = world(10, 98);
    let f = r
        .observe_attributes(&w, 7, reading(2, [1, 1, 0, 0, 0, 0], 10))
        .unwrap();
    let p = predict(&mut r, &s, &f);
    assert_eq!(p.transitions().status, PredictionStatus::Observed);
    assert_eq!(p.transitions().outcomes[0].to, 99);
    let review = r.execute_forecast(&s, &mut w, p, &request(2, 12)).unwrap();
    assert_eq!(review.agreement, Some(false));
    assert_eq!(review.transition_agreement, Some(false));
    assert!(review.review_required);
    assert_eq!(r.action_trials().last().unwrap().to, 98);
}
