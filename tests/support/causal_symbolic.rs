//! Controle independente: agenda própria, transição neuronal e decisão próprias.
//! Compartilha somente tipos de registro e configuração experimental declarada.
use super::model::{Action, Intervention, Record, Scenario, State};
#[derive(Clone, Copy)]
struct Event {
    time: u64,
    sequence: u64,
    to: usize,
    payload: i32,
}
pub fn execute(s: Scenario, intervention: Intervention) -> Record {
    let original = match s {
        Scenario::N => [1, 0, 1, 0],
        Scenario::R => [1, 0, 2, 0],
    };
    let mut weights = original;
    match intervention {
        Intervention::F => {}
        Intervention::S => weights[0] = 1,
        Intervention::X => weights[0] = 0,
        Intervention::B => weights[2] = 0,
        Intervention::XB => {
            weights[0] = 0;
            weights[2] = 0;
        }
    }
    let initial = [State::default(); 4];
    let mut states = initial;
    let mut agenda = vec![
        Event {
            time: 0,
            sequence: 0,
            to: 0,
            payload: 1,
        },
        Event {
            time: 0,
            sequence: 1,
            to: 1,
            payload: 1,
        },
    ];
    let mut sequence = 2;
    let mut peak = 2;
    let mut processed = 0;
    let mut activations = 0;
    let mut transmissions = 0;
    while !agenda.is_empty() && processed < 8 {
        let index = (0..agenda.len())
            .min_by_key(|i| (agenda[*i].time, agenda[*i].sequence))
            .unwrap();
        let e = agenda.remove(index);
        processed += 1;
        let cell = &mut states[e.to];
        cell.received += 1;
        cell.potential = (cell.potential + e.payload).clamp(-2, 2);
        let threshold = if e.to < 2 { 1 } else { 2 };
        if cell.potential >= threshold {
            cell.potential = 0;
            cell.activations += 1;
            activations += 1;
            if e.to < 2 {
                for j in 0..2 {
                    let w = weights[2 * e.to + j];
                    if w != 0 {
                        assert!(agenda.len() < 8);
                        agenda.push(Event {
                            time: e.time + 1,
                            sequence,
                            to: 2 + j,
                            payload: w,
                        });
                        sequence += 1;
                        transmissions += 1;
                        peak = peak.max(agenda.len());
                    }
                }
            }
        }
    }
    assert!(agenda.is_empty());
    assert!(processed <= 8);
    let action = if states[2].activations != 0 {
        if states[3].activations != 0 {
            Action::Ambiguous
        } else {
            Action::A0
        }
    } else if states[3].activations != 0 {
        Action::A1
    } else {
        Action::Silent
    };
    let coactivity = [
        states[0].activations > 0 && states[2].activations > 0,
        states[0].activations > 0 && states[3].activations > 0,
        states[1].activations > 0 && states[2].activations > 0,
        states[1].activations > 0 && states[3].activations > 0,
    ];
    Record {
        scenario: s,
        intervention,
        original,
        effective: weights,
        after: weights,
        initial,
        states,
        coactivity,
        action,
        processed,
        remaining: agenda.len(),
        limit_reached: !agenda.is_empty() && processed == 8,
        enqueued: sequence,
        dispatched: processed as u64,
        rejected: 0,
        peak,
        activations,
        transmissions,
        rejected_transmissions: 0,
        error: false,
        queue_len: agenda.len(),
    }
}
