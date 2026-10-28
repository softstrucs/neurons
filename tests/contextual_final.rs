#[path = "support/context_locked.rs"]
mod locked;
#[path = "support/context_protocol.rs"]
mod protocol;
use neurons::languages::contextual::{Context, ContextOutcome};
use neurons::languages::{Direction, Language};
use protocol::{build, from_pairs, simplify, symbolic, Fact, Simple};

#[test]
fn frozen_context_protocol_and_independent_controls() {
    let facts = from_pairs(&locked::PAIRS);
    let lex = build(&facts);
    assert_eq!(lex.association_count(), 48);
    let mut correct = [0; 2];
    let mut symbolic_correct = [0; 2];
    let mut without_context = 0;
    let mut without_context_abstentions = 0;
    for (pt, en, cx) in locked::PAIRS {
        for (index, d, input, expected) in [
            (0, Direction::PtToEn, pt, en),
            (1, Direction::EnToPt, en, pt),
        ] {
            let outcome = lex.translate(d, input, Some(cx), 2).unwrap();
            assert_eq!(outcome, lex.translate(d, input, Some(cx), 2).unwrap());
            let plain = simplify(outcome);
            let expected_simple = match &plain {
                Simple::Supported(text, _, _) => text == expected,
                _ => false,
            };
            correct[index] += usize::from(expected_simple);
            let reference = symbolic(&facts, d, input, Some(cx));
            assert_eq!(plain, reference);
            symbolic_correct[index] +=
                usize::from(matches!(reference,Simple::Supported(ref text,_,_) if text==expected));
            let no_ctx = lex.translate(d, input, None, 2).unwrap();
            assert_eq!(simplify(no_ctx.clone()), symbolic(&facts, d, input, None));
            match no_ctx {
                ContextOutcome::Supported { text, .. } if text == expected => without_context += 1,
                ContextOutcome::Ambiguous => without_context_abstentions += 1,
                _ => {}
            }
        }
    }
    let absent = [
        (Direction::PtToEn, "reserva ausente alpha"),
        (Direction::PtToEn, "reserva ausente beta"),
        (Direction::PtToEn, "reserva ausente gama"),
        (Direction::PtToEn, "reserva ausente delta"),
        (Direction::EnToPt, "reserved missing alpha"),
        (Direction::EnToPt, "reserved missing beta"),
        (Direction::EnToPt, "reserved missing gamma"),
        (Direction::EnToPt, "reserved missing delta"),
    ];
    let mut negatives = 0;
    let mut negative_control = 0;
    for (d, input) in absent {
        let out = lex.translate(d, input, Some(Context::Finance), 2).unwrap();
        assert_eq!(
            out,
            lex.translate(d, input, Some(Context::Finance), 2).unwrap()
        );
        let base = symbolic(&facts, d, input, Some(Context::Finance));
        assert_eq!(simplify(out), base);
        negatives += usize::from(base == Simple::Unknown);
        negative_control += usize::from(base == Simple::Unknown);
    }
    for index in 0..4 {
        // Mesmo conjunto de fatos para o mecanismo e o controle negativo.
        let p = [
            "nova entrada alfa",
            "nova entrada beta",
            "nova entrada gama",
            "nova entrada delta",
        ][index];
        let a = [
            "fresh target alpha",
            "fresh target beta",
            "fresh target gamma",
            "fresh target delta",
        ][index];
        let b = [
            "fresh second alpha",
            "fresh second beta",
            "fresh second gamma",
            "fresh second delta",
        ][index];
        let d = if index < 2 {
            Direction::PtToEn
        } else {
            Direction::EnToPt
        };
        let (src_lang, dst_lang) = if index < 2 {
            (Language::PtBr, Language::En)
        } else {
            (Language::En, Language::PtBr)
        };
        let source = Fact {
            language: src_lang,
            text: p,
            context: Context::Finance,
            concept: 1,
            positive: true,
        };
        let target = Fact {
            language: dst_lang,
            text: a,
            context: Context::Finance,
            concept: 1,
            positive: true,
        };
        let alt = Fact {
            language: dst_lang,
            text: b,
            context: Context::Finance,
            concept: 1,
            positive: true,
        };
        let ambiguous = [source, target, alt];
        let x = build(&ambiguous);
        let out = x.translate(d, p, Some(Context::Finance), 2).unwrap();
        assert_eq!(out, x.translate(d, p, Some(Context::Finance), 2).unwrap());
        let baseline = symbolic(&ambiguous, d, p, Some(Context::Finance));
        assert_eq!(simplify(out), baseline);
        negatives += usize::from(baseline == Simple::Ambiguous);
        negative_control += usize::from(baseline == Simple::Ambiguous);
        let contradiction = [
            source,
            target,
            Fact {
                positive: false,
                ..source
            },
        ];
        let x = build(&contradiction);
        let out = x.translate(d, p, Some(Context::Finance), 2).unwrap();
        assert_eq!(out, x.translate(d, p, Some(Context::Finance), 2).unwrap());
        let baseline = symbolic(&contradiction, d, p, Some(Context::Finance));
        assert_eq!(simplify(out), baseline);
        negatives += usize::from(baseline == Simple::Conflict);
        negative_control += usize::from(baseline == Simple::Conflict);
    }
    assert_eq!(correct, [24, 24]);
    assert_eq!(symbolic_correct, [24, 24]);
    assert_eq!((without_context, without_context_abstentions), (44, 4));
    assert_eq!((negatives, negative_control), (16, 16));
    println!("contextual pt_en=24/24 en_pt=24/24 symbolic=48/48 without_context=44/48 abstentions=4/48 negatives=16/16 negative_control=16/16 conflicts=4 ambiguous=4 inconclusive=0 repeat=64/64");
}
