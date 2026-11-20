#[path = "support/dendritic_core_compatibility_model.rs"]
mod reference;

#[path = "support/dendritic_eda_integrity_model.rs"]
mod dendritic_adapter;

use dendritic_adapter::IntegrityAdapter;
use neurons::neuron::Neuron;
use reference::{id, legacy_circuit, run_legacy, somatic_reference, synapse};

#[test]
fn legacy_circuit_matches_independent_somatic_reference() {
    let stimuli = [(1, 0), (1, 3), (1, 6)];

    let mut circuit = legacy_circuit(2, &[1]);
    let report = run_legacy(&mut circuit, &stimuli);

    let (expected_source, expected_target) = somatic_reference(2, &[1], &stimuli);

    assert_eq!(report.cycle.processed, 6);
    assert_eq!(report.activations, 4);
    assert_eq!(report.transmissions, 3);
    assert_eq!(report.rejected_transmissions, 0);
    assert_eq!(report.first_error, None);

    assert_eq!(circuit.neuron_state(id(1)), Some(expected_source));

    assert_eq!(circuit.neuron_state(id(2)), Some(expected_target));
}

#[test]
fn inhibitory_legacy_circuit_matches_reference() {
    let stimuli = [(1, 0), (1, 3)];

    let mut circuit = legacy_circuit(2, &[-2]);
    let report = run_legacy(&mut circuit, &stimuli);

    let (expected_source, expected_target) = somatic_reference(2, &[-2], &stimuli);

    assert_eq!(report.cycle.processed, 4);
    assert_eq!(report.rejected_transmissions, 0);

    assert_eq!(circuit.neuron_state(id(1)), Some(expected_source));

    assert_eq!(circuit.neuron_state(id(2)), Some(expected_target));
}

#[test]
fn dendritic_adapter_does_not_mutate_legacy_circuit() {
    let mut legacy = legacy_circuit(2, &[1]);

    let before_source = legacy.neuron_state(id(1));
    let before_target = legacy.neuron_state(id(2));
    let before_queue = legacy.queue_len();

    let mut dendritic =
        IntegrityAdapter::new(Neuron::new(id(1), 1, 10).unwrap(), id(2), 8, 8).unwrap();

    dendritic.connect(synapse(1, 2, 1), id(101), 0).unwrap();

    dendritic.stimulate(1, 0).unwrap();
    dendritic.run_cycle();

    assert_eq!(dendritic.activations(), 1);
    assert_eq!(legacy.neuron_state(id(1)), before_source);
    assert_eq!(legacy.neuron_state(id(2)), before_target);
    assert_eq!(legacy.queue_len(), before_queue);
}

#[test]
fn dendritic_and_somatic_integration_can_differ() {
    // Duas sinapses entregam uma unidade cada.
    // Soma tradicional: limiar 2, ativa.
    // Dendritos separados: limiar local 2, nao ativam.

    let mut legacy = legacy_circuit(2, &[1, 1]);
    let legacy_report = run_legacy(&mut legacy, &[(1, 0)]);

    assert_eq!(legacy_report.activations, 2);
    assert_eq!(legacy.neuron_state(id(2)).unwrap().activations, 1);

    let mut dendritic =
        IntegrityAdapter::new(Neuron::new(id(1), 1, 10).unwrap(), id(2), 8, 8).unwrap();

    dendritic.connect(synapse(1, 1, 1), id(101), 0).unwrap();

    dendritic.connect(synapse(2, 1, 1), id(102), 1).unwrap();

    dendritic.stimulate(1, 0).unwrap();
    let report = dendritic.run_cycle();

    assert_eq!(report.cycle.processed, 3);
    assert_eq!(report.rejected_groups, 0);
    assert_eq!(dendritic.activations(), 0);
    assert_eq!(dendritic.potentials(), [1, 1]);
}

#[test]
fn legacy_snapshot_remains_observable() {
    let mut circuit = legacy_circuit(2, &[1]);

    run_legacy(&mut circuit, &[(1, 0)]);

    let snapshot = circuit.snapshot().unwrap();

    assert_eq!(snapshot.neurons.len(), 2);
    assert_eq!(snapshot.synapses.len(), 1);
    assert!(!snapshot.engine.is_empty());
}

#[test]
fn legacy_circuit_remains_deterministic() {
    fn execute() -> (neurons::neuron::NeuronState, neurons::neuron::NeuronState) {
        let mut circuit = legacy_circuit(2, &[1, 1]);

        run_legacy(&mut circuit, &[(1, 0), (1, 3), (1, 6)]);

        (
            circuit.neuron_state(id(1)).unwrap(),
            circuit.neuron_state(id(2)).unwrap(),
        )
    }

    assert_eq!(execute(), execute());
}
