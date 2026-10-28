#[path = "support/concept_blind.rs"]
mod blind;
#[path = "support/a02_r1_validation_locked.rs"]
mod locked;
use blind::{fact, neurons, select, BlindDecision, Fact};
use neurons::languages::{ConceptOutcome, Direction, Language};

fn decisions(
    facts: &[Fact],
    direction: Direction,
    input: &str,
    budget: usize,
) -> (ConceptOutcome, BlindDecision) {
    let (source, target) = match direction {
        Direction::PtToEn => (Language::PtBr, Language::En),
        Direction::EnToPt => (Language::En, Language::PtBr),
    };
    let a = neurons(facts).compose(direction, input, budget).unwrap();
    let b = select(facts, source, input, target, budget);
    (a, b)
}

fn category(a: &ConceptOutcome, b: &BlindDecision) -> (&'static str, &'static str) {
    let a = match a {
        ConceptOutcome::Supported { .. } => "supported",
        ConceptOutcome::Unknown => "unknown",
        ConceptOutcome::Ambiguous => "ambiguous",
        ConceptOutcome::Conflict => "conflict",
        ConceptOutcome::LimitExceeded => "limit",
        ConceptOutcome::Inconclusive => "inconclusive",
    };
    let b = match b {
        BlindDecision::Supported { .. } => "supported",
        BlindDecision::Unknown => "unknown",
        BlindDecision::Ambiguous => "ambiguous",
        BlindDecision::Conflict => "conflict",
        BlindDecision::LimitExceeded => "limit",
        BlindDecision::Inconclusive => "inconclusive",
        BlindDecision::Invalid => "invalid",
    };
    (a, b)
}

#[test]
fn independent_validation_frozen_before_execution() {
    let mut facts = Vec::new();
    for (i, (pt, en)) in locked::PAIRS.iter().enumerate() {
        facts.push(fact(
            (2 * i + 1) as u32,
            Language::PtBr,
            pt,
            (i + 1) as u32,
            true,
        ));
        facts.push(fact(
            (2 * i + 2) as u32,
            Language::En,
            en,
            (i + 1) as u32,
            true,
        ));
    }
    let mut n = 0;
    for (i, (pt, en)) in locked::PAIRS.iter().enumerate() {
        for (d, input, answer, first, second) in [
            (Direction::PtToEn, *pt, *en, 2 * i + 1, 2 * i + 2),
            (Direction::EnToPt, *en, *pt, 2 * i + 2, 2 * i + 1),
        ] {
            let (a, b) = decisions(&facts, d, input, 2);
            // A resposta só é usada neste ponto, depois das duas decisões.
            assert!(
                matches!(&a, ConceptOutcome::Supported{text,concept,source_evidence,target_evidence,..} if text==answer && *concept==(i+1) as u32 && source_evidence.get()==first as u32 && target_evidence.get()==second as u32)
            );
            assert!(
                matches!(&b, BlindDecision::Supported{text,concept,source_evidence,target_evidence} if *text==answer && *concept==(i+1) as u32 && *source_evidence==first as u32 && *target_evidence==second as u32)
            );
            assert_eq!((a, b), decisions(&facts, d, input, 2));
            n += 1;
        }
    }
    for (pt, en) in locked::ABSENT {
        for (d, input) in [(Direction::PtToEn, pt), (Direction::EnToPt, en)] {
            let (a, b) = decisions(&facts, d, input, 2);
            assert_eq!(category(&a, &b), ("unknown", "unknown"));
            n += 1;
        }
    }
    for d in [Direction::PtToEn, Direction::EnToPt] {
        let (input, source, target, extra) = match d {
            Direction::PtToEn => ("janela", Language::PtBr, Language::En, "pane"),
            Direction::EnToPt => ("window", Language::En, Language::PtBr, "vidraça"),
        };
        let mut amb = facts.clone();
        amb.push(fact(9, target, extra, 1, true));
        let (a, b) = decisions(&amb, d, input, 2);
        assert_eq!(category(&a, &b), ("ambiguous", "ambiguous"));
        n += 1;
        let mut conflict = facts.clone();
        conflict.push(fact(9, source, input, 1, false));
        let (a, b) = decisions(&conflict, d, input, 2);
        assert_eq!(category(&a, &b), ("conflict", "conflict"));
        n += 1;
        let (a, b) = decisions(&facts, d, input, 1);
        assert_eq!(category(&a, &b), ("inconclusive", "inconclusive"));
        n += 1;
    }
    assert_eq!(n, 18);
    println!("validacao: 8 sustentadas, 4 ausencias, 2 ambiguidades, 2 conflitos, 2 inconclusoes; concordancia 18/18, evidencias 8/8");
}

fn adversarial(count: usize, conflict_first: bool, conflict: bool) -> Vec<Fact> {
    let mut entries = Vec::new();
    let mut push = |language, form, concept, positive| {
        let id = entries.len() as u32 + 1;
        entries.push(fact(id, language, form, concept, positive));
    };
    if conflict && conflict_first {
        push(Language::PtBr, "sinal", 2, true);
        push(Language::PtBr, "sinal", 2, false);
    }
    push(Language::PtBr, "sinal", 1, true);
    for text in [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .take(count)
    {
        push(Language::En, text, 1, true);
    }
    if conflict && !conflict_first {
        push(Language::PtBr, "sinal", 2, true);
        push(Language::PtBr, "sinal", 2, false);
    }
    entries
}

#[test]
fn adversarial_precedence_and_budgets() {
    for count in [1, 7, 8, 9] {
        for conflict in [false, true] {
            for first in [false, true] {
                for budget in [1, 2] {
                    let facts = adversarial(count, first, conflict);
                    let (a, b) = decisions(&facts, Direction::PtToEn, "sinal", budget);
                    let pair = category(&a, &b);
                    println!("count={count} conflict={conflict} first={first} budget={budget} production={} blind={}",pair.0,pair.1);
                    if budget == 1 {
                        assert_eq!(pair, ("inconclusive", "inconclusive"));
                    } else if !conflict {
                        let expected = if count == 1 {
                            "supported"
                        } else if count <= 8 {
                            "ambiguous"
                        } else {
                            "limit"
                        };
                        assert_eq!(pair, (expected, expected));
                    } else {
                        assert_eq!(pair.1, "conflict");
                        if first || count <= 8 {
                            assert_eq!(pair.0, "conflict");
                        } else {
                            assert_eq!(pair.0, "limit");
                        }
                    }
                }
            }
        }
    }
}
