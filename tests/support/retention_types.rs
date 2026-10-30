//! Passive DTOs for EC-05 only. No oracle, eligibility or decision code.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Context(pub u32, pub u16);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(pub u32, pub u16);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Kind {
    Source,
    Fact,
    Rule,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Active,
    Suspended,
    Retracted,
    Contested,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fact {
    pub key: Key,
    pub context: Context,
    pub from: u32,
    pub to: u32,
    pub source: u32,
    pub observation: u32,
    pub positive: bool,
    pub hypothesis: bool,
    pub start: u32,
    pub end: u32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rule {
    pub key: Key,
    pub context: Context,
    pub start: u32,
    pub end: u32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Grant {
    pub kind: Kind,
    pub key: Key,
    pub from: Context,
    pub to: Context,
    pub declaration: u32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Op {
    Context(Context, Option<Context>),
    Source(u32, Option<u32>),
    Fact(Fact),
    Rule(Rule),
    Grant(Grant),
    Set(Kind, Key, Status),
    Refresh,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Decision {
    Supported,
    Indeterminate,
    Conflict,
    Suspended,
    Retracted,
    Expired,
    Stale,
    Unknown,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Invalid,
    Duplicate,
    Terminal,
    InvalidCut,
    Capacity,
    Budget,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Query {
    pub context: Context,
    pub from: u32,
    pub to: u32,
    pub epoch: u32,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dependency {
    Fact(Key),
    Observation(u32),
    Source(u32),
    Rule(Key),
    Declaration(u32),
    Context(Context),
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Proof {
    pub context: Context,
    pub rule: Key,
    pub first: Key,
    pub second: Key,
    pub dependencies: Vec<Dependency>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Answer {
    pub query: Query,
    pub cut: usize,
    pub decision: Decision,
    pub causes: Vec<Decision>,
    pub proofs: Vec<Proof>,
    pub conflicts: Vec<Key>,
}
pub trait Executor: Clone + std::fmt::Debug + PartialEq {
    fn new() -> Self;
    fn apply(&mut self, op: Op) -> Result<usize, Error>;
    fn query(&self, q: Query, cut: usize) -> Result<(Answer, usize), Error>;
    fn cut(&self) -> usize;
    fn bytes(&self) -> usize;
}
