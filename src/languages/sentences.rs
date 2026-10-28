//! Gramática delimitada: pt substantivo+adjetivo; en adjetivo+substantivo.
use super::{Direction, MAX_INPUT_BYTES};
use crate::engram::{Cue, Engram, EngramId, EngramStore, RecordStatus};
use crate::event::NodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SentenceError {
    InvalidInput,
    Duplicate,
    Capacity,
    UnknownEvidence,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SentenceOutcome {
    Supported {
        text: String,
        rule: EngramId,
        noun: EngramId,
        adjective: EngramId,
    },
    Unknown,
    Ambiguous,
    Conflict,
    OutOfGrammar,
    LimitExceeded,
    Inconclusive,
}
struct Noun {
    pt: String,
    en: String,
    gender: Gender,
    evidence: EngramId,
}
struct Adjective {
    masculine: String,
    feminine: String,
    en: String,
    evidence: EngramId,
}
/// Inventário explícito; não armazena frases completas.
pub struct SentenceLexicon {
    nouns: Vec<Noun>,
    adjectives: Vec<Adjective>,
    facts: EngramStore,
    rule: Option<EngramId>,
}
impl SentenceLexicon {
    pub fn new(rule_enabled: bool) -> Self {
        let mut facts = EngramStore::new(128).expect("capacidade constante");
        let rule = if rule_enabled {
            let id = EngramId::new(1).expect("id fixo");
            facts
                .record(
                    Engram::new(
                        id,
                        Cue::new(2200, NodeId::new(1).expect("id fixo"), 1).expect("pista fixa"),
                        1,
                        None,
                        0,
                        None,
                        RecordStatus::Observed,
                    )
                    .expect("regra fixa"),
                )
                .expect("registro fixo");
            Some(id)
        } else {
            None
        };
        Self {
            nouns: Vec::new(),
            adjectives: Vec::new(),
            facts,
            rule,
        }
    }
    pub fn add_noun(
        &mut self,
        pt: &str,
        en: &str,
        gender: Gender,
    ) -> Result<EngramId, SentenceError> {
        token(pt)?;
        token(en)?;
        if self
            .nouns
            .iter()
            .any(|x| x.pt == pt && x.en == en && x.gender == gender)
        {
            return Err(SentenceError::Duplicate);
        }
        if self.nouns.len() >= 32 || self.facts.len() >= 128 {
            return Err(SentenceError::Capacity);
        }
        let evidence = self.fact(2201)?;
        self.nouns.push(Noun {
            pt: pt.into(),
            en: en.into(),
            gender,
            evidence,
        });
        Ok(evidence)
    }
    pub fn add_adjective(
        &mut self,
        masculine: &str,
        feminine: &str,
        en: &str,
    ) -> Result<EngramId, SentenceError> {
        token(masculine)?;
        token(feminine)?;
        token(en)?;
        if self
            .adjectives
            .iter()
            .any(|x| x.masculine == masculine && x.feminine == feminine && x.en == en)
        {
            return Err(SentenceError::Duplicate);
        }
        if self.adjectives.len() >= 32 || self.facts.len() >= 128 {
            return Err(SentenceError::Capacity);
        }
        let evidence = self.fact(2202)?;
        self.adjectives.push(Adjective {
            masculine: masculine.into(),
            feminine: feminine.into(),
            en: en.into(),
            evidence,
        });
        Ok(evidence)
    }
    fn fact(&mut self, context: u32) -> Result<EngramId, SentenceError> {
        let num = (self.facts.len() + 1) as u32;
        let id = EngramId::new(num).map_err(|_| SentenceError::Capacity)?;
        let cue = Cue::new(
            context,
            NodeId::new(num).map_err(|_| SentenceError::Capacity)?,
            1,
        )
        .map_err(|_| SentenceError::Capacity)?;
        self.facts
            .record(
                Engram::new(id, cue, 1, None, 0, None, RecordStatus::Observed)
                    .map_err(|_| SentenceError::Capacity)?,
            )
            .map_err(|_| SentenceError::Capacity)?;
        Ok(id)
    }
    /// Evidência contrária explícita para um fato previamente cadastrado.
    pub fn contradict(&mut self, id: EngramId) -> Result<(), SentenceError> {
        let original = self
            .facts
            .get(id)
            .map_err(|_| SentenceError::UnknownEvidence)?;
        if original.value() != 1 {
            return Err(SentenceError::UnknownEvidence);
        }
        if self
            .facts
            .iter()
            .any(|r| r.cue() == original.cue() && r.value() == -1)
        {
            return Err(SentenceError::Duplicate);
        }
        if self.facts.len() >= 128 {
            return Err(SentenceError::Capacity);
        }
        let new_id =
            EngramId::new((self.facts.len() + 1) as u32).map_err(|_| SentenceError::Capacity)?;
        self.facts
            .record(
                Engram::new(
                    new_id,
                    original.cue(),
                    -1,
                    None,
                    0,
                    None,
                    RecordStatus::Observed,
                )
                .map_err(|_| SentenceError::Capacity)?,
            )
            .map_err(|_| SentenceError::Capacity)
    }
    fn conflicting(&self, id: EngramId) -> bool {
        self.facts.get(id).ok().is_some_and(|r| {
            self.facts
                .iter()
                .any(|item| item.cue() == r.cue() && item.value() == -1)
        })
    }
    /// Uma única tentativa sem eventos. A regra ausente causa inconclusão.
    pub fn translate(
        &self,
        direction: Direction,
        input: &str,
        budget: usize,
    ) -> Result<SentenceOutcome, SentenceError> {
        if budget > 2 {
            return Err(SentenceError::InvalidInput);
        }
        if input.len() > MAX_INPUT_BYTES || input.split_whitespace().count() > 12 {
            return Ok(SentenceOutcome::LimitExceeded);
        }
        if input.chars().any(char::is_control) || input.is_empty() {
            return Err(SentenceError::InvalidInput);
        }
        if budget == 0 || self.rule.is_none() {
            return Ok(SentenceOutcome::Inconclusive);
        }
        let words: Vec<_> = input.split_whitespace().collect();
        if words.len() != 2 {
            return Ok(SentenceOutcome::OutOfGrammar);
        }
        let (noun_word, adj_word) = match direction {
            Direction::PtToEn => (words[0], words[1]),
            Direction::EnToPt => (words[1], words[0]),
        };
        let mut nouns = self.nouns.iter().filter(|x| match direction {
            Direction::PtToEn => x.pt == noun_word,
            Direction::EnToPt => x.en == noun_word,
        });
        let Some(noun) = nouns.next() else {
            return Ok(if self.any_word(noun_word, direction) {
                SentenceOutcome::OutOfGrammar
            } else {
                SentenceOutcome::Unknown
            });
        };
        if nouns.next().is_some() {
            return Ok(SentenceOutcome::Ambiguous);
        }
        let mut adjs = self.adjectives.iter().filter(|x| match direction {
            Direction::PtToEn => match noun.gender {
                Gender::Masculine => x.masculine == adj_word,
                Gender::Feminine => x.feminine == adj_word,
            },
            Direction::EnToPt => x.en == adj_word,
        });
        let Some(adj) = adjs.next() else {
            return Ok(if self.any_word(adj_word, direction) {
                SentenceOutcome::OutOfGrammar
            } else {
                SentenceOutcome::Unknown
            });
        };
        if adjs.next().is_some() {
            return Ok(SentenceOutcome::Ambiguous);
        }
        let rule = self.rule.ok_or(SentenceError::InvalidInput)?;
        if self.conflicting(rule)
            || self.conflicting(noun.evidence)
            || self.conflicting(adj.evidence)
        {
            return Ok(SentenceOutcome::Conflict);
        }
        let text = match direction {
            Direction::PtToEn => format!("{} {}", adj.en, noun.en),
            Direction::EnToPt => format!(
                "{} {}",
                noun.pt,
                match noun.gender {
                    Gender::Masculine => &adj.masculine,
                    Gender::Feminine => &adj.feminine,
                }
            ),
        };
        Ok(SentenceOutcome::Supported {
            text,
            rule,
            noun: noun.evidence,
            adjective: adj.evidence,
        })
    }
    /// Consulta registros e dados lexicais usados na evidência, sem alterá-los.
    pub fn evidence(&self, id: EngramId) -> Option<Engram> {
        self.facts.get(id).ok()
    }
    pub fn noun_fact(&self, id: EngramId) -> Option<(&str, &str, Gender)> {
        self.nouns
            .iter()
            .find(|n| n.evidence == id)
            .map(|n| (n.pt.as_str(), n.en.as_str(), n.gender))
    }
    pub fn adjective_fact(&self, id: EngramId) -> Option<(&str, &str, &str)> {
        self.adjectives
            .iter()
            .find(|a| a.evidence == id)
            .map(|a| (a.masculine.as_str(), a.feminine.as_str(), a.en.as_str()))
    }
    fn any_word(&self, word: &str, direction: Direction) -> bool {
        self.nouns.iter().any(|x| match direction {
            Direction::PtToEn => x.pt == word,
            Direction::EnToPt => x.en == word,
        }) || self.adjectives.iter().any(|x| match direction {
            Direction::PtToEn => x.masculine == word || x.feminine == word,
            Direction::EnToPt => x.en == word,
        })
    }
}
fn token(value: &str) -> Result<(), SentenceError> {
    if value.is_empty()
        || value.len() > MAX_INPUT_BYTES
        || value.chars().any(|c| c.is_whitespace() || c.is_control())
    {
        Err(SentenceError::InvalidInput)
    } else {
        Ok(())
    }
}
