//! PIFI: engenharia pública sintética; não é protocolo de validação científica.
use neurons::cognitive_contracts::*;
use neurons::engram::{EngramId, EvidenceId, RecordStatus};
use neurons::event::{Event, EventKind, NodeId};
use neurons::integration::development::*;
use neurons::integration::synaptic::SynapticEpisode;
use neurons::learning::Feedback;
use neurons::session::f1::Session;
use neurons::synapse::SynapseId;

fn n(id: u32) -> NodeId {
    NodeId::new(id).unwrap()
}
fn neural(weight: i32, extra: bool) -> SynapticEpisode {
    let mut e = SynapticEpisode::new();
    for command in [
        "novo 41 61 81 1 0 3 2 16 64".to_string(),
        "neuronio 101 1 4".into(),
        "neuronio 103 2 4".into(),
        format!("sinapse 11 101 103 {weight} 1"),
        "autorizar 7 0 1000".into(),
    ] {
        e.execute(&command).unwrap();
    }
    if extra {
        e.execute("neuronio 105 1 4").unwrap();
        e.execute("sinapse 12 103 105 1 1").unwrap();
    }
    e
}
fn sensor() -> Sensor {
    Sensor {
        id: 1,
        source: SourceId::new(7).unwrap(),
        context: 61,
        input: n(101),
        encoding: Encoding::Scalar,
    }
}
fn access() -> Access {
    Access {
        inspect: true,
        record: true,
        read_knowledge: false,
        explore: true,
        adapt: false,
        learn_concepts: false,
        investigate_relations: false,
        sensors: vec![1],
    }
}
fn runtime() -> Runtime {
    Runtime::new(neural(0, false), &[sensor()], access(), Limits::default()).unwrap()
}
fn reading(id: u32, payload: i32, at: u64) -> Reading {
    Reading {
        id,
        item: n(501),
        sensor: 1,
        source: SourceId::new(7).unwrap(),
        evidence: EvidenceId::new(1000 + id).unwrap(),
        event: Event::new(None, n(101), EventKind::Stimulus, payload, at),
    }
}

#[test]
fn birth_observes_actual_circuit_and_available_ports() {
    let mut a = runtime();
    let mut sensors = vec![sensor()];
    let mut disconnected = sensor();
    disconnected.id = 2;
    disconnected.input = n(999);
    sensors.push(disconnected);
    let mut b = Runtime::new(neural(2, true), &sensors, access(), Limits::default()).unwrap();
    let before = a.neural().state();
    let da = a.discover(None).unwrap();
    let db = b.discover(None).unwrap();
    let ca = da.circuit.unwrap();
    let cb = db.circuit.unwrap();
    assert_eq!((ca.neurons.len(), ca.synapses.len()), (2, 1));
    assert_eq!((cb.neurons.len(), cb.synapses.len()), (3, 2));
    assert_eq!(ca.synapses[0].3, 0);
    assert_eq!(cb.synapses[0].3, 2);
    assert!(db.sensors[0].connected);
    assert!(!db.sensors[1].connected);
    assert_eq!(
        a.memory().iter().map(|r| r.value()).collect::<Vec<_>>(),
        vec![2, 1, 0, 0]
    );
    assert!(a
        .memory()
        .iter()
        .all(|r| r.status() == RecordStatus::Observed && r.evidence().is_none()));
    assert_eq!(before, a.neural().state());
    assert_eq!(a.discoveries().len(), 1);
}
#[test]
fn newborn_without_circuit_does_not_claim_neural_readiness() {
    let mut r = Runtime::new(
        SynapticEpisode::new(),
        &[sensor()],
        access(),
        Limits::default(),
    )
    .unwrap();
    let d = r.discover(None).unwrap();
    assert!(d.configuration.is_none());
    assert!(d.circuit.is_none());
    assert!(!d.sensors[0].connected);
    assert!(r.observe(reading(1, 1, 0)).is_err());
    assert!(r.experiences().is_empty());
}
#[test]
fn introspection_and_recording_permissions_are_independent() {
    for field in 0..2 {
        let mut a = access();
        if field == 0 {
            a.inspect = false;
        } else {
            a.record = false;
        }
        let mut r = Runtime::new(neural(0, false), &[sensor()], a, Limits::default()).unwrap();
        let bytes = r.neural().encode().unwrap();
        assert!(r.discover(None).is_err());
        assert!(r.memory().is_empty());
        assert_eq!(r.neural().encode().unwrap(), bytes);
    }
}
#[test]
fn knowledge_discovery_is_measured_and_permissioned() {
    let mut session = Session::new();
    for c in ["config 91 61 81 1 0 1000", "conceito 501 opaco"] {
        session.execute(c).unwrap();
    }
    let bytes = session.encode();
    let mut a = access();
    a.read_knowledge = true;
    let mut r = Runtime::new(neural(0, false), &[], a, Limits::default()).unwrap();
    let d = r.discover(Some(&session)).unwrap();
    let k = d.knowledge.unwrap();
    assert_eq!(k.facts, session.memory().len());
    assert_eq!(k.examples, 0);
    assert_eq!(k.versions, 1);
    let mut denied = runtime();
    assert!(denied.discover(Some(&session)).unwrap().knowledge.is_none());
    assert_eq!(session.encode(), bytes);
}
#[test]
fn observation_change_and_provenance_are_real_records() {
    let mut r = runtime();
    let first = r.observe(reading(1, 1, 0)).unwrap();
    let stable = r.observe(reading(2, 1, 3)).unwrap();
    let changed = r.observe(reading(3, 0, 6)).unwrap();
    assert_eq!(first.change, Change::First);
    assert_eq!(stable.change, Change::Stable);
    assert_eq!(changed.change, Change::Changed { previous: 1 });
    assert_eq!(
        r.memory().get(changed.record).unwrap().evidence(),
        Some(changed.reading.evidence)
    );
    assert!(r
        .experiences()
        .iter()
        .all(|e| e.nature == RecordStatus::Observed));
    assert_eq!(r.neural().experiences().len(), 3);
    assert!(r.neural().updates().is_empty());
    assert_eq!(r.neural().circuit().unwrap().engine_metrics().dispatched, 3);
}
#[test]
fn denied_or_forged_sensor_cannot_modify_neural_state() {
    for field in 0..5 {
        let mut a = access();
        let mut p = reading(1, 1, 0);
        match field {
            0 => a.sensors.clear(),
            1 => p.source = SourceId::new(8).unwrap(),
            2 => p.sensor = 5,
            3 => p.event = Event::new(None, n(999), EventKind::Stimulus, 1, 0),
            _ => p.event = Event::new(Some(n(99)), n(101), EventKind::Stimulus, 1, 0),
        };
        let mut r = Runtime::new(neural(0, false), &[sensor()], a, Limits::default()).unwrap();
        let before = r.neural().state();
        assert!(r.observe(p).is_err());
        assert_eq!(r.neural().state(), before);
        assert!(r.memory().is_empty());
    }
}
#[test]
fn attention_selects_authorized_change_and_explains_choice() {
    let mut r = runtime();
    r.observe(reading(1, 1, 0)).unwrap();
    let mut denied = reading(4, 9, 7);
    denied.sensor = 9;
    let environment =
        SyntheticEnvironment::new(&[reading(2, 1, 3), reading(3, 0, 6), denied]).unwrap();
    let (decision, experience) = r
        .explore(
            &environment,
            &AttentionGoal {
                sensors: vec![1],
                prefer_change: true,
            },
        )
        .unwrap();
    assert_eq!(decision.selected, Some(3));
    assert_eq!(decision.scanned, 3);
    assert_eq!(experience.unwrap().change, Change::Changed { previous: 1 });
    assert_eq!(r.attention_scans(), 3);
}
#[test]
fn attention_without_future_labels_works_with_different_values() {
    for (before, after) in [(1, 2), (2, 0), (-2, 3)] {
        let mut r = runtime();
        r.observe(reading(1, before, 0)).unwrap();
        let env =
            SyntheticEnvironment::new(&[reading(2, before, 3), reading(3, after, 6)]).unwrap();
        let (a, e) = r
            .explore(
                &env,
                &AttentionGoal {
                    sensors: vec![1],
                    prefer_change: true,
                },
            )
            .unwrap();
        assert_eq!(a.selected, Some(3));
        assert_eq!(e.unwrap().reading.event.payload(), after);
    }
}
#[test]
fn capacity_and_neural_reservation_refuse_before_mutation() {
    for field in 0..3 {
        let mut limits = Limits::default();
        match field {
            0 => limits.records = 1,
            1 => limits.neural_events = 15,
            _ => limits.observations = 0,
        };
        let mut r = Runtime::new(neural(0, false), &[sensor()], access(), limits).unwrap();
        if field == 0 {
            r.observe(reading(1, 1, 0)).unwrap();
        }
        let before = r.neural().state();
        let count = r.memory().len();
        assert!(r.observe(reading(2, 1, 3)).is_err());
        assert_eq!(r.neural().state(), before);
        assert_eq!(r.memory().len(), count);
    }
}
#[test]
fn discovery_and_attention_have_aggregate_limits() {
    let mut limits = Limits::default();
    limits.discoveries = 1;
    limits.attention_scans = 1;
    let mut r = Runtime::new(neural(0, false), &[sensor()], access(), limits).unwrap();
    r.discover(None).unwrap();
    let count = r.memory().len();
    assert!(r.discover(None).is_err());
    assert_eq!(r.memory().len(), count);
    let env = SyntheticEnvironment::new(&[reading(1, 1, 0), reading(2, 2, 3)]).unwrap();
    assert!(r
        .explore(
            &env,
            &AttentionGoal {
                sensors: vec![1],
                prefer_change: true
            }
        )
        .is_err());
    assert_eq!(r.attention_scans(), 0);
}
#[test]
fn repeated_and_obsolete_observations_are_not_new_experiences() {
    let mut r = runtime();
    r.observe(reading(1, 1, 3)).unwrap();
    let before = r.neural().state();
    assert!(r.observe(reading(1, 1, 6)).is_err());
    assert!(r.observe(reading(2, 1, 0)).is_err());
    assert_eq!(r.neural().state(), before);
    assert_eq!(r.experiences().len(), 1);
}
#[test]
fn mp05_port_records_explicit_weight_change_without_claiming_general_learning() {
    let mut a = access();
    a.adapt = true;
    let mut treatment = Runtime::new(neural(0, false), &[sensor()], a, Limits::default()).unwrap();
    let mut control = runtime();
    for r in [&mut treatment, &mut control] {
        r.observe(reading(1, 1, 0)).unwrap();
    }
    let request = FeedbackRequest {
        id: 201,
        experience: 1,
        source: SourceId::new(7).unwrap(),
        evidence: EvidenceId::new(2201).unwrap(),
        synapse: SynapseId::new(11).unwrap(),
        signal: Feedback::Positive,
        at: 1,
    };
    assert!(control.feedback(request).is_err());
    let update = treatment.feedback(request).unwrap();
    assert_eq!((update.update.before, update.update.after), (0, 2));
    for r in [&mut treatment, &mut control] {
        r.observe(reading(2, 1, 2)).unwrap();
    }
    assert_eq!(
        treatment
            .neural()
            .circuit()
            .unwrap()
            .neuron_state(n(103))
            .unwrap()
            .activations,
        1
    );
    assert_eq!(
        control
            .neural()
            .circuit()
            .unwrap()
            .neuron_state(n(103))
            .unwrap()
            .activations,
        0
    );
    assert!(treatment.feedback(request).is_err());
    assert_eq!(treatment.memory().len(), control.memory().len());
}
#[test]
fn restored_neural_content_does_not_grant_runtime_authority() {
    let mut original = runtime();
    original.observe(reading(1, 1, 0)).unwrap();
    let restored = SynapticEpisode::decode(&original.neural().encode().unwrap()).unwrap();
    let mut r = Runtime::new(restored, &[sensor()], Access::default(), Limits::default()).unwrap();
    let before = r.neural().state();
    assert!(r.discover(None).is_err());
    assert!(r.observe(reading(2, 1, 3)).is_err());
    assert_eq!(r.neural().state(), before);
}

fn scope() -> Scope {
    Scope {
        catalog: CatalogId::new(91).unwrap(),
        context: ContextId::new(61).unwrap(),
        rule: RuleRef {
            id: RuleId::new(81).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 1000).unwrap(),
    }
}
fn statement(from: u32, to: u32) -> Statement {
    let c = |v| Concept::new(n(v), scope().catalog, "opaco", "símbolo público").unwrap();
    Statement::inclusion(&c(from), &c(to), Polarity::Positive, scope()).unwrap()
}
fn relational_session() -> Session {
    let mut s = Session::new();
    for c in [
        "config 91 61 81 1 0 1000",
        "simbolo 101 a",
        "simbolo 103 b",
        "simbolo 105 c",
        "autorizar 61",
        "receber 411 61 101 103 + 8 1411 observacao",
        "interpretar",
        "avaliar",
        "admitir",
        "agora 10",
    ] {
        s.execute(c).unwrap();
    }
    s
}
fn relational_task() -> RelationalTask {
    use neurons::integration::{investigation::*, query::PathAuthorization};
    RelationalTask {
        query: statement(101, 105),
        permissions: RegulationPermissions {
            evaluation: Some(scope()),
            mp06: Some(PathAuthorization::inclusion_three(scope())),
            investigation: Some(scope()),
            source: Some((scope(), SourceId::new(7).unwrap())),
            admission: Some((scope(), EngramId::new(412).unwrap())),
        },
        budget: RegulationBudget::default(),
    }
}
fn observed_relation() -> Runtime {
    let mut a = access();
    a.investigate_relations = true;
    let mut port = sensor();
    port.encoding = Encoding::Inclusion;
    let mut r = Runtime::new(neural(0, false), &[port], a, Limits::default()).unwrap();
    let mut p = reading(1, 105, 1);
    p.item = n(103);
    r.observe(p).unwrap();
    r
}
#[test]
fn observed_relation_is_acquired_only_after_actual_gap_and_authorization() {
    use neurons::metacognition::GoalResult;
    let mut s = relational_session();
    assert_eq!(s.memory().len(), 1);
    let mut r = observed_relation();
    let report = r.investigate(&mut s, &[1], relational_task()).unwrap();
    assert!(
        report.admitted,
        "{:?}: {:?}",
        report.diagnostics, report.history
    );
    assert_eq!(report.source_calls, 1);
    assert_eq!(report.goal, GoalResult::Met);
    assert_eq!(s.memory().len(), 2);
    let new = s
        .metadata()
        .iter()
        .find(|m| m.id == EngramId::new(412).unwrap())
        .unwrap();
    assert_eq!(new.information.statement, statement(103, 105));
    assert_eq!(
        new.information.evidence,
        Some(EvidenceId::new(1001).unwrap())
    );
    assert!(!s
        .metadata()
        .iter()
        .any(|m| m.information.statement == statement(101, 105)));
    assert!(report.events <= 96);
    assert!(report.queries <= 3);
    assert!(r.investigate(&mut s, &[1], relational_task()).is_err());
}
#[test]
fn relation_admission_denial_cannot_be_replaced_by_sensor_confidence() {
    let mut s = relational_session();
    let before = s.memory().iter().collect::<Vec<_>>();
    let mut r = observed_relation();
    let mut task = relational_task();
    task.permissions.admission = None;
    let report = r.investigate(&mut s, &[1], task).unwrap();
    assert!(!report.admitted);
    assert_ne!(report.goal, neurons::metacognition::GoalResult::Met);
    assert_eq!(s.memory().iter().collect::<Vec<_>>(), before);
}
#[test]
fn relational_observation_must_be_present_current_and_unique() {
    for field in 0..3 {
        let mut s = relational_session();
        let before = s.encode();
        let mut r = observed_relation();
        let mut task = relational_task();
        let readings = match field {
            0 => vec![99],
            1 => vec![1, 1],
            _ => {
                task.query = Statement::inclusion(
                    &Concept::new(n(101), scope().catalog, "a", "a").unwrap(),
                    &Concept::new(n(105), scope().catalog, "c", "c").unwrap(),
                    Polarity::Positive,
                    Scope {
                        context: ContextId::new(62).unwrap(),
                        ..scope()
                    },
                )
                .unwrap();
                vec![1]
            }
        };
        assert!(r.investigate(&mut s, &readings, task).is_err());
        assert_eq!(s.encode(), before);
    }
}

fn concept_session() -> Session {
    let mut s = relational_session();
    for c in [
        "receber 413 61 103 105 + 8 1413 observacao",
        "interpretar",
        "avaliar",
        "admitir",
        "conceito 501 opaco",
        "exemplo 401 901 61 111111 + 7 2401 observacao",
        "interpretar-exemplo",
        "avaliar-exemplo",
        "admitir-exemplo 401",
        "revisar",
        "agora 100",
    ] {
        s.execute(c).unwrap();
    }
    s
}
fn observation(bits: [u8; 6]) -> neurons::cognition::concepts::Observation {
    neurons::cognition::concepts::Attributes::new(bits)
        .unwrap()
        .into()
}
fn conceptual_setup() -> (
    Runtime,
    Session,
    ConceptualTask,
    neurons::integration::composed::SyntheticRegistry,
    neurons::integration::experiential::LocalExperiences,
) {
    use neurons::curriculum::{Lesson, PedagogyPermissions};
    use neurons::integration::{composed::*, experiential::*};
    let s = concept_session();
    let mut a = access();
    a.learn_concepts = true;
    let mut port = sensor();
    port.encoding = Encoding::BooleanAttributes;
    let mut r = Runtime::new(neural(0, false), &[port], a, Default::default()).unwrap();
    // 110001 codificado como evento, com identidade/procedência preservadas.
    r.observe(reading(1, 36, 100)).unwrap();
    let b = Binding {
        id: EpisodeId::new(700).unwrap(),
        item: n(501),
        observation: observation([1, 1, 0, 0, 0, 1]),
        concept: n(501),
        relational_concept: n(101),
        destination: n(105),
        scope: scope(),
        source: SourceId::new(7).unwrap(),
        evidence: EvidenceId::new(1001).unwrap(),
        origin: BindingOrigin::ExternalSyntheticObservation,
    };
    let rows: Vec<_> = [[0, 1, 1, 1, 1, 1], [1, 1, 1, 1, 1, 0]]
        .iter()
        .enumerate()
        .map(|(i, bits)| Lesson {
            id: EngramId::new(402 + i as u32).unwrap(),
            receipt: 402 + i as u32,
            exemplar: n(902 + i as u32),
            attributes: neurons::cognition::concepts::Attributes::new(*bits).unwrap(),
            label: neurons::cognition::concepts::Label::Negative,
            source: SourceId::new(7).unwrap(),
            evidence: EvidenceId::new(2402 + i as u32).unwrap(),
        })
        .collect();
    let permissions = neurons::integration::composed::Permissions {
        conceptual: Some(scope()),
        relational: Some(neurons::integration::query::PathAuthorization::inclusion_three(scope())),
        composition: Some(b.clone()),
    };
    let task = ConceptualTask {
        objective: EpisodeId::new(701).unwrap(),
        binding: b.clone(),
        authority: Authority {
            observation: true,
            query: permissions,
            investigate: true,
            source: Some(SourceId::new(7).unwrap()),
            pedagogy: PedagogyPermissions {
                evaluation: Some(scope()),
                presentation: Some(scope()),
                admissions: rows.iter().map(|l| l.id).collect(),
            },
        },
        budget: Budget::default(),
        transfer_probe: observation([0, 1, 0, 1, 0, 1]),
    };
    (
        r,
        s,
        task,
        SyntheticRegistry::new(b, 1).unwrap(),
        LocalExperiences::new(SourceId::new(7).unwrap(), n(501), scope(), &rows).unwrap(),
    )
}
#[test]
fn perception_feeds_existing_conceptual_learning_without_inventing_a_label() {
    let (mut r, mut s, task, registry, mut source) = conceptual_setup();
    let before = r.memory().iter().collect::<Vec<_>>();
    let report = r.learn(&mut s, 1, task, &registry, &mut source).unwrap();
    assert_eq!(
        report.initial.status,
        neurons::integration::composed::Status::Insufficient
    );
    assert_eq!(
        report.result,
        neurons::integration::composed::Status::Supported
    );
    assert!(report.committed);
    assert_eq!(r.memory().iter().collect::<Vec<_>>(), before);
    assert!(report.usage.neural_events > 0);
    assert_eq!(s.concepts().unwrap().models()[0].current().number, 3);
}
#[test]
fn conceptual_learning_cannot_substitute_an_unobserved_item_or_evidence() {
    for field in 0..3 {
        let (mut r, mut s, mut task, registry, mut source) = conceptual_setup();
        let before = s.encode();
        match field {
            0 => task.binding.item = n(999),
            1 => task.binding.evidence = EvidenceId::new(999).unwrap(),
            _ => task.binding.observation = observation([1, 0, 0, 0, 0, 1]),
        };
        assert!(r.learn(&mut s, 1, task, &registry, &mut source).is_err());
        assert_eq!(s.encode(), before);
        assert!(!source.consumed());
        assert_eq!(r.cognitive_resources().operations_without_report, 1);
        assert_eq!(r.cognitive_resources().reserved_neural_events, 128);
    }
}

#[test]
fn scalar_sensor_is_not_a_semantic_relation() {
    let mut a = access();
    a.investigate_relations = true;
    let mut r = Runtime::new(neural(0, false), &[sensor()], a, Limits::default()).unwrap();
    let mut packet = reading(1, 105, 1);
    packet.item = n(103);
    r.observe(packet).unwrap();
    let mut session = relational_session();
    let before = session.encode();
    assert!(r
        .investigate(&mut session, &[1], relational_task())
        .is_err());
    assert_eq!(session.encode(), before);
}

fn observed_binding(
    item: u32,
    bits: [u8; 6],
    evidence: u32,
) -> neurons::integration::composed::Binding {
    use neurons::integration::composed::*;
    Binding {
        id: EpisodeId::new(710).unwrap(),
        item: n(item),
        observation: observation(bits),
        concept: n(501),
        relational_concept: n(101),
        destination: n(105),
        scope: scope(),
        source: SourceId::new(7).unwrap(),
        evidence: EvidenceId::new(evidence).unwrap(),
        origin: BindingOrigin::ExternalSyntheticObservation,
    }
}
fn verification_task(binding: neurons::integration::composed::Binding) -> VerificationTask {
    let permissions = neurons::integration::composed::Permissions {
        conceptual: Some(scope()),
        relational: Some(neurons::integration::query::PathAuthorization::inclusion_three(scope())),
        composition: Some(binding.clone()),
    };
    VerificationTask {
        objective: EpisodeId::new(711).unwrap(),
        binding,
        permissions,
    }
}
fn goal_task() -> LessonTask {
    use neurons::curriculum::*;
    let rows: Vec<_> = [[0, 1, 1, 1, 1, 1], [1, 1, 1, 1, 1, 0]]
        .iter()
        .enumerate()
        .map(|(i, bits)| Lesson {
            id: EngramId::new(402 + i as u32).unwrap(),
            receipt: 402 + i as u32,
            exemplar: n(902 + i as u32),
            attributes: neurons::cognition::concepts::Attributes::new(*bits).unwrap(),
            label: neurons::cognition::concepts::Label::Negative,
            source: SourceId::new(7).unwrap(),
            evidence: EvidenceId::new(2402 + i as u32).unwrap(),
        })
        .collect();
    LessonTask {
        goal: ConceptGoal {
            id: EpisodeId::new(720).unwrap(),
            concept: n(501),
            scope: scope(),
            observation: observation([1, 1, 0, 0, 0, 1]),
            criterion: GoalCriterion::SupportedByCurrentEvidence,
        },
        plan: TeachingPlan::new(n(501), scope(), &rows).unwrap(),
        assessment: PedagogyAssessment::new(
            &[observation([1, 0, 1, 0, 0, 1])],
            &[observation([0, 1, 0, 1, 0, 1])],
        )
        .unwrap(),
        permissions: PedagogyPermissions {
            evaluation: Some(scope()),
            presentation: Some(scope()),
            admissions: rows.iter().map(|l| l.id).collect(),
        },
        budget: ConceptGoalBudget::default(),
    }
}
fn acquisition_cycle() -> (Runtime, Session) {
    let mut s = relational_session();
    for c in [
        "conceito 501 opaco",
        "exemplo 401 901 61 111111 + 7 2401 observacao",
        "interpretar-exemplo",
        "avaliar-exemplo",
        "admitir-exemplo 401",
        "revisar",
        "agora 100",
    ] {
        s.execute(c).unwrap();
    }
    let mut a = access();
    a.learn_concepts = true;
    a.investigate_relations = true;
    a.read_knowledge = true;
    a.sensors = vec![1, 2];
    let mut bits_sensor = sensor();
    bits_sensor.encoding = Encoding::BooleanAttributes;
    let mut relational_sensor = sensor();
    relational_sensor.id = 2;
    relational_sensor.encoding = Encoding::Inclusion;
    let mut r = Runtime::new(
        neural(0, false),
        &[bits_sensor, relational_sensor],
        a,
        Limits::default(),
    )
    .unwrap();
    let birth = r.discover(Some(&s)).unwrap();
    assert_eq!(birth.knowledge.unwrap().facts, 1);
    r.observe(reading(1, 36, 100)).unwrap();
    let teaching = r.teach_observed(&mut s, 1, goal_task()).unwrap();
    assert_eq!(
        teaching.initial.unwrap().report.decision,
        neurons::metacognition::concepts::ConceptDecision::Abstain(
            neurons::metacognition::concepts::Abstention::InsufficientEvidence
        )
    );
    assert_eq!(
        teaching.outcome,
        neurons::curriculum::ConceptGoalResult::Resolved
    );
    let mut packet = reading(2, 105, 101);
    packet.sensor = 2;
    packet.item = n(103);
    r.observe(packet).unwrap();
    let investigation = r.investigate(&mut s, &[2], relational_task()).unwrap();
    assert_eq!(investigation.goal, neurons::metacognition::GoalResult::Met);
    assert!(investigation.admitted);
    let binding = observed_binding(501, [1, 1, 0, 0, 0, 1], 1001);
    let registry =
        neurons::integration::composed::SyntheticRegistry::new(binding.clone(), 1).unwrap();
    let final_report = r
        .verify_observed(&s, 1, verification_task(binding), &registry)
        .unwrap();
    assert_eq!(
        final_report.verification,
        neurons::integration::composed::Verification::Verified(
            neurons::integration::composed::Status::Supported
        )
    );
    assert_eq!(
        final_report
            .report
            .relational
            .unwrap()
            .proof
            .unwrap()
            .premises
            .iter()
            .map(|p| p.engram.id().get())
            .collect::<Vec<_>>(),
        vec![411, 412]
    );
    assert_eq!(s.memory().len(), 2);
    assert!(!s
        .metadata()
        .iter()
        .any(|m| m.information.statement == statement(101, 105)));
    let resources = r.cognitive_resources();
    assert_eq!(
        (
            resources.reserved_conceptual_queries,
            resources.reserved_relational_queries,
            resources.reserved_neural_events
        ),
        (7, 5, 160)
    );
    assert_eq!(
        resources.conceptual_queries,
        teaching.queries + final_report.usage.conceptual_queries
    );
    assert_eq!(
        resources.relational_queries,
        investigation.queries + final_report.usage.relational_queries
    );
    assert_eq!(
        resources.neural_events,
        investigation.events + final_report.usage.neural_events
    );
    assert_eq!(resources.operations_without_report, 0);
    let after = r.discover(Some(&s)).unwrap();
    assert_eq!(after.knowledge.unwrap().facts, 2);
    assert_eq!(after.cognitive_resources, resources);
    (r, s)
}
#[test]
fn birth_perception_concept_acquisition_relation_acquisition_and_verified_inference() {
    let (r, s) = acquisition_cycle();
    assert_eq!(r.experiences().len(), 2);
    assert_eq!(s.concepts().unwrap().models()[0].current().number, 3);
}
#[test]
fn pifi_restored_process_worker() {
    let Some(dir) = std::env::var_os("NEURONS_PIFI_RESTORE") else {
        return;
    };
    let dir = std::path::Path::new(&dir);
    let session = Session::load(&dir.join("knowledge.f1")).unwrap();
    let bytes = session.encode();
    let neural = SynapticEpisode::load(&dir.join("neural.mp05")).unwrap();
    let operational = std::fs::read(dir.join("runtime.pifi")).unwrap();
    let (saved, saved_knowledge) =
        Runtime::restore(&operational, &acquisition_ports(), Access::default()).unwrap();
    assert_eq!(saved_knowledge.encode(), bytes);
    assert_eq!(saved.neural().encode().unwrap(), neural.encode().unwrap());
    assert_eq!(saved.experiences().len(), 2);
    assert_eq!(saved.memory().len(), 10);
    assert_eq!(saved.checkpoint(&saved_knowledge).unwrap(), operational);
    let mut fresh_sensor = sensor();
    fresh_sensor.encoding = Encoding::BooleanAttributes;
    let mut denied = Runtime::new(
        SynapticEpisode::decode(&neural.encode().unwrap()).unwrap(),
        &[fresh_sensor.clone()],
        Access::default(),
        Limits::default(),
    )
    .unwrap();
    assert!(denied.discover(Some(&session)).is_err());
    assert!(denied
        .observe(reading(99, 42, session.logical_time()))
        .is_err());
    let mut a = access();
    a.read_knowledge = true;
    let mut restored = Runtime::new(neural, &[fresh_sensor], a, Limits::default()).unwrap();
    let mut packet = reading(99, 42, session.logical_time());
    packet.item = n(502);
    restored.observe(packet).unwrap();
    let binding = observed_binding(502, [1, 0, 0, 1, 0, 1], 1099);
    let registry =
        neurons::integration::composed::SyntheticRegistry::new(binding.clone(), 1).unwrap();
    let report = restored
        .verify_observed(&session, 99, verification_task(binding), &registry)
        .unwrap();
    assert_eq!(
        report.report.status,
        neurons::integration::composed::Status::Supported
    );
    assert_eq!(
        report.verification,
        neurons::integration::composed::Verification::Verified(
            neurons::integration::composed::Status::Supported
        )
    );
    assert_eq!(session.encode(), bytes);
    println!("PIFI: conhecimento e estado neural restaurados; caso inédito verificado; novas concessões explícitas");
}
#[test]
fn acquired_concepts_and_relations_survive_process_restart_with_existing_codecs() {
    let (r, s) = acquisition_cycle();
    let dir = std::env::temp_dir().join(format!(
        "neurons-pifi-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&dir).unwrap();
    s.save(&dir.join("knowledge.f1")).unwrap();
    r.neural().save(&dir.join("neural.mp05")).unwrap();
    use std::io::Write;
    std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dir.join("runtime.pifi"))
        .unwrap()
        .write_all(&r.checkpoint(&s).unwrap())
        .unwrap();
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "pifi_restored_process_worker", "--nocapture"])
        .env("NEURONS_PIFI_RESTORE", &dir)
        .output()
        .unwrap();
    assert!(
        child.status.success(),
        "{}{}",
        String::from_utf8_lossy(&child.stdout),
        String::from_utf8_lossy(&child.stderr)
    );
    assert!(String::from_utf8_lossy(&child.stdout).contains("caso inédito verificado"));
}

fn acquisition_ports() -> Vec<Sensor> {
    let mut first = sensor();
    first.encoding = Encoding::BooleanAttributes;
    let mut second = sensor();
    second.id = 2;
    second.encoding = Encoding::Inclusion;
    vec![first, second]
}
#[test]
fn operational_checkpoint_preserves_memory_provenance_and_spent_resources() {
    let (runtime, knowledge) = acquisition_cycle();
    let bytes = runtime.checkpoint(&knowledge).unwrap();
    let (restored, restored_knowledge) =
        Runtime::restore(&bytes, &acquisition_ports(), Access::default()).unwrap();
    assert_eq!(
        restored.memory().iter().collect::<Vec<_>>(),
        runtime.memory().iter().collect::<Vec<_>>()
    );
    assert_eq!(restored.experiences(), runtime.experiences());
    assert_eq!(restored.discoveries_used(), runtime.discoveries_used());
    assert!(
        restored.discoveries().is_empty(),
        "relatórios transitórios não são autoridade persistida"
    );
    assert_eq!(
        restored.cognitive_resources(),
        runtime.cognitive_resources()
    );
    assert_eq!(restored.neural().state(), runtime.neural().state());
    assert_eq!(restored_knowledge.encode(), knowledge.encode());
    assert_eq!(restored.checkpoint(&restored_knowledge).unwrap(), bytes);
}
#[test]
fn restore_does_not_reopen_consumed_conceptual_or_verification_episodes() {
    let (runtime, knowledge) = acquisition_cycle();
    let bytes = runtime.checkpoint(&knowledge).unwrap();
    let mut a = access();
    a.learn_concepts = true;
    a.read_knowledge = true;
    a.investigate_relations = true;
    a.sensors = vec![1, 2];
    let (mut restored, mut state) = Runtime::restore(&bytes, &acquisition_ports(), a).unwrap();
    let before = state.encode();
    let resources = restored.cognitive_resources();
    assert!(restored.teach_observed(&mut state, 1, goal_task()).is_err());
    let b = observed_binding(501, [1, 1, 0, 0, 0, 1], 1001);
    let registry = neurons::integration::composed::SyntheticRegistry::new(b.clone(), 1).unwrap();
    assert!(restored
        .verify_observed(&state, 1, verification_task(b), &registry)
        .is_err());
    assert!(restored
        .investigate(&mut state, &[2], relational_task())
        .is_err());
    assert_eq!(restored.cognitive_resources(), resources);
    assert_eq!(state.encode(), before);
}
#[test]
fn corrupt_truncated_or_reinterpreted_checkpoint_is_rejected() {
    let (runtime, knowledge) = acquisition_cycle();
    let bytes = runtime.checkpoint(&knowledge).unwrap();
    let ports = acquisition_ports();
    for cut in 0..bytes.len() {
        assert!(
            Runtime::restore(&bytes[..cut], &ports, Access::default()).is_err(),
            "cut={cut}"
        );
    }
    for index in (0..bytes.len()).step_by((bytes.len() / 64).max(1)) {
        let mut changed = bytes.clone();
        changed[index] ^= 1;
        assert!(
            Runtime::restore(&changed, &ports, Access::default()).is_err(),
            "byte={index}"
        );
    }
    let mut changed_ports = ports.clone();
    changed_ports[0].encoding = Encoding::Scalar;
    assert!(Runtime::restore(&bytes, &changed_ports, Access::default()).is_err());
    assert!(Runtime::restore(&bytes, &ports[..1], Access::default()).is_err());
}
fn resign(bytes: &mut [u8]) {
    let end = bytes.len() - 8;
    let hash = bytes[..end].iter().fold(0xcbf29ce484222325u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    });
    bytes[end..].copy_from_slice(&hash.to_le_bytes());
}
fn metadata_offset(bytes: &[u8]) -> usize {
    let neural = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let logical_offset = 16 + neural;
    let logical = u64::from_le_bytes(
        bytes[logical_offset..logical_offset + 8]
            .try_into()
            .unwrap(),
    ) as usize;
    logical_offset + 8 + logical
}
#[test]
fn valid_checksum_does_not_allow_resource_refund_or_provenance_change() {
    let (runtime, knowledge) = acquisition_cycle();
    let original = runtime.checkpoint(&knowledge).unwrap();
    let metadata = metadata_offset(&original);
    let mut refunded = original.clone();
    let reserved_neural = metadata + 7 * 8 + 8 + 3 * 8 + 2 * 8;
    refunded[reserved_neural..reserved_neural + 8].copy_from_slice(&0u64.to_le_bytes());
    resign(&mut refunded);
    assert!(Runtime::restore(&refunded, &acquisition_ports(), Access::default()).is_err());
    // Metadata: 7 limites/contadores, relógio, 3 flags, 8 custos, 2 sensores.
    let memory_count = metadata + 7 * 8 + 8 + 3 * 8 + 8 * 8 + 8 + 2 * 5 * 8;
    let observed_payload = memory_count + 8 + 4 * 7 * 8 + 4 * 8;
    let mut changed = original.clone();
    changed[observed_payload..observed_payload + 8].copy_from_slice(&37u64.to_le_bytes());
    resign(&mut changed);
    assert!(Runtime::restore(&changed, &acquisition_ports(), Access::default()).is_err());
}
#[test]
fn failed_operation_reservations_survive_checkpoint() {
    let mut runtime = observed_relation();
    let mut knowledge = relational_session();
    assert!(runtime
        .investigate(&mut knowledge, &[99], relational_task())
        .is_err());
    let bytes = runtime.checkpoint(&knowledge).unwrap();
    let mut port = sensor();
    port.encoding = Encoding::Inclusion;
    let (restored, _) = Runtime::restore(&bytes, &[port], Access::default()).unwrap();
    assert_eq!(restored.cognitive_resources().operations_without_report, 1);
    assert_eq!(restored.cognitive_resources().reserved_neural_events, 96);
}
#[test]
fn discovery_budget_is_not_reset_when_transient_reports_are_omitted() {
    let mut limits = Limits::default();
    limits.discoveries = 1;
    let mut runtime = Runtime::new(neural(0, false), &[sensor()], access(), limits).unwrap();
    runtime.discover(None).unwrap();
    let bytes = runtime.checkpoint(&Session::new()).unwrap();
    let (mut restored, _) = Runtime::restore(&bytes, &[sensor()], access()).unwrap();
    assert_eq!(restored.discoveries_used(), 1);
    assert!(restored.discoveries().is_empty());
    assert!(restored.discover(None).is_err());
}
