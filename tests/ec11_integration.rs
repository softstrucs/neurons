use neurons::cognition::{Hypothesis, TwoHopTask};
use neurons::engine::{Engine, EngineError};
use neurons::engram::{Cue, Engram, EngramId, EngramStore, EvidenceId, RecordStatus};
use neurons::event::{Event, EventKind, NodeId};
use neurons::integration::IntegratedState;
use neurons::metacognition::{AbstainReason, Decision, EvidenceRefs, Monitor};

fn node(x: u32) -> NodeId {
    NodeId::new(x).unwrap()
}
fn monitor() -> Monitor {
    Monitor::new(TwoHopTask::new(8, 8, 16).unwrap(), 1).unwrap()
}
fn state() -> IntegratedState {
    let mut engine = Engine::new(2, 2).unwrap();
    engine.register_destination(node(1));
    for payload in [10, 20] {
        engine
            .enqueue(Event::new(None, node(1), EventKind::Stimulus, payload, 3))
            .unwrap();
    }
    let mut memory = EngramStore::new(4).unwrap();
    for (id, from, to) in [(1, 1, 2), (2, 2, 3)] {
        memory
            .record(
                Engram::new(
                    EngramId::new(id).unwrap(),
                    Cue::new(1, node(from), to).unwrap(),
                    1,
                    Some(node(from)),
                    9,
                    Some(EvidenceId::new(id + 10).unwrap()),
                    RecordStatus::Validated,
                )
                .unwrap(),
            )
            .unwrap();
    }
    IntegratedState::new(engine, memory, monitor())
}

#[test]
fn checkpoint_connects_ordered_events_contextual_evidence_and_monitor_without_mutation() {
    let original = state();
    let bytes = original.checkpoint().unwrap();
    let query = Hypothesis::new(1, node(1), node(3)).unwrap();
    let before = original.review(query, &[]).unwrap();
    assert!(
        matches!(before.decision, Decision::Respond { evidence: EvidenceRefs::TwoHop { first, second }, .. } if first == EngramId::new(1).unwrap() && second == EngramId::new(2).unwrap())
    );
    assert_eq!(original.checkpoint().unwrap(), bytes);
    let mut restored = IntegratedState::restore(&bytes, monitor()).unwrap();
    assert_eq!(restored.review(query, &[]).unwrap(), before);
    assert_eq!(
        restored
            .memory()
            .get(EngramId::new(1).unwrap())
            .unwrap()
            .evidence(),
        Some(EvidenceId::new(11).unwrap())
    );
    let missing = restored
        .review(Hypothesis::new(2, node(1), node(3)).unwrap(), &[])
        .unwrap();
    assert!(matches!(
        missing.decision,
        Decision::Abstain(AbstainReason::MissingEvidence)
    ));
    assert_eq!(restored.checkpoint().unwrap(), bytes);
    assert!(matches!(
        restored
            .engine_mut()
            .enqueue(Event::new(None, node(1), EventKind::Signal, 30, 3)),
        Err(EngineError::QueueFull)
    ));
    let mut trace = vec![];
    restored.engine_mut().run_cycle(|event, _| {
        trace.push((event.destination(), event.payload(), event.logical_time()))
    });
    assert_eq!(trace, [(node(1), 10, 3), (node(1), 20, 3)]);
    assert_eq!(original.checkpoint().unwrap(), bytes);
}

#[test]
fn invalid_checkpoint_and_empty_memory_remain_inconclusive() {
    let original = state();
    let bytes = original.checkpoint().unwrap();
    let mut corrupt = bytes.clone();
    *corrupt.last_mut().unwrap() ^= 1;
    assert!(IntegratedState::restore(&corrupt, monitor()).is_err());
    assert_eq!(original.checkpoint().unwrap(), bytes);
    let empty = IntegratedState::new(
        Engine::new(2, 2).unwrap(),
        EngramStore::new(4).unwrap(),
        monitor(),
    );
    assert!(matches!(
        empty
            .review(Hypothesis::new(1, node(1), node(3)).unwrap(), &[])
            .unwrap()
            .decision,
        Decision::Abstain(_)
    ));
}
