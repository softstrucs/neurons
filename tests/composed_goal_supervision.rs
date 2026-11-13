//! Cenário público próprio. Referência simbólica não recebe gabarito ou plano futuro.
use neurons::cognition::concepts::{Attributes, Label, Observation};
use neurons::cognitive_contracts::*;
use neurons::curriculum::{Lesson, PedagogyPermissions};
use neurons::engram::{EngramId, EvidenceId, RecordStatus};
use neurons::event::NodeId;
use neurons::integration::composed::Episode;
use neurons::integration::composed::*;
use neurons::integration::query::{PathAuthorization, QueryCoordinator, RuleAuthorization};
use neurons::metacognition::concepts::{Abstention, ConceptDecision};
use neurons::session::f1::Session;
fn node(n: u32) -> NodeId {
    NodeId::new(n).unwrap()
}
fn eid(n: u32) -> EpisodeId {
    EpisodeId::new(n).unwrap()
}
fn scope() -> Scope {
    Scope {
        catalog: CatalogId::new(91).unwrap(),
        context: ContextId::new(41).unwrap(),
        rule: RuleRef {
            id: RuleId::new(51).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 10000).unwrap(),
    }
}
fn attrs(s: &str) -> Attributes {
    Attributes::new(
        s.as_bytes()
            .try_into()
            .map(|a: [u8; 6]| a.map(|v| v - b'0'))
            .unwrap(),
    )
    .unwrap()
}
fn obs(s: &str) -> Observation {
    Observation::new(
        s.as_bytes()
            .try_into()
            .map(|a: [u8; 6]| a.map(|v| Some(v - b'0')))
            .unwrap(),
    )
    .unwrap()
}
fn apply(s: &mut Session, c: &str) {
    s.execute(c).unwrap_or_else(|e| panic!("{c}: {e}"));
}
fn example(s: &mut Session, id: u32, bits: &str, label: &str) {
    for c in [
        format!(
            "exemplo {id} {} 41 {bits} {label} 7 {} observacao",
            id + 500,
            id + 2000
        ),
        "interpretar-exemplo".into(),
        "avaliar-exemplo".into(),
        format!("admitir-exemplo {id}"),
        "revisar".into(),
    ] {
        apply(s, &c);
    }
}
fn relation(s: &mut Session, id: u32, from: u32, to: u32, positive: bool, evidence: u32) {
    for c in [
        format!(
            "receber {id} 41 {from} {to} {} 8 {evidence} observacao",
            if positive { "+" } else { "-" }
        ),
        "interpretar".into(),
        "avaliar".into(),
        "admitir".into(),
    ] {
        apply(s, &c);
    }
}
fn base(with_relation: bool) -> Session {
    let mut s = Session::new();
    for c in [
        "config 91 41 51 1 0 10000",
        "simbolo 101 classe-c",
        "simbolo 103 classe-d",
        "simbolo 105 ponte",
        "autorizar 41",
        "conceito 501 opaco",
    ] {
        apply(&mut s, c);
    }
    example(&mut s, 401, "111111", "+");
    example(&mut s, 402, "000001", "-");
    if with_relation {
        relation(&mut s, 11, 101, 103, true, 8011);
    }
    s
}
fn binding() -> Binding {
    Binding {
        id: eid(800),
        item: node(9010),
        observation: obs("100101"),
        concept: node(501),
        relational_concept: node(101),
        destination: node(103),
        scope: scope(),
        source: SourceId::new(9).unwrap(),
        evidence: EvidenceId::new(9900).unwrap(),
        origin: BindingOrigin::ExternalSyntheticObservation,
    }
}
fn registry() -> SyntheticRegistry {
    // Tabela pública congelada, independente do construtor de pedidos binding().
    // Não recebe Session, pergunta, hipótese, experiências ou rótulo esperado.
    let record = Binding {
        id: eid(800),
        item: node(9010),
        observation: Observation::new([Some(1), Some(0), Some(0), Some(1), Some(0), Some(1)])
            .unwrap(),
        concept: node(501),
        relational_concept: node(101),
        destination: node(103),
        scope: scope(),
        source: SourceId::new(9).unwrap(),
        evidence: EvidenceId::new(9900).unwrap(),
        origin: BindingOrigin::ExternalSyntheticObservation,
    };
    SyntheticRegistry::new(record, 1).unwrap()
}
fn permissions(b: &Binding) -> Permissions {
    Permissions {
        conceptual: Some(b.scope),
        relational: Some(PathAuthorization::inclusion_three(b.scope)),
        composition: Some(b.clone()),
    }
}
fn lesson(id: u32, bits: &str, label: Label) -> Lesson {
    Lesson {
        id: EngramId::new(id).unwrap(),
        receipt: id,
        exemplar: node(id + 500),
        attributes: attrs(bits),
        label,
        source: SourceId::new(7).unwrap(),
        evidence: EvidenceId::new(id + 2000).unwrap(),
    }
}
fn teaching_permissions(id: u32) -> PedagogyPermissions {
    PedagogyPermissions {
        evaluation: Some(scope()),
        presentation: Some(scope()),
        admissions: vec![EngramId::new(id).unwrap()],
    }
}
/// Verificador público do harness, independente da classificação do adaptador.
/// Recalcula H23 por tabelas booleanas e percorre somente evidências admitidas.
/// A tabela de identidade e as concessões são entradas independentes, sem rótulo alvo.
pub fn reference_verify(
    s: &Session,
    root: &SyntheticRegistry,
    r: &Report,
    p: &Permissions,
) -> Result<Status, String> {
    let b = root.record();
    if &r.binding != b
        || b.origin != BindingOrigin::ExternalSyntheticObservation
        || p.composition.as_ref() != Some(b)
        || p.conceptual != Some(b.scope)
        || p.relational != Some(PathAuthorization::inclusion_three(b.scope))
    {
        return Err("identidade/vínculo/permissões".into());
    }
    if r.dependencies.registry_revision != root.revision() || r.dependencies.state != s.encode() {
        return Err("dependência obsoleta".into());
    }
    let c = s.concepts().ok_or("sem conceito")?;
    if c.concept().id() != b.concept || c.models().len() != 1 || c.models()[0].scope() != b.scope {
        return Err("contexto/conceito".into());
    }
    let cr = r.conceptual.as_ref().ok_or("sem relatório conceitual")?;
    let examples = c.examples().map_err(|e| format!("{e:?}"))?;
    if cr.episode != r.objective
        || cr.scope != b.scope
        || cr.observation != b.observation
        || cr.at != s.logical_time()
        || cr.version.as_ref() != Some(c.models()[0].current())
        || cr.consulted != examples
    {
        return Err("relatório/versão/evidências conceituais adulterados".into());
    }
    let sources = [SourceId::new(7).unwrap(), SourceId::new(8).unwrap()];
    for ex in &examples {
        if ex.metadata.scope != b.scope
            || !sources.contains(&ex.metadata.source)
            || ex.metadata.incorporated_at > s.logical_time()
            || !b.scope.validity.contains(s.logical_time())
            || ex.record.evidence() != Some(ex.metadata.evidence)
            || ex.record.status() != RecordStatus::Observed
        {
            return Err("proveniência conceitual".into());
        }
    }
    let contradictory = examples.iter().enumerate().any(|(i, a)| {
        examples[..i].iter().any(|b| {
            a.example.attributes == b.example.attributes && a.example.label != b.example.label
        })
    });
    let mut rules: Vec<(u8, Option<bool>)> = (0..6).map(|i| (1 << i, None)).collect();
    for i in 0..6 {
        for j in i + 1..6 {
            rules.push(((1 << i) | (1 << j), None));
        }
    }
    rules.extend([(0, Some(false)), (0, Some(true))]);
    let predict = |rule: (u8, Option<bool>), a: Attributes| {
        rule.1.unwrap_or_else(|| {
            a.values()
                .iter()
                .enumerate()
                .all(|(i, v)| rule.0 & (1 << i) == 0 || *v)
        })
    };
    rules.retain(|rule| {
        examples.iter().all(|ex| {
            predict(*rule, ex.example.attributes) == (ex.example.label == Label::Positive)
        })
    });
    let votes = rules
        .iter()
        .map(|rule| predict(*rule, b.observation.complete().unwrap()))
        .collect::<Vec<_>>();
    let cd = if contradictory {
        ConceptDecision::Abstain(Abstention::ContradictoryEvidence)
    } else if examples.is_empty() {
        ConceptDecision::Abstain(Abstention::NoEvidence)
    } else if votes.is_empty() {
        ConceptDecision::Abstain(Abstention::ModelClassInadequate)
    } else if votes.iter().all(|v| *v == votes[0]) {
        ConceptDecision::Supported(if votes[0] {
            Label::Positive
        } else {
            Label::Negative
        })
    } else {
        ConceptDecision::Abstain(Abstention::InsufficientEvidence)
    };
    if cr.decision != cd {
        return Err("classificação adulterada".into());
    }
    let rr = r.relational.as_ref().ok_or("sem relatório relacional")?;
    if rr.id != r.objective
        || rr.rule != b.scope.rule
        || rr.query.scope() != b.scope
        || rr.query.subject() != b.relational_concept
        || rr.query.object() != b.destination
        || rr.at != s.logical_time()
    {
        return Err("consulta relacional".into());
    }
    let eligible = s
        .metadata()
        .iter()
        .filter_map(|m| {
            let e = s.memory().get(m.id).ok()?;
            (m.state == EpistemicState::AdmittedObservation
                && m.disposition.is_none()
                && e.status() == RecordStatus::Observed
                && e.observed_at() <= s.logical_time()
                && b.scope.validity.contains(e.observed_at())
                && check_observation(m.information, b.scope, &sources, s.logical_time()).is_ok())
            .then_some((e, *m))
        })
        .collect::<Vec<_>>();
    let consulted = eligible
        .iter()
        .map(
            |(engram, metadata)| neurons::integration::query::BoundRecord {
                engram: *engram,
                metadata: *metadata,
            },
        )
        .collect::<Vec<_>>();
    if rr.consulted != consulted {
        return Err("projeção relacional adulterada".into());
    }
    let snapshot = MemorySnapshot::new(&s.memory().iter().map(|e| e.id()).collect::<Vec<_>>())
        .map_err(|e| format!("{e:?}"))?;
    if rr.before != snapshot || rr.after != snapshot {
        return Err("snapshot relacional adulterado".into());
    }
    let positives = eligible
        .iter()
        .filter(|(_, m)| m.information.statement.polarity() == Polarity::Positive)
        .map(|(e, m)| {
            (
                m.information.statement.subject(),
                m.information.statement.object(),
                e.id(),
            )
        })
        .collect::<Vec<_>>();
    let negatives = eligible
        .iter()
        .filter(|(_, m)| m.information.statement.polarity() == Polarity::Negative)
        .map(|(_, m)| {
            (
                m.information.statement.subject(),
                m.information.statement.object(),
            )
        })
        .collect::<Vec<_>>();
    // Enumeração simbólica independente, finita: no máximo 8+64+512 extensões.
    fn paths(
        node: NodeId,
        target: NodeId,
        edges: &[(NodeId, NodeId, EngramId)],
        visited: &mut Vec<NodeId>,
        path: &mut Vec<(NodeId, NodeId)>,
        out: &mut Vec<Vec<(NodeId, NodeId)>>,
    ) {
        if path.len() == 3 {
            return;
        }
        for (a, b, _) in edges {
            if *a != node || visited.contains(b) {
                continue;
            }
            path.push((*a, *b));
            if *b == target {
                out.push(path.clone());
            } else {
                visited.push(*b);
                paths(*b, target, edges, visited, path, out);
                visited.pop();
            }
            path.pop();
        }
    }
    let mut alternatives = vec![];
    paths(
        b.relational_concept,
        b.destination,
        &positives,
        &mut vec![b.relational_concept],
        &mut vec![],
        &mut alternatives,
    );
    let opposed = !alternatives.is_empty()
        && (negatives.contains(&(b.relational_concept, b.destination))
            || alternatives
                .iter()
                .any(|path| path.iter().any(|edge| negatives.contains(edge))));
    if rr
        .review
        .as_ref()
        .is_some_and(|v| (!v.conflicts.is_empty()) != opposed || v.inference.processed_events > 32)
    {
        return Err("oposição ou eventos relatados inconsistentes".into());
    }
    let coordinator = QueryCoordinator::new(
        s.memory(),
        s.metadata(),
        RuleAuthorization::inclusion_in(b.scope),
        &sources,
    )
    .map_err(|e| format!("{e:?}"))?;
    if let Some(proof) = &rr.proof {
        if proof.premises.len() != 1 {
            return Err("fora do recorte de uma premissa".into());
        }
        let ids = proof
            .premises
            .iter()
            .map(|p| p.engram.id())
            .collect::<Vec<_>>();
        if coordinator
            .verify_path(rr.query, s.logical_time(), &ids)
            .map_err(|e| format!("{e:?}"))?
            != *proof
        {
            return Err("prova adulterada".into());
        }
    }
    if cr
        .consulted
        .iter()
        .any(|ex| ex.metadata.evidence == b.evidence)
        || rr.proof.as_ref().is_some_and(|proof| {
            proof.premises.iter().any(|p| {
                p.metadata.information.evidence == Some(b.evidence)
                    || examples
                        .iter()
                        .any(|e| Some(e.metadata.evidence) == p.metadata.information.evidence)
            })
        })
    {
        return Err("evidência circular/dependente".into());
    }
    let direct = positives
        .iter()
        .any(|(a, d, _)| *a == b.relational_concept && *d == b.destination);
    let expected = if contradictory || opposed {
        Status::Contradictory
    } else if cd == ConceptDecision::Supported(Label::Positive) && direct {
        Status::Supported
    } else {
        Status::Insufficient
    };
    if r.status != expected
        || (expected == Status::Supported && (rr.proof.is_none() || rr.refusal.is_some()))
    {
        return Err("conclusão não demonstrada".into());
    }
    Ok(expected)
}

fn request() -> TeachingRequest {
    TeachingRequest {
        lesson: lesson(403, "100001", Label::Positive),
        before: obs("110000"),
        after: obs("010101"),
        permissions: teaching_permissions(403),
    }
}
fn run(s: &mut Session, teaching: Option<TeachingRequest>) -> GoalReport {
    Episode::new(s, eid(9001), binding()).supervise(
        s,
        &registry(),
        permissions(&binding()),
        teaching,
    )
}
#[test]
fn insufficient_goal_is_taught_requeried_and_independently_verified() {
    let mut s = base(true);
    let facts = s.memory().iter().collect::<Vec<_>>();
    let metadata = s.metadata().to_vec();
    let r = run(&mut s, Some(request()));
    assert_eq!(r.diagnosis, GoalDiagnosis::ConceptualInsufficiency);
    assert_eq!(
        r.strategy,
        neurons::metacognition::concepts::ConceptStrategy::TeachOnce
    );
    assert_eq!(r.outcome, GoalOutcome::Supported, "{r:#?}");
    assert_eq!(r.initial.as_ref().unwrap().status, Status::Insufficient);
    let final_report = r.final_query.as_ref().unwrap();
    assert_eq!(
        reference_verify(&s, &registry(), final_report, &r.permissions).unwrap(),
        Status::Supported
    );
    assert_eq!(
        r.verification,
        Some(Verification::Verified(Status::Supported))
    );
    for report in [r.initial.as_ref().unwrap(), final_report] {
        assert_eq!(report.objective, r.objective);
        assert_eq!(report.binding, binding());
        assert_eq!(
            report.conceptual.as_ref().unwrap().observation,
            binding().observation
        );
        assert_eq!(
            report.relational.as_ref().unwrap().query.object(),
            binding().destination
        );
    }
    assert_eq!(
        r.initial
            .as_ref()
            .unwrap()
            .conceptual
            .as_ref()
            .unwrap()
            .version
            .as_ref()
            .unwrap()
            .number,
        2
    );
    assert_eq!(
        final_report
            .conceptual
            .as_ref()
            .unwrap()
            .version
            .as_ref()
            .unwrap()
            .number,
        3
    );
    assert_ne!(
        r.initial.as_ref().unwrap().dependencies.state,
        final_report.dependencies.state
    );
    assert_eq!(r.action.as_ref().unwrap().lessons.len(), 1);
    assert!(r.action.as_ref().unwrap().lessons[0].admitted);
    assert_eq!(
        r.usage,
        Usage {
            conceptual_queries: 5,
            relational_queries: 3,
            conceptual_events: 5,
            teaching_attempts: 1,
            ..Usage::default()
        }
    );
    assert_eq!(facts, s.memory().iter().collect::<Vec<_>>());
    assert_eq!(metadata, s.metadata());
    assert_eq!(s.concepts().unwrap().examples().unwrap().len(), 3);
    assert!(r.history.len() <= 8);
}
#[test]
fn already_supported_does_not_teach_and_verification_is_charged() {
    let mut s = base(true);
    example(&mut s, 403, "100001", "+");
    let bytes = s.encode();
    let r = run(&mut s, Some(request()));
    assert_eq!(r.outcome, GoalOutcome::Supported);
    assert_eq!(r.diagnosis, GoalDiagnosis::Supported);
    assert!(r.action.is_none() && r.final_query.is_none());
    assert_eq!(r.usage.conceptual_queries, 2);
    assert_eq!(r.usage.relational_queries, 2);
    assert_eq!(r.usage.teaching_attempts, 0);
    assert_eq!(bytes, s.encode());
    reference_verify(&s, &registry(), r.initial.as_ref().unwrap(), &r.permissions).unwrap();
}
#[test]
fn equivalent_no_teaching_control_preserves_knowledge() {
    let mut control = base(true);
    let mut treatment = Session::decode(&control.encode()).unwrap();
    let before = control.encode();
    let mut denied = request();
    denied.permissions.presentation = None;
    let c = run(&mut control, Some(denied));
    let t = run(&mut treatment, Some(request()));
    assert_eq!(c.outcome, GoalOutcome::Interrupted);
    assert_eq!(c.diagnosis, GoalDiagnosis::PermissionMissing);
    assert!(c.action.is_none());
    assert_eq!(control.encode(), before);
    assert_eq!(t.outcome, GoalOutcome::Supported);
    assert_eq!(
        c.initial.as_ref().unwrap().conceptual,
        t.initial.as_ref().unwrap().conceptual
    );
    let n = run(&mut control, None);
    assert_eq!(n.outcome, GoalOutcome::Abstention);
    assert_eq!(n.diagnosis, GoalDiagnosis::NoApplicableLesson);
}
#[test]
fn missing_relation_never_teaches() {
    let mut s = base(false);
    let bytes = s.encode();
    let r = run(&mut s, Some(request()));
    assert_eq!(r.diagnosis, GoalDiagnosis::RelationalInsufficiency);
    assert_eq!(r.outcome, GoalOutcome::Abstention);
    assert!(r.action.is_none());
    assert_eq!(bytes, s.encode());
}
#[test]
fn opposition_in_either_domain_prevents_teaching() {
    for conceptual in [false, true] {
        let mut s = base(true);
        if conceptual {
            example(&mut s, 404, "111111", "-");
        } else {
            relation(&mut s, 12, 101, 103, false, 8012);
        }
        let bytes = s.encode();
        let r = run(&mut s, Some(request()));
        assert_eq!(r.diagnosis, GoalDiagnosis::Opposition);
        assert_eq!(r.outcome, GoalOutcome::Abstention);
        assert!(r.action.is_none());
        assert_eq!(bytes, s.encode());
    }
}
#[test]
fn negative_classification_is_not_a_reason_to_teach() {
    let mut s = base(true);
    example(&mut s, 403, "100001", "+");
    let mut b = binding();
    b.observation = obs("010101");
    let root = SyntheticRegistry::new(b.clone(), 1).unwrap();
    let p = permissions(&b);
    let bytes = s.encode();
    let r = Episode::new(&s, eid(9001), b).supervise(&mut s, &root, p, Some(request()));
    assert_eq!(r.diagnosis, GoalDiagnosis::NegativeClassification);
    assert_eq!(r.outcome, GoalOutcome::Abstention);
    assert!(r.action.is_none());
    assert_eq!(bytes, s.encode());
}
#[test]
fn incompatible_binding_or_scope_does_not_execute_cognition() {
    for field in 0..4 {
        let mut s = base(true);
        let bytes = s.encode();
        let mut b = binding();
        match field {
            0 => b.item = node(999),
            1 => b.concept = node(502),
            2 => b.destination = node(105),
            _ => b.scope.context = ContextId::new(42).unwrap(),
        }
        let p = permissions(&b);
        let r = Episode::new(&s, eid(9001), b).supervise(&mut s, &registry(), p, Some(request()));
        assert_eq!(r.outcome, GoalOutcome::Interrupted);
        assert_eq!(r.usage, Usage::default());
        assert!(r.action.is_none());
        assert_eq!(bytes, s.encode());
    }
}
#[test]
fn each_composition_permission_is_independent() {
    for which in 0..3 {
        let mut s = base(true);
        let bytes = s.encode();
        let mut p = permissions(&binding());
        match which {
            0 => p.conceptual = None,
            1 => p.relational = None,
            _ => p.composition = None,
        }
        let r = Episode::new(&s, eid(9001), binding()).supervise(
            &mut s,
            &registry(),
            p,
            Some(request()),
        );
        assert_eq!(r.diagnosis, GoalDiagnosis::PermissionMissing);
        assert_eq!(r.outcome, GoalOutcome::Interrupted);
        assert_eq!(r.usage, Usage::default());
        assert!(r.initial.unwrap().dependencies.state.is_empty());
        assert_eq!(bytes, s.encode());
    }
}
#[test]
fn each_pedagogical_permission_is_independent() {
    for which in 0..4 {
        let mut s = base(true);
        let bytes = s.encode();
        let mut req = request();
        match which {
            0 => req.permissions.evaluation = None,
            1 => req.permissions.presentation = None,
            2 => req.permissions.admissions.clear(),
            _ => req.permissions.evaluation.as_mut().unwrap().context = ContextId::new(42).unwrap(),
        }
        let r = run(&mut s, Some(req));
        assert_eq!(r.outcome, GoalOutcome::Interrupted);
        assert_eq!(r.diagnosis, GoalDiagnosis::PermissionMissing);
        assert_eq!(r.usage.teaching_attempts, 0);
        assert!(r.action.is_none());
        assert_eq!(bytes, s.encode());
    }
}
#[test]
fn budget_reserves_requery_and_verifier_before_any_teaching() {
    for limits in [
        Limits {
            conceptual_queries: 0,
            ..Limits::default()
        },
        Limits {
            conceptual_queries: 4,
            ..Limits::default()
        },
        Limits {
            relational_queries: 2,
            ..Limits::default()
        },
        Limits {
            neural_events: 63,
            ..Limits::default()
        },
    ] {
        let mut s = base(true);
        let bytes = s.encode();
        let r = Episode::with_limits(&s, eid(9001), binding(), limits)
            .unwrap()
            .supervise(
                &mut s,
                &registry(),
                permissions(&binding()),
                Some(request()),
            );
        assert_eq!(r.outcome, GoalOutcome::Interrupted);
        assert_eq!(r.diagnosis, GoalDiagnosis::ResourceLimited);
        assert_eq!(r.usage.teaching_attempts, 0);
        assert!(r.action.is_none());
        assert_eq!(bytes, s.encode());
    }
}
#[test]
fn used_episode_cannot_restart_or_teach_again_under_supervision() {
    let mut s = base(true);
    let mut episode = Episode::new(&s, eid(9001), binding());
    episode
        .teach_once(
            &mut s,
            request().lesson,
            request().before,
            request().after,
            request().permissions,
        )
        .unwrap();
    let bytes = s.encode();
    let usage = episode.usage();
    let r = episode.supervise(
        &mut s,
        &registry(),
        permissions(&binding()),
        Some(request()),
    );
    assert_eq!(r.outcome, GoalOutcome::Interrupted);
    assert!(r.initial.is_none());
    assert_eq!(r.usage, usage);
    assert_eq!(r.usage.teaching_attempts, 1);
    assert_eq!(bytes, s.encode());
}
#[test]
fn diagnosis_rejects_obsolete_and_tampered_reports_without_querying() {
    let mut s = base(true);
    let mut e = Episode::new(&s, eid(9001), binding());
    let p = permissions(&binding());
    let root = registry();
    let initial = e.query(&s, &root, &p).unwrap();
    let bytes = s.encode();
    let usage = e.usage();
    assert_eq!(
        e.diagnose(&s, &root, &initial, &p),
        GoalDiagnosis::ConceptualInsufficiency
    );
    assert_eq!(e.usage(), usage);
    assert_eq!(s.encode(), bytes);
    let mut forged = initial.clone();
    forged.reason.push('x');
    assert_eq!(
        e.diagnose(&s, &root, &forged, &p),
        GoalDiagnosis::VerificationFailed
    );
    e.teach_once(
        &mut s,
        request().lesson,
        request().before,
        request().after,
        request().permissions,
    )
    .unwrap();
    assert_eq!(
        e.diagnose(&s, &root, &initial, &p),
        GoalDiagnosis::StaleReport
    );
    assert_eq!(
        e.verify(&s, &root, &initial, &p).unwrap(),
        Verification::ReevaluationRequired
    );
    let current = e.query(&s, &root, &p).unwrap();
    let mut forged = current.clone();
    forged
        .conceptual
        .as_mut()
        .unwrap()
        .version
        .as_mut()
        .unwrap()
        .number += 1;
    assert_eq!(
        e.verify(&s, &root, &forged, &p).unwrap(),
        Verification::Rejected
    );
}
#[test]
fn unsuitable_or_leaking_lessons_are_not_executed() {
    for which in 0..5 {
        let mut s = base(true);
        let bytes = s.encode();
        let mut req = request();
        match which {
            0 => req.lesson.attributes = attrs("100101"),
            1 => req.before = binding().observation,
            2 => req.lesson.source = SourceId::new(999).unwrap(),
            3 => req.lesson.evidence = EvidenceId::new(8011).unwrap(),
            _ => req.before = obs("111111"),
        }
        let r = run(&mut s, Some(req));
        assert_eq!(r.outcome, GoalOutcome::Abstention);
        assert_eq!(r.diagnosis, GoalDiagnosis::NoApplicableLesson);
        assert!(r.action.is_none());
        assert_eq!(bytes, s.encode());
    }
}
#[test]
fn partial_revision_is_not_composed_success() {
    let mut s = base(true);
    let mut req = request();
    req.lesson.attributes = attrs("110000");
    req.lesson.label = Label::Negative;
    req.before = obs("100001");
    req.after = obs("010101");
    let r = run(&mut s, Some(req));
    assert_eq!(r.outcome, GoalOutcome::Partial, "{r:#?}");
    assert_eq!(
        r.verification,
        Some(Verification::Verified(Status::Insufficient))
    );
    assert_eq!(r.usage.teaching_attempts, 1);
    reference_verify(
        &s,
        &registry(),
        r.final_query.as_ref().unwrap(),
        &r.permissions,
    )
    .unwrap();
}
#[test]
fn restored_knowledge_requires_explicit_new_grants() {
    let mut s = base(true);
    let r = run(&mut s, Some(request()));
    assert_eq!(r.outcome, GoalOutcome::Supported);
    let bytes = s.encode();
    let mut restored = Session::decode(&bytes).unwrap();
    let p = Permissions {
        conceptual: None,
        relational: None,
        composition: None,
    };
    let r = Episode::new(&restored, eid(9002), binding()).supervise(
        &mut restored,
        &registry(),
        p,
        None,
    );
    assert_eq!(r.outcome, GoalOutcome::Interrupted);
    assert_eq!(r.usage, Usage::default());
    assert_eq!(restored.encode(), bytes);
    let r = Episode::new(&restored, eid(9003), binding()).supervise(
        &mut restored,
        &registry(),
        permissions(&binding()),
        None,
    );
    assert_eq!(r.outcome, GoalOutcome::Supported);
    assert!(r.action.is_none());
    assert_eq!(restored.encode(), bytes);
}
#[test]
fn deterministic_single_episode_has_bounded_history_and_consumption() {
    let mut a = base(true);
    let mut b = base(true);
    let ra = run(&mut a, Some(request()));
    let rb = run(&mut b, Some(request()));
    assert_eq!(ra, rb);
    assert_eq!(a.encode(), b.encode());
    assert!(ra.history.len() <= 8);
    assert!(ra.usage.conceptual_queries <= 8);
    assert!(ra.usage.relational_queries <= 3);
    assert!(ra.usage.neural_events <= 96);
    assert!(ra.usage.conceptual_events <= 5);
}

#[test]
fn cyclic_relation_exhaustion_and_two_premise_scope_do_not_teach() {
    for cyclic in [false, true] {
        let mut s = base(false);
        relation(&mut s, 12, 101, 105, true, 8012);
        if cyclic {
            relation(&mut s, 13, 105, 101, true, 8013);
        }
        relation(&mut s, 14, 105, 103, true, 8014);
        let bytes = s.encode();
        let r = run(&mut s, Some(request()));
        assert_eq!(r.outcome, GoalOutcome::Interrupted);
        assert_eq!(
            r.diagnosis,
            if cyclic {
                GoalDiagnosis::ResourceLimited
            } else {
                GoalDiagnosis::ContractBlocked
            }
        );
        assert!(r.action.is_none());
        assert_eq!(r.usage.teaching_attempts, 0);
        if cyclic {
            assert_eq!(r.usage.neural_events, 32);
        }
        assert_eq!(bytes, s.encode());
    }
}
#[test]
fn already_supported_without_verification_reserve_is_interrupted() {
    let mut s = base(true);
    example(&mut s, 403, "100001", "+");
    let bytes = s.encode();
    let r = Episode::with_limits(
        &s,
        eid(9001),
        binding(),
        Limits {
            relational_queries: 1,
            ..Limits::default()
        },
    )
    .unwrap()
    .supervise(&mut s, &registry(), permissions(&binding()), None);
    assert_eq!(r.outcome, GoalOutcome::Interrupted);
    assert_eq!(r.verification, Some(Verification::Interrupted));
    assert!(r.action.is_none());
    assert_eq!(s.encode(), bytes);
}
#[test]
fn capacity_reservation_prevents_partial_teaching() {
    let mut s = base(true);
    apply(&mut s, "agora 9999");
    let bytes = s.encode();
    let r = run(&mut s, Some(request()));
    assert_eq!(r.outcome, GoalOutcome::Interrupted);
    assert_eq!(r.diagnosis, GoalDiagnosis::ResourceLimited);
    assert!(!r.action_started);
    assert_eq!(r.usage.teaching_attempts, 0);
    assert_eq!(s.encode(), bytes);
}
#[test]
fn teaching_failure_retains_prefix_and_conservative_reservations() {
    let mut s = base(true);
    let mut req = request();
    req.lesson.id = EngramId::new(401).unwrap();
    req.permissions.admissions = vec![req.lesson.id];
    let count = s.concepts().unwrap().examples().unwrap().len();
    let r = run(&mut s, Some(req));
    assert_eq!(r.outcome, GoalOutcome::Interrupted, "{r:#?}");
    assert!(r.action_started);
    assert!(r.action_failure.is_some());
    assert_eq!(r.usage.teaching_attempts, 1);
    assert_eq!(r.usage.uncertain_conceptual_reservations, 2);
    assert_eq!(r.accepted_operations.len(), 3);
    assert!(r.final_query.is_some());
    assert!(r.verification.is_some());
    assert_eq!(s.concepts().unwrap().examples().unwrap().len(), count);
    assert_eq!(r.usage.conceptual_events, 3);
}
#[test]
fn uninformative_lesson_and_supported_pedagogy_probe_do_not_fake_teaching() {
    let mut s = base(true);
    let bytes = s.encode();
    let mut req = request();
    req.lesson.attributes = attrs("000000");
    req.lesson.label = Label::Negative;
    let r = run(&mut s, Some(req));
    assert_eq!(r.outcome, GoalOutcome::Abstention);
    assert!(!r.action_started);
    assert_eq!(r.diagnosis, GoalDiagnosis::NoApplicableLesson);
    assert_eq!(bytes, s.encode());
    let mut req = request();
    req.before = obs("000000");
    // A decisão MP09 sobre a avaliação independente pode impedir apresentação.
    // Não se força ensino só porque o supervisor o selecionou para o objetivo.
    let r = run(&mut s, Some(req));
    assert_ne!(r.outcome, GoalOutcome::Supported);
    if r.action.as_ref().is_some_and(|a| a.lessons.is_empty()) {
        assert_eq!(bytes, s.encode());
    }
}
#[test]
fn registry_revision_and_goal_tampering_are_diagnosed() {
    let s = base(true);
    let p = permissions(&binding());
    let root = registry();
    let mut e = Episode::new(&s, eid(9001), binding());
    let report = e.query(&s, &root, &p).unwrap();
    let root2 = SyntheticRegistry::new(binding(), 2).unwrap();
    assert_eq!(
        e.diagnose(&s, &root2, &report, &p),
        GoalDiagnosis::StaleReport
    );
    for which in 0..3 {
        let mut altered = report.clone();
        match which {
            0 => altered.objective = eid(9002),
            1 => altered.binding.item = node(9011),
            _ => altered
                .relational
                .as_mut()
                .unwrap()
                .proof
                .as_mut()
                .unwrap()
                .premises
                .clear(),
        }
        assert_eq!(
            e.diagnose(&s, &root, &altered, &p),
            GoalDiagnosis::VerificationFailed
        );
    }
}
#[test]
fn new_process_restores_only_knowledge_without_supervisor_authority() {
    const KEY: &str = "NEURONS_COMPOSED_GOAL_RESTORE";
    if let Some(dir) = std::env::var_os(KEY) {
        let dir = std::path::PathBuf::from(dir);
        assert!(dir.starts_with(std::env::temp_dir()));
        let mut s = Session::load(&dir.join("knowledge.f1")).unwrap();
        let bytes = s.encode();
        let r = Episode::new(&s, eid(9002), binding()).supervise(
            &mut s,
            &registry(),
            Permissions {
                conceptual: None,
                relational: None,
                composition: None,
            },
            None,
        );
        assert_eq!(r.outcome, GoalOutcome::Interrupted);
        assert_eq!(r.usage, Usage::default());
        let r = Episode::new(&s, eid(9003), binding()).supervise(
            &mut s,
            &registry(),
            permissions(&binding()),
            None,
        );
        assert_eq!(r.outcome, GoalOutcome::Supported);
        assert!(!r.action_started);
        reference_verify(&s, &registry(), r.initial.as_ref().unwrap(), &r.permissions).unwrap();
        assert_eq!(s.encode(), bytes);
        s.save(&dir.join("restored.f1")).unwrap();
        return;
    }
    let mut s = base(true);
    assert_eq!(run(&mut s, Some(request())).outcome, GoalOutcome::Supported);
    let dir = std::env::temp_dir().join(format!(
        "neurons-composed-goal-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&dir).unwrap();
    s.save(&dir.join("knowledge.f1")).unwrap();
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "new_process_restores_only_knowledge_without_supervisor_authority",
        ])
        .env(KEY, &dir)
        .output()
        .unwrap();
    assert!(
        child.status.success(),
        "{} {}",
        String::from_utf8_lossy(&child.stdout),
        String::from_utf8_lossy(&child.stderr)
    );
    assert_eq!(
        std::fs::read(dir.join("knowledge.f1")).unwrap(),
        std::fs::read(dir.join("restored.f1")).unwrap()
    );
}
