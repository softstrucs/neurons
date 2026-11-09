//! Integração delimitada de motor, memória, monitor e checkpoint existente.
//! O checkpoint não contém configuração do monitor ou estado de circuito.
use crate::cognition::Hypothesis;
use crate::engine::Engine;
use crate::engram::EngramStore;
use crate::metacognition::{Monitor, ReviewError, ReviewReport};
use crate::persistence::{self, PersistenceError};

/// Consulta tipada M2, sem aquisição ou alteração do estado integrado legado.
pub mod query;

/// Aquisição simbólica assistida M3, com incorporação explícita e histórico finito.
pub mod acquisition;

pub struct IntegratedState {
    engine: Engine,
    memory: EngramStore,
    monitor: Monitor,
}

impl IntegratedState {
    pub fn new(engine: Engine, memory: EngramStore, monitor: Monitor) -> Self {
        Self {
            engine,
            memory,
            monitor,
        }
    }

    /// A configuração do monitor é responsabilidade explícita do chamador.
    pub fn restore(bytes: &[u8], monitor: Monitor) -> Result<Self, PersistenceError> {
        let (engine, memory) = persistence::decode(bytes)?;
        Ok(Self::new(engine, memory, monitor))
    }

    pub fn checkpoint(&self) -> Result<Vec<u8>, PersistenceError> {
        persistence::encode(&self.engine, &self.memory)
    }

    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }
    pub fn memory_mut(&mut self) -> &mut EngramStore {
        &mut self.memory
    }
    pub fn memory(&self) -> &EngramStore {
        &self.memory
    }

    pub fn review(
        &self,
        hypothesis: Hypothesis,
        contexts: &[u32],
    ) -> Result<ReviewReport, ReviewError> {
        self.monitor.review(&self.memory, hypothesis, contexts)
    }
}

/// CIC-I1: contratos e mecanismo conceitual simbólico delimitado.
pub mod concepts;
