//! Experimental controls. No generator, target or evaluator imports.
use super::{candidate::Candidate, independent::Independent, types::*};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Condition {
    Candidate,
    Literal,
    AlwaysPositive,
    AlwaysNegative,
    AlwaysAbstain,
    NoAcquisition,
    NoUpdate,
    Independent,
    FixedP7,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Control {
    pub condition: Condition,
    candidate: Candidate,
    independent: Independent,
    episodes: Vec<(u32, Episode)>,
}
impl Control {
    pub fn declare(&mut self, context: u32) {
        match self.condition {
            Condition::Candidate | Condition::NoUpdate => self.candidate.declare(context),
            Condition::Independent => self.independent.declare(context),
            _ => {}
        }
    }

    pub fn new(condition: Condition) -> Self {
        Self {
            condition,
            candidate: Candidate::new(),
            independent: Independent::new(),
            episodes: Vec::new(),
        }
    }
    /// Explicit experimental branch from the already acquired candidate, not an oracle.
    pub fn frozen(candidate: &Self) -> Self {
        assert_eq!(candidate.condition, Condition::Candidate);
        Self {
            condition: Condition::NoUpdate,
            candidate: candidate.candidate.clone(),
            independent: Independent::new(),
            episodes: Vec::new(),
        }
    }
    pub fn acquire(&mut self, context: u32, e: Episode) -> Result<Work, &'static str> {
        if !e.acquisition {
            return Err("evaluation episode");
        }
        match self.condition {
            Condition::Candidate => self.candidate.acquire(context, e),
            Condition::Independent => self.independent.acquire(context, e),
            Condition::NoAcquisition => Ok(Work::default()),
            Condition::Literal | Condition::NoUpdate => {
                let mut w = Work::default();
                for (c, old) in &self.episodes {
                    w.units += 3;
                    if *c == context && (old.id == e.id || old.x == e.x) {
                        return Err("duplicate");
                    }
                }
                if context == 0 || e.id == 0 {
                    return Err("invalid");
                }
                if self.episodes.len() >= 80 {
                    return Err("capacity");
                }
                self.episodes.push((context, e));
                Ok(w)
            }
            _ => Ok(Work::default()),
        }
    }
    pub fn query(&self, context: u32, x: u8) -> Result<(Answer, Work), &'static str> {
        match self.condition {
            Condition::Candidate => return self.candidate.query(context, x),
            Condition::Independent => return self.independent.query(context, x),
            Condition::NoUpdate => return self.candidate.query(context, x),
            _ => {}
        }
        let mut w = Work::default();
        let outcome = match self.condition {
            Condition::Literal => {
                let mut found = None;
                for (c, e) in &self.episodes {
                    w.units += 2;
                    if *c == context && e.x == x {
                        found = Some(e.label)
                    }
                }
                match found {
                    Some(true) => Outcome::Positive,
                    Some(false) => Outcome::Negative,
                    None => Outcome::Missing,
                }
            }
            Condition::AlwaysPositive => Outcome::Positive,
            Condition::AlwaysNegative => Outcome::Negative,
            Condition::FixedP7 => {
                w.units += 1;
                if x & 128 != 0 {
                    Outcome::Positive
                } else {
                    Outcome::Negative
                }
            }
            Condition::NoAcquisition | Condition::AlwaysAbstain => Outcome::Ambiguous,
            _ => unreachable!(),
        };
        if w.units > 8192 {
            return Err("budget");
        }
        Ok((
            Answer {
                outcome,
                version: 0,
                start: 0,
                end: 0,
                hypotheses: 0,
            },
            w,
        ))
    }
}
