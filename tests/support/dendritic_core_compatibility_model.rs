use neurons::circuit::{Circuit, CircuitReport};
use neurons::event::{Event, EventKind, NodeId};
use neurons::neuron::{Neuron, NeuronState};
use neurons::synapse::{Synapse, SynapseId};

pub fn id(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

pub fn synapse(number: u32, weight: i32, delay: u64) -> Synapse {
    Synapse::new(SynapseId::new(number).unwrap(), id(1), id(2), weight, delay).unwrap()
}

pub fn legacy_circuit(target_threshold: i32, weights: &[i32]) -> Circuit {
    let mut circuit = Circuit::new(16, 32).unwrap();

    circuit
        .add_neuron(Neuron::new(id(1), 1, 10).unwrap())
        .unwrap();

    circuit
        .add_neuron(Neuron::new(id(2), target_threshold, 10).unwrap())
        .unwrap();

    for (index, weight) in weights.iter().enumerate() {
        circuit
            .connect(synapse((index + 1) as u32, *weight, 1))
            .unwrap();
    }

    circuit
}

pub fn run_legacy(circuit: &mut Circuit, stimuli: &[(i32, u64)]) -> CircuitReport {
    for &(payload, time) in stimuli {
        circuit.stimulate(id(1), payload, time).unwrap();
    }

    circuit.run_cycle()
}

// Referencia somatica independente do Circuit e do Engine.
// Utiliza os contratos reais de Neuron e Synapse.
pub fn somatic_reference(
    target_threshold: i32,
    weights: &[i32],
    stimuli: &[(i32, u64)],
) -> (NeuronState, NeuronState) {
    let mut source = Neuron::new(id(1), 1, 10).unwrap();
    let mut target = Neuron::new(id(2), target_threshold, 10).unwrap();

    let connections: Vec<Synapse> = weights
        .iter()
        .enumerate()
        .map(|(index, weight)| synapse((index + 1) as u32, *weight, 1))
        .collect();

    // Esta referencia e valida para estimulos ordenados
    // e separados o suficiente para suas transmissoes.
    for &(payload, time) in stimuli {
        let event = Event::new(None, id(1), EventKind::Stimulus, payload, time);

        if let Some(emission) = source.receive(event).unwrap() {
            for connection in &connections {
                if let Some(signal) = connection.transmit(emission).unwrap() {
                    target.receive(signal).unwrap();
                }
            }
        }
    }

    (source.state(), target.state())
}
