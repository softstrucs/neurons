#[path = "support/reward_final_locked.rs"]
mod locked;
#[path = "support/reward_model.rs"]
mod reward;
use reward::{
    neural::{symbolic, Decision, Local},
    Learner, Symbolic,
};

fn run_step(m: &mut Learner, s: &mut Symbolic, input: [bool; 2], target: usize, mode: u8) {
    let (action, marks, observation) = m.act(input);
    let (other_action, other_marks) = s.act(input);
    assert_eq!((action, marks), (other_action, other_marks));
    // O professor avalia a ação apenas depois de fixada a elegibilidade.
    let signal = match mode {
        0 => 0,
        1 => {
            if action == target {
                1
            } else {
                -1
            }
        }
        2 => 1,
        _ => panic!("modo inválido"),
    };
    let t = m.reinforce(action, marks, observation, signal);
    let weights = s.update(other_marks, signal);
    assert_eq!(
        (t.action, t.eligibility, t.reward, t.after),
        (other_action, other_marks, signal, weights)
    );
    assert_eq!(t.observation, symbolic(t.before, input, false));
}
#[test]
fn development_exploration_eligibility_reward() {
    let mut m = Learner::new();
    let mut s = Symbolic::new();
    let (action, marks, obs) = m.act([true, false]);
    assert_eq!(action, 0);
    assert_eq!(marks, [true, false, false, false]);
    assert_eq!(obs.decision, Decision::Silent);
    m = Learner::new();
    run_step(&mut m, &mut s, [true, false], 0, 1);
    assert_eq!(m.net.weights, [1, 0, 0, 0]);
    run_step(&mut m, &mut s, [true, false], 0, 1);
    assert_eq!(m.net.weights, [1, 0, 0, 0]);
    let before = m.net.weights;
    let mut zero = Symbolic {
        weights: before,
        tick: m.tick,
    };
    run_step(&mut m, &mut zero, [true, false], 0, 0);
    assert_eq!(m.net.weights, before);
}
#[test]
fn validation_policy_and_controls_before_final() {
    let mut m = Learner::new();
    let mut s = Symbolic::new();
    for (p, t) in [
        ([true, false], 0),
        ([true, false], 0),
        ([false, true], 1),
        ([false, true], 1),
    ] {
        run_step(&mut m, &mut s, p, t, 1);
    }
    assert_eq!(m.net.weights, [1, 0, 0, 1]);
    for p in [[true, false], [false, true], [true, true], [false, false]] {
        assert_eq!(m.net.query(p, false), symbolic(m.net.weights, p, false));
    }
}
#[test]
fn frozen_global_reward_protocol() {
    let forms = locked::FORMS;
    let mut reward = Learner::new();
    let mut rep = Learner::new();
    let mut sym = Symbolic::new();
    let mut repeat_sym = Symbolic::new();
    let mut idle = Learner::new();
    let mut idle_sym = Symbolic::new();
    let mut noise = Learner::new();
    let mut noise_sym = Symbolic::new();
    let mut neutral = Learner::new();
    let mut neutral_sym = Symbolic::new();
    let mut directed = Local::new();
    for p in locked::PATTERNS {
        assert_eq!(reward.net.query(p, false).decision, Decision::Silent);
    }
    for _ in 0..2 {
        run_step(&mut reward, &mut sym, [true, false], 0, 1);
        run_step(&mut rep, &mut repeat_sym, [true, false], 0, 1);
        run_step(&mut idle, &mut idle_sym, [true, false], 0, 0);
        run_step(&mut noise, &mut noise_sym, [true, false], 0, 2);
        run_step(&mut neutral, &mut neutral_sym, [true, true], 0, 1);
        directed.learn([true, false], [true, false]);
    }
    assert_eq!(reward.net.weights, [1, 0, 0, 0]);
    assert_eq!(reward, rep);
    assert_eq!(idle.net.weights, [0; 4]);
    let mut correct = 0;
    let mut silent = 0;
    for i in 0..24 {
        let p = locked::PATTERNS[i];
        let a = reward.net.query(p, false);
        assert_eq!(a, symbolic(reward.net.weights, p, false));
        assert_eq!(a, rep.net.query(p, false));
        let expected = if locked::EXPECT_FIRST[i] {
            Decision::First
        } else {
            Decision::Second
        };
        correct += usize::from(a.decision == expected);
        silent += usize::from(a.decision == Decision::Silent);
    }
    assert_eq!((correct, silent), (12, 12));
    assert_eq!(
        reward.net.query([true, false], false).decision,
        Decision::First
    );
    assert_eq!(
        directed.query([true, false], false).decision,
        Decision::First
    );
    for _ in 0..2 {
        run_step(&mut reward, &mut sym, [false, true], 1, 1);
        run_step(&mut rep, &mut repeat_sym, [false, true], 1, 1);
        run_step(&mut idle, &mut idle_sym, [false, true], 1, 0);
        run_step(&mut noise, &mut noise_sym, [false, true], 1, 2);
        run_step(&mut neutral, &mut neutral_sym, [true, true], 1, 1);
        directed.learn([false, true], [false, true]);
    }
    assert_eq!(reward.net.weights, [1, 0, 0, 1]);
    assert_eq!(reward, rep);
    assert_eq!(noise.net.weights, [1, 1, 1, 1]);
    assert_eq!(neutral.net.weights, [0, 1, 0, 1]);
    assert_eq!(directed.weights, [2, 0, 0, 2]);
    assert_eq!(
        reward.net.query([true, false], false).decision,
        Decision::First
    );
    assert_eq!(
        reward.net.query([false, true], false).decision,
        Decision::Second
    );
    let mut hits = 0;
    let mut dir = 0;
    let mut noise_amb = 0;
    let mut neutral_hit = 0;
    let mut neutral_error = 0;
    let mut off = 0;
    for (i, form) in forms.iter().enumerate() {
        assert!(!form.is_empty());
        let p = locked::PATTERNS[i];
        let a = reward.net.query(p, false);
        assert_eq!(a, symbolic(reward.net.weights, p, false));
        assert_eq!(a, rep.net.query(p, false));
        assert_eq!(idle.net.query(p, false).decision, Decision::Silent);
        let d = directed.query(p, false);
        let n = noise.net.query(p, false);
        let z = neutral.net.query([true, true], false);
        let b = reward.net.query(p, true);
        assert_eq!(b, symbolic(reward.net.weights, p, true));
        let expected = if locked::EXPECT_FIRST[i] {
            Decision::First
        } else {
            Decision::Second
        };
        hits += usize::from(a.decision == expected);
        dir += usize::from(d.decision == expected);
        noise_amb += usize::from(n.decision == Decision::Ambiguous);
        neutral_hit += usize::from(z.decision == expected);
        neutral_error += usize::from(z.decision != expected);
        off += usize::from(b.decision == Decision::Silent);
    }
    assert_eq!(
        (hits, dir, noise_amb, neutral_hit, neutral_error, off),
        (24, 24, 24, 12, 12, 24)
    );
    // Triagem externa literal; os casos válidos sem evidência ou com sinais em conflito chegam ao circuito.
    for i in 0..12 {
        if i < 4 {
            assert!(!forms.contains(&"inexistente"));
        } else {
            let p = if i < 8 { [false, false] } else { [true, true] };
            let a = reward.net.query(p, false);
            assert_eq!(a, symbolic(reward.net.weights, p, false));
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
    println!("A02.4 pre_silent=24 first_correct=12 first_silent=12 final_correct=24 directed=24 no_update_silent=24 uninformative_ambiguous=24 neutral_correct=12 neutral_errors=12 ablated_silent=24 negative_external=4 negative_silent=4 negative_ambiguous=4 repeat=24");
}
