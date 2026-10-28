use neurons::cognition::{CognitionError, Hypothesis, Outcome, TwoHopTask};
use neurons::engram::{Cue, Engram, EngramError, EngramId, EngramStore, RecordStatus};
use neurons::event::NodeId;

fn node(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn fact(id: u32, context: u32, from: u32, to: u32, status: RecordStatus) -> Engram {
    Engram::new(
        EngramId::new(id).unwrap(),
        Cue::new(context, node(from), to as i32).unwrap(),
        1,
        Some(node(from)),
        0,
        None,
        status,
    )
    .unwrap()
}

fn task() -> TwoHopTask {
    TwoHopTask::new(16, 16, 32).unwrap()
}

fn query(context: u32, from: u32, to: u32) -> Hypothesis {
    Hypothesis::new(context, node(from), node(to)).unwrap()
}

fn training_facts() -> EngramStore {
    let mut store = EngramStore::new(8).unwrap();
    for (id, from, to) in [
        (1, 1, 2),
        (2, 2, 3),
        (3, 4, 5),
        (4, 5, 6),
        (5, 7, 8),
        (6, 8, 9),
        (7, 10, 11),
        (8, 11, 12),
    ] {
        store
            .record(fact(id, 1, from, to, RecordStatus::Observed))
            .unwrap();
    }
    store
}

#[test]
fn two_hop_answer_requires_both_facts_and_coordinated_activations() {
    let store = training_facts();
    let report = task().evaluate(&store, query(1, 1, 3)).unwrap();
    let Outcome::Inferred(proof) = report.outcome else {
        panic!("a composição deveria ter suporte: {report:?}");
    };
    assert_eq!(proof.via, node(2));
    assert_eq!(proof.first_fact, EngramId::new(1).unwrap());
    assert_eq!(proof.second_fact, EngramId::new(2).unwrap());
    assert_eq!(proof.via_activations, 1);
    assert_eq!(report.source_activations, 1);
    assert_eq!(report.target_activations, 1);
    assert_eq!(report.processed_events, 3);
    assert_eq!(store.get(proof.first_fact).unwrap().cue().source(), node(1));
    assert_eq!(
        store.get(proof.second_fact).unwrap().cue().source(),
        node(2)
    );
    assert_eq!(
        store.recall(Cue::new(1, node(1), 3).unwrap()),
        Err(EngramError::NoAssociation)
    );
}

#[test]
fn direct_fact_is_reported_as_recall_not_inference() {
    let store = training_facts();
    let report = task().evaluate(&store, query(1, 1, 2)).unwrap();
    assert_eq!(
        report.outcome,
        Outcome::DirectRecall(EngramId::new(1).unwrap())
    );
    assert_eq!(report.processed_events, 0);
    assert_eq!(report.target_activations, 0);
}

#[test]
fn no_synapse_control_and_no_memory_do_not_infer() {
    let store = training_facts();
    let hypothesis = query(1, 1, 3);
    let control = task()
        .evaluate_without_synapses(&store, hypothesis)
        .unwrap();
    assert_eq!(control.outcome, Outcome::Unsupported);
    assert_eq!(control.target_activations, 0);
    assert_eq!(control.processed_events, 1);

    let empty = EngramStore::new(1).unwrap();
    let absent = task().evaluate(&empty, hypothesis).unwrap();
    assert_eq!(absent.outcome, Outcome::Unsupported);
    assert_eq!(absent.facts_loaded, 0);
}

#[test]
fn protocol_scores_held_out_queries_against_simple_baselines() {
    let store = training_facts();
    let held_out = [
        (1, 3, true),
        (4, 6, true),
        (7, 9, true),
        (10, 12, true),
        (1, 6, false),
        (4, 9, false),
        (7, 12, false),
        (10, 3, false),
    ];
    let direct_table: Vec<_> = store
        .iter()
        .map(|record| (record.cue().source(), record.cue().pattern()))
        .collect();

    let mut model_correct = 0;
    let mut recall_correct = 0;
    let mut table_correct = 0;
    let mut symbolic_correct = 0;
    let mut ablated_correct = 0;
    let mut random_correct = 0;
    let mut random_state = 17_u64;
    for (from, to, expected) in held_out {
        let result = task().evaluate(&store, query(1, from, to)).unwrap();
        let predicted = matches!(result.outcome, Outcome::Inferred(_));
        assert!(matches!(
            result.outcome,
            Outcome::Inferred(_) | Outcome::Unsupported
        ));
        model_correct += usize::from(predicted == expected);

        let recalled = store
            .recall(Cue::new(1, node(from), to as i32).unwrap())
            .is_ok();
        recall_correct += usize::from(recalled == expected);
        let table_answer = direct_table.contains(&(node(from), to as i32));
        table_correct += usize::from(table_answer == expected);
        let symbolic_answer = direct_table.iter().any(|(first_from, middle)| {
            *first_from == node(from) && direct_table.contains(&(node(*middle as u32), to as i32))
        });
        symbolic_correct += usize::from(symbolic_answer == expected);
        let ablated = task()
            .evaluate_without_synapses(&store, query(1, from, to))
            .unwrap();
        ablated_correct += usize::from(matches!(ablated.outcome, Outcome::Inferred(_)) == expected);
        random_state = random_state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        let random_answer = random_state >> 63 != 0;
        random_correct += usize::from(random_answer == expected);
    }

    assert_eq!(model_correct, held_out.len());
    assert_eq!(recall_correct, 4);
    assert_eq!(table_correct, 4);
    assert_eq!(symbolic_correct, held_out.len());
    assert_eq!(ablated_correct, 4);
    println!(
        "stage06_protocol total={} model={} direct_recall={} fixed_table={} symbolic_two_hop={} no_synapses={} seeded_random={}",
        held_out.len(),
        model_correct,
        recall_correct,
        table_correct,
        symbolic_correct,
        ablated_correct,
        random_correct
    );
}

#[test]
fn longer_path_or_unconfirmed_fact_does_not_count_as_two_hop_proof() {
    let mut store = EngramStore::new(4).unwrap();
    store
        .record(fact(1, 1, 1, 2, RecordStatus::Observed))
        .unwrap();
    store
        .record(fact(2, 1, 2, 3, RecordStatus::Observed))
        .unwrap();
    store
        .record(fact(3, 1, 3, 4, RecordStatus::Observed))
        .unwrap();
    let longer = task().evaluate(&store, query(1, 1, 4)).unwrap();
    assert_eq!(longer.outcome, Outcome::Unsupported);
    assert_eq!(longer.target_activations, 1);

    store
        .record(fact(4, 1, 2, 4, RecordStatus::Hypothesis))
        .unwrap();
    let hypothetical = task().evaluate(&store, query(1, 1, 4)).unwrap();
    assert_eq!(hypothetical.outcome, Outcome::Unsupported);
    assert_eq!(hypothetical.facts_loaded, 3);
}

#[test]
fn facts_from_other_contexts_cannot_complete_a_path() {
    let mut store = EngramStore::new(2).unwrap();
    store
        .record(fact(1, 1, 1, 2, RecordStatus::Observed))
        .unwrap();
    store
        .record(fact(2, 2, 2, 3, RecordStatus::Observed))
        .unwrap();

    let report = task().evaluate(&store, query(1, 1, 3)).unwrap();
    assert_eq!(report.outcome, Outcome::Unsupported);
    assert_eq!(report.facts_loaded, 1);
}

#[test]
fn invalid_config_duplicate_relation_and_limits_are_explicit() {
    assert_eq!(TwoHopTask::new(0, 1, 1), Err(CognitionError::ZeroFactLimit));
    assert_eq!(
        TwoHopTask::new(1, 0, 1),
        Err(CognitionError::ZeroQueueCapacity)
    );
    assert_eq!(
        TwoHopTask::new(1, 1, 0),
        Err(CognitionError::ZeroEventBudget)
    );
    assert_eq!(
        Hypothesis::new(0, node(1), node(2)),
        Err(CognitionError::InvalidContext)
    );
    assert_eq!(
        Hypothesis::new(1, node(1), node(1)),
        Err(CognitionError::ReflexiveQuery)
    );

    let store = training_facts();
    assert_eq!(
        TwoHopTask::new(1, 4, 4)
            .unwrap()
            .evaluate(&store, query(1, 1, 3)),
        Err(CognitionError::FactLimitExceeded)
    );
    let budget = TwoHopTask::new(8, 8, 2)
        .unwrap()
        .evaluate(&store, query(1, 1, 3))
        .unwrap();
    assert_eq!(budget.outcome, Outcome::InconclusiveBudget);
    assert_eq!(budget.processed_events, 2);

    let mut duplicate = EngramStore::new(2).unwrap();
    duplicate
        .record(fact(1, 1, 1, 2, RecordStatus::Observed))
        .unwrap();
    duplicate
        .record(fact(2, 1, 1, 2, RecordStatus::Observed))
        .unwrap();
    assert_eq!(
        task().evaluate(&duplicate, query(1, 1, 3)),
        Err(CognitionError::DuplicateRelation {
            from: node(1),
            to: node(2),
        })
    );
}

#[test]
fn queue_overflow_is_inconclusive_and_repetition_is_exact() {
    let mut store = EngramStore::new(3).unwrap();
    for (id, from, to) in [(1, 1, 2), (2, 1, 4), (3, 2, 3)] {
        store
            .record(fact(id, 1, from, to, RecordStatus::Observed))
            .unwrap();
    }
    let limited = TwoHopTask::new(3, 1, 8).unwrap();
    let first = limited.evaluate(&store, query(1, 1, 3)).unwrap();
    assert!(matches!(first.outcome, Outcome::InconclusivePropagation(_)));
    assert_eq!(first, limited.evaluate(&store, query(1, 1, 3)).unwrap());

    let full = task().evaluate(&store, query(1, 1, 3)).unwrap();
    assert!(matches!(full.outcome, Outcome::Inferred(_)));
    assert_eq!(full, task().evaluate(&store, query(1, 1, 3)).unwrap());
}
