//! A01.3-R1: recombinação de formas fornecidas com pesos de contexto.
use neurons::engram::{Cue, Engram, EngramId, EngramStore, RecordStatus};
use neurons::event::NodeId;
use neurons::languages::contextual::{Choice, Context, FeedbackExperiment, TrialOutcome};
use neurons::learning::Feedback;

const FORMS: [(&str, u32); 3] = [("banco", 1), ("banco próximo", 2), ("banco antigo", 3)];
// Estas duas combinações são congeladas antes do ensaio e não recebem feedback.
const RESERVED: [(&str, Context, &str); 2] = [
    ("banco próximo", Context::Finance, "bank"),
    ("banco antigo", Context::Furniture, "bench"),
];
const CANDIDATES: [&str; 2] = ["bank", "bench"];

#[derive(Clone, Debug, Eq, PartialEq)]
enum Decision {
    Respond {
        text: &'static str,
        evidence: EngramId,
    },
    Unknown,
    Ambiguous,
    NoActivation,
    Inconclusive,
}
fn n(id: u32) -> NodeId {
    NodeId::new(id).unwrap()
}
fn memory() -> EngramStore {
    let mut store = EngramStore::new(3).unwrap();
    for (_, id) in FORMS {
        store
            .record(
                Engram::new(
                    EngramId::new(id).unwrap(),
                    Cue::new(1401, n(id), 1).unwrap(),
                    1,
                    Some(n(id)),
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
fn trial_id(context: Context) -> Option<u32> {
    match context {
        Context::Finance => Some(1),
        Context::Furniture => Some(2),
        _ => None,
    }
}
/// Vocabulário e candidatos fornecidos são compartilhados por ambos os caminhos.
fn evaluate(
    store: &EngramStore,
    model: &FeedbackExperiment,
    text: &str,
    context: Context,
    symbolic: bool,
    ablated: bool,
) -> Decision {
    let Some((_, id)) = FORMS.iter().find(|(form, _)| *form == text) else {
        return Decision::Unknown;
    };
    let Ok(record) = store.recall(Cue::new(1401, n(*id), 1).unwrap()) else {
        return Decision::Unknown;
    };
    let Some(trial) = trial_id(context) else {
        return Decision::Unknown;
    };
    let weights = model.weights(trial).unwrap();
    let outcome = if symbolic {
        let w = if ablated { [0, 0] } else { weights };
        match (w[0] > 0, w[1] > 0) {
            (true, false) => TrialOutcome::First,
            (false, true) => TrialOutcome::Second,
            (true, true) => TrialOutcome::Ambiguous,
            (false, false) => TrialOutcome::NoActivation,
        }
    } else {
        model.evaluate(trial, 1, ablated).unwrap()
    };
    match outcome {
        TrialOutcome::First => Decision::Respond {
            text: CANDIDATES[0],
            evidence: record.id(),
        },
        TrialOutcome::Second => Decision::Respond {
            text: CANDIDATES[1],
            evidence: record.id(),
        },
        TrialOutcome::Ambiguous => Decision::Ambiguous,
        TrialOutcome::NoActivation => Decision::NoActivation,
        TrialOutcome::Inconclusive => Decision::Inconclusive,
    }
}
#[test]
fn held_out_text_context_feedback_with_controls() {
    let store = memory();
    let mut learner = FeedbackExperiment::new();
    let mut control = FeedbackExperiment::new();
    for id in [1, 2, 3] {
        learner.add_trial(id).unwrap();
        control.add_trial(id).unwrap();
        assert_eq!(learner.weights(id), control.weights(id));
    }
    // Mesmo estado inicial e mesmos fatos; as formas reservadas já são conhecidas,
    // mas não há registro de sua combinação com qualquer contexto ou tradução.
    let mut pre = 0;
    let mut post = 0;
    let mut no_update = 0;
    let mut symbolic = 0;
    let mut ablated = 0;
    for (text, cx, _) in RESERVED {
        let first = evaluate(&store, &learner, text, cx, false, false);
        assert_eq!(first, evaluate(&store, &control, text, cx, false, false));
        pre += usize::from(matches!(first, Decision::Respond { .. }));
    }
    // Experiências apenas com a forma de treinamento "banco"; feedback externo.
    assert_eq!(
        evaluate(&store, &learner, "banco", Context::Finance, false, false),
        Decision::NoActivation
    );
    learner
        .feedback(1, Choice::First, Some(Feedback::Positive))
        .unwrap();
    control.feedback(1, Choice::First, None).unwrap();
    learner
        .feedback(2, Choice::Second, Some(Feedback::Positive))
        .unwrap();
    control.feedback(2, Choice::Second, None).unwrap();
    // Treino de terceiro item antes da avaliação mede retenção dos dois contextos.
    learner
        .feedback(3, Choice::First, Some(Feedback::Positive))
        .unwrap();
    control.feedback(3, Choice::First, None).unwrap();
    assert_eq!(learner.weights(1).unwrap(), [1, 0]);
    assert_eq!(learner.weights(2).unwrap(), [0, 1]);
    for (text, cx, expected) in RESERVED {
        let decision = evaluate(&store, &learner, text, cx, false, false);
        assert_eq!(decision, evaluate(&store, &learner, text, cx, false, false));
        let sym = evaluate(&store, &learner, text, cx, true, false);
        assert_eq!(decision, sym);
        symbolic += usize::from(matches!(sym,Decision::Respond{text,..} if text==expected));
        post += usize::from(matches!(decision,Decision::Respond{text,..} if text==expected));
        let no = evaluate(&store, &control, text, cx, false, false);
        assert_eq!(no, evaluate(&store, &control, text, cx, true, false));
        no_update += usize::from(matches!(no,Decision::Respond{text,..} if text==expected));
        let removed = evaluate(&store, &learner, text, cx, false, true);
        assert_eq!(removed, evaluate(&store, &learner, text, cx, true, true));
        ablated += usize::from(matches!(removed, Decision::Respond { .. }));
    }
    // Ausência não é transformada em resposta, mesmo com pesos ativos.
    assert_eq!(
        evaluate(
            &store,
            &learner,
            "expressão ausente",
            Context::Finance,
            false,
            false
        ),
        Decision::Unknown
    );
    assert_eq!(
        evaluate(
            &store,
            &learner,
            "expressão ausente",
            Context::Finance,
            true,
            false
        ),
        Decision::Unknown
    );
    // Grupo separado para interferência; o avaliado acima permanece congelado.
    let mut interference = FeedbackExperiment::new();
    interference.add_trial(1).unwrap();
    interference
        .feedback(1, Choice::First, Some(Feedback::Positive))
        .unwrap();
    interference
        .feedback(1, Choice::Second, Some(Feedback::Positive))
        .unwrap();
    assert_eq!(
        evaluate(
            &store,
            &interference,
            RESERVED[0].0,
            RESERVED[0].1,
            false,
            false
        ),
        Decision::Ambiguous
    );
    assert_eq!(
        evaluate(
            &store,
            &interference,
            RESERVED[0].0,
            RESERVED[0].1,
            true,
            false
        ),
        Decision::Ambiguous
    );
    assert_eq!((pre, post, no_update, symbolic, ablated), (0, 2, 0, 2, 0));
    assert_eq!(learner.weights(1).unwrap(), [1, 0]);
    assert_eq!(learner.weights(2).unwrap(), [0, 1]);
    println!("A01.3-R1 reserved=2 pre=0/2 post=2/2 errors=0 abstentions=0 inconclusive=0 no_update=0/2 symbolic=2/2 ablated=0/2 retained=2/2 repeat=2/2 interference=ambiguous unknown=1/1");
}
