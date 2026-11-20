#[path = "support/dendritic_eda_integrity_model.rs"]
mod model;

use model::{IntegrityAdapter, IntegrityError};
use neurons::engine::EngineError;
use neurons::event::NodeId;
use neurons::neuron::Neuron;
use neurons::synapse::{Synapse, SynapseId};

fn id(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn connection(number: u32, weight: i32, delay: u64) -> Synapse {
    Synapse::new(SynapseId::new(number).unwrap(), id(1), id(2), weight, delay).unwrap()
}

fn adapter(capacity: usize, cycle_limit: usize) -> IntegrityAdapter {
    IntegrityAdapter::new(
        Neuron::new(id(1), 1, 10).unwrap(),
        id(2),
        capacity,
        cycle_limit,
    )
    .unwrap()
}

#[test]
fn source_is_processed_only_during_eda_cycle() {
    let mut system = adapter(8, 8);

    system.connect(connection(1, 2, 1), id(101), 0).unwrap();

    system.stimulate(1, 0).unwrap();

    assert_eq!(system.source_activations(), 0);
    assert_eq!(system.activations(), 0);
    assert_eq!(system.queue_len(), 1);

    system.run_cycle();

    assert_eq!(system.source_activations(), 1);
    assert_eq!(system.activations(), 1);
    assert_eq!(system.queue_len(), 0);
}

#[test]
fn same_source_can_address_distinct_dendrites() {
    let mut system = adapter(8, 8);

    system.connect(connection(1, 2, 1), id(101), 0).unwrap();
    system.connect(connection(2, 1, 1), id(102), 1).unwrap();

    system.stimulate(1, 0).unwrap();
    system.run_cycle();

    assert_eq!(system.activations(), 1);
    assert_eq!(system.potentials(), [0, 1]);
    assert_eq!(system.delivered(), &[(id(101), 2, 1), (id(102), 1, 1)]);
}

#[test]
fn insufficient_capacity_rejects_whole_group() {
    let mut system = adapter(1, 8);

    system.connect(connection(1, 2, 1), id(101), 0).unwrap();
    system.connect(connection(2, 2, 1), id(102), 1).unwrap();

    system.stimulate(1, 0).unwrap();

    let report = system.run_cycle();

    assert_eq!(report.rejected_groups, 1);
    assert_eq!(
        report.first_error,
        Some(IntegrityError::Engine(EngineError::QueueFull))
    );

    assert_eq!(system.source_activations(), 1);
    assert_eq!(system.activations(), 0);
    assert_eq!(system.queue_len(), 0);
    assert!(system.delivered().is_empty());
}

#[test]
fn cycle_limit_keeps_generated_events_pending() {
    let mut system = adapter(4, 1);

    system.connect(connection(1, 2, 1), id(101), 0).unwrap();

    system.stimulate(1, 0).unwrap();

    let first = system.run_cycle();

    assert_eq!(first.cycle.processed, 1);
    assert_eq!(first.cycle.remaining, 1);
    assert_eq!(system.source_activations(), 1);
    assert_eq!(system.activations(), 0);

    let second = system.run_cycle();

    assert_eq!(second.cycle.processed, 1);
    assert_eq!(second.cycle.remaining, 0);
    assert_eq!(system.activations(), 1);
}

#[test]
fn collisions_and_invalid_compartments_are_rejected() {
    let mut system = adapter(8, 8);

    system.connect(connection(1, 1, 1), id(101), 0).unwrap();

    assert_eq!(
        system.connect(connection(1, 1, 1), id(102), 1),
        Err(IntegrityError::DuplicateSynapse)
    );

    assert_eq!(
        system.connect(connection(2, 1, 1), id(101), 1),
        Err(IntegrityError::DuplicateAddress)
    );

    assert_eq!(
        system.connect(connection(3, 1, 1), id(103), 2),
        Err(IntegrityError::InvalidCompartment)
    );
}

#[test]
fn temporal_order_is_preserved() {
    let mut system = adapter(8, 8);

    system.connect(connection(1, 1, 5), id(101), 0).unwrap();
    system.connect(connection(2, 1, 1), id(102), 1).unwrap();

    system.stimulate(1, 0).unwrap();
    system.run_cycle();

    assert_eq!(system.delivered(), &[(id(102), 1, 1), (id(101), 1, 5)]);
}

#[test]
fn identical_runs_are_deterministic() {
    fn execute() -> (u64, [i32; 2], Vec<(NodeId, i32, u64)>) {
        let mut system = adapter(12, 12);

        system.connect(connection(1, 1, 2), id(101), 0).unwrap();
        system.connect(connection(2, 2, 1), id(102), 1).unwrap();

        for time in 0..3 {
            system.stimulate(1, time).unwrap();
        }

        while system.queue_len() > 0 {
            system.run_cycle();
        }

        (
            system.activations(),
            system.potentials(),
            system.delivered().to_vec(),
        )
    }

    assert_eq!(execute(), execute());
}
