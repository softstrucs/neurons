//! Literal control: scans the accepted journal and recomposes from live evidence.
//! Own admission, graph validation and inference; no candidate/symbolic calls.
use super::types::*;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal {
    diary: Vec<Op>,
}
impl Literal {
    fn facts(log: &[Op]) -> Vec<Fact> {
        log.iter()
            .filter_map(|o| match o {
                Op::Fact(f) | Op::Revise(f) => Some(*f),
                _ => None,
            })
            .collect()
    }
    fn rules(log: &[Op]) -> Vec<Rule> {
        log.iter()
            .filter_map(|o| if let Op::Rule(r) = o { Some(*r) } else { None })
            .collect()
    }
    fn status(log: &[Op], kind: Kind, k: Key, w: &mut Work) -> Option<Status> {
        w.reads += log.len();
        let mut result = None;
        for op in log {
            match *op {
                Op::Source(x) if kind == Kind::Source && x == k => result = Some(Status::Active),
                Op::Fact(f) | Op::Revise(f) if kind == Kind::Fact && f.key == k => {
                    result = Some(Status::Active)
                }
                Op::Rule(r) if kind == Kind::Rule && r.key == k => result = Some(Status::Active),
                Op::Set(t, x, s) if t == kind && x == k => result = Some(s),
                _ => {}
            }
        }
        result
    }
    fn fact_bad(log: &[Op], f: Fact, epoch: u32, w: &mut Work) -> Vec<Decision> {
        let mut r = Vec::new();
        for status in [
            Self::status(log, Kind::Fact, f.key, w),
            Self::status(log, Kind::Source, f.source, w),
        ] {
            match status {
                Some(Status::Suspended) => r.push(Decision::Suspended),
                Some(Status::Retracted) => r.push(Decision::Retracted),
                None => r.push(Decision::Unknown),
                _ => {}
            }
        }
        if epoch < f.start || epoch >= f.end {
            r.push(Decision::Expired)
        }
        w.reads += log.len();
        if log
            .iter()
            .any(|o| matches!(o,Op::Fact(x)|Op::Revise(x) if x.key.0==f.key.0&&x.key.1>f.key.1))
        {
            r.push(Decision::Stale)
        }
        if f.hypothesis {
            r.push(Decision::Unknown)
        }
        r
    }
    fn rule_bad(log: &[Op], r: Rule, epoch: u32, w: &mut Work) -> Vec<Decision> {
        let mut v = Vec::new();
        match Self::status(log, Kind::Rule, r.key, w) {
            Some(Status::Suspended) => v.push(Decision::Suspended),
            Some(Status::Retracted) => v.push(Decision::Retracted),
            None => v.push(Decision::Unknown),
            _ => {}
        }
        if epoch < r.start || epoch >= r.end {
            v.push(Decision::Expired)
        }
        w.reads += log.len();
        if log
            .iter()
            .any(|o| matches!(o,Op::Rule(x) if x.key.0==r.key.0&&x.key.1>r.key.1))
        {
            v.push(Decision::Stale)
        }
        v
    }
    fn compose(log: &[Op], epoch: u32, w: &mut Work) -> Vec<Proof> {
        w.reads += log.len() * 2;
        let live: Vec<_> = Self::facts(log)
            .into_iter()
            .filter(|f| f.positive && Self::fact_bad(log, *f, epoch, w).is_empty())
            .collect();
        let mut proofs = Vec::new();
        for r in Self::rules(log) {
            if !Self::rule_bad(log, r, epoch, w).is_empty() {
                continue;
            }
            for left in &live {
                for right in &live {
                    w.pairs += 1;
                    w.reads += 3;
                    if left.context == r.context
                        && right.context == r.context
                        && left.to == right.from
                        && left.from != left.to
                        && right.from != right.to
                        && left.from != right.to
                    {
                        proofs.push(Proof {
                            rule: r.key,
                            first: left.key,
                            second: right.key,
                        });
                    }
                }
            }
        }
        proofs.sort();
        proofs.dedup();
        proofs
    }
    fn historical_proofs(log: &[Op], w: &mut Work) -> Vec<Proof> {
        let mut epoch = 1;
        let mut out = Vec::new();
        for (i, op) in log.iter().enumerate() {
            w.reads += 1;
            match op {
                Op::Epoch(e) => epoch = *e,
                Op::Refresh => out.extend(Self::compose(&log[..i], epoch, w)),
                _ => {}
            }
        }
        out.sort();
        out.dedup();
        out
    }
    fn accept(&self, op: Op) -> Result<Work, Error> {
        let log = &self.diary;
        let mut w = Work {
            reads: log.len(),
            pairs: 0,
        };
        let fs = Self::facts(log);
        let rs = Self::rules(log);
        let epoch = log
            .iter()
            .rev()
            .find_map(|o| if let Op::Epoch(e) = o { Some(*e) } else { None })
            .unwrap_or(1);
        if log.len() >= 128 {
            return Err(Error::Capacity);
        }
        match op {
            Op::Source(k) => {
                if k.0 == 0 || k.1 != 1 {
                    return Err(Error::InvalidInput);
                }
                if log.contains(&Op::Source(k)) {
                    return Err(Error::Duplicate);
                }
                if log.iter().filter(|o| matches!(o, Op::Source(_))).count() >= 8 {
                    return Err(Error::Capacity);
                }
            }
            Op::Fact(f) | Op::Revise(f) => {
                if Self::status(log, Kind::Source, f.source, &mut w) != Some(Status::Active) {
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
                    || f.observed != epoch
                {
                    return Err(Error::InvalidInput);
                }
                if fs
                    .iter()
                    .any(|a| a.key == f.key || a.observation == f.observation)
                {
                    return Err(Error::Duplicate);
                }
                let last = fs
                    .iter()
                    .filter(|a| a.key.0 == f.key.0)
                    .map(|a| a.key.1)
                    .max()
                    .unwrap_or(0);
                if f.key.1 != last + 1 {
                    return Err(Error::InvalidInput);
                }
                if matches!(op, Op::Revise(_)) && last == 0 {
                    return Err(Error::MissingReference);
                }
                // Representation count includes previously materialized proofs, not virtual queries.
                let proofs = Self::historical_proofs(log, &mut w);
                if fs.len() + proofs.len() >= 64 || fs.len() >= 64 {
                    return Err(Error::Capacity);
                }
            }
            Op::Rule(r) => {
                if r.start >= r.end {
                    return Err(Error::InvalidInterval);
                }
                if rs.iter().any(|x| x.key == r.key) {
                    return Err(Error::Duplicate);
                }
                if r.key.0 == 0 || r.key.1 == 0 || r.context == 0 {
                    return Err(Error::InvalidInput);
                }
                if rs.len() >= 4 {
                    return Err(Error::Capacity);
                }
            }
            Op::Set(t, k, s) => {
                let old = Self::status(log, t, k, &mut w).ok_or(Error::MissingReference)?;
                if old == Status::Retracted {
                    return Err(Error::TerminalRetraction);
                }
                if s == Status::Active && old != Status::Suspended {
                    return Err(Error::InvalidInput);
                }
            }
            Op::Epoch(e) => {
                if e < epoch {
                    return Err(Error::InvalidInput);
                }
            }
            Op::Refresh => {
                let mut ps = Self::historical_proofs(log, &mut w);
                ps.extend(Self::compose(log, epoch, &mut w));
                ps.sort();
                ps.dedup();
                if fs.len() + ps.len() > 64 || ps.len() * 2 > 128 {
                    return Err(Error::Capacity);
                }
            }
            Op::Associate(rows, n) => {
                if n > 2 {
                    return Err(Error::InvalidInput);
                }
                let mut edges = Vec::new();
                for row in &rows[..n] {
                    for dep in row.deps {
                        if !fs.iter().any(|f| f.key.0 == dep)
                            && !rows[..n].iter().any(|r| r.id == dep)
                        {
                            return Err(Error::MissingReference);
                        }
                        edges.push((row.id, dep));
                    }
                }
                let mut reachable = edges.clone();
                for _ in 0..n {
                    for &(a, b) in &edges {
                        for &(c, d) in &reachable.clone() {
                            w.pairs += 1;
                            if b == c && !reachable.contains(&(a, d)) {
                                reachable.push((a, d));
                            }
                        }
                    }
                }
                if reachable.iter().any(|(a, b)| a == b) {
                    return Err(Error::Cycle);
                }
                return Err(Error::InvalidInput);
            }
            Op::Schema(v) => {
                if v != 1 {
                    return Err(Error::UnsupportedVersion);
                }
            }
        }
        if w.reads + w.pairs > 8192 {
            return Err(Error::Budget);
        }
        Ok(w)
    }
}
impl Laboratory for Literal {
    fn new() -> Self {
        Self { diary: Vec::new() }
    }
    fn len(&self) -> usize {
        self.diary.len()
    }
    fn apply(&mut self, op: Op) -> Result<Work, Error> {
        let w = self.accept(op)?;
        self.diary.push(op);
        Ok(w)
    }
    fn query(&self, q: Query, cut: usize) -> Result<(Answer, Work), Error> {
        if cut > self.len() {
            return Err(Error::InvalidCut);
        }
        if q.context == 0 || q.from == 0 || q.to == 0 {
            return Err(Error::InvalidInput);
        }
        let log = &self.diary[..cut];
        let mut w = Work::default();
        let facts = Self::facts(log);
        let rules = Self::rules(log);
        let old = Self::historical_proofs(log, &mut w);
        let current = Self::compose(log, q.epoch, &mut w);
        let mut out = Answer {
            decision: Decision::Unknown,
            causes: Vec::new(),
            proofs: Vec::new(),
            evidence: Vec::new(),
            conflicts: Vec::new(),
            cut,
            epoch: q.epoch,
        };
        for p in old {
            let a = *facts
                .iter()
                .find(|a| a.key == p.first)
                .ok_or(Error::MissingReference)?;
            let b = *facts
                .iter()
                .find(|a| a.key == p.second)
                .ok_or(Error::MissingReference)?;
            let r = *rules
                .iter()
                .find(|r| r.key == p.rule)
                .ok_or(Error::MissingReference)?;
            if a.from == q.from
                && b.to == q.to
                && a.context == q.context
                && b.context == q.context
                && r.context == q.context
            {
                out.causes.extend(Self::fact_bad(log, a, q.epoch, &mut w));
                out.causes.extend(Self::fact_bad(log, b, q.epoch, &mut w));
                out.causes.extend(Self::rule_bad(log, r, q.epoch, &mut w));
            }
        }
        for p in current {
            let a = facts.iter().find(|f| f.key == p.first).unwrap();
            let b = facts.iter().find(|f| f.key == p.second).unwrap();
            if a.from == q.from && b.to == q.to && a.context == q.context {
                out.proofs.push(p)
            }
        }
        for f in &facts {
            if f.positive
                || f.context != q.context
                || !Self::fact_bad(log, *f, q.epoch, &mut w).is_empty()
            {
                continue;
            }
            let direct = f.from == q.from && f.to == q.to;
            let affects = out.proofs.iter().any(|p| {
                [p.first, p.second].iter().any(|key| {
                    facts
                        .iter()
                        .any(|a| a.key == *key && a.from == f.from && a.to == f.to)
                })
            });
            if direct || affects {
                out.conflicts.push(f.key);
                for a in &facts {
                    if a.positive
                        && a.context == q.context
                        && a.from == f.from
                        && a.to == f.to
                        && Self::fact_bad(log, *a, q.epoch, &mut w).is_empty()
                    {
                        out.conflicts.push(a.key)
                    }
                }
            }
        }
        out.causes.sort();
        out.causes.dedup();
        out.conflicts.sort();
        out.conflicts.dedup();
        out.decision = if !out.conflicts.is_empty() {
            Decision::Conflict
        } else if !out.proofs.is_empty() {
            Decision::Supported
        } else {
            let mut result = Decision::Unknown;
            for d in [
                Decision::Stale,
                Decision::Expired,
                Decision::Retracted,
                Decision::Suspended,
            ] {
                if out.causes.contains(&d) {
                    result = d
                }
            }
            result
        };
        for p in &out.proofs {
            for key in [p.first, p.second] {
                let f = facts.iter().find(|f| f.key == key).unwrap();
                out.evidence.push((key, f.observation, f.source))
            }
        }
        for key in &out.conflicts {
            let f = facts.iter().find(|f| f.key == *key).unwrap();
            out.evidence.push((*key, f.observation, f.source))
        }
        out.evidence.sort();
        out.evidence.dedup();
        // Charge an additional bound for vector lookups in proof/evidence assembly.
        w.reads += (facts.len() + rules.len() + 1).pow(2) * 4;
        if w.reads + w.pairs > 8192 {
            return Err(Error::Budget);
        }
        Ok((out, w))
    }
    fn bytes(&self) -> usize {
        std::mem::size_of::<Self>() + self.diary.capacity() * std::mem::size_of::<Op>() + 64 * 1024
    }
}
