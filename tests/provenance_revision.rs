//! EC-03.4 isolated experiment. Expected outcomes live only in this evaluator.
#[path = "support/provenance_candidate.rs"]
mod candidate;
#[path = "support/provenance_literal.rs"]
mod literal;
#[path = "support/provenance_symbolic.rs"]
mod symbolic;
#[path = "support/provenance_types.rs"]
mod types;
use std::{mem::size_of, time::Instant};
use types::*;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Check {
    scenario: u32,
    phase: u8,
    kind: u8,
    result: Result<Answer, Error>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Mutation {
    scenario: u32,
    sequence: usize,
    epoch: u32,
    actor: u32,
    op: Op,
    result: Result<(), Error>,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Run {
    changes: Vec<Mutation>,
    checks: Vec<Check>,
    ablations: Vec<Answer>,
    reads: usize,
    pairs: usize,
    max_work: usize,
    peak_data: usize,
}
impl Run {
    fn bytes(&self) -> usize {
        size_of::<Self>()
            + self.ablations.capacity() * size_of::<Answer>()
            + self.changes.capacity() * size_of::<Mutation>()
            + self.checks.capacity() * size_of::<Check>()
            + self
                .checks
                .iter()
                .map(|c| {
                    c.result.as_ref().map_or(0, |a| {
                        a.causes.capacity() * size_of::<Decision>()
                            + a.proofs.capacity() * size_of::<Proof>()
                            + a.evidence.capacity() * size_of::<(Key, u32, Key)>()
                            + a.conflicts.capacity() * size_of::<Key>()
                    })
                })
                .sum::<usize>()
    }
    fn work(&mut self, w: Work) {
        let n = w.reads + w.pairs;
        assert!(n <= 8192);
        self.max_work = self.max_work.max(n);
        self.reads += w.reads;
        self.pairs += w.pairs;
    }
}
struct Harness<L: Laboratory> {
    lab: L,
    run: Run,
    scenario: u32,
    epoch: u32,
    extra: usize,
    started: Instant,
    ablate: bool,
}
impl<L: Laboratory> Harness<L> {
    fn guard(&mut self) {
        assert!(self.started.elapsed().as_secs_f64() <= 30.0, "STOP time");
        let text = std::fs::read_to_string("/proc/self/status").unwrap();
        let rss: usize = text
            .lines()
            .find(|l| l.starts_with("VmRSS:"))
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        assert!(rss < 256 * 1024, "STOP RSS");
        let df = std::process::Command::new("df")
            .args(["-B1", "--output=avail", "/home"])
            .output()
            .unwrap();
        assert!(df.status.success());
        let free: u64 = std::str::from_utf8(&df.stdout)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        assert!(free >= 3 * 1024 * 1024 * 1024, "STOP disk");
        // Reserve covers simultaneous candidate clone, queries and temporary vectors/maps.
        let data = self.lab.bytes() + self.run.bytes() + self.extra + 16 * 1024;
        self.run.peak_data = self.run.peak_data.max(data);
        assert!(data <= 256 * 1024, "STOP data bytes={data}");
    }
    fn op(&mut self, op: Op) {
        let w = self.lab.apply(op).expect("STOP operation rejected");
        self.run.work(w);
        if let Op::Epoch(e) = op {
            self.epoch = e;
        }
        self.run.changes.push(Mutation {
            scenario: self.scenario,
            sequence: self.lab.len(),
            epoch: self.epoch,
            actor: 1,
            op,
            result: Ok(()),
        });
    }
    fn reject(&mut self, op: Op, expected: Error) {
        let before = self.lab.clone();
        let got = self.lab.apply(op).map(|_| ());
        assert_eq!(got, Err(expected));
        assert_eq!(self.lab, before, "Rejected operation mutated state");
        self.run.changes.push(Mutation {
            scenario: self.scenario,
            sequence: self.lab.len(),
            epoch: self.epoch,
            actor: 1,
            op,
            result: got,
        });
        self.run.checks.push(Check {
            scenario: self.scenario,
            phase: 0,
            kind: 3,
            result: Err(expected),
        });
    }
    fn query(&mut self, q: Query, cut: usize, phase: u8, kind: u8, expected: Decision) -> Answer {
        let before = self.lab.clone();
        let (a, w) = self.lab.query(q, cut).expect("STOP query error");
        assert_eq!(self.lab, before, "Evaluation mutated learner");
        assert_eq!(
            a.decision, expected,
            "scenario={} phase={phase} kind={kind}",
            self.scenario
        );
        self.run.work(w);
        if kind == 0 && self.ablate {
            let empty = candidate::Candidate::new();
            let (no_acquisition, _) = empty.query(q, 0).unwrap();
            assert_eq!(no_acquisition.decision, Decision::Unknown);
            let inaccessible = access_query(&self.lab, q, cut, false).unwrap().0;
            assert_eq!(inaccessible.decision, Decision::Unavailable);
            assert!(inaccessible.proofs.is_empty() && inaccessible.evidence.is_empty());
            assert_eq!(self.lab, before);
            self.run.ablations.extend([no_acquisition, inaccessible]);
        }
        self.run.checks.push(Check {
            scenario: self.scenario,
            phase,
            kind,
            result: Ok(a.clone()),
        });
        a
    }
}
// Explicit information-ablation boundary; unavailable branch never reads the model.
fn access_query<L: Laboratory>(
    lab: &L,
    q: Query,
    cut: usize,
    available: bool,
) -> Result<(Answer, Work), Error> {
    if available {
        return lab.query(q, cut);
    }
    Ok((
        Answer {
            decision: Decision::Unavailable,
            causes: Vec::new(),
            proofs: Vec::new(),
            evidence: Vec::new(),
            conflicts: Vec::new(),
            cut,
            epoch: q.epoch,
        },
        Work::default(),
    ))
}
fn fact(
    i: u32,
    role: u32,
    offset: u32,
    ver: u16,
    ctx: u32,
    from: u32,
    to: u32,
    positive: bool,
    start: u32,
    end: u32,
    epoch: u32,
) -> Fact {
    Fact {
        key: Key(3000 + 17 * i + role + offset, ver),
        observation: 2000 + 17 * i + role + offset + 10000 * u32::from(ver - 1),
        source: Key(1000 + 17 * i + role + offset, 1),
        context: ctx,
        from,
        to,
        positive,
        hypothesis: false,
        observed: epoch,
        start,
        end,
    }
}
fn panel<L: Laboratory>(
    ids: &[u32],
    offset: u32,
    negatives: &[u32],
    is_literal: bool,
    extra: usize,
    started: Instant,
) -> Run {
    let mut h = Harness {
        lab: L::new(),
        run: Run::default(),
        scenario: 0,
        epoch: 1,
        extra,
        started,
        ablate: offset == 0
            && std::any::type_name::<L>() == std::any::type_name::<candidate::Candidate>(),
    };
    for &i in ids {
        h.lab = L::new();
        h.scenario = i;
        h.epoch = 1;
        h.guard();
        let (x, y, z, w, c, d) = (
            11 + offset,
            23 + offset,
            37 + offset,
            41 + offset,
            53 + offset,
            67 + offset,
        );
        let q = Query {
            from: x,
            to: z,
            context: if i == 6 { d } else { c },
            epoch: 1,
        };
        h.query(q, 0, 0, 0, Decision::Unknown);
        let roles: Vec<u32> = if i == 8 { vec![1, 2, 4, 5] } else { vec![1, 2] };
        for role in &roles {
            h.op(Op::Source(Key(1000 + 17 * i + role + offset, 1)));
        }
        let f1 = fact(
            i,
            1,
            offset,
            1,
            c,
            x,
            y,
            true,
            1,
            if i == 5 { 3 } else { 10 },
            1,
        );
        let f2 = fact(i, 2, offset, 1, c, y, z, true, 1, 10, 1);
        h.op(Op::Fact(f1));
        h.op(Op::Fact(f2));
        if i == 8 {
            h.op(Op::Fact(fact(i, 4, offset, 1, c, x, w, true, 1, 10, 1)));
            h.op(Op::Fact(fact(i, 5, offset, 1, c, w, z, true, 1, 10, 1)));
        }
        let rk = Key(4000 + 17 * i + offset, 1);
        h.op(Op::Rule(Rule {
            key: rk,
            context: c,
            start: 1,
            end: 10,
        }));
        h.op(Op::Refresh);
        let p1 = h.lab.len();
        let old = h.query(
            q,
            p1,
            1,
            0,
            if i == 6 {
                Decision::Unknown
            } else {
                Decision::Supported
            },
        );
        if i == 8 {
            assert_eq!(old.proofs.len(), 2);
        }
        match i {
            1 => {
                let f = fact(i, 1, offset, 2, c, x, y, true, 1, 10, 1);
                h.op(Op::Revise(f));
                let a = h.query(
                    q,
                    h.lab.len(),
                    2,
                    2,
                    if is_literal {
                        Decision::Supported
                    } else {
                        Decision::Stale
                    },
                );
                if is_literal {
                    assert!(a.evidence.iter().any(|e| e.0 == f.key));
                }
            }
            2 | 8 => h.op(Op::Set(Kind::Fact, f1.key, Status::Retracted)),
            3 => {
                h.op(Op::Source(Key(1000 + 17 * i + 3 + offset, 1)));
                h.op(Op::Fact(fact(i, 3, offset, 1, c, x, y, false, 1, 10, 1)));
            }
            4 => h.op(Op::Set(Kind::Source, f1.source, Status::Suspended)),
            5 => h.op(Op::Epoch(3)),
            6 => {
                for role in [1, 2] {
                    h.op(Op::Source(Key(1000 + 17 * i + role + offset + 5000, 1)));
                }
                h.op(Op::Fact(fact(
                    i,
                    1,
                    offset + 5000,
                    1,
                    d,
                    x,
                    y,
                    true,
                    1,
                    10,
                    1,
                )));
                h.op(Op::Fact(fact(
                    i,
                    2,
                    offset + 5000,
                    1,
                    d,
                    y,
                    z,
                    true,
                    1,
                    10,
                    1,
                )));
                h.op(Op::Rule(Rule {
                    key: Key(rk.0 + 5000, 1),
                    context: d,
                    start: 1,
                    end: 10,
                }));
            }
            7 => h.op(Op::Set(Kind::Rule, rk, Status::Suspended)),
            _ => unreachable!(),
        }
        h.op(Op::Refresh);
        let q2 = Query {
            epoch: h.epoch,
            ..q
        };
        let expected = [
            Decision::Supported,
            Decision::Retracted,
            Decision::Conflict,
            Decision::Suspended,
            Decision::Expired,
            Decision::Supported,
            Decision::Suspended,
            Decision::Supported,
        ][i as usize - 1];
        let a = h.query(q2, h.lab.len(), 2, 0, expected);
        if i == 8 {
            assert_eq!(a.proofs.len(), 1);
            assert!(a.proofs.iter().all(|p| p.first != f1.key));
        }
        let historic = h.query(q, p1, 2, 1, old.decision);
        assert_eq!(historic, old);
        match i {
            2 => h.op(Op::Fact(fact(i, 1, offset, 2, c, x, y, true, 1, 10, 1))),
            3 => h.op(Op::Set(
                Kind::Fact,
                Key(3000 + 17 * i + 3 + offset, 1),
                Status::Retracted,
            )),
            4 => h.op(Op::Set(Kind::Source, f1.source, Status::Active)),
            5 => {
                let f = fact(i, 1, offset, 2, c, x, y, true, 3, 10, 3);
                h.op(Op::Revise(f));
                let a = h.query(
                    q2,
                    h.lab.len(),
                    3,
                    2,
                    if is_literal {
                        Decision::Supported
                    } else {
                        Decision::Expired
                    },
                );
                if is_literal {
                    assert!(a.evidence.iter().any(|e| e.0 == f.key));
                }
            }
            7 => h.op(Op::Set(Kind::Rule, rk, Status::Active)),
            _ => {}
        }
        if matches!(i, 2 | 3 | 4 | 5 | 7) {
            h.op(Op::Refresh);
        }
        h.query(q2, h.lab.len(), 3, 0, Decision::Supported);
        let historic = h.query(q, p1, 3, 1, old.decision);
        assert_eq!(historic, old);
        h.guard();
    }
    for &n in negatives {
        h.lab = L::new();
        h.scenario = 100 + n;
        h.epoch = 1;
        h.guard();
        let f = Fact {
            key: Key(9003 + offset, 1),
            observation: 9002 + offset,
            source: Key(9001 + offset, 1),
            context: 53 + offset,
            from: 11 + offset,
            to: 23 + offset,
            positive: true,
            hypothesis: false,
            observed: 1,
            start: 1,
            end: 10,
        };
        let rule = Rule {
            key: Key(9004 + offset, 1),
            context: f.context,
            start: 1,
            end: 10,
        };
        let q = Query {
            from: f.from,
            to: 37 + offset,
            context: f.context,
            epoch: 1,
        };
        match n {
            1 => h.reject(Op::Fact(f), Error::MissingSource),
            2 => {
                h.op(Op::Source(f.source));
                h.op(Op::Fact(f));
                h.reject(Op::Fact(f), Error::Duplicate);
            }
            3 => {
                h.op(Op::Source(f.source));
                h.reject(
                    Op::Fact(Fact {
                        start: 3,
                        end: 3,
                        ..f
                    }),
                    Error::InvalidInterval,
                );
            }
            4 => {
                h.op(Op::Rule(rule));
                h.reject(
                    Op::Associate(
                        [Proposal {
                            id: f.key.0,
                            deps: [9999 + offset; 2],
                        }; 2],
                        1,
                    ),
                    Error::MissingReference,
                );
            }
            5 => {
                h.op(Op::Rule(rule));
                h.reject(
                    Op::Associate(
                        [Proposal {
                            id: f.key.0,
                            deps: [f.key.0; 2],
                        }; 2],
                        1,
                    ),
                    Error::Cycle,
                );
            }
            6 => {
                h.op(Op::Rule(rule));
                h.reject(
                    Op::Associate(
                        [
                            Proposal {
                                id: f.key.0,
                                deps: [9005 + offset; 2],
                            },
                            Proposal {
                                id: 9005 + offset,
                                deps: [f.key.0; 2],
                            },
                        ],
                        2,
                    ),
                    Error::Cycle,
                );
            }
            7 => {
                h.op(Op::Source(f.source));
                h.op(Op::Fact(Fact {
                    hypothesis: true,
                    to: q.to,
                    ..f
                }));
                h.query(q, h.lab.len(), 0, 3, Decision::Unknown);
            }
            8 => {
                h.op(Op::Source(f.source));
                h.op(Op::Set(Kind::Source, f.source, Status::Retracted));
                h.reject(
                    Op::Set(Kind::Source, f.source, Status::Active),
                    Error::TerminalRetraction,
                );
            }
            9 => {
                h.query(q, 0, 0, 3, Decision::Unknown);
            }
            10 => {
                let before = h.lab.clone();
                let result = h.lab.query(q, 1);
                assert_eq!(result, Err(Error::InvalidCut));
                assert_eq!(before, h.lab);
                h.run.checks.push(Check {
                    scenario: h.scenario,
                    phase: 0,
                    kind: 3,
                    result: Err(Error::InvalidCut),
                });
            }
            11 => {
                h.op(Op::Source(Key(61001 + offset, 1)));
                for i in 1..=64 {
                    h.op(Op::Fact(Fact {
                        key: Key(60000 + i + offset, 1),
                        observation: 62000 + i + offset,
                        source: Key(61001 + offset, 1),
                        from: 70001 + i + offset,
                        to: 80001 + i + offset,
                        ..f
                    }));
                }
                h.reject(
                    Op::Fact(Fact {
                        key: Key(60065 + offset, 1),
                        observation: 62065 + offset,
                        source: Key(61001 + offset, 1),
                        from: 70066 + offset,
                        to: 80066 + offset,
                        ..f
                    }),
                    Error::Capacity,
                );
            }
            12 => h.reject(Op::Schema(2), Error::UnsupportedVersion),
            _ => unreachable!(),
        }
        h.guard();
    }
    h.run
}
fn same_content(a: &Run, b: &Run) {
    assert_eq!(a.changes, b.changes);
    assert_eq!(a.checks, b.checks);
    assert_eq!(a.ablations, b.ablations);
    assert_eq!(a.reads, b.reads);
    assert_eq!(a.pairs, b.pairs);
    assert_eq!(a.max_work, b.max_work);
}
fn development() {
    let t = Instant::now();
    panel::<candidate::Candidate>(&[1, 3], 20000, &[1, 2, 3], false, 0, t);
    panel::<symbolic::Symbolic>(&[1, 3], 20000, &[1, 2, 3], false, 0, t);
    panel::<literal::Literal>(&[1, 3], 20000, &[1, 2, 3], true, 0, t);
}
#[test]
fn development_partition() {
    development();
}
#[test]
fn validation_partition() {
    let t = Instant::now();
    panel::<candidate::Candidate>(&[2, 4], 40000, &[], false, 0, t);
    panel::<symbolic::Symbolic>(&[2, 4], 40000, &[], false, 0, t);
    panel::<literal::Literal>(&[2, 4], 40000, &[], true, 0, t);
}
#[test]
fn frozen_final_panel() {
    let t = Instant::now();
    let ids: Vec<_> = (1..=8).collect();
    let ns: Vec<_> = (1..=12).collect();
    let a = panel::<candidate::Candidate>(&ids, 0, &ns, false, 0, t);
    let aa = panel::<candidate::Candidate>(&ids, 0, &ns, false, a.bytes(), t);
    same_content(&a, &aa);
    println!(
        "candidate checks={} reads={} pair_tests={} max_work={} peak_accounted_bytes={}",
        a.checks.len(),
        a.reads,
        a.pairs,
        a.max_work,
        aa.peak_data.max(a.peak_data)
    );
    drop(aa);
    let b = panel::<symbolic::Symbolic>(&ids, 0, &ns, false, a.bytes(), t);
    {
        let bb = panel::<symbolic::Symbolic>(&ids, 0, &ns, false, a.bytes() + b.bytes(), t);
        same_content(&b, &bb);
        println!(
            "symbolic checks={} reads={} pair_tests={} max_work={} peak_accounted_bytes={}",
            b.checks.len(),
            b.reads,
            b.pairs,
            b.max_work,
            bb.peak_data.max(b.peak_data)
        );
    }
    assert_eq!(a.checks, b.checks);
    assert_eq!(a.changes, b.changes);
    drop(b);
    let c = panel::<literal::Literal>(&ids, 0, &ns, true, a.bytes(), t);
    {
        let cc = panel::<literal::Literal>(&ids, 0, &ns, true, a.bytes() + c.bytes(), t);
        same_content(&c, &cc);
        println!(
            "literal checks={} reads={} pair_tests={} max_work={} peak_accounted_bytes={}",
            c.checks.len(),
            c.reads,
            c.pairs,
            c.max_work,
            cc.peak_data.max(c.peak_data)
        );
    }
    assert_eq!(a.changes, c.changes);
    for (x, y) in a.checks.iter().zip(&c.checks) {
        if x.kind == 2 {
            assert_eq!(y.result.as_ref().unwrap().decision, Decision::Supported);
        } else {
            assert_eq!(x, y);
        }
    }
    let mut phase = [(0, 0); 4];
    for check in &a.checks {
        if check.kind == 0 {
            let r = check.result.as_ref().unwrap();
            phase[check.phase as usize].1 += 1;
            if r.decision == Decision::Supported {
                phase[check.phase as usize].0 += 1;
            }
        }
        println!("CANDIDATE {check:?}");
    }
    assert_eq!(phase, [(0, 8), (7, 8), (3, 8), (8, 8)]);
    assert_eq!(a.checks.len(), 62);
    assert_eq!(c.checks.len(), 62);
    for x in c.checks.iter().filter(|x| x.kind == 2) {
        println!("LITERAL_PROBE {x:?}");
    }
    assert_eq!(a.ablations.len(), 64);
    assert_eq!(a.checks.len() * 6 + a.ablations.len() * 2, 500);
    assert!(t.elapsed().as_secs_f64() <= 30.0);
    println!("EC-03.4 checks=500 main_per_condition=32 history=16 probes=2 negatives=12 repetitions=2 main_supported=18 main_abstentions=14 candidate_symbolic_equal=62 literal_equal=60 literal_different_probes=2 circuit_events=0 elapsed_seconds={:.6}",t.elapsed().as_secs_f64());
}
