#[path = "support/dendritic_structural_model.rs"]
mod model;

use model::{did, nid, sid, Connection, StructuralError, StructuralTopology, Target};

fn topology() -> StructuralTopology {
    let mut t = StructuralTopology::default();
    t.add_neuron(nid(1)).unwrap();
    t.add_neuron(nid(2)).unwrap();
    t
}

#[test]
fn registered_neurons_accept_somatic_connection() {
    let mut t = topology();
    t.connect(sid(1), nid(1), Target::Soma(nid(2))).unwrap();

    assert_eq!(
        t.connection(sid(1)),
        Some(Connection {
            source: nid(1),
            target: Target::Soma(nid(2)),
        })
    );
}

#[test]
fn duplicate_neuron_is_rejected() {
    let mut t = topology();

    assert_eq!(t.add_neuron(nid(1)), Err(StructuralError::DuplicateNeuron));
}

#[test]
fn unknown_source_is_rejected_without_mutation() {
    let mut t = topology();

    assert_eq!(
        t.connect(sid(1), nid(3), Target::Soma(nid(2))),
        Err(StructuralError::UnknownNeuron)
    );
    assert_eq!(t.counts(), (2, 0, 0));
}

#[test]
fn unknown_destination_is_rejected() {
    let mut t = topology();

    assert_eq!(
        t.connect(sid(1), nid(1), Target::Soma(nid(3))),
        Err(StructuralError::UnknownNeuron)
    );
    assert_eq!(t.counts(), (2, 0, 0));
}

#[test]
fn dendrite_requires_registered_owner() {
    let mut t = topology();

    assert_eq!(
        t.add_dendrite(nid(3), did(1)),
        Err(StructuralError::UnknownNeuron)
    );
}

#[test]
fn duplicate_dendrite_is_rejected() {
    let mut t = topology();
    t.add_dendrite(nid(2), did(1)).unwrap();

    assert_eq!(
        t.add_dendrite(nid(2), did(1)),
        Err(StructuralError::DuplicateDendrite)
    );
}

#[test]
fn wrong_neuron_dendrite_is_rejected() {
    let mut t = topology();
    t.add_dendrite(nid(1), did(1)).unwrap();

    assert_eq!(
        t.connect(sid(1), nid(1), Target::Dendrite(nid(2), did(1)),),
        Err(StructuralError::WrongNeuron)
    );
    assert_eq!(t.counts(), (2, 1, 0));
}

#[test]
fn unknown_dendrite_is_rejected() {
    let mut t = topology();

    assert_eq!(
        t.connect(sid(1), nid(1), Target::Dendrite(nid(2), did(7)),),
        Err(StructuralError::UnknownDendrite)
    );
}

#[test]
fn duplicate_synapse_does_not_replace_route() {
    let mut t = topology();
    t.connect(sid(1), nid(1), Target::Soma(nid(2))).unwrap();

    assert_eq!(
        t.connect(sid(1), nid(2), Target::Soma(nid(1))),
        Err(StructuralError::DuplicateSynapse)
    );

    assert_eq!(t.connection(sid(1)).unwrap().source, nid(1));
}

#[test]
fn canonical_topology_roundtrip_preserves_routes() {
    let mut t = topology();
    t.add_dendrite(nid(2), did(1)).unwrap();
    t.add_dendrite(nid(2), did(2)).unwrap();

    t.connect(sid(1), nid(1), Target::Dendrite(nid(2), did(1)))
        .unwrap();

    t.connect(sid(2), nid(1), Target::Dendrite(nid(2), did(2)))
        .unwrap();

    t.connect(sid(3), nid(2), Target::Soma(nid(1))).unwrap();

    let encoded = t.encode();
    let restored = StructuralTopology::decode(&encoded).unwrap();

    assert_eq!(restored.encode(), encoded);
    assert_eq!(restored.counts(), (2, 2, 3));
    assert_eq!(restored.connection(sid(1)), t.connection(sid(1)));
    assert_eq!(restored.connection(sid(2)), t.connection(sid(2)));
    assert_eq!(restored.connection(sid(3)), t.connection(sid(3)));
}

#[test]
fn truncated_topology_is_rejected() {
    let t = topology();
    let mut bytes = t.encode();
    bytes.pop();

    assert!(matches!(
        StructuralTopology::decode(&bytes),
        Err(StructuralError::InvalidFormat)
    ));
}

#[test]
fn unsupported_version_is_rejected() {
    let t = topology();
    let mut bytes = t.encode();
    bytes[4] = 2;

    assert!(matches!(
        StructuralTopology::decode(&bytes),
        Err(StructuralError::UnsupportedVersion)
    ));
}

#[test]
fn malformed_connection_is_rejected() {
    let mut t = topology();
    t.connect(sid(1), nid(1), Target::Soma(nid(2))).unwrap();

    let mut bytes = t.encode();

    // Cabecalho 17 + dois neuronios de 4 bytes.
    // Registro sinaptico: ID, origem, destino, dendrito.
    let offset = 17 + 2 * 4;

    // Altera destino para neuronio inexistente.
    bytes[offset + 8..offset + 12].copy_from_slice(&99_u32.to_le_bytes());

    assert!(matches!(
        StructuralTopology::decode(&bytes),
        Err(StructuralError::UnknownNeuron)
    ));
}

#[test]
fn zero_identifier_in_serialized_topology_is_rejected() {
    let t = topology();
    let mut bytes = t.encode();

    // Primeiro NodeId depois do cabecalho.
    bytes[17..21].copy_from_slice(&0_u32.to_le_bytes());

    assert!(matches!(
        StructuralTopology::decode(&bytes),
        Err(StructuralError::InvalidFormat)
    ));
}

#[test]
fn identical_topologies_encode_identically() {
    let mut a = topology();
    let mut b = topology();

    a.add_dendrite(nid(2), did(2)).unwrap();
    a.add_dendrite(nid(2), did(1)).unwrap();

    b.add_dendrite(nid(2), did(1)).unwrap();
    b.add_dendrite(nid(2), did(2)).unwrap();

    assert_eq!(a.encode(), b.encode());
}
