#[path = "support/dendritic_operational_model.rs"]
mod model;

use model::{AdmissionError, OperationalAdapter};

use neurons::engine::{Engine, EngineError};
use neurons::event::{Event, EventKind, NodeId};

fn id(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn signal(destination: u32, payload: i32, time: u64) -> Event {
    Event::new(
        Some(id(1)),
        id(destination),
        EventKind::Signal,
        payload,
        time,
    )
}

fn adapter(capacity: usize, limit: usize) -> OperationalAdapter {
    let mut adapter = OperationalAdapter::new(capacity, limit).unwrap();

    adapter.register(id(101));
    adapter.register(id(102));

    adapter
}

#[test]
fn valid_group_is_admitted_completely() {
    let mut system = adapter(4, 4);

    system
        .admit(&[signal(101, 2, 1), signal(102, 3, 1)])
        .unwrap();

    assert_eq!(system.queue_len(), 2);

    assert_eq!(system.run_cycle(), vec![(id(101), 2, 1), (id(102), 3, 1),]);
}

#[test]
fn insufficient_capacity_rejects_entire_group() {
    let mut system = adapter(1, 4);

    let result = system.admit(&[signal(101, 2, 1), signal(102, 3, 1)]);

    assert_eq!(result, Err(AdmissionError::InsufficientCapacity));

    assert_eq!(system.queue_len(), 0);
    assert_eq!(system.metrics().enqueued, 0);
}

#[test]
fn invalid_destination_rejects_entire_group() {
    let mut system = adapter(4, 4);

    let result = system.admit(&[signal(101, 2, 1), signal(999, 3, 1)]);

    assert_eq!(result, Err(AdmissionError::UnknownDestination(id(999))));

    assert_eq!(system.queue_len(), 0);
}

#[test]
fn past_time_rejects_entire_group() {
    let mut system = adapter(4, 4);

    system.admit(&[signal(101, 1, 10)]).unwrap();

    system.run_cycle();

    assert_eq!(system.logical_time(), Some(10));

    let result = system.admit(&[signal(101, 2, 11), signal(102, 3, 9)]);

    assert_eq!(result, Err(AdmissionError::PastLogicalTime));

    assert_eq!(system.queue_len(), 0);
}

#[test]
fn temporal_order_is_preserved() {
    let mut system = adapter(4, 4);

    system
        .admit(&[signal(101, 1, 5), signal(102, 2, 1), signal(101, 3, 3)])
        .unwrap();

    assert_eq!(
        system.run_cycle(),
        vec![(id(102), 2, 1), (id(101), 3, 3), (id(101), 1, 5),]
    );
}

#[test]
fn pending_events_survive_cycle_boundary() {
    let mut system = adapter(4, 1);

    system
        .admit(&[signal(101, 1, 1), signal(102, 2, 2)])
        .unwrap();

    assert_eq!(system.run_cycle(), vec![(id(101), 1, 1)]);

    assert_eq!(system.queue_len(), 1);

    assert_eq!(system.run_cycle(), vec![(id(102), 2, 2)]);

    assert_eq!(system.queue_len(), 0);
}

#[test]
fn engine_individual_enqueue_can_be_partial() {
    let mut engine = Engine::new(1, 4).unwrap();

    engine.register_destination(id(101));
    engine.register_destination(id(102));

    assert_eq!(engine.enqueue(signal(101, 1, 1)), Ok(()));

    assert_eq!(
        engine.enqueue(signal(102, 2, 1)),
        Err(EngineError::QueueFull)
    );

    // A primeira insercao permanece na fila.
    assert_eq!(engine.queue_len(), 1);
    assert_eq!(engine.metrics().enqueued, 1);
    assert_eq!(engine.metrics().rejected, 1);
}

#[test]
fn rejected_group_preserves_existing_events() {
    let mut system = adapter(2, 4);

    system.admit(&[signal(101, 7, 1)]).unwrap();

    assert_eq!(
        system.admit(&[signal(101, 8, 2), signal(102, 9, 2),]),
        Err(AdmissionError::InsufficientCapacity)
    );

    assert_eq!(system.queue_len(), 1);

    assert_eq!(system.run_cycle(), vec![(id(101), 7, 1)]);
}

#[test]
fn identical_runs_are_deterministic() {
    fn execute() -> Vec<(NodeId, i32, u64)> {
        let mut system = adapter(8, 8);

        system
            .admit(&[signal(102, 2, 3), signal(101, 1, 1), signal(102, 3, 3)])
            .unwrap();

        system.run_cycle()
    }

    assert_eq!(execute(), execute());
}
