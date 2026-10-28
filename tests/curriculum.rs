use neurons::cognition::{Hypothesis, Outcome, TwoHopTask};
use neurons::curriculum::{CurriculumError, Domain, Microcurriculum, Polarity};
use neurons::engram::{Cue, EngramId, RecordStatus};
use neurons::event::NodeId;
use neurons::metacognition::{AbstainReason, Decision, EvidenceRefs};

fn node(id: u32) -> NodeId {
    NodeId::new(id).unwrap()
}
fn record_id(id: u32) -> EngramId {
    EngramId::new(id).unwrap()
}
fn add(c: &mut Microcurriculum, id: u32, d: Domain, a: u32, b: u32) {
    c.observe(record_id(id), d, node(a), node(b), Polarity::Positive)
        .unwrap();
}
fn supported(decision: Decision) -> bool {
    matches!(decision, Decision::Respond { .. })
}
const DOMAINS: [Domain; 4] = [
    Domain::Communication,
    Domain::Mathematics,
    Domain::NaturalScience,
    Domain::Computing,
];

// Oráculos ficam no protocolo, nunca no adaptador. A relação só permite provas
// diretas ou de dois passos, não fechamento transitivo irrestrito.
fn protocol(domain: Domain, symbols: [u32; 4]) {
    let [a, b, c, d] = symbols;
    let mut curriculum = Microcurriculum::new(4).unwrap();
    add(&mut curriculum, 1, domain, a, b);
    add(&mut curriculum, 2, domain, b, c);
    add(&mut curriculum, 3, domain, c, d);
    let task = TwoHopTask::new(32, 64, 128).unwrap();
    let mut monitored = 0;
    let mut recall_only = 0;
    let mut no_synapses = 0;
    let mut no_memory = 0;
    let empty = Microcurriculum::new(1).unwrap();
    // Direto; composição reservada; reverso sem prova; três passos fora do escopo.
    for (from, to, expected) in [(a, b, true), (a, c, true), (c, a, false), (a, d, false)] {
        let report = curriculum.evaluate(domain, node(from), node(to)).unwrap();
        assert_eq!(
            report,
            curriculum.evaluate(domain, node(from), node(to)).unwrap()
        );
        monitored += usize::from(supported(report.decision) == expected);
        let cue = Cue::new(domain.context(), node(from), to as i32).unwrap();
        recall_only += usize::from(curriculum.memory().recall(cue).is_ok() == expected);
        let query = Hypothesis::new(domain.context(), node(from), node(to)).unwrap();
        let control = task
            .evaluate_without_synapses(curriculum.memory(), query)
            .unwrap();
        no_synapses += usize::from(
            matches!(
                control.outcome,
                Outcome::DirectRecall(_) | Outcome::Inferred(_)
            ) == expected,
        );
        no_memory += usize::from(
            supported(
                empty
                    .evaluate(domain, node(from), node(to))
                    .unwrap()
                    .decision,
            ) == expected,
        );
    }
    assert_eq!(
        (monitored, recall_only, no_synapses, no_memory),
        (4, 3, 3, 2)
    );
    assert_eq!(curriculum.memory().len(), 3); // consultas não armazenam conclusões.
    assert!(curriculum
        .memory()
        .iter()
        .all(|r| r.status() == RecordStatus::Observed));
    let composed = curriculum.evaluate(domain, node(a), node(c)).unwrap();
    assert!(
        matches!(composed.decision, Decision::Respond { evidence: EvidenceRefs::TwoHop { first, second }, .. } if first == record_id(1) && second == record_id(2))
    );
    println!(
        "{domain:?}: monitor=4/4 recall=3/4 no_synapses=3/4 no_memory=2/4 reserved_composition=1/1"
    );
}

#[test]
fn communication_protocol() {
    // Inclusão de categorias simbólicas: pardal ⊂ ave ⊂ animal ⊂ organismo.
    protocol(Domain::Communication, [11, 12, 13, 14]);
}
#[test]
fn mathematics_protocol() {
    // Ordem dos números 2 < 5 < 9 < 12; oráculo numérico externo explícito.
    assert!(2 < 5 && 5 < 9 && 9 < 12);
    protocol(Domain::Mathematics, [2, 5, 9, 12]);
}
#[test]
fn natural_science_protocol() {
    // Instantes observados de um mundo sintético: frio → morno → quente → muito quente.
    // Relação: precedência observada, sem inferência causal ou lei física.
    protocol(Domain::NaturalScience, [21, 22, 23, 24]);
}
#[test]
fn computing_protocol() {
    // Estados de contador sintético: 0 → 1 → 2 → 3, IDs não nulos 31..34.
    protocol(Domain::Computing, [31, 32, 33, 34]);
}

#[test]
fn retention_context_interference_and_conflicting_evidence() {
    let mut c = Microcurriculum::new(16).unwrap();
    add(&mut c, 1, DOMAINS[0], 1, 2);
    add(&mut c, 2, DOMAINS[0], 2, 3);
    let baseline = c.evaluate(DOMAINS[0], node(1), node(3)).unwrap();
    for (index, domain) in DOMAINS.into_iter().enumerate().skip(1) {
        add(&mut c, (index * 2 + 1) as u32, domain, 1, 2);
        add(&mut c, (index * 2 + 2) as u32, domain, 2, 3);
        assert_eq!(baseline, c.evaluate(DOMAINS[0], node(1), node(3)).unwrap());
    }
    c.observe(
        record_id(9),
        DOMAINS[3],
        node(1),
        node(3),
        Polarity::Negative,
    )
    .unwrap();
    assert_eq!(baseline, c.evaluate(DOMAINS[0], node(1), node(3)).unwrap());
    assert_eq!(
        c.evaluate(DOMAINS[3], node(1), node(3)).unwrap().decision,
        Decision::Abstain(AbstainReason::ContradictoryEvidence)
    );
    c.observe(
        record_id(10),
        DOMAINS[0],
        node(2),
        node(3),
        Polarity::Negative,
    )
    .unwrap();
    assert_eq!(
        c.evaluate(DOMAINS[0], node(1), node(3)).unwrap().decision,
        Decision::Abstain(AbstainReason::ContradictoryEvidence)
    );
    assert!(supported(
        c.evaluate(DOMAINS[1], node(1), node(3)).unwrap().decision
    ));
    println!("retention=3/3 cross_context_isolation=1/1 same_context_conflict_detection=1/1");
}

#[test]
fn transfer_probe_requires_new_domain_observations_not_training_credit() {
    let mut c = Microcurriculum::new(8).unwrap();
    add(&mut c, 1, DOMAINS[0], 1, 2);
    add(&mut c, 2, DOMAINS[0], 2, 3);
    for (index, domain) in DOMAINS.into_iter().enumerate().skip(1) {
        let a = 10 * index as u32;
        let b = a + 1;
        let end = a + 2;
        assert!(!supported(
            c.evaluate(domain, node(a), node(end)).unwrap().decision
        ));
        add(&mut c, (index * 2 + 1) as u32, domain, a, b);
        add(&mut c, (index * 2 + 2) as u32, domain, b, end);
        assert!(supported(
            c.evaluate(domain, node(a), node(end)).unwrap().decision
        ));
    }
    println!("transfer: without_target_facts=0/3 supported; with_target_facts=3/3; learned_transfer=not_demonstrated");
}

#[test]
fn invalid_inputs_and_capacity_preserve_memory() {
    assert!(matches!(
        Microcurriculum::new(0),
        Err(CurriculumError::InvalidCapacity)
    ));
    assert!(matches!(
        Microcurriculum::new(33),
        Err(CurriculumError::InvalidCapacity)
    ));
    let mut c = Microcurriculum::new(1).unwrap();
    assert_eq!(
        c.observe(
            record_id(1),
            DOMAINS[0],
            node(1),
            node(u32::MAX),
            Polarity::Positive
        ),
        Err(CurriculumError::InvalidTarget)
    );
    assert_eq!(
        c.observe(
            record_id(1),
            DOMAINS[0],
            node(1),
            node(1),
            Polarity::Positive
        ),
        Err(CurriculumError::ReflexiveObservation)
    );
    assert!(c.memory().is_empty());
    add(&mut c, 1, DOMAINS[0], 1, 2);
    assert!(c
        .observe(
            record_id(2),
            DOMAINS[1],
            node(2),
            node(3),
            Polarity::Positive
        )
        .is_err());
    assert_eq!(c.memory().len(), 1);
    assert!(c.evaluate(DOMAINS[0], node(1), node(1)).is_err());
}
