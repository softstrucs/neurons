#[path = "support/dendritic_eda_model.rs"]
mod dendritic_eda_model;

use dendritic_eda_model::{AdapterError, DendriticEda};
use neurons::engine::EngineError;
use neurons::event::NodeId;
use neurons::neuron::Neuron;
use neurons::synapse::{Synapse, SynapseId};

fn id(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn synapse(number: u32, weight: i32, delay: u64) -> Synapse {
    Synapse::new(SynapseId::new(number).unwrap(), id(1), id(2), weight, delay).unwrap()
}

fn adapter(capacity: usize, limit: usize) -> DendriticEda {
    DendriticEda::new(Neuron::new(id(1), 1, 10).unwrap(), id(2), capacity, limit).unwrap()
}

#[test]
fn distinct_synapses_reach_distinct_dendrites() {
    let mut system = adapter(8, 8);

    system.connect(synapse(1, 2, 1), id(101), 0).unwrap();
    system.connect(synapse(2, 1, 1), id(102), 1).unwrap();

    system.stimulate(1, 0).unwrap();
    system.run_cycle();

    assert_eq!(system.activations(), 1);
    assert_eq!(system.potentials(), [0, 1]);
    assert_eq!(system.delivered(), &[(id(101), 2, 1), (id(102), 1, 1)]);
}

#[test]
fn inhibitory_weight_is_preserved() {
    let mut system = adapter(8, 8);

    system.connect(synapse(1, -3, 1), id(101), 0).unwrap();

    system.stimulate(1, 0).unwrap();
    system.run_cycle();

    assert_eq!(system.activations(), 0);
    assert_eq!(system.potentials(), [-3, 0]);
}

#[test]
fn logical_delay_controls_delivery_order() {
    let mut system = adapter(8, 8);

    system.connect(synapse(1, 1, 5), id(101), 0).unwrap();
    system.connect(synapse(2, 1, 1), id(102), 1).unwrap();

    system.stimulate(1, 0).unwrap();
    system.run_cycle();

    assert_eq!(system.delivered(), &[(id(102), 1, 1), (id(101), 1, 5)]);
}

#[test]
fn invalid_routes_are_rejected() {
    let mut system = adapter(8, 8);

    system.connect(synapse(1, 1, 1), id(101), 0).unwrap();

    assert!(matches!(
        system.connect(synapse(1, 1, 1), id(102), 1),
        Err(AdapterError::DuplicateSynapse)
    ));

    assert!(matches!(
        system.connect(synapse(2, 1, 1), id(101), 1),
        Err(AdapterError::DuplicateAddress)
    ));

    assert!(matches!(
        system.connect(synapse(3, 1, 1), id(103), 2),
        Err(AdapterError::InvalidCompartment)
    ));

    assert_eq!(system.queue_len(), 0);
}

#[test]
fn cycle_limit_preserves_pending_events() {
    let mut system = adapter(8, 1);

    system.connect(synapse(1, 1, 1), id(101), 0).unwrap();
    system.connect(synapse(2, 1, 2), id(102), 1).unwrap();

    system.stimulate(1, 0).unwrap();

    let first = system.run_cycle();
    assert_eq!(first.processed, 1);
    assert_eq!(first.remaining, 1);
    assert!(first.limit_reached);

    let second = system.run_cycle();
    assert_eq!(second.processed, 1);
    assert_eq!(second.remaining, 0);
}

#[test]
fn queue_capacity_rejects_excess_transmission() {
    let mut system = adapter(1, 1);

    system.connect(synapse(1, 1, 1), id(101), 0).unwrap();
    system.connect(synapse(2, 1, 1), id(102), 1).unwrap();

    assert_eq!(
        system.stimulate(1, 0),
        Err(AdapterError::Engine(EngineError::QueueFull))
    );

    // A primeira transmissão permanece na fila.
    assert_eq!(system.queue_len(), 1);
}

#[test]
fn identical_experiments_are_deterministic() {
    fn execute() -> (u64, [i32; 2], Vec<(NodeId, i32, u64)>) {
        let mut system = adapter(8, 8);

        system.connect(synapse(1, 1, 2), id(101), 0).unwrap();
        system.connect(synapse(2, 2, 1), id(102), 1).unwrap();

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
