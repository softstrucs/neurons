use std::time::Instant;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Action {
    id: u8,
    allowed: bool,
    cost: u8,
    p: [u8; 2],
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
    seed: u64,
    policy: Policy,
    eligible: Vec<u8>,
    action: Option<u8>,
    signal: Option<u8>,
    answer: Option<u8>,
    reason: &'static str,
    cost: u8,
}
fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut x = *state;
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
fn index(state: &mut u64, n: u64) -> usize {
    assert!(n > 0);
    let threshold = n.wrapping_neg() % n;
    loop {
        let x = next(state);
        if x >= threshold {
            return (x % n) as usize;
        }
    }
}
fn actions(profile: u8) -> ([Action; 3], u8) {
    let special = match profile {
        0 | 5 | 6 => Some(2),
        1 => Some(0),
        2 => Some(1),
        4 => Some(2),
        _ => None,
    };
    (
        std::array::from_fn(|i| Action {
            id: i as u8,
            allowed: !(profile == 5 && i == 2),
            cost: if profile == 4 { 2 } else { 1 },
            p: if special == Some(i) { [0, 1] } else { [0, 0] },
        }),
        1,
    )
}
fn choose(policy: Policy, a: &[Action; 3], budget: u8, seed: u64) -> (Vec<u8>, Option<Action>) {
    let v: Vec<_> = a
        .iter()
        .copied()
        .filter(|x| x.allowed && x.cost <= budget)
        .collect();
    let ids = v.iter().map(|x| x.id).collect();
    let c = match policy {
        Policy::Active => v
            .iter()
            .copied()
            .filter(|x| x.p[0] != x.p[1])
            .min_by_key(|x| (x.cost, x.id)),
        Policy::Fixed => v.first().copied(),
        Policy::Random => {
            if v.is_empty() {
                None
            } else {
                Some(v[index(&mut seed.clone(), v.len() as u64)])
            }
        }
        Policy::NoEvidence => None,
    };
    (ids, c)
}
fn run(profile: u8, target: u8, seed: u64, policy: Policy) -> Row {
    let (a, b) = actions(profile);
    let before = a;
    let (eligible, choice) = choose(policy, &a, b, seed);
    assert_eq!(before, a);
    if let Some(c) = choice {
        assert!(c.allowed && c.cost <= b && eligible.contains(&c.id));
    }
    let signal = choice.and_then(|c| {
        if profile != 6 && c.p[0] != c.p[1] {
            Some(target)
        } else {
            None
        }
    });
    let answer = signal;
    if let Some(x) = answer {
        assert_eq!(x, target)
    }
    let reason = if choice.is_none() {
        "no_selection"
    } else if signal.is_none() {
        "no_signal"
    } else {
        "supported"
    };
    Row {
        profile,
        target,
        seed,
        policy,
        eligible,
        action: choice.map(|x| x.id),
        signal,
        answer,
        reason,
        cost: choice.map_or(0, |x| x.cost),
    }
}
fn panel(seeds: std::ops::RangeInclusive<u64>, reps: usize, final_panel: bool) {
    let start = Instant::now();
    let mut previous = None;
    for rep in 0..reps {
        let mut rows = Vec::new();
        for profile in 0..7 {
            for target in 0..2 {
                for seed in seeds.clone() {
                    for policy in [
                        Policy::Active,
                        Policy::Random,
                        Policy::Fixed,
                        Policy::NoEvidence,
                    ] {
                        let row = run(profile, target, seed, policy);
                        if profile >= 3 {
                            assert!(row.answer.is_none())
                        }
                        if policy == Policy::Active {
                            assert_eq!(row.answer.is_some(), profile <= 2)
                        }
                        if policy == Policy::Fixed {
                            assert_eq!(row.answer.is_some(), profile == 1)
                        }
                        if final_panel {
                            println!("R1 rep={rep} {row:?}");
                        }
                        rows.push(row);
                    }
                }
            }
        }
        if final_panel {
            assert_eq!(rows.len(), 1792);
            for profile in 0..7 {
                for policy in [
                    Policy::Active,
                    Policy::Random,
                    Policy::Fixed,
                    Policy::NoEvidence,
                ] {
                    let r: Vec<_> = rows
                        .iter()
                        .filter(|x| x.profile == profile && x.policy == policy)
                        .collect();
                    println!("R1_SUM rep={rep} profile={profile} policy={policy:?} cases={} correct={} chosen={} cost={}",r.len(),r.iter().filter(|x|x.answer.is_some()).count(),r.iter().filter(|x|x.action.is_some()).count(),r.iter().map(|x|u32::from(x.cost)).sum::<u32>());
                }
            }
        }
        if let Some(old) = &previous {
            assert_eq!(old, &rows)
        }
        previous = Some(rows);
        assert!(start.elapsed().as_secs_f64() < 30.);
        let status = std::fs::read_to_string("/proc/self/status").unwrap();
        let h: usize = status
            .lines()
            .find(|x| x.starts_with("VmHWM:"))
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        assert!(h < 256 * 1024);
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
    println!("R1_SECONDS {:.6}", start.elapsed().as_secs_f64());
}
#[test]
fn prng_vectors_and_positions() {
    let mut s = 0;
    assert_eq!(next(&mut s), 0xe220a8397b1dcdaf);
    assert_eq!(next(&mut s), 0x6e789e6aa1b965f4);
    for n in [2, 3] {
        let mut seen = vec![false; n];
        for seed in 1..=32 {
            seen[index(&mut seed.clone(), n as u64)] = true
        }
        assert!(seen.into_iter().all(|x| x));
    }
}
#[test]
fn development() {
    panel(101..=104, 1, false)
}
#[test]
fn validation() {
    panel(201..=204, 1, false)
}
#[test]
fn frozen_final() {
    panel(1..=32, 2, true)
}
