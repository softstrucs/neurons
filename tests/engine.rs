use neurons::engine::{Engine, EngineError};
use neurons::event::{Event, EventKind, NodeId, NodeIdError};

fn id(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn event(destination: NodeId, payload: i32, logical_time: u64) -> Event {
    Event::new(
        None,
        destination,
        EventKind::Stimulus,
        payload,
        logical_time,
    )
}

#[test]
fn orders_by_time_then_insertion_sequence() {
    let destination = id(1);
    let mut engine = Engine::new(8, 8).unwrap();
    engine.register_destination(destination);
    for (payload, time) in [(10, 2), (20, 1), (30, 2), (40, 1)] {
        engine.enqueue(event(destination, payload, time)).unwrap();
    }

    let mut received = Vec::new();
    let report = engine.run_cycle(|event, _| received.push(event.payload()));
    assert_eq!(received, [20, 40, 10, 30]);
    assert_eq!(report.processed, 4);
    assert_eq!(report.remaining, 0);
    assert!(!report.limit_reached);
    assert_eq!(engine.metrics().peak_queue_len, 4);
}

#[test]
fn dispatches_to_registered_destinations_with_typed_data() {
    let first = id(1);
    let second = id(2);
    let mut engine = Engine::new(4, 4).unwrap();
    assert!(engine.register_destination(first));
    assert!(engine.register_destination(second));
    assert!(!engine.register_destination(second));
    engine
        .enqueue(Event::new(Some(first), second, EventKind::Signal, -7, 3))
        .unwrap();
    engine
        .enqueue(Event::new(None, first, EventKind::Control, 9, 4))
        .unwrap();

    let mut observed = Vec::new();
    engine.run_cycle(|event, _| {
        observed.push((
            event.origin(),
            event.destination(),
            event.kind(),
            event.payload(),
            event.logical_time(),
        ));
    });
    assert_eq!(
        observed,
        [
            (Some(first), second, EventKind::Signal, -7, 3),
            (None, first, EventKind::Control, 9, 4),
        ]
    );
    assert_eq!(engine.metrics().dispatched, 2);
    assert!(std::mem::size_of::<Event>() <= 32);
}

#[test]
fn newly_emitted_event_follows_earlier_events_at_the_same_time() {
    let destination = id(1);
    let mut engine = Engine::new(3, 3).unwrap();
    engine.register_destination(destination);
    engine.enqueue(event(destination, 1, 0)).unwrap();
    engine.enqueue(event(destination, 2, 0)).unwrap();

    let mut observed = Vec::new();
    engine.run_cycle(|received, engine| {
        observed.push(received.payload());
        if received.payload() == 1 {
            engine.enqueue(event(destination, 3, 0)).unwrap();
        }
    });
    assert_eq!(observed, [1, 2, 3]);
}

#[test]
fn rejects_new_event_when_queue_is_full_and_recovers_after_dispatch() {
    let destination = id(1);
    let mut engine = Engine::new(1, 1).unwrap();
    engine.register_destination(destination);
    engine.enqueue(event(destination, 1, 0)).unwrap();
    assert_eq!(
        engine.enqueue(event(destination, 2, 0)),
        Err(EngineError::QueueFull)
    );
    assert_eq!(engine.queue_len(), 1);
    engine.run_cycle(|_, _| {});
    engine.enqueue(event(destination, 3, 0)).unwrap();
    assert_eq!(engine.metrics().enqueued, 2);
    assert_eq!(engine.metrics().rejected, 1);
}

#[test]
fn rejects_invalid_configuration_address_and_past_time() {
    assert_eq!(NodeId::new(0), Err(NodeIdError));
    assert!(matches!(
        Engine::new(0, 1),
        Err(EngineError::ZeroQueueCapacity)
    ));
    assert!(matches!(
        Engine::new(1, 0),
        Err(EngineError::ZeroCycleLimit)
    ));

    let destination = id(1);
    let unknown = id(2);
    let mut engine = Engine::new(2, 2).unwrap();
    assert_eq!(
        engine.enqueue(event(unknown, 1, 0)),
        Err(EngineError::UnknownDestination(unknown))
    );
    engine.register_destination(destination);
    engine.enqueue(event(destination, 2, 5)).unwrap();
    engine.run_cycle(|_, _| {});
    assert_eq!(
        engine.enqueue(event(destination, 3, 4)),
        Err(EngineError::PastLogicalTime {
            attempted: 4,
            current: 5,
        })
    );
    assert_eq!(engine.metrics().rejected, 2);
    assert_eq!(engine.queue_len(), 0);
}

#[test]
fn bounded_cycles_stop_self_propagation() {
    let destination = id(1);
    let mut engine = Engine::new(1, 3).unwrap();
    engine.register_destination(destination);
    engine.enqueue(event(destination, 1, 0)).unwrap();

    let mut seen = Vec::new();
    let first = engine.run_cycle(|event, engine| {
        seen.push(event.payload());
        engine
            .enqueue(event_with_next_payload(event))
            .expect("a fila tem espaço após o despacho");
    });
    assert_eq!(seen, [1, 2, 3]);
    assert_eq!(first.processed, 3);
    assert_eq!(first.remaining, 1);
    assert!(first.limit_reached);

    let second = engine.run_cycle(|event, _| seen.push(event.payload()));
    assert_eq!(seen, [1, 2, 3, 4]);
    assert_eq!(second.remaining, 0);
    assert!(!second.limit_reached);
}

fn event_with_next_payload(previous: Event) -> Event {
    Event::new(
        Some(previous.destination()),
        previous.destination(),
        EventKind::Signal,
        previous.payload() + 1,
        previous.logical_time(),
    )
}

#[test]
fn identical_inputs_produce_identical_outputs_and_metrics() {
    fn run() -> (Vec<i32>, neurons::engine::EngineMetrics) {
        let destination = id(1);
        let mut engine = Engine::new(6, 2).unwrap();
        engine.register_destination(destination);
        for (payload, time) in [(1, 3), (2, 1), (3, 1), (4, 2)] {
            engine.enqueue(event(destination, payload, time)).unwrap();
        }
        let mut outputs = Vec::new();
        while engine.queue_len() > 0 {
            engine.run_cycle(|event, _| outputs.push(event.payload()));
        }
        (outputs, engine.metrics())
    }

    let first = run();
    assert_eq!(first, run());
    assert_eq!(first.0, [2, 3, 4, 1]);
    assert_eq!(first.1.dispatched, 4);
}
