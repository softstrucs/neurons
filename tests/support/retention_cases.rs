//! Fixtures and evaluator expectations. Never imported by either mechanism.
use super::retention_types::*;
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Step {
    Apply(Op),
    Ask(Query, Decision, Decision),
    History(usize),
    Reject(Op, Error),
    BadCut(Query),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Case {
    pub id: u8,
    pub steps: Vec<Step>,
}
fn fact(id: u32, a: u32, b: u32, source: u32, obs: u32, c: Context) -> Fact {
    Fact {
        key: Key(id, 1),
        context: c,
        from: a,
        to: b,
        source,
        observation: obs,
        positive: true,
        hypothesis: false,
        start: 1,
        end: 10,
    }
}
fn ask(c: Context, t: u32, p: Decision, s: Decision) -> Step {
    Step::Ask(
        Query {
            context: c,
            from: 11,
            to: 37,
            epoch: t,
        },
        p,
        s,
    )
}
pub fn cases(offset: u32, selected: &[u8]) -> Vec<Case> {
    use Decision::*;
    let c = Context(53, 1);
    let d = Context(67, 1);
    let c2 = Context(53, 2);
    let mut result = Vec::new();
    for &id in selected {
        let mut steps = vec![
            Step::Apply(Op::Context(c, None)),
            Step::Apply(Op::Context(d, None)),
        ];
        for source in 1..=4 {
            let parent = if source == 3 && [9, 14].contains(&id) {
                Some(1)
            } else if source == 3 && id == 10 {
                None
            } else {
                Some(source)
            };
            steps.push(Step::Apply(Op::Source(source, parent)));
        }
        let r = Rule {
            key: Key(401, 1),
            context: c,
            start: 1,
            end: 10,
        };
        steps.extend([
            Step::Apply(Op::Rule(r)),
            Step::Apply(Op::Rule(Rule {
                key: Key(402, 1),
                context: d,
                ..r
            })),
        ]);
        if id == 16 {
            steps.extend([
                Step::Reject(Op::Context(Context(53, 3), Some(c)), Error::Invalid),
                Step::Reject(
                    Op::Grant(Grant {
                        kind: Kind::Rule,
                        key: r.key,
                        from: c,
                        to: d,
                        declaration: 501,
                    }),
                    Error::Invalid,
                ),
                Step::Reject(Op::Source(1, Some(1)), Error::Duplicate),
                Step::BadCut(Query {
                    context: c,
                    from: 11,
                    to: 37,
                    epoch: 1,
                }),
            ]);
        } else {
            let mut a = fact(201, 11, 23, 1, 101, c);
            if id == 4 {
                a.end = 3;
            }
            let b = fact(202, 23, 37, 2, 102, if id == 1 { d } else { c });
            steps.extend([Step::Apply(Op::Fact(a)), Step::Apply(Op::Fact(b))]);
            if (6..=11).contains(&id) || [14, 15].contains(&id) {
                let mut alt = fact(203, 11, 41, 3, 103, c);
                if id == 7 {
                    alt.observation = 101;
                }
                if id == 15 {
                    alt.hypothesis = true;
                }
                steps.extend([
                    Step::Apply(Op::Fact(alt)),
                    Step::Apply(Op::Fact(fact(204, 41, 37, 4, 104, c))),
                ]);
            }
            if id == 13 {
                steps.extend([
                    Step::Apply(Op::Fact(fact(205, 11, 23, 3, 105, d))),
                    Step::Apply(Op::Fact(fact(206, 23, 37, 4, 106, d))),
                ]);
            }
            steps.push(Step::Apply(Op::Refresh));
            let neg = |direct: bool| {
                let mut n = fact(207, 11, if direct { 37 } else { 23 }, 5, 107, c);
                n.positive = false;
                [
                    Step::Apply(Op::Source(5, Some(5))),
                    Step::Apply(Op::Fact(n)),
                ]
            };
            match id {
                1 => steps.extend([
                    ask(c, 1, Unknown, Unknown),
                    ask(d, 1, Unknown, Unknown),
                    Step::Apply(Op::Fact(fact(212, 23, 37, 2, 122, c))),
                    ask(c, 1, Unknown, Supported),
                    Step::Apply(Op::Refresh),
                    ask(c, 1, Supported, Supported),
                ]),
                2 => steps.extend([
                    ask(c, 1, Supported, Supported),
                    Step::Apply(Op::Context(c2, Some(c))),
                    Step::Apply(Op::Rule(Rule {
                        key: Key(403, 1),
                        context: c2,
                        ..r
                    })),
                    ask(c2, 1, Unknown, Unknown),
                    Step::Apply(Op::Fact(fact(211, 11, 23, 1, 121, c2))),
                    Step::Apply(Op::Refresh),
                    ask(c2, 1, Unknown, Unknown),
                    Step::Apply(Op::Fact(fact(212, 23, 37, 2, 122, c2))),
                    ask(c2, 1, Unknown, Supported),
                    Step::Apply(Op::Refresh),
                    ask(c2, 1, Supported, Supported),
                    Step::History(0),
                ]),
                3 | 4 => {
                    let time = if id == 4 { 2 } else { 1 };
                    steps.push(ask(c, time, Supported, Supported));
                    if id == 4 {
                        steps.push(ask(c, 3, Expired, Expired));
                    }
                    let mut revised = a;
                    revised.key.1 = 2;
                    revised.observation = 111;
                    if id == 4 {
                        revised.start = 3;
                        revised.end = 10;
                    }
                    let t = if id == 4 { 3 } else { 1 };
                    steps.extend([
                        Step::Apply(Op::Fact(revised)),
                        ask(c, t, if id == 4 { Expired } else { Stale }, Supported),
                        Step::Apply(Op::Refresh),
                        ask(c, t, Supported, Supported),
                        Step::History(0),
                    ]);
                }
                5 => steps.extend([
                    ask(c, 1, Supported, Supported),
                    Step::Apply(Op::Set(Kind::Source, Key(1, 1), Status::Suspended)),
                    ask(c, 1, Suspended, Suspended),
                    Step::Apply(Op::Refresh),
                    ask(c, 1, Suspended, Suspended),
                    Step::Apply(Op::Set(Kind::Source, Key(1, 1), Status::Active)),
                    Step::Apply(Op::Refresh),
                    ask(c, 1, Supported, Supported),
                    Step::Apply(Op::Set(Kind::Fact, a.key, Status::Retracted)),
                    ask(c, 1, Retracted, Retracted),
                    Step::Reject(Op::Set(Kind::Fact, a.key, Status::Active), Error::Terminal),
                    Step::History(0),
                ]),
                6 | 7 | 10 | 11 | 15 => {
                    steps.push(ask(c, 1, Supported, Supported));
                    steps.extend(neg(id == 11));
                    let verdict = match id {
                        6 => Supported,
                        10 => Indeterminate,
                        _ => Conflict,
                    };
                    steps.extend([ask(c, 1, verdict, verdict), Step::History(0)]);
                }
                8 | 9 => steps.extend([
                    ask(c, 1, Supported, Supported),
                    Step::Apply(Op::Set(
                        if id == 8 { Kind::Rule } else { Kind::Source },
                        if id == 8 { r.key } else { Key(1, 1) },
                        Status::Contested,
                    )),
                    ask(c, 1, Conflict, Conflict),
                    Step::History(0),
                ]),
                12 => {
                    steps.extend([
                        ask(c, 1, Supported, Supported),
                        Step::Apply(Op::Context(c2, Some(c))),
                        ask(c2, 1, Unknown, Unknown),
                    ]);
                    for (kind, key, decl) in [(Kind::Fact, a.key, 501), (Kind::Fact, b.key, 502)] {
                        steps.push(Step::Apply(Op::Grant(Grant {
                            kind,
                            key,
                            from: c,
                            to: c2,
                            declaration: decl,
                        })));
                    }
                    steps.push(ask(c2, 1, Unknown, Unknown));
                    steps.extend([
                        Step::Apply(Op::Grant(Grant {
                            kind: Kind::Rule,
                            key: r.key,
                            from: c,
                            to: c2,
                            declaration: 503,
                        })),
                        ask(c2, 1, Unknown, Supported),
                        Step::Apply(Op::Refresh),
                        ask(c2, 1, Supported, Supported),
                        Step::History(0),
                    ]);
                }
                13 => steps.extend([
                    ask(c, 1, Supported, Supported),
                    ask(d, 1, Supported, Supported),
                    Step::Apply(Op::Set(Kind::Source, Key(1, 1), Status::Suspended)),
                    ask(c, 1, Suspended, Suspended),
                    ask(d, 1, Supported, Supported),
                    Step::Apply(Op::Refresh),
                    ask(d, 1, Supported, Supported),
                    Step::History(0),
                    Step::History(1),
                ]),
                14 => {
                    steps.push(ask(c, 1, Supported, Supported));
                    steps.extend(neg(false));
                    steps.extend([
                        ask(c, 1, Supported, Supported),
                        Step::Apply(Op::Set(Kind::Source, Key(1, 1), Status::Contested)),
                        ask(c, 1, Conflict, Conflict),
                        Step::History(1),
                    ]);
                }
                _ => panic!("unfrozen case"),
            }
        }
        for step in &mut steps {
            shift_step(step, offset);
        }
        result.push(Case { id, steps });
    }
    result
}
fn shift_step(step: &mut Step, o: u32) {
    let ctx = |c: &mut Context| c.0 += o;
    let key = |k: &mut Key| k.0 += o;
    fn change(op: &mut Op, o: u32) {
        let ctx = |c: &mut Context| c.0 += o;
        let key = |k: &mut Key| k.0 += o;
        match op {
            Op::Context(c, p) => {
                ctx(c);
                if let Some(p) = p {
                    ctx(p);
                }
            }
            Op::Source(id, p) => {
                *id += o;
                if let Some(p) = p {
                    *p += o;
                }
            }
            Op::Fact(f) => {
                key(&mut f.key);
                ctx(&mut f.context);
                f.from += o;
                f.to += o;
                f.source += o;
                f.observation += o;
            }
            Op::Rule(r) => {
                key(&mut r.key);
                ctx(&mut r.context);
            }
            Op::Grant(g) => {
                key(&mut g.key);
                ctx(&mut g.from);
                ctx(&mut g.to);
                g.declaration += o;
            }
            Op::Set(_, k, _) => key(k),
            Op::Refresh => (),
        }
    }
    match step {
        Step::Apply(op) | Step::Reject(op, _) => change(op, o),
        Step::Ask(q, _, _) | Step::BadCut(q) => {
            ctx(&mut q.context);
            q.from += o;
            q.to += o;
        }
        Step::History(_) => (),
    }
    let _ = key;
}
pub const ALL: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
pub const DEV: [u8; 5] = [1, 3, 6, 12, 16];
pub const VALID: [u8; 6] = [2, 4, 7, 9, 13, 15];
