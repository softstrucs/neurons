#[path = "support/causal_diagnostic.rs"]
mod model;
#[path = "support/causal_symbolic.rs"]
mod symbolic;
use model::{Action, Intervention, Record, Scenario};

// Oráculo exclusivamente no avaliador: nunca passado aos executores.
fn verify_prediction(r: &Record) {
    let (events, transmissions, received, fired, potential, action) =
        match (r.scenario, r.intervention) {
            (_, Intervention::F | Intervention::S) => (4, 2, 2, 1, 0, Action::A0),
            (Scenario::N, Intervention::X) => (3, 1, 1, 0, 1, Action::Silent),
            (Scenario::R, Intervention::X) => (3, 1, 1, 1, 0, Action::A0),
            (_, Intervention::B) => (3, 1, 1, 0, 1, Action::Silent),
            (_, Intervention::XB) => (2, 0, 0, 0, 0, Action::Silent),
        };
    assert_eq!(
        (
            r.processed,
            r.transmissions,
            r.states[2].received,
            r.states[2].activations,
            r.states[2].potential,
            r.action
        ),
        (events, transmissions, received, fired, potential, action)
    );
    for i in 0..2 {
        assert_eq!(
            r.states[i],
            model::State {
                potential: 0,
                received: 1,
                activations: 1
            }
        );
    }
    assert_eq!(r.states[3], model::State::default());
    assert_eq!(r.after, r.effective);
    assert_eq!(r.enqueued, r.processed as u64);
    assert_eq!(r.dispatched, r.enqueued);
    assert_eq!(r.peak, 2);
    assert_eq!(r.activations, 2 + fired);
}
fn reward(action: Action, target: Action) -> i8 {
    match action {
        Action::A0 | Action::A1 => {
            if action == target {
                1
            } else {
                -1
            }
        }
        _ => 0,
    }
}

#[test]
fn ec02_2_frozen_causal_panel() {
    let mut runs = 0;
    let mut events = 0;
    let mut first_panel = Vec::new();
    for repetition in 0..2 {
        let mut panel = Vec::new();
        for scenario in [Scenario::N, Scenario::R] {
            for intervention in [
                Intervention::F,
                Intervention::S,
                Intervention::X,
                Intervention::B,
                Intervention::XB,
            ] {
                let actual = model::execute(scenario, intervention);
                let independent = symbolic::execute(scenario, intervention);
                assert_eq!(
                    actual, independent,
                    "interromper: divergência entre implementações"
                );
                runs += 2;
                events += actual.processed + independent.processed;
                assert!(events <= 320);
                verify_prediction(&actual);
                println!("repeat={repetition} circuit={actual:?} symbolic_equal=true");
                panel.push(actual);
            }
        }
        // Todas as decisões já foram produzidas: somente agora avaliar contrastes e recompensas.
        for (offset, expected_ca) in [(0, 1), (5, 0)] {
            let factual = &panel[offset];
            let sham = &panel[offset + 1];
            let removed = &panel[offset + 2];
            assert_eq!(factual.states, sham.states);
            assert_eq!(factual.action, sham.action);
            let ca = usize::from(factual.action != removed.action);
            assert_eq!(ca, expected_ca);
            let delta_y0 = reward(factual.action, Action::A0) - reward(removed.action, Action::A0);
            let delta_y1 = reward(factual.action, Action::A1) - reward(removed.action, Action::A1);
            assert_eq!(
                (delta_y0, delta_y1),
                if offset == 0 { (1, -1) } else { (0, 0) }
            );
            println!("repeat={repetition} scenario={:?} C_A={ca} delta_Y0={delta_y0} delta_Y1={delta_y1}",factual.scenario);
        }
        assert_eq!(panel[0].states, panel[5].states);
        assert_eq!(panel[0].coactivity, [true, false, true, false]);
        assert_eq!(panel[0].coactivity, panel[5].coactivity);
        if repetition == 0 {
            first_panel = panel;
        } else {
            assert_eq!(panel, first_panel, "interromper: repetição divergente");
        }
    }
    assert_eq!(runs, 40);
    assert_eq!(events, 128);
    println!("EC-02.3 reproductions_observed={runs} events_observed={events} events_prediction=128 events_ceiling=320 comparable_records_equal=20 repeated_records_equal=10");
}
