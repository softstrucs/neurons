#[path = "support/activity_r1.rs"]
mod experiment;
#[path = "support/activity_r1_locked.rs"]
mod locked;
use experiment::{
    prior::neural::{Decision, Local},
    symbolic_trial, trial,
};

fn validate_phase(m: &Local, phase: usize) {
    let mut silent = 0;
    for (i, form) in locked::FORMS.iter().enumerate() {
        assert!(!form.is_empty());
        let o = m.query(locked::input(i), false);
        silent += usize::from(o.decision == Decision::Silent);
        let _oracle = locked::expected(i);
    }
    assert_eq!(silent, 24);
    println!(
        "phase={phase} correct=0 errors=0 silent=24 inconclusive=0 weights={:?}",
        m.weights
    );
}
#[test]
fn development_trace_order_and_zero_reward() {
    let mut m = Local::new();
    let t = trial(&mut m, [true, false], 0, 0, false);
    assert!(t.explored);
    assert_eq!(t.reward, 0);
    assert_eq!(t.eligibility, [false; 4]);
    assert_eq!(t.weights, [0; 4]);
}
#[test]
fn validation_independent_patterns_before_final() {
    for p in [[false, false], [true, true], [false, true], [true, false]] {
        let mut m = Local::new();
        let mut w = [0; 4];
        assert_eq!(
            trial(&mut m, p, 0, 1, false),
            symbolic_trial(&mut w, p, 0, 1, false)
        );
    }
}
#[test]
fn frozen_controls_repeated_null_protocol() {
    for (name, mode, neutral, ablated) in [
        ("activity", 1, false, false),
        ("no_update", 0, false, false),
        ("fixed_reward", 2, false, false),
        ("neutral", 1, true, false),
        ("ablated", 1, false, true),
    ] {
        let mut m = Local::new();
        let mut rep = Local::new();
        let mut w = [0; 4];
        let mut log = Vec::new();
        validate_phase(&m, 0);
        for tick in 0..4 {
            let p = if neutral {
                [true, true]
            } else {
                locked::input(if tick < 2 { 0 } else { 12 })
            };
            let t = trial(&mut m, p, tick, mode, ablated);
            assert_eq!(t, symbolic_trial(&mut w, p, tick, mode, ablated));
            assert_eq!(t, trial(&mut rep, p, tick, mode, ablated));
            assert!(t.explored);
            assert_eq!(t.eligibility, [false; 4]);
            println!("{name} tick={tick} {t:?}");
            log.push(t);
            if tick == 1 {
                validate_phase(&m, 1);
            }
        }
        validate_phase(&m, 2);
        assert_eq!(m.weights, [0; 4]);
        assert_eq!(log.len(), 4);
        // Triagem literal implementada; oito padrões válidos chegam efetivamente ao circuito.
        let mut outside = 0;
        let mut inside = 0;
        for i in 0..12 {
            let text = if i < 4 {
                "ausente R1"
            } else {
                locked::FORMS[i]
            };
            let id = locked::FORMS.iter().position(|x| *x == text);
            if id.is_none() {
                outside += 1;
            } else {
                let p = if i < 8 { [false, false] } else { [true, true] };
                assert_eq!(m.query(p, false).decision, Decision::Silent);
                inside += 1;
            }
        }
        assert_eq!((outside, inside), (4, 8));
    }
    let mut baseline = experiment::prior::Learner::new();
    for tick in 0..4 {
        let p = locked::input(if tick < 2 { 0 } else { 12 });
        let (a, e, o) = baseline.act(p);
        let r = if a == usize::from(tick >= 2) { 1 } else { -1 };
        baseline.reinforce(a, e, o, r);
        if tick == 1 {
            assert_eq!(baseline.net.weights, [1, 0, 0, 0]);
        }
    }
    assert_eq!(baseline.net.weights, [1, 0, 0, 1]);
    for i in 0..24 {
        assert_eq!(
            baseline.net.query(locked::input(i), false).decision,
            if locked::expected(i) == 0 {
                Decision::First
            } else {
                Decision::Second
            }
        );
    }
    let preset = Local {
        weights: [1, 0, 0, 1],
    };
    for (tick, p) in [(0, [true, false]), (2, [false, true])] {
        let o = preset.query(p, false);
        assert!(!experiment::action(&o, tick).1);
    }
    assert_eq!(
        preset.query([true, true], false).decision,
        Decision::Ambiguous
    );
}
