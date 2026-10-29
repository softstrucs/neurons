//! Estado experimental: episódios observados e resumo modificável são separados.
//! Não importa a tabela do ambiente, pesos, Scenario ou previsões do avaliador.
use super::model::{Action, State};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Purpose {
    Acquisition,
    Evaluation,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Observation {
    pub id: u32,
    pub initial: [State; 4],
    pub states: [State; 4],
    pub action: Action,
    pub blocked: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Packet {
    pub id: u32,
    pub key: u16,
    pub purpose: Purpose,
    pub connection: u32,
    pub factual: Observation,
    pub blocked: Observation,
    pub complete: bool,
    pub comparable: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Acquisition(Packet);
impl Acquisition {
    pub fn checked(p: Packet) -> Result<Self, &'static str> {
        if p.purpose != Purpose::Acquisition
            || !p.complete
            || !p.comparable
            || p.factual.initial != p.blocked.initial
            || p.factual.initial != [State::default(); 4]
            || p.factual.blocked
            || !p.blocked.blocked
            || p.connection != 1
            || p.factual.id == p.blocked.id
        {
            return Err("par inválido ou reservado à avaliação");
        }
        Ok(Self(p))
    }
    pub fn packet(&self) -> &Packet {
        &self.0
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    Allow,
    Protect,
    Investigate,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Reason {
    Confirmed,
    Unconfirmed,
    Missing,
    Unavailable,
    Literal,
    Fixed,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Answer {
    pub decision: Decision,
    pub refs: [Option<u32>; 2],
    pub reason: Reason,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Entry {
    pub key: u16,
    pub last: Option<bool>,
    pub streak: u8,
    pub confirmed: Option<bool>,
    pub total: u16,
    pub refs: [Option<u32>; 2],
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Learner {
    pub episodes: Vec<Acquisition>,
    pub entries: Vec<Entry>,
    pub update: bool,
}
impl Learner {
    pub fn new(update: bool) -> Self {
        Self {
            episodes: Vec::with_capacity(40),
            entries: Vec::with_capacity(20),
            update,
        }
    }
    pub fn acquire(&mut self, a: Acquisition) -> Result<(), &'static str> {
        let p = a.packet();
        if self.episodes.len() >= 40 || self.episodes.iter().any(|x| x.packet().id == p.id) {
            return Err("capacidade ou ID duplicado");
        }
        if self.update {
            let index = if let Some(i) = self.entries.iter().position(|e| e.key == p.key) {
                i
            } else {
                if self.entries.len() == 20 {
                    return Err("capacidade de resumos");
                }
                self.entries.push(Entry {
                    key: p.key,
                    last: None,
                    streak: 0,
                    confirmed: None,
                    total: 0,
                    refs: [None; 2],
                });
                self.entries.len() - 1
            };
            let c = p.factual.action != p.blocked.action;
            let e = &mut self.entries[index];
            if e.last == Some(c) {
                e.streak = (e.streak + 1).min(2);
                e.refs = [e.refs[1].or(e.refs[0]), Some(p.id)];
            } else {
                e.last = Some(c);
                e.streak = 1;
                e.refs = [Some(p.id), None];
            }
            e.confirmed = if e.streak == 2 { Some(c) } else { None };
            e.total += 1;
        }
        self.episodes.push(a);
        Ok(())
    }
    pub fn query(&self, key: u16, available: bool) -> Answer {
        if !available {
            return Answer {
                decision: Decision::Investigate,
                refs: [None; 2],
                reason: Reason::Unavailable,
            };
        }
        match self.entries.iter().find(|e| e.key == key) {
            None => Answer {
                decision: Decision::Investigate,
                refs: [None; 2],
                reason: Reason::Missing,
            },
            Some(e) => Answer {
                decision: match e.confirmed {
                    Some(true) => Decision::Protect,
                    Some(false) => Decision::Allow,
                    None => Decision::Investigate,
                },
                refs: e.refs,
                reason: if e.confirmed.is_some() {
                    Reason::Confirmed
                } else {
                    Reason::Unconfirmed
                },
            },
        }
    }
    pub fn literal(&self, key: u16) -> Answer {
        match self.episodes.iter().rev().find(|a| a.packet().key == key) {
            Some(a) => {
                let p = a.packet();
                Answer {
                    decision: if p.factual.action == p.blocked.action {
                        Decision::Allow
                    } else {
                        Decision::Protect
                    },
                    refs: [Some(p.id), None],
                    reason: Reason::Literal,
                }
            }
            None => Answer {
                decision: Decision::Investigate,
                refs: [None; 2],
                reason: Reason::Missing,
            },
        }
    }
}
