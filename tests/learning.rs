use std::time::Instant;

use neurons::circuit::{Circuit, CircuitError};
use neurons::event::NodeId;
use neurons::learning::{Feedback, LearningError, PlasticityRule};
use neurons::neuron::Neuron;
use neurons::synapse::{Synapse, SynapseId};

fn node(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn link(value: u32) -> SynapseId {
    SynapseId::new(value).unwrap()
}

fn circuit(weight: i32, second_source: bool) -> Circuit {
    let mut network = Circuit::new(4, 4).unwrap();
    network
        .add_neuron(Neuron::new(node(1), 1, 4).unwrap())
        .unwrap();
    network
        .add_neuron(Neuron::new(node(2), 2, 4).unwrap())
        .unwrap();
    network
        .connect(Synapse::new(link(1), node(1), node(2), weight, 1).unwrap())
        .unwrap();
    if second_source {
        network
            .add_neuron(Neuron::new(node(3), 1, 4).unwrap())
            .unwrap();
        network
            .connect(Synapse::new(link(2), node(3), node(2), 0, 1).unwrap())
            .unwrap();
    }
    network
}

fn response(network: &mut Circuit, source: NodeId, payload: i32, time: u64) -> bool {
    let before = network.neuron_state(node(2)).unwrap().activations;
    network.stimulate(source, payload, time).unwrap();
    while network.queue_len() > 0 {
        let report = network.run_cycle();
        assert_eq!(report.first_error, None);
    }
    network.neuron_state(node(2)).unwrap().activations > before
}

#[test]
fn positive_feedback_changes_response_against_no_update_control() {
    let rule = PlasticityRule::new(0, 2, 2).unwrap();
    let mut learner = circuit(0, false);
    let mut control = circuit(0, false);

    // Momento 1: mesma resposta inicial sem transmissão.
    assert!(!response(&mut learner, node(1), 1, 0));
    assert!(!response(&mut control, node(1), 1, 0));
    let state_before_feedback = learner.neuron_state(node(2)).unwrap();

    // Momento 2: somente o circuito experimental recebe feedback.
    let update = rule
        .apply(&mut learner, link(1), Some(Feedback::Positive))
        .unwrap();
    let unchanged = rule.apply(&mut control, link(1), None).unwrap();
    assert_eq!((update.before, update.after), (0, 2));
    assert!(update.changed());
    assert_eq!((unchanged.before, unchanged.after), (0, 0));
    assert!(!unchanged.changed());
    assert_eq!(
        learner.neuron_state(node(2)).unwrap(),
        state_before_feedback
    );

    // Momento 3: carga 2 não foi usada na experiência com carga 1.
    assert!(response(&mut learner, node(1), 2, 2));
    assert!(!response(&mut control, node(1), 2, 2));
    assert_eq!(learner.synapse_weight(link(1)), Ok(2));
    assert_eq!(control.synapse_weight(link(1)), Ok(0));
}

#[test]
fn negative_feedback_suppresses_response_and_positive_reverses_it() {
    let rule = PlasticityRule::new(0, 2, 2).unwrap();
    let mut network = circuit(2, false);

    assert!(response(&mut network, node(1), 1, 0));
    let negative = rule
        .apply(&mut network, link(1), Some(Feedback::Negative))
        .unwrap();
    assert_eq!((negative.before, negative.after), (2, 0));
    assert!(!response(&mut network, node(1), 1, 2));
    let positive = rule
        .apply(&mut network, link(1), Some(Feedback::Positive))
        .unwrap();
    assert_eq!((positive.before, positive.after), (0, 2));
    assert!(response(&mut network, node(1), 1, 4));
}

#[test]
fn adjustment_is_bounded_and_safe_at_numeric_extremes() {
    let rule = PlasticityRule::new(-3, 3, 2).unwrap();
    let upper = rule.adjust(2, Some(Feedback::Positive)).unwrap();
    assert_eq!(upper.after, 3);
    assert!(upper.saturated);
    assert_eq!(rule.adjust(3, Some(Feedback::Positive)).unwrap().after, 3);
    let lower = rule.adjust(-2, Some(Feedback::Negative)).unwrap();
    assert_eq!(lower.after, -3);
    assert!(lower.saturated);

    let extremes = PlasticityRule::new(i32::MIN, i32::MAX, i32::MAX).unwrap();
    assert_eq!(
        extremes
            .adjust(i32::MAX, Some(Feedback::Positive))
            .unwrap()
            .after,
        i32::MAX
    );
    assert_eq!(
        extremes
            .adjust(i32::MIN, Some(Feedback::Negative))
            .unwrap()
            .after,
        i32::MIN
    );
}

#[test]
fn invalid_configuration_weight_and_synapse_leave_circuit_unchanged() {
    assert_eq!(
        PlasticityRule::new(2, 1, 1),
        Err(LearningError::InvalidBounds)
    );
    assert_eq!(
        PlasticityRule::new(0, 2, 0),
        Err(LearningError::InvalidStep)
    );
    assert_eq!(
        PlasticityRule::new(0, 2, -1),
        Err(LearningError::InvalidStep)
    );

    let rule = PlasticityRule::new(0, 2, 1).unwrap();
    assert_eq!(rule.bounds(), (0, 2));
    assert_eq!(rule.step(), 1);
    assert_eq!(
        rule.adjust(3, Some(Feedback::Positive)),
        Err(LearningError::WeightOutOfBounds {
            weight: 3,
            min: 0,
            max: 2,
        })
    );
    let mut network = circuit(3, false);
    assert_eq!(
        rule.apply(&mut network, link(1), Some(Feedback::Positive)),
        Err(LearningError::WeightOutOfBounds {
            weight: 3,
            min: 0,
            max: 2,
        })
    );
    assert_eq!(network.synapse_weight(link(1)), Ok(3));
    assert_eq!(
        rule.apply(&mut network, link(99), Some(Feedback::Positive)),
        Err(LearningError::Circuit(CircuitError::UnknownSynapse(link(
            99
        ))))
    );
    assert_eq!(network.synapse_weight(link(1)), Ok(3));
}

#[test]
fn update_is_local_to_selected_synapse_without_cross_connection_transfer() {
    let rule = PlasticityRule::new(0, 2, 2).unwrap();
    let mut network = circuit(0, true);
    assert!(!response(&mut network, node(1), 1, 0));
    rule.apply(&mut network, link(1), Some(Feedback::Positive))
        .unwrap();

    assert!(response(&mut network, node(1), 2, 2));
    assert!(!response(&mut network, node(3), 1, 4));
    assert_eq!(network.synapse_weight(link(1)), Ok(2));
    assert_eq!(network.synapse_weight(link(2)), Ok(0));
}

#[test]
fn identical_feedback_sequence_produces_identical_results() {
    fn run() -> (Vec<bool>, Vec<i32>) {
        let rule = PlasticityRule::new(0, 3, 1).unwrap();
        let mut network = circuit(0, false);
        let mut responses = Vec::new();
        let mut weights = Vec::new();
        for (step, feedback) in [
            None,
            Some(Feedback::Positive),
            Some(Feedback::Positive),
            Some(Feedback::Negative),
            None,
        ]
        .into_iter()
        .enumerate()
        {
            responses.push(response(&mut network, node(1), 1, (step as u64) * 2));
            rule.apply(&mut network, link(1), feedback).unwrap();
            weights.push(network.synapse_weight(link(1)).unwrap());
        }
        (responses, weights)
    }

    let first = run();
    assert_eq!(first, run());
    assert_eq!(first.1, [0, 1, 2, 1, 1]);
}

#[test]
fn bounded_update_probe() {
    const UPDATES: u64 = 10_000;
    let rule = PlasticityRule::new(0, 2, 1).unwrap();
    let mut network = circuit(1, false);
    let start = Instant::now();
    for step in 0..UPDATES {
        let feedback = if step % 2 == 0 {
            Feedback::Positive
        } else {
            Feedback::Negative
        };
        assert!(rule
            .apply(&mut network, link(1), Some(feedback))
            .unwrap()
            .changed());
    }
    let elapsed = start.elapsed();
    assert_eq!(network.synapse_weight(link(1)), Ok(1));
    println!(
        "stage05_probe updates={UPDATES} ns_per_update={}",
        elapsed.as_nanos() / u128::from(UPDATES)
    );
}
