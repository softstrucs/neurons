#[allow(dead_code)]
#[path = "support/sentence_common.rs"]
mod common;
use common::{build, pair, simple, symbolic, ADJS, NOUNS};
use neurons::languages::sentences::{Gender, SentenceError, SentenceOutcome};
use neurons::languages::Direction;
#[test]
fn development_24_examples_and_16_validation_queries() {
    let lex = build(&NOUNS, &ADJS, true);
    let mut training = Vec::new();
    for n in &NOUNS[..4] {
        for a in ADJS {
            let (pt, en) = pair(*n, a);
            training.push((pt, en));
        }
    }
    assert_eq!(training.len() * 2, 24);
    for (pt, en) in &training[..8] {
        for (d, source, target) in [(Direction::PtToEn, pt, en), (Direction::EnToPt, en, pt)] {
            let out = lex.translate(d, source, 1).unwrap();
            assert!(matches!(&out,SentenceOutcome::Supported{text,..} if text==target));
            assert_eq!(simple(out), symbolic(&NOUNS, &ADJS, d, source, true, None));
        }
    }
}
#[test]
fn grammar_errors_budgets_and_evidence_conflict() {
    let mut x = build(&NOUNS, &ADJS, true);
    assert_eq!(
        x.translate(Direction::PtToEn, "gato branca", 1),
        Ok(SentenceOutcome::OutOfGrammar)
    );
    assert_eq!(
        x.translate(Direction::PtToEn, "branco gato", 1),
        Ok(SentenceOutcome::OutOfGrammar)
    );
    assert_eq!(
        x.translate(Direction::PtToEn, "desconhecido branco", 1),
        Ok(SentenceOutcome::Unknown)
    );
    assert_eq!(
        x.translate(Direction::PtToEn, "gato branco", 0),
        Ok(SentenceOutcome::Inconclusive)
    );
    assert_eq!(
        x.translate(Direction::PtToEn, &"x".repeat(257), 1),
        Ok(SentenceOutcome::LimitExceeded)
    );
    assert_eq!(
        x.translate(Direction::PtToEn, "a b c d e f g h i j k l m", 1),
        Ok(SentenceOutcome::LimitExceeded)
    );
    assert_eq!(
        x.translate(Direction::PtToEn, "gato branco", 3),
        Err(SentenceError::InvalidInput)
    );
    let id = x.add_noun("gato", "volume", Gender::Masculine).unwrap();
    assert_eq!(
        x.translate(Direction::PtToEn, "gato branco", 1),
        Ok(SentenceOutcome::Ambiguous)
    );
    x.contradict(id).unwrap();
    let y = build(&NOUNS, &ADJS, false);
    assert_eq!(
        y.translate(Direction::PtToEn, "gato branco", 1),
        Ok(SentenceOutcome::Inconclusive)
    );
}
