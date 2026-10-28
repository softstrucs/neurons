//! Contextos declarados e ensaio separado de plasticidade limitada.
use super::{Direction, Language, MAX_ASSOCIATIONS, MAX_CANDIDATES, MAX_FORMS, MAX_INPUT_BYTES};
use crate::circuit::Circuit;
use crate::engram::{Cue, Engram, EngramId, EngramStore, RecordStatus};
use crate::event::NodeId;
use crate::learning::{Feedback, PlasticityRule, WeightUpdate};
use crate::neuron::Neuron;
use crate::synapse::{Synapse, SynapseId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Context {
    Finance,
    Furniture,
    Nature,
    Technology,
}
impl Context {
    fn code(self) -> u32 {
        match self {
            Self::Finance => 1,
            Self::Furniture => 2,
            Self::Nature => 3,
            Self::Technology => 4,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextError {
    InvalidInput,
    Capacity,
    Duplicate,
    UnknownConcept,
    UnknownTrial,
    Learning,
    Circuit,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContextOutcome {
    Supported {
        text: String,
        concept: u32,
        sense: String,
        context: Context,
        source_evidence: EngramId,
        target_evidence: EngramId,
    },
    Unknown,
    Ambiguous,
    Conflict,
    LimitExceeded,
    Inconclusive,
}
struct Concept {
    id: u32,
    sense: String,
}
struct Form {
    id: NodeId,
    language: Language,
    text: String,
}
/// Fatos de forma→conceito registrados por idioma e contexto.
pub struct ContextLexicon {
    concepts: Vec<Concept>,
    forms: Vec<Form>,
    links: EngramStore,
}
impl Default for ContextLexicon {
    fn default() -> Self {
        Self::new()
    }
}
impl ContextLexicon {
    pub fn new() -> Self {
        Self {
            concepts: Vec::new(),
            forms: Vec::new(),
            links: EngramStore::new(MAX_ASSOCIATIONS).expect("capacidade constante"),
        }
    }
    pub fn add_concept(&mut self, id: u32, sense: &str) -> Result<(), ContextError> {
        if id == 0
            || id > i32::MAX as u32
            || sense.is_empty()
            || sense.len() > MAX_INPUT_BYTES
            || sense.chars().any(char::is_control)
        {
            return Err(ContextError::InvalidInput);
        }
        if self.concepts.iter().any(|c| c.id == id) {
            return Err(ContextError::Duplicate);
        }
        if self.concepts.len() >= 64 {
            return Err(ContextError::Capacity);
        }
        self.concepts.push(Concept {
            id,
            sense: sense.to_owned(),
        });
        Ok(())
    }
    pub fn associate(
        &mut self,
        language: Language,
        text: &str,
        context: Context,
        concept: u32,
        positive: bool,
    ) -> Result<(), ContextError> {
        validate_expression(text)?;
        if !self.concepts.iter().any(|c| c.id == concept) {
            return Err(ContextError::UnknownConcept);
        }
        let existing = self
            .forms
            .iter()
            .find(|f| f.language == language && f.text == text)
            .map(|f| f.id);
        let tag = context_tag(language, context);
        let value = if positive { 1 } else { -1 };
        if let Some(id) = existing {
            if self.links.iter().any(|r| {
                r.cue().context() == tag
                    && r.cue().source() == id
                    && r.cue().pattern() == concept as i32
                    && r.value() == value
            }) {
                return Err(ContextError::Duplicate);
            }
        }
        if self.links.len() >= MAX_ASSOCIATIONS
            || (existing.is_none() && self.forms.len() >= MAX_FORMS)
        {
            return Err(ContextError::Capacity);
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
            Cue::new(tag, id, concept as i32).expect("pista limitada"),
            value,
            Some(id),
            0,
            None,
            RecordStatus::Observed,
        )
        .expect("observação válida");
        self.links.record(record).expect("capacidade verificada");
        Ok(())
    }
    /// `context=None` considera todos os contextos; múltiplas provas são ambíguas.
    pub fn translate(
        &self,
        direction: Direction,
        text: &str,
        context: Option<Context>,
        budget: usize,
    ) -> Result<ContextOutcome, ContextError> {
        validate_expression(text)?;
        if budget > 2 {
            return Err(ContextError::InvalidInput);
        }
        if budget < 2 {
            return Ok(ContextOutcome::Inconclusive);
        }
        let (source_language, target_language) = direction.languages();
        let Some(source) = self
            .forms
            .iter()
            .find(|f| f.language == source_language && f.text == text)
        else {
            return Ok(ContextOutcome::Unknown);
        };
        let contexts = [
            Context::Finance,
            Context::Furniture,
            Context::Nature,
            Context::Technology,
        ];
        let mut selected = None;
        let mut candidates = 0;
        let mut ambiguous = false;
        for cx in contexts {
            if context.is_some_and(|wanted| wanted != cx) {
                continue;
            }
            let source_tag = context_tag(source_language, cx);
            let target_tag = context_tag(target_language, cx);
            for first in self.links.iter().filter(|r| {
                r.cue().context() == source_tag && r.cue().source() == source.id && r.value() == 1
            }) {
                if self
                    .links
                    .iter()
                    .any(|r| r.cue() == first.cue() && r.value() == -1)
                {
                    return Ok(ContextOutcome::Conflict);
                }
                for second in self.links.iter().filter(|r| {
                    r.cue().context() == target_tag
                        && r.cue().pattern() == first.cue().pattern()
                        && r.value() == 1
                }) {
                    candidates += 1;
                    if candidates > MAX_CANDIDATES {
                        return Ok(ContextOutcome::LimitExceeded);
                    }
                    if self
                        .links
                        .iter()
                        .any(|r| r.cue() == second.cue() && r.value() == -1)
                    {
                        return Ok(ContextOutcome::Conflict);
                    }
                    if selected.is_some() {
                        ambiguous = true;
                    } else {
                        selected = Some((cx, first, second));
                    }
                }
            }
        }
        if ambiguous {
            return Ok(ContextOutcome::Ambiguous);
        }
        let Some((cx, first, second)) = selected else {
            return Ok(ContextOutcome::Unknown);
        };
        let Some(form) = self
            .forms
            .iter()
            .find(|f| f.language == target_language && f.id == second.cue().source())
        else {
            return Ok(ContextOutcome::Inconclusive);
        };
        let Some(concept) = self
            .concepts
            .iter()
            .find(|c| c.id == first.cue().pattern() as u32)
        else {
            return Ok(ContextOutcome::Inconclusive);
        };
        Ok(ContextOutcome::Supported {
            text: form.text.clone(),
            concept: concept.id,
            sense: concept.sense.clone(),
            context: cx,
            source_evidence: first.id(),
            target_evidence: second.id(),
        })
    }
    pub fn association_count(&self) -> usize {
        self.links.len()
    }
}
fn context_tag(language: Language, context: Context) -> u32 {
    1300 + context.code() * 2 + u32::from(language == Language::En)
}
fn validate_expression(text: &str) -> Result<(), ContextError> {
    if text.is_empty()
        || text.len() > MAX_INPUT_BYTES
        || text.chars().any(char::is_control)
        || text.split_whitespace().count() == 0
        || text.split_whitespace().count() > 12
    {
        return Err(ContextError::InvalidInput);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Choice {
    First,
    Second,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrialOutcome {
    First,
    Second,
    Ambiguous,
    NoActivation,
    Inconclusive,
}
struct Trial {
    id: u32,
    weights: [i32; 2],
}
/// Cada chave representa um item/contexto fornecido; pesos são mantidos no ensaio.
pub struct FeedbackExperiment {
    trials: Vec<Trial>,
    rule: PlasticityRule,
}
impl Default for FeedbackExperiment {
    fn default() -> Self {
        Self::new()
    }
}
impl FeedbackExperiment {
    pub fn new() -> Self {
        Self {
            trials: Vec::new(),
            rule: PlasticityRule::new(0, 2, 1).expect("regra fixa"),
        }
    }
    pub fn add_trial(&mut self, id: u32) -> Result<(), ContextError> {
        if id == 0 {
            return Err(ContextError::InvalidInput);
        }
        if self.trials.iter().any(|t| t.id == id) {
            return Err(ContextError::Duplicate);
        }
        if self.trials.len() >= 64 {
            return Err(ContextError::Capacity);
        }
        self.trials.push(Trial {
            id,
            weights: [0, 0],
        });
        Ok(())
    }
    pub fn weights(&self, id: u32) -> Result<[i32; 2], ContextError> {
        self.trials
            .iter()
            .find(|t| t.id == id)
            .map(|t| t.weights)
            .ok_or(ContextError::UnknownTrial)
    }
    pub fn feedback(
        &mut self,
        id: u32,
        choice: Choice,
        feedback: Option<Feedback>,
    ) -> Result<WeightUpdate, ContextError> {
        let trial = self
            .trials
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(ContextError::UnknownTrial)?;
        let slot = match choice {
            Choice::First => 0,
            Choice::Second => 1,
        };
        let update = self
            .rule
            .adjust(trial.weights[slot], feedback)
            .map_err(|_| ContextError::Learning)?;
        trial.weights[slot] = update.after;
        Ok(update)
    }
    /// Cada consulta reconstrói o circuito com pesos atuais; não persiste estado neuronal.
    pub fn evaluate(
        &self,
        id: u32,
        payload: i32,
        ablated: bool,
    ) -> Result<TrialOutcome, ContextError> {
        if !(1..=2).contains(&payload) {
            return Err(ContextError::InvalidInput);
        }
        let weights = self.weights(id)?;
        let mut circuit = Circuit::new(2, 3).map_err(|_| ContextError::Circuit)?;
        for n in 1..=3 {
            circuit
                .add_neuron(
                    Neuron::new(NodeId::new(n).map_err(|_| ContextError::Circuit)?, 1, 2)
                        .map_err(|_| ContextError::Circuit)?,
                )
                .map_err(|_| ContextError::Circuit)?;
        }
        for i in 0..2 {
            circuit
                .connect(
                    Synapse::new(
                        SynapseId::new((i + 1) as u32).map_err(|_| ContextError::Circuit)?,
                        NodeId::new(1).map_err(|_| ContextError::Circuit)?,
                        NodeId::new((i + 2) as u32).map_err(|_| ContextError::Circuit)?,
                        if ablated { 0 } else { weights[i] },
                        1,
                    )
                    .map_err(|_| ContextError::Circuit)?,
                )
                .map_err(|_| ContextError::Circuit)?;
        }
        circuit
            .stimulate(
                NodeId::new(1).map_err(|_| ContextError::Circuit)?,
                payload,
                0,
            )
            .map_err(|_| ContextError::Circuit)?;
        let report = circuit.run_cycle();
        if report.first_error.is_some() || report.cycle.remaining > 0 {
            return Ok(TrialOutcome::Inconclusive);
        }
        let first = circuit
            .neuron_state(NodeId::new(2).map_err(|_| ContextError::Circuit)?)
            .ok_or(ContextError::Circuit)?
            .activations
            > 0;
        let second = circuit
            .neuron_state(NodeId::new(3).map_err(|_| ContextError::Circuit)?)
            .ok_or(ContextError::Circuit)?
            .activations
            > 0;
        Ok(match (first, second) {
            (true, false) => TrialOutcome::First,
            (false, true) => TrialOutcome::Second,
            (true, true) => TrialOutcome::Ambiguous,
            (false, false) => TrialOutcome::NoActivation,
        })
    }
}
