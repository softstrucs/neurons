use neurons::engine::Engine;
use neurons::engram::EngramStore;
use neurons::event::{Event, EventKind, NodeId};
use neurons::persistence;

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

fn id(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn event(destination: u32, payload: i32, time: u64) -> Event {
    Event::new(None, id(destination), EventKind::Stimulus, payload, time)
}

fn fixture() -> (Engine, EngramStore) {
    let mut engine = Engine::new(8, 2).unwrap();

    engine.register_destination(id(1));
    engine.register_destination(id(2));

    engine.enqueue(event(1, 10, 5)).unwrap();

    engine
        .enqueue_batch(&[event(2, 20, 3), event(1, 30, 5), event(2, 40, 7)])
        .unwrap();

    let memory = EngramStore::new(4).unwrap();

    (engine, memory)
}

fn drain(engine: &mut Engine) -> Vec<(u32, i32, u64)> {
    let mut received = Vec::new();

    while engine.queue_len() > 0 {
        engine.run_cycle(|event, _| {
            received.push((
                event.destination().get(),
                event.payload(),
                event.logical_time(),
            ));
        });
    }

    received
}

#[test]
fn batch_checkpoint_round_trip_preserves_order_and_metrics() {
    let (mut original, memory) = fixture();

    let before_metrics = original.metrics();

    let encoded = persistence::encode(&original, &memory).unwrap();

    let (mut restored, recovered) = persistence::decode(&encoded).unwrap();

    assert_eq!(encoded, persistence::encode(&restored, &recovered).unwrap());

    assert_eq!(restored.queue_len(), 4);
    assert_eq!(restored.metrics(), before_metrics);

    let expected = vec![(2, 20, 3), (1, 10, 5), (1, 30, 5), (2, 40, 7)];

    assert_eq!(drain(&mut original), expected);
    assert_eq!(drain(&mut restored), expected);

    assert_eq!(original.metrics(), restored.metrics());
}

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);

        let path = std::env::temp_dir().join(format!(
            "neurons-den-int-007-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));

        std::fs::create_dir(&path).unwrap();

        Self(path)
    }

    fn checkpoint(&self) -> PathBuf {
        self.0.join("checkpoint.bin")
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn disk_checkpoint_restores_batch_and_continues_execution() {
    let (mut original, memory) = fixture();
    let directory = TemporaryDirectory::new();
    let path = directory.checkpoint();

    persistence::save(&path, &original, &memory).unwrap();

    let (mut restored, recovered) = persistence::load(&path).unwrap();

    assert_eq!(
        persistence::encode(&original, &memory).unwrap(),
        persistence::encode(&restored, &recovered).unwrap()
    );

    // Novos eventos apos a restauracao.
    original
        .enqueue_batch(&[event(1, 50, 8), event(2, 60, 8)])
        .unwrap();

    restored
        .enqueue_batch(&[event(1, 50, 8), event(2, 60, 8)])
        .unwrap();

    let expected = vec![
        (2, 20, 3),
        (1, 10, 5),
        (1, 30, 5),
        (2, 40, 7),
        (1, 50, 8),
        (2, 60, 8),
    ];

    assert_eq!(drain(&mut original), expected);
    assert_eq!(drain(&mut restored), expected);

    assert_eq!(original.metrics(), restored.metrics());

    assert_eq!(
        persistence::encode(&original, &memory).unwrap(),
        persistence::encode(&restored, &recovered).unwrap()
    );
}
