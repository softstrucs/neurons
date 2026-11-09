//! Vocabulário CIC: identidade não contém definição; nenhuma regra-alvo é configurada.
use crate::cognitive_contracts::{CatalogId, DeclaredNature, EpisodeId, Scope, SourceId};
use crate::engram::EvidenceId;
use crate::event::NodeId;

pub const ATTRIBUTES: usize = 6;
pub const FAMILY_SIZE: usize = 23;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputError {
    InvalidId,
    InvalidName,
    InvalidAttribute,
    TooManyUnknowns,
    IncompleteExample,
    InvalidConjunction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptIdentity {
    id: NodeId,
    catalog: CatalogId,
    name: String,
}
impl ConceptIdentity {
    pub fn new(id: NodeId, catalog: CatalogId, name: &str) -> Result<Self, InputError> {
        if name.trim().is_empty() || name.len() > 128 || name.chars().any(char::is_control) {
            return Err(InputError::InvalidName);
        }
        Ok(Self {
            id,
            catalog,
            name: name.into(),
        })
    }
    pub fn id(&self) -> NodeId {
        self.id
    }
    pub fn catalog(&self) -> CatalogId {
        self.catalog
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReceiptId(u32);
impl ReceiptId {
    pub fn new(value: u32) -> Result<Self, InputError> {
        if value == 0 {
            Err(InputError::InvalidId)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Attribute {
    A,
    B,
    C,
    D,
    E,
    F,
}
impl Attribute {
    pub const ALL: [Self; 6] = [Self::A, Self::B, Self::C, Self::D, Self::E, Self::F];
    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Attributes([bool; 6]);
impl Attributes {
    pub fn new(values: [u8; 6]) -> Result<Self, InputError> {
        if values.iter().any(|v| *v > 1) {
            return Err(InputError::InvalidAttribute);
        }
        Ok(Self(values.map(|v| v == 1)))
    }
    pub fn values(self) -> [bool; 6] {
        self.0
    }
    /// Codificação de valores observados, sem semântica aritmética dos IDs.
    pub(crate) fn encode(self) -> i32 {
        1 + self
            .0
            .iter()
            .enumerate()
            .fold(0, |bits, (i, v)| bits | (i32::from(*v) << i))
    }
    pub(crate) fn decode(pattern: i32) -> Result<Self, InputError> {
        if !(1..=64).contains(&pattern) {
            return Err(InputError::InvalidAttribute);
        }
        Ok(Self(std::array::from_fn(|i| (pattern - 1) & (1 << i) != 0)))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Observation([Option<bool>; 6]);
impl Observation {
    pub fn new(values: [Option<u8>; 6]) -> Result<Self, InputError> {
        if values.iter().flatten().any(|v| *v > 1) {
            return Err(InputError::InvalidAttribute);
        }
        if values.iter().filter(|v| v.is_none()).count() > 1 {
            return Err(InputError::TooManyUnknowns);
        }
        Ok(Self(values.map(|v| v.map(|b| b == 1))))
    }
    pub fn complete(self) -> Result<Attributes, InputError> {
        if self.0.iter().any(Option::is_none) {
            return Err(InputError::IncompleteExample);
        }
        Ok(Attributes(self.0.map(|v| v.unwrap())))
    }
    pub fn completions(self) -> Vec<Attributes> {
        let mut first = self.0.map(|v| v.unwrap_or(false));
        let mut result = vec![Attributes(first)];
        if let Some(i) = self.0.iter().position(Option::is_none) {
            first[i] = true;
            result.push(Attributes(first));
        }
        result
    }
}
impl From<Attributes> for Observation {
    fn from(value: Attributes) -> Self {
        Self(value.0.map(Some))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Label {
    Positive,
    Negative,
}
impl Label {
    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::Positive
        } else {
            Self::Negative
        }
    }
    pub(crate) fn encode(self) -> i32 {
        if self == Self::Positive {
            1
        } else {
            -1
        }
    }
    pub(crate) fn decode(value: i32) -> Result<Self, InputError> {
        match value {
            1 => Ok(Self::Positive),
            -1 => Ok(Self::Negative),
            _ => Err(InputError::InvalidAttribute),
        }
    }
}

/// Campos não autenticados preservados na recepção, inclusive formato inválido.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IncomingExample {
    pub receipt: ReceiptId,
    pub teacher_episode: EpisodeId,
    pub concept: NodeId,
    pub exemplar: NodeId,
    pub attributes: [Option<u8>; 6],
    pub label: Label,
    pub scope: Scope,
    pub source: Option<SourceId>,
    pub evidence: Option<EvidenceId>,
    pub nature: DeclaredNature,
    pub observed_at: u64,
    pub sent_at: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Example {
    pub exemplar: NodeId,
    pub attributes: Attributes,
    pub label: Label,
}
impl IncomingExample {
    pub fn interpret(self) -> Result<Example, InputError> {
        Ok(Example {
            exemplar: self.exemplar,
            attributes: Observation::new(self.attributes)?.complete()?,
            label: self.label,
        })
    }
}

/// Apenas os construtores da família pública produzem critérios.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Criterion {
    mask: u8,
    constant: Option<bool>,
}
impl Criterion {
    pub fn attribute(a: Attribute) -> Self {
        Self {
            mask: 1 << a.index(),
            constant: None,
        }
    }
    pub fn conjunction(a: Attribute, b: Attribute) -> Result<Self, InputError> {
        if a == b {
            return Err(InputError::InvalidConjunction);
        }
        Ok(Self {
            mask: (1 << a.index()) | (1 << b.index()),
            constant: None,
        })
    }
    pub fn constant(value: bool) -> Self {
        Self {
            mask: 0,
            constant: Some(value),
        }
    }
    pub fn evaluate(self, input: Attributes) -> Label {
        Label::from_bool(self.constant.unwrap_or_else(|| {
            input
                .0
                .iter()
                .enumerate()
                .all(|(i, v)| self.mask & (1 << i) == 0 || *v)
        }))
    }
    pub fn literal_count(self) -> u32 {
        self.mask.count_ones()
    }
}

/// Ordem pública: constantes false/true, atributos A..F, pares lexicográficos.
pub fn family() -> Vec<Criterion> {
    let mut result = vec![Criterion::constant(false), Criterion::constant(true)];
    result.extend(Attribute::ALL.map(Criterion::attribute));
    for (i, a) in Attribute::ALL.iter().enumerate() {
        for b in &Attribute::ALL[i + 1..] {
            result.push(Criterion::conjunction(*a, *b).unwrap());
        }
    }
    result
}
