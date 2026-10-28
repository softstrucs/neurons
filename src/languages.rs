//! Associações lexicais fornecidas, com comparação UTF-8 exata.
use crate::engram::{Cue, Engram, EngramId, EngramStore, RecordStatus};
use crate::event::NodeId;

pub const MAX_FORMS: usize = 128;
pub const MAX_ASSOCIATIONS: usize = 256;
pub const MAX_INPUT_BYTES: usize = 256;
pub const MAX_CANDIDATES: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Language {
    PtBr,
    En,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    PtToEn,
    EnToPt,
}
impl Direction {
    fn languages(self) -> (Language, Language) {
        match self {
            Self::PtToEn => (Language::PtBr, Language::En),
            Self::EnToPt => (Language::En, Language::PtBr),
        }
    }
    fn context(self) -> u32 {
        match self {
            Self::PtToEn => 1101,
            Self::EnToPt => 1102,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Identification {
    Identified(Language),
    Unknown,
    Ambiguous,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Outcome {
    Supported { text: String, evidence: EngramId },
    Unknown,
    Ambiguous,
    Inconclusive,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LexiconError {
    InvalidInput,
    Capacity,
    DuplicateAssociation,
}
struct Form {
    id: NodeId,
    language: Language,
    text: String,
}

/// Armazena associações direcionais como engramas; conteúdo é fornecido pelo chamador.
pub struct Lexicon {
    forms: Vec<Form>,
    records: EngramStore,
}
impl Lexicon {
    pub fn new() -> Self {
        Self {
            forms: Vec::new(),
            records: EngramStore::new(MAX_ASSOCIATIONS).expect("capacidade constante válida"),
        }
    }
    /// Um par adiciona duas associações. Não altera estado se a entrada for inválida ou não couber.
    pub fn add_pair(&mut self, pt: &str, en: &str) -> Result<(), LexiconError> {
        validate(pt)?;
        validate(en)?;
        let p = self.find(Language::PtBr, pt);
        let e = self.find(Language::En, en);
        if let (Some(p), Some(e)) = (p, e) {
            if self.records.iter().any(|r| {
                r.cue().context() == Direction::PtToEn.context()
                    && r.cue().source() == p
                    && r.value() == e.get() as i32
            }) {
                return Err(LexiconError::DuplicateAssociation);
            }
        }
        let needed = usize::from(p.is_none()) + usize::from(e.is_none());
        if self.forms.len() + needed > MAX_FORMS || self.records.len() + 2 > MAX_ASSOCIATIONS {
            return Err(LexiconError::Capacity);
        }
        let p = match p {
            Some(id) => id,
            None => self.insert_form(Language::PtBr, pt),
        };
        let e = match e {
            Some(id) => id,
            None => self.insert_form(Language::En, en),
        };
        for (source, target, direction) in [(p, e, Direction::PtToEn), (e, p, Direction::EnToPt)] {
            let id =
                EngramId::new((self.records.len() + 1) as u32).expect("id limitado e não nulo");
            let cue = Cue::new(direction.context(), source, 1).expect("pista constante válida");
            let record = Engram::new(
                id,
                cue,
                target.get() as i32,
                Some(source),
                0,
                None,
                RecordStatus::Observed,
            )
            .expect("registro observado válido");
            self.records
                .record(record)
                .expect("capacidade e identidade verificadas");
        }
        Ok(())
    }
    fn insert_form(&mut self, language: Language, text: &str) -> NodeId {
        let id = NodeId::new((self.forms.len() + 1) as u32).expect("id limitado e não nulo");
        self.forms.push(Form {
            id,
            language,
            text: text.to_owned(),
        });
        id
    }
    fn find(&self, language: Language, text: &str) -> Option<NodeId> {
        self.forms
            .iter()
            .find(|f| f.language == language && f.text == text)
            .map(|f| f.id)
    }
    pub fn identify(&self, text: &str) -> Result<Identification, LexiconError> {
        validate(text)?;
        Ok(
            match (
                self.find(Language::PtBr, text),
                self.find(Language::En, text),
            ) {
                (Some(_), Some(_)) => Identification::Ambiguous,
                (Some(_), None) => Identification::Identified(Language::PtBr),
                (None, Some(_)) => Identification::Identified(Language::En),
                (None, None) => Identification::Unknown,
            },
        )
    }
    /// Um orçamento zero resulta em avaliação inconclusiva; no máximo uma tentativa.
    pub fn translate(
        &self,
        direction: Direction,
        text: &str,
        attempts: usize,
    ) -> Result<Outcome, LexiconError> {
        validate(text)?;
        if attempts > 2 {
            return Err(LexiconError::InvalidInput);
        }
        if attempts == 0 {
            return Ok(Outcome::Inconclusive);
        }
        let (source_language, target_language) = direction.languages();
        let Some(source) = self.find(source_language, text) else {
            return Ok(Outcome::Unknown);
        };
        let mut matches = self
            .records
            .iter()
            .filter(|r| r.cue().context() == direction.context() && r.cue().source() == source);
        let Some(first) = matches.next() else {
            return Ok(Outcome::Unknown);
        };
        if matches.next().is_some() {
            return Ok(Outcome::Ambiguous);
        }
        let target = self
            .forms
            .iter()
            .find(|f| f.id.get() == first.value() as u32 && f.language == target_language)
            .ok_or(LexiconError::InvalidInput)?;
        Ok(Outcome::Supported {
            text: target.text.clone(),
            evidence: first.id(),
        })
    }
    pub fn form_count(&self) -> usize {
        self.forms.len()
    }
    pub fn association_count(&self) -> usize {
        self.records.len()
    }
}
impl Default for Lexicon {
    fn default() -> Self {
        Self::new()
    }
}
fn validate(text: &str) -> Result<(), LexiconError> {
    if text.is_empty()
        || text.len() > MAX_INPUT_BYTES
        || text.chars().any(char::is_whitespace)
        || text.chars().any(char::is_control)
    {
        return Err(LexiconError::InvalidInput);
    }
    Ok(())
}

/// Resultado da composição de duas relações tipadas, com evidências fornecidas.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConceptOutcome {
    Supported {
        text: String,
        concept: u32,
        sense: String,
        source_evidence: EngramId,
        target_evidence: EngramId,
    },
    Unknown,
    Ambiguous,
    Conflict,
    LimitExceeded,
    Inconclusive,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConceptError {
    InvalidInput,
    Capacity,
    Duplicate,
    UnknownConcept,
}
struct Concept {
    id: u32,
    sense: String,
}
/// Associa apenas forma→conceito; nunca registra forma→forma.
pub struct ConceptLexicon {
    forms: Vec<Form>,
    concepts: Vec<Concept>,
    links: EngramStore,
}
impl Default for ConceptLexicon {
    fn default() -> Self {
        Self::new()
    }
}
impl ConceptLexicon {
    pub fn new() -> Self {
        Self {
            forms: Vec::new(),
            concepts: Vec::new(),
            links: EngramStore::new(MAX_ASSOCIATIONS).expect("capacidade fixa"),
        }
    }
    pub fn add_concept(&mut self, id: u32, sense: &str) -> Result<(), ConceptError> {
        if id == 0
            || id > i32::MAX as u32
            || sense.is_empty()
            || sense.len() > MAX_INPUT_BYTES
            || sense.chars().any(char::is_control)
        {
            return Err(ConceptError::InvalidInput);
        }
        if self.concepts.iter().any(|c| c.id == id) {
            return Err(ConceptError::Duplicate);
        }
        if self.concepts.len() == 64 {
            return Err(ConceptError::Capacity);
        }
        self.concepts.push(Concept {
            id,
            sense: sense.to_owned(),
        });
        Ok(())
    }
    /// `positive=false` registra evidência contrária à mesma associação.
    pub fn associate(
        &mut self,
        language: Language,
        text: &str,
        concept: u32,
        positive: bool,
    ) -> Result<(), ConceptError> {
        validate(text).map_err(|_| ConceptError::InvalidInput)?;
        if !self.concepts.iter().any(|c| c.id == concept) {
            return Err(ConceptError::UnknownConcept);
        }
        let existing = self
            .forms
            .iter()
            .find(|f| f.language == language && f.text == text)
            .map(|f| f.id);
        let context = match language {
            Language::PtBr => 1201,
            Language::En => 1202,
        };
        let value = if positive { 1 } else { -1 };
        if let Some(id) = existing {
            if self.links.iter().any(|r| {
                r.cue().context() == context
                    && r.cue().source() == id
                    && r.cue().pattern() == concept as i32
                    && r.value() == value
            }) {
                return Err(ConceptError::Duplicate);
            }
        }
        if self.links.len() == MAX_ASSOCIATIONS
            || (existing.is_none() && self.forms.len() == MAX_FORMS)
        {
            return Err(ConceptError::Capacity);
        }
        let id = if let Some(id) = existing {
            id
        } else {
            let id = NodeId::new((self.forms.len() + 1) as u32).expect("id limitado");
            self.forms.push(Form {
                id,
                language,
                text: text.to_owned(),
            });
            id
        };
        let record = Engram::new(
            EngramId::new((self.links.len() + 1) as u32).expect("id limitado"),
            Cue::new(context, id, concept as i32).expect("pista válida"),
            value,
            Some(id),
            0,
            None,
            RecordStatus::Observed,
        )
        .expect("observação válida");
        self.links
            .record(record)
            .expect("capacidade e id verificados");
        Ok(())
    }
    pub fn compose(
        &self,
        direction: Direction,
        text: &str,
        budget: usize,
    ) -> Result<ConceptOutcome, ConceptError> {
        validate(text).map_err(|_| ConceptError::InvalidInput)?;
        if budget > 2 {
            return Err(ConceptError::InvalidInput);
        }
        if budget < 2 {
            return Ok(ConceptOutcome::Inconclusive);
        }
        let (source_lang, target_lang) = direction.languages();
        let Some(source) = self
            .forms
            .iter()
            .find(|f| f.language == source_lang && f.text == text)
        else {
            return Ok(ConceptOutcome::Unknown);
        };
        let src_context = match source_lang {
            Language::PtBr => 1201,
            Language::En => 1202,
        };
        let dst_context = match target_lang {
            Language::PtBr => 1201,
            Language::En => 1202,
        };
        let mut chosen = None;
        let mut ambiguous = false;
        let mut count = 0;
        for first in self.links.iter().filter(|r| {
            r.cue().context() == src_context && r.cue().source() == source.id && r.value() == 1
        }) {
            let concept = first.cue().pattern();
            if self
                .links
                .iter()
                .any(|r| r.cue() == first.cue() && r.value() == -1)
            {
                return Ok(ConceptOutcome::Conflict);
            }
            for second in self.links.iter().filter(|r| {
                r.cue().context() == dst_context && r.cue().pattern() == concept && r.value() == 1
            }) {
                count += 1;
                if count > MAX_CANDIDATES {
                    return Ok(ConceptOutcome::LimitExceeded);
                }
                if self
                    .links
                    .iter()
                    .any(|r| r.cue() == second.cue() && r.value() == -1)
                {
                    return Ok(ConceptOutcome::Conflict);
                }
                if chosen.is_some() {
                    ambiguous = true;
                } else {
                    chosen = Some((first, second));
                }
            }
        }
        if ambiguous {
            return Ok(ConceptOutcome::Ambiguous);
        }
        let Some((first, second)) = chosen else {
            return Ok(ConceptOutcome::Unknown);
        };
        let Some(form) = self
            .forms
            .iter()
            .find(|f| f.id == second.cue().source() && f.language == target_lang)
        else {
            return Ok(ConceptOutcome::Inconclusive);
        };
        let Some(concept) = self
            .concepts
            .iter()
            .find(|c| c.id == first.cue().pattern() as u32)
        else {
            return Ok(ConceptOutcome::Inconclusive);
        };
        Ok(ConceptOutcome::Supported {
            text: form.text.clone(),
            concept: concept.id,
            sense: concept.sense.clone(),
            source_evidence: first.id(),
            target_evidence: second.id(),
        })
    }
    pub fn association_count(&self) -> usize {
        self.links.len()
    }
}

/// Experimentos contextuais e de feedback, independentes da CLI.
pub mod contextual;
