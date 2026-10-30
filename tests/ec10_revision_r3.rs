// Candidate is included unchanged from R2; the control imports no candidate code.
include!("support/ec10_r3_candidate.rs");
#[path = "support/ec10_r3_control.rs"]
mod control_r3;
use control_r3::{Independent, Output};
#[derive(Clone)]
struct R3Data {
    train: Vec<u16>,
    rev: Vec<u16>,
    test: [Vec<u16>; 3],
    b: Vec<u16>,
}
fn r3old(p: usize, x: u16) -> bool {
    match p {
        0 => x & 1 != 0,
        1 => x & 2 == 0,
        2 => x & 8 != 0 && x & 32 == 0,
        _ => unreachable!(),
    }
}
fn r3new(p: usize, x: u16) -> bool {
    r3old(p, x)
        && match p {
            0 => x & 2 != 0,
            1 => x & 4 != 0,
            2 => x & 16 != 0,
            _ => unreachable!(),
        }
}
fn r3data(p: usize) -> R3Data {
    let (base, size) = match p {
        0 => (0u16, 128u16),
        1 => (128, 128),
        2 => (256, 256),
        _ => unreachable!(),
    };
    let prev = dataset(2);
    let old_used: Vec<u16> = prev
        .train
        .iter()
        .chain(&prev.rev)
        .chain(prev.test.iter().flatten())
        .chain(&prev.b)
        .copied()
        .collect();
    let mut classes: [Vec<u16>; 3] = [vec![], vec![], vec![]];
    let mut order = Vec::new();
    for i in 0..size {
        let x = base + (73 * i + 19) % size;
        if p == 2 && old_used.contains(&x) {
            continue;
        }
        order.push(x);
        let a = r3old(p, x);
        let b = r3new(p, x);
        classes[if a && !b {
            0
        } else if a && b {
            1
        } else {
            2
        }]
        .push(x)
    }
    let counts = [4usize, 12, 16];
    let (mut train, mut rev) = (Vec::new(), Vec::new());
    let mut test: [Vec<u16>; 3] = [vec![], vec![], vec![]];
    for k in 0..3 {
        let n = counts[k];
        assert!(
            classes[k].len() >= n + 16,
            "insufficient witnesses class={k} p={p}"
        );
        train.extend_from_slice(&classes[k][..n]);
        rev.extend_from_slice(&classes[k][n..n + 8]);
        test[k].extend_from_slice(&classes[k][n + 8..n + 16]);
    }
    let mut used: Vec<u16> = train
        .iter()
        .chain(&rev)
        .chain(test.iter().flatten())
        .copied()
        .collect();
    let b: Vec<_> = order
        .into_iter()
        .filter(|x| !used.contains(x))
        .take(8)
        .collect();
    assert_eq!(b.len(), 8);
    used.extend(&b);
    let mut unique = used.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), used.len());
    if p == 2 {
        assert!(used.iter().all(|x| *x >= 256 && !old_used.contains(x)))
    }
    for x in rev.iter().take(8).chain(test[0].iter()) {
        assert!(r3old(p, *x) && !r3new(p, *x))
    }
    for x in rev
        .iter()
        .skip(8)
        .chain(test[1].iter())
        .chain(test[2].iter())
    {
        assert_eq!(r3old(p, *x), r3new(p, *x))
    }
    R3Data {
        train,
        rev,
        test,
        b,
    }
}
fn to_out(d: Decision) -> Output {
    match d {
        Decision::Positive => Output::Positive,
        Decision::Negative => Output::Negative,
        Decision::Ambiguous => Output::Ambiguous,
        Decision::Missing => Output::Missing,
    }
}
fn put(m: &mut Model, ind: &mut Independent, ctx: u32, xs: &[u16], p: usize, kind: u8, start: u32) {
    for (i, &x) in xs.iter().enumerate() {
        let y = match kind {
            0 => r3old(p, x),
            1 => r3new(p, x),
            2 => x & 8 != 0,
            _ => unreachable!(),
        };
        let id = start + i as u32;
        m.acquire(ctx, Episode { id, x, label: y });
        ind.acquire(ctx, id, x, y)
    }
}
fn old_correct(d: Decision, y: bool) -> bool {
    matches!(
        (d, y),
        (Decision::Positive, true) | (Decision::Negative, false)
    )
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct R3Row {
    phase: u8,
    class: usize,
    model: usize,
    correct_new: usize,
    correct_old: usize,
    wrong_new: usize,
    abstain: usize,
}
fn r3panel(p: usize, reps: usize, print: bool) {
    let start = Instant::now();
    let d = r3data(p);
    let mut previous = None;
    for rep in 0..reps {
        let (mut m, mut ctl, mut practice, mut frozen) = (
            Model::new(false),
            Independent::new(),
            Model::new(false),
            Model::new(false),
        );
        let mut rows = Vec::new();
        for phase in 0..4 {
            if phase == 1 {
                put(&mut m, &mut ctl, 7, &d.train, p, 0, 1);
                for (i, &x) in d.train.iter().enumerate() {
                    practice.acquire(
                        7,
                        Episode {
                            id: i as u32 + 1,
                            x,
                            label: r3old(p, x),
                        },
                    )
                }
            }
            if phase == 2 {
                let t = Instant::now();
                std::thread::sleep(Duration::from_millis(25));
                let dt = t.elapsed();
                assert!(dt >= Duration::from_millis(25));
                if print {
                    println!("R3 interval_ms={:.3}", dt.as_secs_f64() * 1000.)
                }
                put(&mut m, &mut ctl, 8, &d.b, p, 2, 1);
                for (i, &x) in d.b.iter().enumerate() {
                    practice.acquire(
                        8,
                        Episode {
                            id: i as u32 + 1,
                            x,
                            label: x & 8 != 0,
                        },
                    )
                }
                frozen = m.clone();
            }
            if phase == 3 {
                let mut required = 0;
                for xs in &d.test {
                    for &x in xs {
                        if old_correct(m.query(7, x).0, r3old(p, x)) {
                            required += 1
                        }
                    }
                }
                if print {
                    println!("R3 rep={rep} pre_revision_old_correct={required}/24")
                };
                if required != 24 {
                    println!("R3 IMPEDIMENT prior_correct={required}/24; revision not executed");
                    return;
                }
                put(&mut m, &mut ctl, 7, &d.rev, p, 1, 33);
                for (i, &x) in d.rev.iter().enumerate() {
                    practice.acquire(
                        7,
                        Episode {
                            id: 33 + i as u32,
                            x,
                            label: r3old(p, x),
                        },
                    )
                }
            }
            for (class, xs) in d.test.iter().enumerate() {
                let mut scores = vec![(0usize, 0usize, 0usize, 0usize); 4];
                for &x in xs {
                    let snapshot = (m.clone(), ctl.clone(), frozen.clone(), practice.clone());
                    let a = m.query(7, x).0;
                    let b = ctl.query(7, x);
                    let f = frozen.query(7, x).0;
                    let q = practice.query(7, x).0;
                    assert_eq!(to_out(a), b.0, "independent decision mismatch");
                    assert_eq!(
                        snapshot,
                        (m.clone(), ctl.clone(), frozen.clone(), practice.clone())
                    );
                    let expected_new = r3new(p, x);
                    let expected_old = r3old(p, x);
                    for (j, v) in [
                        a,
                        match b.0 {
                            Output::Positive => Decision::Positive,
                            Output::Negative => Decision::Negative,
                            Output::Ambiguous => Decision::Ambiguous,
                            Output::Missing => Decision::Missing,
                        },
                        f,
                        q,
                    ]
                    .iter()
                    .enumerate()
                    {
                        if old_correct(*v, expected_new) {
                            scores[j].0 += 1
                        } else if matches!(v, Decision::Ambiguous | Decision::Missing) {
                            scores[j].3 += 1
                        } else {
                            scores[j].2 += 1
                        }
                        if old_correct(*v, expected_old) {
                            scores[j].1 += 1
                        }
                    }
                }
                for (model, (correct_new, correct_old, wrong_new, abstain)) in
                    scores.into_iter().enumerate()
                {
                    let r = R3Row {
                        phase,
                        class,
                        model,
                        correct_new,
                        correct_old,
                        wrong_new,
                        abstain,
                    };
                    if print {
                        println!("R3 rep={rep} {r:?}")
                    };
                    rows.push(r)
                }
            }
        }
        if let Some(prev) = &previous {
            assert_eq!(prev, &rows)
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
    println!("R3_COMPLETE seconds={:.6}", start.elapsed().as_secs_f64());
}
#[test]
fn r3_preflight() {
    let previous = dataset(2);
    let excluded: Vec<u16> = previous
        .train
        .iter()
        .chain(&previous.rev)
        .chain(previous.test.iter().flatten())
        .chain(&previous.b)
        .copied()
        .collect();
    let mut counts = [0usize; 3];
    for x in 256..512u16 {
        if excluded.contains(&x) {
            continue;
        }
        let a = r3old(2, x);
        let b = r3new(2, x);
        counts[if a && !b {
            0
        } else if a && b {
            1
        } else {
            2
        }] += 1;
    }
    println!("R3 BLOCKED preflight available={counts:?} required=[20, 28, 32]; final not executed");
    assert_eq!(counts, [20, 20, 136]);
    assert!(counts[1] < 28);
}
#[test]
fn r3_development() {
    r3panel(0, 1, false)
}
#[test]
fn r3_validation() {
    r3panel(1, 1, false)
}
#[test]
#[ignore = "R3 preflight: only 20 invariant-positive vectors available; 28 required"]
fn r3_frozen_final() {
    r3panel(2, 2, true)
}
