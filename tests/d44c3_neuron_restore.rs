use neurons::event::{Event, EventKind, NodeId};
use neurons::neuron::{Neuron, NeuronRestoreError, NeuronSnapshot, NeuronState};

fn id() -> NodeId {
    NodeId::new(1).unwrap()
}

fn snapshot(potential: i32, received: u64, activations: u64) -> NeuronSnapshot {
    NeuronSnapshot {
        id: id(),
        threshold: 3,
        state_limit: 10,
        state: NeuronState {
            potential,
            received,
            activations,
        },
    }
}

#[test]
fn valid_snapshot_round_trip() {
    let original = snapshot(2, 5, 1);
    let restored = Neuron::from_snapshot(original).unwrap();

    assert_eq!(restored.snapshot(), original);
}

#[test]
fn restored_neuron_continues_processing() {
    let mut neuron = Neuron::from_snapshot(snapshot(2, 5, 1)).unwrap();

    let event = Event::new(None, id(), EventKind::Stimulus, 1, 10);

    assert!(neuron.receive(event).unwrap().is_some());

    let state = neuron.state();
    assert_eq!(state.potential, 0);
    assert_eq!(state.received, 6);
    assert_eq!(state.activations, 2);
}

#[test]
fn rejects_invalid_potential() {
    for potential in [-11, 3, 11] {
        assert_eq!(
            Neuron::from_snapshot(snapshot(potential, 1, 0)).err(),
            Some(NeuronRestoreError::InvalidState)
        );
    }
}

#[test]
fn rejects_inconsistent_counters() {
    assert_eq!(
        Neuron::from_snapshot(snapshot(0, 1, 2)).err(),
        Some(NeuronRestoreError::InvalidState)
    );
}

#[test]
fn rejects_invalid_configuration() {
    let mut invalid = snapshot(0, 0, 0);
    invalid.threshold = 0;

    assert!(matches!(
        Neuron::from_snapshot(invalid),
        Err(NeuronRestoreError::Configuration(_))
    ));
}
