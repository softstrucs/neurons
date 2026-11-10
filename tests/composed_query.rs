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
fn teach(e: &mut Episode, s: &mut Session) {
    let r = e
        .teach_once(
            s,
            lesson(403, "100001", Label::Positive),
            obs("110000"),
            obs("010101"),
            teaching_permissions(403),
        )
        .unwrap();
    assert_eq!(r.lessons.len(), 1);
    assert!(r.lessons[0].admitted);
}
fn prepared() -> (Session, Episode, SyntheticRegistry, Permissions) {
    let mut s = base(true);
    let b = binding();
    let mut e = Episode::new(&s, eid(9001), b.clone());
    teach(&mut e, &mut s);
    (s, e, registry(), permissions(&b))
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
#[test]
fn actual_scenario_learning_reference_and_no_factual_promotion() {
    let mut s = base(true);
    let b = binding();
    let p = permissions(&b);
    let root = registry();
    let mut e = Episode::new(&s, eid(9001), b);
    let before = s.encode();
    let initial = e.query(&s, &root, &p).unwrap();
    assert_eq!(initial.status, Status::Insufficient);
    assert_eq!(
        reference_verify(&s, &root, &initial, &p).unwrap(),
        Status::Insufficient
    );
    assert_eq!(s.encode(), before);
    let facts = s.memory().iter().collect::<Vec<_>>();
    let metadata = s.metadata().to_vec();
    teach(&mut e, &mut s);
    let before = s.encode();
    let r = e.query(&s, &root, &p).unwrap();
    assert_eq!(r.status, Status::Supported);
    assert_eq!(r.assurance, Some(IdentityAssurance::SyntheticRegistryMatch));
    assert_eq!(
        r.conceptual
            .as_ref()
            .unwrap()
            .version
            .as_ref()
            .unwrap()
            .number,
        3
    );
    assert_eq!(
        r.relational
            .as_ref()
            .unwrap()
            .proof
            .as_ref()
            .unwrap()
            .premises
            .len(),
        1
    );
    assert_eq!(
        reference_verify(&s, &root, &r, &p).unwrap(),
        Status::Supported
    );
    assert_eq!(
        e.verify(&s, &root, &r, &p).unwrap(),
        Verification::Verified(Status::Supported)
    );
    assert_eq!(s.encode(), before);
    assert_eq!(s.memory().iter().collect::<Vec<_>>(), facts);
    assert_eq!(s.metadata(), metadata);
    assert_eq!(s.memory().len(), 1);
    assert_eq!(e.usage().conceptual_queries, 5);
    assert_eq!(e.usage().relational_queries, 3);
    assert_eq!(e.usage().conceptual_events, 5);
    assert!(e.usage().neural_events <= 96);
}
#[test]
fn controls_without_teaching_or_relation_remain_insufficient() {
    let s = base(true);
    let mut e = Episode::new(&s, eid(9001), binding());
    let p = permissions(&binding());
    let root = registry();
    let before = s.encode();
    let r = e.query(&s, &root, &p).unwrap();
    assert_eq!(r.status, Status::Insufficient);
    assert_eq!(
        reference_verify(&s, &root, &r, &p).unwrap(),
        Status::Insufficient
    );
    assert_eq!(s.encode(), before);
    let mut s = base(false);
    let mut e = Episode::new(&s, eid(9001), binding());
    teach(&mut e, &mut s);
    let r = e.query(&s, &root, &p).unwrap();
    assert_eq!(r.status, Status::Insufficient);
    assert_eq!(
        reference_verify(&s, &root, &r, &p).unwrap(),
        Status::Insufficient
    );
    assert_eq!(
        r.conceptual.unwrap().decision,
        ConceptDecision::Supported(Label::Positive)
    );
    assert!(r.relational.unwrap().proof.is_none());
}
#[test]
fn both_domains_preserve_opposition() {
    for conceptual in [true, false] {
        let mut s = base(true);
        if conceptual {
            example(&mut s, 450, "111111", "-");
        } else {
            relation(&mut s, 12, 101, 103, false, 8012);
        }
        let mut e = Episode::new(&s, eid(9001), binding());
        let p = permissions(&binding());
        let root = registry();
        let r = e.query(&s, &root, &p).unwrap();
        assert_eq!(r.status, Status::Contradictory);
        assert_eq!(
            reference_verify(&s, &root, &r, &p).unwrap(),
            Status::Contradictory
        );
        assert_eq!(
            e.verify(&s, &root, &r, &p).unwrap(),
            Verification::Verified(Status::Contradictory)
        );
    }
}
#[test]
fn permissions_are_separate_and_denials_do_not_query() {
    for mode in 0..5 {
        let s = base(true);
        let before = s.encode();
        let b = binding();
        let mut p = permissions(&b);
        match mode {
            0 => p.conceptual = None,
            1 => p.relational = None,
            2 => p.composition = None,
            3 => {
                p.relational = Some(PathAuthorization::inclusion_three(Scope {
                    context: ContextId::new(42).unwrap(),
                    ..scope()
                }))
            }
            _ => p.composition.as_mut().unwrap().item = node(999),
        };
        let mut e = Episode::new(&s, eid(9001), b);
        let r = e.query(&s, &registry(), &p).unwrap();
        assert_eq!(r.status, Status::Denied);
        assert!(
            r.conceptual.is_none() && r.relational.is_none() && r.dependencies.state.is_empty()
        );
        assert_eq!(e.usage(), Usage::default());
        assert_eq!(s.encode(), before);
    }
}
#[test]
fn registry_checks_identity_attributes_mapping_and_context() {
    for mode in 0..5 {
        let s = base(true);
        let mut b = binding();
        match mode {
            0 => b.item = node(999),
            1 => b.observation = obs("100100"),
            2 => b.relational_concept = b.concept,
            3 => b.concept = node(502),
            _ => b.scope.context = ContextId::new(42).unwrap(),
        };
        let p = permissions(&b);
        let mut e = Episode::new(&s, eid(9001), b.clone());
        let r = e.query(&s, &registry(), &p).unwrap();
        assert_eq!(r.status, Status::Denied);
        assert_eq!(e.usage().conceptual_queries, 0);
        if mode == 4 {
            let root = SyntheticRegistry::new(b, 1).unwrap();
            assert_eq!(e.query(&s, &root, &p).unwrap().status, Status::Denied);
        }
    }
}
#[test]
fn altered_conceptual_report_binding_and_audit_fields_are_rejected() {
    let (s, mut e, root, p) = prepared();
    let r = e.query(&s, &root, &p).unwrap();
    let used = e.usage();
    for mode in 0..7 {
        let mut forged = r.clone();
        match mode {
            0 => {
                forged.conceptual.as_mut().unwrap().decision =
                    ConceptDecision::Supported(Label::Negative)
            }
            1 => {
                forged
                    .conceptual
                    .as_mut()
                    .unwrap()
                    .version
                    .as_mut()
                    .unwrap()
                    .number += 1
            }
            2 => {
                forged.conceptual.as_mut().unwrap().consulted[0]
                    .metadata
                    .evidence = EvidenceId::new(999).unwrap()
            }
            3 => forged.binding.item = node(999),
            4 => forged.usage.conceptual_queries += 1,
            5 => forged.dependencies.state.push(0),
            _ => forged.assurance = None,
        };
        assert_eq!(
            e.verify(&s, &root, &forged, &p).unwrap(),
            Verification::Rejected
        );
        assert_eq!(e.usage(), used);
        if mode < 4 {
            assert!(reference_verify(&s, &root, &forged, &p).is_err());
        }
    }
}
#[test]
fn forged_proof_does_not_bypass_path_verifier() {
    let (s, mut e, root, p) = prepared();
    let r = e.query(&s, &root, &p).unwrap();
    for empty in [true, false] {
        let mut forged = r.clone();
        let proof = forged.relational.as_mut().unwrap().proof.as_mut().unwrap();
        if empty {
            proof.premises.clear();
        } else {
            proof.premises[0].metadata.information.evidence = Some(EvidenceId::new(999).unwrap());
        }
        assert!(reference_verify(&s, &root, &forged, &p).is_err());
        assert_eq!(
            e.verify(&s, &root, &forged, &p).unwrap(),
            Verification::Rejected
        );
    }
}
#[test]
fn revision_requires_reassessment_even_when_classification_stays_positive() {
    let (mut s, mut e, root, p) = prepared();
    let old = e.query(&s, &root, &p).unwrap();
    assert_eq!(
        e.verify(&s, &root, &old, &p).unwrap(),
        Verification::Verified(Status::Supported)
    );
    e.revise_once(
        &mut s,
        lesson(404, "100000", Label::Negative),
        teaching_permissions(404),
    )
    .unwrap();
    let usage = e.usage();
    assert_eq!(
        e.check_current(&s, &root, &old, &p),
        Verification::ReevaluationRequired
    );
    assert_eq!(
        e.verify(&s, &root, &old, &p).unwrap(),
        Verification::ReevaluationRequired
    );
    assert_eq!(usage, e.usage());
    assert!(reference_verify(&s, &root, &old, &p).is_err());
    let fresh = e.query(&s, &root, &p).unwrap();
    assert_eq!(fresh.status, Status::Supported);
    assert_eq!(
        fresh
            .conceptual
            .as_ref()
            .unwrap()
            .version
            .as_ref()
            .unwrap()
            .number,
        4
    );
    assert_eq!(
        reference_verify(&s, &root, &fresh, &p).unwrap(),
        Status::Supported
    );
    assert_eq!(e.usage().conceptual_events, 10);
    assert_eq!(e.usage().relational_queries, 3);
    assert!(e
        .revise_once(
            &mut s,
            lesson(405, "101001", Label::Positive),
            teaching_permissions(405)
        )
        .is_err());
}
#[test]
fn time_and_new_opposition_change_dependencies() {
    for expired in [true, false] {
        let (mut s, mut e, root, p) = prepared();
        let old = e.query(&s, &root, &p).unwrap();
        if expired {
            apply(&mut s, "agora 10001");
            assert!(check_observation(
                old.relational
                    .as_ref()
                    .unwrap()
                    .proof
                    .as_ref()
                    .unwrap()
                    .premises[0]
                    .metadata
                    .information,
                scope(),
                &[SourceId::new(8).unwrap()],
                s.logical_time()
            )
            .is_err());
        } else {
            relation(&mut s, 12, 101, 103, false, 8012);
        }
        assert_eq!(
            e.check_current(&s, &root, &old, &p),
            Verification::ReevaluationRequired
        );
        let fresh = e.query(&s, &root, &p).unwrap();
        assert_eq!(
            fresh.status,
            if expired {
                Status::Denied
            } else {
                Status::Contradictory
            }
        );
    }
}
#[test]
fn circular_origins_and_shared_evidence_are_not_independent() {
    let s = base(true);
    let mut b = binding();
    b.origin = BindingOrigin::ComposedReport(eid(9001));
    assert!(SyntheticRegistry::new(b.clone(), 1).is_err());
    let p = permissions(&b);
    let mut e = Episode::new(&s, eid(9001), b);
    assert_eq!(e.query(&s, &registry(), &p).unwrap().status, Status::Denied);
    let mut s = base(false);
    relation(&mut s, 11, 101, 103, true, 2401);
    let mut e = Episode::new(&s, eid(9001), binding());
    teach(&mut e, &mut s);
    assert_eq!(
        e.query(&s, &registry(), &permissions(&binding()))
            .unwrap()
            .status,
        Status::Denied
    );
}
#[test]
fn budget_is_shared_and_neural_limit_is_really_32() {
    for limits in [
        Limits {
            conceptual_queries: 0,
            ..Limits::default()
        },
        Limits {
            relational_queries: 0,
            ..Limits::default()
        },
        Limits {
            neural_events: 31,
            ..Limits::default()
        },
    ] {
        let s = base(true);
        let mut e = Episode::with_limits(&s, eid(9001), binding(), limits).unwrap();
        assert_eq!(
            e.query(&s, &registry(), &permissions(&binding()))
                .unwrap()
                .status,
            Status::Interrupted
        );
        assert_eq!(e.usage(), Usage::default());
    }
    let mut s = base(false);
    relation(&mut s, 12, 101, 105, true, 8012);
    relation(&mut s, 13, 105, 101, true, 8013);
    relation(&mut s, 14, 105, 103, true, 8014);
    let mut e = Episode::new(&s, eid(9001), binding());
    let p = permissions(&binding());
    let root = registry();
    for n in 1..=3 {
        let r = e.query(&s, &root, &p).unwrap();
        assert_eq!(r.status, Status::Interrupted);
        assert_eq!(
            r.relational
                .unwrap()
                .review
                .unwrap()
                .inference
                .processed_events,
            32
        );
        assert_eq!(e.usage().neural_events, n * 32);
    }
    let before = e.usage();
    assert_eq!(e.query(&s, &root, &p).unwrap().status, Status::Interrupted);
    assert_eq!(e.usage(), before);
}
#[test]
fn verification_consumes_budget_and_is_not_just_a_receipt() {
    let s = base(true);
    let root = registry();
    let p = permissions(&binding());
    let mut e = Episode::with_limits(
        &s,
        eid(9001),
        binding(),
        Limits {
            relational_queries: 1,
            ..Limits::default()
        },
    )
    .unwrap();
    let r = e.query(&s, &root, &p).unwrap();
    assert_eq!(e.check_current(&s, &root, &r, &p), Verification::Current);
    assert_eq!(
        e.verify(&s, &root, &r, &p).unwrap(),
        Verification::Interrupted
    );
    let mut other = Episode::new(&s, eid(9001), binding());
    assert_eq!(
        other.verify(&s, &root, &r, &p).unwrap(),
        Verification::Rejected
    );
}
#[test]
fn authority_revision_is_an_explicit_dependency() {
    let (s, mut e, root, p) = prepared();
    let r = e.query(&s, &root, &p).unwrap();
    let changed = SyntheticRegistry::new(binding(), 2).unwrap();
    assert_eq!(
        e.check_current(&s, &changed, &r, &p),
        Verification::ReevaluationRequired
    );
    let fresh = e.query(&s, &changed, &p).unwrap();
    assert_eq!(fresh.dependencies.registry_revision, 2);
    assert_eq!(
        reference_verify(&s, &changed, &fresh, &p).unwrap(),
        Status::Supported
    );
}
#[test]
fn deterministic_checkpoint_materialization_does_not_include_composition() {
    // O filho executa somente este teste e recebe apenas o checkpoint público.
    if let Some(dir) = std::env::var_os("NEURONS_COMPOSED_RESTORE_STAGE3") {
        let dir = std::path::PathBuf::from(dir);
        assert!(dir.starts_with(std::env::temp_dir()));
        let bytes = std::fs::read(dir.join("knowledge.f1")).unwrap();
        let restored = Session::load(&dir.join("knowledge.f1")).unwrap();
        assert_eq!(restored.encode(), bytes);
        let mut fresh = Episode::new(&restored, eid(9002), binding());
        let root = registry();
        let denied = Permissions {
            conceptual: None,
            relational: None,
            composition: None,
        };
        assert_eq!(
            fresh.query(&restored, &root, &denied).unwrap().status,
            Status::Denied
        );
        assert_eq!(fresh.usage(), Usage::default());
        let p = permissions(&binding());
        let report = fresh.query(&restored, &root, &p).unwrap();
        assert_eq!(report.status, Status::Supported);
        assert_eq!(
            reference_verify(&restored, &root, &report, &p).unwrap(),
            Status::Supported
        );
        assert_eq!(
            fresh.verify(&restored, &root, &report, &p).unwrap(),
            Verification::Verified(Status::Supported)
        );
        assert_eq!(restored.encode(), bytes);
        restored.save(&dir.join("reencoded.f1")).unwrap();
        return;
    }
    let execute = || {
        let (s, mut e, root, p) = prepared();
        let before = s.encode();
        let r = e.query(&s, &root, &p).unwrap();
        assert_eq!(s.encode(), before);
        (r, before)
    };
    assert_eq!(execute(), execute());
    let (s, mut e, root, p) = prepared();
    let r = e.query(&s, &root, &p).unwrap();
    let bytes = s.encode();
    let restored = Session::decode(&bytes).unwrap();
    assert_eq!(restored.encode(), bytes);
    let mut fresh = Episode::new(&restored, eid(9002), binding());
    assert_eq!(
        fresh.verify(&restored, &root, &r, &p).unwrap(),
        Verification::Rejected
    );
    let denied = Permissions {
        conceptual: None,
        relational: None,
        composition: None,
    };
    assert_eq!(
        fresh.query(&restored, &root, &denied).unwrap().status,
        Status::Denied
    );
    assert_eq!(
        fresh.query(&restored, &root, &p).unwrap().status,
        Status::Supported
    );
    assert_eq!(restored.encode(), bytes);

    let dir = std::env::temp_dir().join(format!(
        "neurons-composed-restore-{}-{}",
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
            "deterministic_checkpoint_materialization_does_not_include_composition",
            "--nocapture",
        ])
        .env("NEURONS_COMPOSED_RESTORE_STAGE3", &dir)
        .output()
        .unwrap();
    assert!(
        child.status.success(),
        "child stdout={} stderr={}",
        String::from_utf8_lossy(&child.stdout),
        String::from_utf8_lossy(&child.stderr)
    );
    assert_eq!(
        std::fs::read(dir.join("knowledge.f1")).unwrap(),
        std::fs::read(dir.join("reencoded.f1")).unwrap()
    );
}

#[test]
fn teaching_permissions_and_reservations_are_not_reset() {
    for mode in 0..3 {
        let mut s = base(true);
        let mut e = Episode::new(&s, eid(9001), binding());
        let before = s.concepts().unwrap().models().to_vec();
        let mut p = teaching_permissions(403);
        match mode {
            0 => p.evaluation = None,
            1 => p.presentation = None,
            _ => p.admissions.clear(),
        };
        let result = e.teach_once(
            &mut s,
            lesson(403, "100001", Label::Positive),
            obs("110000"),
            obs("010101"),
            p,
        );
        if mode == 0 {
            assert!(result.is_err());
            assert_eq!(e.usage().conceptual_queries, 0);
        } else {
            assert!(result.unwrap().lessons.iter().all(|l| !l.admitted));
            assert_eq!(e.usage().conceptual_queries, 2);
        }
        assert_eq!(s.concepts().unwrap().models(), before);
    }
    let mut s = base(true);
    let mut e = Episode::new(&s, eid(9001), binding());
    teach(&mut e, &mut s);
    assert!(e
        .teach_once(
            &mut s,
            lesson(404, "100000", Label::Negative),
            obs("110000"),
            obs("010101"),
            teaching_permissions(404)
        )
        .is_err());
    assert_eq!(e.usage().teaching_attempts, 1);
}
#[test]
fn supported_negative_and_two_premises_do_not_become_composite_success() {
    let (s, _, _, _) = prepared();
    let mut b = binding();
    b.observation = obs("010101");
    let root = SyntheticRegistry::new(b.clone(), 1).unwrap();
    let p = permissions(&b);
    let mut e = Episode::new(&s, eid(9001), b);
    let r = e.query(&s, &root, &p).unwrap();
    assert_eq!(
        r.conceptual.as_ref().unwrap().decision,
        ConceptDecision::Supported(Label::Negative)
    );
    assert_eq!(r.status, Status::Insufficient);
    assert_eq!(
        reference_verify(&s, &root, &r, &p).unwrap(),
        Status::Insufficient
    );
    let mut s = base(false);
    relation(&mut s, 12, 101, 105, true, 8012);
    relation(&mut s, 13, 105, 103, true, 8013);
    let mut e = Episode::new(&s, eid(9001), binding());
    teach(&mut e, &mut s);
    let r = e.query(&s, &registry(), &permissions(&binding())).unwrap();
    assert_eq!(r.status, Status::Interrupted);
    assert_eq!(r.relational.unwrap().proof.unwrap().premises.len(), 2);
}

#[test]
fn hiding_relational_opposition_is_not_a_successful_verification() {
    let mut s = base(true);
    relation(&mut s, 12, 101, 103, false, 8012);
    let mut e = Episode::new(&s, eid(9001), binding());
    let root = registry();
    let p = permissions(&binding());
    let r = e.query(&s, &root, &p).unwrap();
    assert_eq!(r.status, Status::Contradictory);
    let mut forged = r.clone();
    forged.status = Status::Supported;
    forged
        .relational
        .as_mut()
        .unwrap()
        .review
        .as_mut()
        .unwrap()
        .conflicts
        .clear();
    assert!(reference_verify(&s, &root, &forged, &p).is_err());
    assert_eq!(
        e.verify(&s, &root, &forged, &p).unwrap(),
        Verification::Rejected
    );
}
