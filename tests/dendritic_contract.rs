#[path = "support/dendritic_contract_model.rs"]
mod model;

use model::{
    nid, sid, ContractError, DeliveryTarget, DendriteAddress, DendriteId, DendriticTopology,
};

fn address(neuron: u32, dendrite: u32) -> DendriteAddress {
    DendriteAddress {
        neuron: nid(neuron),
        dendrite: DendriteId::new(dendrite).unwrap(),
    }
}

#[test]
fn dendrite_identity_rejects_zero() {
    assert_eq!(DendriteId::new(0), None);
    assert_eq!(DendriteId::new(1).unwrap().get(), 1);
}

#[test]
fn dendrite_identity_is_local_to_neuron() {
    let mut topology = DendriticTopology::default();

    topology.register(address(1, 1)).unwrap();
    topology.register(address(2, 1)).unwrap();

    assert_eq!(topology.dendrite_count(), 2);
}

#[test]
fn duplicate_dendrite_is_rejected() {
    let mut topology = DendriticTopology::default();

    topology.register(address(2, 1)).unwrap();

    assert_eq!(
        topology.register(address(2, 1)),
        Err(ContractError::DuplicateDendrite)
    );
}

#[test]
fn somatic_route_preserves_legacy_semantics() {
    let mut topology = DendriticTopology::default();

    topology.connect(sid(1), nid(2), None).unwrap();

    assert_eq!(topology.target(sid(1)), Some(DeliveryTarget::Soma(nid(2))));
}

#[test]
fn dendritic_route_requires_registered_address() {
    let mut topology = DendriticTopology::default();

    assert_eq!(
        topology.connect(sid(1), nid(2), DendriteId::new(1)),
        Err(ContractError::UnknownDendrite)
    );

    assert_eq!(topology.route_count(), 0);
}

#[test]
fn dendritic_route_is_explicit() {
    let mut topology = DendriticTopology::default();

    topology.register(address(2, 1)).unwrap();

    topology
        .connect(sid(1), nid(2), DendriteId::new(1))
        .unwrap();

    assert_eq!(
        topology.target(sid(1)),
        Some(DeliveryTarget::Dendrite(address(2, 1)))
    );
}

#[test]
fn duplicate_synapse_is_rejected_without_mutation() {
    let mut topology = DendriticTopology::default();

    topology.connect(sid(1), nid(2), None).unwrap();

    assert_eq!(
        topology.connect(sid(1), nid(3), None),
        Err(ContractError::DuplicateSynapse)
    );

    assert_eq!(topology.route_count(), 1);
    assert_eq!(topology.target(sid(1)), Some(DeliveryTarget::Soma(nid(2))));
}

#[test]
fn configuration_roundtrip_is_canonical() {
    let mut topology = DendriticTopology::default();

    topology.register(address(2, 2)).unwrap();
    topology.register(address(2, 1)).unwrap();
    topology.register(address(3, 1)).unwrap();

    let encoded = topology.encode_configuration();
    let decoded = DendriticTopology::decode_configuration(&encoded).unwrap();

    assert_eq!(decoded.encode_configuration(), encoded);
    assert_eq!(decoded.dendrite_count(), 3);
}

#[test]
fn malformed_configuration_is_rejected() {
    let mut topology = DendriticTopology::default();
    topology.register(address(2, 1)).unwrap();

    let mut bytes = topology.encode_configuration();
    bytes.pop();

    assert!(matches!(
        DendriticTopology::decode_configuration(&bytes),
        Err(ContractError::InvalidFormat)
    ));
}

#[test]
fn unsupported_configuration_version_is_rejected() {
    let mut bytes = DendriticTopology::default().encode_configuration();

    bytes[4] = 2;

    assert!(matches!(
        DendriticTopology::decode_configuration(&bytes),
        Err(ContractError::UnsupportedVersion)
    ));
}
