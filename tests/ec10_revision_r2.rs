//! Isolated nine-bit experimental extension; not production learning.
use std::time::{Duration, Instant};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Episode {
    id: u32,
    x: u16,
    label: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rule {
    mask: u16,
    value: u16,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    context: u32,
    log: Vec<Episode>,
    start: usize,
    rules: Vec<Rule>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Model {
    states: Vec<State>,
    independent: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Decision {
    Positive,
    Negative,
    Ambiguous,
    Missing,
}
fn rules(log: &[Episode]) -> Vec<Rule> {
    let mut result = Vec::new();
    for mask in 1u16..512 {
        if mask.count_ones() > 3 {
            continue;
        }
        for value in 0u16..512 {
            if value & !mask != 0 {
                continue;
            }
            if log.iter().all(|e| (e.x & mask == value) == e.label) {
                result.push(Rule { mask, value })
            }
        }
    }
    result
}
impl Model {
    fn new(independent: bool) -> Self {
        Self {
            states: Vec::new(),
            independent,
        }
    }
    fn acquire(&mut self, ctx: u32, e: Episode) {
        assert!(e.x < 512 && e.id > 0);
        let pos = self.states.iter().position(|s| s.context == ctx);
        if pos.is_none() {
            self.states.push(State {
                context: ctx,
                log: Vec::new(),
                start: 0,
                rules: Vec::new(),
            })
        }
        let s = self.states.iter_mut().find(|s| s.context == ctx).unwrap();
        assert!(!s.log.iter().any(|old| old.id == e.id || old.x == e.x));
        s.log.push(e);
        let mut start = if self.independent { 0 } else { s.start };
        loop {
            let r = rules(&s.log[start..]);
            if !r.is_empty() {
                s.start = start;
                s.rules = r;
                break;
            }
            start += 1;
            assert!(start < s.log.len());
        }
    }
    fn query(&self, ctx: u32, x: u16) -> (Decision, usize) {
        let Some(s) = self.states.iter().find(|s| s.context == ctx) else {
            return (Decision::Missing, 0);
        };
        let (mut yes, mut no) = (false, false);
        for r in &s.rules {
            if x & r.mask == r.value {
                yes = true
            } else {
                no = true
            }
        }
        (
            match (yes, no) {
                (true, false) => Decision::Positive,
                (false, true) => Decision::Negative,
                (true, true) => Decision::Ambiguous,
                _ => Decision::Missing,
            },
            s.rules.len(),
        )
    }
}
fn old(p: usize, x: u16) -> bool {
    match p {
        0 => x & 1 != 0,
        1 => x & 2 == 0,
        2 => x & 3 == 1,
        _ => unreachable!(),
    }
}
fn revised(p: usize, x: u16) -> bool {
    old(p, x)
        && match p {
            0 => x & 2 != 0,
            1 | 2 => x & 4 != 0,
            _ => unreachable!(),
        }
}
#[derive(Clone, Debug)]
struct Dataset {
    train: Vec<u16>,
    rev: Vec<u16>,
    test: [Vec<u16>; 3],
    b: Vec<u16>,
}
fn dataset(p: usize) -> Dataset {
    let (first, end) = match p {
        0 => (0, 128),
        1 => (128, 256),
        2 => (256, 512),
        _ => unreachable!(),
    };
    let mut groups: [Vec<u16>; 3] = [vec![], vec![], vec![]];
    for x in first..end {
        let a = old(p, x);
        let b = revised(p, x);
        groups[if a && !b {
            0
        } else if a && b {
            1
        } else {
            2
        }]
        .push(x)
    }
    assert!(groups.iter().all(|g| g.len() >= 24));
    let mut train = vec![];
    let mut rev = vec![];
    let mut test: [Vec<u16>; 3] = [vec![], vec![], vec![]];
    for (i, g) in groups.iter().enumerate() {
        train.extend_from_slice(&g[..8]);
        rev.extend_from_slice(&g[8..16]);
        test[i].extend_from_slice(&g[16..24]);
    }
    let used: Vec<_> = train
        .iter()
        .chain(&rev)
        .chain(test.iter().flatten())
        .copied()
        .collect();
    let b: Vec<_> = (first..end).filter(|x| !used.contains(x)).take(8).collect();
    assert_eq!(b.len(), 8);
    let mut all = used;
    all.extend(&b);
    let mut sorted = all.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), all.len());
    for &x in rev.iter().take(8).chain(test[0].iter()) {
        assert!(old(p, x) && !revised(p, x))
    }
    for x in rev
        .iter()
        .skip(8)
        .chain(test[1].iter())
        .chain(test[2].iter())
    {
        assert_eq!(old(p, *x), revised(p, *x))
    }
    if p == 2 {
        assert!(all.iter().all(|x| *x >= 256));
    }
    Dataset {
        train,
        rev,
        test,
        b,
    }
}
fn feed(m: &mut Model, ctx: u32, xs: &[u16], p: usize, kind: u8, start: u32, rename: bool) {
    for (i, &x) in xs.iter().enumerate() {
        let id = start + i as u32;
        let label = match kind {
            0 => old(p, x),
            1 => revised(p, x),
            2 => x & 8 != 0,
            _ => unreachable!(),
        };
        let e = Episode {
            id: if rename { id + 1000 } else { id },
            x,
            label,
        };
        if rename {
            assert_eq!(e.id - 1000, id)
        }
        m.acquire(if rename { ctx * 10 } else { ctx }, e)
    }
}
fn normalized(m: &Model) -> Model {
    let mut n = m.clone();
    for s in &mut n.states {
        s.context /= 10;
        for e in &mut s.log {
            e.id -= 1000
        }
    }
    n
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    phase: u8,
    class: usize,
    model: usize,
    correct: usize,
    wrong: usize,
    abstain: usize,
    scans: usize,
    old_correct: usize,
    old_wrong: usize,
    old_abstain: usize,
}
fn panel(p: usize, reps: usize, report: bool) {
    let started = Instant::now();
    let d = dataset(p);
    let mut previous = None;
    for rep in 0..reps {
        let (mut candidate, mut rename, mut independent, mut practice) = (
            Model::new(false),
            Model::new(false),
            Model::new(true),
            Model::new(false),
        );
        let mut frozen = Model::new(false);
        let mut literal: Vec<u16> = Vec::new();
        let mut rows = Vec::new();
        let mut before_b: Option<Vec<Decision>> = None;
        for phase in 0..4 {
            if phase == 1 {
                for m in [&mut candidate, &mut independent, &mut practice] {
                    feed(m, 7, &d.train, p, 0, 1, false)
                }
                feed(&mut rename, 7, &d.train, p, 0, 1, true);
                literal.extend(&d.train)
            }
            if phase == 2 {
                let t = Instant::now();
                std::thread::sleep(Duration::from_millis(25));
                let elapsed = t.elapsed();
                assert!(elapsed >= Duration::from_millis(25));
                if report {
                    println!("R2 interval_ms={:.3}", elapsed.as_secs_f64() * 1000.)
                }
                for m in [&mut candidate, &mut independent, &mut practice] {
                    feed(m, 8, &d.b, p, 2, 1, false)
                }
                feed(&mut rename, 8, &d.b, p, 2, 1, true);
                literal.extend(&d.b);
                frozen = candidate.clone()
            }
            if phase == 3 {
                feed(&mut candidate, 7, &d.rev, p, 1, 25, false);
                feed(&mut rename, 7, &d.rev, p, 1, 25, true);
                feed(&mut independent, 7, &d.rev, p, 1, 25, false);
                feed(&mut practice, 7, &d.rev, p, 0, 25, false);
                literal.extend(&d.rev)
            }
            assert_eq!(candidate, normalized(&rename));
            assert_eq!(candidate.states, independent.states);
            let mut all_candidate = Vec::new();
            for (class, xs) in d.test.iter().enumerate() {
                let mut totals = vec![(0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize); 6];
                for &x in xs {
                    let snapshot = (
                        candidate.clone(),
                        rename.clone(),
                        independent.clone(),
                        practice.clone(),
                        frozen.clone(),
                    );
                    let c = candidate.query(7, x);
                    let r = rename.query(70, x);
                    let i = independent.query(7, x);
                    let f = frozen.query(7, x);
                    let pr = practice.query(7, x);
                    assert_eq!(c, r);
                    assert_eq!(c.0, i.0);
                    assert_eq!(
                        snapshot,
                        (
                            candidate.clone(),
                            rename.clone(),
                            independent.clone(),
                            practice.clone(),
                            frozen.clone()
                        )
                    );
                    let l = if literal.contains(&x) {
                        panic!("evaluation vector was presented as literal")
                    } else {
                        Decision::Missing
                    };
                    let decisions = [c.0, i.0, f.0, pr.0, l, Decision::Missing];
                    let costs = [c.1, i.1, f.1, pr.1, 0, 0];
                    let expected = if phase == 3 { revised(p, x) } else { old(p, x) };
                    for (j, decision) in decisions.iter().enumerate() {
                        let v = &mut totals[j];
                        match decision {
                            Decision::Positive => {
                                if expected {
                                    v.0 += 1
                                } else {
                                    v.1 += 1
                                }
                            }
                            Decision::Negative => {
                                if !expected {
                                    v.0 += 1
                                } else {
                                    v.1 += 1
                                }
                            }
                            _ => v.2 += 1,
                        };
                        match decision {
                            Decision::Positive => {
                                if old(p, x) {
                                    v.4 += 1
                                } else {
                                    v.5 += 1
                                }
                            }
                            Decision::Negative => {
                                if !old(p, x) {
                                    v.4 += 1
                                } else {
                                    v.5 += 1
                                }
                            }
                            _ => v.6 += 1,
                        };
                        v.3 += costs[j]
                    }
                    all_candidate.push(c.0)
                }
                for (
                    model,
                    (correct, wrong, abstain, scans, old_correct, old_wrong, old_abstain),
                ) in totals.into_iter().enumerate()
                {
                    let row = Row {
                        phase,
                        class,
                        model,
                        correct,
                        wrong,
                        abstain,
                        scans,
                        old_correct,
                        old_wrong,
                        old_abstain,
                    };
                    if report {
                        println!("R2 rep={rep} {row:?}")
                    }
                    rows.push(row)
                }
            }
            if phase == 1 {
                before_b = Some(all_candidate.clone())
            }
            if phase == 2 {
                let old = before_b.as_ref().unwrap();
                let same = old
                    .iter()
                    .zip(&all_candidate)
                    .filter(|(a, b)| a == b)
                    .count();
                if report {
                    println!("R2 rep={rep} retention_same={same}/24")
                }
            }
            if report {
                for s in &candidate.states {
                    println!(
                        "R2 rep={rep} phase={phase} ctx={} version={} start={} hypotheses={}",
                        s.context,
                        s.log.len(),
                        s.start,
                        s.rules.len()
                    )
                }
            }
        }
        if let Some(old) = &previous {
            assert_eq!(old, &rows)
        }
        previous = Some(rows);
        assert!(started.elapsed().as_secs_f64() < 30.);
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
    println!("R2 seconds={:.6}", started.elapsed().as_secs_f64());
}
#[test]
fn preflight() {
    let ds = [dataset(0), dataset(1), dataset(2)];
    for i in 0..3 {
        let all: Vec<_> = ds[i]
            .train
            .iter()
            .chain(&ds[i].rev)
            .chain(ds[i].test.iter().flatten())
            .chain(&ds[i].b)
            .copied()
            .collect();
        for j in 0..i {
            assert!(all.iter().all(|x| !ds[j].train.contains(x)
                && !ds[j].rev.contains(x)
                && !ds[j].b.contains(x)
                && !ds[j].test.iter().any(|g| g.contains(x))))
        }
    }
    println!("R2 preflight F revision_discriminant=8 evaluation_discriminant=8 evaluation_invariant=16 historical_u8_overlap=0")
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
