//! Microcurrículos relacionais: contextos de domínio, sem semântica ou respostas embutidas.
use crate::cognition::{CognitionError, Hypothesis, TwoHopTask};
use crate::engram::{Cue, Engram, EngramError, EngramId, EngramStore, RecordStatus};
use crate::event::NodeId;
use crate::metacognition::{Monitor, ReviewError, ReviewReport};

/// Cada contexto contém uma única relação definida externamente pelo protocolo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Domain {
    Communication,
    Mathematics,
    NaturalScience,
    Computing,
}

impl Domain {
    pub const fn context(self) -> u32 {
        match self {
            Self::Communication => 901,
            Self::Mathematics => 902,
            Self::NaturalScience => 903,
            Self::Computing => 904,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Polarity {
    Positive,
    Negative,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CurriculumError {
    InvalidCapacity,
    InvalidTarget,
    ReflexiveObservation,
    Memory(EngramError),
    Query(CognitionError),
    Review(ReviewError),
}

/// Memória compartilhada limitada; registros são fornecidos como observações.
/// Não há treino implícito, promoção a validado ou gravação das conclusões.
pub struct Microcurriculum {
    memory: EngramStore,
    monitor: Monitor,
}

impl Microcurriculum {
    pub fn new(capacity: usize) -> Result<Self, CurriculumError> {
        if capacity == 0 || capacity > 32 {
            return Err(CurriculumError::InvalidCapacity);
        }
        let task = TwoHopTask::new(32, 64, 128).map_err(CurriculumError::Query)?;
        Ok(Self {
            memory: EngramStore::new(capacity).map_err(CurriculumError::Memory)?,
            monitor: Monitor::new(task, 1).map_err(CurriculumError::Review)?,
        })
    }

    pub fn observe(
        &mut self,
        id: EngramId,
        domain: Domain,
        from: NodeId,
        to: NodeId,
        polarity: Polarity,
    ) -> Result<(), CurriculumError> {
        if to.get() > i32::MAX as u32 {
            return Err(CurriculumError::InvalidTarget);
        }
        if from == to {
            return Err(CurriculumError::ReflexiveObservation);
        }
        let cue =
            Cue::new(domain.context(), from, to.get() as i32).map_err(CurriculumError::Memory)?;
        let value = match polarity {
            Polarity::Positive => 1,
            Polarity::Negative => -1,
        };
        let record = Engram::new(id, cue, value, Some(from), 0, None, RecordStatus::Observed)
            .map_err(CurriculumError::Memory)?;
        self.memory.record(record).map_err(CurriculumError::Memory)
    }

    pub fn evaluate(
        &self,
        domain: Domain,
        from: NodeId,
        to: NodeId,
    ) -> Result<ReviewReport, CurriculumError> {
        let hypothesis =
            Hypothesis::new(domain.context(), from, to).map_err(CurriculumError::Query)?;
        self.monitor
            .review(&self.memory, hypothesis, &[])
            .map_err(CurriculumError::Review)
    }

    /// Evidência acessível para auditoria e controles, sem acesso mutável.
    pub fn memory(&self) -> &EngramStore {
        &self.memory
    }
}
