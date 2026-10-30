//! P2: independent log replay and on-demand relational join.
//! Imports DTOs only; never calls P1 validation, maintenance or decisions.
use super::retention_types::*;
use std::collections::BTreeMap;
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Symbolic {
    journal: Vec<Op>,
}
#[derive(Default)]
struct Snapshot {
    contexts: BTreeMap<Context, Option<Context>>,
    origins: BTreeMap<u32, Option<u32>>,
    facts: BTreeMap<Key, Fact>,
    rules: BTreeMap<Key, Rule>,
    states: BTreeMap<(Kind, Key), Status>,
    links: Vec<Grant>,
}
impl Snapshot {
    fn rights(
        &self,
        kind: Kind,
        key: Key,
        born: Context,
        target: Context,
    ) -> Option<Vec<Dependency>> {
        if born.0 != target.0 || born.1 > target.1 {
            return None;
        }
        let mut evidence = Vec::new();
        for v in born.1..target.1 {
            let bridge = self.links.iter().find(|g| {
                g.kind == kind
                    && g.key == key
                    && g.from == Context(born.0, v)
                    && g.to == Context(born.0, v + 1)
            })?;
            evidence.push(Dependency::Declaration(bridge.declaration));
        }
        Some(evidence)
    }
    fn state_causes(&self, kind: Kind, key: Key) -> Vec<Decision> {
        match self
            .states
            .get(&(kind, key))
            .copied()
            .unwrap_or(Status::Active)
        {
            Status::Active => vec![],
            Status::Suspended => vec![Decision::Suspended],
            Status::Retracted => vec![Decision::Retracted],
            Status::Contested => vec![Decision::Conflict],
        }
    }
    fn inspect(&self, f: Fact, epoch: u32) -> (Vec<Decision>, Vec<Dependency>) {
        let mut reasons = self.state_causes(Kind::Fact, f.key);
        let mut evidence = vec![
            Dependency::Fact(f.key),
            Dependency::Observation(f.observation),
        ];
        if self.facts.keys().any(|k| k.0 == f.key.0 && k.1 > f.key.1) {
            reasons.push(Decision::Stale);
        }
        if !(f.start..f.end).contains(&epoch) {
            reasons.push(Decision::Expired);
        }
        let mut origin = f.source;
        let mut seen = Vec::new();
        loop {
            if seen.len() == 8 || seen.contains(&origin) {
                reasons.push(Decision::Indeterminate);
                break;
            }
            seen.push(origin);
            evidence.push(Dependency::Source(origin));
            reasons.extend(self.state_causes(Kind::Source, Key(origin, 1)));
            match self.origins.get(&origin) {
                Some(Some(parent)) if *parent == origin => break,
                Some(Some(parent)) => origin = *parent,
                _ => {
                    reasons.push(Decision::Indeterminate);
                    break;
                }
            }
        }
        (reasons, evidence)
    }
    fn ingest(&mut self, op: Op) -> Result<(), Error> {
        match op {
            Op::Context(c, p) => {
                if self.contexts.contains_key(&c) {
                    return Err(Error::Duplicate);
                }
                let valid = if c.1 == 1 {
                    p.is_none()
                } else {
                    p == c.1.checked_sub(1).map(|v| Context(c.0, v))
                        && p.is_some_and(|x| self.contexts.contains_key(&x))
                };
                if c.0 == 0 || c.1 == 0 || !valid {
                    return Err(Error::Invalid);
                }
                if self.contexts.len() == 8 {
                    return Err(Error::Capacity);
                }
                self.contexts.insert(c, p);
            }
            Op::Source(id, parent) => {
                if self.origins.contains_key(&id) {
                    return Err(Error::Duplicate);
                }
                if id == 0 || parent.is_some_and(|x| x != id && !self.origins.contains_key(&x)) {
                    return Err(Error::Invalid);
                }
                if self.origins.len() == 8 {
                    return Err(Error::Capacity);
                }
                self.origins.insert(id, parent);
            }
            Op::Fact(f) => {
                if self.facts.contains_key(&f.key) {
                    return Err(Error::Duplicate);
                }
                if f.key.0 == 0
                    || f.key.1 == 0
                    || f.from == 0
                    || f.to == 0
                    || f.observation == 0
                    || f.start >= f.end
                    || !self.contexts.contains_key(&f.context)
                    || !self.origins.contains_key(&f.source)
                    || self
                        .states
                        .get(&(Kind::Source, Key(f.source, 1)))
                        .is_some_and(|x| *x != Status::Active)
                {
                    return Err(Error::Invalid);
                }
                let prior = self
                    .facts
                    .iter()
                    .rev()
                    .find(|(k, _)| k.0 == f.key.0)
                    .map(|(_, v)| v);
                if match prior {
                    None => f.key.1 != 1,
                    Some(p) => {
                        f.key.1 != p.key.1 + 1
                            || f.context != p.context
                            || f.observation == p.observation
                    }
                } {
                    return Err(Error::Invalid);
                }
                if self.facts.len() == 64 {
                    return Err(Error::Capacity);
                }
                self.facts.insert(f.key, f);
            }
            Op::Rule(r) => {
                if self.rules.keys().any(|x| x.0 == r.key.0) {
                    return Err(Error::Duplicate);
                }
                if r.key.0 == 0
                    || r.key.1 != 1
                    || r.start >= r.end
                    || !self.contexts.contains_key(&r.context)
                {
                    return Err(Error::Invalid);
                }
                if self.rules.len() == 4 {
                    return Err(Error::Capacity);
                }
                self.rules.insert(r.key, r);
            }
            Op::Grant(g) => {
                let born = match g.kind {
                    Kind::Fact => self.facts.get(&g.key).map(|f| f.context),
                    Kind::Rule => self.rules.get(&g.key).map(|r| r.context),
                    _ => None,
                }
                .ok_or(Error::Invalid)?;
                if self.contexts.get(&g.to) != Some(&Some(g.from))
                    || g.declaration == 0
                    || self.rights(g.kind, g.key, born, g.from).is_none()
                {
                    return Err(Error::Invalid);
                }
                if self.links.iter().any(|x| {
                    x.declaration == g.declaration
                        || (x.kind == g.kind && x.key == g.key && x.to == g.to)
                }) {
                    return Err(Error::Duplicate);
                }
                if self.links.len() == 128 {
                    return Err(Error::Capacity);
                }
                self.links.push(g);
            }
            Op::Set(k, key, s) => {
                let exists = match k {
                    Kind::Source => key.1 == 1 && self.origins.contains_key(&key.0),
                    Kind::Fact => self.facts.contains_key(&key),
                    Kind::Rule => self.rules.contains_key(&key),
                };
                if !exists {
                    return Err(Error::Invalid);
                }
                if self.states.get(&(k, key)) == Some(&Status::Retracted) {
                    return Err(Error::Terminal);
                }
                self.states.insert((k, key), s);
            }
            Op::Refresh => (),
        }
        Ok(())
    }
    fn replay(ops: &[Op]) -> Result<Self, Error> {
        let mut s = Self::default();
        for op in ops {
            s.ingest(*op)?;
        }
        Ok(s)
    }
    fn solve(&self, q: Query, cut: usize) -> Result<(Answer, usize), Error> {
        if !self.contexts.contains_key(&q.context) || q.from == 0 || q.to == 0 {
            return Err(Error::Invalid);
        }
        let mut ticks = cut + 1;
        let mut blocked_observations = Vec::new();
        let mut conflicts = Vec::new();
        let mut direct = false;
        for neg in self.facts.values().filter(|f| !f.positive && !f.hypothesis) {
            ticks += 1;
            if self
                .rights(Kind::Fact, neg.key, neg.context, q.context)
                .is_none()
                || !self.inspect(*neg, q.epoch).0.is_empty()
            {
                continue;
            }
            if (neg.from, neg.to) == (q.from, q.to) {
                direct = true;
                conflicts.push(neg.key);
            }
            for pos in self.facts.values().filter(|f| f.positive && !f.hypothesis) {
                ticks += 1;
                if (pos.from, pos.to) == (neg.from, neg.to)
                    && self
                        .rights(Kind::Fact, pos.key, pos.context, q.context)
                        .is_some()
                    && self.inspect(*pos, q.epoch).0.is_empty()
                {
                    blocked_observations.push(pos.observation);
                    conflicts.extend([pos.key, neg.key]);
                }
            }
        }
        let mut causes = Vec::new();
        let mut proofs = Vec::new();
        for rule in self.rules.values() {
            let Some(rule_links) = self.rights(Kind::Rule, rule.key, rule.context, q.context)
            else {
                continue;
            };
            for left in self
                .facts
                .values()
                .filter(|f| f.positive && !f.hypothesis && f.from == q.from)
            {
                for right in self
                    .facts
                    .values()
                    .filter(|f| f.positive && !f.hypothesis && f.to == q.to)
                {
                    ticks += 1;
                    if ticks > 8192 {
                        return Err(Error::Budget);
                    }
                    if left.to != right.from
                        || left.from == left.to
                        || right.from == right.to
                        || left.from == right.to
                    {
                        continue;
                    }
                    let Some(a_links) = self.rights(Kind::Fact, left.key, left.context, q.context)
                    else {
                        continue;
                    };
                    let Some(b_links) =
                        self.rights(Kind::Fact, right.key, right.context, q.context)
                    else {
                        continue;
                    };
                    let (mut why, mut deps) = self.inspect(*left, q.epoch);
                    let (rb, db) = self.inspect(*right, q.epoch);
                    why.extend(rb);
                    deps.extend(db);
                    why.extend(self.state_causes(Kind::Rule, rule.key));
                    if !(rule.start..rule.end).contains(&q.epoch) {
                        why.push(Decision::Expired);
                    }
                    if [left.observation, right.observation]
                        .iter()
                        .any(|id| blocked_observations.contains(id))
                    {
                        why.push(Decision::Conflict);
                    }
                    if why.is_empty() {
                        deps.extend([Dependency::Context(q.context), Dependency::Rule(rule.key)]);
                        deps.extend(rule_links.iter().copied());
                        deps.extend(a_links);
                        deps.extend(b_links);
                        deps.sort();
                        deps.dedup();
                        proofs.push(Proof {
                            context: q.context,
                            rule: rule.key,
                            first: left.key,
                            second: right.key,
                            dependencies: deps,
                        });
                    } else {
                        causes.extend(why);
                    }
                }
            }
        }
        if direct {
            causes.push(Decision::Conflict);
        }
        causes.sort();
        causes.dedup();
        proofs.sort();
        proofs.dedup();
        conflicts.sort();
        conflicts.dedup();
        let mut outcome = Decision::Unknown;
        for reason in [
            Decision::Stale,
            Decision::Expired,
            Decision::Retracted,
            Decision::Suspended,
            Decision::Conflict,
            Decision::Indeterminate,
        ] {
            if causes.contains(&reason) {
                outcome = reason;
            }
        }
        if !proofs.is_empty() {
            outcome = Decision::Supported;
        }
        if causes.contains(&Decision::Conflict) && causes.contains(&Decision::Indeterminate) {
            outcome = Decision::Indeterminate;
        }
        if direct {
            outcome = Decision::Conflict;
        }
        if outcome != Decision::Supported {
            proofs.clear();
        }
        Ok((
            Answer {
                query: q,
                cut,
                decision: outcome,
                causes,
                proofs,
                conflicts,
            },
            ticks,
        ))
    }
}
impl Executor for Symbolic {
    fn new() -> Self {
        Self::default()
    }
    fn apply(&mut self, op: Op) -> Result<usize, Error> {
        if self.journal.len() == 128 {
            return Err(Error::Capacity);
        }
        let mut snapshot = Snapshot::replay(&self.journal)?;
        snapshot.ingest(op)?;
        let cost = self.journal.len() + 1;
        self.journal.push(op);
        Ok(cost)
    }
    fn query(&self, q: Query, cut: usize) -> Result<(Answer, usize), Error> {
        if cut > self.journal.len() {
            return Err(Error::InvalidCut);
        }
        Snapshot::replay(&self.journal[..cut])?.solve(q, cut)
    }
    fn cut(&self) -> usize {
        self.journal.len()
    }
    fn bytes(&self) -> usize {
        std::mem::size_of::<Self>() + self.journal.capacity() * std::mem::size_of::<Op>()
    }
}
