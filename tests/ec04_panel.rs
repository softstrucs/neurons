//! EC-04 evaluator: targets and labels never enter the model query interface.
#[path = "support/ec04_candidate.rs"]
mod candidate;
#[path = "support/ec04_controls.rs"]
mod controls;
#[allow(dead_code)]
#[path = "support/ec04_generator.rs"]
mod data;
#[path = "support/ec04_independent.rs"]
mod independent;
#[allow(dead_code)]
#[path = "support/ec04_types.rs"]
mod types;
use controls::{Condition, Control};
use neurons::{
    cognition::{Hypothesis, Outcome as Inferred, TwoHopTask},
    engram::{Cue, Engram, EngramId, EngramStore, RecordStatus},
    event::NodeId,
};
use std::{collections::BTreeMap, time::Instant};
use types::{Episode, Outcome};
#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    task: u8,
    phase: u8,
    block: u8,
    context: u8,
    x: u32,
    decision: i8,
    label: bool,
    validation: bool,
    version: u16,
    start: u16,
    end: u16,
    hypotheses: u16,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Score {
    positive: u32,
    negative: u32,
    correct_positive: u32,
    correct_negative: u32,
    errors: u32,
    abstentions: u32,
    inconclusive: u32,
}
fn guard(start: Instant, bytes: usize) {
    assert!(start.elapsed().as_secs_f64() < 30.0, "STOP time");
    assert!(bytes <= 256 * 1024, "STOP data {bytes}");
    let s = std::fs::read_to_string("/proc/self/status").unwrap();
    let rss: usize = s
        .lines()
        .find(|s| s.starts_with("VmRSS:"))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    assert!(rss < 256 * 1024, "STOP RSS");
    let o = std::process::Command::new("df")
        .args(["-B1", "--output=avail", "/home"])
        .output()
        .unwrap();
    assert!(o.status.success());
    let free: u64 = std::str::from_utf8(&o.stdout)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    assert!(free >= 3 * 1024 * 1024 * 1024, "STOP disk");
}
fn label(t: data::Target, x: u8, revised: bool) -> bool {
    ((x >> t.p1) & 1 == u8::from(t.v1))
        && ((x >> t.p2) & 1 == u8::from(t.v2))
        && (!revised || t.p3.is_some_and(|(p, v)| (x >> p) & 1 == u8::from(v)))
}
fn targets(part: u8) -> Vec<(data::Target, data::Target)> {
    use data::Target as T;
    match part {
        0 => vec![(
            T {
                p1: 1,
                v1: true,
                p2: 4,
                v2: false,
                p3: Some((7, true)),
            },
            T {
                p1: 0,
                v1: true,
                p2: 6,
                v2: true,
                p3: None,
            },
        )],
        1 => vec![(
            T {
                p1: 3,
                v1: false,
                p2: 5,
                v2: true,
                p3: Some((0, false)),
            },
            T {
                p1: 2,
                v1: true,
                p2: 4,
                v2: false,
                p3: None,
            },
        )],
        _ => vec![
            (
                T {
                    p1: 0,
                    v1: true,
                    p2: 1,
                    v2: true,
                    p3: Some((4, true)),
                },
                T {
                    p1: 2,
                    v1: true,
                    p2: 3,
                    v2: false,
                    p3: None,
                },
            ),
            (
                T {
                    p1: 2,
                    v1: false,
                    p2: 3,
                    v2: true,
                    p3: Some((6, false)),
                },
                T {
                    p1: 4,
                    v1: true,
                    p2: 5,
                    v2: true,
                    p3: None,
                },
            ),
            (
                T {
                    p1: 4,
                    v1: true,
                    p2: 5,
                    v2: false,
                    p3: Some((0, true)),
                },
                T {
                    p1: 6,
                    v1: false,
                    p2: 7,
                    v2: false,
                    p3: None,
                },
            ),
            (
                T {
                    p1: 6,
                    v1: false,
                    p2: 7,
                    v2: false,
                    p3: Some((2, false)),
                },
                T {
                    p1: 0,
                    v1: false,
                    p2: 1,
                    v2: true,
                    p3: None,
                },
            ),
        ],
    }
}
fn offer(m: &mut Control, ctx: u32, t: data::Target, values: &[u8], revised: bool) {
    let mut xs = values.to_vec();
    xs.sort_by_key(|x| data::rank(*x));
    for (i, x) in xs.into_iter().enumerate() {
        let e = Episode {
            acquisition: true,
            id: (if revised { 33 } else { 1 }) + i as u32,
            x,
            label: label(t, x, revised),
        };
        let w = m.acquire(ctx, e).unwrap();
        assert!(w.units <= 4_000_000);
    }
}
fn ask(
    m: &Control,
    rows: &mut Vec<Row>,
    task: u8,
    phase: u8,
    block: u8,
    ci: u8,
    ctx: u32,
    t: data::Target,
    xs: &[u8],
    revised: bool,
    validation: bool,
) {
    for &x in xs {
        let before = m.clone();
        let (a, w) = m.query(ctx, x).unwrap();
        assert_eq!(*m, before);
        assert!(w.units <= 8192);
        let d = match a.outcome {
            Outcome::Positive => 1,
            Outcome::Negative => 0,
            Outcome::Inconclusive => -2,
            _ => -1,
        };
        rows.push(Row {
            task,
            phase,
            block,
            context: ci,
            x: u32::from(x),
            decision: d,
            label: label(t, x, revised),
            validation,
            version: a.version as u16,
            start: a.start as u16,
            end: a.end as u16,
            hypotheses: a.hypotheses as u16,
        });
    }
}
fn concepts(part: u8, condition: Condition, start: Instant) -> Vec<Row> {
    let mut rows = Vec::new();
    for (block, (a, b)) in targets(part).into_iter().enumerate() {
        let av = data::generate(a).unwrap().0;
        let bv = data::generate(b).unwrap().0;
        let ctx = if part == 0 {
            2001
        } else if part == 1 {
            3001
        } else {
            1001 + 2 * block as u32
        };
        let mut m = Control::new(condition);
        let mut reference = Control::new(Condition::Candidate);
        for c in [ctx, ctx + 1] {
            m.declare(c);
            reference.declare(c);
        }
        for (ci, t, v) in [(0, a, &av), (1, b, &bv)] {
            ask(
                &m,
                &mut rows,
                2,
                0,
                block as u8,
                ci,
                ctx + ci as u32,
                t,
                &v[48..80],
                false,
                false,
            );
        }
        offer(&mut m, ctx, a, &av[..32], false);
        offer(&mut m, ctx + 1, b, &bv[..32], false);
        if condition == Condition::NoUpdate {
            offer(&mut reference, ctx, a, &av[..32], false);
            offer(&mut reference, ctx + 1, b, &bv[..32], false);
        }
        for (ci, t, v) in [(0, a, &av), (1, b, &bv)] {
            ask(
                &m,
                &mut rows,
                2,
                1,
                block as u8,
                ci,
                ctx + ci as u32,
                t,
                &v[32..48],
                false,
                true,
            );
            ask(
                &m,
                &mut rows,
                2,
                1,
                block as u8,
                ci,
                ctx + ci as u32,
                t,
                &v[48..80],
                false,
                false,
            );
        }
        if condition == Condition::NoUpdate {
            m = Control::frozen(&reference);
        }
        let mut pos: Vec<_> = bv[48..80]
            .iter()
            .copied()
            .filter(|x| label(b, *x, false))
            .collect();
        let mut neg: Vec<_> = bv[48..80]
            .iter()
            .copied()
            .filter(|x| !label(b, *x, false))
            .collect();
        pos.sort_by_key(|x| data::rank(*x));
        neg.sort_by_key(|x| data::rank(*x));
        let retention: Vec<_> = pos[..4].iter().chain(&neg[..12]).copied().collect();
        ask(
            &m,
            &mut rows,
            3,
            0,
            block as u8,
            0,
            ctx,
            a,
            &av[104..120],
            true,
            false,
        );
        let before = rows.len();
        ask(
            &m,
            &mut rows,
            3,
            0,
            block as u8,
            1,
            ctx + 1,
            b,
            &retention,
            false,
            false,
        );
        let retained = rows[before..].to_vec();
        offer(&mut m, ctx, a, &av[80..96], true);
        ask(
            &m,
            &mut rows,
            3,
            1,
            block as u8,
            0,
            ctx,
            a,
            &av[96..104],
            true,
            true,
        );
        ask(
            &m,
            &mut rows,
            3,
            1,
            block as u8,
            0,
            ctx,
            a,
            &av[104..120],
            true,
            false,
        );
        let before = rows.len();
        ask(
            &m,
            &mut rows,
            3,
            1,
            block as u8,
            1,
            ctx + 1,
            b,
            &retention,
            false,
            false,
        );
        for (old, new) in retained.iter().zip(&rows[before..]) {
            let mut old = old.clone();
            old.phase = 1;
            assert_eq!(&old, new, "retention changed");
        }
        guard(
            start,
            rows.capacity() * std::mem::size_of::<Row>() + 96 * 1024,
        );
    }
    rows
}
fn score(rows: &[Row], task: u8, phase: u8, context: u8, block: Option<u8>) -> Score {
    let mut s = Score::default();
    for r in rows.iter().filter(|r| {
        r.task == task
            && r.phase == phase
            && r.context == context
            && !r.validation
            && block.is_none_or(|b| r.block == b)
    }) {
        if r.label {
            s.positive += 1
        } else {
            s.negative += 1
        };
        if r.decision < 0 {
            if r.decision == -2 {
                s.inconclusive += 1
            } else {
                s.abstentions += 1
            };
            continue;
        }
        if (r.decision == 1) == r.label {
            if r.label {
                s.correct_positive += 1
            } else {
                s.correct_negative += 1
            }
        } else {
            s.errors += 1
        }
    }
    s
}
fn recall(part: u8, condition: usize) -> Vec<Row> {
    let mut rows = Vec::new();
    let mut pairs = Vec::new();
    let mut independent = BTreeMap::new();
    for i in 0..16u32 {
        let k = 101 + 37 * i;
        let v = 1009 + 53 * i;
        if condition != 2 {
            pairs.push((k, v));
        }
        if condition == 4 {
            independent.insert(k, v);
        }
    }
    for validation in [true, false] {
        let mut queries = Vec::new();
        if validation {
            for i in 0..4 {
                queries.push((101 + 37 * i, Some(1009 + 53 * i)));
            }
            for i in 0..4 {
                queries.push((10001 + 37 * i, None));
            }
        } else {
            for i in 4..12 {
                queries.push((101 + 37 * i, Some(1009 + 53 * i)));
            }
            for i in 0..8 {
                queries.push((20001 + 37 * i, None));
            }
        }
        for (x, expected) in queries {
            let response = match condition {
                0 | 1 => pairs.iter().find(|p| p.0 == x).map(|p| p.1),
                4 => independent.get(&x).copied(),
                5 => Some(0),
                _ => None,
            };
            let correct = response == expected;
            rows.push(Row {
                task: 0,
                phase: 1,
                block: part,
                context: 0,
                x,
                decision: if correct { 1 } else { 0 },
                label: expected.is_some(),
                validation,
                version: u16::from(response.is_some()),
                start: 0,
                end: response.unwrap_or(0) as u16,
                hypotheses: 0,
            });
        }
    }
    rows
}
fn composition(part: u8, condition: usize) -> (Vec<Row>, usize) {
    let mut rows = Vec::new();
    let mut events = 0;
    let groups: Vec<(bool, std::ops::Range<usize>)> = match part {
        0 => vec![(false, 0..4)],
        1 => vec![(false, 4..6)],
        _ => vec![(true, 4..6), (false, 6..14)],
    };
    for (validation, graphs) in groups {
        for g in graphs {
            let nodes: Vec<_> = (0..8)
                .map(|k| NodeId::new((10000 + 32 * g + k + 1) as u32).unwrap())
                .collect();
            let context = (5000 + g) as u32;
            let edges = [(0, 1), (1, 2), (3, 4), (4, 5), (6, 7), (1, 7)];
            let mut store = EngramStore::new(8).unwrap();
            let mut adjacency: BTreeMap<usize, Vec<(usize, u32)>> = BTreeMap::new();
            let mut facts = Vec::new();
            for (j, &(from, to)) in edges.iter().enumerate() {
                let id = (1000 + 16 * g + j + 1) as u32;
                if condition != 2 {
                    facts.push((from, to, id));
                    adjacency.entry(from).or_default().push((to, id));
                    store
                        .record(
                            Engram::new(
                                EngramId::new(id).unwrap(),
                                Cue::new(context, nodes[from], nodes[to].get() as i32).unwrap(),
                                1,
                                None,
                                0,
                                None,
                                RecordStatus::Observed,
                            )
                            .unwrap(),
                        )
                        .unwrap();
                }
            }
            for (from, to, expected) in [(0, 2, true), (0, 5, false), (3, 5, true), (3, 2, false)] {
                let proof: Option<(u32, u32)> = match condition {
                    0 => {
                        let r = TwoHopTask::new(8, 32, 128)
                            .unwrap()
                            .evaluate(
                                &store,
                                Hypothesis::new(context, nodes[from], nodes[to]).unwrap(),
                            )
                            .unwrap();
                        events += r.processed_events;
                        match r.outcome {
                            Inferred::Inferred(p) => {
                                Some((p.first_fact.get(), p.second_fact.get()))
                            }
                            Inferred::Unsupported => None,
                            _ => panic!("inconclusive circuit"),
                        }
                    }
                    4 => {
                        let mut found = None;
                        if let Some(first) = adjacency.get(&from) {
                            for &(via, id) in first {
                                if let Some(second) = adjacency.get(&via) {
                                    for &(end, id2) in second {
                                        if end == to {
                                            found = Some((id, id2));
                                        }
                                    }
                                }
                            }
                        }
                        found
                    }
                    5 => {
                        let mut found = None;
                        for &(a, b, id) in &facts {
                            for &(c, d, id2) in &facts {
                                if a == from && b == c && d == to {
                                    found = Some((id, id2));
                                }
                            }
                        }
                        found
                    }
                    _ => None,
                };
                let supported = if condition == 6 {
                    true
                } else {
                    proof.is_some()
                };
                let proof_valid = proof.is_some();
                let correct = if expected {
                    supported && proof_valid
                } else {
                    !supported
                };
                rows.push(Row {
                    task: 1,
                    phase: 1,
                    block: g as u8,
                    context: 0,
                    x: (from * 8 + to) as u32,
                    decision: if correct { 1 } else { 0 },
                    label: expected,
                    validation,
                    version: u16::from(supported),
                    start: proof.map_or(0, |p| p.0 as u16),
                    end: proof.map_or(0, |p| p.1 as u16),
                    hypotheses: 0,
                });
            }
        }
    }
    (rows, events)
}
fn run(part: u8) {
    let start = Instant::now();
    let mut final_count = 0;
    let mut val_count = 0;
    let mut event_count = 0;
    for condition in 0..7 {
        let a = recall(part, condition);
        let b = if part == 2 {
            recall(part, condition)
        } else {
            Vec::new()
        };
        if part == 2 {
            assert_eq!(a, b);
        }
        for r in a.iter().chain(&b) {
            if r.validation {
                val_count += 1
            } else {
                final_count += 1
            }
        }
        println!(
            "TR condition={condition} correct={} total={}",
            a.iter()
                .filter(|r| !r.validation && r.decision == 1)
                .count(),
            16
        );
    }
    for condition in 0..8 {
        let (a, e) = composition(part, condition);
        let (b, e2) = if part == 2 {
            composition(part, condition)
        } else {
            (Vec::new(), 0)
        };
        if part == 2 {
            assert_eq!(a, b);
            assert_eq!(e, e2);
        }
        event_count += e + e2;
        for r in a.iter().chain(&b) {
            if r.validation {
                val_count += 1
            } else {
                final_count += 1
            }
        }
        println!(
            "TC condition={condition} correct={} total={}",
            a.iter()
                .filter(|r| !r.validation && r.decision == 1)
                .count(),
            a.iter().filter(|r| !r.validation).count()
        );
    }
    let conditions = [
        Condition::Candidate,
        Condition::Literal,
        Condition::AlwaysPositive,
        Condition::AlwaysNegative,
        Condition::AlwaysAbstain,
        Condition::NoAcquisition,
        Condition::NoUpdate,
        Condition::Independent,
        Condition::FixedP7,
    ];
    let mut reference = None;
    for condition in conditions {
        let a = concepts(part, condition, start);
        let b = if part == 2 {
            concepts(part, condition, start)
        } else {
            Vec::new()
        };
        if part == 2 {
            assert_eq!(a, b);
        }
        for r in a.iter().chain(&b) {
            if r.validation {
                val_count += 1
            } else {
                final_count += 1
            }
        }
        if condition == Condition::Candidate {
            reference = Some(a.clone());
        }
        if condition == Condition::Independent {
            assert_eq!(reference.as_ref().unwrap(), &a);
        }
        for block in 0..targets(part).len() as u8 {
            for (task, phase, context) in [
                (2, 0, 0),
                (2, 0, 1),
                (2, 1, 0),
                (2, 1, 1),
                (3, 0, 0),
                (3, 0, 1),
                (3, 1, 0),
                (3, 1, 1),
            ] {
                let s = score(&a, task, phase, context, Some(block));
                println!("DETAIL {condition:?} block={block} task={task} phase={phase} context={context} {s:?}");
                if matches!(condition, Condition::Candidate | Condition::Independent) {
                    let expected = if task == 2 && phase == 0 {
                        (0, 0, 0, 32)
                    } else if task == 3 && context == 0 && phase == 0 {
                        (2, 6, 8, 0)
                    } else if task == 2 {
                        (8, 24, 0, 0)
                    } else if context == 0 {
                        (2, 14, 0, 0)
                    } else {
                        (4, 12, 0, 0)
                    };
                    assert_eq!(
                        (
                            s.correct_positive,
                            s.correct_negative,
                            s.errors,
                            s.abstentions
                        ),
                        expected
                    );
                    assert_eq!(s.inconclusive, 0);
                }
            }
        }

        for (task, phase, context) in [
            (2, 0, 0),
            (2, 1, 0),
            (2, 1, 1),
            (3, 0, 0),
            (3, 1, 0),
            (3, 1, 1),
        ] {
            println!(
                "CONCEPT {condition:?} task={task} phase={phase} context={context} {:?}",
                score(&a, task, phase, context, None)
            );
        }
        guard(
            start,
            (a.capacity()
                + b.capacity()
                + reference.as_ref().map_or(0, |r: &Vec<Row>| r.capacity()))
                * std::mem::size_of::<Row>()
                + 64 * 1024,
        );
    }
    if part != 2 {
        assert_eq!(final_count + val_count, if part == 0 { 2384 } else { 2320 });
    }
    if part == 2 {
        assert_eq!((final_count, val_count, event_count), (14560, 3120, 280));
    }
    println!("EC04 part={part} final_queries={final_count} validation_queries={val_count} circuit_events={event_count} seconds={:.6}",start.elapsed().as_secs_f64());
}
#[test]
fn development_panel() {
    run(0)
}
#[test]
fn validation_panel() {
    run(1)
}
#[test]
fn frozen_final_panel() {
    run(2)
}
