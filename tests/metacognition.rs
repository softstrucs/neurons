use neurons::cognition::{Hypothesis, Outcome, TwoHopTask};
use neurons::engram::{Cue, Engram, EngramId, EngramStore, RecordStatus};
use neurons::event::NodeId;
use neurons::metacognition::{
    AbstainReason, Assessment, Conflict, Decision, EvidenceRefs, Monitor, ReviewError,
    RevisionReason,
};

fn node(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn id(value: u32) -> EngramId {
    EngramId::new(value).unwrap()
}

fn fact(id_value: u32, context: u32, from: u32, to: u32, value: i32) -> Engram {
    Engram::new(
        id(id_value),
        Cue::new(context, node(from), to as i32).unwrap(),
        value,
        Some(node(from)),
        0,
        None,
        RecordStatus::Observed,
    )
    .unwrap()
}

fn store_with(records: &[(u32, u32, u32, u32, i32)]) -> EngramStore {
    let mut store = EngramStore::new(records.len().max(1)).unwrap();
    for &(record_id, context, from, to, value) in records {
        store
            .record(fact(record_id, context, from, to, value))
            .unwrap();
    }
    store
}

fn consistent() -> EngramStore {
    store_with(&[(1, 1, 1, 2, 1), (2, 1, 2, 3, 1)])
}

fn hypothesis(context: u32) -> Hypothesis {
    Hypothesis::new(context, node(1), node(3)).unwrap()
}

fn task() -> TwoHopTask {
    TwoHopTask::new(8, 8, 16).unwrap()
}

fn monitor(attempts: usize) -> Monitor {
    Monitor::new(task(), attempts).unwrap()
}

#[test]
fn sufficient_evidence_yields_auditable_response_without_revision() {
    let store = consistent();
    let report = monitor(2).review(&store, hypothesis(1), &[]).unwrap();
    assert_eq!(report.attempt_count(), 1);
    assert_eq!(report.attempts[0].assessment, Assessment::Sufficient);
    assert_eq!(report.attempts[0].conflict, None);
    assert_eq!(
        report.decision,
        Decision::Respond {
            hypothesis: hypothesis(1),
            evidence: EvidenceRefs::TwoHop {
                first: id(1),
                second: id(2),
            },
            revision: None,
        }
    );
    let inference = report.attempts[0].inference.unwrap();
    assert!(matches!(inference.outcome, Outcome::Inferred(_)));
    assert_eq!(inference.processed_events, 3);
}

#[test]
fn missing_evidence_and_resource_exhaustion_have_distinct_reasons() {
    let missing = store_with(&[(1, 1, 1, 2, 1)]);
    let result = monitor(2).review(&missing, hypothesis(1), &[]).unwrap();
    assert_eq!(
        result.decision,
        Decision::Abstain(AbstainReason::MissingEvidence)
    );
    assert_eq!(result.attempts[0].assessment, Assessment::MissingEvidence);
    assert_eq!(result.attempts[0].evidence, None);

    let bounded = Monitor::new(TwoHopTask::new(8, 8, 2).unwrap(), 2).unwrap();
    let limited = bounded.review(&consistent(), hypothesis(1), &[]).unwrap();
    assert_eq!(
        limited.decision,
        Decision::Abstain(AbstainReason::ResourceLimited)
    );
    assert_eq!(limited.attempts[0].assessment, Assessment::ResourceLimited);
    assert_eq!(limited.attempt_count(), 1);

    let fact_limited = Monitor::new(TwoHopTask::new(1, 8, 8).unwrap(), 1)
        .unwrap()
        .review(&consistent(), hypothesis(1), &[])
        .unwrap();
    assert_eq!(
        fact_limited.decision,
        Decision::Abstain(AbstainReason::ResourceLimited)
    );
    assert!(fact_limited.attempts[0].inference.is_none());
    assert_eq!(
        fact_limited.attempts[0].evaluation_error,
        Some(neurons::cognition::CognitionError::FactLimitExceeded)
    );

    let fanout = store_with(&[(1, 1, 1, 2, 1), (2, 1, 1, 4, 1), (3, 1, 2, 3, 1)]);
    let failed = Monitor::new(TwoHopTask::new(3, 1, 8).unwrap(), 1)
        .unwrap()
        .review(&fanout, hypothesis(1), &[])
        .unwrap();
    assert_eq!(
        failed.decision,
        Decision::Abstain(AbstainReason::PropagationFailure)
    );
    assert_eq!(
        failed.attempts[0].assessment,
        Assessment::PropagationFailure
    );
}

#[test]
fn contradictory_conclusion_and_premise_are_detected_by_record_id() {
    let conclusion = store_with(&[(1, 1, 1, 2, 1), (2, 1, 2, 3, 1), (3, 1, 1, 3, -1)]);
    let report = monitor(2).review(&conclusion, hypothesis(1), &[]).unwrap();
    assert_eq!(
        report.decision,
        Decision::Abstain(AbstainReason::ContradictoryEvidence)
    );
    assert_eq!(
        report.attempts[0].assessment,
        Assessment::ContradictoryEvidence
    );
    assert_eq!(
        report.attempts[0].conflict,
        Some(Conflict::NegativeConclusion { negative: id(3) })
    );
    assert_eq!(
        report.attempts[0].evidence,
        Some(EvidenceRefs::TwoHop {
            first: id(1),
            second: id(2),
        })
    );

    let premise = store_with(&[(1, 1, 1, 2, 1), (2, 1, 2, 3, 1), (4, 1, 2, 3, -1)]);
    let report = monitor(1).review(&premise, hypothesis(1), &[]).unwrap();
    assert_eq!(
        report.attempts[0].conflict,
        Some(Conflict::NegativePremise {
            positive: id(2),
            negative: id(4),
        })
    );
}

#[test]
fn context_revision_requires_unique_supported_candidate_and_records_reason() {
    let store = store_with(&[(1, 1, 1, 2, 1), (2, 2, 1, 2, 1), (3, 2, 2, 3, 1)]);
    let result = monitor(2).review(&store, hypothesis(1), &[2]).unwrap();
    assert_eq!(result.attempt_count(), 2);
    assert_eq!(result.attempts[0].assessment, Assessment::MissingEvidence);
    assert_eq!(result.attempts[1].assessment, Assessment::Sufficient);
    assert_eq!(
        result.decision,
        Decision::Respond {
            hypothesis: hypothesis(2),
            evidence: EvidenceRefs::TwoHop {
                first: id(2),
                second: id(3),
            },
            revision: Some(RevisionReason::MissingEvidence),
        }
    );
}

#[test]
fn contradiction_can_trigger_context_revision_but_multiple_candidates_abstain() {
    let store = store_with(&[
        (1, 1, 1, 2, 1),
        (2, 1, 2, 3, 1),
        (3, 1, 1, 3, -1),
        (4, 2, 1, 2, 1),
        (5, 2, 2, 3, 1),
        (6, 3, 1, 2, 1),
        (7, 3, 2, 3, 1),
    ]);
    let revised = monitor(2).review(&store, hypothesis(1), &[2]).unwrap();
    assert!(matches!(
        revised.decision,
        Decision::Respond {
            hypothesis: Hypothesis { context: 2, .. },
            revision: Some(RevisionReason::ContradictoryEvidence),
            ..
        }
    ));
    let ambiguous = monitor(3).review(&store, hypothesis(1), &[2, 3]).unwrap();
    assert_eq!(
        ambiguous.decision,
        Decision::Abstain(AbstainReason::MultipleSupportedContexts)
    );
    assert_eq!(ambiguous.attempt_count(), 3);
}

#[test]
fn review_budget_and_invalid_inputs_stop_without_unbounded_retries() {
    assert_eq!(
        Monitor::new(task(), 0),
        Err(ReviewError::InvalidAttemptBudget)
    );
    assert_eq!(
        Monitor::new(task(), 9),
        Err(ReviewError::InvalidAttemptBudget)
    );
    let store = store_with(&[(1, 1, 1, 2, 1), (2, 2, 1, 2, 1), (3, 2, 2, 3, 1)]);
    let limited = monitor(1).review(&store, hypothesis(1), &[2]).unwrap();
    assert_eq!(
        limited.decision,
        Decision::Abstain(AbstainReason::ReviewBudget)
    );
    assert_eq!(limited.attempt_count(), 1);
    let two_supported = store_with(&[
        (1, 1, 1, 2, 1),
        (2, 2, 1, 2, 1),
        (3, 2, 2, 3, 1),
        (4, 3, 1, 2, 1),
        (5, 3, 2, 3, 1),
    ]);
    let partially_checked = monitor(2)
        .review(&two_supported, hypothesis(1), &[2, 3])
        .unwrap();
    assert_eq!(
        partially_checked.decision,
        Decision::Abstain(AbstainReason::ReviewBudget)
    );
    assert_eq!(partially_checked.attempt_count(), 2);
    assert_eq!(
        monitor(2).review(&store, hypothesis(1), &[0]),
        Err(ReviewError::InvalidCandidateContext)
    );
    assert_eq!(
        monitor(3).review(&store, hypothesis(1), &[2, 2]),
        Err(ReviewError::DuplicateCandidateContext)
    );
    let invalid = Hypothesis {
        context: 0,
        from: node(1),
        to: node(3),
    };
    assert!(matches!(
        monitor(1).review(&store, invalid, &[]),
        Err(ReviewError::InvalidHypothesis(_))
    ));
}

#[test]
fn new_evidence_changes_decision_without_changing_monitor_rule() {
    let mut store = EngramStore::new(2).unwrap();
    store.record(fact(1, 1, 1, 2, 1)).unwrap();
    let before = monitor(1).review(&store, hypothesis(1), &[]).unwrap();
    assert_eq!(
        before.decision,
        Decision::Abstain(AbstainReason::MissingEvidence)
    );

    store.record(fact(2, 1, 2, 3, 1)).unwrap();
    let after = monitor(1).review(&store, hypothesis(1), &[]).unwrap();
    assert!(matches!(
        after.decision,
        Decision::Respond { revision: None, .. }
    ));
    assert_eq!(after.attempt_count(), 1);
}

#[derive(Clone, Copy)]
enum Expected {
    Respond,
    Revise,
    Abstain,
}

#[derive(Default)]
struct Score {
    cases: usize,
    monitor_correct: usize,
    control_correct: usize,
    revisions_adequate: usize,
    abstentions_adequate: usize,
    monitor_false_alarms: usize,
    control_false_alarms: usize,
    monitor_missed_detection: usize,
    control_missed_detection: usize,
}

fn score_case(
    score: &mut Score,
    store: &EngramStore,
    task: TwoHopTask,
    candidates: &[u32],
    expected: Expected,
) {
    let initial = hypothesis(1);
    let reviewed = Monitor::new(task, 3)
        .unwrap()
        .review(store, initial, candidates)
        .unwrap();
    let raw = task.evaluate(store, initial).unwrap();
    let control_responds = matches!(raw.outcome, Outcome::DirectRecall(_) | Outcome::Inferred(_));
    let monitor_responds = matches!(reviewed.decision, Decision::Respond { .. });
    let monitor_revised = matches!(
        reviewed.decision,
        Decision::Respond {
            revision: Some(_),
            ..
        }
    );

    score.cases += 1;
    match expected {
        Expected::Respond => {
            score.monitor_correct += usize::from(monitor_responds && !monitor_revised);
            score.control_correct += usize::from(control_responds);
            score.monitor_false_alarms += usize::from(!monitor_responds);
            score.control_false_alarms += usize::from(!control_responds);
        }
        Expected::Revise => {
            score.monitor_correct += usize::from(monitor_revised);
            score.revisions_adequate += usize::from(monitor_revised);
            // O controle não recebe nem avalia o contexto alternativo.
        }
        Expected::Abstain => {
            score.monitor_correct += usize::from(!monitor_responds);
            score.control_correct += usize::from(!control_responds);
            score.abstentions_adequate += usize::from(!monitor_responds);
            score.monitor_missed_detection += usize::from(monitor_responds);
            score.control_missed_detection += usize::from(control_responds);
        }
    }
}

#[test]
fn controlled_protocol_outperforms_unmonitored_inference_on_declared_cases() {
    let mut score = Score::default();
    score_case(&mut score, &consistent(), task(), &[], Expected::Respond);
    score_case(
        &mut score,
        &store_with(&[(1, 1, 1, 2, 1)]),
        task(),
        &[],
        Expected::Abstain,
    );
    score_case(
        &mut score,
        &store_with(&[(1, 1, 1, 2, 1), (2, 1, 2, 3, 1), (3, 1, 1, 3, -1)]),
        task(),
        &[],
        Expected::Abstain,
    );
    score_case(
        &mut score,
        &store_with(&[(1, 1, 1, 2, 1), (2, 2, 1, 2, 1), (3, 2, 2, 3, 1)]),
        task(),
        &[2],
        Expected::Revise,
    );
    score_case(
        &mut score,
        &consistent(),
        TwoHopTask::new(8, 8, 2).unwrap(),
        &[],
        Expected::Abstain,
    );
    let mut changing = EngramStore::new(2).unwrap();
    changing.record(fact(1, 1, 1, 2, 1)).unwrap();
    score_case(&mut score, &changing, task(), &[], Expected::Abstain);
    changing.record(fact(2, 1, 2, 3, 1)).unwrap();
    score_case(&mut score, &changing, task(), &[], Expected::Respond);

    assert_eq!(score.cases, 7);
    assert_eq!(score.monitor_correct, 7);
    assert_eq!(score.control_correct, 5);
    assert_eq!(score.revisions_adequate, 1);
    assert_eq!(score.abstentions_adequate, 4);
    assert_eq!(score.monitor_false_alarms, 0);
    assert_eq!(score.control_false_alarms, 0);
    assert_eq!(score.monitor_missed_detection, 0);
    assert_eq!(score.control_missed_detection, 1);
    println!(
        "stage07_protocol cases={} monitor_correct={} control_correct={} adequate_revisions={} adequate_abstentions={} monitor_false_alarms={} control_false_alarms={} monitor_missed_detection={} control_missed_detection={}",
        score.cases,
        score.monitor_correct,
        score.control_correct,
        score.revisions_adequate,
        score.abstentions_adequate,
        score.monitor_false_alarms,
        score.control_false_alarms,
        score.monitor_missed_detection,
        score.control_missed_detection
    );
}

#[test]
fn identical_inputs_produce_identical_audit_log_and_decision() {
    let store = store_with(&[(1, 1, 1, 2, 1), (2, 2, 1, 2, 1), (3, 2, 2, 3, 1)]);
    let first = monitor(2).review(&store, hypothesis(1), &[2]).unwrap();
    assert_eq!(
        first,
        monitor(2).review(&store, hypothesis(1), &[2]).unwrap()
    );
}
