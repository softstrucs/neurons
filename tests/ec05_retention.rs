#[path = "support/retention_candidate.rs"]
mod retention_candidate;
#[path = "support/retention_cases.rs"]
mod retention_cases;
#[path = "support/retention_symbolic.rs"]
mod retention_symbolic;
#[path = "support/retention_types.rs"]
mod retention_types;
use retention_cases::{Case, Step};
use retention_types::*;
use std::time::Instant;
#[derive(Clone, Debug, Eq, PartialEq)]
struct Record {
    case: u8,
    step: usize,
    result: Result<(Answer, usize), Error>,
}
#[derive(Default)]
struct Counts {
    queries: usize,
    errors: usize,
    history: usize,
    supported: usize,
    max_work: usize,
    peak_bytes: usize,
}
fn guard(start: Instant) {
    assert!(start.elapsed().as_secs_f64() < 30.0, "panel time limit");
    let rss = std::fs::read_to_string("/proc/self/status").unwrap();
    let kib: usize = rss
        .lines()
        .find(|s| s.starts_with("VmHWM:"))
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    assert!(kib < 256 * 1024, "RSS limit");
    let df = std::process::Command::new("df")
        .args(["-B1", "--output=avail", "/home"])
        .output()
        .unwrap();
    assert!(df.status.success());
    let free: u64 = String::from_utf8(df.stdout)
        .unwrap()
        .lines()
        .last()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert!(free >= 3 * 1024 * 1024 * 1024, "disk limit");
}
fn answer_bytes(a: &Answer) -> usize {
    std::mem::size_of::<Answer>()
        + a.causes.capacity() * std::mem::size_of::<Decision>()
        + a.conflicts.capacity() * std::mem::size_of::<Key>()
        + a.proofs.capacity() * std::mem::size_of::<Proof>()
        + a.proofs
            .iter()
            .map(|p| p.dependencies.capacity() * std::mem::size_of::<Dependency>())
            .sum::<usize>()
}
fn record_bytes(r: &[Record]) -> usize {
    std::mem::size_of_val(r)
        + r.iter()
            .map(|r| r.result.as_ref().map(|(a, _)| answer_bytes(a)).unwrap_or(0))
            .sum::<usize>()
}
// Evaluator only: reconstruct exact dependencies from permitted prefix, not from expected answers.
fn verify(a: &Answer, log: &[Op]) {
    use std::collections::BTreeMap;
    let mut facts = BTreeMap::new();
    let mut rules = BTreeMap::new();
    let mut sources = BTreeMap::new();
    let mut states = BTreeMap::new();
    let mut grants = Vec::new();
    for op in &log[..a.cut] {
        match *op {
            Op::Fact(f) => {
                facts.insert(f.key, f);
            }
            Op::Rule(r) => {
                rules.insert(r.key, r);
            }
            Op::Source(id, p) => {
                sources.insert(id, p);
            }
            Op::Set(k, id, s) => {
                states.insert((k, id), s);
            }
            Op::Grant(g) => grants.push(g),
            _ => (),
        }
    }
    assert_eq!(a.decision == Decision::Supported, !a.proofs.is_empty());
    let active = |k, id| states.get(&(k, id)).copied().unwrap_or(Status::Active) == Status::Active;
    let applicable = |kind: Kind, key: Key, born: Context, target: Context| -> Vec<Dependency> {
        assert_eq!(born.0, target.0);
        assert!(born.1 <= target.1);
        let mut out = Vec::new();
        for v in born.1..target.1 {
            let g = grants
                .iter()
                .find(|g| {
                    g.kind == kind
                        && g.key == key
                        && g.from == Context(born.0, v)
                        && g.to == Context(born.0, v + 1)
                })
                .expect("missing continuity declaration");
            out.push(Dependency::Declaration(g.declaration));
        }
        out
    };
    for p in &a.proofs {
        let f = facts[&p.first];
        let g = facts[&p.second];
        let r = rules[&p.rule];
        assert_eq!(p.context, a.query.context);
        assert_eq!(
            (f.from, f.to, g.from, g.to),
            (a.query.from, g.from, f.to, a.query.to)
        );
        assert!(f.from != g.to && f.from != f.to && g.from != g.to);
        let mut deps = vec![Dependency::Context(p.context), Dependency::Rule(r.key)];
        assert!(active(Kind::Rule, r.key));
        assert!((r.start..r.end).contains(&a.query.epoch));
        deps.extend(applicable(Kind::Rule, r.key, r.context, p.context));
        for fact in [f, g] {
            assert!(fact.positive && !fact.hypothesis);
            assert!(active(Kind::Fact, fact.key));
            assert!(!facts.keys().any(|k| k.0 == fact.key.0 && k.1 > fact.key.1));
            assert!((fact.start..fact.end).contains(&a.query.epoch));
            deps.extend([
                Dependency::Fact(fact.key),
                Dependency::Observation(fact.observation),
            ]);
            deps.extend(applicable(Kind::Fact, fact.key, fact.context, p.context));
            let mut source = fact.source;
            for depth in 0..8 {
                assert!(active(Kind::Source, Key(source, 1)));
                deps.push(Dependency::Source(source));
                let parent = sources[&source].expect("unknown origin used as proof");
                if parent == source {
                    break;
                }
                assert!(depth < 7);
                source = parent;
            }
        }
        deps.sort();
        deps.dedup();
        assert_eq!(p.dependencies, deps);
    }
}
fn execute<E: Executor>(
    cases: &[Case],
    condition: usize,
    start: Instant,
    retained: usize,
) -> (Vec<Record>, Counts) {
    let mut records = Vec::new();
    let mut count = Counts::default();
    let fixture_bytes = std::mem::size_of_val(cases)
        + cases
            .iter()
            .map(|c| c.steps.capacity() * std::mem::size_of::<Step>())
            .sum::<usize>();
    for case in cases {
        guard(start);
        let mut model = E::new();
        let mut saved = Vec::<Answer>::new();
        let mut log = Vec::new();
        let mut tally = [0usize; 8];
        for (idx, step) in case.steps.iter().enumerate() {
            let before = model.clone();
            let result = match *step {
                Step::Apply(op) => {
                    let w = model.apply(op).expect("frozen acquisition rejected");
                    count.max_work = count.max_work.max(w);
                    log.push(op);
                    continue;
                }
                Step::Ask(q, p1, p2) => {
                    let result = model.query(q, model.cut());
                    let (answer, work) = result.clone().expect("query error");
                    assert_eq!(
                        answer.decision,
                        if condition == 0 { p1 } else { p2 },
                        "case {} step {}",
                        case.id,
                        idx
                    );
                    count.max_work = count.max_work.max(work);
                    saved.push(answer.clone());
                    Ok((answer, work))
                }
                Step::History(n) => {
                    let original = &saved[n];
                    let result = model.query(original.query, original.cut);
                    assert_eq!(result.as_ref().unwrap().0, *original);
                    count.history += 1;
                    result
                }
                Step::Reject(op, error) => {
                    assert_eq!(model.apply(op), Err(error));
                    Err(error)
                }
                Step::BadCut(q) => {
                    let result = model.query(q, model.cut() + 1);
                    assert_eq!(result, Err(Error::InvalidCut));
                    result
                }
            };
            assert_eq!(model, before, "read/rejection mutated state");
            if let Ok((answer, work)) = &result {
                verify(answer, &log);
                count.max_work = count.max_work.max(*work);
                assert!(*work <= 8192);
                count.queries += 1;
                tally[answer.decision as usize] += 1;
                count.supported += usize::from(answer.decision == Decision::Supported);
            } else {
                count.errors += 1;
            }
            records.push(Record {
                case: case.id,
                step: idx,
                result,
            });
            // Real Vec capacities + simultaneous model clone + saved answers + scratch reserve.
            let bytes = fixture_bytes
                + records.capacity() * std::mem::size_of::<Record>()
                + records
                    .iter()
                    .map(|r| r.result.as_ref().map(|(a, _)| answer_bytes(a)).unwrap_or(0))
                    .sum::<usize>()
                + model.bytes() * 2
                + log.capacity() * std::mem::size_of::<Op>()
                + saved.capacity() * std::mem::size_of::<Answer>()
                + saved.iter().map(answer_bytes).sum::<usize>()
                + retained
                + 32768;
            count.peak_bytes = count.peak_bytes.max(bytes);
            assert!(bytes <= 256 * 1024, "data budget {}", bytes);
        }
        println!(
            "CASE condition={} id={} decisions={:?}",
            condition, case.id, tally
        );
    }
    (records, count)
}
fn run(selected: &[u8], offset: u32, repetitions: usize, expected: (usize, usize, usize)) {
    let start = Instant::now();
    let fixtures = retention_cases::cases(offset, selected);
    let mut previous: Option<(Vec<Record>, Vec<Record>)> = None;
    for rep in 0..repetitions {
        let retained = previous
            .as_ref()
            .map(|(a, b)| record_bytes(a) + record_bytes(b))
            .unwrap_or(0);
        let (one, a) = execute::<retention_candidate::Candidate>(&fixtures, 0, start, retained);
        let (two, b) = execute::<retention_symbolic::Symbolic>(
            &fixtures,
            1,
            start,
            retained + record_bytes(&one),
        );
        assert_eq!((a.queries, a.errors, a.history), expected);
        assert_eq!((b.queries, b.errors, b.history), expected);
        assert_eq!(one.len(), two.len());
        let mut differences = 0;
        let mut equal_proofs = 0;
        for (x, y) in one.iter().zip(&two) {
            assert_eq!((x.case, x.step), (y.case, y.step));
            match (&x.result, &y.result) {
                (Ok((a, _)), Ok((b, _))) => {
                    if a.decision != b.decision {
                        differences += 1;
                    }
                    if a.decision == Decision::Supported && b.decision == Decision::Supported {
                        assert_eq!(a.proofs, b.proofs);
                        equal_proofs += 1;
                    }
                }
                (Err(a), Err(b)) => assert_eq!(a, b),
                _ => panic!("error disagreement"),
            }
        }
        if offset == 30000 {
            assert_eq!(differences, 5);
        }
        println!("SUMMARY repetition={} queries_per_executor={} errors={} history={} supported_p1={} supported_p2={} differences={} equal_supported_proofs={} max_work={} peak_accounted_bytes={}",rep,a.queries,a.errors,a.history,a.supported,b.supported,differences,equal_proofs,a.max_work.max(b.max_work),a.peak_bytes.max(b.peak_bytes));
        if let Some((old_a, old_b)) = &previous {
            assert_eq!(*old_a, one);
            assert_eq!(*old_b, two);
        }
        previous = Some((one, two));
    }
    guard(start);
    println!("PANEL seconds={:.6}", start.elapsed().as_secs_f64());
}
#[test]
fn development() {
    run(&retention_cases::DEV, 10000, 1, (17, 4, 3));
}
#[test]
fn validation() {
    run(&retention_cases::VALID, 20000, 1, (27, 0, 7));
}
#[test]
fn manifest_preflight() {
    for offset in [10000, 20000, 30000] {
        let data = retention_cases::cases(offset, &retention_cases::ALL);
        assert_eq!(data, retention_cases::cases(offset, &retention_cases::ALL));
        let q = data
            .iter()
            .flat_map(|x| &x.steps)
            .filter(|s| matches!(s, Step::Ask(..) | Step::History(_)))
            .count();
        let h = data
            .iter()
            .flat_map(|x| &x.steps)
            .filter(|s| matches!(s, Step::History(_)))
            .count();
        assert_eq!((q, h), (63, 15));
        for c in &data {
            let ops = c
                .steps
                .iter()
                .filter(|s| matches!(s, Step::Apply(_)))
                .count();
            assert!(ops <= 128);
        }
        // Canonical debug bytes are an audit manifest, not knowledge input.
        let bytes = format!("{:?}", data);
        use std::io::Write;
        let mut child = std::process::Command::new("sha256sum")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(bytes.as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success());
        println!(
            "MANIFEST offset={} bytes={} sha256={}",
            offset,
            bytes.len(),
            String::from_utf8(out.stdout).unwrap().trim()
        );
    }
}
#[test]
fn frozen_final_panel() {
    run(&retention_cases::ALL, 30000, 2, (63, 5, 15));
}
