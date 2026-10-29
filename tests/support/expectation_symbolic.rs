//! Resumo e política independentes: sem chamar a atualização do Learner.
use super::expectation::{Acquisition, Answer, Decision, Entry, Reason};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Summary {
    pub rows: Vec<Entry>,
}
impl Summary {
    pub fn new() -> Self {
        Self {
            rows: Vec::with_capacity(20),
        }
    }
    pub fn observe(&mut self, a: &Acquisition) {
        let p = a.packet();
        let changed = p.factual.action != p.blocked.action;
        let position = self.rows.iter().position(|x| x.key == p.key);
        if let Some(i) = position {
            let old = self.rows[i];
            let same = old.last == Some(changed);
            self.rows[i] = Entry {
                key: p.key,
                last: Some(changed),
                streak: if same { 2 } else { 1 },
                confirmed: if same { Some(changed) } else { None },
                total: old.total + 1,
                refs: if same {
                    [old.refs[1].or(old.refs[0]), Some(p.id)]
                } else {
                    [Some(p.id), None]
                },
            };
        } else {
            assert!(self.rows.len() < 20);
            self.rows.push(Entry {
                key: p.key,
                last: Some(changed),
                streak: 1,
                confirmed: None,
                total: 1,
                refs: [Some(p.id), None],
            });
        }
    }
    pub fn decide(&self, key: u16) -> Answer {
        for row in &self.rows {
            if row.key == key {
                let decision = if row.confirmed == Some(false) {
                    Decision::Allow
                } else if row.confirmed == Some(true) {
                    Decision::Protect
                } else {
                    Decision::Investigate
                };
                return Answer {
                    decision,
                    refs: row.refs,
                    reason: if row.confirmed.is_none() {
                        Reason::Unconfirmed
                    } else {
                        Reason::Confirmed
                    },
                };
            }
        }
        Answer {
            decision: Decision::Investigate,
            refs: [None; 2],
            reason: Reason::Missing,
        }
    }
}
