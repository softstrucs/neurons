use neurons::engram::{Cue, Engram, EngramId, EngramStore, RecordStatus};
use neurons::event::NodeId;
use neurons::languages::contextual::{Choice, FeedbackExperiment, TrialOutcome};
use neurons::learning::Feedback;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Context {
    Finance,
    Furniture,
    Missing,
    Conflict,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Decision {
    First(u32),
    Second(u32),
    Unknown,
    Conflict,
    Ambiguous,
    NoActivation,
    Inconclusive,
}

pub fn memory(forms: &[&str]) -> EngramStore {
    let mut store = EngramStore::new(forms.len()).unwrap();
    for (i, _) in forms.iter().enumerate() {
        let id = (i + 1) as u32;
        store
            .record(
                Engram::new(
                    EngramId::new(id).unwrap(),
                    Cue::new(1600, NodeId::new(id).unwrap(), 1).unwrap(),
                    1,
                    None,
                    0,
                    None,
                    RecordStatus::Observed,
                )
                .unwrap(),
            )
            .unwrap();
    }
    store
}

// A seleção literal de texto e a codificação contextual são externas ao circuito.
pub fn evaluate(
    forms: &[&str],
    store: &EngramStore,
    model: &FeedbackExperiment,
    text: &str,
    context: Context,
    id: u32,
    symbolic: bool,
    ablated: bool,
) -> Decision {
    let Some(index) = forms.iter().position(|f| *f == text) else {
        return Decision::Unknown;
    };
    let evidence = store
        .recall(Cue::new(1600, NodeId::new((index + 1) as u32).unwrap(), 1).unwrap())
        .unwrap()
        .id()
        .get();
    match context {
        Context::Missing => return Decision::Unknown,
        Context::Conflict => return Decision::Conflict,
        _ => {}
    }
    let outcome = if symbolic {
        let w = if ablated {
            [0, 0]
        } else {
            model.weights(id).unwrap()
        };
        match (w[0] > 0, w[1] > 0) {
            (true, false) => TrialOutcome::First,
            (false, true) => TrialOutcome::Second,
            (true, true) => TrialOutcome::Ambiguous,
            (false, false) => TrialOutcome::NoActivation,
        }
    } else {
        model.evaluate(id, 1, ablated).unwrap()
    };
    match outcome {
        TrialOutcome::First => Decision::First(evidence),
        TrialOutcome::Second => Decision::Second(evidence),
        TrialOutcome::Ambiguous => Decision::Ambiguous,
        TrialOutcome::NoActivation => Decision::NoActivation,
        TrialOutcome::Inconclusive => Decision::Inconclusive,
    }
}
pub fn fresh() -> FeedbackExperiment {
    let mut x = FeedbackExperiment::new();
    x.add_trial(1).unwrap();
    x.add_trial(2).unwrap();
    x
}
pub fn train(model: &mut FeedbackExperiment, id: u32, choice: Choice) {
    model
        .feedback(id, choice, Some(Feedback::Positive))
        .unwrap();
}
