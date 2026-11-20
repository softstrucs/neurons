use neurons::engine::{Engine, EngineError};
use neurons::event::{Event, EventKind, NodeId};

fn id(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn event(destination: u32, payload: i32, time: u64) -> Event {
    Event::new(None, id(destination), EventKind::Stimulus, payload, time)
}

fn engine(capacity: usize) -> Engine {
    let mut engine = Engine::new(capacity, 16).unwrap();
    engine.register_destination(id(1));
    engine.register_destination(id(2));
    engine
}

#[test]
fn empty_batch_does_not_change_state() {
    let mut engine = engine(4);
    let before = engine.metrics();

    assert_eq!(engine.enqueue_batch(&[]), Ok(()));
    assert_eq!(engine.queue_len(), 0);
    assert_eq!(engine.metrics(), before);
}

#[test]
fn valid_batch_preserves_temporal_and_insertion_order() {
    let mut engine = engine(4);

    engine
        .enqueue_batch(&[event(1, 10, 3), event(2, 20, 1), event(1, 30, 3)])
        .unwrap();

    let mut received = Vec::new();
    engine.run_cycle(|event, _| received.push(event.payload()));

    assert_eq!(received, [20, 10, 30]);
    assert_eq!(engine.metrics().enqueued, 3);
    assert_eq!(engine.metrics().dispatched, 3);
}

#[test]
fn unknown_destination_rejects_entire_batch() {
    let mut engine = engine(4);

    assert_eq!(
        engine.enqueue_batch(&[event(1, 10, 1), event(99, 20, 1),]),
        Err(EngineError::UnknownDestination(id(99)))
    );

    assert_eq!(engine.queue_len(), 0);
    assert_eq!(engine.metrics().enqueued, 0);
    assert_eq!(engine.metrics().rejected, 1);
}

#[test]
fn insufficient_capacity_rejects_entire_batch() {
    let mut engine = engine(2);

    assert_eq!(
        engine.enqueue_batch(&[event(1, 10, 1), event(2, 20, 1), event(1, 30, 1),]),
        Err(EngineError::QueueFull)
    );

    assert_eq!(engine.queue_len(), 0);
    assert_eq!(engine.metrics().enqueued, 0);
    assert_eq!(engine.metrics().rejected, 1);
}

#[test]
fn past_time_rejects_entire_batch() {
    let mut engine = engine(4);

    engine.enqueue(event(1, 1, 10)).unwrap();
    engine.run_cycle(|_, _| {});

    let before = engine.metrics();

    assert_eq!(
        engine.enqueue_batch(&[event(1, 2, 11), event(2, 3, 9),]),
        Err(EngineError::PastLogicalTime {
            attempted: 9,
            current: 10,
        })
    );

    assert_eq!(engine.queue_len(), 0);
    assert_eq!(engine.metrics().enqueued, before.enqueued);
    assert_eq!(engine.metrics().rejected, before.rejected + 1);
}

#[test]
fn rejected_batch_preserves_existing_events() {
    let mut engine = engine(2);

    engine.enqueue(event(1, 7, 1)).unwrap();

    assert_eq!(
        engine.enqueue_batch(&[event(1, 8, 2), event(2, 9, 2),]),
        Err(EngineError::QueueFull)
    );

    let mut received = Vec::new();
    engine.run_cycle(|event, _| received.push(event.payload()));

    assert_eq!(received, [7]);
    assert_eq!(engine.metrics().enqueued, 1);
    assert_eq!(engine.metrics().rejected, 1);
}
