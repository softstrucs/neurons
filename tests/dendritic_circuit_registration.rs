use neurons::circuit::{Circuit, CircuitError};
use neurons::dendrite::{Dendrite, DendriteAddress, DendriteError, DendriteId};
use neurons::event::NodeId;
use neurons::neuron::Neuron;
use neurons::persistence::PersistenceError;

fn nid(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn address(neuron: u32, dendrite: u32) -> DendriteAddress {
    DendriteAddress {
        neuron: nid(neuron),
        dendrite: DendriteId::new(dendrite).unwrap(),
    }
}

fn circuit() -> Circuit {
    let mut circuit = Circuit::new(8, 8).unwrap();

    circuit
        .add_neuron(Neuron::new(nid(1), 2, 10).unwrap())
        .unwrap();

    circuit
        .add_neuron(Neuron::new(nid(2), 2, 10).unwrap())
        .unwrap();

    circuit
}

#[test]
fn registers_multiple_independent_dendrites() {
    let mut circuit = circuit();

    circuit
        .add_dendrite(address(1, 1), Dendrite::new(2, 10).unwrap())
        .unwrap();

    circuit
        .add_dendrite(address(1, 2), Dendrite::new(3, 10).unwrap())
        .unwrap();

    circuit
        .add_dendrite(address(2, 1), Dendrite::new(2, 10).unwrap())
        .unwrap();

    assert_eq!(circuit.dendrite_count(), 3);
    assert_eq!(circuit.dendrite_potential(address(1, 1)), Ok(0));
    assert_eq!(circuit.dendrite_potential(address(1, 2)), Ok(0));
    assert_eq!(circuit.dendrite_potential(address(2, 1)), Ok(0));
}

#[test]
fn rejects_duplicate_dendrite_without_modification() {
    let mut circuit = circuit();
    let id = address(1, 1);

    circuit
        .add_dendrite(id, Dendrite::new(2, 10).unwrap())
        .unwrap();

    assert_eq!(
        circuit.add_dendrite(id, Dendrite::new(5, 20).unwrap()),
        Err(CircuitError::Dendrite(DendriteError::DuplicateDendrite))
    );

    assert_eq!(circuit.dendrite_count(), 1);
    assert_eq!(circuit.dendrite_potential(id), Ok(0));
}

#[test]
fn rejects_dendrite_without_existing_neuron() {
    let mut circuit = circuit();

    assert_eq!(
        circuit.add_dendrite(address(3, 1), Dendrite::new(2, 10).unwrap(),),
        Err(CircuitError::UnknownNeuron(nid(3)))
    );

    assert_eq!(circuit.dendrite_count(), 0);
}

#[test]
fn rejects_unknown_dendrite_query() {
    let circuit = circuit();
    let id = address(1, 1);

    assert_eq!(
        circuit.dendrite_potential(id),
        Err(CircuitError::UnknownDendrite(id))
    );
}

#[test]
fn dendritic_snapshot_fails_explicitly() {
    let mut circuit = circuit();

    circuit
        .add_dendrite(address(1, 1), Dendrite::new(2, 10).unwrap())
        .unwrap();

    assert!(matches!(
        circuit.snapshot(),
        Err(PersistenceError::DendriticSnapshotUnsupported)
    ));
}

#[test]
fn somatic_snapshot_remains_supported() {
    let circuit = circuit();

    let snapshot = circuit.snapshot().unwrap();

    assert_eq!(snapshot.neurons.len(), 2);
    assert!(snapshot.synapses.is_empty());
    assert!(!snapshot.engine.is_empty());
}
