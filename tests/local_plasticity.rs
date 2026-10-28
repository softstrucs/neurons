#[path = "support/local_plasticity.rs"]
mod local;
#[path = "support/local_final_locked.rs"]
mod locked;
use local::{symbolic, Decision, Local, Observation};

fn pipeline(
    model: &Local,
    form: &str,
    input: [bool; 2],
    ablated: bool,
) -> (Option<usize>, Observation) {
    let Some(i) = locked::FORMS.iter().position(|f| *f == form) else {
        return (
            None,
            Observation {
                decision: Decision::Unknown,
                activity: [0; 4],
            },
        );
    };
    (Some(i + 1), model.query(input, ablated))
}
#[test]
fn development_local_pre_post_eligibility() {
    let mut m = Local::new();
    let a = m.learn([true, false], [true, false]);
    assert_eq!(a.activity, [1, 0, 1, 0]);
    assert_eq!(m.weights, [1, 0, 0, 0]);
    let b = m.learn([false, true], [false, true]);
    assert_eq!(b.activity, [0, 1, 0, 1]);
    assert_eq!(m.weights, [1, 0, 0, 1]);
}
#[test]
fn validation_local_training_and_symbolic() {
    let mut a = Local::new();
    let mut neutral = Local::new();
    a.learn([true, false], [true, false]);
    a.learn([false, true], [false, true]);
    neutral.learn([true, true], [true, false]);
    neutral.learn([true, true], [false, true]);
    for p in [[true, false], [false, true], [true, true], [false, false]] {
        assert_eq!(a.query(p, false), symbolic(a.weights, p, false));
    }
    assert_eq!(a.query([true, true], false).decision, Decision::Ambiguous);
    assert_eq!(
        neutral.query([true, true], false).decision,
        Decision::Ambiguous
    );
}
#[test]
fn frozen_local_protocol() {
    let mut m = Local::new();
    let mut again = Local::new();
    let mut neutral = Local::new();
    let idle = Local::new();
    for (i, form) in locked::FORMS.iter().enumerate() {
        let (id, a) = pipeline(&m, form, locked::INPUTS[i], false);
        assert_eq!(id, Some(i + 1));
        assert_eq!(a.decision, Decision::Silent);
        assert_eq!(a, symbolic(m.weights, locked::INPUTS[i], false));
    }
    m.learn([true, false], [true, false]);
    again.learn([true, false], [true, false]);
    neutral.learn([true, true], [true, false]);
    assert_eq!(m.weights, [1, 0, 0, 0]);
    assert_eq!(neutral.weights, [1, 0, 1, 0]);
    let mut first = 0;
    let mut silent = 0;
    let mut neutral_correct = 0;
    let mut neutral_errors = 0;
    for (i, form) in locked::FORMS.iter().enumerate() {
        let p = locked::INPUTS[i];
        let (id, a) = pipeline(&m, form, p, false);
        assert_eq!(id, Some(i + 1));
        assert_eq!(a, symbolic(m.weights, p, false));
        assert_eq!(a, again.query(p, false));
        let b = neutral.query([true, true], false);
        assert_eq!(b, symbolic(neutral.weights, [true, true], false));
        assert_eq!(idle.query(p, false).decision, Decision::Silent);
        let expected = if locked::FIRST[i] {
            Decision::First
        } else {
            Decision::Second
        };
        first += usize::from(a.decision == expected);
        silent += usize::from(a.decision == Decision::Silent);
        neutral_correct += usize::from(b.decision == expected);
        neutral_errors += usize::from(b.decision != expected);
    }
    assert_eq!(
        (first, silent, neutral_correct, neutral_errors),
        (12, 12, 12, 12)
    );
    m.learn([false, true], [false, true]);
    again.learn([false, true], [false, true]);
    neutral.learn([true, true], [false, true]);
    assert_eq!(m.weights, [1, 0, 0, 1]);
    assert_eq!(again.weights, m.weights);
    assert_eq!(neutral.weights, [2, 1, 2, 1]);
    let mut hits = 0;
    let mut neutral_amb = 0;
    let mut ablated = 0;
    for (i, form) in locked::FORMS.iter().enumerate() {
        let p = locked::INPUTS[i];
        let (id, a) = pipeline(&m, form, p, false);
        assert_eq!(id, Some(i + 1));
        assert_eq!(a, symbolic(m.weights, p, false));
        assert_eq!(a, again.query(p, false));
        let b = neutral.query([true, true], false);
        assert_eq!(b, symbolic(neutral.weights, [true, true], false));
        neutral_amb += usize::from(b.decision == Decision::Ambiguous);
        let off = m.query(p, true);
        assert_eq!(off, symbolic(m.weights, p, true));
        ablated += usize::from(off.decision == Decision::Silent);
        let expected = if locked::FIRST[i] {
            Decision::First
        } else {
            Decision::Second
        };
        hits += usize::from(a.decision == expected);
    }
    assert_eq!((hits, neutral_amb, ablated), (24, 24, 24));
    // Quatro rejeitados na entrada; oito chegam ao circuito: vazio e conflito dual.
    for i in 0..12 {
        if i < 4 {
            let (id, a) = pipeline(&m, "forma_inexistente", [true, false], false);
            assert_eq!(id, None);
            assert_eq!(a.decision, Decision::Unknown);
        } else {
            let p = if i < 8 { [false, false] } else { [true, true] };
            let (id, a) = pipeline(&m, locked::FORMS[i], p, false);
            assert!(id.is_some());
            assert_eq!(a, symbolic(m.weights, p, false));
            assert_eq!(
                a.decision,
                if i < 8 {
                    Decision::Silent
                } else {
                    Decision::Ambiguous
                }
            );
        }
    }
    println!("R1 pre_silent=24 first_correct=12 first_silent=12 neutral_first_correct=12 neutral_first_errors=12 final_correct=24 neutral_ambiguous=24 ablated_silent=24 negative_gate=4 negative_circuit_silent=4 negative_circuit_ambiguous=4 repeat=24");
}
