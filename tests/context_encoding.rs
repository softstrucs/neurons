#[path = "support/shared_feedback.rs"]
mod baseline;
#[path = "support/context_circuit.rs"]
mod cc;
#[path = "support/context_circuit_final.rs"]
mod locked;
use baseline::{Context, Decision};
use cc::{symbolic, Model, Outcome};
use neurons::languages::contextual::Choice;

#[test]
fn development_context_patterns_and_interference() {
    let mut m = Model::new();
    assert_eq!(m.run([true, false], false).0, Outcome::Silent);
    m.train([true, false], 0);
    assert_eq!(m.run([true, false], false).0, Outcome::First);
    assert_eq!(m.run([false, true], false).0, Outcome::Silent);
    m.train([false, true], 1);
    assert_eq!(m.weights(), [1, 0, 0, 1]);
    assert_eq!(m.run([true, false], false).0, Outcome::First);
    assert_eq!(m.run([false, true], false).0, Outcome::Second);
    assert_eq!(m.run([true, true], false).0, Outcome::Ambiguous);
}

#[test]
fn validation_distinct_cases_before_final() {
    let mut m = Model::new();
    m.train([true, false], 0);
    m.train([false, true], 1);
    for p in [[true, false], [false, true], [true, true], [false, false]] {
        assert_eq!(m.run(p, false), symbolic(m.weights(), p, false));
    }
    assert_eq!(m.run([true, true], false).0, Outcome::Ambiguous);
}

#[test]
fn frozen_context_protocol() {
    let forms = locked::FORMS;
    let store = baseline::memory(&forms);
    let mut neural = Model::new();
    let mut repeat = Model::new();
    let idle = Model::new();
    let mut prior = baseline::fresh();
    let mut pre = 0;
    for (i, text) in forms.iter().enumerate() {
        let p = locked::CONTEXTS[i];
        assert_eq!(neural.run(p, false), (Outcome::Silent, [0, 0]));
        assert_eq!(idle.run(p, false), (Outcome::Silent, [0, 0]));
        let ctx = if p[0] {
            Context::Finance
        } else {
            Context::Furniture
        };
        assert_eq!(
            baseline::evaluate(&forms, &store, &prior, text, ctx, 1, false, false),
            Decision::NoActivation
        );
        pre += 1;
    }
    assert_eq!(pre, 24);
    neural.train([true, false], 0);
    repeat.train([true, false], 0);
    baseline::train(&mut prior, 1, Choice::First);
    assert_eq!(neural.weights(), [1, 0, 0, 0]);
    let mut first_correct = 0;
    let mut first_silent = 0;
    let mut prior_correct = 0;
    let mut prior_wrong = 0;
    for (i, text) in forms.iter().enumerate() {
        let p = locked::CONTEXTS[i];
        let a = neural.run(p, false);
        assert_eq!(a, symbolic(neural.weights(), p, false));
        assert_eq!(a, repeat.run(p, false));
        assert_eq!(idle.run(p, false).0, Outcome::Silent);
        let ctx = if p[0] {
            Context::Finance
        } else {
            Context::Furniture
        };
        let old = baseline::evaluate(&forms, &store, &prior, text, ctx, 1, false, false);
        let expected = if locked::EXPECT_FIRST[i] {
            Outcome::First
        } else {
            Outcome::Second
        };
        first_correct += usize::from(a.0 == expected);
        first_silent += usize::from(a.0 == Outcome::Silent);
        prior_correct +=
            usize::from(matches!(old, Decision::First(_)) && expected == Outcome::First);
        prior_wrong +=
            usize::from(matches!(old, Decision::First(_)) && expected == Outcome::Second);
    }
    assert_eq!(
        (first_correct, first_silent, prior_correct, prior_wrong),
        (12, 12, 12, 12)
    );
    neural.train([false, true], 1);
    repeat.train([false, true], 1);
    baseline::train(&mut prior, 1, Choice::Second);
    assert_eq!(neural.weights(), [1, 0, 0, 1]);
    assert_eq!(repeat.weights(), neural.weights());
    let mut correct = 0;
    let mut abstain = 0;
    let mut neutral = 0;
    let mut ablated = 0;
    let mut prior_amb = 0;
    for (i, text) in forms.iter().enumerate() {
        let p = locked::CONTEXTS[i];
        let a = neural.run(p, false);
        assert_eq!(a, symbolic(neural.weights(), p, false));
        assert_eq!(a, repeat.run(p, false));
        assert_eq!(
            neural.run([true, true], false),
            symbolic(neural.weights(), [true, true], false)
        );
        neutral += usize::from(neural.run([true, true], false).0 == Outcome::Ambiguous);
        assert_eq!(neural.run(p, true), symbolic(neural.weights(), p, true));
        ablated += usize::from(neural.run(p, true).0 == Outcome::Silent);
        let ctx = if p[0] {
            Context::Finance
        } else {
            Context::Furniture
        };
        prior_amb += usize::from(
            baseline::evaluate(&forms, &store, &prior, text, ctx, 1, false, false)
                == Decision::Ambiguous,
        );
        let expected = if locked::EXPECT_FIRST[i] {
            Outcome::First
        } else {
            Outcome::Second
        };
        correct += usize::from(a.0 == expected);
        abstain += usize::from(matches!(
            a.0,
            Outcome::Ambiguous | Outcome::Silent | Outcome::Inconclusive
        ));
    }
    assert_eq!(
        (correct, abstain, neutral, ablated, prior_amb),
        (24, 0, 24, 24, 24)
    );
    // Negativos: a triagem de presença e conflito é externa e comum aos caminhos.
    let mut negatives = 0;
    for i in 0..12 {
        let (text, ctx) = match i % 3 {
            0 => ("ausente", Context::Finance),
            1 => (forms[i], Context::Missing),
            _ => (forms[i], Context::Conflict),
        };
        let a = baseline::evaluate(&forms, &store, &prior, text, ctx, 1, false, false);
        let b = baseline::evaluate(&forms, &store, &prior, text, ctx, 1, true, false);
        assert_eq!(a, b);
        assert!(matches!(a, Decision::Unknown | Decision::Conflict));
        negatives += 1;
    }
    assert_eq!(negatives, 12);
    println!("A02.3 pre=0/24 silent=24 first=12/24 silent=12 prior=12/24 prior_errors=12 second=24/24 prior_ambiguous=24 neutral_ambiguous=24 ablated_silent=24 negative=12 repeat=24");
}
