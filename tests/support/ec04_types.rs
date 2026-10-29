//! Passive data types shared across experimental executors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Episode {
    pub id: u32,
    pub acquisition: bool,
    pub x: u8,
    pub label: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    Positive,
    Negative,
    Ambiguous,
    Missing,
    Inconclusive,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Answer {
    pub outcome: Outcome,
    pub version: usize,
    pub start: usize,
    pub end: usize,
    pub hypotheses: usize,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Work {
    pub units: usize,
}
pub trait Learner: Clone + PartialEq {
    fn new() -> Self;
    fn acquire(&mut self, context: u32, episode: Episode) -> Result<Work, &'static str>;
    fn query(&self, context: u32, x: u8) -> Result<(Answer, Work), &'static str>;
}
