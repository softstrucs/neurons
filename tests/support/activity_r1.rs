#[allow(dead_code)]
#[path = "reward_model.rs"]
pub mod prior;
use neurons::learning::{Feedback, PlasticityRule};
use prior::neural::{Decision, Local, Observation};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Trace {
    pub observation: Observation,
    pub action: usize,
    pub explored: bool,
    pub ambiguous: bool,
    pub eligibility: [bool; 4],
    pub reward: i8,
    pub weights: [i32; 4],
}

// Mesmo mecanismo de A02.5: atividade discriminante, senão alternância limitada.
pub fn action(obs: &Observation, tick: usize) -> (usize, bool) {
    assert!(tick < 4);
    match obs.decision {
        Decision::First => (0, false),
        Decision::Second => (1, false),
        _ => (tick % 2, true),
    }
}
pub fn trial(m: &mut Local, p: [bool; 2], tick: usize, mode: u8, ablated: bool) -> Trace {
    let obs = m.query(p, ablated);
    let (a, e) = action(&obs, tick);
    let eligibility =
        std::array::from_fn(|k| obs.activity[k / 2] > 0 && obs.activity[2 + k % 2] > 0);
    // Professor separado da seleção: somente agora consulta o alvo de treino.
    let target = usize::from(tick >= 2);
    let reward = match mode {
        0 => 0,
        1 => {
            if a == target {
                1
            } else {
                -1
            }
        }
        2 => 1,
        _ => panic!("modo"),
    };
    let rule = PlasticityRule::new(0, 2, 1).unwrap();
    for (k, eligible) in eligibility.into_iter().enumerate() {
        if eligible {
            let f = match reward {
                1 => Some(Feedback::Positive),
                -1 => Some(Feedback::Negative),
                _ => None,
            };
            m.weights[k] = rule.adjust(m.weights[k], f).unwrap().after;
        }
    }
    Trace {
        ambiguous: obs.decision == Decision::Ambiguous,
        observation: obs,
        action: a,
        explored: e,
        eligibility,
        reward,
        weights: m.weights,
    }
}
// Implementação simbólica independente de atividade, política, traços e atualização.
pub fn symbolic_trial(
    w: &mut [i32; 4],
    p: [bool; 2],
    tick: usize,
    mode: u8,
    ablated: bool,
) -> Trace {
    let mut activity = [0; 4];
    for i in 0..2 {
        if p[i] {
            activity[i] = 1;
            for j in 0..2 {
                if !ablated && w[2 * i + j] > 0 {
                    activity[2 + j] += 1;
                }
            }
        }
    }
    let decision = match (activity[2] > 0, activity[3] > 0) {
        (true, false) => Decision::First,
        (false, true) => Decision::Second,
        (true, true) => Decision::Ambiguous,
        _ => Decision::Silent,
    };
    let (a, e) = if decision == Decision::First {
        (0, false)
    } else if decision == Decision::Second {
        (1, false)
    } else {
        (tick % 2, true)
    };
    let mut eligibility = [false; 4];
    for i in 0..2 {
        for j in 0..2 {
            eligibility[2 * i + j] = activity[i] > 0 && activity[2 + j] > 0;
        }
    }
    let reward = if mode == 0 {
        0
    } else if mode == 2 || a == usize::from(tick >= 2) {
        1
    } else {
        -1
    };
    for k in 0..4 {
        if eligibility[k] {
            w[k] = (w[k] + i32::from(reward)).clamp(0, 2);
        }
    }
    Trace {
        observation: Observation { decision, activity },
        action: a,
        explored: e,
        ambiguous: decision == Decision::Ambiguous,
        eligibility,
        reward,
        weights: *w,
    }
}
