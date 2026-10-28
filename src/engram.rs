//! Associações experimentais em memória volátil, independentes do motor.

use crate::event::NodeId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngramError {
    InvalidId,
    InvalidContext,
    InvalidPattern,
    InvalidEvidence,
    ValidationWithoutEvidence,
    NoActivation,
    ActivationCounterRegression,
    ZeroCapacity,
    DuplicateId(EngramId),
    CapacityFull,
    NotFound(EngramId),
    NoAssociation,
    AmbiguousAssociation,
}

/// Identificador do registro; zero é reservado.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EngramId(u32);

impl EngramId {
    pub fn new(value: u32) -> Result<Self, EngramError> {
        if value == 0 {
            return Err(EngramError::InvalidId);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Referência a evidência fornecida pelo experimento; não prova a afirmação.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvidenceId(u32);

impl EvidenceId {
    pub fn new(value: u32) -> Result<Self, EngramError> {
        if value == 0 {
            return Err(EngramError::InvalidEvidence);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Chave exata de consulta. O padrão é um pequeno símbolo numérico, não texto.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cue {
    context: u32,
    source: NodeId,
    pattern: i32,
}

impl Cue {
    pub fn new(context: u32, source: NodeId, pattern: i32) -> Result<Self, EngramError> {
        if context == 0 {
            return Err(EngramError::InvalidContext);
        }
        if pattern == 0 {
            return Err(EngramError::InvalidPattern);
        }
        Ok(Self {
            context,
            source,
            pattern,
        })
    }

    /// Adaptador para `Circuit::neuron_state`: cada nova ativação da unidade
    /// produz a mesma pista simbólica `1`, sem guardar o contador absoluto.
    pub fn from_activation_counts(
        context: u32,
        source: NodeId,
        before: u64,
        after: u64,
    ) -> Result<Self, EngramError> {
        if after < before {
            return Err(EngramError::ActivationCounterRegression);
        }
        if after == before {
            return Err(EngramError::NoActivation);
        }
        Self::new(context, source, 1)
    }

    pub const fn context(self) -> u32 {
        self.context
    }

    pub const fn source(self) -> NodeId {
        self.source
    }

    pub const fn pattern(self) -> i32 {
        self.pattern
    }
}

/// Estado declarado do registro. `Validated` exige referência explícita de
/// evidência, mas a existência da referência não verifica sua qualidade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordStatus {
    Observed,
    Hypothesis,
    Validated,
}

/// Associação compacta com procedência e tempo de observação.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Engram {
    id: EngramId,
    cue: Cue,
    value: i32,
    origin: Option<NodeId>,
    observed_at: u64,
    evidence: Option<EvidenceId>,
    status: RecordStatus,
}

impl Engram {
    pub fn new(
        id: EngramId,
        cue: Cue,
        value: i32,
        origin: Option<NodeId>,
        observed_at: u64,
        evidence: Option<EvidenceId>,
        status: RecordStatus,
    ) -> Result<Self, EngramError> {
        if status == RecordStatus::Validated && evidence.is_none() {
            return Err(EngramError::ValidationWithoutEvidence);
        }
        Ok(Self {
            id,
            cue,
            value,
            origin,
            observed_at,
            evidence,
            status,
        })
    }

    pub const fn id(self) -> EngramId {
        self.id
    }

    pub const fn cue(self) -> Cue {
        self.cue
    }

    pub const fn value(self) -> i32 {
        self.value
    }

    pub const fn origin(self) -> Option<NodeId> {
        self.origin
    }

    pub const fn observed_at(self) -> u64 {
        self.observed_at
    }

    pub const fn evidence(self) -> Option<EvidenceId> {
        self.evidence
    }

    pub const fn status(self) -> RecordStatus {
        self.status
    }
}

/// Armazenamento volátil com capacidade fixa de registros. Consultas usam
/// comparação exata e não alteram estado, contadores ou status.
pub struct EngramStore {
    records: Vec<Engram>,
    capacity: usize,
}

impl EngramStore {
    pub fn new(capacity: usize) -> Result<Self, EngramError> {
        if capacity == 0 {
            return Err(EngramError::ZeroCapacity);
        }
        Ok(Self {
            records: Vec::new(),
            capacity,
        })
    }

    /// Registros com a mesma pista são permitidos, mas a consulta será ambígua.
    pub fn record(&mut self, engram: Engram) -> Result<(), EngramError> {
        if self.records.iter().any(|item| item.id() == engram.id()) {
            return Err(EngramError::DuplicateId(engram.id()));
        }
        if self.records.len() >= self.capacity {
            return Err(EngramError::CapacityFull);
        }
        self.records.push(engram);
        Ok(())
    }

    pub fn get(&self, id: EngramId) -> Result<Engram, EngramError> {
        self.records
            .iter()
            .copied()
            .find(|item| item.id() == id)
            .ok_or(EngramError::NotFound(id))
    }

    /// Recupera somente se houver um único registro com a pista exata.
    pub fn recall(&self, cue: Cue) -> Result<Engram, EngramError> {
        let mut found = None;
        for record in self.records.iter().copied() {
            if record.cue() == cue {
                if found.is_some() {
                    return Err(EngramError::AmbiguousAssociation);
                }
                found = Some(record);
            }
        }
        found.ok_or(EngramError::NoAssociation)
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Percorre registros em ordem de inserção, sem alocar ou alterar estado.
    pub fn iter(&self) -> impl Iterator<Item = Engram> + '_ {
        self.records.iter().copied()
    }
}
