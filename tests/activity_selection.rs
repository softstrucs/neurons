#[allow(dead_code)]
#[path = "support/reward_model.rs"]
mod prior;
use neurons::learning::{Feedback, PlasticityRule};
use prior::neural::{symbolic, Decision, Local};

// Política fixée: activité discriminante sinon exploration alternée, au plus 4 fois.
fn step(m: &mut Local, tick: usize) -> (usize, bool, [bool; 4]) {
    let p = if tick < 2 {
        [true, false]
    } else {
        [false, true]
    };
    let obs = m.query(p, false);
    let (action, explored) = match obs.decision {
        Decision::First => (0, false),
        Decision::Second => (1, false),
        _ => (tick % 2, true),
    };
    let traces = std::array::from_fn(|k| obs.activity[k / 2] > 0 && obs.activity[2 + k % 2] > 0);
    (action, explored, traces)
}
#[test]
fn activity_selection_and_null_learning_protocol() {
    let mut m = Local::new();
    let mut baseline = prior::Learner::new();
    let mut exploration = 0;
    let mut positive = 0;
    let mut negative = 0;
    for tick in 0..4 {
        let (a, e, traces) = step(&mut m, tick);
        exploration += usize::from(e);
        assert_eq!(traces, [false; 4]);
        let p = if tick < 2 {
            [true, false]
        } else {
            [false, true]
        };
        assert_eq!(m.query(p, false), symbolic(m.weights, p, false));
        let target = usize::from(tick >= 2);
        let reward = if a == target {
            positive += 1;
            Feedback::Positive
        } else {
            negative += 1;
            Feedback::Negative
        };
        for (k, eligible) in traces.into_iter().enumerate() {
            if eligible {
                m.weights[k] = PlasticityRule::new(0, 2, 1)
                    .unwrap()
                    .adjust(m.weights[k], Some(reward))
                    .unwrap()
                    .after;
            }
        }
        let (a, marks, obs) = baseline.act(p);
        let signal = if a == target { 1 } else { -1 };
        baseline.reinforce(a, marks, obs, signal);
    }
    assert_eq!((exploration, positive, negative), (4, 2, 2));
    assert_eq!(m.weights, [0; 4]);
    assert_eq!(baseline.net.weights, [1, 0, 0, 1]);
    // 24 nouvelles étiquettes réservées; seules les entrées numériques sont utilisées.
    for i in 0..24 {
        let text = format!("sonda contextual A025 {i}");
        assert!(!text.is_empty());
        let p = if i < 12 { [true, false] } else { [false, true] };
        assert_eq!(m.query(p, false).decision, Decision::Silent);
        assert_eq!(m.query(p, true).decision, Decision::Silent);
    }
    for i in 0..12 {
        if i < 4 {
            assert_ne!("inexistente", "sonda contextual A025");
        } else {
            let p = if i < 8 { [false, false] } else { [true, true] };
            assert_eq!(m.query(p, false).decision, Decision::Silent);
        }
    }
}
#[test]
fn discriminative_activity_can_select_without_oracle() {
    let m = Local {
        weights: [1, 0, 0, 1],
    };
    assert_eq!(
        step(&mut m.clone(), 0),
        (0, false, [true, false, false, false])
    );
    assert_eq!(
        step(&mut m.clone(), 2),
        (1, false, [false, false, false, true])
    );
}
