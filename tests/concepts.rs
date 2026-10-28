#[allow(dead_code)]
#[path = "support/lexical.rs"]
mod development;
#[path = "support/concept_locked.rs"]
mod final_data;
use neurons::languages::{ConceptError, ConceptLexicon, ConceptOutcome, Direction, Language};
fn build(pairs: &[(&str, &str)]) -> ConceptLexicon {
    let mut x = ConceptLexicon::new();
    for (i, (pt, en)) in pairs.iter().enumerate() {
        let id = (i + 1) as u32;
        x.add_concept(id, &format!("referente: {pt}")).unwrap();
        x.associate(Language::PtBr, pt, id, true).unwrap();
        x.associate(Language::En, en, id, true).unwrap();
    }
    x
}
#[test]
fn development_sixteen_queries() {
    let c = build(&development::PAIRS[..8]);
    for (pt, en) in &development::PAIRS[..8] {
        assert!(
            matches!(c.compose(Direction::PtToEn,pt,2).unwrap(),ConceptOutcome::Supported{text,..} if text==*en)
        );
        assert!(
            matches!(c.compose(Direction::EnToPt,en,2).unwrap(),ConceptOutcome::Supported{text,..} if text==*pt)
        );
    }
}
#[test]
fn budgets_invalid_registration_and_absence_are_explicit() {
    let mut c = ConceptLexicon::new();
    assert_eq!(c.add_concept(0, "x"), Err(ConceptError::InvalidInput));
    assert_eq!(
        c.associate(Language::PtBr, "casa", 1, true),
        Err(ConceptError::UnknownConcept)
    );
    c.add_concept(1, "edificação").unwrap();
    c.associate(Language::PtBr, "casa", 1, true).unwrap();
    assert_eq!(
        c.associate(Language::PtBr, "casa", 1, true),
        Err(ConceptError::Duplicate)
    );
    assert_eq!(
        c.compose(Direction::PtToEn, "casa", 1),
        Ok(ConceptOutcome::Inconclusive)
    );
    assert_eq!(
        c.compose(Direction::PtToEn, "casa", 3),
        Err(ConceptError::InvalidInput)
    );
    assert_eq!(
        c.compose(Direction::PtToEn, "casa", 2),
        Ok(ConceptOutcome::Unknown)
    );
    assert_eq!(c.association_count(), 1);
}
#[test]
fn ambiguity_and_conflict_are_explicit_and_reproducible() {
    let mut c = ConceptLexicon::new();
    c.add_concept(1, "sentido").unwrap();
    c.associate(Language::PtBr, "banco", 1, true).unwrap();
    c.associate(Language::En, "bank", 1, true).unwrap();
    c.associate(Language::En, "bench", 1, true).unwrap();
    let result = c.compose(Direction::PtToEn, "banco", 2).unwrap();
    assert_eq!(result, ConceptOutcome::Ambiguous);
    assert_eq!(result, c.compose(Direction::PtToEn, "banco", 2).unwrap());
    c.associate(Language::PtBr, "banco", 1, false).unwrap();
    assert_eq!(
        c.compose(Direction::PtToEn, "banco", 2),
        Ok(ConceptOutcome::Conflict)
    );
}
#[test]
fn controlled_final_protocol_with_symbolic_and_ablation_controls() {
    let pairs = &final_data::PAIRS[..16];
    let c = build(pairs);
    assert_eq!(c.association_count(), 32);
    // Grafo de controle contém somente arestas tipadas forma→conceito.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Vertex<'a> {
        Form(Language, &'a str),
        Concept(u32),
    }
    let mut facts = Vec::new();
    for (i, (pt, en)) in pairs.iter().enumerate() {
        facts.push((
            Vertex::Form(Language::PtBr, *pt),
            Vertex::Concept((i + 1) as u32),
        ));
        facts.push((
            Vertex::Form(Language::En, *en),
            Vertex::Concept((i + 1) as u32),
        ));
    }
    let mut correct = [0, 0];
    let mut symbolic = [0, 0];
    let mut direct = 0;
    let mut removed = 0;
    for (pt, en) in pairs {
        for (index, d, input, expected) in [
            (0, Direction::PtToEn, *pt, *en),
            (1, Direction::EnToPt, *en, *pt),
        ] {
            let out = c.compose(d, input, 2).unwrap();
            assert_eq!(out, c.compose(d, input, 2).unwrap());
            if matches!(out,ConceptOutcome::Supported{ref text,..} if text==expected) {
                correct[index] += 1;
            }
            // Controle simbólico e consulta direta acessam os mesmos fatos tipados.
            let (source_lang, target_lang) = if index == 0 {
                (Language::PtBr, Language::En)
            } else {
                (Language::En, Language::PtBr)
            };
            let source = Vertex::Form(source_lang, input);
            let target = Vertex::Form(target_lang, expected);
            if facts.iter().any(|(a, b)| *a == source && *b == target) {
                direct += 1;
            }
            if facts.iter().any(|(a, concept)| {
                *a == source
                    && matches!(concept, Vertex::Concept(_))
                    && facts
                        .iter()
                        .any(|(form, c)| *form == target && c == concept)
            }) {
                symbolic[index] += 1;
            }
            let mut incomplete = ConceptLexicon::new();
            for (i, (p, e)) in pairs.iter().enumerate() {
                let concept = (i + 1) as u32;
                incomplete
                    .add_concept(concept, &format!("referente: {p}"))
                    .unwrap();
                incomplete
                    .associate(Language::PtBr, p, concept, true)
                    .unwrap();
                if *p != *pt || index == 1 {
                    incomplete
                        .associate(Language::En, e, concept, true)
                        .unwrap();
                }
            }
            // Na direção inversa, remover a relação de origem em inglês.
            if index == 1 {
                let mut inverse = ConceptLexicon::new();
                for (i, (p, e)) in pairs.iter().enumerate() {
                    let concept = (i + 1) as u32;
                    inverse
                        .add_concept(concept, &format!("referente: {p}"))
                        .unwrap();
                    inverse.associate(Language::PtBr, p, concept, true).unwrap();
                    if *e != *en {
                        inverse.associate(Language::En, e, concept, true).unwrap();
                    }
                }
                if matches!(
                    inverse.compose(d, input, 2).unwrap(),
                    ConceptOutcome::Supported { .. }
                ) {
                    removed += 1;
                }
            } else if matches!(
                incomplete.compose(d, input, 2).unwrap(),
                ConceptOutcome::Supported { .. }
            ) {
                removed += 1;
            }
        }
    }
    let mut negative = 0;
    for (d, text) in [
        (Direction::PtToEn, "semregistro_pt_a"),
        (Direction::PtToEn, "semregistro_pt_b"),
        (Direction::PtToEn, "semregistro_pt_c"),
        (Direction::PtToEn, "semregistro_pt_d"),
        (Direction::EnToPt, "missing_en_a"),
        (Direction::EnToPt, "missing_en_b"),
        (Direction::EnToPt, "missing_en_c"),
        (Direction::EnToPt, "missing_en_d"),
    ] {
        let out = c.compose(d, text, 2).unwrap();
        assert_eq!(out, c.compose(d, text, 2).unwrap());
        if out == ConceptOutcome::Unknown {
            negative += 1;
        }
    }
    for i in 0..4 {
        let mut a = ConceptLexicon::new();
        a.add_concept(1, "ambíguo").unwrap();
        let p = format!("ptreservado{i}");
        let e = format!("enreservado{i}");
        a.associate(Language::PtBr, &p, 1, true).unwrap();
        a.associate(Language::En, &e, 1, true).unwrap();
        if i < 2 {
            a.associate(Language::En, &format!("destinoreservado{i}"), 1, true)
                .unwrap();
            let out = a.compose(Direction::PtToEn, &p, 2).unwrap();
            assert_eq!(out, a.compose(Direction::PtToEn, &p, 2).unwrap());
            if out == ConceptOutcome::Ambiguous {
                negative += 1;
            }
        } else {
            a.associate(Language::PtBr, &format!("destinoreservado{i}"), 1, true)
                .unwrap();
            let out = a.compose(Direction::EnToPt, &e, 2).unwrap();
            assert_eq!(out, a.compose(Direction::EnToPt, &e, 2).unwrap());
            if out == ConceptOutcome::Ambiguous {
                negative += 1;
            }
        }
        let mut conflict = ConceptLexicon::new();
        conflict.add_concept(1, "conflito").unwrap();
        conflict.associate(Language::PtBr, &p, 1, true).unwrap();
        conflict.associate(Language::En, &e, 1, true).unwrap();
        let (lang, token, d) = if i < 2 {
            (Language::PtBr, &p, Direction::PtToEn)
        } else {
            (Language::En, &e, Direction::EnToPt)
        };
        conflict.associate(lang, token, 1, false).unwrap();
        let out = conflict.compose(d, token, 2).unwrap();
        assert_eq!(out, conflict.compose(d, token, 2).unwrap());
        if out == ConceptOutcome::Conflict {
            negative += 1;
        }
    }
    assert_eq!(correct, [16, 16]);
    assert_eq!(symbolic, [16, 16]);
    assert_eq!((direct, removed, negative), (0, 0, 16));
    println!("concepts pt_en=16/16 en_pt=16/16 symbolic=32/32 direct=0/32 removed=0/32 negatives=16/16 ambiguous=4 conflicts=4 inconclusive=0 positive_coverage=32/32 false_abstentions=0/32");
}

#[test]
fn too_many_candidates_return_explicit_limit() {
    let mut c = ConceptLexicon::new();
    c.add_concept(1, "sentido").unwrap();
    c.associate(Language::PtBr, "origem", 1, true).unwrap();
    for i in 0..9 {
        c.associate(Language::En, &format!("target{i}"), 1, true)
            .unwrap();
    }
    assert_eq!(
        c.compose(Direction::PtToEn, "origem", 2),
        Ok(ConceptOutcome::LimitExceeded)
    );
}
