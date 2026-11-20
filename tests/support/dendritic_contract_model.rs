use std::collections::{BTreeMap, BTreeSet};

use neurons::event::NodeId;
use neurons::synapse::SynapseId;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DendriteId(u32);

impl DendriteId {
    pub fn new(value: u32) -> Option<Self> {
        (value != 0).then_some(Self(value))
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DendriteAddress {
    pub neuron: NodeId,
    pub dendrite: DendriteId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryTarget {
    Soma(NodeId),
    Dendrite(DendriteAddress),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractError {
    DuplicateDendrite,
    DuplicateSynapse,
    UnknownDendrite,
    WrongNeuron,
    InvalidFormat,
    UnsupportedVersion,
}

#[derive(Default)]
pub struct DendriticTopology {
    dendrites: BTreeSet<DendriteAddress>,
    routes: BTreeMap<SynapseId, DeliveryTarget>,
}

impl DendriticTopology {
    pub fn register(&mut self, address: DendriteAddress) -> Result<(), ContractError> {
        if !self.dendrites.insert(address) {
            return Err(ContractError::DuplicateDendrite);
        }
        Ok(())
    }

    pub fn connect(
        &mut self,
        synapse: SynapseId,
        destination: NodeId,
        dendrite: Option<DendriteId>,
    ) -> Result<(), ContractError> {
        if self.routes.contains_key(&synapse) {
            return Err(ContractError::DuplicateSynapse);
        }

        let target = match dendrite {
            None => DeliveryTarget::Soma(destination),
            Some(dendrite) => {
                let address = DendriteAddress {
                    neuron: destination,
                    dendrite,
                };

                if !self.dendrites.contains(&address) {
                    return Err(ContractError::UnknownDendrite);
                }

                DeliveryTarget::Dendrite(address)
            }
        };

        self.routes.insert(synapse, target);
        Ok(())
    }

    pub fn target(&self, synapse: SynapseId) -> Option<DeliveryTarget> {
        self.routes.get(&synapse).copied()
    }

    pub fn dendrite_count(&self) -> usize {
        self.dendrites.len()
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    // Formato experimental de configuracao, nao checkpoint.
    // Cabecalho: DENB + versao u8 + quantidade u32 LE.
    // Registros: NodeId u32 LE + DendriteId u32 LE.
    pub fn encode_configuration(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"DENB");
        bytes.push(1);
        bytes.extend_from_slice(&(self.dendrites.len() as u32).to_le_bytes());

        for address in &self.dendrites {
            bytes.extend_from_slice(&address.neuron.get().to_le_bytes());
            bytes.extend_from_slice(&address.dendrite.get().to_le_bytes());
        }

        bytes
    }

    pub fn decode_configuration(bytes: &[u8]) -> Result<Self, ContractError> {
        if bytes.len() < 9 || &bytes[..4] != b"DENB" {
            return Err(ContractError::InvalidFormat);
        }

        if bytes[4] != 1 {
            return Err(ContractError::UnsupportedVersion);
        }

        let count = u32::from_le_bytes(bytes[5..9].try_into().unwrap()) as usize;

        let expected = count
            .checked_mul(8)
            .and_then(|size| size.checked_add(9))
            .ok_or(ContractError::InvalidFormat)?;

        if bytes.len() != expected {
            return Err(ContractError::InvalidFormat);
        }

        let mut topology = Self::default();

        for index in 0..count {
            let offset = 9 + index * 8;

            let neuron = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());

            let dendrite = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap());

            let neuron = NodeId::new(neuron).map_err(|_| ContractError::InvalidFormat)?;

            let dendrite = DendriteId::new(dendrite).ok_or(ContractError::InvalidFormat)?;

            topology.register(DendriteAddress { neuron, dendrite })?;
        }

        if topology.encode_configuration() != bytes {
            return Err(ContractError::InvalidFormat);
        }

        Ok(topology)
    }
}

pub fn sid(value: u32) -> SynapseId {
    SynapseId::new(value).unwrap()
}

pub fn nid(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}
