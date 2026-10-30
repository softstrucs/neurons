use neurons::cognition::{Hypothesis, TwoHopTask};
use neurons::engine::{Engine, EngineError};
use neurons::engram::{Cue, Engram, EngramId, EngramStore, EvidenceId, RecordStatus};
use neurons::event::{Event, EventKind, NodeId};
use neurons::integration::IntegratedState;
use neurons::metacognition::{AbstainReason, Decision, EvidenceRefs, Monitor};
use std::time::Instant;

fn node(x: u32) -> NodeId {
    NodeId::new(x).unwrap()
}
fn monitor() -> Monitor {
    Monitor::new(TwoHopTask::new(8, 8, 16).unwrap(), 1).unwrap()
}
fn fixture() -> IntegratedState {
    let mut e = Engine::new(2, 2).unwrap();
    e.register_destination(node(1));
    for x in [10, 20] {
        e.enqueue(Event::new(None, node(1), EventKind::Stimulus, x, 3))
            .unwrap();
    }
    let mut m = EngramStore::new(4).unwrap();
    for (id, from, to) in [(1, 1, 2), (2, 2, 3)] {
        m.record(
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
    IntegratedState::new(e, m, monitor())
}
fn query(ctx: u32) -> Hypothesis {
    Hypothesis::new(ctx, node(1), node(3)).unwrap()
}
fn verify(s: &mut IntegratedState, bytes: &[u8]) {
    let result = s.review(query(1), &[]).unwrap();
    assert!(
        matches!(result.decision, Decision::Respond { evidence: EvidenceRefs::TwoHop { first, second }, .. } if first == EngramId::new(1).unwrap() && second == EngramId::new(2).unwrap())
    );
    assert_eq!(
        s.memory()
            .get(EngramId::new(1).unwrap())
            .unwrap()
            .evidence(),
        Some(EvidenceId::new(11).unwrap())
    );
    assert!(matches!(
        s.review(query(2), &[]).unwrap().decision,
        Decision::Abstain(AbstainReason::MissingEvidence)
    ));
    assert!(matches!(
        s.review(query(2), &[1]).unwrap().decision,
        Decision::Abstain(AbstainReason::ReviewBudget)
    ));
    assert!(s.review(query(1), &[1]).is_err());
    assert_eq!(s.checkpoint().unwrap(), bytes);
    assert!(matches!(
        s.engine_mut()
            .enqueue(Event::new(None, node(1), EventKind::Signal, 30, 3)),
        Err(EngineError::QueueFull)
    ));
    let mut trace = vec![];
    s.engine_mut()
        .run_cycle(|e, _| trace.push((e.destination().get(), e.payload(), e.logical_time())));
    assert_eq!(trace, [(1, 10, 3), (1, 20, 3)]);
}
#[test]
fn child_restore() {
    let Some(path) = std::env::var_os("NEURONS_EC11_CHECKPOINT") else {
        return;
    };
    let bytes = std::fs::read(path).unwrap();
    let mut s = IntegratedState::restore(&bytes, monitor()).unwrap();
    verify(&mut s, &bytes);
}
#[test]
fn frozen_panel() {
    let start = Instant::now();
    let mut previous = None;
    for repetition in 0..2 {
        let s = fixture();
        let t = Instant::now();
        let bytes = s.checkpoint().unwrap();
        let encode_us = t.elapsed().as_micros();
        if let Some(prev) = &previous {
            assert_eq!(prev, &bytes);
        }
        previous = Some(bytes.clone());
        assert!(bytes.len() <= 256 * 1024);
        let t = Instant::now();
        let mut restored = IntegratedState::restore(&bytes, monitor()).unwrap();
        let decode_us = t.elapsed().as_micros();
        verify(&mut restored, &bytes);
        let mut corrupted = bytes.clone();
        *corrupted.last_mut().unwrap() ^= 1;
        assert!(IntegratedState::restore(&corrupted, monitor()).is_err());
        assert!(IntegratedState::restore(&vec![0; 256 * 1024 + 1], monitor()).is_err());
        assert_eq!(s.checkpoint().unwrap(), bytes);
        let empty = IntegratedState::new(
            Engine::new(2, 2).unwrap(),
            EngramStore::new(4).unwrap(),
            monitor(),
        );
        assert!(matches!(
            empty.review(query(1), &[]).unwrap().decision,
            Decision::Abstain(_)
        ));
        let path = std::env::temp_dir().join(format!(
            "neurons-ec11-{}-{repetition}.bin",
            std::process::id()
        ));
        std::fs::write(&path, &bytes).unwrap();
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "child_restore", "--nocapture"])
            .env("NEURONS_EC11_CHECKPOINT", &path)
            .output()
            .unwrap();
        std::fs::remove_file(&path).unwrap();
        assert!(
            output.status.success(),
            "child: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        println!("rep={repetition} records=2 events=2 bytes={} checkpoint_us={encode_us} restore_us={decode_us}", bytes.len());
    }
    println!("panel_elapsed_us={}", start.elapsed().as_micros());
    assert!(start.elapsed().as_secs() < 30);
}
