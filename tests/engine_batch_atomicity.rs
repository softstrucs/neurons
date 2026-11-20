use neurons::engine::{Engine, EngineError};
use neurons::event::{Event, EventKind, NodeId};

fn id(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn event(payload: i32, time: u64) -> Event {
    Event::new(None, id(1), EventKind::Stimulus, payload, time)
}

#[test]
fn rejected_batch_does_not_consume_sequence() {
    let mut engine = Engine::new(3, 10).unwrap();
    engine.register_destination(id(1));

    engine.enqueue(event(1, 1)).unwrap();

    assert_eq!(
        engine.enqueue_batch(&[event(2, 2), event(3, 2), event(4, 2),]),
        Err(EngineError::QueueFull)
    );

    engine.enqueue(event(5, 2)).unwrap();

    let mut received = Vec::new();
    engine.run_cycle(|event, _| received.push(event.payload()));

    assert_eq!(received, [1, 5]);
    assert_eq!(engine.metrics().enqueued, 2);
    assert_eq!(engine.metrics().rejected, 1);
}

#[test]
fn event_sink_batch_preserves_existing_order() {
    let mut engine = Engine::new(4, 10).unwrap();
    engine.register_destination(id(1));

    engine.enqueue(event(1, 0)).unwrap();
    engine.enqueue(event(2, 0)).unwrap();

    let mut received = Vec::new();

    engine.run_cycle(|current, sink| {
        received.push(current.payload());

        if current.payload() == 1 {
            sink.enqueue_batch(&[event(3, 0), event(4, 0)]).unwrap();
        }
    });

    assert_eq!(received, [1, 2, 3, 4]);
    assert_eq!(engine.metrics().enqueued, 4);
    assert_eq!(engine.metrics().dispatched, 4);
    assert_eq!(engine.queue_len(), 0);
}

#[test]
fn event_sink_rejects_entire_oversized_batch() {
    let mut engine = Engine::new(2, 10).unwrap();
    engine.register_destination(id(1));

    engine.enqueue(event(1, 0)).unwrap();
    engine.enqueue(event(2, 0)).unwrap();

    let mut received = Vec::new();

    engine.run_cycle(|current, sink| {
        received.push(current.payload());

        if current.payload() == 1 {
            assert_eq!(
                sink.enqueue_batch(&[event(3, 0), event(4, 0),]),
                Err(EngineError::QueueFull)
            );
        }
    });

    assert_eq!(received, [1, 2]);
    assert_eq!(engine.metrics().enqueued, 2);
    assert_eq!(engine.metrics().dispatched, 2);
    assert_eq!(engine.metrics().rejected, 1);
}

#[test]
fn mixed_individual_and_batch_insertion_is_deterministic() {
    fn execute() -> (Vec<i32>, neurons::engine::EngineMetrics) {
        let mut engine = Engine::new(6, 10).unwrap();
        engine.register_destination(id(1));

        engine.enqueue(event(1, 3)).unwrap();
        engine.enqueue_batch(&[event(2, 1), event(3, 3)]).unwrap();
        engine.enqueue(event(4, 1)).unwrap();

        let mut received = Vec::new();
        engine.run_cycle(|event, _| received.push(event.payload()));

        (received, engine.metrics())
    }

    let first = execute();
    assert_eq!(first, execute());
    assert_eq!(first.0, [2, 4, 1, 3]);
}
