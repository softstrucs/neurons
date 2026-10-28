use neurons::{
    circuit::Circuit,
    event::NodeId,
    learning::{Feedback, PlasticityRule},
    neuron::Neuron,
    synapse::{Synapse, SynapseId},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    First,
    Second,
    Ambiguous,
    Silent,
    Inconclusive,
}

// Uma matriz única: entrada 0/1 × saída 0/1; IDs 1..4.
pub struct Model {
    weights: [i32; 4],
}
impl Model {
    pub fn new() -> Self {
        Self { weights: [0; 4] }
    }
    pub fn weights(&self) -> [i32; 4] {
        self.weights
    }
    // Regra supervisionada: atualiza a conexão da entrada ativa ao alvo informado.
    pub fn train(&mut self, input: [bool; 2], target: usize) {
        assert!(target < 2);
        let rule = PlasticityRule::new(0, 2, 1).unwrap();
        for (i, active) in input.into_iter().enumerate() {
            if active {
                let slot = 2 * i + target;
                self.weights[slot] = rule
                    .adjust(self.weights[slot], Some(Feedback::Positive))
                    .unwrap()
                    .after;
            }
        }
    }
    pub fn run(&self, input: [bool; 2], ablated: bool) -> (Outcome, [u64; 2]) {
        let mut c = Circuit::new(8, 8).unwrap();
        for id in 1..=4 {
            c.add_neuron(Neuron::new(NodeId::new(id).unwrap(), 1, 2).unwrap())
                .unwrap();
        }
        for i in 0..2 {
            for j in 0..2 {
                let slot = i * 2 + j;
                c.connect(
                    Synapse::new(
                        SynapseId::new(slot as u32 + 1).unwrap(),
                        NodeId::new(i as u32 + 1).unwrap(),
                        NodeId::new(j as u32 + 3).unwrap(),
                        if ablated { 0 } else { self.weights[slot] },
                        1,
                    )
                    .unwrap(),
                )
                .unwrap();
            }
        }
        for (i, active) in input.into_iter().enumerate() {
            if active {
                c.stimulate(NodeId::new(i as u32 + 1).unwrap(), 1, 0)
                    .unwrap();
            }
        }
        let report = c.run_cycle();
        if report.first_error.is_some() || report.cycle.remaining > 0 {
            return (Outcome::Inconclusive, [0, 0]);
        }
        let a = [
            c.neuron_state(NodeId::new(3).unwrap()).unwrap().activations,
            c.neuron_state(NodeId::new(4).unwrap()).unwrap().activations,
        ];
        (classify(a), a)
    }
}
fn classify(a: [u64; 2]) -> Outcome {
    match (a[0] > 0, a[1] > 0) {
        (true, false) => Outcome::First,
        (false, true) => Outcome::Second,
        (true, true) => Outcome::Ambiguous,
        (false, false) => Outcome::Silent,
    }
}
// Controle simbólico independente soma os sinais sobre a mesma matriz e orçamento.
pub fn symbolic(weights: [i32; 4], input: [bool; 2], ablated: bool) -> (Outcome, [u64; 2]) {
    let mut activations = [0_u64; 2];
    for i in 0..2 {
        if input[i] {
            for j in 0..2 {
                if !ablated && weights[i * 2 + j] > 0 {
                    activations[j] += 1;
                }
            }
        }
    }
    let result = if activations[0] > 0 && activations[1] > 0 {
        Outcome::Ambiguous
    } else if activations[0] > 0 {
        Outcome::First
    } else if activations[1] > 0 {
        Outcome::Second
    } else {
        Outcome::Silent
    };
    (result, activations)
}
