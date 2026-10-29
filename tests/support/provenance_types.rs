//! Passive experimental DTOs only; no validation, inference or oracle here.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(pub u32, pub u16);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Kind {
    Source,
    Fact,
    Rule,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Status {
    Active,
    Suspended,
    Retracted,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fact {
    pub key: Key,
    pub observation: u32,
    pub source: Key,
    pub context: u32,
    pub from: u32,
    pub to: u32,
    pub positive: bool,
    pub hypothesis: bool,
    pub observed: u32,
    pub start: u32,
    pub end: u32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rule {
    pub key: Key,
    pub context: u32,
    pub start: u32,
    pub end: u32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Proof {
    pub rule: Key,
    pub first: Key,
    pub second: Key,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: u32,
    pub deps: [u32; 2],
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Op {
    Source(Key),
    Fact(Fact),
    Revise(Fact),
    Rule(Rule),
    Set(Kind, Key, Status),
    Epoch(u32),
    Refresh,
    Associate([Proposal; 2], usize),
    Schema(u32),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    MissingSource,
    Duplicate,
    InvalidInterval,
    MissingReference,
    Cycle,
    TerminalRetraction,
    InvalidCut,
    Capacity,
    UnsupportedVersion,
    InvalidInput,
    Budget,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Decision {
    Supported,
    Conflict,
    Suspended,
    Retracted,
    Expired,
    Stale,
    Unknown,
    Unavailable,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Query {
    pub from: u32,
    pub to: u32,
    pub context: u32,
    pub epoch: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Answer {
    pub decision: Decision,
    pub causes: Vec<Decision>,
    pub proofs: Vec<Proof>,
    pub evidence: Vec<(Key, u32, Key)>,
    pub conflicts: Vec<Key>,
    pub cut: usize,
    pub epoch: u32,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Work {
    pub reads: usize,
    pub pairs: usize,
}
// Interface serves only orchestration. Each executor implements its own logic.
pub trait Laboratory: Clone + std::fmt::Debug + PartialEq {
    fn new() -> Self;
    fn apply(&mut self, op: Op) -> Result<Work, Error>;
    fn query(&self, query: Query, cut: usize) -> Result<(Answer, Work), Error>;
    fn len(&self) -> usize;
    fn bytes(&self) -> usize;
}
