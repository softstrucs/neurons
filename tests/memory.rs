use neurons::circuit::Circuit;
use neurons::engram::{Cue, Engram, EngramError, EngramId, EngramStore, EvidenceId, RecordStatus};
use neurons::event::NodeId;
use neurons::neuron::Neuron;
use neurons::synapse::{Synapse, SynapseId};

fn node(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn id(value: u32) -> EngramId {
    EngramId::new(value).unwrap()
}

fn cue(context: u32, source: u32, pattern: i32) -> Cue {
    Cue::new(context, node(source), pattern).unwrap()
}

fn record(id_value: u32, cue: Cue, value: i32, status: RecordStatus) -> Engram {
    Engram::new(
        id(id_value),
        cue,
        value,
        Some(cue.source()),
        7,
        None,
        status,
    )
    .unwrap()
}

#[test]
fn records_and_recovers_exact_contextual_association() {
    assert!(std::mem::size_of::<Engram>() <= 64);
    let mut store = EngramStore::new(3).unwrap();
    let target = cue(1, 1, 9);
    let entry = Engram::new(
        id(1),
        target,
        42,
        Some(node(1)),
        7,
        Some(EvidenceId::new(5).unwrap()),
        RecordStatus::Observed,
    )
    .unwrap();
    store.record(entry).unwrap();
    store
        .record(record(2, cue(2, 1, 9), 99, RecordStatus::Hypothesis))
        .unwrap();
    store
        .record(record(3, cue(1, 2, 9), -1, RecordStatus::Hypothesis))
        .unwrap();

    assert_eq!(store.recall(target), Ok(entry));
    assert_eq!(store.get(id(1)), Ok(entry));
    assert_eq!(entry.value(), 42);
    assert_eq!(entry.origin(), Some(node(1)));
    assert_eq!(entry.observed_at(), 7);
    assert_eq!(entry.evidence().unwrap().get(), 5);
    assert_eq!(entry.status(), RecordStatus::Observed);
    assert_eq!(store.recall(cue(1, 1, 8)), Err(EngramError::NoAssociation));
}

#[test]
fn missing_record_and_invalid_inputs_return_errors() {
    assert_eq!(EngramId::new(0), Err(EngramError::InvalidId));
    assert_eq!(EvidenceId::new(0), Err(EngramError::InvalidEvidence));
    assert_eq!(Cue::new(0, node(1), 1), Err(EngramError::InvalidContext));
    assert_eq!(Cue::new(1, node(1), 0), Err(EngramError::InvalidPattern));
    assert!(matches!(
        EngramStore::new(0),
        Err(EngramError::ZeroCapacity)
    ));
    assert_eq!(
        Engram::new(
            id(1),
            cue(1, 1, 1),
            7,
            None,
            0,
            None,
            RecordStatus::Validated
        ),
        Err(EngramError::ValidationWithoutEvidence)
    );

    let store = EngramStore::new(1).unwrap();
    assert_eq!(store.get(id(1)), Err(EngramError::NotFound(id(1))));
    assert_eq!(store.recall(cue(1, 1, 1)), Err(EngramError::NoAssociation));
    assert!(store.is_empty());
}

#[test]
fn capacity_and_duplicate_id_do_not_overwrite_records() {
    let mut store = EngramStore::new(1).unwrap();
    let first = record(1, cue(1, 1, 1), 10, RecordStatus::Observed);
    store.record(first).unwrap();
    assert_eq!(
        store.record(record(1, cue(2, 1, 1), 20, RecordStatus::Hypothesis)),
        Err(EngramError::DuplicateId(id(1)))
    );
    assert_eq!(
        store.record(record(2, cue(2, 1, 1), 20, RecordStatus::Hypothesis)),
        Err(EngramError::CapacityFull)
    );
    assert_eq!(store.len(), 1);
    assert_eq!(store.capacity(), 1);
    assert_eq!(store.get(id(1)), Ok(first));
}

#[test]
fn conflicting_or_repeated_cue_is_ambiguous_not_validated() {
    let mut store = EngramStore::new(3).unwrap();
    let key = cue(1, 1, 1);
    store
        .record(record(1, key, 10, RecordStatus::Hypothesis))
        .unwrap();
    store
        .record(record(2, key, 20, RecordStatus::Hypothesis))
        .unwrap();
    assert_eq!(store.recall(key), Err(EngramError::AmbiguousAssociation));
    assert_eq!(store.get(id(1)).unwrap().status(), RecordStatus::Hypothesis);

    let mut repeated = EngramStore::new(2).unwrap();
    repeated
        .record(record(1, key, 10, RecordStatus::Observed))
        .unwrap();
    repeated
        .record(record(2, key, 10, RecordStatus::Observed))
        .unwrap();
    assert_eq!(repeated.recall(key), Err(EngramError::AmbiguousAssociation));
    assert_eq!(
        repeated.get(id(1)).unwrap().status(),
        RecordStatus::Observed
    );
}

#[test]
fn activation_cue_requires_new_activity_and_ignores_absolute_count() {
    assert_eq!(
        Cue::from_activation_counts(1, node(1), 2, 2),
        Err(EngramError::NoActivation)
    );
    assert_eq!(
        Cue::from_activation_counts(1, node(1), 3, 2),
        Err(EngramError::ActivationCounterRegression)
    );
    assert_eq!(
        Cue::from_activation_counts(1, node(1), 0, 1),
        Cue::from_activation_counts(1, node(1), 5, 6)
    );
}

fn two_neuron_circuit() -> Circuit {
    let mut circuit = Circuit::new(2, 2).unwrap();
    circuit
        .add_neuron(Neuron::new(node(1), 1, 4).unwrap())
        .unwrap();
    circuit
        .add_neuron(Neuron::new(node(2), 1, 4).unwrap())
        .unwrap();
    circuit
        .connect(Synapse::new(SynapseId::new(1).unwrap(), node(1), node(2), 1, 1).unwrap())
        .unwrap();
    circuit
}

fn activate_and_cue(circuit: &mut Circuit, logical_time: u64) -> Cue {
    let before = circuit.neuron_state(node(2)).unwrap().activations;
    circuit.stimulate(node(1), 1, logical_time).unwrap();
    let report = circuit.run_cycle();
    assert_eq!(report.cycle.processed, 2);
    assert_eq!(report.first_error, None);
    let after = circuit.neuron_state(node(2)).unwrap().activations;
    Cue::from_activation_counts(7, node(2), before, after).unwrap()
}

#[test]
fn circuit_activity_can_be_registered_then_recalled_later() {
    let mut circuit = two_neuron_circuit();
    let mut store = EngramStore::new(2).unwrap();
    let training_cue = activate_and_cue(&mut circuit, 0);
    store
        .record(record(1, training_cue, 73, RecordStatus::Observed))
        .unwrap();

    let later_cue = activate_and_cue(&mut circuit, 2);
    assert_eq!(later_cue, training_cue);
    assert_eq!(store.recall(later_cue).unwrap().value(), 73);
    assert_eq!(circuit.neuron_state(node(2)).unwrap().potential, 0);
    assert_eq!(circuit.neuron_state(node(2)).unwrap().activations, 2);
    assert_eq!(store.len(), 1);
}

#[test]
fn identical_runs_recover_identical_records_and_absence_without_memory() {
    fn run() -> (Engram, u64) {
        let mut circuit = two_neuron_circuit();
        let mut store = EngramStore::new(1).unwrap();
        let key = activate_and_cue(&mut circuit, 0);
        assert_eq!(store.recall(key), Err(EngramError::NoAssociation));
        store
            .record(record(1, key, 73, RecordStatus::Hypothesis))
            .unwrap();
        let later = activate_and_cue(&mut circuit, 2);
        (
            store.recall(later).unwrap(),
            circuit.engine_metrics().dispatched,
        )
    }

    let first = run();
    assert_eq!(first, run());
    assert_eq!(first.1, 4);
    assert_eq!(first.0.status(), RecordStatus::Hypothesis);
}
