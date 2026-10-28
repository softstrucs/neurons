//! Instrumentação experimental; não altera o núcleo e não é API pública.
use neurons::circuit::Circuit;
use neurons::engine::Engine;
use neurons::engram::{Cue, Engram, EngramId, EngramStore, RecordStatus};
use neurons::event::{Event, EventKind, NodeId};
use neurons::learning::{Feedback, PlasticityRule};
use neurons::neuron::Neuron;
use neurons::synapse::{Synapse, SynapseId};
use std::hint::black_box;
use std::time::Instant;

pub const ROUNDS: usize = 1_000;
#[derive(Debug)]
pub struct Sample {
    pub units: usize,
    pub addressed_ns: u128,
    pub scan_ns: u128,
    pub chain_ns: u128,
    pub chain_events: usize,
    pub update_ns: u128,
    pub record_ns: u128,
    pub recall_ns: u128,
    pub dispatches: u64,
    pub activations: u64,
    pub final_weight: i32,
    pub recovered: usize,
    pub rss_before_kib: Option<u64>,
    pub rss_loaded_kib: Option<u64>,
    pub rss_after_kib: Option<u64>,
    pub rss_peak_kib: Option<u64>,
}
fn node(n: usize) -> NodeId {
    NodeId::new(n as u32).unwrap()
}
fn link(n: usize) -> SynapseId {
    SynapseId::new(n as u32).unwrap()
}
fn neuron(n: usize) -> Neuron {
    Neuron::new(node(n), 1, 1).unwrap()
}
fn memory_kib(field: &str) -> Option<u64> {
    std::fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find(|line| line.starts_with(field))?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()
}

fn rss() -> Option<u64> {
    memory_kib("VmRSS:")
}

pub fn run(units: usize) -> Result<Sample, &'static str> {
    if ![10, 100, 1000].contains(&units) {
        return Err("perfil deve ser 10, 100 ou 1000");
    }
    let rss_before_kib = rss();
    let mut addressed = Circuit::new(1, 1).unwrap();
    let mut dense = Vec::new();
    let mut engine = Engine::new(1, 1).unwrap();
    for id in 1..=units {
        addressed.add_neuron(neuron(id)).unwrap();
        dense.push(neuron(id));
        engine.register_destination(node(id));
    }
    let rss_loaded_kib = rss();
    // Mesmo motor, eventos, neurônios e ordem. Sem sinapses nesta comparação.
    let start = Instant::now();
    for tick in 0..ROUNDS {
        addressed.stimulate(node(units), 1, tick as u64).unwrap();
        let report = addressed.run_cycle();
        assert_eq!(report.first_error, None);
        assert_eq!(report.cycle.processed, 1);
    }
    let addressed_ns = start.elapsed().as_nanos();
    let start = Instant::now();
    for tick in 0..ROUNDS {
        engine
            .enqueue(Event::new(
                None,
                node(units),
                EventKind::Stimulus,
                1,
                tick as u64,
            ))
            .unwrap();
        engine.run_cycle(|event, _| {
            for unit in &mut dense {
                // Impede eliminação do trabalho que define o controle integral.
                if black_box(unit.id()) == event.destination() {
                    black_box(unit.receive(event).unwrap());
                }
            }
        });
    }
    let scan_ns = start.elapsed().as_nanos();
    assert_eq!(engine.metrics(), addressed.engine_metrics());
    for unit in &dense {
        assert_eq!(Some(unit.state()), addressed.neuron_state(unit.id()));
    }
    let activations = addressed.neuron_state(node(units)).unwrap().activations;
    drop(addressed);
    drop(dense);
    drop(engine);

    let mut chain = Circuit::new(1, units).unwrap();
    for id in 1..=units {
        chain.add_neuron(neuron(id)).unwrap();
    }
    for id in 1..units {
        chain
            .connect(Synapse::new(link(id), node(id), node(id + 1), 1, 1).unwrap())
            .unwrap();
    }
    let start = Instant::now();
    let mut chain_events = 0;
    for tick in 0..10 {
        chain.stimulate(node(1), 1, (tick * units) as u64).unwrap();
        let report = chain.run_cycle();
        assert_eq!(report.first_error, None);
        assert_eq!(report.cycle.remaining, 0);
        chain_events += report.cycle.processed;
    }
    let chain_ns = start.elapsed().as_nanos();
    assert_eq!(chain_events, 10 * units);
    assert_eq!(chain.neuron_state(node(units)).unwrap().activations, 10);
    let rule = PlasticityRule::new(0, 2, 1).unwrap();
    let start = Instant::now();
    for tick in 0..ROUNDS {
        rule.apply(
            &mut chain,
            link(units - 1),
            Some(if tick % 2 == 0 {
                Feedback::Positive
            } else {
                Feedback::Negative
            }),
        )
        .unwrap();
    }
    let update_ns = start.elapsed().as_nanos();
    let final_weight = chain.synapse_weight(link(units - 1)).unwrap();
    assert_eq!(final_weight, 1);
    drop(chain);

    let mut memory = EngramStore::new(units).unwrap();
    let start = Instant::now();
    for id in 1..=units {
        memory
            .record(
                Engram::new(
                    EngramId::new(id as u32).unwrap(),
                    Cue::new(1, node(id), 1).unwrap(),
                    id as i32,
                    Some(node(id)),
                    0,
                    None,
                    RecordStatus::Observed,
                )
                .unwrap(),
            )
            .unwrap();
    }
    let record_ns = start.elapsed().as_nanos();
    let start = Instant::now();
    let mut recovered = 0;
    for id in 1..=units {
        let record = black_box(memory.recall(Cue::new(1, node(id), 1).unwrap()).unwrap());
        assert_eq!(record.value(), id as i32);
        recovered += 1;
    }
    let recall_ns = start.elapsed().as_nanos();
    assert!(memory.recall(Cue::new(2, node(1), 1).unwrap()).is_err());
    Ok(Sample {
        units,
        addressed_ns,
        scan_ns,
        chain_ns,
        chain_events,
        update_ns,
        record_ns,
        recall_ns,
        dispatches: ROUNDS as u64,
        activations,
        final_weight,
        recovered,
        rss_before_kib,
        rss_loaded_kib,
        rss_after_kib: rss(),
        rss_peak_kib: memory_kib("VmHWM:"),
    })
}
