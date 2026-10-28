#[path = "support/concept_blind.rs"]
mod blind;
#[allow(dead_code)]
#[path = "support/a02_final_locked.rs"]
mod locked;
use blind::{fact, neurons, select, BlindDecision, Fact};
use neurons::languages::{ConceptOutcome, Direction, Language};

#[derive(Clone, Debug, Eq, PartialEq)]
enum View {
    Supported {
        text: String,
        concept: u32,
        source_evidence: u32,
        target_evidence: u32,
    },
    Unknown,
    Ambiguous,
    Conflict,
    LimitExceeded,
    Inconclusive,
}
fn neuron_view(out: ConceptOutcome) -> View {
    match out {
        ConceptOutcome::Supported {
            text,
            concept,
            source_evidence,
            target_evidence,
            ..
        } => View::Supported {
            text,
            concept,
            source_evidence: source_evidence.get(),
            target_evidence: target_evidence.get(),
        },
        ConceptOutcome::Unknown => View::Unknown,
        ConceptOutcome::Ambiguous => View::Ambiguous,
        ConceptOutcome::Conflict => View::Conflict,
        ConceptOutcome::LimitExceeded => View::LimitExceeded,
        ConceptOutcome::Inconclusive => View::Inconclusive,
    }
}
fn control_view(out: BlindDecision) -> View {
    match out {
        BlindDecision::Supported {
            text,
            concept,
            source_evidence,
            target_evidence,
        } => View::Supported {
            text: text.to_owned(),
            concept,
            source_evidence,
            target_evidence,
        },
        BlindDecision::Unknown => View::Unknown,
        BlindDecision::Ambiguous => View::Ambiguous,
        BlindDecision::Conflict => View::Conflict,
        BlindDecision::LimitExceeded => View::LimitExceeded,
        BlindDecision::Inconclusive => View::Inconclusive,
        BlindDecision::Invalid => panic!("consulta inválida no protocolo"),
    }
}
fn decide(facts: &[Fact], direction: Direction, input: &str) -> (View, View) {
    let model = neurons(facts);
    let (source, target) = match direction {
        Direction::PtToEn => (Language::PtBr, Language::En),
        Direction::EnToPt => (Language::En, Language::PtBr),
    };
    // Nenhum destino esperado entra em qualquer das decisões.
    let candidate = neuron_view(model.compose(direction, input, 2).unwrap());
    let control = control_view(select(facts, source, input, target, 2));
    assert_eq!(
        candidate,
        neuron_view(model.compose(direction, input, 2).unwrap())
    );
    assert_eq!(
        control,
        control_view(select(facts, source, input, target, 2))
    );
    (candidate, control)
}
fn base() -> Vec<Fact> {
    let mut facts = Vec::new();
    for (index, (pt, en)) in locked::PAIRS.iter().copied().enumerate() {
        facts.push(fact(
            (2 * index + 1) as u32,
            Language::PtBr,
            pt,
            (index + 1) as u32,
            true,
        ));
        facts.push(fact(
            (2 * index + 2) as u32,
            Language::En,
            en,
            (index + 1) as u32,
            true,
        ));
    }
    facts
}
#[derive(Default)]
struct Metrics {
    pt: usize,
    en: usize,
    negative: usize,
    errors: usize,
    false_abstentions: usize,
    inappropriate: usize,
    inconclusive: usize,
}
fn grade(stats: &mut Metrics, out: &View, expected: &View, direction: Option<Direction>) {
    if out == expected {
        match direction {
            Some(Direction::PtToEn) => stats.pt += 1,
            Some(Direction::EnToPt) => stats.en += 1,
            None => stats.negative += 1,
        };
        return;
    }
    if matches!(out, View::Inconclusive | View::LimitExceeded) {
        stats.inconclusive += 1;
    } else if direction.is_some() && !matches!(out, View::Supported { .. }) {
        stats.false_abstentions += 1;
    } else if direction.is_none() && matches!(out, View::Supported { .. }) {
        stats.inappropriate += 1;
    } else {
        stats.errors += 1;
    }
}
#[test]
fn frozen_paired_protocol_blind_control() {
    let facts = base();
    let mut model = Metrics::default();
    let mut control = Metrics::default();
    let mut paired = 0;
    let mut evidence = 0;
    for (index, (pt, en)) in locked::PAIRS.iter().copied().enumerate() {
        for (d, input, target, first, second) in [
            (
                Direction::PtToEn,
                pt,
                en,
                (2 * index + 1) as u32,
                (2 * index + 2) as u32,
            ),
            (
                Direction::EnToPt,
                en,
                pt,
                (2 * index + 2) as u32,
                (2 * index + 1) as u32,
            ),
        ] {
            let (a, b) = decide(&facts, d, input);
            // Oráculo só é usado depois de as duas decisões estarem completas.
            let expected = View::Supported {
                text: target.to_owned(),
                concept: (index + 1) as u32,
                source_evidence: first,
                target_evidence: second,
            };
            grade(&mut model, &a, &expected, Some(d));
            grade(&mut control, &b, &expected, Some(d));
            paired += usize::from(a == b);
            evidence += usize::from(a == b && matches!(a, View::Supported { .. }));
        }
    }
    for (d, items) in [
        (Direction::PtToEn, locked::ABSENT_PT),
        (Direction::EnToPt, locked::ABSENT_EN),
    ] {
        for input in items {
            let (a, b) = decide(&facts, d, input);
            let expected = View::Unknown;
            grade(&mut model, &a, &expected, None);
            grade(&mut control, &b, &expected, None);
            paired += usize::from(a == b);
        }
    }
    // Quatro grafos com candidatos múltiplos e quatro com evidência contraditória.
    for index in 0..4 {
        let concept = (index + 1) as u32;
        let (direction, input, target_language, extra) = if index < 2 {
            (
                Direction::PtToEn,
                locked::PAIRS[index].0,
                Language::En,
                locked::EXTRA_EN[index],
            )
        } else {
            (
                Direction::EnToPt,
                locked::PAIRS[index].1,
                Language::PtBr,
                locked::EXTRA_PT[index - 2],
            )
        };
        let mut ambiguous = facts.clone();
        ambiguous.push(fact(33, target_language, extra, concept, true));
        let (a, b) = decide(&ambiguous, direction, input);
        let expected = View::Ambiguous;
        grade(&mut model, &a, &expected, None);
        grade(&mut control, &b, &expected, None);
        paired += usize::from(a == b);
        let source_language = if direction == Direction::PtToEn {
            Language::PtBr
        } else {
            Language::En
        };
        let mut conflicting = facts.clone();
        conflicting.push(fact(33, source_language, input, concept, false));
        let (a, b) = decide(&conflicting, direction, input);
        let expected = View::Conflict;
        grade(&mut model, &a, &expected, None);
        grade(&mut control, &b, &expected, None);
        paired += usize::from(a == b);
    }
    assert_eq!(
        (
            model.pt,
            model.en,
            model.negative,
            model.errors,
            model.false_abstentions,
            model.inappropriate,
            model.inconclusive
        ),
        (16, 16, 16, 0, 0, 0, 0)
    );
    assert_eq!(
        (
            control.pt,
            control.en,
            control.negative,
            control.errors,
            control.false_abstentions,
            control.inappropriate,
            control.inconclusive
        ),
        (16, 16, 16, 0, 0, 0, 0)
    );
    assert_eq!((paired, evidence), (48, 32));
    println!("a02_final neuron_pt=16/16 neuron_en=16/16 blind_pt=16/16 blind_en=16/16 negatives=16/16 blind_negatives=16/16 paired=48/48 evidence=32/32 errors=0 false_abstentions=0 inappropriate=0 inconclusive=0");
}
