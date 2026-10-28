#[path = "support/concept_blind.rs"]
mod blind;
use blind::{fact, neurons, select, BlindDecision};
use neurons::languages::{ConceptOutcome, Direction, Language};

#[test]
fn development_unique_absent_budget_and_evidence() {
    let facts = vec![
        fact(1, Language::PtBr, "fonte", 1, true),
        fact(2, Language::En, "target", 1, true),
    ];
    let model = neurons(&facts);
    let result = select(&facts, Language::PtBr, "fonte", Language::En, 2);
    assert_eq!(
        result,
        BlindDecision::Supported {
            text: "target",
            concept: 1,
            source_evidence: 1,
            target_evidence: 2
        }
    );
    assert!(
        matches!(model.compose(Direction::PtToEn,"fonte",2).unwrap(),ConceptOutcome::Supported{text,concept:1,..} if text=="target")
    );
    assert_eq!(
        select(&facts, Language::PtBr, "faltante", Language::En, 2),
        BlindDecision::Unknown
    );
    assert_eq!(
        select(&facts, Language::PtBr, "fonte", Language::En, 1),
        BlindDecision::Inconclusive
    );
    assert_eq!(
        select(&facts, Language::PtBr, "fonte", Language::PtBr, 2),
        BlindDecision::Invalid
    );
}
#[test]
fn development_multiple_candidates_conflict_and_limit() {
    let mut facts = vec![
        fact(1, Language::PtBr, "origem", 1, true),
        fact(2, Language::En, "a", 1, true),
        fact(3, Language::En, "b", 1, true),
    ];
    assert_eq!(
        select(&facts, Language::PtBr, "origem", Language::En, 2),
        BlindDecision::Ambiguous
    );
    assert_eq!(
        neurons(&facts)
            .compose(Direction::PtToEn, "origem", 2)
            .unwrap(),
        ConceptOutcome::Ambiguous
    );
    facts.push(fact(4, Language::PtBr, "origem", 1, false));
    assert_eq!(
        select(&facts, Language::PtBr, "origem", Language::En, 2),
        BlindDecision::Conflict
    );
    assert_eq!(
        neurons(&facts)
            .compose(Direction::PtToEn, "origem", 2)
            .unwrap(),
        ConceptOutcome::Conflict
    );
}
#[test]
fn development_control_is_repeatable_and_not_given_expected_output() {
    let facts = vec![
        fact(1, Language::PtBr, "pt", 3, true),
        fact(2, Language::En, "en", 3, true),
    ];
    let a = select(&facts, Language::PtBr, "pt", Language::En, 2);
    assert_eq!(a, select(&facts, Language::PtBr, "pt", Language::En, 2));
    assert_eq!(
        select(&facts, Language::PtBr, "pt", Language::En, 3),
        BlindDecision::Invalid
    );
}
#[test]
fn development_overlapping_conflict_and_candidate_limit_have_different_precedence() {
    let mut facts = vec![fact(1, Language::PtBr, "origem_limite", 1, true)];
    for (i, text) in ["a", "b", "c", "d", "e", "f", "g", "h", "i"]
        .into_iter()
        .enumerate()
    {
        facts.push(fact((i + 2) as u32, Language::En, text, 1, true));
    }
    facts.push(fact(11, Language::PtBr, "origem_limite", 2, true));
    facts.push(fact(12, Language::PtBr, "origem_limite", 2, false));
    assert_eq!(
        select(&facts, Language::PtBr, "origem_limite", Language::En, 2),
        BlindDecision::Conflict
    );
    assert_eq!(
        neurons(&facts)
            .compose(Direction::PtToEn, "origem_limite", 2)
            .unwrap(),
        ConceptOutcome::LimitExceeded
    );
}
