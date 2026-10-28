#[path = "support/context_protocol.rs"]
mod protocol;
use neurons::languages::contextual::{
    Choice, Context, ContextError, ContextLexicon, ContextOutcome, FeedbackExperiment, TrialOutcome,
};
use neurons::languages::{Direction, Language};
use neurons::learning::Feedback;
use protocol::{build, from_pairs, simplify, symbolic, Fact};

// 12 pares x 2 direções = 24 experiências de desenvolvimento.
const DEV: [(&str, &str, Context); 12] = [
    ("dinheiro", "money", Context::Finance),
    ("cartão", "card", Context::Finance),
    ("preço", "price", Context::Finance),
    ("moeda", "coin", Context::Finance),
    ("poltrona", "armchair", Context::Furniture),
    ("gaveta", "drawer", Context::Furniture),
    ("cama", "bed", Context::Furniture),
    ("lâmpada", "lamp", Context::Furniture),
    ("grama", "grass", Context::Nature),
    ("nuvem", "cloud", Context::Nature),
    ("teclado", "keyboard", Context::Technology),
    ("monitor", "monitor", Context::Technology),
];
#[test]
fn development_validation_16_queries() {
    let facts = from_pairs(&DEV);
    let lex = build(&facts);
    for (pt, en, cx) in &DEV[..8] {
        for (d, input, expected) in [(Direction::PtToEn, *pt, *en), (Direction::EnToPt, *en, *pt)] {
            let out = lex.translate(d, input, Some(*cx), 2).unwrap();
            assert!(matches!(&out,ContextOutcome::Supported{text,..} if text==expected));
            assert_eq!(simplify(out), symbolic(&facts, d, input, Some(*cx)));
        }
    }
}
#[test]
fn explicit_context_conflict_and_limits() {
    let facts = [
        Fact {
            language: Language::PtBr,
            text: "banco",
            context: Context::Finance,
            concept: 1,
            positive: true,
        },
        Fact {
            language: Language::En,
            text: "bank",
            context: Context::Finance,
            concept: 1,
            positive: true,
        },
        Fact {
            language: Language::PtBr,
            text: "banco",
            context: Context::Furniture,
            concept: 2,
            positive: true,
        },
        Fact {
            language: Language::En,
            text: "bench",
            context: Context::Furniture,
            concept: 2,
            positive: true,
        },
    ];
    let lex = build(&facts);
    assert!(
        matches!(lex.translate(Direction::PtToEn,"banco",Some(Context::Finance),2),Ok(ContextOutcome::Supported{text,..}) if text=="bank")
    );
    assert!(
        matches!(lex.translate(Direction::PtToEn,"banco",Some(Context::Furniture),2),Ok(ContextOutcome::Supported{text,..}) if text=="bench")
    );
    assert_eq!(
        lex.translate(Direction::PtToEn, "banco", None, 2),
        Ok(ContextOutcome::Ambiguous)
    );
    assert_eq!(
        lex.translate(Direction::PtToEn, "banco", None, 1),
        Ok(ContextOutcome::Inconclusive)
    );
    assert_eq!(
        lex.translate(Direction::PtToEn, "banco", None, 3),
        Err(ContextError::InvalidInput)
    );
    assert_eq!(
        lex.translate(Direction::PtToEn, &"x".repeat(257), None, 2),
        Err(ContextError::InvalidInput)
    );
    assert_eq!(
        lex.translate(Direction::PtToEn, "a b c d e f g h i j k l m", None, 2),
        Err(ContextError::InvalidInput)
    );
    let mut crowded = ContextLexicon::new();
    crowded.add_concept(1, "sentido").unwrap();
    crowded
        .associate(Language::PtBr, "fonte", Context::Finance, 1, true)
        .unwrap();
    for i in 0..9 {
        crowded
            .associate(
                Language::En,
                &format!("destino{i}"),
                Context::Finance,
                1,
                true,
            )
            .unwrap();
    }
    assert_eq!(
        crowded.translate(Direction::PtToEn, "fonte", Some(Context::Finance), 2),
        Ok(ContextOutcome::LimitExceeded)
    );
}
fn symbolic_weights(weights: [i32; 2]) -> TrialOutcome {
    match (weights[0] > 0, weights[1] > 0) {
        (true, false) => TrialOutcome::First,
        (false, true) => TrialOutcome::Second,
        (true, true) => TrialOutcome::Ambiguous,
        (false, false) => TrialOutcome::NoActivation,
    }
}
#[test]
fn feedback_retention_interference_and_ablation() {
    let mut learner = FeedbackExperiment::new();
    let mut control = FeedbackExperiment::new();
    for id in [1, 2] {
        learner.add_trial(id).unwrap();
        control.add_trial(id).unwrap();
    }
    for id in [1, 2] {
        assert_eq!(learner.weights(id).unwrap(), control.weights(id).unwrap());
        assert_eq!(
            learner.evaluate(id, 1, false).unwrap(),
            TrialOutcome::NoActivation
        );
    }
    learner
        .feedback(1, Choice::First, Some(Feedback::Positive))
        .unwrap();
    control.feedback(1, Choice::First, None).unwrap();
    assert_eq!(learner.evaluate(1, 1, false).unwrap(), TrialOutcome::First);
    assert_eq!(
        control.evaluate(1, 1, false).unwrap(),
        TrialOutcome::NoActivation
    );
    learner
        .feedback(2, Choice::Second, Some(Feedback::Positive))
        .unwrap();
    control.feedback(2, Choice::Second, None).unwrap();
    assert_eq!(learner.evaluate(1, 2, false).unwrap(), TrialOutcome::First);
    assert_eq!(learner.evaluate(2, 2, false).unwrap(), TrialOutcome::Second);
    for id in [1, 2] {
        assert_eq!(
            learner.evaluate(id, 2, false).unwrap(),
            symbolic_weights(learner.weights(id).unwrap())
        );
        assert_eq!(
            learner.evaluate(id, 2, true).unwrap(),
            TrialOutcome::NoActivation
        );
        assert_eq!(
            control.evaluate(id, 2, false).unwrap(),
            TrialOutcome::NoActivation
        );
    }
    learner
        .feedback(1, Choice::Second, Some(Feedback::Positive))
        .unwrap();
    assert_eq!(
        learner.evaluate(1, 1, false).unwrap(),
        TrialOutcome::Ambiguous
    );
    assert_eq!(learner.evaluate(2, 1, false).unwrap(), TrialOutcome::Second);
    learner
        .feedback(1, Choice::Second, Some(Feedback::Negative))
        .unwrap();
    assert_eq!(learner.evaluate(1, 1, false).unwrap(), TrialOutcome::First);
    learner
        .feedback(1, Choice::First, Some(Feedback::Negative))
        .unwrap();
    assert_eq!(
        learner.evaluate(1, 1, false).unwrap(),
        TrialOutcome::NoActivation
    );
    assert_eq!(
        learner.evaluate(3, 1, false),
        Err(ContextError::UnknownTrial)
    );
    assert_eq!(
        learner.evaluate(2, 0, false),
        Err(ContextError::InvalidInput)
    );
    println!("feedback pre=0/2 post=2/2 control=0/2 retained=1/1 unseen_payload=2/2 ablated=0/2 symbolic=2/2 interference=ambiguous local_isolation=1/1");
}
