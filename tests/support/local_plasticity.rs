use neurons::{
    circuit::Circuit,
    event::NodeId,
    learning::{Feedback, PlasticityRule},
    neuron::Neuron,
    synapse::{Synapse, SynapseId},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    First,
    Second,
    Ambiguous,
    Silent,
    Inconclusive,
    Unknown,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Observation {
    pub decision: Decision,
    pub activity: [u64; 4],
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Local {
    pub weights: [i32; 4],
}
impl Local {
    pub fn new() -> Self {
        Self { weights: [0; 4] }
    }
    fn circuit(&self, ablated: bool) -> Circuit {
        let mut c = Circuit::new(8, 8).unwrap();
        for n in 1..=4 {
            c.add_neuron(Neuron::new(NodeId::new(n).unwrap(), 1, 2).unwrap())
                .unwrap();
        }
        for i in 0..2 {
            for j in 0..2 {
                let k = i * 2 + j;
                c.connect(
                    Synapse::new(
                        SynapseId::new(k as u32 + 1).unwrap(),
                        NodeId::new(i as u32 + 1).unwrap(),
                        NodeId::new(j as u32 + 3).unwrap(),
                        if ablated { 0 } else { self.weights[k] },
                        1,
                    )
                    .unwrap(),
                )
                .unwrap();
            }
        }
        c
    }
    fn execute(&self, input: [bool; 2], teacher: [bool; 2], ablated: bool) -> Observation {
        let mut c = self.circuit(ablated);
        for (i, active) in input.into_iter().chain(teacher).enumerate() {
            if active {
                c.stimulate(NodeId::new(i as u32 + 1).unwrap(), 1, 0)
                    .unwrap();
            }
        }
        let report = c.run_cycle();
        if report.first_error.is_some() || report.cycle.remaining > 0 {
            return Observation {
                decision: Decision::Inconclusive,
                activity: [0; 4],
            };
        }
        let mut a = [0; 4];
        for (i, x) in a.iter_mut().enumerate() {
            *x = c
                .neuron_state(NodeId::new(i as u32 + 1).unwrap())
                .unwrap()
                .activations;
        }
        Observation {
            decision: match (a[2] > 0, a[3] > 0) {
                (true, false) => Decision::First,
                (false, true) => Decision::Second,
                (true, true) => Decision::Ambiguous,
                (false, false) => Decision::Silent,
            },
            activity: a,
        }
    }
    pub fn query(&self, input: [bool; 2], ablated: bool) -> Observation {
        self.execute(input, [false, false], ablated)
    }
    // O professor ativa saída, não passa ID de sinapse. Cada conexão observa seus extremos.
    pub fn learn(&mut self, input: [bool; 2], teacher: [bool; 2]) -> Observation {
        let obs = self.execute(input, teacher, false);
        assert_ne!(obs.decision, Decision::Inconclusive);
        let rule = PlasticityRule::new(0, 2, 1).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                if obs.activity[i] > 0 && obs.activity[j + 2] > 0 {
                    let k = 2 * i + j;
                    self.weights[k] = rule
                        .adjust(self.weights[k], Some(Feedback::Positive))
                        .unwrap()
                        .after;
                }
            }
        }
        obs
    }
}
// Controle independente para consulta: soma transmissões e produz atividade e decisão.
pub fn symbolic(weights: [i32; 4], input: [bool; 2], ablated: bool) -> Observation {
    let mut a = [0_u64; 4];
    for i in 0..2 {
        if input[i] {
            a[i] = 1;
            for j in 0..2 {
                if !ablated && weights[2 * i + j] > 0 {
                    a[2 + j] += 1;
                }
            }
        }
    }
    let decision = if a[2] > 0 && a[3] > 0 {
        Decision::Ambiguous
    } else if a[2] > 0 {
        Decision::First
    } else if a[3] > 0 {
        Decision::Second
    } else {
        Decision::Silent
    };
    Observation {
        decision,
        activity: a,
    }
}
