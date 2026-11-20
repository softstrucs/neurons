#[path = "support/dendritic_model.rs"]
mod dendritic_model;

use dendritic_model::{Dendrite, DendriticNeuron};

fn run(signals: &[(usize, i32)]) -> DendriticNeuron {
    let mut neuron = DendriticNeuron::new(2, 10, 1, 10).unwrap();

    for &(compartment, signal) in signals {
        neuron.receive(compartment, signal).unwrap();
    }

    neuron
}

#[test]
fn signal_order_can_change_activation() {
    let first = run(&[(0, 2), (0, -2)]);
    let second = run(&[(0, -2), (0, 2)]);

    assert_eq!(first.activations(), 1);
    assert_eq!(second.activations(), 0);
}

#[test]
fn independent_compartment_order_is_equivalent() {
    let first = run(&[(0, 1), (1, -1)]);
    let second = run(&[(1, -1), (0, 1)]);

    assert_eq!(first, second);
}

#[test]
fn repeated_subthreshold_signals_accumulate() {
    let neuron = run(&[(0, 1), (0, 1)]);

    assert_eq!(neuron.activations(), 1);
    assert_eq!(neuron.potentials(), [0, 0]);
}

#[test]
fn inhibition_can_prevent_local_activation() {
    let neuron = run(&[(0, 1), (0, -2), (0, 2)]);

    assert_eq!(neuron.activations(), 0);
    assert_eq!(neuron.potentials(), [1, 0]);
}

#[test]
fn long_extreme_sequence_remains_bounded_and_deterministic() {
    fn execute() -> DendriticNeuron {
        let mut neuron = DendriticNeuron::new(2, 10, 1, 10).unwrap();

        for i in 0..10_000 {
            let signal = if i % 2 == 0 { i32::MAX } else { i32::MIN };

            neuron.receive(i % 2, signal).unwrap();

            for potential in neuron.potentials() {
                assert!((-10..=10).contains(&potential));
            }
        }

        neuron
    }

    assert_eq!(execute(), execute());
}

#[test]
fn saturated_inhibition_requires_recovery_before_activation() {
    let mut dendrite = Dendrite::new(2, 10).unwrap();

    assert!(!dendrite.receive(-100));
    assert_eq!(dendrite.potential(), -10);

    assert!(!dendrite.receive(10));
    assert_eq!(dendrite.potential(), 0);

    assert!(dendrite.receive(2));
    assert_eq!(dendrite.potential(), 0);
}
