//! Evaluator-only frozen cases. Existing models import only provenance_types.
use super::types::*;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cut {
    Current,
    Saved(u8),
    Empty,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Step {
    Apply(Op),
    Probe {
        query: Query,
        cut: Cut,
        expected: [Decision; 3],
        save: u8,
        stable: u8,
    },
    InvalidCut(Query),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Case {
    pub name: &'static str,
    pub steps: Vec<Step>,
}
struct Builder {
    b: u32,
    epoch: u32,
    steps: Vec<Step>,
}
impl Builder {
    fn new(b: u32) -> Self {
        Self {
            b,
            epoch: 1,
            steps: Vec::new(),
        }
    }
    fn key(&self, role: u32, version: u16) -> Key {
        Key(self.b + 200 + role, version)
    }
    fn source(&self, role: u32) -> Key {
        Key(self.b + 100 + role, 1)
    }
    fn rule(&self, d: bool) -> Key {
        Key(self.b + 401 + u32::from(d), 1)
    }
    fn context(&self, d: bool) -> u32 {
        self.b + if d { 67 } else { 53 }
    }
    fn op(&mut self, op: Op) {
        if let Op::Epoch(e) = op {
            self.epoch = e
        };
        self.steps.push(Step::Apply(op));
    }
    fn sources(&mut self, n: u32) {
        for r in 1..=n {
            self.op(Op::Source(self.source(r)));
        }
    }
    fn fact(
        &self,
        role: u32,
        version: u16,
        from: u32,
        to: u32,
        positive: bool,
        d: bool,
        start: u32,
        end: u32,
    ) -> Fact {
        Fact {
            key: self.key(role, version),
            observation: self.b + 300 + role + 50 * u32::from(version - 1),
            source: self.source(role),
            context: self.context(d),
            from: self.b + from,
            to: self.b + to,
            positive,
            hypothesis: false,
            observed: self.epoch,
            start,
            end,
        }
    }
    fn base(&mut self, kind: u8) {
        // 0 common, 1 temporal, 2 cross-context opposition, 3 cross-context positive, 4 alternative.
        self.sources(if kind >= 2 { 4 } else { 2 });
        self.op(Op::Fact(self.fact(
            1,
            1,
            11,
            23,
            true,
            false,
            1,
            if kind == 1 || kind == 4 { 3 } else { 10 },
        )));
        self.op(Op::Fact(self.fact(2, 1, 23, 37, true, false, 1, 10)));
        if kind >= 2 {
            self.op(Op::Fact(self.fact(
                3,
                1,
                11,
                if kind == 4 { 41 } else { 23 },
                kind != 2,
                kind != 4,
                1,
                10,
            )));
            self.op(Op::Fact(self.fact(
                4,
                1,
                if kind == 4 { 41 } else { 23 },
                37,
                true,
                kind != 4,
                1,
                10,
            )));
        }
        self.op(Op::Rule(Rule {
            key: self.rule(false),
            context: self.context(false),
            start: 1,
            end: 10,
        }));
        if kind == 2 || kind == 3 {
            self.op(Op::Rule(Rule {
                key: self.rule(true),
                context: self.context(true),
                start: 1,
                end: 10,
            }));
        }
        self.op(Op::Refresh);
    }
    fn q(&self, d: bool) -> Query {
        Query {
            from: self.b + 11,
            to: self.b + 37,
            context: self.context(d),
            epoch: self.epoch,
        }
    }
    fn probe(&mut self, cs: Decision, lit: Decision, d: bool, cut: Cut, save: u8, stable: u8) {
        self.steps.push(Step::Probe {
            query: self.q(d),
            cut,
            expected: [cs, cs, lit],
            save,
            stable,
        });
    }
    fn same(&mut self, d: Decision) {
        self.probe(d, d, false, Cut::Current, 0, 0);
    }
    fn revise(&mut self, changed: bool, start: u32) {
        self.op(Op::Revise(self.fact(
            1,
            2,
            11,
            if changed { 41 } else { 23 },
            true,
            false,
            start,
            10,
        )));
    }
}
pub fn cases(offset: u32) -> Vec<Case> {
    use Decision::*;
    let names = ["A", "B", "C", "D", "D-alt", "E", "F", "G"];
    let mut cases = Vec::new();
    for (i, name) in names.into_iter().enumerate() {
        let mut x = Builder::new(offset + 1000 * (i as u32 + 1));
        x.same(Unknown);
        match name {
            "A" => {
                x.base(0);
                x.same(Supported);
                x.revise(true, 1);
                x.same(Stale);
                x.op(Op::Refresh);
                x.same(Stale);
                x.op(Op::Source(x.source(3)));
                x.op(Op::Fact(x.fact(3, 1, 41, 37, true, false, 1, 10)));
                x.op(Op::Refresh);
                x.same(Supported);
            }
            "B" => {
                x.base(2);
                x.same(Supported);
                x.probe(Unknown, Unknown, true, Cut::Current, 0, 0);
                let mut q = x.q(false);
                q.context = x.b + 79;
                x.steps.push(Step::Probe {
                    query: q,
                    cut: Cut::Current,
                    expected: [Unknown; 3],
                    save: 0,
                    stable: 0,
                });
            }
            "C" => {
                x.base(0);
                x.same(Supported);
                x.op(Op::Source(x.source(3)));
                x.op(Op::Fact(x.fact(3, 1, 11, 23, false, false, 1, 10)));
                x.same(Conflict);
                x.op(Op::Refresh);
                x.same(Conflict);
                x.op(Op::Set(Kind::Fact, x.key(3, 1), Status::Retracted));
                x.same(Supported);
                x.op(Op::Refresh);
                x.same(Supported);
            }
            "D" => {
                x.base(1);
                x.same(Supported);
                x.op(Op::Epoch(2));
                x.same(Supported);
                x.op(Op::Epoch(3));
                x.same(Expired);
                x.op(Op::Refresh);
                x.same(Expired);
                x.revise(false, 3);
                x.probe(Expired, Supported, false, Cut::Current, 0, 0);
                x.op(Op::Refresh);
                x.same(Supported);
            }
            "D-alt" => {
                x.base(4);
                x.same(Supported);
                x.op(Op::Epoch(3));
                x.same(Supported);
                x.op(Op::Refresh);
                x.same(Supported);
            }
            "E" => {
                x.base(0);
                x.same(Supported);
                x.op(Op::Set(Kind::Source, x.source(1), Status::Suspended));
                x.same(Suspended);
                x.op(Op::Refresh);
                x.same(Suspended);
                x.op(Op::Set(Kind::Source, x.source(1), Status::Active));
                x.op(Op::Refresh);
                x.same(Supported);
                x.revise(false, 1);
                x.probe(Stale, Supported, false, Cut::Current, 0, 0);
                x.op(Op::Refresh);
                x.same(Supported);
                x.op(Op::Set(Kind::Rule, x.rule(false), Status::Suspended));
                x.same(Suspended);
                x.op(Op::Refresh);
                x.same(Suspended);
                x.op(Op::Set(Kind::Rule, x.rule(false), Status::Active));
                x.op(Op::Refresh);
                x.same(Supported);
            }
            "F" => {
                x.base(3);
                x.same(Supported);
                x.probe(Supported, Supported, true, Cut::Current, 0, 1);
                x.op(Op::Set(Kind::Source, x.source(1), Status::Suspended));
                x.same(Suspended);
                x.probe(Supported, Supported, true, Cut::Current, 0, 2);
                x.op(Op::Refresh);
                x.same(Suspended);
                x.probe(Supported, Supported, true, Cut::Current, 0, 2);
            }
            "G" => {
                x.base(0);
                x.probe(Supported, Supported, false, Cut::Current, 1, 0);
                x.revise(false, 1);
                x.probe(Stale, Supported, false, Cut::Current, 0, 0);
                x.op(Op::Refresh);
                x.probe(Supported, Supported, false, Cut::Current, 2, 0);
                x.op(Op::Set(Kind::Fact, x.key(1, 2), Status::Retracted));
                x.op(Op::Refresh);
                x.same(Retracted);
                x.probe(Supported, Supported, false, Cut::Saved(1), 0, 0);
                x.probe(Supported, Supported, false, Cut::Saved(2), 0, 0);
                x.probe(Unknown, Unknown, false, Cut::Empty, 0, 0);
                x.steps.push(Step::InvalidCut(x.q(false)));
            }
            _ => unreachable!(),
        }
        cases.push(Case {
            name,
            steps: x.steps,
        });
    }
    cases
}
