//! Independent symbolic table: reconstruction of separate maps from its own log.
//! Does not import candidate or literal and implements its own admission checks.
use super::types::*;
use std::collections::BTreeMap;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbolic {
    log: Vec<Op>,
}
#[derive(Clone, Debug, Default)]
struct Table {
    sources: BTreeMap<Key, Status>,
    facts: BTreeMap<Key, (Fact, Status)>,
    rules: BTreeMap<Key, (Rule, Status)>,
    proofs: Vec<Proof>,
    epoch: u32,
}
impl Table {
    fn causes(&self, k: Key, t: u32) -> Vec<Decision> {
        let mut c = Vec::new();
        let (f, status) = self.facts[&k];
        for st in [
            status,
            *self.sources.get(&f.source).unwrap_or(&Status::Retracted),
        ] {
            match st {
                Status::Suspended => c.push(Decision::Suspended),
                Status::Retracted => c.push(Decision::Retracted),
                _ => {}
            }
        }
        if t < f.start || t >= f.end {
            c.push(Decision::Expired)
        }
        if self
            .facts
            .keys()
            .any(|other| other.0 == k.0 && other.1 > k.1)
        {
            c.push(Decision::Stale)
        }
        if f.hypothesis {
            c.push(Decision::Unknown)
        }
        c
    }
    fn rule_causes(&self, k: Key, t: u32) -> Vec<Decision> {
        let (r, st) = self.rules[&k];
        let mut c = Vec::new();
        if st == Status::Suspended {
            c.push(Decision::Suspended)
        }
        if st == Status::Retracted {
            c.push(Decision::Retracted)
        }
        if t < r.start || t >= r.end {
            c.push(Decision::Expired)
        }
        if self
            .rules
            .keys()
            .any(|other| other.0 == k.0 && other.1 > k.1)
        {
            c.push(Decision::Stale)
        }
        c
    }
    fn step(&mut self, op: Op) -> Result<Work, Error> {
        let mut work = Work {
            reads: self.facts.len() + self.rules.len() + self.sources.len(),
            pairs: 0,
        };
        match op {
            Op::Source(k) => {
                if self.sources.contains_key(&k) {
                    return Err(Error::Duplicate);
                }
                if k.0 == 0 || k.1 != 1 {
                    return Err(Error::InvalidInput);
                }
                if self.sources.len() >= 8 {
                    return Err(Error::Capacity);
                }
                self.sources.insert(k, Status::Active);
            }
            Op::Fact(f) | Op::Revise(f) => {
                match self.sources.get(&f.source) {
                    Some(Status::Active) => {}
                    _ => return Err(Error::MissingSource),
                }
                if f.start >= f.end {
                    return Err(Error::InvalidInterval);
                }
                if f.key.0 == 0
                    || f.key.1 == 0
                    || f.observation == 0
                    || f.context == 0
                    || f.from == 0
                    || f.to == 0
                    || f.observed != self.epoch
                {
                    return Err(Error::InvalidInput);
                }
                if self.facts.contains_key(&f.key)
                    || self
                        .facts
                        .values()
                        .any(|x| x.0.observation == f.observation)
                {
                    return Err(Error::Duplicate);
                }
                let n = self
                    .facts
                    .keys()
                    .filter(|k| k.0 == f.key.0)
                    .map(|k| k.1)
                    .max()
                    .unwrap_or(0);
                if f.key.1 != n + 1 {
                    return Err(Error::InvalidInput);
                }
                if matches!(op, Op::Revise(_)) && n == 0 {
                    return Err(Error::MissingReference);
                }
                if self.facts.len() + self.proofs.len() >= 64 {
                    return Err(Error::Capacity);
                }
                self.facts.insert(f.key, (f, Status::Active));
            }
            Op::Rule(r) => {
                if r.start >= r.end {
                    return Err(Error::InvalidInterval);
                }
                if self.rules.contains_key(&r.key) {
                    return Err(Error::Duplicate);
                }
                if r.key.0 == 0 || r.key.1 == 0 || r.context == 0 {
                    return Err(Error::InvalidInput);
                }
                if self.rules.len() >= 4 {
                    return Err(Error::Capacity);
                }
                self.rules.insert(r.key, (r, Status::Active));
            }
            Op::Set(kind, k, next) => {
                let slot = match kind {
                    Kind::Source => self.sources.get_mut(&k),
                    Kind::Fact => self.facts.get_mut(&k).map(|x| &mut x.1),
                    Kind::Rule => self.rules.get_mut(&k).map(|x| &mut x.1),
                }
                .ok_or(Error::MissingReference)?;
                if *slot == Status::Retracted {
                    return Err(Error::TerminalRetraction);
                }
                if next == Status::Active && *slot != Status::Suspended {
                    return Err(Error::InvalidInput);
                }
                *slot = next;
            }
            Op::Epoch(t) => {
                if t < self.epoch {
                    return Err(Error::InvalidInput);
                }
                self.epoch = t;
            }
            Op::Refresh => {
                let mut found = Vec::new();
                for (rk, (r, _)) in &self.rules {
                    for (ak, (a, _)) in &self.facts {
                        for (bk, (b, _)) in &self.facts {
                            work.reads += 3;
                            work.pairs += 1;
                            if !self.rule_causes(*rk, self.epoch).is_empty()
                                || !self.causes(*ak, self.epoch).is_empty()
                                || !self.causes(*bk, self.epoch).is_empty()
                            {
                                continue;
                            }
                            if a.positive
                                && b.positive
                                && a.context == r.context
                                && b.context == r.context
                                && a.to == b.from
                                && a.from != a.to
                                && b.from != b.to
                                && a.from != b.to
                            {
                                let p = Proof {
                                    rule: *rk,
                                    first: *ak,
                                    second: *bk,
                                };
                                if !self.proofs.contains(&p) {
                                    found.push(p)
                                }
                            }
                        }
                    }
                }
                found.sort();
                found.dedup();
                if self.facts.len() + self.proofs.len() + found.len() > 64
                    || (self.proofs.len() + found.len()) * 2 > 128
                {
                    return Err(Error::Capacity);
                }
                self.proofs.extend(found);
                self.proofs.sort();
            }
            Op::Associate(rows, n) => {
                if n > 2 {
                    return Err(Error::InvalidInput);
                }
                let proposed: Vec<_> = rows[..n].iter().map(|r| r.id).collect();
                for r in &rows[..n] {
                    for dep in r.deps {
                        if !proposed.contains(&dep) && !self.facts.keys().any(|k| k.0 == dep) {
                            return Err(Error::MissingReference);
                        }
                    }
                }
                // Graph walk independent of candidate's two-node comparison.
                for r in &rows[..n] {
                    let mut stack = r.deps.to_vec();
                    let mut seen = Vec::new();
                    while let Some(d) = stack.pop() {
                        work.reads += 1;
                        if d == r.id {
                            return Err(Error::Cycle);
                        }
                        if seen.contains(&d) {
                            continue;
                        }
                        seen.push(d);
                        if let Some(next) = rows[..n].iter().find(|x| x.id == d) {
                            stack.extend(next.deps)
                        }
                    }
                }
                return Err(Error::InvalidInput);
            }
            Op::Schema(v) => {
                if v != 1 {
                    return Err(Error::UnsupportedVersion);
                }
            }
        }
        // Conservative accounting includes map traversals and eligibility scans.
        if matches!(op, Op::Refresh) {
            work.reads += (self.facts.len() + self.sources.len() + self.rules.len() + 1).pow(3);
        }
        if work.reads + work.pairs > 8192 {
            return Err(Error::Budget);
        }
        Ok(work)
    }
}
impl Symbolic {
    fn replay(&self, cut: usize) -> Result<(Table, Work), Error> {
        if cut > self.log.len() {
            return Err(Error::InvalidCut);
        }
        let mut t = Table {
            epoch: 1,
            ..Table::default()
        };
        let mut work = Work::default();
        for op in &self.log[..cut] {
            let w = t.step(*op)?;
            work.reads += w.reads;
            work.pairs += w.pairs;
        }
        Ok((t, work))
    }
}
impl Laboratory for Symbolic {
    fn new() -> Self {
        Self { log: Vec::new() }
    }
    fn len(&self) -> usize {
        self.log.len()
    }
    fn apply(&mut self, op: Op) -> Result<Work, Error> {
        if self.log.len() >= 128 {
            return Err(Error::Capacity);
        }
        let (mut table, mut w) = self.replay(self.len())?;
        let d = table.step(op)?;
        w.reads += d.reads;
        w.pairs += d.pairs;
        if w.reads + w.pairs > 8192 {
            return Err(Error::Budget);
        }
        self.log.push(op);
        Ok(w)
    }
    fn query(&self, q: Query, cut: usize) -> Result<(Answer, Work), Error> {
        if q.context == 0 || q.from == 0 || q.to == 0 {
            return Err(Error::InvalidInput);
        }
        let (t, mut w) = self.replay(cut)?;
        let mut out = Answer {
            decision: Decision::Unknown,
            causes: Vec::new(),
            proofs: Vec::new(),
            evidence: Vec::new(),
            conflicts: Vec::new(),
            cut,
            epoch: q.epoch,
        };
        for proof in &t.proofs {
            let (a, _) = t.facts[&proof.first];
            let (b, _) = t.facts[&proof.second];
            let (r, _) = t.rules[&proof.rule];
            if a.from != q.from
                || b.to != q.to
                || a.context != q.context
                || b.context != q.context
                || r.context != q.context
            {
                continue;
            }
            let mut bad = t.causes(proof.first, q.epoch);
            bad.extend(t.causes(proof.second, q.epoch));
            bad.extend(t.rule_causes(proof.rule, q.epoch));
            if bad.is_empty() {
                out.proofs.push(*proof)
            } else {
                out.causes.extend(bad)
            }
        }
        for (key, (f, _)) in &t.facts {
            if f.positive || f.context != q.context || !t.causes(*key, q.epoch).is_empty() {
                continue;
            }
            let mut relevant = f.from == q.from && f.to == q.to;
            for proof in &out.proofs {
                for pk in [proof.first, proof.second] {
                    let p = t.facts[&pk].0;
                    if p.from == f.from && p.to == f.to {
                        relevant = true;
                    }
                }
            }
            if relevant {
                out.conflicts.push(*key);
                for (pk, (p, _)) in &t.facts {
                    if p.positive
                        && p.context == q.context
                        && p.from == f.from
                        && p.to == f.to
                        && t.causes(*pk, q.epoch).is_empty()
                    {
                        out.conflicts.push(*pk)
                    }
                }
            }
        }
        out.proofs.sort();
        out.proofs.dedup();
        out.causes.sort();
        out.causes.dedup();
        out.conflicts.sort();
        out.conflicts.dedup();
        out.decision = if !out.conflicts.is_empty() {
            Decision::Conflict
        } else if !out.proofs.is_empty() {
            Decision::Supported
        } else if out.causes.contains(&Decision::Suspended) {
            Decision::Suspended
        } else if out.causes.contains(&Decision::Retracted) {
            Decision::Retracted
        } else if out.causes.contains(&Decision::Expired) {
            Decision::Expired
        } else if out.causes.contains(&Decision::Stale) {
            Decision::Stale
        } else {
            Decision::Unknown
        };
        for p in &out.proofs {
            for key in [p.first, p.second] {
                let f = t.facts[&key].0;
                out.evidence.push((key, f.observation, f.source))
            }
        }
        for key in &out.conflicts {
            let f = t.facts[key].0;
            out.evidence.push((*key, f.observation, f.source))
        }
        out.evidence.sort();
        out.evidence.dedup();
        w.reads += (t.facts.len() + t.sources.len() + t.rules.len() + 1).pow(3);
        if w.reads + w.pairs > 8192 {
            return Err(Error::Budget);
        }
        Ok((out, w))
    }
    fn bytes(&self) -> usize {
        // Includes a conservative reserve for transient maps, copies and query result.
        std::mem::size_of::<Self>() + self.log.capacity() * std::mem::size_of::<Op>() + 64 * 1024
    }
}
