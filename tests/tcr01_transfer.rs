//! TCR-01: protocolo público congelado na Etapa A; somente harness, sem produção.
use neurons::cognition::concepts::{Attributes, Label, Observation};
use neurons::cognitive_contracts::*;
use neurons::curriculum::{
    teach_concept, Lesson, PedagogyAssessment, PedagogyHost, PedagogyPermissions, TeachingPlan,
};
use neurons::engram::{EngramId, EvidenceId, RecordStatus};
use neurons::event::NodeId;
use neurons::integration::composed::Episode;
use neurons::integration::composed::*;
use neurons::integration::query::{PathAuthorization, QueryCoordinator, RuleAuthorization};
use neurons::metacognition::concepts::{Abstention, ConceptDecision};
use neurons::session::f1::Session;
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
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

const P0: [(&str, Label); 2] = [("111111", Label::Positive), ("000000", Label::Negative)];
const P1: [(&str, Label); 2] = [("101111", Label::Negative), ("111101", Label::Negative)];
const P2: [&str; 8] = [
    "010010", "110011", "011110", "111010", "010000", "001010", "101100", "100111",
];
const AUX: [&str; 2] = ["101010", "110110"];
fn node(n: u32) -> NodeId {
    NodeId::new(n).unwrap()
}
fn eid(n: u32) -> EpisodeId {
    EpisodeId::new(n).unwrap()
}
fn scope() -> Scope {
    Scope {
        catalog: CatalogId::new(191).unwrap(),
        context: ContextId::new(141).unwrap(),
        rule: RuleRef {
            id: RuleId::new(151).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 10000).unwrap(),
    }
}
fn attrs(s: &str) -> Attributes {
    Attributes::new(
        s.as_bytes()
            .try_into()
            .map(|a: [u8; 6]| a.map(|x| x - b'0'))
            .unwrap(),
    )
    .unwrap()
}
fn obs(s: &str) -> Observation {
    Observation::new(
        s.as_bytes()
            .try_into()
            .map(|a: [u8; 6]| a.map(|x| Some(x - b'0')))
            .unwrap(),
    )
    .unwrap()
}
// Somente o avaliador usa esta regra; nenhum seletor/consulta recebe a chave.
fn external(bits: &str) -> Label {
    if bits.as_bytes()[1] == b'1' && bits.as_bytes()[4] == b'1' {
        Label::Positive
    } else {
        Label::Negative
    }
}
fn apply(s: &mut Session, line: &str) {
    s.execute(line).unwrap_or_else(|e| panic!("{line}: {e}"));
}
fn seed(s: &mut Session, id: u32, bits: &str, label: Label) {
    for line in [
        format!(
            "exemplo {id} {} 141 {bits} {} 7 {} observacao",
            id + 1000,
            if label == Label::Positive { "+" } else { "-" },
            id + 10000
        ),
        "interpretar-exemplo".into(),
        "avaliar-exemplo".into(),
        format!("admitir-exemplo {id}"),
        "revisar".into(),
    ] {
        apply(s, &line);
    }
}
fn base(relation: bool) -> Session {
    let mut s = Session::new();
    for c in [
        "config 191 141 151 1 0 10000",
        "simbolo 1101 c-tcr",
        "simbolo 1103 d-tcr",
        "autorizar 141",
        "conceito 1501 opaco-tcr",
    ] {
        apply(&mut s, c);
    }
    for (i, (bits, label)) in P0.iter().enumerate() {
        seed(&mut s, 1401 + i as u32, bits, *label);
    }
    if relation {
        for c in [
            "receber 111 141 1101 1103 + 8 18011 observacao",
            "interpretar",
            "avaliar",
            "admitir",
        ] {
            apply(&mut s, c);
        }
    }
    s
}
fn lessons() -> Vec<Lesson> {
    P1.iter()
        .enumerate()
        .map(|(i, (bits, label))| {
            let n = 1403 + i as u32;
            Lesson {
                id: EngramId::new(n).unwrap(),
                receipt: n,
                exemplar: node(n + 1000),
                attributes: attrs(bits),
                label: *label,
                source: SourceId::new(7).unwrap(),
                evidence: EvidenceId::new(n + 10000).unwrap(),
            }
        })
        .collect()
}
fn teach(s: &mut Session) -> (usize, usize, Vec<EngramId>) {
    let before = s.pedagogy_events();
    let report = teach_concept(
        s,
        TeachingPlan::new(node(1501), scope(), &lessons()).unwrap(),
        PedagogyAssessment::new(&[obs(AUX[0])], &[obs(AUX[1])]).unwrap(),
        PedagogyPermissions {
            evaluation: Some(scope()),
            presentation: Some(scope()),
            admissions: vec![EngramId::new(1403).unwrap(), EngramId::new(1404).unwrap()],
        },
    )
    .unwrap();
    println!(
        "TCR01 teaching diagnosis={:?} versions={}=>{} lessons={:?}",
        report.diagnosis,
        report.initial_version.number,
        report.final_version.number,
        report.lessons
    );
    assert!(!report.leak_detected);
    assert_eq!(report.lessons.len(), 2);
    assert!(report.lessons.iter().all(|l| l.admitted));
    assert_eq!(s.concepts().unwrap().examples().unwrap().len(), 4);
    (
        report.initial.len() + report.posterior.len(),
        s.pedagogy_events() - before,
        report
            .lessons
            .iter()
            .filter(|l| l.admitted)
            .map(|l| l.lesson.id)
            .collect(),
    )
}
// Registro fixo sem rótulo, estabelecido antes da consulta; IDs não têm significado epistêmico.
fn binding(i: usize) -> Binding {
    Binding {
        id: eid(2800 + i as u32),
        item: node(2900 + i as u32),
        observation: obs(P2[i]),
        concept: node(1501),
        relational_concept: node(1101),
        destination: node(1103),
        scope: scope(),
        source: SourceId::new(9).unwrap(),
        evidence: EvidenceId::new(29900 + i as u32).unwrap(),
        origin: BindingOrigin::ExternalSyntheticObservation,
    }
}
fn root(i: usize) -> SyntheticRegistry {
    let mut b = binding(i);
    b.observation = obs(P2[i]);
    SyntheticRegistry::new(b, 1).unwrap()
}
fn permissions(b: &Binding) -> Permissions {
    Permissions {
        conceptual: Some(scope()),
        relational: Some(PathAuthorization::inclusion_three(scope())),
        composition: Some(b.clone()),
    }
}
#[derive(Default, Debug)]
struct Totals {
    conceptual: usize,
    relational: usize,
    neural: usize,
    concept_events: usize,
}
impl Totals {
    fn add(&mut self, u: Usage) {
        assert!(u.conceptual_queries <= 8 && u.relational_queries <= 3 && u.neural_events <= 96);
        self.conceptual += u.conceptual_queries;
        self.relational += u.relational_queries;
        self.neural += u.neural_events;
        self.check();
    }
    fn check(&self) {
        assert!(self.conceptual <= 120 && self.relational <= 120 && self.neural <= 3840);
    }
}
fn exclusive(path: &Path, bytes: &[u8]) {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    f.write_all(bytes).unwrap();
}
fn evaluate(
    s: &Session,
    group: &str,
    dir: &Path,
    denial: Option<usize>,
    tot: &mut Totals,
) -> String {
    let baseline = s.encode();
    let mut csv=String::from("group,item,bits,truth,conceptual,composed,version,conceptual_queries,relational_queries,neural_events,concept_events,query_ns,production_verify_ns,reference_ns\n");
    let mut signatures = String::new();
    for (i, bits) in P2.iter().enumerate() {
        let b = binding(i);
        let root = root(i);
        let mut p = permissions(&b);
        if let Some(k) = denial {
            match k {
                0 => p.conceptual = None,
                1 => p.relational = None,
                2 => p.composition = None,
                _ => unreachable!(),
            }
        }
        let mut episode = Episode::new(s, eid(3900 + i as u32), b);
        let t = Instant::now();
        let report = episode.query(s, &root, &p).unwrap();
        let qns = t.elapsed().as_nanos();
        let mut vns = 0;
        let mut rns = 0;
        if denial.is_some() {
            assert_eq!(report.status, Status::Denied);
            assert!(report.conceptual.is_none() && report.relational.is_none());
            assert_eq!(episode.usage(), Usage::default());
        } else {
            let t = Instant::now();
            assert_eq!(
                reference_verify(s, &root, &report, &p).unwrap(),
                report.status
            );
            rns = t.elapsed().as_nanos();
            let t = Instant::now();
            assert_eq!(
                episode.verify(s, &root, &report, &p).unwrap(),
                Verification::Verified(report.status)
            );
            vns = t.elapsed().as_nanos();
        }
        assert_eq!(s.encode(), baseline, "consulta alterou conhecimento");
        let u = episode.usage();
        tot.add(u);
        let decision = report
            .conceptual
            .as_ref()
            .map(|c| match c.decision {
                ConceptDecision::Supported(Label::Positive) => "positive",
                ConceptDecision::Supported(Label::Negative) => "negative",
                ConceptDecision::Abstain(_) => "abstain",
            })
            .unwrap_or("denied");
        let version = report
            .conceptual
            .as_ref()
            .and_then(|c| c.version.as_ref())
            .map_or(0, |v| v.number);
        // A chave externa é aplicada só depois das consultas, nunca ao escolher ensino.
        csv.push_str(&format!(
            "{group},E{:02},{bits},{:?},{decision},{:?},{version},{},{},{},{},{qns},{vns},{rns}\n",
            i + 1,
            external(bits),
            report.status,
            u.conceptual_queries,
            u.relational_queries,
            u.neural_events,
            u.conceptual_events
        ));
        signatures.push_str(&format!(
            "{:?}\n{:?}\n",
            report.conceptual, report.relational
        ));
    }
    exclusive(&dir.join(format!("{group}.csv")), csv.as_bytes());
    exclusive(
        &dir.join(format!("{group}.signature")),
        signatures.as_bytes(),
    );
    csv
}
fn c4(s: &Session, dir: &Path, tot: &mut Totals) {
    let bytes = s.encode();
    let mut out = String::from("item,bits,truth,decision\n");
    for (i, (bits, _)) in P1.iter().enumerate() {
        let r = s.pedagogy_query(eid(4900 + i as u32), obs(bits)).unwrap();
        tot.conceptual += 1;
        out.push_str(&format!(
            "L{},{bits},{:?},{:?}\n",
            i + 1,
            external(bits),
            r.decision
        ));
    }
    tot.check();
    assert_eq!(s.encode(), bytes);
    exclusive(&dir.join("C4.csv"), out.as_bytes());
}

fn add_relation(s: &mut Session) {
    for line in [
        "receber 111 141 1101 1103 + 8 18011 observacao",
        "interpretar",
        "avaliar",
        "admitir",
    ] {
        apply(s, line);
    }
    assert_eq!(s.memory().len(), 1);
    assert_eq!(s.metadata().len(), 1);
    let m = &s.metadata()[0];
    assert_eq!(m.information.statement.subject(), node(1101));
    assert_eq!(m.information.statement.object(), node(1103));
    assert_eq!(
        m.information.evidence,
        Some(EvidenceId::new(18011).unwrap())
    );
    assert_eq!(m.state, EpistemicState::AdmittedObservation);
}
fn concept_state(s: &Session) -> String {
    let c = s.concepts().unwrap();
    format!(
        "{:?}|{:?}|{:?}|{:?}|{:?}",
        c.concept(),
        c.examples().unwrap(),
        c.models(),
        c.trace(),
        c.memory().iter().collect::<Vec<_>>()
    )
}
fn no_relation(s: &Session) {
    assert_eq!(s.memory().len(), 0);
    assert!(s.metadata().is_empty());
}
fn no_evaluation_admitted(s: &Session) {
    let examples = s.concepts().unwrap().examples().unwrap();
    assert!(examples
        .iter()
        .all(|e| !P2.iter().any(|b| e.example.attributes == attrs(b))));
}
fn worker(mode: &str, dir: &Path) {
    let mut tot = Totals::default();
    if mode == "run" {
        let started = Instant::now();
        let k0 = base(false);
        no_relation(&k0);
        let initialization_ns = started.elapsed().as_nanos();
        let initialization_events = k0.pedagogy_events();
        let k0_bytes = k0.encode();
        k0.save(&dir.join("K0.f1")).unwrap();
        let mut replay_events = 0;
        let mut replay_count = 0;
        let mut replay_ns = 0;
        let mut reconstruct = |bytes: &[u8]| {
            let t = Instant::now();
            let state = Session::decode(bytes).unwrap();
            replay_ns += t.elapsed().as_nanos();
            replay_events += state.pedagogy_events();
            replay_count += 1;
            assert_eq!(state.encode(), bytes);
            state
        };
        let mut initial_t = reconstruct(&k0_bytes);
        let mut control = reconstruct(&k0_bytes);
        add_relation(&mut initial_t);
        add_relation(&mut control);
        assert_eq!(initial_t.encode(), control.encode());
        let control_bytes = control.encode();
        evaluate(&initial_t, "T_pre", dir, None, &mut tot);
        evaluate(&control, "C0_pre", dir, None, &mut tot);
        let mut learner = reconstruct(&k0_bytes);
        no_relation(&learner);
        let t = Instant::now();
        let (q, e, admitted) = teach(&mut learner);
        let teaching_ns = t.elapsed().as_nanos();
        assert_eq!(
            admitted,
            vec![EngramId::new(1403).unwrap(), EngramId::new(1404).unwrap()]
        );
        tot.conceptual += q;
        tot.concept_events += e;
        tot.check();
        no_relation(&learner);
        no_evaluation_admitted(&learner);
        // K1 estende somente o journal de K0. Nenhum byte do codec é editado.
        assert!(learner.history().starts_with(k0.history()));
        let k1_bytes = learner.encode();
        learner.save(&dir.join("K1.f1")).unwrap();
        let mut treatment = reconstruct(&k1_bytes);
        let c1 = reconstruct(&k1_bytes);
        assert_eq!(treatment.encode(), c1.encode());
        let common_concept = concept_state(&treatment);
        assert_eq!(common_concept, concept_state(&c1));
        no_relation(&treatment);
        no_relation(&c1);
        let c1_bytes = c1.encode();
        add_relation(&mut treatment);
        assert_eq!(common_concept, concept_state(&treatment));
        assert_eq!(common_concept, concept_state(&c1));
        assert_eq!(c1.encode(), c1_bytes);
        no_relation(&c1);
        evaluate(&treatment, "T", dir, None, &mut tot);
        evaluate(&control, "C0", dir, None, &mut tot);
        assert_eq!(control.encode(), control_bytes);
        evaluate(&c1, "C1", dir, None, &mut tot);
        no_relation(&c1);
        assert_eq!(c1.encode(), c1_bytes);
        assert_eq!(concept_state(&treatment), concept_state(&c1));
        for k in 0..3 {
            evaluate(&treatment, &format!("C2_{k}"), dir, Some(k), &mut tot);
        }
        c4(&treatment, dir, &mut tot);
        no_evaluation_admitted(&treatment);
        no_evaluation_admitted(&control);
        no_evaluation_admitted(&c1);
        assert_eq!(admitted.len(), 2);
        exclusive(&dir.join("new-teaching-admissions.txt"), b"1403\n1404\n");
        exclusive(
            &dir.join("concept-equivalence.txt"),
            common_concept.as_bytes(),
        );
        exclusive(&dir.join("integrity.json"),br#"{"conceptual_state_equal":true,"relation_addition_preserves_concepts":true,"C1_no_relation_before_and_after":true,"T_independent_relation_admitted":true,"initial_T_C0_equal":true,"no_evaluation_admitted":true}"#);
        treatment.save(&dir.join("knowledge.f1")).unwrap();
        exclusive(&dir.join("run-counters.txt"),format!("{tot:?}\nteaching_ns={teaching_ns}\ncheckpoint_bytes={}\ninitialization_concept_events={initialization_events}\ninitialization_ns={initialization_ns}\nexplicit_replay_count={replay_count}\nreplayed_concept_events={replay_events}\nexplicit_replay_ns={replay_ns}\nnew_teaching_operations=1\nnew_teaching_admissions={}\n",treatment.encode().len(),admitted.len()).as_bytes());
    } else {
        let t = Instant::now();
        let restored = Session::load(&dir.join("knowledge.f1")).unwrap();
        let load_ns = t.elapsed().as_nanos();
        let baseline = fs::read(dir.join("knowledge.f1")).unwrap();
        assert_eq!(restored.encode(), baseline);
        let b = binding(0);
        let mut episode = Episode::new(&restored, eid(5900), b);
        let denied = Permissions {
            conceptual: None,
            relational: None,
            composition: None,
        };
        assert_eq!(
            episode.query(&restored, &root(0), &denied).unwrap().status,
            Status::Denied
        );
        assert_eq!(episode.usage(), Usage::default());
        evaluate(&restored, "C3", dir, None, &mut tot);
        assert_eq!(
            fs::read(dir.join("T.signature")).unwrap(),
            fs::read(dir.join("C3.signature")).unwrap(),
            "decisões/versões/evidências não retidas"
        );
        assert_eq!(restored.encode(), baseline);
        no_evaluation_admitted(&restored);
        restored.save(&dir.join("restored.f1")).unwrap();
        assert_eq!(fs::read(dir.join("restored.f1")).unwrap(), baseline);
        exclusive(&dir.join("restore-counters.txt"),format!("{tot:?}\nload_ns={load_ns}\nexplicit_replay_count=1\nreplayed_concept_events={}\nnew_teaching_operations=0\nnew_teaching_admissions=0\n",restored.pedagogy_events()).as_bytes());
    }
}

fn counter(dir: &Path, file: &str, key: &str) -> usize {
    let text = fs::read_to_string(dir.join(file)).unwrap();
    text.split(&format!("{key}: "))
        .nth(1)
        .unwrap()
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}
#[derive(Default)]
struct Metrics {
    n: usize,
    supported: usize,
    correct: usize,
    errors: usize,
    abstentions: usize,
    denied: usize,
    composite: usize,
    composite_errors: usize,
}
fn metrics(dir: &Path, group: &str) -> Metrics {
    let mut m = Metrics::default();
    let text = fs::read_to_string(dir.join(format!("{group}.csv"))).unwrap();
    for line in text.lines().skip(1) {
        let f = line.split(',').collect::<Vec<_>>();
        assert_eq!(f.len(), 14);
        m.n += 1;
        let truth = external(f[2]);
        assert_eq!(format!("{truth:?}"), f[3]);
        match f[4] {
            "positive" | "negative" => {
                m.supported += 1;
                if (f[4] == "positive") == (truth == Label::Positive) {
                    m.correct += 1
                } else {
                    m.errors += 1
                }
            }
            "abstain" => m.abstentions += 1,
            "denied" => m.denied += 1,
            _ => panic!("classe inesperada"),
        }
        if f[5] == "Supported" {
            m.composite += 1;
            if truth != Label::Positive || group == "C1" || group.starts_with("C2") {
                m.composite_errors += 1;
            }
        }
    }
    assert_eq!(m.n, 8);
    m
}
fn assess(dir: &Path) {
    let groups = [
        "T_pre", "C0_pre", "T", "C0", "C1", "C2_0", "C2_1", "C2_2", "C3",
    ];
    let mut out=String::from("group,n,concept_supported,concept_correct,concept_errors,concept_abstentions,denied,composite_supported,composite_errors\n");
    for g in groups {
        let m = metrics(dir, g);
        out.push_str(&format!(
            "{g},{},{},{},{},{},{},{},{}\n",
            m.n,
            m.supported,
            m.correct,
            m.errors,
            m.abstentions,
            m.denied,
            m.composite,
            m.composite_errors
        ));
    }
    exclusive(&dir.join("metrics.csv"), out.as_bytes());
    let t = metrics(dir, "T");
    let c0 = metrics(dir, "C0");
    let c1 = metrics(dir, "C1");
    let c3 = metrics(dir, "C3");
    let zero_errors = groups.iter().all(|g| {
        let m = metrics(dir, g);
        m.errors == 0 && m.composite_errors == 0
    });
    let concept_gain = t.correct as isize - c0.correct as isize;
    let composite_gain = t.composite as isize - c0.composite as isize;
    let permissions = (0..3).all(|k| {
        let m = metrics(dir, &format!("C2_{k}"));
        m.denied == 8 && m.composite == 0
    });
    let retained =
        fs::read(dir.join("T.signature")).unwrap() == fs::read(dir.join("C3.signature")).unwrap();
    assert!(dir.join("integrity.json").is_file());
    let cq = counter(dir, "run-counters.txt", "conceptual")
        + counter(dir, "restore-counters.txt", "conceptual");
    let rq = counter(dir, "run-counters.txt", "relational")
        + counter(dir, "restore-counters.txt", "relational");
    let ne =
        counter(dir, "run-counters.txt", "neural") + counter(dir, "restore-counters.txt", "neural");
    assert!(cq <= 120 && rq <= 120 && ne <= 3840);
    let no_relation_preserves_classification = {
        let t = fs::read_to_string(dir.join("T.csv")).unwrap();
        let c = fs::read_to_string(dir.join("C1.csv")).unwrap();
        t.lines().skip(1).zip(c.lines().skip(1)).all(|(a, b)| {
            let a = a.split(',').collect::<Vec<_>>();
            let b = b.split(',').collect::<Vec<_>>();
            a[4] == b[4] && a[6] == b[6]
        })
    };
    let c4 = fs::read_to_string(dir.join("C4.csv")).unwrap();
    let c4_errors = c4
        .lines()
        .skip(1)
        .filter(|l| !l.ends_with("Supported(Negative)"))
        .count();
    let sustained = concept_gain >= 4
        && composite_gain >= 2
        && zero_errors
        && c4_errors == 0
        && c1.composite == 0
        && no_relation_preserves_classification
        && permissions
        && retained
        && c3.correct == t.correct
        && c3.composite == t.composite;
    let ids = fs::read_to_string(dir.join("new-teaching-admissions.txt"))
        .unwrap()
        .lines()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let observed_admissions = ids.len();
    let protocol_valid = ids == vec![1403, 1404];
    assert!(
        protocol_valid,
        "exatamente duas admissões novas explícitas requeridas"
    );
    exclusive(&dir.join("protocol-validity.json"),format!(r#"{{"global_teaching_admissions_limit":2,"observed_teaching_admissions":{},"admitted_ids":[1403,1404],"valid":{},"prior_execution":"invalid_global_teaching_budget","construction":"K0 ensino único K1 bifurcado; relação somente em T"}}"#,observed_admissions,protocol_valid).as_bytes());
    exclusive(&dir.join("assessment.json"),format!(r#"{{"hypothesis":"{}","conceptual_gain_numerator":{},"composite_gain_numerator":{},"denominator":8,"zero_sustained_errors":{},"retained_exactly":{},"c4_errors":{},"conceptual_queries":{},"relational_queries":{},"neural_events":{},"new_conceptual_events":{},"seed_and_replay_events_accounted_separately":true}}"#,if !protocol_valid {"invalid_global_teaching_budget"} else if sustained{"sustained_in_scope"}else{"refuted_in_scope"},concept_gain,composite_gain,zero_errors,retained,c4_errors,cq,rq,ne,counter(dir,"run-counters.txt","concept_events")).as_bytes());
}
fn spawn_worker(mode: &str, dir: &Path) {
    let out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join(format!("{mode}.log")))
        .unwrap();
    let err = out.try_clone().unwrap();
    // Python stdlib observa o processo real: CPU/RSS via wait4, timeout e RSS por /proc.
    let supervisor = r#"
import os,sys,time,json,subprocess,pathlib
start=time.perf_counter_ns()
p=subprocess.Popen([sys.argv[1],'--exact','tcr01_public_experiment','--nocapture'])
reason=None
while True:
    pid,status,r=os.wait4(p.pid,os.WNOHANG)
    if pid: break
    if (time.perf_counter_ns()-start)>10_000_000_000: reason='timeout_10s'
    try:
        lines=pathlib.Path('/proc/'+str(p.pid)+'/status').read_text().splitlines()
        rss=next(int(x.split()[1]) for x in lines if x.startswith('VmRSS:'))
        if rss>256*1024: reason='rss_256MiB'
    except (FileNotFoundError,StopIteration): pass
    if reason:
        p.kill();pid,status,r=os.wait4(p.pid,0);break
    time.sleep(.002)
if r.ru_maxrss>256*1024 and reason is None: reason="rss_peak_256MiB"
code=os.waitstatus_to_exitcode(status);p.returncode=code
with open(sys.argv[2],'x') as f:json.dump(dict(exit=code,interruption=reason,wall_ms=(time.perf_counter_ns()-start)/1e6,cpu_user_ms=r.ru_utime*1000,cpu_system_ms=r.ru_stime*1000,rss_kib=r.ru_maxrss),f,indent=2)
sys.exit(1 if reason else code)
"#;
    let mut child = Command::new("python3")
        .arg("-c")
        .arg(supervisor)
        .arg(std::env::current_exe().unwrap())
        .arg(dir.join(format!("{mode}-resources.json")))
        .env("NEURONS_TCR01_WORKER", mode)
        .env("NEURONS_TCR01_DIR", dir)
        .stdout(Stdio::from(out))
        .stderr(Stdio::from(err))
        .spawn()
        .unwrap();
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            assert!(
                status.success(),
                "{}",
                fs::read_to_string(dir.join(format!("{mode}.log"))).unwrap()
            );
            break;
        }
        assert!(
            start.elapsed() < Duration::from_secs(12),
            "timeout supervisor TCR-01"
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}
#[test]
fn tcr01_public_experiment() {
    if let Ok(mode) = std::env::var("NEURONS_TCR01_WORKER") {
        let dir = std::path::PathBuf::from(std::env::var_os("NEURONS_TCR01_DIR").unwrap());
        assert!(dir.starts_with(std::env::temp_dir()));
        worker(&mode, &dir);
        return;
    }
    let dir = std::env::var_os("NEURONS_TCR01_OUTPUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::env::temp_dir().join(format!(
                "neurons-tcr01-run-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ))
        });
    fs::create_dir(&dir).unwrap();
    for i in 1..=2 {
        let d = dir.join(format!("repeat-{i}"));
        fs::create_dir(&d).unwrap();
        spawn_worker("run", &d);
        spawn_worker("restore", &d);
        assess(&d);
    }
    for group in [
        "T_pre", "C0_pre", "T", "C0", "C1", "C2_0", "C2_1", "C2_2", "C3",
    ] {
        assert_eq!(
            fs::read(dir.join("repeat-1").join(format!("{group}.signature"))).unwrap(),
            fs::read(dir.join("repeat-2").join(format!("{group}.signature"))).unwrap()
        );
    }
    println!("TCR-01 evidence: {}", dir.display());
}
#[test]
fn partitions_and_external_rule_are_frozen() {
    let all = P0
        .iter()
        .map(|(b, _)| *b)
        .chain(P1.iter().map(|(b, _)| *b))
        .chain(P2)
        .chain(AUX)
        .collect::<Vec<_>>();
    for (i, a) in all.iter().enumerate() {
        assert_eq!(a.len(), 6);
        assert!(!all[..i].contains(a));
    }
    for (bits, label) in P0.iter().chain(P1.iter()) {
        assert_eq!(external(bits), *label);
    }
    assert_eq!(
        P2.iter().map(|b| external(b)).collect::<Vec<_>>(),
        vec![
            Label::Positive,
            Label::Positive,
            Label::Positive,
            Label::Positive,
            Label::Negative,
            Label::Negative,
            Label::Negative,
            Label::Negative
        ]
    );
}
#[test]
fn external_reference_rejects_altered_report() {
    let mut s = base(true);
    teach(&mut s);
    let b = binding(0);
    let r = root(0);
    let p = permissions(&b);
    let mut e = Episode::new(&s, eid(3900), b);
    let mut report = e.query(&s, &r, &p).unwrap();
    assert!(reference_verify(&s, &r, &report, &p).is_ok());
    report.binding.item = node(9999);
    assert!(reference_verify(&s, &r, &report, &p).is_err());
}
