#[path = "support/shared_feedback_final.rs"]
mod locked;
#[path = "support/shared_feedback.rs"]
mod shared;
use neurons::languages::contextual::Choice;
use shared::{evaluate, fresh, memory, train, Context, Decision};

#[test]
fn development_shared_parameter_identity_and_interference() {
    let forms = ["dev um", "dev dois"];
    let store = memory(&forms);
    let mut model = fresh();
    assert_eq!(model.weights(1).unwrap(), [0, 0]);
    train(&mut model, 1, Choice::First);
    for form in forms {
        assert!(matches!(
            evaluate(
                &forms,
                &store,
                &model,
                form,
                Context::Finance,
                1,
                false,
                false
            ),
            Decision::First(_)
        ));
    }
    assert_eq!(model.weights(1).unwrap(), [1, 0]);
    assert_eq!(model.weights(2).unwrap(), [0, 0]);
    train(&mut model, 1, Choice::Second);
    assert_eq!(model.weights(1).unwrap(), [1, 1]);
    assert_eq!(
        evaluate(
            &forms,
            &store,
            &model,
            forms[0],
            Context::Finance,
            1,
            false,
            false
        ),
        Decision::Ambiguous
    );
}

#[test]
fn validation_distinct_forms_before_final() {
    let forms = ["validacao um", "validacao dois"];
    let store = memory(&forms);
    let mut model = fresh();
    train(&mut model, 1, Choice::First);
    for form in forms {
        let a = evaluate(
            &forms,
            &store,
            &model,
            form,
            Context::Finance,
            1,
            false,
            false,
        );
        let b = evaluate(
            &forms,
            &store,
            &model,
            form,
            Context::Finance,
            1,
            true,
            false,
        );
        assert_eq!(a, b);
        assert!(matches!(a, Decision::First(_)));
    }
}

#[test]
fn frozen_shared_feedback_protocol() {
    let forms = locked::FORMS;
    let store = memory(&forms);
    let mut learner = fresh();
    let mut repeated = fresh();
    let no_update = fresh();
    let mut independent = fresh();
    let eval =
        |m: &_, text, ctx, id, sym, ablate| evaluate(&forms, &store, m, text, ctx, id, sym, ablate);
    assert_eq!(learner.weights(1).unwrap(), [0, 0]);
    assert_eq!(
        eval(&learner, forms[0], Context::Finance, 1, false, false),
        Decision::NoActivation
    );
    let pre: Vec<_> = forms[4..]
        .iter()
        .enumerate()
        .map(|(i, t)| {
            eval(
                &learner,
                t,
                if i % 2 == 0 {
                    Context::Finance
                } else {
                    Context::Furniture
                },
                1,
                false,
                false,
            )
        })
        .collect();
    assert!(pre.iter().all(|d| *d == Decision::NoActivation));
    train(&mut learner, 1, Choice::First);
    train(&mut repeated, 1, Choice::First);
    train(&mut independent, 1, Choice::First);
    assert_eq!(learner.weights(1).unwrap(), [1, 0]);
    let trained_first = eval(&learner, forms[0], Context::Finance, 1, false, false);
    assert!(matches!(trained_first, Decision::First(_)));
    let mut hit = 0;
    let mut error = 0;
    let mut same = 0;
    let mut ablated = 0;
    let mut idle = 0;
    for (i, text) in forms[4..].iter().enumerate() {
        let ctx = if i % 2 == 0 {
            Context::Finance
        } else {
            Context::Furniture
        };
        let a = eval(&learner, text, ctx, 1, false, false);
        let b = eval(&learner, text, ctx, 1, true, false);
        assert_eq!(a, eval(&learner, text, ctx, 1, false, false));
        assert_eq!(a, eval(&repeated, text, ctx, 1, false, false));
        assert_eq!(a, b);
        same += 1;
        let removed = eval(&learner, text, ctx, 1, false, true);
        assert_eq!(removed, Decision::NoActivation);
        ablated += 1;
        let idle_decision = eval(&no_update, text, ctx, 1, false, false);
        assert_eq!(idle_decision, Decision::NoActivation);
        idle += 1;
        // O oráculo é consultado somente depois das decisões de todos os controles.
        if locked::EXPECTED_FIRST[i] {
            hit += usize::from(matches!(a, Decision::First(_)));
        } else {
            error += usize::from(matches!(a, Decision::First(_)));
        }
    }
    assert_eq!((hit, error, same, ablated, idle), (12, 12, 24, 24, 24));
    // Segunda experiência recebe feedback concorrente nos mesmos IDs sinápticos 1 e 2.
    train(&mut learner, 1, Choice::Second);
    train(&mut repeated, 1, Choice::Second);
    train(&mut independent, 2, Choice::Second);
    assert_eq!(learner.weights(1).unwrap(), [1, 1]);
    assert_eq!(learner.weights(1), repeated.weights(1));
    for (i, text) in forms[4..].iter().enumerate() {
        let context = if i % 2 == 0 {
            Context::Finance
        } else {
            Context::Furniture
        };
        let a = eval(&learner, text, context, 1, false, false);
        assert_eq!(a, Decision::Ambiguous);
        assert_eq!(a, eval(&repeated, text, context, 1, false, false));
        assert_eq!(a, eval(&learner, text, context, 1, true, false));
    }
    assert_eq!(independent.weights(1).unwrap(), [1, 0]);
    assert_eq!(independent.weights(2).unwrap(), [0, 1]);
    assert_eq!(
        eval(&learner, forms[0], Context::Finance, 1, false, false),
        Decision::Ambiguous
    );
    assert_eq!(
        eval(&learner, forms[1], Context::Furniture, 1, false, false),
        Decision::Ambiguous
    );
    assert!(matches!(
        eval(&independent, forms[0], Context::Finance, 1, false, false),
        Decision::First(_)
    ));
    assert!(matches!(
        eval(&independent, forms[1], Context::Furniture, 2, false, false),
        Decision::Second(_)
    ));
    let mut negative = 0;
    for i in 0..12 {
        let (text, cx) = match i % 3 {
            0 => ("forma ausente", Context::Finance),
            1 => (forms[4 + i], Context::Missing),
            _ => (forms[4 + i], Context::Conflict),
        };
        let a = eval(&learner, text, cx, 1, false, false);
        let b = eval(&learner, text, cx, 1, true, false);
        assert_eq!(a, b);
        assert!(matches!(a, Decision::Unknown | Decision::Conflict));
        negative += 1;
    }
    assert_eq!(negative, 12);
    println!("A02.2 reserved=24 pre=0 post=12 errors=12 no_update=0 symbolic=12 ablation=0 negative=12 trained_first=1 retained=0 interference=ambiguous independent_retained=1 independent_second=1 repeated=24");
}
