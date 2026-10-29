//! Candidate: online version projection plus materialized proof provenance.
use super::types::*;
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct View {
    sources: Vec<(Key, Status)>,
    facts: Vec<(Fact, Status)>,
    rules: Vec<(Rule, Status)>,
    proofs: Vec<Proof>,
    epoch: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Candidate {
    journal: Vec<Op>,
    view: View,
}
impl View {
    fn source(&self, k: Key) -> Option<Status> {
        self.sources.iter().find(|x| x.0 == k).map(|x| x.1)
    }
    fn fact(&self, k: Key) -> Option<&(Fact, Status)> {
        self.facts.iter().find(|x| x.0.key == k)
    }
    fn rule(&self, k: Key) -> Option<&(Rule, Status)> {
        self.rules.iter().find(|x| x.0.key == k)
    }
    fn reasons(&self, f: &(Fact, Status), epoch: u32) -> Vec<Decision> {
        let mut out = Vec::new();
        for state in [Some(f.1), self.source(f.0.source)] {
            match state {
                Some(Status::Suspended) => out.push(Decision::Suspended),
                Some(Status::Retracted) => out.push(Decision::Retracted),
                None => out.push(Decision::Unknown),
                _ => {}
            }
        }
        if !(f.0.start <= epoch && epoch < f.0.end) {
            out.push(Decision::Expired);
        }
        if self
            .facts
            .iter()
            .any(|x| x.0.key.0 == f.0.key.0 && x.0.key.1 > f.0.key.1)
        {
            out.push(Decision::Stale);
        }
        if f.0.hypothesis {
            out.push(Decision::Unknown);
        }
        out
    }
    fn rule_reasons(&self, r: &(Rule, Status), epoch: u32) -> Vec<Decision> {
        let mut v = Vec::new();
        match r.1 {
            Status::Suspended => v.push(Decision::Suspended),
            Status::Retracted => v.push(Decision::Retracted),
            _ => {}
        }
        if epoch < r.0.start || epoch >= r.0.end {
            v.push(Decision::Expired);
        }
        if self
            .rules
            .iter()
            .any(|x| x.0.key.0 == r.0.key.0 && x.0.key.1 > r.0.key.1)
        {
            v.push(Decision::Stale);
        }
        v
    }
    fn mutate(&mut self, op: Op, work: &mut Work) -> Result<(), Error> {
        work.reads += self.sources.len() + self.facts.len() + self.rules.len() + self.proofs.len();
        match op {
            Op::Source(k) => {
                if k.0 == 0 || k.1 != 1 {
                    return Err(Error::InvalidInput);
                }
                if self.source(k).is_some() {
                    return Err(Error::Duplicate);
                }
                if self.sources.len() == 8 {
                    return Err(Error::Capacity);
                }
                self.sources.push((k, Status::Active));
            }
            Op::Fact(f) | Op::Revise(f) => {
                if self.source(f.source) != Some(Status::Active) {
                    return Err(Error::MissingSource);
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
                if self
                    .facts
                    .iter()
                    .any(|x| x.0.key == f.key || x.0.observation == f.observation)
                {
                    return Err(Error::Duplicate);
                }
                let previous = self
                    .facts
                    .iter()
                    .filter(|x| x.0.key.0 == f.key.0)
                    .map(|x| x.0.key.1)
                    .max()
                    .unwrap_or(0);
                if f.key.1 != previous + 1 {
                    return Err(Error::InvalidInput);
                }
                if matches!(op, Op::Revise(_)) && previous == 0 {
                    return Err(Error::MissingReference);
                }
                if self.facts.len() + self.proofs.len() >= 64 || self.facts.len() >= 64 {
                    return Err(Error::Capacity);
                }
                self.facts.push((f, Status::Active));
            }
            Op::Rule(r) => {
                if r.start >= r.end {
                    return Err(Error::InvalidInterval);
                }
                if self.rule(r.key).is_some() {
                    return Err(Error::Duplicate);
                }
                if r.key.0 == 0 || r.key.1 == 0 || r.context == 0 {
                    return Err(Error::InvalidInput);
                }
                if self.rules.len() == 4 {
                    return Err(Error::Capacity);
                }
                self.rules.push((r, Status::Active));
            }
            Op::Set(kind, key, target) => {
                let state = match kind {
                    Kind::Source => self
                        .sources
                        .iter_mut()
                        .find(|x| x.0 == key)
                        .map(|x| &mut x.1),
                    Kind::Fact => self
                        .facts
                        .iter_mut()
                        .find(|x| x.0.key == key)
                        .map(|x| &mut x.1),
                    Kind::Rule => self
                        .rules
                        .iter_mut()
                        .find(|x| x.0.key == key)
                        .map(|x| &mut x.1),
                }
                .ok_or(Error::MissingReference)?;
                if *state == Status::Retracted {
                    return Err(Error::TerminalRetraction);
                }
                if target == Status::Active && *state != Status::Suspended {
                    return Err(Error::InvalidInput);
                }
                *state = target;
            }
            Op::Epoch(e) => {
                if e < self.epoch {
                    return Err(Error::InvalidInput);
                }
                self.epoch = e;
            }
            Op::Refresh => {
                let mut add = Vec::new();
                for r in &self.rules {
                    for a in &self.facts {
                        for b in &self.facts {
                            work.pairs += 1;
                            work.reads += 3;
                            if work.pairs + work.reads > 8192 {
                                return Err(Error::Budget);
                            }
                            if !self.rule_reasons(r, self.epoch).is_empty()
                                || !self.reasons(a, self.epoch).is_empty()
                                || !self.reasons(b, self.epoch).is_empty()
                            {
                                continue;
                            }
                            if a.0.positive
                                && b.0.positive
                                && a.0.context == r.0.context
                                && b.0.context == r.0.context
                                && a.0.to == b.0.from
                                && a.0.from != a.0.to
                                && b.0.from != b.0.to
                                && a.0.from != b.0.to
                            {
                                let p = Proof {
                                    rule: r.0.key,
                                    first: a.0.key,
                                    second: b.0.key,
                                };
                                if !self.proofs.contains(&p) && !add.contains(&p) {
                                    add.push(p);
                                }
                            }
                        }
                    }
                }
                if self.facts.len() + self.proofs.len() + add.len() > 64
                    || (self.proofs.len() + add.len()) * 2 > 128
                {
                    return Err(Error::Capacity);
                }
                self.proofs.extend(add);
                self.proofs.sort();
            }
            Op::Associate(rows, n) => {
                if n > 2 {
                    return Err(Error::InvalidInput);
                }
                for row in &rows[..n] {
                    for dep in row.deps {
                        if !rows[..n].iter().any(|r| r.id == dep)
                            && !self.facts.iter().any(|f| f.0.key.0 == dep)
                        {
                            return Err(Error::MissingReference);
                        }
                    }
                }
                for row in &rows[..n] {
                    if row.deps.contains(&row.id)
                        || rows[..n].iter().any(|other| {
                            row.deps.contains(&other.id) && other.deps.contains(&row.id)
                        })
                    {
                        return Err(Error::Cycle);
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
        if matches!(op, Op::Refresh) {
            work.reads += (self.facts.len() + self.sources.len() + self.rules.len() + 1).pow(3);
        }
        if work.reads + work.pairs > 8192 {
            return Err(Error::Budget);
        }
        Ok(())
    }
    fn answer(&self, q: Query, cut: usize) -> Result<(Answer, Work), Error> {
        let mut work = Work::default();
        if q.context == 0 || q.from == 0 || q.to == 0 {
            return Err(Error::InvalidInput);
        }
        let mut ans = Answer {
            decision: Decision::Unknown,
            causes: Vec::new(),
            proofs: Vec::new(),
            evidence: Vec::new(),
            conflicts: Vec::new(),
            cut,
            epoch: q.epoch,
        };
        for p in &self.proofs {
            work.reads += 3;
            let a = self.fact(p.first).ok_or(Error::MissingReference)?;
            let b = self.fact(p.second).ok_or(Error::MissingReference)?;
            let r = self.rule(p.rule).ok_or(Error::MissingReference)?;
            if a.0.context != q.context
                || b.0.context != q.context
                || r.0.context != q.context
                || a.0.from != q.from
                || b.0.to != q.to
            {
                continue;
            }
            let mut causes = self.reasons(a, q.epoch);
            causes.extend(self.reasons(b, q.epoch));
            causes.extend(self.rule_reasons(r, q.epoch));
            if causes.is_empty() {
                ans.proofs.push(*p);
            } else {
                ans.causes.extend(causes);
            }
        }
        // Detect conflict from current evidence, including opposing conclusion.
        for n in &self.facts {
            work.reads += 1;
            if n.0.positive || n.0.context != q.context || !self.reasons(n, q.epoch).is_empty() {
                continue;
            }
            let conclusion = n.0.from == q.from && n.0.to == q.to;
            let premise = ans.proofs.iter().any(|p| {
                [p.first, p.second].iter().any(|k| {
                    self.fact(*k)
                        .is_some_and(|a| a.0.from == n.0.from && a.0.to == n.0.to)
                })
            });
            if conclusion || premise {
                ans.conflicts.push(n.0.key);
                for f in &self.facts {
                    if f.0.positive
                        && f.0.context == q.context
                        && f.0.from == n.0.from
                        && f.0.to == n.0.to
                        && self.reasons(f, q.epoch).is_empty()
                    {
                        ans.conflicts.push(f.0.key);
                    }
                }
            }
        }
        ans.causes.sort();
        ans.causes.dedup();
        ans.conflicts.sort();
        ans.conflicts.dedup();
        ans.decision = if !ans.conflicts.is_empty() {
            Decision::Conflict
        } else if !ans.proofs.is_empty() {
            Decision::Supported
        } else {
            [
                Decision::Suspended,
                Decision::Retracted,
                Decision::Expired,
                Decision::Stale,
            ]
            .into_iter()
            .find(|r| ans.causes.contains(r))
            .unwrap_or(Decision::Unknown)
        };
        // Invalidated paths remain causes, but never evidence used for a response.
        for p in &ans.proofs {
            for k in [p.first, p.second] {
                let f = &self.fact(k).unwrap().0;
                ans.evidence.push((k, f.observation, f.source));
            }
        }
        for k in &ans.conflicts {
            let f = &self.fact(*k).unwrap().0;
            ans.evidence.push((*k, f.observation, f.source));
        }
        ans.evidence.sort();
        ans.evidence.dedup();
        // Conservative bound covers all linear searches and status scans above.
        work.reads += (self.facts.len() + self.rules.len() + self.sources.len() + 1).pow(3);
        if work.reads + work.pairs > 8192 {
            return Err(Error::Budget);
        }
        Ok((ans, work))
    }
}
impl Laboratory for Candidate {
    fn new() -> Self {
        Self {
            journal: Vec::new(),
            view: View {
                epoch: 1,
                ..View::default()
            },
        }
    }
    fn len(&self) -> usize {
        self.journal.len()
    }
    fn apply(&mut self, op: Op) -> Result<Work, Error> {
        if self.journal.len() >= 128 {
            return Err(Error::Capacity);
        }
        let mut next = self.view.clone();
        let mut w = Work::default();
        next.mutate(op, &mut w)?;
        self.view = next;
        self.journal.push(op);
        Ok(w)
    }
    fn query(&self, q: Query, cut: usize) -> Result<(Answer, Work), Error> {
        if cut > self.len() {
            return Err(Error::InvalidCut);
        }
        if cut == self.len() {
            return self.view.answer(q, cut);
        }
        let mut v = View {
            epoch: 1,
            ..View::default()
        };
        let mut total = Work::default();
        for op in &self.journal[..cut] {
            let mut w = Work::default();
            v.mutate(*op, &mut w)?;
            total.reads += w.reads;
            total.pairs += w.pairs;
        }
        let (ans, w) = v.answer(q, cut)?;
        total.reads += w.reads;
        total.pairs += w.pairs;
        if total.reads + total.pairs > 8192 {
            return Err(Error::Budget);
        }
        Ok((ans, total))
    }
    fn bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.journal.capacity() * std::mem::size_of::<Op>()
            + self.view.sources.capacity() * std::mem::size_of::<(Key, Status)>()
            + self.view.facts.capacity() * std::mem::size_of::<(Fact, Status)>()
            + self.view.rules.capacity() * std::mem::size_of::<(Rule, Status)>()
            + self.view.proofs.capacity() * std::mem::size_of::<Proof>()
    }
}
