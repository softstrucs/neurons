use neurons::circuit::{Circuit, CircuitError};
use neurons::dendrite::{DeliveryTarget, Dendrite, DendriteAddress, DendriteError, DendriteId};
use neurons::event::NodeId;
use neurons::neuron::Neuron;
use neurons::persistence::PersistenceError;
use neurons::synapse::{Synapse, SynapseId};

fn nid(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn sid(value: u32) -> SynapseId {
    SynapseId::new(value).unwrap()
}

fn address(neuron: u32, dendrite: u32) -> DendriteAddress {
    DendriteAddress {
        neuron: nid(neuron),
        dendrite: DendriteId::new(dendrite).unwrap(),
    }
}

fn synapse(id: u32, source: u32, destination: u32) -> Synapse {
    Synapse::new(sid(id), nid(source), nid(destination), 1, 1).unwrap()
}

fn circuit(capacity: usize, cycle_limit: usize) -> Circuit {
    let mut circuit = Circuit::new(capacity, cycle_limit).unwrap();

    for id in 1..=3 {
        circuit
            .add_neuron(Neuron::new(nid(id), 1, 10).unwrap())
            .unwrap();
    }

    circuit
}

#[test]
fn identical_events_reach_distinct_dendrites() {
    let mut circuit = circuit(8, 1);
    let first = address(2, 1);
    let second = address(2, 2);

    for target in [first, second] {
        circuit
            .add_dendrite(target, Dendrite::new(5, 10).unwrap())
            .unwrap();
    }

    circuit.connect_dendrite(synapse(1, 1, 2), first).unwrap();
    circuit.connect_dendrite(synapse(2, 1, 2), second).unwrap();

    circuit.stimulate(nid(1), 1, 0).unwrap();

    let report = circuit.run_cycle();

    assert_eq!(report.transmissions, 2);
    assert_eq!(circuit.pending_dendritic_count(), 2);
    assert_eq!(circuit.queue_len(), 2);

    circuit.run_cycle();

    assert_eq!(circuit.dendrite_potential(first), Ok(1));
    assert_eq!(circuit.dendrite_potential(second), Ok(0));
    assert_eq!(circuit.pending_dendritic_count(), 1);

    circuit.run_cycle();

    assert_eq!(circuit.dendrite_potential(first), Ok(1));
    assert_eq!(circuit.dendrite_potential(second), Ok(1));
    assert_eq!(circuit.pending_dendritic_count(), 0);

    // Nenhum sinal dendritico deve ativar diretamente o soma na D4.2.
    assert_eq!(circuit.neuron_state(nid(2)).unwrap().activations, 0);
}

#[test]
fn queue_rejection_does_not_create_orphan_routes() {
    let mut circuit = circuit(1, 1);
    let first = address(2, 1);
    let second = address(3, 1);

    for target in [first, second] {
        circuit
            .add_dendrite(target, Dendrite::new(5, 10).unwrap())
            .unwrap();
    }

    circuit.connect_dendrite(synapse(1, 1, 2), first).unwrap();
    circuit.connect_dendrite(synapse(2, 1, 3), second).unwrap();

    circuit.stimulate(nid(1), 1, 0).unwrap();

    let report = circuit.run_cycle();

    assert_eq!(report.transmissions, 1);
    assert_eq!(report.rejected_transmissions, 1);
    assert_eq!(circuit.pending_dendritic_count(), 1);
    assert_eq!(circuit.queue_len(), 1);

    circuit.run_cycle();

    assert_eq!(circuit.dendrite_potential(first), Ok(1));
    assert_eq!(circuit.dendrite_potential(second), Ok(0));
    assert_eq!(circuit.pending_dendritic_count(), 0);
    assert_eq!(circuit.queue_len(), 0);
}

#[test]
fn somatic_and_dendritic_routes_remain_independent() {
    let mut circuit = circuit(8, 1);
    let target = address(3, 1);

    circuit
        .add_dendrite(target, Dendrite::new(5, 10).unwrap())
        .unwrap();

    circuit.connect(synapse(1, 1, 2)).unwrap();
    circuit.connect_dendrite(synapse(2, 1, 3), target).unwrap();

    assert_eq!(
        circuit.delivery_target(sid(1)),
        Ok(DeliveryTarget::Soma(nid(2)))
    );

    assert_eq!(
        circuit.delivery_target(sid(2)),
        Ok(DeliveryTarget::Dendrite(target))
    );

    circuit.stimulate(nid(1), 1, 0).unwrap();

    circuit.run_cycle();
    circuit.run_cycle();
    circuit.run_cycle();

    assert_eq!(circuit.neuron_state(nid(2)).unwrap().activations, 1);
    assert_eq!(circuit.neuron_state(nid(3)).unwrap().activations, 0);
    assert_eq!(circuit.dendrite_potential(target), Ok(1));
    assert_eq!(circuit.pending_dendritic_count(), 0);
}

#[test]
fn invalid_dendritic_connections_are_atomic() {
    let mut circuit = circuit(8, 8);
    let target = address(2, 1);

    circuit
        .add_dendrite(target, Dendrite::new(5, 10).unwrap())
        .unwrap();

    assert_eq!(
        circuit.connect_dendrite(synapse(1, 1, 3), target),
        Err(CircuitError::Dendrite(DendriteError::WrongNeuron))
    );

    assert_eq!(
        circuit.delivery_target(sid(1)),
        Err(CircuitError::UnknownSynapse(sid(1)))
    );

    let missing = address(2, 2);

    assert_eq!(
        circuit.connect_dendrite(synapse(2, 1, 2), missing),
        Err(CircuitError::UnknownDendrite(missing))
    );

    assert_eq!(
        circuit.delivery_target(sid(2)),
        Err(CircuitError::UnknownSynapse(sid(2)))
    );

    circuit.connect_dendrite(synapse(3, 1, 2), target).unwrap();

    assert_eq!(
        circuit.connect_dendrite(synapse(3, 1, 2), target),
        Err(CircuitError::DuplicateSynapse(sid(3)))
    );

    assert_eq!(
        circuit.delivery_target(sid(3)),
        Ok(DeliveryTarget::Dendrite(target))
    );
}

#[test]
fn dendritic_pending_events_cannot_be_snapshotted_as_v1() {
    let mut circuit = circuit(8, 1);
    let target = address(2, 1);

    circuit
        .add_dendrite(target, Dendrite::new(5, 10).unwrap())
        .unwrap();

    circuit.connect_dendrite(synapse(1, 1, 2), target).unwrap();

    circuit.stimulate(nid(1), 1, 0).unwrap();
    circuit.run_cycle();

    assert_eq!(circuit.pending_dendritic_count(), 1);

    assert!(matches!(
        circuit.snapshot(),
        Err(PersistenceError::DendriticSnapshotUnsupported)
    ));
}
