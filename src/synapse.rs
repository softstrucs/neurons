//! Conexão configurável que transforma uma emissão em um evento.

use crate::event::{Event, EventKind, NodeId};
use crate::neuron::Emission;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SynapseIdError;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SynapseId(u32);

impl SynapseId {
    pub fn new(value: u32) -> Result<Self, SynapseIdError> {
        if value == 0 {
            return Err(SynapseIdError);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SynapseError {
    ZeroDelay,
    WrongSource { expected: NodeId, actual: NodeId },
    TimeOverflow,
}

/// `weight = 0` bloqueia a transmissão; valores negativos são inibitórios.
/// Peso e atraso são fixos até uma configuração explícita do chamador.
pub struct Synapse {
    id: SynapseId,
    source: NodeId,
    destination: NodeId,
    weight: i32,
    delay: u64,
}

impl Synapse {
    pub fn new(
        id: SynapseId,
        source: NodeId,
        destination: NodeId,
        weight: i32,
        delay: u64,
    ) -> Result<Self, SynapseError> {
        if delay == 0 {
            return Err(SynapseError::ZeroDelay);
        }
        Ok(Self {
            id,
            source,
            destination,
            weight,
            delay,
        })
    }

    pub const fn id(&self) -> SynapseId {
        self.id
    }

    pub const fn source(&self) -> NodeId {
        self.source
    }

    pub const fn destination(&self) -> NodeId {
        self.destination
    }

    pub const fn weight(&self) -> i32 {
        self.weight
    }

    pub fn set_weight(&mut self, weight: i32) {
        self.weight = weight;
    }

    pub fn transmit(&self, emission: Emission) -> Result<Option<Event>, SynapseError> {
        if emission.source() != self.source {
            return Err(SynapseError::WrongSource {
                expected: self.source,
                actual: emission.source(),
            });
        }
        if self.weight == 0 {
            return Ok(None);
        }
        let logical_time = emission
            .logical_time()
            .checked_add(self.delay)
            .ok_or(SynapseError::TimeOverflow)?;
        Ok(Some(Event::new(
            Some(self.source),
            self.destination,
            EventKind::Signal,
            emission.amplitude().saturating_mul(self.weight),
            logical_time,
        )))
    }
}
