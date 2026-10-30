//! EC-06: isolated evaluation; expected profile never enters either decision.
use neurons::cognition::{Hypothesis, Outcome, TwoHopTask};
use neurons::engram::{Cue, Engram, EngramId, EngramStore, RecordStatus};
use neurons::event::NodeId;
use neurons::metacognition::{AbstainReason, Conflict, Decision, EvidenceRefs, Monitor};
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Fact {
    id: u32,
    from: u32,
    to: u32,
    value: i32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Case {
    context: u32,
    x: u32,
    y: u32,
    z: u32,
    facts: Vec<Fact>,
    class: u8,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ResultKind {
    Direct(u32),
    Inferred(u32, u32),
    Missing,
    Conflict,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    case: usize,
    context: u32,
    facts: Vec<Fact>,
    conflict: Option<u32>,
    model: ResultKind,
    symbolic: ResultKind,
    literal: Option<u32>,
    ablated: Option<u32>,
    model_events: usize,
    ablated_events: usize,
}
fn node(n: u32) -> NodeId {
    NodeId::new(n).unwrap()
}
fn build(offset: u32, n: usize) -> Vec<Case> {
    let mut permutation: (Vec<usize>, u64) = ((0..n).collect(), 17 + u64::from(offset));
    for j in (1..n).rev() {
        permutation.1 = permutation
            .1
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let k = (permutation.1 % (j as u64 + 1)) as usize;
        permutation.0.swap(j, k);
    }
    (0..n)
        .map(|i| {
            let base = offset + 100 * permutation.0[i] as u32;
            let (x, y, z, w) = (base + 11, base + 23, base + 37, base + 41);
            let edge = |id, from, to, value| Fact {
                id,
                from,
                to,
                value,
            };
            let facts = match i % 4 {
                0 => vec![edge(1, x, z, 1)],
                1 => vec![edge(1, x, y, 1), edge(2, y, z, 1), edge(3, x, w, 1)],
                2 => vec![edge(1, x, y, 1), edge(2, z, w, 1)],
                _ => vec![edge(1, x, y, 1), edge(2, y, z, 1), edge(3, x, z, -1)],
            };
            Case {
                context: base + 53,
                x,
                y,
                z,
                facts,
                class: (i % 4) as u8,
            }
        })
        .collect()
}
fn memory(case: &Case) -> EngramStore {
    let mut store = EngramStore::new(8).unwrap();
    for f in &case.facts {
        store
            .record(
                Engram::new(
                    EngramId::new(f.id).unwrap(),
                    Cue::new(case.context, node(f.from), f.to as i32).unwrap(),
                    f.value,
                    Some(node(f.from)),
                    0,
                    None,
                    RecordStatus::Observed,
                )
                .unwrap(),
            )
            .unwrap();
    }
    store
}
// Independent symbolic decision: only memory, context and queried endpoints.
fn symbolic(store: &EngramStore, context: u32, x: NodeId, z: NodeId) -> ResultKind {
    let records: Vec<_> = store
        .iter()
        .filter(|r| {
            r.cue().context() == context
                && r.status() != RecordStatus::Hypothesis
                && r.cue().pattern() > 0
        })
        .collect();
    let direct = records
        .iter()
        .find(|r| r.value() == 1 && r.cue().source() == x && r.cue().pattern() as u32 == z.get());
    let negative = records
        .iter()
        .any(|r| r.value() == -1 && r.cue().source() == x && r.cue().pattern() as u32 == z.get());
    if let Some(r) = direct {
        return if negative {
            ResultKind::Conflict
        } else {
            ResultKind::Direct(r.id().get())
        };
    }
    let mut proofs = Vec::new();
    for a in &records {
        for b in &records {
            if a.value() == 1
                && b.value() == 1
                && a.cue().source() == x
                && a.cue().pattern() as u32 == b.cue().source().get()
                && b.cue().pattern() as u32 == z.get()
                && a.cue().pattern() as u32 != x.get()
                && a.cue().pattern() as u32 != z.get()
            {
                proofs.push((a.id().get(), b.id().get()));
            }
        }
    }
    if proofs.is_empty() {
        ResultKind::Missing
    } else if negative {
        ResultKind::Conflict
    } else {
        proofs.sort();
        ResultKind::Inferred(proofs[0].0, proofs[0].1)
    }
}
fn literal(store: &EngramStore, context: u32, x: NodeId, z: NodeId) -> Option<u32> {
    let positives: Vec<_> = store
        .iter()
        .filter(|r| {
            r.cue().context() == context
                && r.cue().source() == x
                && r.cue().pattern() == z.get() as i32
                && r.value() == 1
                && r.status() != RecordStatus::Hypothesis
        })
        .collect();
    (positives.len() == 1).then(|| positives[0].id().get())
}
fn verify(store: &EngramStore, context: u32, x: NodeId, z: NodeId, answer: ResultKind) {
    match answer {
        ResultKind::Direct(i) => {
            let r = store.get(EngramId::new(i).unwrap()).unwrap();
            assert_eq!(
                (
                    r.cue().context(),
                    r.cue().source(),
                    r.cue().pattern(),
                    r.value()
                ),
                (context, x, z.get() as i32, 1)
            );
            assert_eq!(r.origin(), Some(x));
        }
        ResultKind::Inferred(a, b) => {
            let p = store.get(EngramId::new(a).unwrap()).unwrap();
            let q = store.get(EngramId::new(b).unwrap()).unwrap();
            assert_eq!((p.cue().context(), q.cue().context()), (context, context));
            assert_eq!(
                (
                    p.cue().source(),
                    p.cue().pattern() as u32,
                    q.cue().source().get(),
                    q.cue().pattern() as u32
                ),
                (x, q.cue().source().get(), p.cue().pattern() as u32, z.get())
            );
            assert_eq!((p.value(), q.value()), (1, 1));
            assert_eq!(
                (p.origin(), q.origin()),
                (Some(p.cue().source()), Some(q.cue().source()))
            );
            assert_ne!(p.status(), RecordStatus::Hypothesis);
            assert_ne!(q.status(), RecordStatus::Hypothesis);
        }
        _ => (),
    }
}
fn guard(start: Instant) {
    assert!(start.elapsed().as_secs_f64() < 30.0);
    let rss = std::fs::read_to_string("/proc/self/status").unwrap();
    let hwm: usize = rss
        .lines()
        .find(|l| l.starts_with("VmHWM:"))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    assert!(hwm < 256 * 1024);
    let out = std::process::Command::new("df")
        .args(["-B1", "--output=avail", "/home"])
        .output()
        .unwrap();
    assert!(out.status.success());
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
fn panel(offset: u32, n: usize, repetitions: usize) {
    let start = Instant::now();
    let fixtures = build(offset, n);
    assert_eq!(fixtures, build(offset, n), "generator nondeterminism");
    let mut first: Option<Vec<Row>> = None;
    for rep in 0..repetitions {
        let mut rows = Vec::new();
        let mut events = 0;
        let mut counts = [[0usize; 4]; 4];
        for (i, c) in fixtures.iter().enumerate() {
            let store = memory(c);
            let before: Vec<_> = store.iter().collect();
            let x = node(c.x);
            let z = node(c.z);
            let q = Hypothesis::new(c.context, x, z).unwrap();
            let task = TwoHopTask::new(8, 16, 32).unwrap();
            let decision = Monitor::new(task, 1)
                .unwrap()
                .review(&store, q, &[])
                .unwrap();
            let inference = decision.attempts[0].inference.unwrap();
            let event = inference.processed_events;
            let conflict = match decision.attempts[0].conflict {
                Some(Conflict::NegativeConclusion { negative }) => {
                    let r = store.get(negative).unwrap();
                    assert_eq!(
                        (
                            r.cue().context(),
                            r.cue().source(),
                            r.cue().pattern(),
                            r.value()
                        ),
                        (c.context, x, z.get() as i32, -1)
                    );
                    Some(negative.get())
                }
                None => None,
                other => panic!("unexpected conflict {other:?}"),
            };
            if let Outcome::Inferred(proof) = inference.outcome {
                assert_eq!(proof.via, node(c.y));
                assert!(proof.via_activations > 0);
                verify(
                    &store,
                    c.context,
                    x,
                    z,
                    ResultKind::Inferred(proof.first_fact.get(), proof.second_fact.get()),
                );
            }
            let model = match decision.decision {
                Decision::Respond {
                    evidence: EvidenceRefs::Direct(id),
                    ..
                } => ResultKind::Direct(id.get()),
                Decision::Respond {
                    evidence: EvidenceRefs::TwoHop { first, second },
                    ..
                } => ResultKind::Inferred(first.get(), second.get()),
                Decision::Abstain(AbstainReason::MissingEvidence) => ResultKind::Missing,
                Decision::Abstain(AbstainReason::ContradictoryEvidence) => ResultKind::Conflict,
                other => panic!("inconclusive {other:?}"),
            };
            let symbolic = symbolic(&store, c.context, x, z);
            let direct = literal(&store, c.context, x, z);
            let ablation = task.evaluate_without_synapses(&store, q).unwrap();
            let ablated = match ablation.outcome {
                Outcome::DirectRecall(id) => Some(id.get()),
                Outcome::Unsupported => None,
                other => panic!("ablation inconclusive {other:?}"),
            };
            assert_eq!(before, store.iter().collect::<Vec<_>>());
            // Evaluation oracle is consulted only after all decisions.
            let expected = match c.class {
                0 => ResultKind::Direct(1),
                1 => ResultKind::Inferred(1, 2),
                2 => ResultKind::Missing,
                _ => ResultKind::Conflict,
            };
            assert_eq!(conflict, if c.class == 3 { Some(3) } else { None });
            assert_eq!(model, expected, "case {i}");
            assert_eq!(symbolic, expected, "symbolic {i}");
            verify(&store, c.context, x, z, model);
            verify(&store, c.context, x, z, symbolic);
            if let ResultKind::Inferred(a, b) = model {
                let pa = store.get(EngramId::new(a).unwrap()).unwrap();
                assert_eq!(pa.cue().pattern() as u32, c.y);
                assert_eq!(b, 2);
                assert!(!c
                    .facts
                    .iter()
                    .any(|f| f.from == c.x && f.to == c.z && f.value == 1));
            }
            assert_eq!(direct, if c.class == 0 { Some(1) } else { None });
            assert_eq!(ablated, if c.class == 0 { Some(1) } else { None });
            assert_eq!(event, [0, 4, 2, 3][c.class as usize]);
            assert_eq!(ablation.processed_events, if c.class == 0 { 0 } else { 1 });
            events += event + ablation.processed_events;
            counts[c.class as usize][0] += 1;
            counts[c.class as usize][1] += 1;
            counts[c.class as usize][2] += usize::from(direct.is_some());
            counts[c.class as usize][3] += usize::from(ablated.is_some());
            rows.push(Row {
                case: i,
                context: c.context,
                facts: c.facts.clone(),
                conflict,
                model,
                symbolic,
                literal: direct,
                ablated,
                model_events: event,
                ablated_events: ablation.processed_events,
            });
            let estimate = fixtures.capacity() * std::mem::size_of::<Case>()
                + fixtures
                    .iter()
                    .map(|x| x.facts.capacity() * std::mem::size_of::<Fact>())
                    .sum::<usize>()
                + rows.capacity() * std::mem::size_of::<Row>()
                + before.capacity() * std::mem::size_of::<Engram>()
                + first
                    .as_ref()
                    .map_or(0, |v| v.capacity() * std::mem::size_of::<Row>())
                + 32768;
            assert!(estimate <= 256 * 1024);
        }
        assert_eq!(events, n / 4 * 12);
        for c in counts {
            assert_eq!(c[0], n / 4);
            assert_eq!(c[1], n / 4);
        }
        for row in &rows {
            println!("EC06_ROW rep={rep} {row:?}");
        }
        println!(
            "EC06 offset={offset} rep={rep} cases={n} queries={} events={events} counts={counts:?}",
            n * 4
        );
        if let Some(previous) = &first {
            assert_eq!(*previous, rows);
        }
        first = Some(rows);
        guard(start);
    }
    println!("EC06 seconds={:.6}", start.elapsed().as_secs_f64());
}
#[test]
fn development() {
    panel(10000, 8, 1)
}
#[test]
fn validation() {
    panel(20000, 8, 1)
}
#[test]
fn frozen_final() {
    panel(30000, 24, 2)
}
