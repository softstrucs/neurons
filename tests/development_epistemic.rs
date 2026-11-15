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

fn eid(id: u32) -> EpisodeId {
    EpisodeId::new(id).unwrap()
}
fn bad(bits: [u8; 6], actual: i32) -> (Runtime, Session) {
    let (h, s) = history();
    let mut r = runtime();
    r.inherit_consequence_contract(&h).unwrap();
    let mut w = world(999, &[(999, 7, actual)]);
    r.observe_attributes(&w, 7, reading(9, bits, 30)).unwrap();
    r.intervene(&mut w, &request(10, 7, 32)).unwrap();
    (r, s)
}
fn assess(r: &mut Runtime, m: &impl EpisodeMemory, s: &Session, bits: [u8; 6]) -> EpistemicReport {
    r.assess_epistemic(m, s, contract(s), Attributes::new(bits).unwrap())
        .unwrap()
}
fn task(from: i32) -> PlanningGoal {
    PlanningGoal {
        world: 501,
        revision: 1,
        context: 61,
        source: source(),
        from,
        to: 30,
    }
}
fn query() -> SelectiveQuery {
    SelectiveQuery {
        goal: task(888),
        conceptual_seed: Some(20),
    }
}
fn apply(
    r: &mut Runtime,
    m: &impl EpisodeMemory,
    s: &Session,
    bits: [u8; 6],
) -> (PredictivePursuit, World) {
    let mut w = world(888, &[(888, 7, 20), (20, 9, 30)]);
    let out = r
        .pursue_with_consequences(
            m,
            s,
            &mut w,
            PursuitGoal {
                objective: eid(80),
                task: task(888),
                max_actions: 2,
            },
            contract(s),
            reading(19, bits, 50),
            &[request(20, 7, 52), request(21, 9, 70)],
        )
        .unwrap();
    (out, w)
}
#[test]
fn distinguishes_insufficiency_candidate_refutation_and_expressive_limit() {
    let (h, s) = history();
    let mut r = runtime();
    assert_eq!(
        assess(&mut r, &h, &s, [0, 0, 0, 0, 0, 0]).state,
        EpistemicDiagnosis::InsufficientEvidence
    );
    let (m, s) = bad([0, 0, 0, 0, 0, 0], 77);
    let mut r = runtime();
    let report = assess(&mut r, &m, &s, [0, 0, 0, 0, 0, 0]);
    assert_eq!(report.state, EpistemicDiagnosis::HypothesisRefuted);
    assert_eq!(
        report.question,
        Some(EpistemicQuestion::AcquireIndependentEvidence)
    );
    let (m, s) = bad([1, 1, 1, 0, 0, 0], 77);
    let mut r = runtime();
    let report = assess(&mut r, &m, &s, [1, 1, 1, 0, 0, 0]);
    assert_eq!(report.state, EpistemicDiagnosis::ModelExpressiveness);
    assert_eq!(
        report.question,
        Some(EpistemicQuestion::ReviewAuthorizedRepresentation)
    );
    assert_eq!(report.evidence[0].trial.to, 77);
    assert_eq!(report.evaluated_version.number, 1);
    assert!(report.criterion_evaluations <= 23 * 8);
}
#[test]
fn conflict_with_identical_admitted_attributes_is_not_an_expressiveness_claim() {
    let (m, s) = bad([1, 0, 0, 0, 0, 0], 77);
    let mut r = runtime();
    let report = assess(&mut r, &m, &s, [1, 0, 0, 0, 0, 0]);
    assert_eq!(report.state, EpistemicDiagnosis::ObservationConflict);
    assert_eq!(
        report.question,
        Some(EpistemicQuestion::CheckSourceAndContext)
    );
}
#[test]
fn refutation_on_another_state_prevents_automatic_repetition_but_control_can_apply() {
    let (mut m, s) = bad([1, 1, 1, 0, 0, 0], 77);
    let mut sample = world(20, &[(20, 9, 30)]);
    m.intervene(&mut sample, &request(11, 9, 40)).unwrap();
    let bytes = m.checkpoint(&s).unwrap();
    let mut r = runtime();
    let (out, w) = apply(&mut r, &m, &s, [1, 1, 1, 0, 0, 0]);
    assert_eq!(out.reason, PredictiveReason::ConflictingEvidence);
    assert_eq!(w.action_count(), 0);
    assert_eq!(
        out.epistemic.unwrap().state,
        EpistemicDiagnosis::ModelExpressiveness
    );
    assert_eq!(m.checkpoint(&s).unwrap(), bytes);
    let (h, s) = history();
    let mut control = runtime();
    let (out, w) = apply(&mut control, &h, &s, [1, 1, 1, 0, 0, 0]);
    assert_eq!(out.reason, PredictiveReason::Reached);
    assert_eq!(w.action_count(), 2);
}
#[test]
fn selective_memory_recovers_counterexample_outside_goal_transition_closure() {
    let (h, s) = history();
    let (m, _) = bad([1, 1, 1, 0, 0, 0], 77);
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &s).unwrap();
    a.insert(eid(2), &m, &s).unwrap();
    let mut c = EpisodeCatalog::new();
    c.insert(1, a).unwrap();
    let mut r = runtime();
    let search = r
        .select_evidence(&c, &s, query(), RetrievalBudget::default())
        .unwrap();
    assert!(search.coverage().complete_for_goal);
    let report = assess(&mut r, search.memory(), &s, [1, 1, 1, 0, 0, 0]);
    assert_eq!(report.state, EpistemicDiagnosis::ModelExpressiveness);
    assert_eq!(report.evidence[0].origins, vec![eid(2)]);
    let (out, w) = apply(&mut r, search.memory(), &s, [1, 1, 1, 0, 0, 0]);
    assert_eq!(w.action_count(), 0);
    assert!(out.epistemic.unwrap().blocks_application());
}
#[test]
fn constraints_do_not_invent_authority_and_incomplete_checks_suspend() {
    let (m, s) = bad([1, 1, 1, 0, 0, 0], 77);
    let mut denied = runtime_with(Access::default(), Limits::default());
    assert!(denied
        .assess_epistemic(
            &m,
            &s,
            contract(&s),
            Attributes::new([1, 1, 1, 0, 0, 0]).unwrap()
        )
        .is_err());
    let mut limits = Limits::default();
    limits.attention_scans = 0;
    let mut r = runtime_with(access(), limits);
    assert_eq!(
        assess(&mut r, &m, &s, [1, 1, 1, 0, 0, 0]).state,
        EpistemicDiagnosis::ResourceLimited
    );
}
#[test]
fn unrelated_attributes_do_not_become_a_global_ban_on_investigation() {
    let (m, s) = bad([1, 1, 1, 0, 0, 0], 77);
    let mut r = runtime();
    assert_eq!(
        assess(&mut r, &m, &s, [0, 0, 0, 0, 0, 0]).state,
        EpistemicDiagnosis::InsufficientEvidence
    );
    let mut w = world(777, &[(777, 9, 55)]);
    assert!(r.intervene(&mut w, &request(30, 9, 60)).is_ok());
    assert_eq!(w.state(), 55);
}
#[test]
fn persistent_limitations_reconstruct_in_a_new_process_without_new_codec_or_authority() {
    let (m, s) = bad([1, 1, 1, 0, 0, 0], 77);
    let path = std::env::temp_dir().join(format!("neurons-epistemic-{}.bin", std::process::id()));
    assert!(!path.exists());
    let bytes = m.checkpoint(&s).unwrap();
    std::fs::write(&path, &bytes).unwrap();
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "epistemic_restore_worker", "--nocapture"])
        .env("NEURONS_EPISTEMIC_RESTORE", &path)
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
fn epistemic_restore_worker() {
    let Some(path) = std::env::var_os("NEURONS_EPISTEMIC_RESTORE") else {
        return;
    };
    let bytes = std::fs::read(path).unwrap();
    let (mut m, s) = Runtime::restore(&bytes, &sensors(), Access::default()).unwrap();
    assert!(m
        .assess_epistemic(
            &runtime(),
            &s,
            contract(&s),
            Attributes::new([1, 1, 1, 0, 0, 0]).unwrap()
        )
        .is_err());
    let mut fresh = runtime();
    let report = assess(&mut fresh, &m, &s, [1, 1, 1, 0, 0, 0]);
    assert_eq!(report.state, EpistemicDiagnosis::ModelExpressiveness);
    assert_eq!(report.evidence[0].after.source, source());
    assert_eq!(report.evidence[0].recorded_version.number, 1);
    assert_eq!(m.checkpoint(&s).unwrap(), bytes);
}

#[test]
fn conflicting_independent_observations_are_recovered_without_majority_voting() {
    let (h, s) = history();
    let (negative, _) = bad([1, 1, 1, 0, 0, 0], 77);
    let mut positive = runtime();
    positive.inherit_consequence_contract(&h).unwrap();
    let mut w = world(777, &[(777, 7, 20)]);
    positive
        .observe_attributes(&w, 7, reading(10, [1, 1, 1, 0, 0, 0], 30))
        .unwrap();
    positive.intervene(&mut w, &request(12, 7, 32)).unwrap();
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &h, &s).unwrap();
    a.insert(eid(2), &negative, &s).unwrap();
    a.insert(eid(3), &negative, &s).unwrap();
    a.insert(eid(4), &positive, &s).unwrap();
    let mut c = EpisodeCatalog::new();
    c.insert(1, a).unwrap();
    let mut r = runtime();
    let search = r
        .select_evidence(&c, &s, query(), RetrievalBudget::default())
        .unwrap();
    assert!(search.coverage().complete_for_goal);
    let report = assess(&mut r, search.memory(), &s, [1, 1, 1, 0, 0, 0]);
    assert_eq!(report.state, EpistemicDiagnosis::ObservationConflict);
    assert_eq!(
        report.evidence.iter().filter(|e| e.trial.to == 77).count(),
        1
    );
    assert_eq!(
        report
            .evidence
            .iter()
            .find(|e| e.trial.to == 77)
            .unwrap()
            .origins,
        vec![eid(2), eid(3)]
    );
}
#[test]
fn new_admissible_experience_allows_reevaluation_but_stale_views_are_rejected() {
    let (m, mut s) = bad([0, 0, 0, 0, 0, 0], 77);
    let mut r = runtime();
    let mut a = EpisodeArchive::new();
    a.insert(eid(1), &m, &s).unwrap();
    let mut c = EpisodeCatalog::new();
    c.insert(1, a).unwrap();
    let old = r
        .select_evidence(&c, &s, query(), RetrievalBudget::default())
        .unwrap();
    let report = assess(&mut r, old.memory(), &s, [0, 0, 0, 0, 0, 0]);
    assert_eq!(report.state, EpistemicDiagnosis::HypothesisRefuted);
    // Admissão explícita existente, fora da avaliação: não concedida pelo relatório.
    s.pedagogy_present(neurons::curriculum::Lesson {
        id: EngramId::new(502).unwrap(),
        receipt: 902,
        exemplar: n(902),
        attributes: Attributes::new([0, 0, 0, 0, 0, 0]).unwrap(),
        label: neurons::cognition::concepts::Label::Negative,
        source: source(),
        evidence: EvidenceId::new(4021).unwrap(),
    })
    .unwrap();
    s.pedagogy_admit(EngramId::new(502).unwrap()).unwrap();
    assert!(r
        .assess_epistemic(
            old.memory(),
            &s,
            contract(&s),
            Attributes::new([0, 0, 0, 0, 0, 0]).unwrap()
        )
        .is_err());
    let mut fresh = runtime();
    let current = fresh
        .select_evidence(&c, &s, query(), RetrievalBudget::default())
        .unwrap();
    let report = assess(&mut fresh, current.memory(), &s, [0, 0, 0, 0, 0, 0]);
    assert_eq!(report.state, EpistemicDiagnosis::NoCounterexample);
    assert_eq!(report.evaluated_version.number, 2);
    assert_eq!(report.evidence[0].recorded_version.number, 1);
}
