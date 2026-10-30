#[allow(dead_code)]
#[path = "support/ec04_candidate.rs"]
mod candidate;
#[allow(dead_code)]
#[path = "support/ec04_controls.rs"]
mod controls;
#[allow(dead_code)]
#[path = "support/ec04_generator.rs"]
mod data;
#[allow(dead_code)]
#[path = "support/ec04_independent.rs"]
mod independent;
#[allow(dead_code)]
#[path = "support/ec04_types.rs"]
mod types;
use controls::{Condition, Control};
use std::time::{Duration, Instant};
use types::{Answer, Episode, Outcome};
fn partitions() -> [Vec<u8>; 3] {
    let a = data::Target {
        p1: 1,
        v1: true,
        p2: 4,
        v2: false,
        p3: Some((7, true)),
    };
    let b = data::Target {
        p1: 0,
        v1: true,
        p2: 6,
        v2: true,
        p3: None,
    };
    let mut used = [false; 256];
    for x in data::generate(a)
        .unwrap()
        .0
        .into_iter()
        .chain(data::generate(b).unwrap().0)
    {
        used[x as usize] = true;
    }
    let rest: Vec<u8> = (0..=255u8).filter(|x| !used[*x as usize]).collect();
    assert!(rest.len() >= 96);
    let parts = [
        rest[..32].to_vec(),
        rest[32..64].to_vec(),
        rest[64..].to_vec(),
    ];
    for i in 0..3 {
        assert!(parts[i].len() >= 32);
        for x in &parts[i] {
            assert!(!used[*x as usize]);
            for j in 0..i {
                assert!(!parts[j].contains(x));
            }
        }
    }
    parts
}
fn truth(part: usize, kind: u8, x: u8) -> bool {
    let bit = |p: u8| ((x >> p) & 1) != 0;
    match (part, kind) {
        (0, 0) => bit(2),
        (0, 1) => bit(2) && bit(3),
        (0, 2) => bit(4),
        (1, 0) => !bit(3),
        (1, 1) => !bit(3) && bit(4),
        (1, 2) => bit(5),
        (2, 0) => bit(3) && !bit(5),
        (2, 1) => bit(3) && !bit(5) && bit(4),
        (2, 2) => bit(2) && !bit(4),
        _ => unreachable!(),
    }
}
fn record(models: &mut [Control], part: usize, ctx: u32, kind: u8, xs: &[u8], start: u32) {
    for (i, &x) in xs.iter().enumerate() {
        let original = Episode {
            id: start + i as u32,
            acquisition: true,
            x,
            label: truth(part, kind, x),
        };
        let renamed = Episode {
            id: original.id + 1000,
            ..original
        };
        assert_eq!(renamed.id - 1000, original.id);
        assert_eq!(
            (renamed.x, renamed.label, renamed.acquisition),
            (original.x, original.label, original.acquisition)
        );
        assert_eq!((if ctx == 7 { 70 } else { 80 }) / 10, ctx);
        for (j, m) in models.iter_mut().enumerate() {
            let (id, e) = if j == 1 {
                (ctx * 10, renamed)
            } else {
                (ctx, original)
            };
            m.acquire(id, e).unwrap();
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Metric {
    correct: usize,
    wrong: usize,
    abstain: usize,
    version: usize,
    start: usize,
    end: usize,
    hypotheses: usize,
    work: usize,
}
fn classify(a: &Answer, label: bool) -> u8 {
    match a.outcome {
        Outcome::Positive => {
            if label {
                1
            } else {
                2
            }
        }
        Outcome::Negative => {
            if !label {
                1
            } else {
                2
            }
        }
        _ => 0,
    }
}
fn panel(part: usize, reps: usize, output: bool) {
    let timer = Instant::now();
    let parts = partitions();
    let v = &parts[part];
    let (train, query, btrain, revision) = (&v[..12], &v[12..20], &v[20..26], &v[26..32]);
    let mut previous = None;
    for rep in 0..reps {
        let mut models = [
            Control::new(Condition::Candidate),
            Control::new(Condition::Candidate),
            Control::new(Condition::Independent),
            Control::new(Condition::Literal),
            Control::new(Condition::NoAcquisition),
            Control::new(Condition::NoAcquisition),
            Control::new(Condition::FixedP7),
        ];
        let mut saved = None;
        let mut rows = Vec::new();
        for phase in 0..5 {
            if phase == 1 {
                record(&mut models, part, 7, 0, train, 1)
            }
            if phase == 2 {
                let t = Instant::now();
                std::thread::sleep(Duration::from_millis(25));
                assert!(t.elapsed() >= Duration::from_millis(25));
                record(&mut models, part, 8, 2, btrain, 1)
            }
            if phase == 3 {
                models[5] = Control::frozen(&models[0]);
                record(&mut models, part, 7, 1, revision, 13)
            }
            let kind = if phase == 4 { 1 } else { 0 };
            let mut totals: Vec<Metric> = vec![
                Metric {
                    correct: 0,
                    wrong: 0,
                    abstain: 0,
                    version: 0,
                    start: 0,
                    end: 0,
                    hypotheses: 0,
                    work: 0
                };
                7
            ];
            let mut candidate_correct = Vec::new();
            for &x in query {
                let before = models.clone();
                let mut answers = Vec::new();
                for (i, m) in models.iter().enumerate() {
                    let ctx = if i == 1 { 70 } else { 7 };
                    let (ans, work) = m.query(ctx, x).unwrap();
                    totals[i].version = ans.version;
                    totals[i].start = ans.start;
                    totals[i].end = ans.end;
                    totals[i].hypotheses = ans.hypotheses;
                    totals[i].work += work.units;
                    answers.push((ans, work));
                }
                assert_eq!(before, models);
                assert_eq!(
                    answers[0], answers[1],
                    "renaming changed public state/decision/work"
                );
                assert_eq!(
                    answers[0].0, answers[2].0,
                    "independent public state/decision differs"
                );
                let label = truth(part, kind, x);
                for (i, (a, _)) in answers.iter().enumerate() {
                    match classify(a, label) {
                        1 => totals[i].correct += 1,
                        2 => totals[i].wrong += 1,
                        _ => totals[i].abstain += 1,
                    }
                }
                candidate_correct.push(classify(&answers[0].0, label));
            }
            if phase == 1 {
                saved = Some(candidate_correct.clone())
            }
            if phase == 2 {
                let p = saved.as_ref().unwrap();
                let n = p.iter().filter(|&&x| x == 1).count();
                let r = p
                    .iter()
                    .zip(&candidate_correct)
                    .filter(|(a, b)| **a == 1 && **b == 1)
                    .count();
                if output {
                    println!("R1 rep={rep} retention={r}/{n}")
                }
            }
            if output {
                for (i, m) in totals.iter().enumerate() {
                    println!("R1 rep={rep} phase={phase} model={i} {m:?}")
                }
            }
            rows.push(totals);
        }
        if let Some(old) = &previous {
            assert_eq!(old, &rows)
        }
        previous = Some(rows);
        assert!(timer.elapsed().as_secs_f64() < 30.);
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
        let d = std::process::Command::new("df")
            .args(["-B1", "--output=avail", "/home"])
            .output()
            .unwrap();
        let free: u64 = String::from_utf8(d.stdout)
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        assert!(free >= 3 * 1024 * 1024 * 1024);
    }
    println!("EC10R1 seconds={:.6}", timer.elapsed().as_secs_f64());
}
#[test]
fn development() {
    panel(0, 1, false)
}
#[test]
fn validation() {
    panel(1, 1, false)
}
#[test]
fn frozen_final() {
    panel(2, 2, true)
}
