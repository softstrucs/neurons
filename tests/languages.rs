#[path = "support/lexical.rs"]
mod lexical;
use neurons::languages::{
    Direction, Identification, Language, Lexicon, LexiconError, Outcome, MAX_INPUT_BYTES,
};
#[test]
fn development_validation_sixteen_queries() {
    let l = lexical::lexicon();
    for (p, e) in &lexical::PAIRS[..8] {
        assert!(
            matches!(l.translate(Direction::PtToEn,p,1).unwrap(),Outcome::Supported{text,..} if text==*e)
        );
        assert!(
            matches!(l.translate(Direction::EnToPt,e,1).unwrap(),Outcome::Supported{text,..} if text==*p)
        );
    }
}
#[test]
fn identification_and_exact_accent_handling() {
    let l = lexical::lexicon();
    assert_eq!(
        l.identify("água"),
        Ok(Identification::Identified(Language::PtBr))
    );
    assert_eq!(
        l.identify("water"),
        Ok(Identification::Identified(Language::En))
    );
    assert_eq!(l.identify("hotel"), Ok(Identification::Ambiguous));
    assert_eq!(l.identify("agua"), Ok(Identification::Unknown));
    assert_eq!(l.identify("a\u{301}gua"), Ok(Identification::Unknown));
    assert_eq!(l.identify("ÁGUA"), Ok(Identification::Unknown));
}
#[test]
fn conflicting_associations_and_resource_limits_are_explicit() {
    let mut l = Lexicon::new();
    l.add_pair("banco", "bank").unwrap();
    l.add_pair("banco", "bench").unwrap();
    assert_eq!(
        l.translate(Direction::PtToEn, "banco", 1),
        Ok(Outcome::Ambiguous)
    );
    assert_eq!(
        l.translate(Direction::PtToEn, "banco", 0),
        Ok(Outcome::Inconclusive)
    );
    assert_eq!(
        l.translate(Direction::PtToEn, "banco", 3),
        Err(LexiconError::InvalidInput)
    );
    assert_eq!(
        l.add_pair("banco", "bank"),
        Err(LexiconError::DuplicateAssociation)
    );
    assert_eq!(l.association_count(), 4);
    for invalid in ["", "duas palavras", "a\nb"] {
        assert_eq!(l.identify(invalid), Err(LexiconError::InvalidInput));
    }
    assert_eq!(
        l.identify(&"x".repeat(MAX_INPUT_BYTES + 1)),
        Err(LexiconError::InvalidInput)
    );
}
#[test]
fn capacity_rejection_does_not_modify_lexicon() {
    let mut l = Lexicon::new();
    for i in 0..64 {
        l.add_pair(&format!("p{i}"), &format!("e{i}")).unwrap();
    }
    assert_eq!(l.form_count(), 128);
    assert_eq!(l.association_count(), 128);
    assert_eq!(l.add_pair("novo", "new"), Err(LexiconError::Capacity));
    assert_eq!(l.form_count(), 128);
    assert_eq!(l.association_count(), 128);
}
