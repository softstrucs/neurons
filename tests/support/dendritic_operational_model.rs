use std::collections::BTreeSet;

use neurons::engine::{Engine, EngineError};
use neurons::event::{Event, NodeId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmissionError {
    UnknownDestination(NodeId),
    PastLogicalTime,
    InsufficientCapacity,
    Engine(EngineError),
}

pub struct OperationalAdapter {
    engine: Engine,
    capacity: usize,
    destinations: BTreeSet<NodeId>,
}

impl OperationalAdapter {
    pub fn new(capacity: usize, cycle_limit: usize) -> Result<Self, EngineError> {
        Ok(Self {
            engine: Engine::new(capacity, cycle_limit)?,
            capacity,
            destinations: BTreeSet::new(),
        })
    }

    pub fn register(&mut self, id: NodeId) {
        self.destinations.insert(id);
        self.engine.register_destination(id);
    }

    pub fn admit(&mut self, group: &[Event]) -> Result<(), AdmissionError> {
        // Nenhuma insercao ocorre durante a validacao.
        for event in group {
            if !self.destinations.contains(&event.destination()) {
                return Err(AdmissionError::UnknownDestination(event.destination()));
            }

            if let Some(current) = self.engine.logical_time() {
                if event.logical_time() < current {
                    return Err(AdmissionError::PastLogicalTime);
                }
            }
        }

        let available = self.capacity.saturating_sub(self.engine.queue_len());

        if group.len() > available {
            return Err(AdmissionError::InsufficientCapacity);
        }

        // Esta fase nao possui rollback.
        // A prevalidacao cobre apenas condicoes observaveis.
        for event in group {
            self.engine
                .enqueue(*event)
                .map_err(AdmissionError::Engine)?;
        }

        Ok(())
    }

    pub fn queue_len(&self) -> usize {
        self.engine.queue_len()
    }

    pub fn logical_time(&self) -> Option<u64> {
        self.engine.logical_time()
    }

    pub fn run_cycle(&mut self) -> Vec<(NodeId, i32, u64)> {
        let mut delivered = Vec::new();

        self.engine.run_cycle(|event, _sink| {
            delivered.push((event.destination(), event.payload(), event.logical_time()));
        });

        delivered
    }

    pub fn metrics(&self) -> neurons::engine::EngineMetrics {
        self.engine.metrics()
    }
}
