#[path = "support/dendritic_model.rs"]
mod dendritic_model;

use dendritic_model::{Dendrite, DendriteError, DendriticNeuron};
use neurons::{
    event::{Event, EventKind, NodeId},
    neuron::Neuron,
};

fn model() -> DendriticNeuron {
    DendriticNeuron::new(2, 10, 1, 10).unwrap()
}

fn execute(pattern: &[(usize, i32)]) -> DendriticNeuron {
    let mut neuron = model();
    for &(compartment, signal) in pattern {
        neuron.receive(compartment, signal).unwrap();
    }
    neuron
}

fn soma_baseline(signals: &[i32]) -> u64 {
    let id = NodeId::new(1).unwrap();
    let mut neuron = Neuron::new(id, 2, 10).unwrap();

    for &signal in signals {
        neuron
            .receive(Event::new(None, id, EventKind::Stimulus, signal, 0))
            .unwrap();
    }

    neuron.state().activations
}

#[test]
fn equal_total_inputs_can_produce_different_dendritic_responses() {
    let p1 = execute(&[(0, 2), (1, 0)]);
    let p2 = execute(&[(0, 1), (1, 1)]);

    assert_eq!(p1.activations(), 1);
    assert_eq!(p2.activations(), 0);
    assert_eq!(p2.potentials(), [1, 1]);

    assert_eq!(soma_baseline(&[2, 0]), 1);
    assert_eq!(soma_baseline(&[1, 1]), 1);
}

#[test]
fn compartments_preserve_independent_local_states() {
    let mut neuron = model();

    neuron.receive(0, 1).unwrap();
    assert_eq!(neuron.potentials(), [1, 0]);

    neuron.receive(1, -3).unwrap();
    assert_eq!(neuron.potentials(), [1, -3]);
    assert_eq!(neuron.activations(), 0);
}

#[test]
fn excitation_and_inhibition_are_integrated_locally() {
    let mut neuron = model();

    neuron.receive(0, 3).unwrap();
    assert_eq!(neuron.activations(), 1);

    neuron.receive(1, -2).unwrap();
    neuron.receive(1, 3).unwrap();

    assert_eq!(neuron.activations(), 1);
    assert_eq!(neuron.potentials(), [0, 1]);
}

#[test]
fn repeated_inputs_are_deterministic() {
    let pattern = [(0, 1), (1, -1), (0, 1), (1, 2), (1, 1)];

    assert_eq!(execute(&pattern), execute(&pattern));
}

#[test]
fn local_potential_is_bounded_at_numeric_extremes() {
    let mut dendrite = Dendrite::new(2, 10).unwrap();

    assert!(!dendrite.receive(i32::MIN));
    assert_eq!(dendrite.potential(), -10);

    assert!(dendrite.receive(i32::MAX));
    assert_eq!(dendrite.potential(), 0);

    assert!(dendrite.receive(i32::MAX));
    assert_eq!(dendrite.potential(), 0);
}

#[test]
fn invalid_configuration_is_rejected() {
    assert_eq!(
        Dendrite::new(0, 10),
        Err(DendriteError::InvalidConfiguration)
    );

    assert_eq!(
        DendriticNeuron::new(2, 1, 1, 10),
        Err(DendriteError::InvalidConfiguration)
    );

    assert_eq!(
        DendriticNeuron::new(2, 10, 0, 10),
        Err(DendriteError::InvalidConfiguration)
    );
}

#[test]
fn unknown_compartment_does_not_mutate_state() {
    let mut neuron = model();
    let before = neuron.clone();

    assert_eq!(
        neuron.receive(2, 10),
        Err(DendriteError::UnknownCompartment)
    );

    assert_eq!(neuron, before);
}

#[test]
fn soma_requires_configured_number_of_dendritic_contributions() {
    let mut neuron = DendriticNeuron::new(2, 10, 2, 10).unwrap();

    assert!(!neuron.receive(0, 2).unwrap());
    assert_eq!(neuron.soma_potential(), 1);

    assert!(neuron.receive(1, 2).unwrap());
    assert_eq!(neuron.soma_potential(), 0);
    assert_eq!(neuron.activations(), 1);
}
