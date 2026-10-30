//! EC-07: seleção sintética; o alvo nunca entra nas políticas.
use std::time::Instant;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Action {
    id: u8,
    allowed: bool,
    cost: u8,
    prediction: [u8; 2],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Policy {
    Active,
    Random,
    Fixed,
    NoEvidence,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    profile: u8,
    target: u8,
    seed: u8,
    policy: Policy,
    action: Option<u8>,
    signal: Option<u8>,
    answer: Option<u8>,
    cost: u8,
}
fn actions(profile: u8) -> ([Action; 3], u8) {
    let special = match profile {
        0 | 5 | 6 => Some(2),
        1 => Some(0),
        2 => Some(1),
        4 => Some(2),
        _ => None,
    };
    let budget = 1;
    let a = std::array::from_fn(|i| Action {
        id: i as u8,
        allowed: !(profile == 5 && i == 2),
        cost: if profile == 4 { 2 } else { 1 },
        prediction: if special == Some(i) { [0, 1] } else { [0, 0] },
    });
    (a, budget)
}
fn choose(policy: Policy, a: &[Action; 3], budget: u8, seed: u8) -> Option<Action> {
    let eligible: Vec<_> = a
        .iter()
        .copied()
        .filter(|x| x.allowed && x.cost <= budget)
        .collect();
    match policy {
        Policy::Active => eligible
            .into_iter()
            .filter(|x| x.prediction[0] != x.prediction[1])
            .min_by_key(|x| (x.cost, x.id)),
        Policy::Fixed => eligible.first().copied(),
        Policy::Random => {
            if eligible.is_empty() {
                None
            } else {
                Some(eligible[(usize::from(seed) - 1) % eligible.len()])
            }
        }
        Policy::NoEvidence => None,
    }
}
fn run(profile: u8, target: u8, seed: u8, policy: Policy) -> Row {
    let (a, budget) = actions(profile);
    let before = a;
    let choice = choose(policy, &a, budget, seed);
    assert_eq!(before, a);
    if let Some(x) = choice {
        assert!(x.allowed && x.cost <= budget);
    }
    // Ambiente é consultado somente após a escolha; U e ações não discriminantes não produzem sinal.
    let signal = choice.and_then(|x| {
        if profile != 6 && x.prediction[0] != x.prediction[1] {
            Some(target)
        } else {
            None
        }
    });
    let answer = signal;
    if let Some(value) = answer {
        assert_eq!(value, target);
    }
    Row {
        profile,
        target,
        seed,
        policy,
        action: choice.map(|x| x.id),
        signal,
        answer,
        cost: choice.map_or(0, |x| x.cost),
    }
}
fn panel(seeds: &[u8], repeats: usize, final_panel: bool) {
    let started = Instant::now();
    let mut previous = None;
    for rep in 0..repeats {
        let mut rows = Vec::new();
        for profile in 0..7 {
            for target in 0..2 {
                for &seed in seeds {
                    for policy in [
                        Policy::Active,
                        Policy::Random,
                        Policy::Fixed,
                        Policy::NoEvidence,
                    ] {
                        let row = run(profile, target, seed, policy);
                        if policy == Policy::Active {
                            assert_eq!(row.answer.is_some(), profile <= 2);
                        }
                        if policy == Policy::Fixed {
                            assert_eq!(row.answer.is_some(), profile == 1);
                        }
                        if profile >= 3 {
                            assert!(row.answer.is_none());
                        }
                        rows.push(row);
                    }
                }
            }
        }
        if final_panel {
            assert_eq!(rows.len(), 224);
            for profile in 0..7 {
                for policy in [
                    Policy::Active,
                    Policy::Random,
                    Policy::Fixed,
                    Policy::NoEvidence,
                ] {
                    let subset: Vec<_> = rows
                        .iter()
                        .filter(|r| r.profile == profile && r.policy == policy)
                        .collect();
                    let correct = subset.iter().filter(|r| r.answer.is_some()).count();
                    let chosen = subset.iter().filter(|r| r.action.is_some()).count();
                    let cost: usize = subset.iter().map(|r| usize::from(r.cost)).sum();
                    let expected = match policy {
                        Policy::Active => {
                            if profile <= 2 {
                                8
                            } else {
                                0
                            }
                        }
                        Policy::Fixed => {
                            if profile == 1 {
                                8
                            } else {
                                0
                            }
                        }
                        Policy::Random => match profile {
                            0 | 2 => 2,
                            1 => 4,
                            _ => 0,
                        },
                        Policy::NoEvidence => 0,
                    };
                    assert_eq!(correct, expected);
                    println!("EC07 rep={rep} profile={profile} policy={policy:?} cases={} correct={correct} abstain={} chosen={chosen} cost={cost}",subset.len(),subset.len()-correct);
                }
            }
        }
        if let Some(old) = &previous {
            assert_eq!(old, &rows);
        }
        previous = Some(rows);
        assert!(started.elapsed().as_secs_f64() < 30.0);
        assert!(std::mem::size_of::<Row>() * 224 * 2 < 256 * 1024);
        let status = std::fs::read_to_string("/proc/self/status").unwrap();
        let rss: usize = status
            .lines()
            .find(|x| x.starts_with("VmHWM:"))
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        assert!(rss < 256 * 1024);
        let df = std::process::Command::new("df")
            .args(["-B1", "--output=avail", "/home"])
            .output()
            .unwrap();
        let free: u64 = String::from_utf8(df.stdout)
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        assert!(free >= 3 * 1024 * 1024 * 1024);
    }
    println!("EC07 seconds={:.6}", started.elapsed().as_secs_f64());
}
#[test]
fn development() {
    panel(&[11], 1, false)
}
#[test]
fn validation() {
    panel(&[21], 1, false)
}
#[test]
fn frozen_final() {
    panel(&[1, 2, 3, 4], 2, true)
}
