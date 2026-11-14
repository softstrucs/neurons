//! Dados prospectivos da seção10; referência externa não chama o candidato para decidir.
use neurons::cognition::concepts::{Label, Observation};
use neurons::cognitive_contracts::*;
use neurons::engram::{EngramId, EvidenceId, RecordStatus};
use neurons::event::NodeId;
use neurons::integration::composed::{
    Binding, BindingOrigin, Permissions, Report, Status, SyntheticRegistry, Verification,
};
use neurons::integration::composed_two::{Episode, Limits};
use neurons::integration::query::PathAuthorization;
use neurons::metacognition::concepts::{Abstention, ConceptDecision};
use neurons::session::f1::Session;
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
fn example(s: &mut Session, id: u32, ex: u32, bits: &str, label: &str, ev: u32) {
    for c in [
        format!("exemplo {id} {ex} 341 {bits} {label} 7 {ev} observacao"),
        "interpretar-exemplo".into(),
        "avaliar-exemplo".into(),
        format!("admitir-exemplo {id}"),
        "revisar".into(),
    ] {
        apply(s, &c);
    }
}
fn receive(s: &mut Session, id: u32, a: u32, b: u32, pol: &str, ev: u32) {
    for c in [
        format!("receber {id} 341 {a} {b} {pol} 8 {ev} observacao"),
        "interpretar".into(),
        "avaliar".into(),
    ] {
        apply(s, &c);
    }
}
fn relation(s: &mut Session, id: u32, a: u32, b: u32, pol: &str, ev: u32) {
    receive(s, id, a, b, pol, ev);
    apply(s, "admitir");
}
fn seed() -> Session {
    let mut s = Session::new();
    for c in [
        "config 391 341 351 1 0 10000",
        "simbolo 3101 c",
        "simbolo 3103 b",
        "simbolo 3105 d",
        "autorizar 341",
        "conceito 3501 opaco",
    ] {
        apply(&mut s, c);
    }
    example(&mut s, 3401, 3601, "100001", "+", 4401);
    example(&mut s, 3402, 3602, "000001", "-", 4402);
    s
}
fn base(first: bool, second: bool) -> Session {
    let mut s = seed();
    if first {
        relation(&mut s, 3111, 3101, 3103, "+", 4811);
    }
    if second {
        relation(&mut s, 3112, 3103, 3105, "+", 4812);
    }
    apply(&mut s, "agora 100");
    s
}
fn binding(second: bool) -> Binding {
    Binding {
        id: eid(if second { 3802 } else { 3801 }),
        item: n(if second { 39011 } else { 39010 }),
        observation: obs(if second { "101001" } else { "110001" }),
        concept: n(3501),
        relational_concept: n(3101),
        destination: n(3105),
        scope: scope(),
        source: SourceId::new(9).unwrap(),
        evidence: EvidenceId::new(if second { 49902 } else { 49901 }).unwrap(),
        origin: BindingOrigin::ExternalSyntheticObservation,
    }
}
fn registry(second: bool) -> SyntheticRegistry {
    // Registro declarado separadamente dos pedidos, sem consultar o candidato.
    SyntheticRegistry::new(
        Binding {
            id: eid(if second { 3802 } else { 3801 }),
            item: n(if second { 39011 } else { 39010 }),
            observation: obs(if second { "101001" } else { "110001" }),
            concept: n(3501),
            relational_concept: n(3101),
            destination: n(3105),
            scope: scope(),
            source: SourceId::new(9).unwrap(),
            evidence: EvidenceId::new(if second { 49902 } else { 49901 }).unwrap(),
            origin: BindingOrigin::ExternalSyntheticObservation,
        },
        1,
    )
    .unwrap()
}
fn grants(b: &Binding) -> Permissions {
    Permissions {
        conceptual: Some(scope()),
        relational: Some(PathAuthorization::inclusion_three(scope())),
        composition: Some(b.clone()),
    }
}
fn episode(b: Binding, p: Permissions, l: Limits) -> Episode {
    Episode::new(eid(if b.item == n(39011) { 39002 } else { 39001 }), b, p, l).unwrap()
}
fn unchanged(s: &Session, bytes: &[u8], snapshot: &str) {
    assert_eq!(s.encode(), bytes);
    assert_eq!(s.snapshot(), snapshot);
}
/// Referência: tabelas booleanas e grafo admitido, sem family/revise/evaluate/verify_path.
fn reference(
    s: &Session,
    root: &SyntheticRegistry,
    b: &Binding,
    p: &Permissions,
    r: &Report,
) -> Result<Status, String> {
    let fail = |m: &str| Err(m.to_string());
    if r.binding != *b
        || r.permissions != *p
        || r.objective != eid(if b.item == n(39011) { 39002 } else { 39001 })
    {
        return fail("identidade/autoridade");
    }
    let allowed = root.record() == b
        && b.origin == BindingOrigin::ExternalSyntheticObservation
        && p.conceptual == Some(scope())
        && p.relational == Some(PathAuthorization::inclusion_three(scope()))
        && p.composition.as_ref() == Some(b);
    if !allowed {
        return if r.status == Status::Denied && r.conceptual.is_none() && r.relational.is_none() {
            Ok(Status::Denied)
        } else {
            fail("consulta negada executada")
        };
    }
    if r.dependencies.state != s.encode() || r.dependencies.registry_revision != root.revision() {
        return fail("dependência obsoleta");
    }
    let c = s.concepts().ok_or("conceito")?;
    let examples = c.examples().map_err(|e| format!("{e:?}"))?;
    let cr = r.conceptual.as_ref().ok_or("relatório conceitual")?;
    if cr.version.as_ref() != Some(c.models()[0].current())
        || cr.consulted != examples
        || cr.scope != scope()
        || cr.at != s.logical_time()
        || cr.episode != r.objective
        || cr.observation != b.observation
    {
        return fail("versão/evidência conceitual");
    }
    let contradictory = examples.iter().enumerate().any(|(i, a)| {
        examples[..i].iter().any(|z| {
            a.example.attributes == z.example.attributes && a.example.label != z.example.label
        })
    });
    let mut rules: Vec<(u8, Option<bool>)> = (0..6).map(|i| (1 << i, None)).collect();
    for i in 0..6 {
        for j in i + 1..6 {
            rules.push(((1 << i) | (1 << j), None));
        }
    }
    rules.extend([(0, Some(false)), (0, Some(true))]);
    let predict = |h: (u8, Option<bool>), a: [bool; 6]| {
        h.1.unwrap_or_else(|| a.iter().enumerate().all(|(i, v)| h.0 & (1 << i) == 0 || *v))
    };
    for e in &examples {
        if e.metadata.scope != scope()
            || ![7, 8].contains(&e.metadata.source.get())
            || e.metadata.incorporated_at > s.logical_time()
            || e.record.status() != RecordStatus::Observed
            || e.record.evidence() != Some(e.metadata.evidence)
        {
            return fail("proveniência conceitual");
        }
    }
    rules.retain(|h| {
        examples.iter().all(|e| {
            predict(*h, e.example.attributes.values()) == (e.example.label == Label::Positive)
        })
    });
    let votes = rules
        .iter()
        .map(|h| predict(*h, b.observation.complete().unwrap().values()))
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
    if cd != cr.decision || cr.version.as_ref().unwrap().criteria.len() != rules.len() {
        return fail("classification/H23");
    }
    let rr = r.relational.as_ref().ok_or("relatório relacional")?;
    if rr.id != r.objective
        || rr.query.subject() != b.relational_concept
        || rr.query.object() != b.destination
        || rr.query.scope() != scope()
        || rr.at != s.logical_time()
        || rr.before != rr.after
        || rr.rule != scope().rule
    {
        return fail("consulta relacional");
    }
    let eligible = s
        .metadata()
        .iter()
        .filter(|m| {
            m.state == EpistemicState::AdmittedObservation
                && m.disposition.is_none()
                && m.information.statement.scope() == scope()
                && m.information.nature == DeclaredNature::Observation
                && m.information.evidence.is_some()
                && m.information
                    .declared_source
                    .is_some_and(|v| [7, 8].contains(&v.get()))
                && m.information.received_at <= s.logical_time()
                && scope().validity.contains(s.logical_time())
        })
        .collect::<Vec<_>>();
    if rr.consulted.len() != eligible.len()
        || rr.consulted.iter().any(|r| {
            !eligible.contains(&&r.metadata)
                || s.memory().get(r.engram.id()).ok() != Some(r.engram)
                || r.engram.status() != RecordStatus::Observed
                || r.engram.observed_at() > s.logical_time()
                || !scope().validity.contains(r.engram.observed_at())
        })
    {
        return fail("projeção/eligibilidade");
    }
    let positive = eligible
        .iter()
        .filter(|m| m.information.statement.polarity() == Polarity::Positive)
        .copied()
        .collect::<Vec<_>>();
    let paths = positive
        .iter()
        .flat_map(|a| {
            positive.iter().filter_map(move |z| {
                let x = a.information.statement;
                let y = z.information.statement;
                (x.subject() == b.relational_concept
                    && x.object() == y.subject()
                    && y.object() == b.destination
                    && x.subject() != x.object()
                    && y.subject() != y.object()
                    && x.subject() != y.object()
                    && a.id != z.id)
                    .then_some([*a, *z])
            })
        })
        .collect::<Vec<_>>();
    let opposed = eligible.iter().any(|m| {
        m.information.statement.polarity() == Polarity::Negative && {
            let x = m.information.statement;
            (x.subject() == b.relational_concept && x.object() == b.destination)
                || paths.iter().any(|path| {
                    path.iter().any(|z| {
                        z.information.statement.subject() == x.subject()
                            && z.information.statement.object() == x.object()
                    })
                })
        }
    });
    let shared = examples.iter().any(|e| e.metadata.evidence == b.evidence)
        || paths.iter().any(|path| {
            path[0].information.evidence == path[1].information.evidence
                || path.iter().any(|m| {
                    m.information.evidence == Some(b.evidence)
                        || examples
                            .iter()
                            .any(|e| Some(e.metadata.evidence) == m.information.evidence)
                })
        });
    if let Some(proof) = &rr.proof {
        if proof.premises.len() != 2
            || !paths.iter().any(|path| {
                proof
                    .premises
                    .iter()
                    .zip(path)
                    .all(|(p, m)| p.metadata == **m && s.memory().get(m.id).ok() == Some(p.engram))
            })
        {
            return fail("prova adulterada/ausente");
        }
    }
    let expected = if shared {
        Status::Denied
    } else if opposed || contradictory {
        Status::Contradictory
    } else if cd == ConceptDecision::Supported(Label::Positive) && !paths.is_empty() {
        Status::Supported
    } else {
        Status::Insufficient
    };
    if r.status != expected
        || (expected == Status::Supported && (rr.proof.is_none() || rr.refusal.is_some()))
    {
        return fail("resultado divergente");
    }
    Ok(expected)
}
// Linux: custo CPU do processo em ticks e RSS nas fronteiras, não pico por fase.
// Executar este alvo com --test-threads=1 para não misturar fases de outros testes.
fn phase<T>(name: &str, f: impl FnOnce() -> T) -> T {
    fn reading() -> (u64, u64) {
        let stat = std::fs::read_to_string("/proc/self/stat").unwrap();
        let fields: Vec<_> = stat
            .rsplit_once(')')
            .unwrap()
            .1
            .split_whitespace()
            .collect();
        let cpu = fields[11].parse::<u64>().unwrap() + fields[12].parse::<u64>().unwrap();
        let status = std::fs::read_to_string("/proc/self/status").unwrap();
        let rss = status
            .lines()
            .find(|s| s.starts_with("VmRSS:"))
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();
        (cpu, rss)
    }
    let before = reading();
    let t = std::time::Instant::now();
    let value = f();
    let elapsed = t.elapsed();
    let after = reading();
    println!(
        "PHASE {name} wall={elapsed:?} cpu_ticks={} rss_before_kib={} rss_after_kib={}",
        after.0 - before.0,
        before.1,
        after.1
    );
    value
}
fn run(
    name: &str,
    s: &Session,
    b: Binding,
    root: &SyntheticRegistry,
    p: Permissions,
    expected: Status,
) {
    let bytes = s.encode();
    let snap = s.snapshot();
    let mut e = episode(b.clone(), p.clone(), Limits::default());
    let t = std::time::Instant::now();
    let r = phase("candidate_query", || e.query(s, root).unwrap());
    let candidate = t.elapsed();
    unchanged(s, &bytes, &snap);
    assert_eq!(r.status, expected, "{name}: {r:?}");
    if matches!(name, "P01" | "P02") {
        assert_eq!(
            r.conceptual
                .as_ref()
                .unwrap()
                .version
                .as_ref()
                .unwrap()
                .number,
            2
        );
        assert_eq!(
            r.relational
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
    }
    let t = std::time::Instant::now();
    assert_eq!(
        phase("independent_reference", || reference(s, root, &b, &p, &r)
            .unwrap()),
        expected
    );
    let ref_time = t.elapsed();
    assert_eq!(
        phase("candidate_verification", || e.verify(s, root, &r).unwrap()),
        Verification::Verified(expected)
    );
    unchanged(s, &bytes, &snap);
    let u = e.usage();
    assert_eq!(u.conceptual_queries, 2);
    assert_eq!(u.relational_queries, 2);
    assert!(u.neural_events <= 64);
    assert_eq!(u.teaching_attempts, 0);
    assert_eq!(u.conceptual_events, 0);
    assert_eq!(
        u.neural_events,
        2 * r
            .relational
            .as_ref()
            .unwrap()
            .review
            .as_ref()
            .unwrap()
            .inference
            .processed_events
    );
    println!("CASE {name} expected={expected:?} observed={:?} usage={u:?} candidate={candidate:?} reference={ref_time:?}",r.status);
}
#[test]
fn positives_and_f2_controls() {
    for second in [false, true] {
        let t = std::time::Instant::now();
        let s = phase("preparation", || base(true, true));
        println!("preparation {:?}", t.elapsed());
        let b = binding(second);
        let root = registry(second);
        run(
            if second { "P02" } else { "P01" },
            &s,
            b.clone(),
            &root,
            grants(&b),
            Status::Supported,
        );
        let mut f2 = neurons::integration::composed::Episode::new(
            &s,
            eid(if second { 39002 } else { 39001 }),
            b.clone(),
        );
        assert_eq!(
            f2.query(&s, &root, &grants(&b)).unwrap().status,
            Status::Interrupted
        );
        let mut original = Session::decode(&s.encode()).unwrap();
        let before = original.encode();
        let report = neurons::integration::composed::Episode::new(
            &original,
            eid(if second { 39002 } else { 39001 }),
            b.clone(),
        )
        .supervise(&mut original, &root, grants(&b), None);
        assert_eq!(
            report.diagnosis,
            neurons::integration::composed::GoalDiagnosis::ContractBlocked
        );
        assert_eq!(
            report.outcome,
            neurons::integration::composed::GoalOutcome::Interrupted
        );
        assert_eq!(original.encode(), before);
        println!(
            "C-F2 expected=ContractBlocked/Interrupted observed={:?}/{:?}",
            report.diagnosis, report.outcome
        );
    }
}
#[test]
fn missing_each_premise() {
    for (i, (a, b)) in [(false, true), (true, false)].into_iter().enumerate() {
        let s = base(a, b);
        let goal = binding(false);
        run(
            &format!("N0{}", i + 1),
            &s,
            goal.clone(),
            &registry(false),
            grants(&goal),
            Status::Insufficient,
        );
    }
}
#[test]
fn relational_opposition() {
    for (a, b) in [(3103, 3105), (3101, 3105)] {
        let mut s = seed();
        relation(&mut s, 3111, 3101, 3103, "+", 4811);
        relation(&mut s, 3112, 3103, 3105, "+", 4812);
        relation(&mut s, 3113, a, b, "-", 4813);
        apply(&mut s, "agora 100");
        let g = binding(false);
        run(
            "N03",
            &s,
            g.clone(),
            &registry(false),
            grants(&g),
            Status::Contradictory,
        );
    }
}
#[test]
fn conceptual_opposition() {
    let mut s = seed();
    example(&mut s, 3403, 3603, "100001", "-", 4403);
    relation(&mut s, 3111, 3101, 3103, "+", 4811);
    relation(&mut s, 3112, 3103, 3105, "+", 4812);
    apply(&mut s, "agora 100");
    let b = binding(false);
    run(
        "N04",
        &s,
        b.clone(),
        &registry(false),
        grants(&b),
        Status::Contradictory,
    );
}
#[test]
fn invalid_binding_and_separate_permissions() {
    let s = base(true, true);
    for i in 0..4 {
        let mut b = binding(false);
        if i == 0 {
            b.item = n(39012);
        }
        let mut p = grants(&b);
        match i {
            1 => p.conceptual = None,
            2 => p.relational = None,
            3 => p.composition = None,
            _ => (),
        };
        let root = registry(false);
        let bytes = s.encode();
        let mut e = episode(b.clone(), p.clone(), Limits::default());
        let r = e.query(&s, &root).unwrap();
        assert_eq!(reference(&s, &root, &b, &p, &r).unwrap(), Status::Denied);
        assert_eq!(e.usage().conceptual_queries, 0);
        assert_eq!(e.usage().relational_queries, 0);
        assert_eq!(s.encode(), bytes);
    }
}
#[test]
fn budgets_reserved_before_cognition() {
    let s = base(true, true);
    for l in [
        Limits {
            conceptual_queries: 1,
            ..Limits::default()
        },
        Limits {
            relational_queries: 1,
            ..Limits::default()
        },
        Limits {
            neural_events: 63,
            ..Limits::default()
        },
    ] {
        let b = binding(false);
        let mut e = episode(b.clone(), grants(&b), l);
        let r = e.query(&s, &registry(false)).unwrap();
        assert_eq!(r.status, Status::Interrupted);
        assert_eq!(e.usage().conceptual_queries, 0);
        assert_eq!(e.usage().relational_queries, 0);
        assert_eq!(
            e.verify(&s, &registry(false), &r).unwrap(),
            Verification::Interrupted
        );
    }
}
#[test]
fn evidence_shared_across_domains_or_origin() {
    for ev in [4401, 49901] {
        let mut s = seed();
        relation(&mut s, 3111, 3101, 3103, "+", ev);
        relation(&mut s, 3112, 3103, 3105, "+", 4812);
        apply(&mut s, "agora 100");
        let b = binding(false);
        run(
            "N08",
            &s,
            b.clone(),
            &registry(false),
            grants(&b),
            Status::Denied,
        );
    }
}
#[test]
fn duplicate_evidence_preparation_is_refused() {
    let mut s = seed();
    relation(&mut s, 3111, 3101, 3103, "+", 4811);
    receive(&mut s, 3112, 3103, 3105, "+", 4811);
    let bytes = s.encode();
    assert!(s.execute("admitir").is_err());
    assert_eq!(s.encode(), bytes);
    assert!(s.memory().get(EngramId::new(3112).unwrap()).is_err());
    println!("N09 preparation refused; no composed query");
}
#[test]
fn circular_origin_preparation_is_refused() {
    let mut b = binding(false);
    b.origin = BindingOrigin::ComposedReport(eid(39001));
    assert!(SyntheticRegistry::new(b, 1).is_err());
    println!("N10 constructor refused; no composed query");
}
#[test]
fn cyclic_graph_never_proves_destination() {
    let mut s = seed();
    relation(&mut s, 3111, 3101, 3103, "+", 4811);
    relation(&mut s, 3112, 3103, 3101, "+", 4812);
    apply(&mut s, "agora 100");
    let b = binding(false);
    run(
        "N11",
        &s,
        b.clone(),
        &registry(false),
        grants(&b),
        Status::Insufficient,
    );
}
#[test]
fn tampered_reports_are_rejected_by_both_verifiers() {
    let s = base(true, true);
    for i in 0..3 {
        let b = binding(false);
        let p = grants(&b);
        let root = registry(false);
        let mut e = episode(b.clone(), p.clone(), Limits::default());
        let mut r = e.query(&s, &root).unwrap();
        match i {
            0 => {
                r.conceptual.as_mut().unwrap().decision =
                    ConceptDecision::Supported(Label::Negative)
            }
            1 => {
                r.relational
                    .as_mut()
                    .unwrap()
                    .proof
                    .as_mut()
                    .unwrap()
                    .premises[1] = r
                    .relational
                    .as_ref()
                    .unwrap()
                    .proof
                    .as_ref()
                    .unwrap()
                    .premises[0]
            }
            _ => {
                r.conceptual
                    .as_mut()
                    .unwrap()
                    .version
                    .as_mut()
                    .unwrap()
                    .number = 1
            }
        };
        assert!(reference(&s, &root, &b, &p, &r).is_err());
        assert_eq!(e.verify(&s, &root, &r).unwrap(), Verification::Rejected);
        assert_eq!(e.usage().relational_queries, 1);
    }
}
#[test]
fn obsolete_reports_are_rejected() {
    let s = base(true, true);
    for i in 0..3 {
        let mut other = base(true, true);
        let b = binding(false);
        let p = grants(&b);
        let root = registry(false);
        let revised = SyntheticRegistry::new(b.clone(), 2).unwrap();
        if i == 0 {
            apply(&mut other, "agora 101");
        }
        if i == 2 {
            other = seed();
            example(&mut other, 3403, 3603, "101001", "+", 4403);
            relation(&mut other, 3111, 3101, 3103, "+", 4811);
            relation(&mut other, 3112, 3103, 3105, "+", 4812);
            apply(&mut other, "agora 100");
        }
        let target = if i == 1 { &revised } else { &root };
        // Ambas as fixtures existem antes de iniciar a unidade candidata.
        let mut e = episode(b.clone(), p.clone(), Limits::default());
        let r = e.query(&s, &root).unwrap();
        assert!(reference(&other, target, &b, &p, &r).is_err());
        assert_eq!(
            e.verify(&other, target, &r).unwrap(),
            Verification::ReevaluationRequired
        );
    }
}
#[test]
fn repeat_and_authority_reuse_are_refused() {
    let s = base(true, true);
    let b = binding(false);
    let p = grants(&b);
    let mut e = episode(b.clone(), p.clone(), Limits::default());
    let root = registry(false);
    let r = e.query(&s, &root).unwrap();
    assert!(e.query(&s, &root).is_err());
    assert_eq!(
        e.verify(&s, &root, &r).unwrap(),
        Verification::Verified(Status::Supported)
    );
    let u = e.usage();
    assert!(e.query(&s, &root).is_err());
    assert!(e.verify(&s, &root, &r).is_err());
    assert_eq!(e.usage(), u);
    let mut e2 = episode(binding(true), p, Limits::default());
    assert_eq!(
        e2.query(&s, &registry(true)).unwrap().status,
        Status::Denied
    );
    assert_eq!(e2.usage().conceptual_queries, 0);
}
#[test]
fn nonexistent_and_wrong_context_proofs_are_rejected() {
    let full = base(true, true);
    let missing = base(true, false);
    let b = binding(false);
    let p = grants(&b);
    let root = registry(false);
    let mut e = episode(b.clone(), p.clone(), Limits::default());
    let r = e.query(&full, &root).unwrap();
    assert!(reference(&missing, &root, &b, &p, &r).is_err());
    assert_eq!(
        e.verify(&missing, &root, &r).unwrap(),
        Verification::ReevaluationRequired
    );
    let mut e = episode(b.clone(), p.clone(), Limits::default());
    let mut r = e.query(&full, &root).unwrap();
    r.relational
        .as_mut()
        .unwrap()
        .proof
        .as_mut()
        .unwrap()
        .premises[1]
        .metadata
        .information
        .statement = {
        let mut sc = scope();
        sc.context = ContextId::new(342).unwrap();
        let a = Concept::new(n(3103), sc.catalog, "b", "b").unwrap();
        let z = Concept::new(n(3105), sc.catalog, "d", "d").unwrap();
        Statement::inclusion(&a, &z, Polarity::Positive, sc).unwrap()
    };
    assert!(reference(&full, &root, &b, &p, &r).is_err());
    assert_eq!(e.verify(&full, &root, &r).unwrap(), Verification::Rejected);
}
#[test]
fn negative_and_insufficient_classification_do_not_teach() {
    let s = base(true, true);
    for (name, bits) in [("N16", "010001"), ("N17", "100010")] {
        let mut b = binding(false);
        b.observation = obs(bits);
        let root = SyntheticRegistry::new(b.clone(), 1).unwrap();
        run(name, &s, b.clone(), &root, grants(&b), Status::Insufficient);
    }
}
