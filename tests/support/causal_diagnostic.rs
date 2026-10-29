//! EC-02.2: parâmetros e registros compartilhados; execução do circuito real.
use neurons::{
    circuit::Circuit,
    event::NodeId,
    neuron::Neuron,
    synapse::{Synapse, SynapseId},
};

pub const CAPACITY: usize = 8;
pub const LIMIT: usize = 8;
pub const ENDS: [(usize, usize); 4] = [(0, 2), (0, 3), (1, 2), (1, 3)];
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scenario {
    N,
    R,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Intervention {
    F,
    S,
    X,
    B,
    XB,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    A0,
    A1,
    Ambiguous,
    Silent,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct State {
    pub potential: i32,
    pub received: u64,
    pub activations: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    pub scenario: Scenario,
    pub intervention: Intervention,
    pub original: [i32; 4],
    pub effective: [i32; 4],
    pub after: [i32; 4],
    pub initial: [State; 4],
    pub states: [State; 4],
    pub coactivity: [bool; 4],
    pub action: Action,
    pub processed: usize,
    pub remaining: usize,
    pub limit_reached: bool,
    pub enqueued: u64,
    pub dispatched: u64,
    pub rejected: u64,
    pub peak: usize,
    pub activations: u64,
    pub transmissions: u64,
    pub rejected_transmissions: u64,
    pub error: bool,
    pub queue_len: usize,
}
pub fn original(s: Scenario) -> [i32; 4] {
    [1, 0, if s == Scenario::N { 1 } else { 2 }, 0]
}
pub fn node(i: usize) -> NodeId {
    NodeId::new(i as u32 + 1).unwrap()
}
fn snapshot(c: &Circuit) -> [State; 4] {
    std::array::from_fn(|i| {
        let x = c.neuron_state(node(i)).unwrap();
        State {
            potential: x.potential,
            received: x.received,
            activations: x.activations,
        }
    })
}
pub fn execute(s: Scenario, intervention: Intervention) -> Record {
    let weights = original(s);
    let mut c = Circuit::new(CAPACITY, LIMIT).unwrap();
    for i in 0..4 {
        c.add_neuron(Neuron::new(node(i), if i < 2 { 1 } else { 2 }, 2).unwrap())
            .unwrap();
    }
    for (i, (from, to)) in ENDS.into_iter().enumerate() {
        c.connect(
            Synapse::new(
                SynapseId::new(i as u32 + 1).unwrap(),
                node(from),
                node(to),
                weights[i],
                1,
            )
            .unwrap(),
        )
        .unwrap();
    }
    match intervention {
        Intervention::F => {}
        Intervention::S => c.set_weight(SynapseId::new(1).unwrap(), 1).unwrap(),
        Intervention::X => c.set_weight(SynapseId::new(1).unwrap(), 0).unwrap(),
        Intervention::B => c.set_weight(SynapseId::new(3).unwrap(), 0).unwrap(),
        Intervention::XB => {
            c.set_weight(SynapseId::new(1).unwrap(), 0).unwrap();
            c.set_weight(SynapseId::new(3).unwrap(), 0).unwrap();
        }
    }
    let effective = std::array::from_fn(|i| {
        c.synapse_weight(SynapseId::new(i as u32 + 1).unwrap())
            .unwrap()
    });
    let initial = snapshot(&c);
    assert_eq!(initial, [State::default(); 4]);
    assert_eq!(c.queue_len(), 0);
    c.stimulate(node(0), 1, 0).unwrap();
    c.stimulate(node(1), 1, 0).unwrap();
    let report = c.run_cycle();
    assert!(report.first_error.is_none());
    assert_eq!(report.rejected_transmissions, 0);
    assert_eq!(report.cycle.remaining, 0);
    assert!(!report.cycle.limit_reached);
    assert!(report.cycle.processed <= LIMIT);
    let states = snapshot(&c);
    let m = c.engine_metrics();
    assert_eq!(m.rejected, 0);
    let action = match (states[2].activations > 0, states[3].activations > 0) {
        (true, false) => Action::A0,
        (false, true) => Action::A1,
        (true, true) => Action::Ambiguous,
        _ => Action::Silent,
    };
    Record {
        scenario: s,
        intervention,
        original: weights,
        effective,
        after: std::array::from_fn(|i| {
            c.synapse_weight(SynapseId::new(i as u32 + 1).unwrap())
                .unwrap()
        }),
        initial,
        states,
        coactivity: std::array::from_fn(|i| {
            states[ENDS[i].0].activations > 0 && states[ENDS[i].1].activations > 0
        }),
        action,
        processed: report.cycle.processed,
        remaining: report.cycle.remaining,
        limit_reached: report.cycle.limit_reached,
        enqueued: m.enqueued,
        dispatched: m.dispatched,
        rejected: m.rejected,
        peak: m.peak_queue_len,
        activations: report.activations,
        transmissions: report.transmissions,
        rejected_transmissions: report.rejected_transmissions,
        error: report.first_error.is_some(),
        queue_len: c.queue_len(),
    }
}
