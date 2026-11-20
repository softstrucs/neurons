//! Compartimentos dendriticos experimentais e enderecamento explicito.

use std::collections::{BTreeMap, BTreeSet};

use crate::event::NodeId;
use crate::synapse::SynapseId;

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
pub enum DendriteError {
    InvalidConfiguration,
    DuplicateDendrite,
    DuplicateSynapse,
    UnknownDendrite,
    WrongNeuron,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dendrite {
    potential: i32,
    threshold: i32,
    limit: i32,
}

impl Dendrite {
    pub fn new(threshold: i32, limit: i32) -> Result<Self, DendriteError> {
        if threshold <= 0 || limit < threshold {
            return Err(DendriteError::InvalidConfiguration);
        }

        Ok(Self {
            potential: 0,
            threshold,
            limit,
        })
    }

    pub const fn potential(&self) -> i32 {
        self.potential
    }

    /// Retorna true quando o compartimento atinge seu limiar.
    pub fn receive(&mut self, signal: i32) -> bool {
        let sum = i64::from(self.potential) + i64::from(signal);
        let limit = i64::from(self.limit);

        self.potential = sum.clamp(-limit, limit) as i32;

        if self.potential >= self.threshold {
            self.potential = 0;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DendriteSnapshot {
    pub potential: i32,
    pub threshold: i32,
    pub limit: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DendriteRestoreError {
    Configuration(DendriteError),
    InvalidState,
}

impl Dendrite {
    pub fn snapshot(&self) -> DendriteSnapshot {
        DendriteSnapshot {
            potential: self.potential,
            threshold: self.threshold,
            limit: self.limit,
        }
    }

    pub fn from_snapshot(snapshot: DendriteSnapshot) -> Result<Self, DendriteRestoreError> {
        let mut dendrite = Self::new(snapshot.threshold, snapshot.limit)
            .map_err(DendriteRestoreError::Configuration)?;

        let potential = i64::from(snapshot.potential);
        let limit = i64::from(snapshot.limit);

        if potential < -limit || potential >= i64::from(snapshot.threshold) {
            return Err(DendriteRestoreError::InvalidState);
        }

        dendrite.potential = snapshot.potential;
        Ok(dendrite)
    }
}

#[derive(Default)]
pub struct DendriticTopology {
    dendrites: BTreeSet<DendriteAddress>,
    routes: BTreeMap<SynapseId, DeliveryTarget>,
}

impl DendriticTopology {
    pub fn register(&mut self, address: DendriteAddress) -> Result<(), DendriteError> {
        if !self.dendrites.insert(address) {
            return Err(DendriteError::DuplicateDendrite);
        }

        Ok(())
    }

    pub fn connect(
        &mut self,
        synapse: SynapseId,
        destination: NodeId,
        dendrite: Option<DendriteId>,
    ) -> Result<(), DendriteError> {
        if self.routes.contains_key(&synapse) {
            return Err(DendriteError::DuplicateSynapse);
        }

        let target = match dendrite {
            None => DeliveryTarget::Soma(destination),
            Some(dendrite) => {
                let address = DendriteAddress {
                    neuron: destination,
                    dendrite,
                };

                if !self.dendrites.contains(&address) {
                    return Err(DendriteError::UnknownDendrite);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nid(value: u32) -> NodeId {
        NodeId::new(value).unwrap()
    }

    fn sid(value: u32) -> SynapseId {
        SynapseId::new(value).unwrap()
    }

    #[test]
    fn dendrite_id_rejects_zero() {
        assert_eq!(DendriteId::new(0), None);
        assert_eq!(DendriteId::new(1).unwrap().get(), 1);
    }

    #[test]
    fn dendrite_accumulates_and_resets_at_threshold() {
        let mut dendrite = Dendrite::new(3, 10).unwrap();

        assert!(!dendrite.receive(1));
        assert_eq!(dendrite.potential(), 1);

        assert!(!dendrite.receive(1));
        assert_eq!(dendrite.potential(), 2);

        assert!(dendrite.receive(1));
        assert_eq!(dendrite.potential(), 0);
    }

    #[test]
    fn dendrite_rejects_invalid_configuration() {
        assert_eq!(
            Dendrite::new(0, 10),
            Err(DendriteError::InvalidConfiguration)
        );

        assert_eq!(
            Dendrite::new(5, 4),
            Err(DendriteError::InvalidConfiguration)
        );
    }

    #[test]
    fn distinct_dendrites_have_distinct_addresses() {
        let first = DendriteAddress {
            neuron: nid(1),
            dendrite: DendriteId::new(1).unwrap(),
        };

        let second = DendriteAddress {
            neuron: nid(2),
            dendrite: DendriteId::new(1).unwrap(),
        };

        assert_ne!(first, second);
    }

    #[test]
    fn topology_rejects_duplicate_dendrite() {
        let address = DendriteAddress {
            neuron: nid(2),
            dendrite: DendriteId::new(1).unwrap(),
        };

        let mut topology = DendriticTopology::default();

        assert_eq!(topology.register(address), Ok(()));
        assert_eq!(
            topology.register(address),
            Err(DendriteError::DuplicateDendrite)
        );
    }

    #[test]
    fn topology_routes_synapses_explicitly() {
        let address = DendriteAddress {
            neuron: nid(2),
            dendrite: DendriteId::new(1).unwrap(),
        };

        let mut topology = DendriticTopology::default();

        topology.register(address).unwrap();

        topology
            .connect(sid(1), nid(2), Some(address.dendrite))
            .unwrap();

        topology.connect(sid(2), nid(2), None).unwrap();

        assert_eq!(
            topology.target(sid(1)),
            Some(DeliveryTarget::Dendrite(address))
        );

        assert_eq!(topology.target(sid(2)), Some(DeliveryTarget::Soma(nid(2))));

        assert_eq!(topology.dendrite_count(), 1);
        assert_eq!(topology.route_count(), 2);
    }

    #[test]
    fn topology_rejects_unknown_dendrite_without_partial_route() {
        let mut topology = DendriticTopology::default();

        assert_eq!(
            topology.connect(sid(1), nid(2), Some(DendriteId::new(1).unwrap())),
            Err(DendriteError::UnknownDendrite)
        );

        assert_eq!(topology.route_count(), 0);
    }
}
