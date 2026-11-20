use neurons::circuit::Circuit;
use neurons::dendrite::{Dendrite, DendriteAddress, DendriteId};
use neurons::event::NodeId;
use neurons::neuron::Neuron;
use neurons::synapse::{Synapse, SynapseId};

fn node(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn synapse(value: u32) -> SynapseId {
    SynapseId::new(value).unwrap()
}

fn dendrite(value: u32) -> DendriteId {
    DendriteId::new(value).unwrap()
}

fn build() -> Circuit {
    let mut circuit = Circuit::new(32, 1).unwrap();

    circuit
        .add_neuron(Neuron::new(node(1), 2, 10).unwrap())
        .unwrap();

    circuit
        .add_neuron(Neuron::new(node(2), 2, 10).unwrap())
        .unwrap();

    let address = DendriteAddress {
        neuron: node(2),
        dendrite: dendrite(1),
    };

    circuit
        .add_dendrite(address, Dendrite::new(2, 10).unwrap())
        .unwrap();

    circuit
        .connect_dendrite(
            Synapse::new(synapse(1), node(1), node(2), 1, 1).unwrap(),
            address,
        )
        .unwrap();

    circuit
}

#[test]
fn checkpoint_restores_empty_circuit_state() {
    let original = build();

    let snapshot = original.snapshot_v2().unwrap();
    let restored = Circuit::from_snapshot_v2(snapshot.clone()).unwrap();

    assert_eq!(restored.snapshot_v2().unwrap(), snapshot);
}

#[test]
fn checkpoint_preserves_pending_dendritic_delivery() {
    let mut original = build();

    original.stimulate(node(1), 2, 0).unwrap();
    original.run_cycle();

    assert_eq!(original.pending_dendritic_count(), 1);

    let snapshot = original.snapshot_v2().unwrap();
    let mut restored = Circuit::from_snapshot_v2(snapshot).unwrap();

    assert_eq!(restored.pending_dendritic_count(), 1);

    original.run_cycle();
    restored.run_cycle();

    assert_eq!(
        original.snapshot_v2().unwrap(),
        restored.snapshot_v2().unwrap()
    );
}

#[test]
fn checkpoint_preserves_deterministic_continuation() {
    let mut original = build();

    original.stimulate(node(1), 2, 0).unwrap();
    original.stimulate(node(1), 2, 1).unwrap();

    original.run_cycle();

    let snapshot = original.snapshot_v2().unwrap();
    let mut restored = Circuit::from_snapshot_v2(snapshot).unwrap();

    for _ in 0..12 {
        original.run_cycle();
        restored.run_cycle();

        assert_eq!(
            original.snapshot_v2().unwrap(),
            restored.snapshot_v2().unwrap()
        );
    }
}

#[test]
fn checkpoint_rejects_unknown_pending_sequence() {
    let mut original = build();

    original.stimulate(node(1), 2, 0).unwrap();
    original.run_cycle();

    let mut snapshot = original.snapshot_v2().unwrap();

    assert!(!snapshot.pending_dendritic.is_empty());

    snapshot.pending_dendritic[0].0 = u64::MAX;

    assert!(Circuit::from_snapshot_v2(snapshot).is_err());
}

#[test]
fn checkpoint_rejects_missing_neuron() {
    let original = build();

    let mut snapshot = original.snapshot_v2().unwrap();
    snapshot.neurons.pop();

    assert!(Circuit::from_snapshot_v2(snapshot).is_err());
}
