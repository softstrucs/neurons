use std::time::Instant;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Condition {
    Informative,
    NoEvidence,
    Redundant,
    Contradictory,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    id: u32,
    target: u8,
    condition: Condition,
    before: Vec<u8>,
    observations: Vec<(u32, u8)>,
    intermediate: Vec<u8>,
    after: Vec<u8>,
    cost: u8,
    gain: Option<usize>,
    abstain: bool,
}
fn apply(h: &[u8], value: u8) -> Vec<u8> {
    h.iter().copied().filter(|x| *x == value).collect()
}
fn run(id: u32, target: u8, condition: Condition) -> Row {
    let before = vec![0, 1];
    // Escolha da condição antecede acesso do ambiente ao alvo.
    let observations = match condition {
        Condition::Informative => vec![(id, target)],
        Condition::NoEvidence => vec![],
        Condition::Redundant => vec![(id, target), (id, target)],
        Condition::Contradictory => vec![(id, 2)],
    };
    let intermediate = observations
        .first()
        .map_or_else(|| before.clone(), |(_, v)| apply(&before, *v));
    let after = observations
        .iter()
        .skip(1)
        .fold(intermediate.clone(), |h, (_, v)| apply(&h, *v));
    let cost = observations.len() as u8;
    assert!(cost <= 2);
    let gain = if after.is_empty() {
        None
    } else {
        Some(before.len() - after.len())
    };
    let abstain = after.len() != 1;
    Row {
        id,
        target,
        condition,
        before,
        observations,
        intermediate,
        after,
        cost,
        gain,
        abstain,
    }
}
fn panel(first: u32, count: u32, reps: usize, final_panel: bool) {
    let start = Instant::now();
    let mut prior = None;
    for rep in 0..reps {
        let mut rows = Vec::new();
        for id in first..first + count {
            for target in 0..2 {
                for condition in [
                    Condition::Informative,
                    Condition::NoEvidence,
                    Condition::Redundant,
                    Condition::Contradictory,
                ] {
                    let row = run(id, target, condition);
                    match condition {
                        Condition::Informative => {
                            assert_eq!(row.after, vec![target]);
                            assert_eq!((row.gain, row.cost, row.abstain), (Some(1), 1, false));
                        }
                        Condition::NoEvidence => assert_eq!(
                            (row.after.len(), row.gain, row.cost, row.abstain),
                            (2, Some(0), 0, true)
                        ),
                        Condition::Redundant => {
                            assert_eq!(row.after, row.intermediate);
                            assert_eq!((row.gain, row.cost, row.abstain), (Some(1), 2, false));
                        }
                        Condition::Contradictory => assert_eq!(
                            (row.after.len(), row.gain, row.cost, row.abstain),
                            (0, None, 1, true)
                        ),
                    }
                    rows.push(row);
                }
            }
        }
        if final_panel {
            assert_eq!(rows.len(), 128);
            for condition in [
                Condition::Informative,
                Condition::NoEvidence,
                Condition::Redundant,
                Condition::Contradictory,
            ] {
                let r: Vec<_> = rows.iter().filter(|x| x.condition == condition).collect();
                println!("INFO rep={rep} condition={condition:?} cases={} gain={} defined={} cost={} abstain={}",r.len(),r.iter().filter_map(|x|x.gain).sum::<usize>(),r.iter().filter(|x|x.gain.is_some()).count(),r.iter().map(|x|usize::from(x.cost)).sum::<usize>(),r.iter().filter(|x|x.abstain).count());
            }
        }
        if let Some(old) = &prior {
            assert_eq!(old, &rows)
        }
        prior = Some(rows);
        assert!(start.elapsed().as_secs_f64() < 30.0);
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
        let out = std::process::Command::new("df")
            .args(["-B1", "--output=avail", "/home"])
            .output()
            .unwrap();
        let free: u64 = String::from_utf8(out.stdout)
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        assert!(free >= 3 * 1024 * 1024 * 1024);
    }
    println!("INFO seconds={:.6}", start.elapsed().as_secs_f64());
}
#[test]
fn development() {
    panel(100, 4, 1, false)
}
#[test]
fn validation() {
    panel(200, 4, 1, false)
}
#[test]
fn frozen_final() {
    panel(300, 16, 2, true)
}
