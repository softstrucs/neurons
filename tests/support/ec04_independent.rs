//! Independent induction: exhaustive suffix scan with fresh hypothesis enumeration.
use super::types::*;
#[derive(Clone, Debug, PartialEq)]
struct Row {
    context: u32,
    events: Vec<Episode>,
    start: usize,
    valid: Vec<(u8, u8)>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Independent {
    rows: Vec<Row>,
}
impl Independent {
    pub fn declare(&mut self, id: u32) {
        assert!(id != 0 && self.rows.len() < 2);
        self.rows.push(Row {
            context: id,
            events: Vec::new(),
            start: 0,
            valid: Self::compatible(&[], &mut Work::default()).unwrap(),
        });
    }

    fn compatible(events: &[Episode], w: &mut Work) -> Result<Vec<(u8, u8)>, &'static str> {
        let mut result = Vec::new();
        for mask in (1u16..=255).rev() {
            if mask.count_ones() > 3 {
                continue;
            }
            for value in (0u16..=255).rev() {
                if value & !mask != 0 {
                    continue;
                }
                let mut good = true;
                for e in events {
                    w.units += 2;
                    if w.units > 4_000_000 {
                        return Err("budget");
                    };
                    if (u16::from(e.x) & mask == value) != e.label {
                        good = false;
                        break;
                    }
                }
                if good {
                    result.push((mask as u8, value as u8))
                }
            }
        }
        result.sort();
        Ok(result)
    }
    fn suffix(events: &[Episode], w: &mut Work) -> Result<(usize, Vec<(u8, u8)>), &'static str> {
        for start in 0..=events.len() {
            let h = Self::compatible(&events[start..], w)?;
            if !h.is_empty() {
                return Ok((start, h));
            }
        }
        Err("invalid state")
    }
}
impl Learner for Independent {
    fn new() -> Self {
        Self { rows: Vec::new() }
    }
    fn acquire(&mut self, id: u32, e: Episode) -> Result<Work, &'static str> {
        if !e.acquisition {
            return Err("evaluation episode");
        }
        let mut w = Work::default();
        if id == 0 || e.id == 0 {
            return Err("invalid");
        }
        let found = self.rows.iter().position(|r| {
            w.units += 1;
            r.context == id
        });
        if found.is_none() && self.rows.len() >= 2 {
            return Err("capacity");
        }
        let mut next = found
            .map(|i| self.rows[i].events.clone())
            .unwrap_or_default();
        if next.len() >= 48 {
            return Err("capacity");
        }
        for old in &next {
            w.units += 2;
            if old.id == e.id || old.x == e.x {
                return Err("duplicate");
            }
        }
        next.push(e);
        let (start, valid) = Self::suffix(&next, &mut w)?;
        if let Some(i) = found {
            self.rows[i].events = next;
            self.rows[i].start = start;
            self.rows[i].valid = valid
        } else {
            self.rows.push(Row {
                context: id,
                events: next,
                start,
                valid,
            })
        }
        Ok(w)
    }
    fn query(&self, id: u32, x: u8) -> Result<(Answer, Work), &'static str> {
        let mut w = Work::default();
        let row = self.rows.iter().find(|r| {
            w.units += 1;
            r.context == id
        });
        let Some(row) = row else {
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
        let start = row.start;
        let h = &row.valid;
        let mut positives = 0usize;
        for &(m, v) in h {
            w.units += 1;
            if x & m == v {
                positives += 1
            }
        }
        if w.units > 8192 {
            return Err("budget");
        }
        let outcome = if positives == 0 {
            Outcome::Negative
        } else if positives == h.len() {
            Outcome::Positive
        } else {
            Outcome::Ambiguous
        };
        Ok((
            Answer {
                outcome,
                version: row.events.len(),
                start,
                end: row.events.len(),
                hypotheses: h.len(),
            },
            w,
        ))
    }
}
