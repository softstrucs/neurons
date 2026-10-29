//! EC-05 local diagnostic. Oracles remain in cases/evaluator, not reused executors.
#[path = "support/provenance_candidate.rs"]
mod candidate;
#[path = "support/ec05_cases.rs"]
mod cases;
#[path = "support/ec05_evidence.rs"]
mod evidence;
#[path = "support/provenance_literal.rs"]
mod literal;
#[path = "support/provenance_symbolic.rs"]
mod symbolic;
#[allow(dead_code)]
#[path = "support/provenance_types.rs"]
mod types;
use cases::{Cut, Step};
use std::{collections::BTreeMap, mem::size_of, time::Instant};
use types::*;
#[derive(Clone, Debug, Eq, PartialEq)]
struct Change {
    case: u8,
    op: Op,
    applied: bool,
    reads: usize,
    pairs: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Check {
    case: u8,
    point: u8,
    query: Query,
    requested_cut: usize,
    result: Result<Answer, Error>,
    reads: usize,
    pairs: usize,
    history: bool,
    cost_known: bool,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Trace {
    changes: Vec<Change>,
    checks: Vec<Check>,
    acquire_reads: usize,
    acquire_pairs: usize,
    query_reads: usize,
    query_pairs: usize,
    max_work: usize,
}
fn answer_bytes(a: &Answer) -> usize {
    a.causes.capacity() * size_of::<Decision>()
        + a.proofs.capacity() * size_of::<Proof>()
        + a.evidence.capacity() * size_of::<(Key, u32, Key)>()
        + a.conflicts.capacity() * size_of::<Key>()
}
impl Trace {
    fn bytes(&self) -> usize {
        size_of::<Self>()
            + self.changes.capacity() * size_of::<Change>()
            + self.checks.capacity() * size_of::<Check>()
            + self
                .checks
                .iter()
                .map(|r| r.result.as_ref().map_or(0, answer_bytes))
                .sum::<usize>()
    }
    fn work(&mut self, w: &Work, query: bool) {
        assert!(w.reads + w.pairs <= 8192, "STOP work");
        self.max_work = self.max_work.max(w.reads + w.pairs);
        if query {
            self.query_reads += w.reads;
            self.query_pairs += w.pairs;
        } else {
            self.acquire_reads += w.reads;
            self.acquire_pairs += w.pairs;
        }
    }
}
fn guard(start: Instant, bytes: usize) -> usize {
    assert!(start.elapsed().as_secs_f64() <= 30.0, "STOP time");
    assert!(bytes <= 256 * 1024, "STOP data {bytes}");
    let s = std::fs::read_to_string("/proc/self/status").unwrap();
    let rss: usize = s
        .lines()
        .find(|l| l.starts_with("VmRSS:"))
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
    bytes
}
// mode 0..2 = candidate/symbolic/literal; 3/4 are explicit information ablations.
fn sequence<L: Laboratory>(
    part: u32,
    mode: usize,
    start: Instant,
    other_bytes: usize,
) -> (Trace, usize) {
    let cases = cases::cases(part);
    let fixture_bytes = cases.capacity() * size_of::<cases::Case>()
        + cases
            .iter()
            .map(|c| c.steps.capacity() * size_of::<Step>())
            .sum::<usize>();
    let mut trace = Trace::default();
    let mut peak = 0;
    for (ci, case) in cases.iter().enumerate() {
        let mut model = L::new();
        let mut ledger = Vec::new();
        let mut saved: BTreeMap<u8, (usize, Answer)> = BTreeMap::new();
        let mut stable = None;
        let mut point = 0u8;
        for (si, step) in case.steps.iter().enumerate() {
            match step {
                Step::Apply(op) => {
                    assert!(ledger.len() < 128, "STOP operations");
                    let w = if mode == 3 {
                        Work::default()
                    } else {
                        model.apply(*op).expect("STOP operation")
                    };
                    trace.work(&w, false);
                    ledger.push(*op);
                    trace.changes.push(Change {
                        case: ci as u8,
                        op: *op,
                        applied: mode != 3,
                        reads: w.reads,
                        pairs: w.pairs,
                    });
                }
                Step::Probe {
                    query,
                    cut,
                    expected,
                    save,
                    stable: st,
                } => {
                    let requested = match cut {
                        Cut::Current => ledger.len(),
                        Cut::Empty => 0,
                        Cut::Saved(k) => saved[k].0,
                    };
                    let before = format!("{model:?}");
                    let (a, w) = if mode == 4 {
                        (
                            Answer {
                                decision: Decision::Unavailable,
                                causes: vec![Decision::Unavailable],
                                proofs: Vec::new(),
                                evidence: Vec::new(),
                                conflicts: Vec::new(),
                                cut: requested,
                                epoch: query.epoch,
                            },
                            Work::default(),
                        )
                    } else {
                        model
                            .query(*query, if mode == 3 { 0 } else { requested })
                            .expect("STOP query")
                    };
                    assert_eq!(before, format!("{model:?}"), "query mutation");
                    trace.work(&w, true);
                    assert_eq!(
                        a.decision,
                        if mode == 3 {
                            Decision::Unknown
                        } else if mode == 4 {
                            Decision::Unavailable
                        } else {
                            expected[mode]
                        },
                        "{name} point={point}",
                        name = case.name
                    );
                    if mode < 3 {
                        assert_eq!(a.cut, requested);
                        evidence::verify(&ledger, *query, &a);
                        if let Cut::Saved(k) = cut {
                            assert_eq!(&a, &saved[k].1, "historical evidence mismatch");
                        }
                    }
                    if *save != 0 {
                        saved.insert(*save, (requested, a.clone()));
                    }
                    if *st == 1 {
                        stable = Some(a.clone());
                    } else if *st == 2 {
                        let mut normalized = a.clone();
                        normalized.cut = stable.as_ref().unwrap().cut;
                        assert_eq!(
                            Some(&normalized),
                            stable.as_ref(),
                            "unaffected context changed"
                        );
                    }
                    println!("PROBE mode={mode} case={} point={point} cut={requested} epoch={} decision={:?} causes={:?} proofs={:?} evidence={:?} conflicts={:?}",case.name,query.epoch,a.decision,a.causes,a.proofs,a.evidence,a.conflicts);
                    trace.checks.push(Check {
                        case: ci as u8,
                        point,
                        query: *query,
                        requested_cut: requested,
                        result: Ok(a),
                        reads: w.reads,
                        pairs: w.pairs,
                        history: !matches!(cut, Cut::Current),
                        cost_known: true,
                    });
                    point += 1;
                }
                Step::InvalidCut(q) => {
                    if mode < 3 {
                        let before = format!("{model:?}");
                        let got = model.query(*q, model.len() + 1);
                        assert_eq!(got.as_ref().err(), Some(&Error::InvalidCut));
                        assert_eq!(before, format!("{model:?}"));
                        trace.checks.push(Check {
                            case: ci as u8,
                            point,
                            query: *q,
                            requested_cut: model.len() + 1,
                            result: Err(Error::InvalidCut),
                            reads: 0,
                            pairs: 0,
                            history: true,
                            cost_known: false,
                        });
                    }
                }
            }
            if si % 8 == 0 || si + 1 == case.steps.len() {
                let saved_bytes = saved
                    .values()
                    .map(|(_, a)| size_of::<Answer>() + answer_bytes(a) + 64)
                    .sum::<usize>()
                    + stable
                        .as_ref()
                        .map_or(0, |a| size_of::<Answer>() + answer_bytes(a));
                let bytes = other_bytes
                    + trace.bytes()
                    + fixture_bytes
                    + ledger.capacity() * size_of::<Op>()
                    + model.bytes()
                    + saved_bytes
                    + 16 * 1024;
                peak = peak.max(guard(start, bytes));
            }
        }
    }
    assert_eq!(trace.checks.len(), if mode < 3 { 52 } else { 51 });
    (trace, peak)
}
fn dispatch(part: u32, mode: usize, start: Instant, bytes: usize) -> (Trace, usize) {
    match mode {
        1 => sequence::<symbolic::Symbolic>(part, mode, start, bytes),
        2 => sequence::<literal::Literal>(part, mode, start, bytes),
        _ => sequence::<candidate::Candidate>(part, mode, start, bytes),
    }
}
fn compare(a: &Trace, b: &Trace, literal: bool) {
    assert_eq!(a.checks.len(), b.checks.len());
    let mut differences = 0;
    for (x, y) in a.checks.iter().zip(&b.checks) {
        assert_eq!(
            (x.case, x.point, x.query, x.requested_cut),
            (y.case, y.point, y.query, y.requested_cut)
        );
        if x.result != y.result {
            differences += 1;
            assert!(literal);
            assert!(
                matches!((x.case, x.point), (3, 5) | (5, 5) | (7, 2)),
                "unexpected control difference"
            );
            assert_eq!(y.result.as_ref().unwrap().decision, Decision::Supported);
        }
    }
    assert_eq!(differences, if literal { 3 } else { 0 });
}
fn run(part: u32, repetitions: usize) {
    let start = Instant::now();
    let mut first: Vec<Trace> = Vec::new();
    let mut peak = 0;
    let mut total = 0;
    for repetition in 0..repetitions {
        for mode in 0..5 {
            let bytes = first.iter().map(Trace::bytes).sum::<usize>();
            let (trace, p) = dispatch(part, mode, start, bytes);
            peak = peak.max(p);
            total += trace.checks.len();
            if repetition == 0 {
                if mode == 1 {
                    compare(&first[0], &trace, false);
                }
                if mode == 2 {
                    compare(&first[0], &trace, true);
                }
                println!("COST mode={mode} acquire_reads={} acquire_pairs={} query_reads={} query_pairs={} max_work={} trace_bytes={}",trace.acquire_reads,trace.acquire_pairs,trace.query_reads,trace.query_pairs,trace.max_work,trace.bytes());
                first.push(trace);
            } else {
                assert_eq!(first[mode], trace, "nondeterministic sequence");
            }
        }
    }
    assert_eq!(total, 258 * repetitions);
    println!("PANEL part={part} repetitions={repetitions} checks={total} queries={} invalid_cut={} circuit_events=0 peak_accounted_bytes={peak} seconds={:.6}",255*repetitions,3*repetitions,start.elapsed().as_secs_f64());
}
#[test]
fn development() {
    run(100000, 1)
}
#[test]
fn validation() {
    run(200000, 1)
}
#[test]
fn frozen_final() {
    run(300000, 2)
}
#[test]
fn manifest_preflight() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    for part in [100000, 200000, 300000] {
        let rows = cases::cases(part);
        assert_eq!(rows, cases::cases(part), "manifest nondeterministic");
        for (case, want) in rows.iter().zip([5, 4, 6, 7, 4, 10, 7, 8]) {
            assert_eq!(
                case.steps
                    .iter()
                    .filter(|s| matches!(s, Step::Probe { .. }))
                    .count(),
                want
            );
            assert!(
                case.steps
                    .iter()
                    .filter(|s| matches!(s, Step::Apply(_)))
                    .count()
                    <= 128
            );
            assert!(
                case.steps
                    .iter()
                    .filter(|s| matches!(s, Step::Apply(Op::Source(_))))
                    .count()
                    <= 8
            );
            assert!(
                case.steps
                    .iter()
                    .filter(|s| matches!(s, Step::Apply(Op::Rule(_))))
                    .count()
                    <= 4
            );
        }

        let queries = rows
            .iter()
            .flat_map(|c| &c.steps)
            .filter(|s| matches!(s, Step::Probe { .. }))
            .count();
        assert_eq!(queries, 51);
        let mut differences = 0;
        for c in &rows {
            for s in &c.steps {
                if let Step::Probe { expected, .. } = s {
                    if expected[0] != expected[2] {
                        differences += 1;
                    }
                }
            }
        }
        assert_eq!(differences, 3);
        let canonical = format!("{rows:?}");
        let mut child = Command::new("sha256sum")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(canonical.as_bytes())
            .unwrap();
        let r = child.wait_with_output().unwrap();
        assert!(r.status.success());
        println!(
            "MANIFEST part={part} bytes={} sha256={}",
            canonical.len(),
            std::str::from_utf8(&r.stdout)
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
        );
    }
}
