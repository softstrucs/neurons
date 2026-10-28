use neurons::engine::{Engine, EngineError};
use neurons::engram::{Cue, Engram, EngramId, EngramStore, EvidenceId, RecordStatus};
use neurons::event::{Event, EventKind, NodeId};
use neurons::persistence::{self, PersistenceError, MAX_FILE_BYTES, MAX_ITEMS};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

fn node(id: u32) -> NodeId {
    NodeId::new(id).unwrap()
}
fn event(payload: i32, time: u64) -> Event {
    Event::new(None, node(1), EventKind::Stimulus, payload, time)
}
fn record(id: u32, status: RecordStatus) -> Engram {
    Engram::new(
        EngramId::new(id).unwrap(),
        Cue::new(id, node(1), -1).unwrap(),
        i32::MIN,
        Some(node(2)),
        u64::MAX,
        if status == RecordStatus::Validated {
            Some(EvidenceId::new(9).unwrap())
        } else {
            None
        },
        status,
    )
    .unwrap()
}
fn fixture() -> (Engine, EngramStore) {
    let mut engine = Engine::new(4, 1).unwrap();
    engine.register_destination(node(1));
    engine.enqueue(event(10, 5)).unwrap();
    engine.enqueue(event(11, 5)).unwrap();
    let mut memory = EngramStore::new(4).unwrap();
    memory.record(record(1, RecordStatus::Observed)).unwrap();
    (engine, memory)
}
struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "neurons-stage11-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn path(&self) -> PathBuf {
        self.0.join("checkpoint.bin")
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn canonical_round_trip_preserves_event_types_signed_values_and_record_metadata() {
    let (mut engine, mut memory) = fixture();
    engine
        .enqueue(Event::new(
            Some(node(2)),
            node(1),
            EventKind::Signal,
            i32::MIN,
            u64::MAX,
        ))
        .unwrap();
    engine
        .enqueue(Event::new(
            None,
            node(1),
            EventKind::Control,
            i32::MAX,
            u64::MAX,
        ))
        .unwrap();
    memory.record(record(2, RecordStatus::Hypothesis)).unwrap();
    memory.record(record(3, RecordStatus::Validated)).unwrap();
    let encoded = persistence::encode(&engine, &memory).unwrap();
    let (mut restored, recovered) = persistence::decode(&encoded).unwrap();
    assert_eq!(encoded, persistence::encode(&restored, &recovered).unwrap());
    assert_eq!(
        memory.iter().collect::<Vec<_>>(),
        recovered.iter().collect::<Vec<_>>()
    );
    let mut types = Vec::new();
    for _ in 0..4 {
        restored.run_cycle(|e, _| types.push(e.kind()));
    }
    assert_eq!(
        types,
        [
            EventKind::Stimulus,
            EventKind::Stimulus,
            EventKind::Signal,
            EventKind::Control
        ]
    );
    println!("metadata_roundtrip=1/1 bytes={}", encoded.len());
}

#[test]
fn disk_resume_matches_uninterrupted_control_with_same_time_generated_and_new_events() {
    let dir = Directory::new();
    let (mut original, memory) = fixture();
    original.enqueue(event(12, 8)).unwrap();
    let first = original.run_cycle(|e, sink| {
        assert_eq!(e.payload(), 10);
        sink.enqueue(event(20, 5)).unwrap();
    });
    assert!(first.limit_reached);
    assert_eq!(first.remaining, 3);
    persistence::save(&dir.path(), &original, &memory).unwrap();
    let (mut resumed, recovered) = persistence::load(&dir.path()).unwrap();
    assert_eq!(resumed.metrics(), original.metrics());
    for engine in [&mut original, &mut resumed] {
        assert_eq!(
            engine.enqueue(event(0, 4)),
            Err(EngineError::PastLogicalTime {
                attempted: 4,
                current: 5
            })
        );
        engine.enqueue(event(13, 5)).unwrap();
        assert_eq!(engine.enqueue(event(99, 5)), Err(EngineError::QueueFull));
    }
    fn finish(engine: &mut Engine) -> Vec<i32> {
        let mut trace = Vec::new();
        for _ in 0..4 {
            assert_eq!(
                engine.run_cycle(|e, _| trace.push(e.payload())).processed,
                1
            );
        }
        assert_eq!(engine.queue_len(), 0);
        trace
    }
    let control = finish(&mut original);
    let result = finish(&mut resumed);
    assert_eq!(control, [11, 20, 13, 12]);
    assert_eq!(result, control);
    assert_eq!(
        persistence::encode(&original, &memory).unwrap(),
        persistence::encode(&resumed, &recovered).unwrap()
    );
    let empty = EngramStore::new(4).unwrap();
    assert_eq!(
        recovered
            .recall(record(1, RecordStatus::Observed).cue())
            .unwrap(),
        record(1, RecordStatus::Observed)
    );
    assert!(empty
        .recall(record(1, RecordStatus::Observed).cue())
        .is_err());
    println!("resume_trace=4/4 equal; restored_recall=1/1; empty_control=0/1; pending_checkpoint_bytes={}",std::fs::metadata(dir.path()).unwrap().len());
}

#[test]
fn corruption_and_truncation_are_rejected_without_panics() {
    let (engine, memory) = fixture();
    let bytes = persistence::encode(&engine, &memory).unwrap();
    for index in 0..bytes.len() {
        let mut corrupted = bytes.clone();
        corrupted[index] ^= 1;
        assert!(persistence::decode(&corrupted).is_err(), "byte {index}");
        assert!(
            persistence::decode(&bytes[..index]).is_err(),
            "truncation {index}"
        );
    }
    assert!(matches!(
        persistence::decode(&vec![0; MAX_FILE_BYTES + 1]),
        Err(PersistenceError::LimitExceeded)
    ));
    println!(
        "corruption_rejected={0}/{0}; truncation_rejected={0}/{0}",
        bytes.len()
    );
}
fn recheck(bytes: &mut [u8]) {
    let hash = bytes[24..].iter().fold(0xcbf29ce484222325u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    });
    bytes[16..24].copy_from_slice(&hash.to_le_bytes());
}
fn change_word(bytes: &mut [u8], index: usize, value: u64) {
    bytes[24 + index * 8..32 + index * 8].copy_from_slice(&value.to_le_bytes());
    recheck(bytes);
}
#[test]
fn checksum_valid_but_semantically_invalid_states_are_rejected() {
    let (engine, memory) = fixture();
    let bytes = persistence::encode(&engine, &memory).unwrap();
    // Schema v1: capacidades, relógio, métricas, destinos, fila e engramas.
    let mutations = [
        (0, 0),
        (0, MAX_ITEMS as u64 + 1),
        (1, 0),
        (1, 1_000_001),
        (3, 2),
        (5, 9),
        (6, 9),
        (8, 0),
        (9, MAX_ITEMS as u64 + 1),
        (10, 0),
        (14, 2),
        (15, 3),
        (16, u64::from(u32::MAX) + 1),
        (18, 0),
        (24, 0),
        (25, 5),
        (26, 0),
        (27, 0),
        (29, 0),
        (34, 2),
    ];
    for (index, value) in mutations {
        let mut invalid = bytes.clone();
        change_word(&mut invalid, index, value);
        assert!(persistence::decode(&invalid).is_err(), "word {index}");
    }
    let mut unsupported = bytes.clone();
    unsupported[8..12].copy_from_slice(&2u32.to_le_bytes());
    assert!(matches!(
        persistence::decode(&unsupported),
        Err(PersistenceError::UnsupportedVersion)
    ));
    let mut trailing = bytes.clone();
    trailing.extend_from_slice(&0u64.to_le_bytes());
    let size = (trailing.len() - 24) as u32;
    trailing[12..16].copy_from_slice(&size.to_le_bytes());
    recheck(&mut trailing);
    assert!(persistence::decode(&trailing).is_err());
    println!(
        "semantic_invalid_rejected={0}/{0}; unsupported_version=1/1; trailing_data=1/1",
        mutations.len()
    );
}

#[test]
fn maximum_supported_counts_round_trip_with_bounded_file_size() {
    let mut engine = Engine::new(MAX_ITEMS, 1_000_000).unwrap();
    let mut memory = EngramStore::new(MAX_ITEMS).unwrap();
    for id in 1..=MAX_ITEMS as u32 {
        engine.register_destination(node(id));
        engine
            .enqueue(Event::new(None, node(id), EventKind::Control, 0, 0))
            .unwrap();
        memory.record(record(id, RecordStatus::Observed)).unwrap();
    }
    let bytes = persistence::encode(&engine, &memory).unwrap();
    assert!(bytes.len() <= MAX_FILE_BYTES);
    let (restored, recovered) = persistence::decode(&bytes).unwrap();
    assert_eq!(bytes, persistence::encode(&restored, &recovered).unwrap());
    assert_eq!(restored.queue_len(), MAX_ITEMS);
    assert_eq!(recovered.len(), MAX_ITEMS);
    println!(
        "max_counts={} events/records/destinations; bytes={}",
        MAX_ITEMS,
        bytes.len()
    );
}

#[test]
fn failed_save_or_load_preserves_existing_file_and_live_objects() {
    let dir = Directory::new();
    let path = dir.path();
    let (engine, memory) = fixture();
    persistence::save(&path, &engine, &memory).unwrap();
    let before = std::fs::read(&path).unwrap();
    let too_large = Engine::new(MAX_ITEMS + 1, 1).unwrap();
    assert!(matches!(
        persistence::save(&path, &too_large, &memory),
        Err(PersistenceError::LimitExceeded)
    ));
    assert_eq!(before, std::fs::read(&path).unwrap());
    let temp = path.with_file_name("checkpoint.bin.neurons-tmp");
    std::fs::write(&temp, b"occupied").unwrap();
    assert!(persistence::save(&path, &engine, &memory).is_err());
    assert_eq!(std::fs::read(&temp).unwrap(), b"occupied");
    assert_eq!(before, std::fs::read(&path).unwrap());
    std::fs::write(&path, b"corrupt").unwrap();
    assert!(persistence::load(&path).is_err());
    assert_eq!(before, persistence::encode(&engine, &memory).unwrap());
    assert!(persistence::load(&dir.0.join("absent")).is_err());
}

#[test]
fn replacements_do_not_accumulate_files_and_failed_rename_cleans_owned_temp() {
    let dir = Directory::new();
    let (engine, memory) = fixture();
    for _ in 0..3 {
        persistence::save(&dir.path(), &engine, &memory).unwrap();
    }
    assert_eq!(std::fs::read_dir(&dir.0).unwrap().count(), 1);
    let destination = dir.0.join("directory");
    std::fs::create_dir(&destination).unwrap();
    assert!(persistence::save(&destination, &engine, &memory).is_err());
    assert!(destination.is_dir());
    assert!(!dir.0.join("directory.neurons-tmp").exists());
    println!("replacements=3; retained_checkpoint_files=1; failed_rename_cleanup=1/1");
}

#[test]
fn oversized_disk_file_is_rejected_and_idle_state_is_preserved() {
    let dir = Directory::new();
    std::fs::write(dir.path(), vec![0; MAX_FILE_BYTES + 1]).unwrap();
    assert!(matches!(
        persistence::load(&dir.path()),
        Err(PersistenceError::LimitExceeded)
    ));
    let engine = Engine::new(1, 1).unwrap();
    let memory = EngramStore::new(1).unwrap();
    let bytes = persistence::encode(&engine, &memory).unwrap();
    let (mut restored, recovered) = persistence::decode(&bytes).unwrap();
    assert_eq!(
        restored
            .run_cycle(|_, _| panic!("no queued event"))
            .processed,
        0
    );
    assert_eq!(bytes, persistence::encode(&restored, &recovered).unwrap());
}

#[test]
fn restored_memory_preserves_inference_and_empty_memory_control_does_not() {
    use neurons::cognition::{Hypothesis, TwoHopTask};
    use neurons::metacognition::{Decision, Monitor};
    let engine = Engine::new(4, 4).unwrap();
    let mut memory = EngramStore::new(4).unwrap();
    for (id, from, to) in [(1, 1, 2), (2, 2, 3)] {
        memory
            .record(
                Engram::new(
                    EngramId::new(id).unwrap(),
                    Cue::new(1, node(from), to).unwrap(),
                    1,
                    Some(node(from)),
                    0,
                    None,
                    RecordStatus::Observed,
                )
                .unwrap(),
            )
            .unwrap();
    }
    let monitor = Monitor::new(TwoHopTask::new(4, 4, 8).unwrap(), 1).unwrap();
    let query = Hypothesis::new(1, node(1), node(3)).unwrap();
    let before = monitor.review(&memory, query, &[]).unwrap();
    let dir = Directory::new();
    persistence::save(&dir.path(), &engine, &memory).unwrap();
    let (_, restored) = persistence::load(&dir.path()).unwrap();
    assert_eq!(before, monitor.review(&restored, query, &[]).unwrap());
    assert!(matches!(before.decision, Decision::Respond { .. }));
    assert!(matches!(
        monitor
            .review(&EngramStore::new(4).unwrap(), query, &[])
            .unwrap()
            .decision,
        Decision::Abstain(_)
    ));
    println!("restored_inference=1/1 exact_report; empty_memory_support=0/1");
}
