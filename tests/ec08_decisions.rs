use neurons::cognition::{Hypothesis, TwoHopTask};
use neurons::engram::{Cue, Engram, EngramId, EngramStore, RecordStatus};
use neurons::event::NodeId;
use neurons::metacognition::{AbstainReason, Decision, EvidenceRefs, Monitor};
use std::time::Instant;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fact {
    ctx: u32,
    from: u32,
    to: u32,
    val: i32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Case {
    name: &'static str,
    facts: Vec<Fact>,
    candidates: Vec<u32>,
    budget: usize,
    expected: View,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum View {
    Original,
    Revised,
    Missing,
    Conflict,
    Multiple,
    Budget,
}
fn facts(contexts: &[(u32, u8)], negative: bool) -> Vec<Fact> {
    let mut f = Vec::new();
    for &(ctx, count) in contexts {
        if count >= 1 {
            f.push(Fact {
                ctx,
                from: 1,
                to: 2,
                val: 1,
            })
        }
        if count >= 2 {
            f.push(Fact {
                ctx,
                from: 2,
                to: 3,
                val: 1,
            })
        }
    }
    if negative {
        f.push(Fact {
            ctx: 1,
            from: 1,
            to: 3,
            val: -1,
        })
    }
    f
}
fn cases() -> Vec<Case> {
    let c = |name, cs: &[(u32, u8)], neg, candidates: Vec<u32>, budget, expected| Case {
        name,
        facts: facts(cs, neg),
        candidates,
        budget,
        expected,
    };
    vec![
        c("S0", &[(1, 2)], false, vec![], 2, View::Original),
        c("S1", &[(1, 1)], false, vec![], 2, View::Missing),
        c("S2", &[(1, 2)], true, vec![], 2, View::Conflict),
        c("S3", &[(1, 1), (2, 2)], false, vec![2], 2, View::Revised),
        c("S4", &[(1, 1), (2, 1)], false, vec![2], 2, View::Missing),
        c(
            "S5",
            &[(1, 1), (2, 2), (3, 2)],
            false,
            vec![2, 3],
            3,
            View::Multiple,
        ),
        c("S6", &[(1, 1), (2, 2)], false, vec![2], 1, View::Budget),
        c("S7_before", &[(1, 1)], false, vec![], 2, View::Missing),
        c("S7_after", &[(1, 2)], false, vec![], 2, View::Original),
        c("S8", &[(1, 1)], false, vec![], 2, View::Missing),
        c("S9", &[(1, 1)], false, vec![], 2, View::Missing),
        c("S10", &[(1, 1)], false, vec![], 2, View::Missing),
        c("S11_first", &[(1, 1)], false, vec![], 2, View::Missing),
        c("S11_second", &[(1, 1)], false, vec![], 2, View::Missing),
    ]
}
fn node(x: u32) -> NodeId {
    NodeId::new(x).unwrap()
}
fn store(case: &Case, offset: u32) -> EngramStore {
    let mut s = EngramStore::new(16).unwrap();
    for (i, f) in case.facts.iter().enumerate() {
        s.record(
            Engram::new(
                EngramId::new(offset + i as u32 + 1).unwrap(),
                Cue::new(f.ctx, node(f.from), f.to as i32).unwrap(),
                f.val,
                Some(node(f.from)),
                0,
                None,
                RecordStatus::Observed,
            )
            .unwrap(),
        )
        .unwrap()
    }
    s
}
fn symbol(c: &Case) -> View {
    let mut selected = 0;
    let mut conflict = false;
    for (i, ctx) in std::iter::once(1)
        .chain(c.candidates.iter().copied())
        .enumerate()
    {
        if i >= c.budget {
            return View::Budget;
        }
        let edge = |a, b, v| {
            c.facts
                .iter()
                .any(|f| f.ctx == ctx && f.from == a && f.to == b && f.val == v)
        };
        let positive = edge(1, 2, 1) && edge(2, 3, 1);
        let bad = positive && edge(1, 3, -1);
        if bad {
            conflict = true
        } else if positive {
            if i == 0 {
                return View::Original;
            }
            selected += 1;
            if selected > 1 {
                return View::Multiple;
            }
        }
    }
    if selected == 1 {
        View::Revised
    } else if conflict {
        View::Conflict
    } else {
        View::Missing
    }
}
fn panel(offset: u32, reps: usize, final_run: bool) {
    let start = Instant::now();
    let mut previous = None;
    for rep in 0..reps {
        let mut rows = Vec::new();
        for c in cases() {
            let s = store(&c, offset);
            let before: Vec<_> = s.iter().collect();
            let query = Hypothesis::new(1, node(1), node(3)).unwrap();
            let result = Monitor::new(TwoHopTask::new(16, 32, 64).unwrap(), c.budget)
                .unwrap()
                .review(&s, query, &c.candidates)
                .unwrap();
            let view = match result.decision {
                Decision::Respond {
                    hypothesis,
                    evidence: EvidenceRefs::TwoHop { first, second },
                    ..
                } => {
                    assert_eq!(
                        (first.get(), second.get()),
                        if hypothesis.context == 1 {
                            (offset + 1, offset + 2)
                        } else {
                            (offset + 2, offset + 3)
                        }
                    );
                    if hypothesis.context == 1 {
                        View::Original
                    } else {
                        View::Revised
                    }
                }
                Decision::Abstain(AbstainReason::MissingEvidence) => View::Missing,
                Decision::Abstain(AbstainReason::ContradictoryEvidence) => View::Conflict,
                Decision::Abstain(AbstainReason::MultipleSupportedContexts) => View::Multiple,
                Decision::Abstain(AbstainReason::ReviewBudget) => View::Budget,
                other => panic!("unexpected {other:?}"),
            };
            let control = symbol(&c);
            assert_eq!(before, s.iter().collect::<Vec<_>>());
            assert_eq!(view, c.expected);
            assert_eq!(control, c.expected);
            let events: usize = result
                .attempts
                .iter()
                .filter_map(|a| a.inference.map(|x| x.processed_events))
                .sum();
            if final_run {
                println!("EC08 rep={rep} case={} model={view:?} control={control:?} attempts={} events={events} report={result:?}",c.name,result.attempt_count())
            }
            rows.push((c.name, view, control, result, events));
        }
        assert_eq!(rows.len(), 14);
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
    println!("EC08 seconds={:.6}", start.elapsed().as_secs_f64());
}
#[test]
fn development() {
    panel(100, 1, false)
}
#[test]
fn validation() {
    panel(200, 1, false)
}
#[test]
fn frozen_final() {
    panel(300, 2, true)
}
