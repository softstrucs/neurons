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
use types::{Episode, Outcome};
fn label(t: data::Target, x: u8, revised: bool) -> bool {
    let a = ((x >> t.p1) & 1) == u8::from(t.v1) && ((x >> t.p2) & 1) == u8::from(t.v2);
    a && (!revised || t.p3.is_some_and(|(p, v)| ((x >> p) & 1) == u8::from(v)))
}
fn acquire(
    models: &mut [Control],
    ctx: u32,
    t: data::Target,
    xs: &[u8],
    first: u32,
    revised: bool,
) {
    let mut vals = xs.to_vec();
    vals.sort_by_key(|x| data::rank(*x));
    for (i, x) in vals.into_iter().enumerate() {
        let e = Episode {
            id: first + i as u32,
            acquisition: true,
            x,
            label: label(t, x, revised),
        };
        for m in models.iter_mut() {
            m.acquire(ctx, e).unwrap();
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Score {
    correct: usize,
    wrong: usize,
    abstain: usize,
    version: usize,
    start: usize,
    hypotheses: usize,
    work: usize,
}
fn eval(m: &Control, ctx: u32, t: data::Target, xs: &[u8], revised: bool) -> (Score, Vec<Outcome>) {
    let before = m.clone();
    let mut s = Score {
        correct: 0,
        wrong: 0,
        abstain: 0,
        version: 0,
        start: 0,
        hypotheses: 0,
        work: 0,
    };
    let mut out = Vec::new();
    for &x in xs {
        let (answer, w) = m.query(ctx, x).unwrap();
        s.work += w.units;
        s.version = answer.version;
        s.start = answer.start;
        s.hypotheses = answer.hypotheses;
        let expected = label(t, x, revised);
        match answer.outcome {
            Outcome::Positive => {
                if expected {
                    s.correct += 1
                } else {
                    s.wrong += 1
                }
            }
            Outcome::Negative => {
                if !expected {
                    s.correct += 1
                } else {
                    s.wrong += 1
                }
            }
            _ => s.abstain += 1,
        };
        out.push(answer.outcome);
    }
    assert_eq!(&before, m);
    (s, out)
}
fn panel(ca: u32, cb: u32, reps: usize, report: bool) {
    let start = Instant::now();
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
    let av = data::generate(a).unwrap().0;
    let bv = data::generate(b).unwrap().0;
    assert_eq!(av.len(), 120);
    let mut old = None;
    for rep in 0..reps {
        let mut models = [
            Control::new(Condition::Candidate),
            Control::new(Condition::Independent),
            Control::new(Condition::Literal),
            Control::new(Condition::NoAcquisition),
        ];
        let mut rows = Vec::new();
        let mut before_b: Option<Vec<Outcome>> = None;
        for phase in 0..5 {
            if phase == 1 {
                acquire(&mut models, ca, a, &av[..32], 1, false)
            }
            if phase == 2 {
                let t = Instant::now();
                std::thread::sleep(Duration::from_millis(25));
                assert!(t.elapsed() >= Duration::from_millis(25));
                acquire(&mut models, cb, b, &bv[..32], 1, false)
            }
            if phase == 3 {
                let frozen = Control::frozen(&models[0]);
                models[3] = frozen;
                acquire(&mut models[..3], ca, a, &av[80..96], 33, true)
            }
            let (xs, revised) = if phase == 4 {
                (&av[96..104], true)
            } else {
                (&av[48..80], false)
            };
            let mut decisions = Vec::new();
            for (i, m) in models.iter().enumerate() {
                let (s, o) = eval(m, ca, a, xs, revised);
                if report {
                    println!("EC10 rep={rep} phase={phase} model={i} score={s:?}")
                }
                decisions.push(o);
                rows.push(s);
            }
            if phase == 1 {
                before_b = Some(decisions[0].clone());
            }
            if phase == 2 {
                let prior = before_b.as_ref().unwrap();
                let mut eligible = 0usize;
                let mut retained = 0usize;
                let mut changed = 0usize;
                for (i, (&before, &after)) in prior.iter().zip(&decisions[0]).enumerate() {
                    let expected = if label(a, xs[i], false) {
                        Outcome::Positive
                    } else {
                        Outcome::Negative
                    };
                    if before == expected {
                        eligible += 1;
                        if after == expected {
                            retained += 1;
                        }
                    }
                    if before != after {
                        changed += 1;
                    }
                }
                if report {
                    println!("EC10 rep={rep} retention={retained}/{eligible} changed={changed}");
                }
            }
            assert_eq!(
                decisions[0], decisions[1],
                "indutores divergentes phase {phase}"
            );
        }
        if let Some(previous) = &old {
            assert_eq!(previous, &rows)
        }
        old = Some(rows);
        assert!(start.elapsed().as_secs_f64() < 30.);
        let st = std::fs::read_to_string("/proc/self/status").unwrap();
        let h: usize = st
            .lines()
            .find(|x| x.starts_with("VmHWM:"))
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        assert!(h < 256 * 1024);
        let o = std::process::Command::new("df")
            .args(["-B1", "--output=avail", "/home"])
            .output()
            .unwrap();
        let f: u64 = String::from_utf8(o.stdout)
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        assert!(f >= 3 * 1024 * 1024 * 1024);
    }
    println!("EC10 seconds={:.6}", start.elapsed().as_secs_f64());
}
#[test]
fn development() {
    panel(2101, 2102, 1, false)
}
#[test]
fn validation() {
    panel(2201, 2202, 1, false)
}
#[test]
fn frozen_final() {
    panel(2301, 2302, 2, true)
}
