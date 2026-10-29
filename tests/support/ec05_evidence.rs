//! Evaluator-only proof audit. Never called by an experimental executor.
use super::types::*;
use std::collections::BTreeSet;
fn status(log: &[Op], kind: Kind, key: Key) -> Option<Status> {
    let mut value = None;
    for op in log {
        match *op {
            Op::Source(k) if kind == Kind::Source && k == key => value = Some(Status::Active),
            Op::Fact(f) | Op::Revise(f) if kind == Kind::Fact && f.key == key => {
                value = Some(Status::Active)
            }
            Op::Rule(r) if kind == Kind::Rule && r.key == key => value = Some(Status::Active),
            Op::Set(k, id, s) if k == kind && id == key => value = Some(s),
            _ => {}
        }
    }
    value
}
pub fn verify(log: &[Op], q: Query, a: &Answer) {
    assert!(a.cut <= log.len());
    assert_eq!(a.epoch, q.epoch);
    let log = &log[..a.cut];
    let facts: Vec<_> = log
        .iter()
        .filter_map(|o| match *o {
            Op::Fact(f) | Op::Revise(f) => Some(f),
            _ => None,
        })
        .collect();
    let rules: Vec<_> = log
        .iter()
        .filter_map(|o| match *o {
            Op::Rule(r) => Some(r),
            _ => None,
        })
        .collect();
    let live = |f: &Fact| {
        f.context == q.context
            && f.start <= q.epoch
            && q.epoch < f.end
            && !f.hypothesis
            && status(log, Kind::Fact, f.key) == Some(Status::Active)
            && status(log, Kind::Source, f.source) == Some(Status::Active)
            && !facts
                .iter()
                .any(|g| g.key.0 == f.key.0 && g.key.1 > f.key.1)
    };
    let live_rule = |r: &Rule| {
        r.context == q.context
            && r.start <= q.epoch
            && q.epoch < r.end
            && status(log, Kind::Rule, r.key) == Some(Status::Active)
            && !rules
                .iter()
                .any(|s| s.key.0 == r.key.0 && s.key.1 > r.key.1)
    };
    let mut paths = BTreeSet::new();
    for r in rules.iter().filter(|r| live_rule(r)) {
        for f in facts
            .iter()
            .filter(|f| live(f) && f.positive && f.from == q.from)
        {
            for g in facts
                .iter()
                .filter(|g| live(g) && g.positive && g.to == q.to)
            {
                if f.to == g.from && f.from != f.to && g.from != g.to && f.from != g.to {
                    paths.insert(Proof {
                        rule: r.key,
                        first: f.key,
                        second: g.key,
                    });
                }
            }
        }
    }
    let actual: BTreeSet<_> = a.proofs.iter().copied().collect();
    assert_eq!(actual.len(), a.proofs.len(), "duplicate proof");
    if matches!(a.decision, Decision::Supported | Decision::Conflict) {
        assert_eq!(actual, paths, "incomplete or ineligible proof");
    } else {
        assert!(a.proofs.is_empty());
    }
    let mut conflicts = BTreeSet::new();
    for n in facts.iter().filter(|f| live(f) && !f.positive) {
        let relevant = (n.from == q.from && n.to == q.to)
            || paths.iter().any(|p| {
                [p.first, p.second].iter().any(|k| {
                    facts
                        .iter()
                        .any(|f| f.key == *k && f.from == n.from && f.to == n.to)
                })
            });
        if relevant {
            conflicts.insert(n.key);
            for f in facts
                .iter()
                .filter(|f| live(f) && f.positive && f.from == n.from && f.to == n.to)
            {
                conflicts.insert(f.key);
            }
        }
    }
    assert_eq!(
        a.conflicts.iter().copied().collect::<BTreeSet<_>>(),
        conflicts
    );
    let mut evidence = BTreeSet::new();
    for key in a
        .proofs
        .iter()
        .flat_map(|p| [p.first, p.second])
        .chain(a.conflicts.iter().copied())
    {
        let f = facts
            .iter()
            .find(|f| f.key == key)
            .expect("broken fact reference");
        assert!(live(f));
        evidence.insert((f.key, f.observation, f.source));
    }
    assert_eq!(
        a.evidence.iter().copied().collect::<BTreeSet<_>>(),
        evidence
    );
    assert_eq!(a.evidence.len(), evidence.len(), "duplicate evidence");
}
