//! Unidade experimental acionada apenas por eventos recebidos.

use crate::event::{Event, EventKind, NodeId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NeuronError {
    InvalidThreshold,
    InvalidStateLimit,
    WrongDestination { expected: NodeId, actual: NodeId },
    UnsupportedEventKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NeuronState {
    pub potential: i32,
    pub received: u64,
    pub activations: u64,
}

/// Saída local; uma sinapse a converte em evento endereçado.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Emission {
    source: NodeId,
    logical_time: u64,
    amplitude: i32,
}

impl Emission {
    pub const fn source(self) -> NodeId {
        self.source
    }

    pub const fn logical_time(self) -> u64 {
        self.logical_time
    }

    pub const fn amplitude(self) -> i32 {
        self.amplitude
    }
}

/// Acumulador inteiro limitado. Uma recepção produz no máximo uma ativação.
pub struct Neuron {
    id: NodeId,
    threshold: i32,
    state_limit: i32,
    state: NeuronState,
}

impl Neuron {
    pub fn new(id: NodeId, threshold: i32, state_limit: i32) -> Result<Self, NeuronError> {
        if threshold <= 0 {
            return Err(NeuronError::InvalidThreshold);
        }
        if state_limit < threshold {
            return Err(NeuronError::InvalidStateLimit);
        }
        Ok(Self {
            id,
            threshold,
            state_limit,
            state: NeuronState {
                potential: 0,
                received: 0,
                activations: 0,
            },
        })
    }

    pub const fn id(&self) -> NodeId {
        self.id
    }

    pub const fn state(&self) -> NeuronState {
        self.state
    }

    /// Soma a carga com saturação em `[-state_limit, state_limit]`. Ao atingir
    /// o limiar, zera o potencial e emite amplitude unitária no mesmo tempo.
    pub fn receive(&mut self, event: Event) -> Result<Option<Emission>, NeuronError> {
        if event.destination() != self.id {
            return Err(NeuronError::WrongDestination {
                expected: self.id,
                actual: event.destination(),
            });
        }
        if !matches!(event.kind(), EventKind::Stimulus | EventKind::Signal) {
            return Err(NeuronError::UnsupportedEventKind);
        }

        let limit = i64::from(self.state_limit);
        self.state.potential = (i64::from(self.state.potential) + i64::from(event.payload()))
            .clamp(-limit, limit) as i32;
        self.state.received = self.state.received.saturating_add(1);

        if self.state.potential >= self.threshold {
            self.state.potential = 0;
            self.state.activations = self.state.activations.saturating_add(1);
            return Ok(Some(Emission {
                source: self.id,
                logical_time: event.logical_time(),
                amplitude: 1,
            }));
        }
        Ok(None)
    }
}
