#[path = "support/sentence_common.rs"]
mod common;
use common::{build, pair, simple, symbolic, Adj, Noun, Simple, ADJS, NOUNS};
use neurons::languages::sentences::{Gender, SentenceLexicon, SentenceOutcome};
use neurons::languages::Direction;

fn compare_negative(
    lex: &SentenceLexicon,
    nouns: &[Noun],
    adjs: &[Adj],
    direction: Direction,
    input: &str,
    conflict: Option<(&str, &str)>,
    expected: Simple,
) -> usize {
    let outcome = lex.translate(direction, input, 1).unwrap();
    assert_eq!(outcome, lex.translate(direction, input, 1).unwrap());
    let actual = simple(outcome);
    let control = symbolic(nouns, adjs, direction, input, true, conflict);
    assert_eq!(actual, control);
    assert_eq!(actual, expected);
    1
}
#[test]
fn frozen_composition_protocol_and_controls() {
    // Itens lexicais são fornecidos; as frases completas finais são combinações
    // não apresentadas entre as 24 frases de demonstração.
    let lex = build(&NOUNS, &ADJS, true);
    let training: Vec<_> = NOUNS[..4]
        .iter()
        .flat_map(|n| ADJS.iter().map(move |a| pair(*n, *a)))
        .collect();
    assert_eq!(training.len() * 2, 24);
    let mut correct = [0; 2];
    let mut symbolic_correct = 0;
    let mut word_correct = 0;
    let mut table_correct = 0;
    let mut rule_removed = 0;
    let mut lex_removed = 0;
    let no_rule = build(&NOUNS, &ADJS, false);
    for n in &NOUNS[4..] {
        for a in ADJS {
            let (pt, en) = pair(*n, a);
            for (index, d, source, expected) in [
                (0, Direction::PtToEn, pt.as_str(), en.as_str()),
                (1, Direction::EnToPt, en.as_str(), pt.as_str()),
            ] {
                let outcome = lex.translate(d, source, 1).unwrap();
                assert_eq!(outcome, lex.translate(d, source, 1).unwrap());
                if let SentenceOutcome::Supported {
                    text,
                    rule,
                    noun,
                    adjective,
                } = &outcome
                {
                    assert_ne!(*rule, *noun);
                    assert_ne!(*noun, *adjective);
                    assert_eq!(lex.evidence(*rule).unwrap().cue().context(), 2200);
                    assert_eq!(lex.evidence(*noun).unwrap().cue().context(), 2201);
                    assert_eq!(lex.evidence(*adjective).unwrap().cue().context(), 2202);
                    assert_eq!(lex.noun_fact(*noun), Some((n.pt, n.en, n.gender)));
                    assert_eq!(lex.adjective_fact(*adjective), Some((a.m, a.f, a.en)));
                    correct[index] += usize::from(text == expected);
                }
                let reference = symbolic(&NOUNS, &ADJS, d, source, true, None);
                assert_eq!(simple(outcome), reference);
                symbolic_correct +=
                    usize::from(reference == Simple::Supported(expected.to_owned()));
                let word_output = if d == Direction::PtToEn {
                    format!("{} {}", n.en, a.en)
                } else {
                    format!(
                        "{} {}",
                        if n.gender == Gender::Masculine {
                            a.m
                        } else {
                            a.f
                        },
                        n.pt
                    )
                };
                word_correct += usize::from(word_output == expected);
                let found = training.iter().find(|(p, e)| {
                    if d == Direction::PtToEn {
                        p == source
                    } else {
                        e == source
                    }
                });
                if let Some((p, e)) = found {
                    table_correct += usize::from(if d == Direction::PtToEn {
                        e == expected
                    } else {
                        p == expected
                    });
                }
                rule_removed += usize::from(
                    no_rule.translate(d, source, 1).unwrap() == SentenceOutcome::Inconclusive,
                );
                let rest: Vec<_> = NOUNS
                    .iter()
                    .copied()
                    .filter(|item| item.pt != n.pt)
                    .collect();
                let removed = build(&rest, &ADJS, true);
                lex_removed += usize::from(
                    removed.translate(d, source, 1).unwrap() == SentenceOutcome::Unknown,
                );
            }
        }
    }
    let mut negatives = 0;
    for (d, input) in [
        (Direction::PtToEn, "nãoexiste branco"),
        (Direction::PtToEn, "ausente preto"),
        (Direction::PtToEn, "vazio novo"),
        (Direction::PtToEn, "incógnita branco"),
        (Direction::EnToPt, "white absentword"),
        (Direction::EnToPt, "black missingword"),
        (Direction::EnToPt, "new phantom"),
        (Direction::EnToPt, "white nowhere"),
    ] {
        negatives += compare_negative(&lex, &NOUNS, &ADJS, d, input, None, Simple::Unknown);
    }
    for (d, input) in [
        (Direction::PtToEn, "branco barco"),
        (Direction::PtToEn, "barco branca"),
        (Direction::EnToPt, "boat white"),
        (Direction::EnToPt, "white train star"),
    ] {
        negatives += compare_negative(&lex, &NOUNS, &ADJS, d, input, None, Simple::OutOfGrammar);
    }
    // Controle recebe exatamente a mesma lista de fatos extras ou contrários.
    let mut extra_nouns = NOUNS.to_vec();
    extra_nouns.push(Noun {
        pt: "barco",
        en: "embarcação",
        gender: Gender::Masculine,
    });
    let mut ambiguous = build(&NOUNS, &ADJS, true);
    ambiguous
        .add_noun("barco", "embarcação", Gender::Masculine)
        .unwrap();
    negatives += compare_negative(
        &ambiguous,
        &extra_nouns,
        &ADJS,
        Direction::PtToEn,
        "barco branco",
        None,
        Simple::Ambiguous,
    );
    let mut extra_nouns = NOUNS.to_vec();
    extra_nouns.push(Noun {
        pt: "navio",
        en: "boat",
        gender: Gender::Masculine,
    });
    let mut ambiguous = build(&NOUNS, &ADJS, true);
    ambiguous
        .add_noun("navio", "boat", Gender::Masculine)
        .unwrap();
    negatives += compare_negative(
        &ambiguous,
        &extra_nouns,
        &ADJS,
        Direction::EnToPt,
        "white boat",
        None,
        Simple::Ambiguous,
    );
    let extra = Adj {
        m: "roxo",
        f: "roxo",
        en: "purple",
    };
    let mut extra_adjs = ADJS.to_vec();
    extra_adjs.push(extra);
    let mut conflicted = build(&NOUNS, &ADJS, true);
    let id = conflicted
        .add_adjective(extra.m, extra.f, extra.en)
        .unwrap();
    conflicted.contradict(id).unwrap();
    negatives += compare_negative(
        &conflicted,
        &NOUNS,
        &extra_adjs,
        Direction::PtToEn,
        "barco roxo",
        Some(("adjective", "purple")),
        Simple::Conflict,
    );
    negatives += compare_negative(
        &conflicted,
        &NOUNS,
        &extra_adjs,
        Direction::EnToPt,
        "purple boat",
        Some(("adjective", "purple")),
        Simple::Conflict,
    );
    assert_eq!(correct, [24, 24]);
    assert_eq!(
        (
            symbolic_correct,
            word_correct,
            table_correct,
            rule_removed,
            lex_removed,
            negatives
        ),
        (48, 0, 0, 48, 48, 16)
    );
    println!("sentences pt_en=24/24 en_pt=24/24 symbolic=48/48 word=0/48 table=0/48 removed_rule_inconclusive=48/48 removed_lex_unknown=48/48 negatives=16/16 negative_control=16/16 repeated=64/64");
}
