//! Candidate: materialized version space, independent of evaluator and controls.
use super::types::*;
#[derive(Clone, Debug, PartialEq)]
struct Context {
    id: u32,
    log: Vec<Episode>,
    start: usize,
    valid: Vec<(u8, u8)>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Candidate {
    contexts: Vec<Context>,
}
impl Candidate {
    pub fn declare(&mut self, id: u32) {
        assert!(id != 0 && self.contexts.len() < 2);
        let mut valid = Vec::new();
        for m in 1u16..=255 {
            if m.count_ones() <= 3 {
                for v in 0u16..=255 {
                    if v & !m == 0 {
                        valid.push((m as u8, v as u8));
                    }
                }
            }
        }
        self.contexts.push(Context {
            id,
            log: Vec::new(),
            start: 0,
            valid,
        });
    }
}
impl Learner for Candidate {
    fn new() -> Self {
        Self {
            contexts: Vec::new(),
        }
    }
    fn acquire(&mut self, id: u32, episode: Episode) -> Result<Work, &'static str> {
        if !episode.acquisition {
            return Err("evaluation episode");
        }
        let mut w = Work::default();
        let pos = self.contexts.iter().position(|x| {
            w.units += 1;
            x.id == id
        });
        if id == 0 {
            return Err("invalid context");
        }
        if pos.is_none() && self.contexts.len() >= 2 {
            return Err("capacity");
        }
        let mut c = pos.map(|i| self.contexts[i].clone()).unwrap_or(Context {
            id,
            log: Vec::new(),
            start: 0,
            valid: Vec::new(),
        });
        for previous in &c.log {
            w.units += 2;
            if previous.id == episode.id || previous.x == episode.x {
                return Err("duplicate");
            }
        }
        if episode.id == 0 || c.log.len() >= 48 {
            return Err("capacity");
        }
        c.log.push(episode);
        loop {
            let mut valid = Vec::new();
            for m in 1u16..=255 {
                if !(1..=3).contains(&m.count_ones()) {
                    continue;
                }
                for v in 0u16..=255 {
                    if v & !m != 0 {
                        continue;
                    }
                    let mut ok = true;
                    for e in &c.log[c.start..] {
                        w.units += 2;
                        if w.units > 4_000_000 {
                            return Err("budget");
                        };
                        if ((u16::from(e.x) & m) == v) != e.label {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        valid.push((m as u8, v as u8));
                    }
                }
            }
            if !valid.is_empty() {
                c.valid = valid;
                break;
            }
            c.start += 1;
        }
        if let Some(i) = pos {
            self.contexts[i] = c
        } else {
            self.contexts.push(c)
        }
        Ok(w)
    }
    fn query(&self, id: u32, x: u8) -> Result<(Answer, Work), &'static str> {
        let mut w = Work::default();
        let c = self.contexts.iter().find(|v| {
            w.units += 1;
            v.id == id
        });
        let Some(c) = c else {
            return Ok((
                Answer {
                    outcome: Outcome::Missing,
                    version: 0,
                    start: 0,
                    end: 0,
                    hypotheses: 0,
                },
                w,
            ));
        };
        let mut yes = false;
        let mut no = false;
        for &(m, v) in &c.valid {
            w.units += 1;
            if w.units > 8192 {
                return Err("budget");
            };
            if x & m == v {
                yes = true
            } else {
                no = true
            }
        }
        let outcome = match (yes, no) {
            (true, false) => Outcome::Positive,
            (false, true) => Outcome::Negative,
            (true, true) => Outcome::Ambiguous,
            _ => Outcome::Inconclusive,
        };
        Ok((
            Answer {
                outcome,
                version: c.log.len(),
                start: c.start,
                end: c.log.len(),
                hypotheses: c.valid.len(),
            },
            w,
        ))
    }
}
