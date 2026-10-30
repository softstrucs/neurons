use super::retention_types::*;
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Candidate {
    log: Vec<Op>,
    contexts: Vec<(Context, Option<Context>)>,
    sources: Vec<(u32, Option<u32>, Status)>,
    facts: Vec<(Fact, Status)>,
    rules: Vec<(Rule, Status)>,
    grants: Vec<Grant>,
    cache: Vec<(Context, Key, Key, Key)>,
}
impl Candidate {
    fn applicable(
        &self,
        kind: Kind,
        key: Key,
        origin: Context,
        target: Context,
    ) -> Option<Vec<Dependency>> {
        if origin == target {
            return Some(Vec::new());
        }
        let mut current = origin;
        let mut used = Vec::new();
        while current.1 < target.1 && current.0 == target.0 {
            let next = Context(current.0, current.1 + 1);
            let g = self
                .grants
                .iter()
                .find(|g| g.kind == kind && g.key == key && g.from == current && g.to == next)?;
            used.push(Dependency::Declaration(g.declaration));
            current = next;
        }
        (current == target).then_some(used)
    }
    fn source(&self, id: u32, deps: &mut Vec<Dependency>, bad: &mut Vec<Decision>) {
        let mut cursor = id;
        for _ in 0..8 {
            let Some((_, parent, status)) = self.sources.iter().find(|s| s.0 == cursor) else {
                bad.push(Decision::Indeterminate);
                return;
            };
            deps.push(Dependency::Source(cursor));
            Self::status(*status, bad);
            match parent {
                Some(p) if *p == cursor => return,
                Some(p) => cursor = *p,
                None => {
                    bad.push(Decision::Indeterminate);
                    return;
                }
            }
        }
        bad.push(Decision::Indeterminate);
    }
    fn status(s: Status, bad: &mut Vec<Decision>) {
        match s {
            Status::Active => (),
            Status::Suspended => bad.push(Decision::Suspended),
            Status::Retracted => bad.push(Decision::Retracted),
            Status::Contested => bad.push(Decision::Conflict),
        }
    }
    fn fact(
        &self,
        f: Fact,
        state: Status,
        epoch: u32,
        deps: &mut Vec<Dependency>,
    ) -> Vec<Decision> {
        let mut bad = Vec::new();
        deps.extend([
            Dependency::Fact(f.key),
            Dependency::Observation(f.observation),
        ]);
        if self
            .facts
            .iter()
            .any(|x| x.0.key.0 == f.key.0 && x.0.key.1 > f.key.1)
        {
            bad.push(Decision::Stale);
        }
        Self::status(state, &mut bad);
        if epoch < f.start || epoch >= f.end {
            bad.push(Decision::Expired);
        }
        self.source(f.source, deps, &mut bad);
        bad
    }
    fn mutate(&mut self, op: Op) -> Result<usize, Error> {
        let mut work = 1;
        match op {
            Op::Context(c, p) => {
                if self.contexts.iter().any(|x| x.0 == c) {
                    return Err(Error::Duplicate);
                }
                if c.0 == 0
                    || c.1 == 0
                    || (c.1 == 1 && p.is_some())
                    || (c.1 > 1
                        && (p != Some(Context(c.0, c.1 - 1))
                            || !self.contexts.iter().any(|x| Some(x.0) == p)))
                {
                    return Err(Error::Invalid);
                }
                if self.contexts.len() >= 8 {
                    return Err(Error::Capacity);
                }
                self.contexts.push((c, p));
            }
            Op::Source(id, parent) => {
                if self.sources.iter().any(|s| s.0 == id) {
                    return Err(Error::Duplicate);
                }
                if id == 0
                    || parent.is_some_and(|p| p != id && !self.sources.iter().any(|s| s.0 == p))
                {
                    return Err(Error::Invalid);
                }
                if self.sources.len() >= 8 {
                    return Err(Error::Capacity);
                }
                self.sources.push((id, parent, Status::Active));
            }
            Op::Fact(f) => {
                if self.facts.iter().any(|x| x.0.key == f.key) {
                    return Err(Error::Duplicate);
                }
                let last = self
                    .facts
                    .iter()
                    .filter(|x| x.0.key.0 == f.key.0)
                    .map(|x| x.0)
                    .max_by_key(|x| x.key.1);
                if f.key.0 == 0
                    || f.key.1 == 0
                    || f.from == 0
                    || f.to == 0
                    || f.observation == 0
                    || f.start >= f.end
                    || !self.contexts.iter().any(|x| x.0 == f.context)
                    || !self
                        .sources
                        .iter()
                        .any(|s| s.0 == f.source && s.2 == Status::Active)
                    || match last {
                        None => f.key.1 != 1,
                        Some(p) => {
                            f.key.1 != p.key.1 + 1
                                || f.context != p.context
                                || f.observation == p.observation
                        }
                    }
                {
                    return Err(Error::Invalid);
                }
                if self.facts.len() >= 64 {
                    return Err(Error::Capacity);
                }
                self.facts.push((f, Status::Active));
            }
            Op::Rule(r) => {
                if self.rules.iter().any(|x| x.0.key.0 == r.key.0) {
                    return Err(Error::Duplicate);
                }
                if r.key.0 == 0
                    || r.key.1 != 1
                    || r.start >= r.end
                    || !self.contexts.iter().any(|x| x.0 == r.context)
                {
                    return Err(Error::Invalid);
                }
                if self.rules.len() >= 4 {
                    return Err(Error::Capacity);
                }
                self.rules.push((r, Status::Active));
            }
            Op::Grant(g) => {
                let origin = match g.kind {
                    Kind::Fact => self
                        .facts
                        .iter()
                        .find(|x| x.0.key == g.key)
                        .map(|x| x.0.context),
                    Kind::Rule => self
                        .rules
                        .iter()
                        .find(|x| x.0.key == g.key)
                        .map(|x| x.0.context),
                    Kind::Source => None,
                }
                .ok_or(Error::Invalid)?;
                if g.declaration == 0
                    || !self.contexts.contains(&(g.to, Some(g.from)))
                    || self.applicable(g.kind, g.key, origin, g.from).is_none()
                {
                    return Err(Error::Invalid);
                }
                if self.grants.iter().any(|x| {
                    x.declaration == g.declaration
                        || (x.kind == g.kind && x.key == g.key && x.to == g.to)
                }) {
                    return Err(Error::Duplicate);
                }
                if self.grants.len() >= 128 {
                    return Err(Error::Capacity);
                }
                self.grants.push(g);
            }
            Op::Set(kind, key, status) => {
                let state = match kind {
                    Kind::Source => self
                        .sources
                        .iter_mut()
                        .find(|s| s.0 == key.0 && key.1 == 1)
                        .map(|s| &mut s.2),
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
                .ok_or(Error::Invalid)?;
                if *state == Status::Retracted {
                    return Err(Error::Terminal);
                }
                *state = status;
            }
            Op::Refresh => {
                for (c, _) in &self.contexts {
                    for (r, _) in &self.rules {
                        if self.applicable(Kind::Rule, r.key, r.context, *c).is_none() {
                            continue;
                        }
                        for (a, _) in &self.facts {
                            for (b, _) in &self.facts {
                                work += 1;
                                if work > 8192 {
                                    return Err(Error::Budget);
                                }
                                if a.positive
                                    && b.positive
                                    && !a.hypothesis
                                    && !b.hypothesis
                                    && a.to == b.from
                                    && a.from != b.to
                                    && a.from != a.to
                                    && b.from != b.to
                                    && self.applicable(Kind::Fact, a.key, a.context, *c).is_some()
                                    && self.applicable(Kind::Fact, b.key, b.context, *c).is_some()
                                {
                                    let p = (*c, r.key, a.key, b.key);
                                    if !self.cache.contains(&p) {
                                        self.cache.push(p);
                                    }
                                }
                            }
                        }
                    }
                }
                if self.cache.len() > 64 {
                    return Err(Error::Capacity);
                }
            }
        }
        Ok(work)
    }
    fn answer(&self, q: Query, cut: usize) -> Result<(Answer, usize), Error> {
        if q.from == 0 || q.to == 0 || !self.contexts.iter().any(|x| x.0 == q.context) {
            return Err(Error::Invalid);
        }
        let mut work = 1;
        let mut negatives = Vec::new();
        let mut tainted = Vec::new();
        let mut conflicts = Vec::new();
        let mut direct = false;
        for (f, s) in &self.facts {
            work += 1;
            if f.positive
                || f.hypothesis
                || self
                    .applicable(Kind::Fact, f.key, f.context, q.context)
                    .is_none()
                || !self.fact(*f, *s, q.epoch, &mut Vec::new()).is_empty()
            {
                continue;
            }
            negatives.push(*f);
            if f.from == q.from && f.to == q.to {
                direct = true;
                conflicts.push(f.key);
            }
            for (a, t) in &self.facts {
                work += 1;
                if a.positive
                    && !a.hypothesis
                    && a.from == f.from
                    && a.to == f.to
                    && self
                        .applicable(Kind::Fact, a.key, a.context, q.context)
                        .is_some()
                    && self.fact(*a, *t, q.epoch, &mut Vec::new()).is_empty()
                {
                    tainted.push(a.observation);
                    conflicts.extend([a.key, f.key]);
                }
            }
        }
        let mut causes = Vec::new();
        let mut proofs = Vec::new();
        for (c, r, a, b) in &self.cache {
            work += 1;
            if work > 8192 {
                return Err(Error::Budget);
            }
            if *c != q.context {
                continue;
            }
            let (af, ast) = self.facts.iter().find(|x| x.0.key == *a).unwrap();
            let (bf, bst) = self.facts.iter().find(|x| x.0.key == *b).unwrap();
            let (rf, rst) = self.rules.iter().find(|x| x.0.key == *r).unwrap();
            if af.from != q.from || bf.to != q.to {
                continue;
            }
            let mut deps = vec![Dependency::Context(*c), Dependency::Rule(*r)];
            let mut bad = self.fact(*af, *ast, q.epoch, &mut deps);
            bad.extend(self.fact(*bf, *bst, q.epoch, &mut deps));
            Self::status(*rst, &mut bad);
            if q.epoch < rf.start || q.epoch >= rf.end {
                bad.push(Decision::Expired);
            }
            for (kind, key, origin) in [
                (Kind::Fact, *a, af.context),
                (Kind::Fact, *b, bf.context),
                (Kind::Rule, *r, rf.context),
            ] {
                match self.applicable(kind, key, origin, *c) {
                    Some(d) => deps.extend(d),
                    None => bad.push(Decision::Unknown),
                }
            }
            if tainted.contains(&af.observation) || tainted.contains(&bf.observation) {
                bad.push(Decision::Conflict);
            }
            if bad.is_empty() {
                deps.sort();
                deps.dedup();
                proofs.push(Proof {
                    context: *c,
                    rule: *r,
                    first: *a,
                    second: *b,
                    dependencies: deps,
                });
            } else {
                causes.extend(bad);
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
        let decision = if direct {
            Decision::Conflict
        } else if causes.contains(&Decision::Indeterminate) && causes.contains(&Decision::Conflict)
        {
            Decision::Indeterminate
        } else if !proofs.is_empty() {
            Decision::Supported
        } else {
            [
                Decision::Indeterminate,
                Decision::Conflict,
                Decision::Suspended,
                Decision::Retracted,
                Decision::Expired,
                Decision::Stale,
            ]
            .into_iter()
            .find(|d| causes.contains(d))
            .unwrap_or(Decision::Unknown)
        };
        if decision != Decision::Supported {
            proofs.clear();
        }
        Ok((
            Answer {
                query: q,
                cut,
                decision,
                causes,
                proofs,
                conflicts,
            },
            work,
        ))
    }
}
impl Executor for Candidate {
    fn new() -> Self {
        Self::default()
    }
    fn apply(&mut self, op: Op) -> Result<usize, Error> {
        if self.log.len() >= 128 {
            return Err(Error::Capacity);
        }
        let mut next = self.clone();
        let w = next.mutate(op)?;
        next.log.push(op);
        *self = next;
        Ok(w)
    }
    fn query(&self, q: Query, cut: usize) -> Result<(Answer, usize), Error> {
        if cut > self.log.len() {
            return Err(Error::InvalidCut);
        }
        if cut == self.log.len() {
            return self.answer(q, cut);
        }
        let mut old = Self::new();
        let mut work = 0;
        for op in &self.log[..cut] {
            work += old.apply(*op)?;
        }
        let (a, w) = old.answer(q, cut)?;
        if work + w > 8192 {
            return Err(Error::Budget);
        }
        Ok((a, work + w))
    }
    fn cut(&self) -> usize {
        self.log.len()
    }
    fn bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.log.capacity() * std::mem::size_of::<Op>()
            + self.contexts.capacity() * std::mem::size_of::<(Context, Option<Context>)>()
            + self.sources.capacity() * std::mem::size_of::<(u32, Option<u32>, Status)>()
            + self.facts.capacity() * std::mem::size_of::<(Fact, Status)>()
            + self.rules.capacity() * std::mem::size_of::<(Rule, Status)>()
            + self.grants.capacity() * std::mem::size_of::<Grant>()
            + self.cache.capacity() * std::mem::size_of::<(Context, Key, Key, Key)>()
    }
}
