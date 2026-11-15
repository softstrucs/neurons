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
fn conflict(bits: [u8; 6]) -> (Runtime, Session) {
    let (h, s) = history();
    let mut m = runtime();
    m.inherit_consequence_contract(&h).unwrap();
    let mut w = world(999, &[(999, 7, 77)]);
    m.observe_attributes(&w, 7, reading(9, bits, 30)).unwrap();
    m.intervene(&mut w, &request(10, 7, 32)).unwrap();
    (m, s)
}
fn grant(s: &Session) -> PredictiveRevision {
    let c = contract(s);
    PredictiveRevision {
        record: EngramId::new(502).unwrap(),
        receipt: 902,
        assessment: PedagogyAssessment::new(
            &[Attributes::new([0, 1, 0, 1, 0, 0]).unwrap().into()],
            &[Attributes::new([1, 1, 0, 1, 0, 0]).unwrap().into()],
        )
        .unwrap(),
        permissions: PedagogyPermissions {
            evaluation: Some(c.scope),
            presentation: Some(c.scope),
            admissions: vec![EngramId::new(502).unwrap()],
        },
    }
}
fn investigate(
    r: &mut Runtime,
    m: &impl EpisodeMemory,
    s: &mut Session,
    actual: i32,
    teach: bool,
) -> (DiscriminatingInvestigation, World, World) {
    let mut boring = world(777, &[(777, 7, 20)]);
    let mut useful = world(888, &[(888, 7, actual)]);
    let c = contract(s);
    let authority = teach.then(|| grant(s));
    let out = r
        .investigate_disagreement(
            m,
            s,
            c,
            Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
            &mut [
                InvestigationOffer {
                    world: &mut boring,
                    attributes: reading(20, [1, 1, 1, 0, 0, 0], 50),
                    intervention: request(20, 7, 52),
                },
                InvestigationOffer {
                    world: &mut useful,
                    attributes: reading(21, [0, 0, 0, 0, 0, 0], 50),
                    intervention: request(21, 7, 52),
                },
            ],
            authority,
        )
        .unwrap();
    (out, boring, useful)
}
#[test]
fn conflict_generates_discriminating_question_and_result_drives_mp09_revision() {
    let (m, mut s) = conflict([1, 0, 0, 0, 0, 0]);
    let mut r = runtime();
    let (out, boring, useful) = investigate(&mut r, &m, &mut s, 77, true);
    assert_eq!(out.origin.state, EpistemicDiagnosis::ObservationConflict);
    assert_eq!(out.selected, Some(121));
    assert_eq!(out.options[0].separation, 0);
    assert_eq!(out.options[1].separation, 1);
    assert_eq!(boring.action_count(), 0);
    assert_eq!(useful.action_count(), 1);
    assert_eq!(out.result, InvestigationResult::ConflictRetained);
    assert_eq!(out.review.unwrap().trial.to, 77);
    assert_eq!(out.retained.len(), 1);
    assert_eq!(out.refuted.len(), 1);
    assert_eq!(out.teaching.unwrap().pedagogy.final_version.number, 2);
    assert!(out.requires_retrieval);
    assert_eq!(s.pedagogy_version().unwrap().number, 2);
    assert!(r.cognitive_resources().conceptual_queries <= 7);
    assert!(r.attention_scans() <= 32);
    assert_eq!(r.experiences().len(), 3);
}
#[test]
fn hidden_outcomes_do_not_change_selection_and_different_results_change_revision() {
    let mut results = vec![];
    for actual in [20, 77] {
        let (m, mut s) = conflict([1, 0, 0, 0, 0, 0]);
        let mut r = runtime();
        let (out, _, w) = investigate(&mut r, &m, &mut s, actual, true);
        assert_eq!(out.selected, Some(121));
        assert_eq!(w.state(), actual);
        results.push(out.retained);
    }
    assert_ne!(results[0], results[1]);
}
#[test]
fn observation_without_pedagogy_grant_keeps_alternatives_provisional_and_conflict_present() {
    let (m, mut s) = conflict([1, 0, 0, 0, 0, 0]);
    let before = s.encode();
    let mut r = runtime();
    let (out, _, _) = investigate(&mut r, &m, &mut s, 77, false);
    assert_eq!(s.encode(), before);
    assert!(out.teaching.is_none());
    assert!(!out.requires_retrieval);
    assert_eq!(
        out.after.unwrap().state,
        EpistemicDiagnosis::ObservationConflict
    );
    assert_eq!(out.retained.len(), 1);
    assert_eq!(r.action_attempts(), 1);
}
#[test]
fn non_discriminating_experiment_is_not_executed() {
    let (m, mut s) = conflict([1, 0, 0, 0, 0, 0]);
    let before = s.encode();
    let mut r = runtime();
    let mut w = world(888, &[(888, 7, 77)]);
    let c = contract(&s);
    let out = r
        .investigate_disagreement(
            &m,
            &mut s,
            c,
            Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
            &mut [InvestigationOffer {
                world: &mut w,
                attributes: reading(20, [1, 1, 1, 0, 0, 0], 50),
                intervention: request(20, 7, 52),
            }],
            None,
        )
        .unwrap();
    assert_eq!(out.result, InvestigationResult::NoDiscriminatingOffer);
    assert_eq!(w.action_count(), 0);
    assert!(r.experiences().is_empty());
    assert_eq!(s.encode(), before);
}
#[test]
fn full_resource_reserve_is_checked_before_observing_or_acting() {
    for kind in 0..3 {
        let (m, mut s) = conflict([1, 0, 0, 0, 0, 0]);
        let mut limits = Limits::default();
        match kind {
            0 => limits.observations = 2,
            1 => limits.neural_events = 32,
            _ => limits.attention_scans = 1,
        }
        let mut r = runtime_with(access(), limits);
        let (out, a, b) = investigate(&mut r, &m, &mut s, 77, false);
        assert_eq!(out.result, InvestigationResult::ResourceLimited);
        assert_eq!(a.action_count() + b.action_count(), 0);
        assert!(r.experiences().is_empty());
    }
}
#[test]
fn denied_action_and_independent_teaching_permissions_never_execute() {
    let (m, mut s) = conflict([1, 0, 0, 0, 0, 0]);
    for kind in 0..4 {
        let mut rights = access();
        if kind == 0 {
            rights.explore = false;
        }
        if kind == 1 {
            rights.sensors = vec![2];
        }
        let mut r = runtime_with(rights, Limits::default());
        let mut w = world(888, &[(888, 7, 77)]);
        let c = contract(&s);
        let mut g = grant(&s);
        if kind == 2 {
            g.permissions.presentation = None;
        }
        if kind == 3 {
            g.permissions.admissions.clear();
        }
        let result = r.investigate_disagreement(
            &m,
            &mut s,
            c,
            Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
            &mut [InvestigationOffer {
                world: &mut w,
                attributes: reading(20, [0, 0, 0, 0, 0, 0], 50),
                intervention: request(20, 7, 52),
            }],
            Some(g),
        );
        if kind == 1 {
            assert_eq!(result.unwrap().result, InvestigationResult::Denied);
        } else {
            assert!(result.is_err());
        }
        assert_eq!(w.action_count(), 0);
        assert!(r.experiences().is_empty());
    }
}
#[test]
fn incompatible_family_does_not_trigger_an_invented_experiment_or_explanation() {
    let (m, mut s) = conflict([1, 1, 1, 0, 0, 0]);
    let mut r = runtime();
    let mut w = world(888, &[(888, 7, 77)]);
    let c = contract(&s);
    let out = r
        .investigate_disagreement(
            &m,
            &mut s,
            c,
            Attributes::new([1, 1, 1, 0, 0, 0]).unwrap(),
            &mut [InvestigationOffer {
                world: &mut w,
                attributes: reading(20, [0, 0, 0, 0, 0, 0], 50),
                intervention: request(20, 7, 52),
            }],
            None,
        )
        .unwrap();
    assert_eq!(out.result, InvestigationResult::RepresentationLimited);
    assert_eq!(w.action_count(), 0);
    assert_eq!(
        out.origin.question,
        Some(EpistemicQuestion::ReviewAuthorizedRepresentation)
    );
}

fn axis_history(axis: usize) -> (Runtime, Session) {
    let mut h = runtime();
    let mut s = session();
    let mut w = world(10, &[(10, 7, 20)]);
    let mut bits = [0; 6];
    bits[axis] = 1;
    let frame = h.observe_attributes(&w, 7, reading(1, bits, 0)).unwrap();
    h.intervene(&mut w, &request(1, 7, 2)).unwrap();
    let c = contract(&s);
    h.teach_consequences(
        &mut s,
        c,
        &[(frame, 1, EngramId::new(501).unwrap(), 901)],
        PedagogyAssessment::new(
            &[Attributes::new([1, 1, 0, 1, 0, 0]).unwrap().into()],
            &[Attributes::new([0, 0, 1, 0, 1, 0]).unwrap().into()],
        )
        .unwrap(),
        PedagogyPermissions {
            evaluation: Some(c.scope),
            presentation: Some(c.scope),
            admissions: vec![EngramId::new(501).unwrap()],
        },
    )
    .unwrap();
    (h, s)
}
#[test]
fn changing_competing_hypotheses_changes_selected_experience() {
    let mut selected = vec![];
    for axis in [0, 1] {
        let (h, mut s) = axis_history(axis);
        let mut r = runtime();
        let mut a = world(777, &[(777, 7, 20)]);
        let mut b = world(888, &[(888, 7, 20)]);
        let c = contract(&s);
        let out = r
            .investigate_disagreement(
                &h,
                &mut s,
                c,
                Attributes::new([0, 0, 0, 0, 0, 0]).unwrap(),
                &mut [
                    InvestigationOffer {
                        world: &mut a,
                        attributes: reading(20, [1, 0, 0, 0, 0, 0], 50),
                        intervention: request(20, 7, 52),
                    },
                    InvestigationOffer {
                        world: &mut b,
                        attributes: reading(21, [0, 1, 0, 0, 0, 0], 50),
                        intervention: request(21, 7, 52),
                    },
                ],
                None,
            )
            .unwrap();
        assert_eq!(out.criterion_evaluations, 4);
        assert_eq!(out.selection_scans, 2);
        selected.push(out.selected.unwrap());
    }
    assert_eq!(selected, vec![121, 120]);
}
#[test]
fn empty_investigation_control_does_not_acquire_or_revise_knowledge() {
    let (m, mut s) = conflict([1, 0, 0, 0, 0, 0]);
    let before = s.encode();
    let mut r = runtime();
    let c = contract(&s);
    let out = r
        .investigate_disagreement(
            &m,
            &mut s,
            c,
            Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
            &mut [],
            None,
        )
        .unwrap();
    assert_eq!(out.result, InvestigationResult::NoDiscriminatingOffer);
    assert_eq!(s.encode(), before);
    assert!(r.experiences().is_empty());
    assert_eq!(r.action_attempts(), 0);
}
#[test]
fn unexpected_numerical_consequence_is_preserved_without_claiming_conflict_resolved() {
    let (m, mut s) = conflict([1, 0, 0, 0, 0, 0]);
    let mut r = runtime();
    let (out, _, w) = investigate(&mut r, &m, &mut s, 88, false);
    assert_eq!(out.trial.unwrap().to, 88);
    assert_eq!(w.state(), 88);
    assert_eq!(out.result, InvestigationResult::ConflictRetained);
    assert_eq!(
        out.after.unwrap().state,
        EpistemicDiagnosis::ObservationConflict
    );
}
#[test]
fn unobservable_or_unavailable_controls_are_not_experiments() {
    let (m, mut s) = conflict([1, 0, 0, 0, 0, 0]);
    let mut r = runtime();
    let c = contract(&s);
    let mut w = world(888, &[(888, 9, 20)]);
    let out = r
        .investigate_disagreement(
            &m,
            &mut s,
            c,
            Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
            &mut [InvestigationOffer {
                world: &mut w,
                attributes: reading(20, [0, 0, 0, 0, 0, 0], 50),
                intervention: request(20, 7, 52),
            }],
            None,
        )
        .unwrap();
    assert_eq!(out.result, InvestigationResult::Denied);
    assert_eq!(w.action_count(), 0);
    assert!(r.experiences().is_empty());
}
#[test]
fn checkpoint_recovers_learning_and_historical_conflict_in_another_process() {
    let (m, mut s) = conflict([1, 0, 0, 0, 0, 0]);
    let mut archive = EpisodeArchive::new();
    archive.insert(eid(1), &m, &s).unwrap();
    let mut r = runtime();
    investigate(&mut r, &m, &mut s, 77, true);
    archive.insert(eid(2), &r, &s).unwrap();
    let dir = std::env::temp_dir().join(format!("neurons-discrimination-{}", std::process::id()));
    assert!(!dir.exists());
    std::fs::create_dir(&dir).unwrap();
    archive.save(&dir.join("archive.bin")).unwrap();
    std::fs::write(dir.join("active.bin"), r.checkpoint(&s).unwrap()).unwrap();
    let before = std::fs::read(dir.join("archive.bin")).unwrap();
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "discrimination_restore_worker", "--nocapture"])
        .env("NEURONS_DISCRIMINATION_RESTORE", &dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(std::fs::read(dir.join("archive.bin")).unwrap(), before);
    std::fs::remove_file(dir.join("archive.bin")).unwrap();
    std::fs::remove_file(dir.join("active.bin")).unwrap();
    std::fs::remove_dir(dir).unwrap();
}
#[test]
fn discrimination_restore_worker() {
    let Some(dir) = std::env::var_os("NEURONS_DISCRIMINATION_RESTORE") else {
        return;
    };
    let dir = std::path::PathBuf::from(dir);
    let active = std::fs::read(dir.join("active.bin")).unwrap();
    let (mut denied, mut s) = Runtime::restore(&active, &sensors(), Access::default()).unwrap();
    assert_eq!(s.pedagogy_version().unwrap().number, 2);
    let c = contract(&s);
    assert!(denied
        .investigate_disagreement(
            &runtime(),
            &mut s,
            c,
            Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
            &mut [],
            None
        )
        .is_err());
    let archive = EpisodeArchive::load(
        &dir.join("archive.bin"),
        &[(eid(1), sensors()), (eid(2), sensors())],
    )
    .unwrap();
    let mut catalog = EpisodeCatalog::new();
    catalog.insert(1, archive).unwrap();
    let mut fresh = runtime();
    let memory = fresh
        .select_evidence(
            &catalog,
            &s,
            SelectiveQuery {
                goal: PlanningGoal {
                    world: 501,
                    revision: 1,
                    context: 61,
                    source: source(),
                    from: 1234,
                    to: 20,
                },
                conceptual_seed: Some(20),
            },
            RetrievalBudget::default(),
        )
        .unwrap();
    let report = fresh
        .assess_epistemic(
            memory.memory(),
            &s,
            c,
            Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
        )
        .unwrap();
    assert_eq!(report.state, EpistemicDiagnosis::ObservationConflict);
    assert_eq!(report.evidence[0].origins, vec![eid(1)]);
    fresh.inherit_consequence_contract(memory.memory()).unwrap();
    let w = world(1234, &[(1234, 7, 55)]);
    let frame = fresh
        .observe_attributes(&w, 7, reading(29, [0, 1, 1, 0, 0, 1], 70))
        .unwrap();
    let forecast = fresh.forecast_consequence(&s, c, &frame, eid(90)).unwrap();
    assert_eq!(
        forecast.report().decision,
        neurons::metacognition::concepts::ConceptDecision::Supported(
            neurons::cognition::concepts::Label::Negative
        )
    );
    // Mesmo concedendo Access novo ao envelope, o episódio consumido não renasce.
    let (mut consumed, mut ss) = Runtime::restore(&active, &sensors(), access()).unwrap();
    assert!(consumed
        .investigate_disagreement(
            &runtime(),
            &mut ss,
            c,
            Attributes::new([1, 0, 0, 0, 0, 0]).unwrap(),
            &mut [],
            None
        )
        .is_err());
}
