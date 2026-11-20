use neurons::dendrite::{
    DeliveryTarget, Dendrite, DendriteAddress, DendriteError, DendriteId, DendriticTopology,
};
use neurons::event::NodeId;
use neurons::synapse::SynapseId;

fn nid(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn did(value: u32) -> DendriteId {
    DendriteId::new(value).unwrap()
}

fn sid(value: u32) -> SynapseId {
    SynapseId::new(value).unwrap()
}

#[test]
fn independent_dendrites_preserve_separate_potentials() {
    let mut first = Dendrite::new(5, 10).unwrap();
    let mut second = Dendrite::new(5, 10).unwrap();

    assert!(!first.receive(3));
    assert!(!second.receive(2));

    assert_eq!(first.potential(), 3);
    assert_eq!(second.potential(), 2);

    assert!(first.receive(2));

    assert_eq!(first.potential(), 0);
    assert_eq!(second.potential(), 2);
}

#[test]
fn extreme_signals_are_clamped_without_overflow() {
    let mut dendrite = Dendrite::new(5, 10).unwrap();

    assert!(!dendrite.receive(i32::MIN));
    assert_eq!(dendrite.potential(), -10);

    assert!(dendrite.receive(i32::MAX));
    assert_eq!(dendrite.potential(), 0);

    assert!(!dendrite.receive(i32::MIN));
    assert_eq!(dendrite.potential(), -10);
}

#[test]
fn duplicate_synapse_does_not_replace_existing_route() {
    let address = DendriteAddress {
        neuron: nid(2),
        dendrite: did(1),
    };

    let mut topology = DendriticTopology::default();

    topology.register(address).unwrap();

    topology.connect(sid(1), nid(2), Some(did(1))).unwrap();

    assert_eq!(
        topology.connect(sid(1), nid(2), None),
        Err(DendriteError::DuplicateSynapse)
    );

    assert_eq!(
        topology.target(sid(1)),
        Some(DeliveryTarget::Dendrite(address))
    );

    assert_eq!(topology.route_count(), 1);
}

#[test]
fn same_dendrite_id_is_local_to_each_neuron() {
    let first = DendriteAddress {
        neuron: nid(1),
        dendrite: did(1),
    };

    let second = DendriteAddress {
        neuron: nid(2),
        dendrite: did(1),
    };

    let mut topology = DendriticTopology::default();

    topology.register(first).unwrap();
    topology.register(second).unwrap();

    topology.connect(sid(1), nid(1), Some(did(1))).unwrap();

    topology.connect(sid(2), nid(2), Some(did(1))).unwrap();

    assert_eq!(
        topology.target(sid(1)),
        Some(DeliveryTarget::Dendrite(first))
    );

    assert_eq!(
        topology.target(sid(2)),
        Some(DeliveryTarget::Dendrite(second))
    );

    assert_eq!(topology.dendrite_count(), 2);
    assert_eq!(topology.route_count(), 2);
}
