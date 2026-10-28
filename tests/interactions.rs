use neurons::circuit::{Circuit, CircuitError};
use neurons::engine::EngineError;
use neurons::event::{Event, EventKind, NodeId};
use neurons::neuron::{Neuron, NeuronError};
use neurons::synapse::{Synapse, SynapseError, SynapseId};
use std::time::Instant;

fn node(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn synapse_id(value: u32) -> SynapseId {
    SynapseId::new(value).unwrap()
}

fn neuron(value: u32, threshold: i32) -> Neuron {
    Neuron::new(node(value), threshold, 16).unwrap()
}

#[test]
fn neuron_validates_identity_input_and_saturates_state() {
    let a = node(1);
    let b = node(2);
    assert_eq!(
        Neuron::new(a, 0, 16).err(),
        Some(NeuronError::InvalidThreshold)
    );
    assert_eq!(
        Neuron::new(a, 2, 1).err(),
        Some(NeuronError::InvalidStateLimit)
    );

    let mut unit = Neuron::new(a, 10, 10).unwrap();
    let wrong = Event::new(None, b, EventKind::Stimulus, 1, 0);
    assert_eq!(
        unit.receive(wrong),
        Err(NeuronError::WrongDestination {
            expected: a,
            actual: b,
        })
    );
    let control = Event::new(None, a, EventKind::Control, 1, 0);
    assert_eq!(
        unit.receive(control),
        Err(NeuronError::UnsupportedEventKind)
    );
    assert_eq!(unit.state().received, 0);

    assert!(unit
        .receive(Event::new(None, a, EventKind::Stimulus, i32::MIN, 1))
        .unwrap()
        .is_none());
    assert_eq!(unit.state().potential, -10);
    let emission = unit
        .receive(Event::new(None, a, EventKind::Stimulus, i32::MAX, 2))
        .unwrap()
        .unwrap();
    assert_eq!(emission.source(), a);
    assert_eq!(emission.amplitude(), 1);
    assert_eq!(unit.state().potential, 0);
    assert_eq!(unit.state().received, 2);
    assert_eq!(unit.state().activations, 1);
}

#[test]
fn connection_causes_second_neuron_to_activate() {
    let mut circuit = Circuit::new(4, 4).unwrap();
    circuit.add_neuron(neuron(1, 2)).unwrap();
    circuit.add_neuron(neuron(2, 2)).unwrap();
    circuit
        .connect(Synapse::new(synapse_id(1), node(1), node(2), 2, 1).unwrap())
        .unwrap();
    circuit.stimulate(node(1), 1, 0).unwrap();
    circuit.stimulate(node(1), 1, 0).unwrap();

    let report = circuit.run_cycle();
    assert_eq!(report.cycle.processed, 3);
    assert_eq!(report.activations, 2);
    assert_eq!(report.transmissions, 1);
    assert_eq!(report.first_error, None);
    assert_eq!(circuit.neuron_state(node(1)).unwrap().received, 2);
    assert_eq!(circuit.neuron_state(node(2)).unwrap().received, 1);
    assert_eq!(circuit.neuron_state(node(2)).unwrap().activations, 1);
    assert_eq!(circuit.engine_metrics().dispatched, 3);
}

#[test]
fn no_connection_means_no_propagation() {
    let mut circuit = Circuit::new(2, 2).unwrap();
    circuit.add_neuron(neuron(1, 1)).unwrap();
    circuit.add_neuron(neuron(2, 1)).unwrap();
    circuit.stimulate(node(1), 1, 0).unwrap();
    let report = circuit.run_cycle();
    assert_eq!(report.activations, 1);
    assert_eq!(report.transmissions, 0);
    assert_eq!(circuit.neuron_state(node(2)).unwrap().received, 0);
}

#[test]
fn changing_weight_changes_downstream_activation() {
    fn build() -> Circuit {
        let mut circuit = Circuit::new(2, 2).unwrap();
        circuit.add_neuron(neuron(1, 1)).unwrap();
        circuit.add_neuron(neuron(2, 2)).unwrap();
        circuit
            .connect(Synapse::new(synapse_id(1), node(1), node(2), 1, 1).unwrap())
            .unwrap();
        circuit
    }

    let mut weak = build();
    let mut strong = build();
    strong.set_weight(synapse_id(1), 2).unwrap();
    for circuit in [&mut weak, &mut strong] {
        circuit.stimulate(node(1), 1, 0).unwrap();
        circuit.run_cycle();
    }
    assert_eq!(weak.neuron_state(node(2)).unwrap().potential, 1);
    assert_eq!(weak.neuron_state(node(2)).unwrap().activations, 0);
    assert_eq!(strong.neuron_state(node(2)).unwrap().potential, 0);
    assert_eq!(strong.neuron_state(node(2)).unwrap().activations, 1);
}

#[test]
fn invalid_connections_and_transmission_return_errors() {
    assert!(SynapseId::new(0).is_err());
    assert_eq!(
        Synapse::new(synapse_id(1), node(1), node(2), 1, 0).err(),
        Some(SynapseError::ZeroDelay)
    );
    let mut circuit = Circuit::new(2, 2).unwrap();
    circuit.add_neuron(neuron(1, 1)).unwrap();
    assert_eq!(
        circuit.add_neuron(neuron(1, 1)),
        Err(CircuitError::DuplicateNeuron(node(1)))
    );
    assert_eq!(
        circuit.connect(Synapse::new(synapse_id(1), node(1), node(2), 1, 1).unwrap()),
        Err(CircuitError::UnknownNeuron(node(2)))
    );
    circuit.add_neuron(neuron(2, 1)).unwrap();
    circuit
        .connect(Synapse::new(synapse_id(1), node(1), node(2), 1, 1).unwrap())
        .unwrap();
    assert_eq!(
        circuit.connect(Synapse::new(synapse_id(1), node(1), node(2), 1, 1).unwrap()),
        Err(CircuitError::DuplicateSynapse(synapse_id(1)))
    );
    assert_eq!(
        circuit.set_weight(synapse_id(2), 2),
        Err(CircuitError::UnknownSynapse(synapse_id(2)))
    );
    assert_eq!(
        circuit.stimulate(node(3), 1, 0),
        Err(CircuitError::Engine(EngineError::UnknownDestination(node(
            3
        ))))
    );
}

#[test]
fn synapse_applies_weight_delay_and_explicit_blocking() {
    let mut source = neuron(1, 1);
    let emission = source
        .receive(Event::new(None, node(1), EventKind::Stimulus, 1, 5))
        .unwrap()
        .unwrap();
    let mut link = Synapse::new(synapse_id(1), node(1), node(2), -3, 2).unwrap();
    let transmitted = link.transmit(emission).unwrap().unwrap();
    assert_eq!(transmitted.origin(), Some(node(1)));
    assert_eq!(transmitted.destination(), node(2));
    assert_eq!(transmitted.payload(), -3);
    assert_eq!(transmitted.logical_time(), 7);
    link.set_weight(0);
    assert_eq!(link.transmit(emission), Ok(None));

    let wrong_source = Synapse::new(synapse_id(2), node(2), node(1), 1, 1).unwrap();
    assert_eq!(
        wrong_source.transmit(emission),
        Err(SynapseError::WrongSource {
            expected: node(2),
            actual: node(1),
        })
    );
    let late_emission = source
        .receive(Event::new(None, node(1), EventKind::Stimulus, 1, u64::MAX))
        .unwrap()
        .unwrap();
    assert_eq!(
        wrong_source.transmit(late_emission),
        Err(SynapseError::WrongSource {
            expected: node(2),
            actual: node(1),
        })
    );
    let overflow_link = Synapse::new(synapse_id(3), node(1), node(2), 1, 1).unwrap();
    assert_eq!(
        overflow_link.transmit(late_emission),
        Err(SynapseError::TimeOverflow)
    );
}

#[test]
fn cycle_limit_bounds_a_recurrent_connection() {
    let mut circuit = Circuit::new(1, 3).unwrap();
    circuit.add_neuron(neuron(1, 1)).unwrap();
    circuit
        .connect(Synapse::new(synapse_id(1), node(1), node(1), 1, 1).unwrap())
        .unwrap();
    circuit.stimulate(node(1), 1, 0).unwrap();

    let first = circuit.run_cycle();
    assert_eq!(first.cycle.processed, 3);
    assert!(first.cycle.limit_reached);
    assert_eq!(first.cycle.remaining, 1);
    assert_eq!(circuit.neuron_state(node(1)).unwrap().activations, 3);
    assert_eq!(first.first_error, None);
}

#[test]
fn fanout_reports_queue_rejection_without_panicking() {
    let mut circuit = Circuit::new(1, 1).unwrap();
    for value in 1..=3 {
        circuit.add_neuron(neuron(value, 1)).unwrap();
    }
    circuit
        .connect(Synapse::new(synapse_id(1), node(1), node(2), 1, 1).unwrap())
        .unwrap();
    circuit
        .connect(Synapse::new(synapse_id(2), node(1), node(3), 1, 1).unwrap())
        .unwrap();
    circuit.stimulate(node(1), 1, 0).unwrap();

    let report = circuit.run_cycle();
    assert_eq!(report.transmissions, 1);
    assert_eq!(report.rejected_transmissions, 1);
    assert_eq!(
        report.first_error,
        Some(CircuitError::Engine(EngineError::QueueFull))
    );
    assert_eq!(circuit.queue_len(), 1);
    assert_eq!(circuit.engine_metrics().rejected, 1);
}

#[test]
fn identical_circuits_and_inputs_repeat_exactly() {
    fn run() -> (u64, u64, neurons::engine::EngineMetrics) {
        let mut circuit = Circuit::new(3, 2).unwrap();
        circuit.add_neuron(neuron(1, 2)).unwrap();
        circuit.add_neuron(neuron(2, 1)).unwrap();
        circuit
            .connect(Synapse::new(synapse_id(1), node(1), node(2), 1, 1).unwrap())
            .unwrap();
        circuit.stimulate(node(1), 1, 0).unwrap();
        circuit.stimulate(node(1), 1, 0).unwrap();
        while circuit.queue_len() > 0 {
            circuit.run_cycle();
        }
        (
            circuit.neuron_state(node(1)).unwrap().activations,
            circuit.neuron_state(node(2)).unwrap().activations,
            circuit.engine_metrics(),
        )
    }
    let first = run();
    assert_eq!(first, run());
    assert_eq!(first.0, 1);
    assert_eq!(first.1, 1);
}

#[test]
fn bounded_cost_probe_for_activation_and_communication() {
    const STIMULI: u64 = 10_000;

    fn measure(connected: bool) -> (u128, u64, u64) {
        let mut circuit = Circuit::new(2, 2).unwrap();
        circuit.add_neuron(neuron(1, 1)).unwrap();
        circuit.add_neuron(neuron(2, 1)).unwrap();
        if connected {
            circuit
                .connect(Synapse::new(synapse_id(1), node(1), node(2), 1, 1).unwrap())
                .unwrap();
        }
        let start = Instant::now();
        for step in 0..STIMULI {
            circuit.stimulate(node(1), 1, step * 2).unwrap();
            let report = circuit.run_cycle();
            assert_eq!(report.first_error, None);
        }
        let elapsed_ns = start.elapsed().as_nanos();
        let source = circuit.neuron_state(node(1)).unwrap().activations;
        let downstream = circuit.neuron_state(node(2)).unwrap().activations;
        (elapsed_ns, source, downstream)
    }

    let isolated = measure(false);
    let connected = measure(true);
    assert_eq!((isolated.1, isolated.2), (STIMULI, 0));
    assert_eq!((connected.1, connected.2), (STIMULI, STIMULI));
    println!(
        "stage03_probe stimuli={STIMULI} isolated_ns_per_activation={} connected_ns_per_stimulus={} connected_ns_per_activation={}",
        isolated.0 / u128::from(STIMULI),
        connected.0 / u128::from(STIMULI),
        connected.0 / u128::from(connected.1 + connected.2)
    );
}
