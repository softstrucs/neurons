use std::collections::{BTreeMap, BTreeSet};

use neurons::event::NodeId;
use neurons::synapse::SynapseId;

#[path = "dendritic_contract_model.rs"]
mod contract;

use contract::DendriteId;

const MAGIC: &[u8; 4] = b"DENS";
const VERSION: u8 = 1;
const MAX_ITEMS: usize = 1024;
const HEADER_SIZE: usize = 17;
const MAX_BYTES: usize = HEADER_SIZE + MAX_ITEMS * 4 + MAX_ITEMS * 8 + MAX_ITEMS * 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StructuralError {
    DuplicateNeuron,
    UnknownNeuron,
    DuplicateDendrite,
    UnknownDendrite,
    WrongNeuron,
    DuplicateSynapse,
    LimitExceeded,
    InvalidFormat,
    UnsupportedVersion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Target {
    Soma(NodeId),
    Dendrite(NodeId, DendriteId),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Connection {
    pub source: NodeId,
    pub target: Target,
}

#[derive(Default)]
pub struct StructuralTopology {
    neurons: BTreeSet<NodeId>,
    dendrites: BTreeSet<(NodeId, DendriteId)>,
    connections: BTreeMap<SynapseId, Connection>,
}

impl StructuralTopology {
    pub fn add_neuron(&mut self, neuron: NodeId) -> Result<(), StructuralError> {
        if self.neurons.contains(&neuron) {
            return Err(StructuralError::DuplicateNeuron);
        }
        if self.neurons.len() >= MAX_ITEMS {
            return Err(StructuralError::LimitExceeded);
        }
        self.neurons.insert(neuron);
        Ok(())
    }

    pub fn add_dendrite(
        &mut self,
        neuron: NodeId,
        dendrite: DendriteId,
    ) -> Result<(), StructuralError> {
        if !self.neurons.contains(&neuron) {
            return Err(StructuralError::UnknownNeuron);
        }
        if self.dendrites.contains(&(neuron, dendrite)) {
            return Err(StructuralError::DuplicateDendrite);
        }
        if self.dendrites.len() >= MAX_ITEMS {
            return Err(StructuralError::LimitExceeded);
        }
        self.dendrites.insert((neuron, dendrite));
        Ok(())
    }

    pub fn connect(
        &mut self,
        synapse: SynapseId,
        source: NodeId,
        target: Target,
    ) -> Result<(), StructuralError> {
        if self.connections.contains_key(&synapse) {
            return Err(StructuralError::DuplicateSynapse);
        }
        if !self.neurons.contains(&source) {
            return Err(StructuralError::UnknownNeuron);
        }

        let destination = match target {
            Target::Soma(neuron) => neuron,
            Target::Dendrite(neuron, _) => neuron,
        };

        if !self.neurons.contains(&destination) {
            return Err(StructuralError::UnknownNeuron);
        }

        if let Target::Dendrite(neuron, dendrite) = target {
            if !self.dendrites.contains(&(neuron, dendrite)) {
                if self.dendrites.iter().any(|(_, id)| *id == dendrite) {
                    return Err(StructuralError::WrongNeuron);
                }
                return Err(StructuralError::UnknownDendrite);
            }
        }

        if self.connections.len() >= MAX_ITEMS {
            return Err(StructuralError::LimitExceeded);
        }

        self.connections
            .insert(synapse, Connection { source, target });
        Ok(())
    }

    pub fn connection(&self, id: SynapseId) -> Option<Connection> {
        self.connections.get(&id).copied()
    }

    pub fn counts(&self) -> (usize, usize, usize) {
        (
            self.neurons.len(),
            self.dendrites.len(),
            self.connections.len(),
        )
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(MAGIC);
        bytes.push(VERSION);

        for count in [
            self.neurons.len(),
            self.dendrites.len(),
            self.connections.len(),
        ] {
            bytes.extend_from_slice(&(count as u32).to_le_bytes());
        }

        for neuron in &self.neurons {
            bytes.extend_from_slice(&neuron.get().to_le_bytes());
        }

        for (neuron, dendrite) in &self.dendrites {
            bytes.extend_from_slice(&neuron.get().to_le_bytes());
            bytes.extend_from_slice(&dendrite.get().to_le_bytes());
        }

        for (synapse, connection) in &self.connections {
            bytes.extend_from_slice(&synapse.get().to_le_bytes());
            bytes.extend_from_slice(&connection.source.get().to_le_bytes());

            match connection.target {
                Target::Soma(neuron) => {
                    bytes.extend_from_slice(&neuron.get().to_le_bytes());
                    bytes.extend_from_slice(&0_u32.to_le_bytes());
                }
                Target::Dendrite(neuron, dendrite) => {
                    bytes.extend_from_slice(&neuron.get().to_le_bytes());
                    bytes.extend_from_slice(&dendrite.get().to_le_bytes());
                }
            }
        }

        bytes
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, StructuralError> {
        if bytes.len() < HEADER_SIZE || bytes.len() > MAX_BYTES || &bytes[..4] != MAGIC {
            return Err(StructuralError::InvalidFormat);
        }

        if bytes[4] != VERSION {
            return Err(StructuralError::UnsupportedVersion);
        }

        let read_u32 = |offset: usize| -> u32 {
            u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
        };

        let neurons = read_u32(5) as usize;
        let dendrites = read_u32(9) as usize;
        let connections = read_u32(13) as usize;

        if [neurons, dendrites, connections]
            .iter()
            .any(|count| *count > MAX_ITEMS)
        {
            return Err(StructuralError::LimitExceeded);
        }

        let expected = HEADER_SIZE
            .checked_add(neurons * 4)
            .and_then(|v| v.checked_add(dendrites * 8))
            .and_then(|v| v.checked_add(connections * 16))
            .ok_or(StructuralError::InvalidFormat)?;

        if bytes.len() != expected {
            return Err(StructuralError::InvalidFormat);
        }

        let mut offset = HEADER_SIZE;
        let mut topology = Self::default();

        for _ in 0..neurons {
            let id = NodeId::new(read_u32(offset)).map_err(|_| StructuralError::InvalidFormat)?;
            offset += 4;
            topology.add_neuron(id)?;
        }

        for _ in 0..dendrites {
            let neuron =
                NodeId::new(read_u32(offset)).map_err(|_| StructuralError::InvalidFormat)?;
            let dendrite =
                DendriteId::new(read_u32(offset + 4)).ok_or(StructuralError::InvalidFormat)?;
            offset += 8;
            topology.add_dendrite(neuron, dendrite)?;
        }

        for _ in 0..connections {
            let synapse =
                SynapseId::new(read_u32(offset)).map_err(|_| StructuralError::InvalidFormat)?;
            let source =
                NodeId::new(read_u32(offset + 4)).map_err(|_| StructuralError::InvalidFormat)?;
            let destination =
                NodeId::new(read_u32(offset + 8)).map_err(|_| StructuralError::InvalidFormat)?;
            let dendrite = read_u32(offset + 12);
            offset += 16;

            let target = if dendrite == 0 {
                Target::Soma(destination)
            } else {
                Target::Dendrite(destination, DendriteId::new(dendrite).unwrap())
            };

            topology.connect(synapse, source, target)?;
        }

        if topology.encode() != bytes {
            return Err(StructuralError::InvalidFormat);
        }

        Ok(topology)
    }
}

pub fn nid(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

pub fn sid(value: u32) -> SynapseId {
    SynapseId::new(value).unwrap()
}

pub fn did(value: u32) -> DendriteId {
    DendriteId::new(value).unwrap()
}
