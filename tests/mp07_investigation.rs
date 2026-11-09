//! Dados públicos próprios MP07; fonte fixa antes da pergunta, sem oráculo.
use neurons::cognitive_contracts::*;
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::NodeId;
use neurons::integration::acquisition::AdmissionVerdict;
use neurons::integration::investigation::*;
use neurons::session::f1::Session;
fn scope(ctx: u32) -> Scope {
    Scope {
        catalog: CatalogId::new(71).unwrap(),
        context: ContextId::new(ctx).unwrap(),
        rule: RuleRef {
            id: RuleId::new(81).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 1000).unwrap(),
    }
}
fn stmt(a: u32, b: u32, pos: bool, s: Scope) -> Statement {
    let c = |n| {
        Concept::new(
            NodeId::new(n).unwrap(),
            s.catalog,
            "publico",
            "símbolo público",
        )
        .unwrap()
    };
    Statement::inclusion(
        &c(a),
        &c(b),
        if pos {
            Polarity::Positive
        } else {
            Polarity::Negative
        },
        s,
    )
    .unwrap()
}
fn info(a: u32, b: u32, pos: bool) -> Information {
    Information {
        statement: stmt(a, b, pos, scope(61)),
        declared_source: Some(SourceId::new(7).unwrap()),
        evidence: Some(EvidenceId::new(1012).unwrap()),
        nature: DeclaredNature::Observation,
        received_at: 0,
    }
}
fn apply(s: &mut Session, c: &str) {
    s.execute(c).unwrap_or_else(|e| panic!("{c}: {e}"));
}
fn base() -> Session {
    let mut s = Session::new();
    for c in [
        "config 71 61 81 1 0 1000",
        "simbolo 101 a",
        "simbolo 103 b",
        "simbolo 107 c",
        "simbolo 109 d",
        "simbolo 113 e",
        "autorizar 61",
    ] {
        apply(&mut s, c);
    }
    s
}
fn fact(s: &mut Session, id: u32, a: u32, b: u32, positive: bool) {
    apply(
        s,
        &format!(
            "receber {id} 61 {a} {b} {} 7 {} observacao",
            if positive { "+" } else { "-" },
            1000 + id
        ),
    );
    for c in ["interpretar", "avaliar", "admitir"] {
        apply(s, c);
    }
}
fn begin(rows: &[Information]) -> (Session, Investigation) {
    let mut s = base();
    fact(&mut s, 11, 101, 103, true);
    let source = LocalSource::new(SourceId::new(7).unwrap(), rows).unwrap();
    let mut r = Investigation::new(source);
    assert!(r
        .begin(&mut s, stmt(101, 107, true, scope(61)))
        .unwrap()
        .proof
        .is_none());
    (s, r)
}
fn request(s: &mut Session, r: &mut Investigation) {
    r.allow_investigation().unwrap();
    let p = r.derive(s).unwrap();
    assert_eq!(p.missing.subject().get(), 103);
    assert_eq!(p.missing.object().get(), 107);
}
fn fetch_assess(s: &mut Session, r: &mut Investigation) {
    request(s, r);
    r.allow_source(s).unwrap();
    r.fetch(s).unwrap();
    r.interpret(s).unwrap();
    r.assess(s).unwrap();
}
#[test]
fn actual_source_lookup_admission_and_proof_have_trace_without_promotion() {
    let original = info(103, 107, true);
    let (mut s, mut r) = begin(&[original]);
    let initial = s.factual_snapshot();
    fetch_assess(&mut s, &mut r);
    assert_eq!(r.source_calls(), 1);
    assert_eq!(initial, s.factual_snapshot());
    assert_eq!(r.assessments()[0].verdict, AdmissionVerdict::Admissible);
    assert!(r.assessments()[0].addresses(r.request_value().unwrap()));
    let call = r.receipt().unwrap();
    assert_eq!(call.original, Some(original));
    let SourceResult::Delivered(p) = call.result else {
        panic!("retorno ausente")
    };
    p.check_link(r.request_value().unwrap()).unwrap();
    let FeedbackContent::One(delivered) = p.content else {
        panic!()
    };
    assert_eq!(delivered.statement, original.statement);
    assert_eq!(delivered.evidence, original.evidence);
    assert!(delivered.received_at > original.received_at);
    assert!(r.admit(&mut s, EngramId::new(12).unwrap()).is_err());
    assert_eq!(initial, s.factual_snapshot());
    r.allow_admission().unwrap();
    r.admit(&mut s, EngramId::new(12).unwrap()).unwrap();
    assert_eq!(s.memory().len(), 2);
    let before = s.factual_snapshot();
    let report = r.finish(&mut s).unwrap();
    assert!(report.proof.is_some());
    assert_eq!(report.episode.before, report.episode.after);
    assert_eq!(before, s.factual_snapshot());
    assert_eq!(
        report.episode.posterior,
        EpistemicState::ProvisionalKnowledge
    );
    assert!(s
        .metadata()
        .iter()
        .all(|m| m.state == EpistemicState::AdmittedObservation));
    assert!(!s
        .metadata()
        .iter()
        .any(|m| m.information.statement.subject().get() == 101
            && m.information.statement.object().get() == 107));
    assert_eq!(r.reports().len(), 2);
}
#[test]
fn independent_permissions_and_external_prohibition_never_call_denied_source() {
    let (mut s, mut r) = begin(&[info(103, 107, true)]);
    assert!(r.derive(&mut s).is_err());
    assert!(r.fetch(&mut s).is_err());
    assert_eq!(r.source_calls(), 0);
    request(&mut s, &mut r);
    assert!(r.fetch(&mut s).is_err());
    assert_eq!(r.source_calls(), 0);
    assert!(r.external_action().is_err());
    r.allow_source(&s).unwrap();
    r.fetch(&mut s).unwrap();
    r.interpret(&mut s).unwrap();
    r.assess(&mut s).unwrap();
    assert!(r.permissions().admission.is_none());
    assert!(r.admit(&mut s, EngramId::new(12).unwrap()).is_err());
    let mut s = base();
    fact(&mut s, 11, 101, 103, true);
    let source = LocalSource::new(SourceId::new(99).unwrap(), &[info(103, 107, true)]).unwrap();
    let mut r = Investigation::new(source);
    r.begin(&mut s, stmt(101, 107, true, scope(61))).unwrap();
    request(&mut s, &mut r);
    assert!(r.allow_source(&s).is_err());
    assert!(r.fetch(&mut s).is_err());
    assert_eq!(r.source_calls(), 0);
}
#[test]
fn no_investigation_when_supported_or_resource_limited() {
    let mut s = base();
    fact(&mut s, 11, 101, 107, true);
    let mut r = Investigation::new(
        LocalSource::new(SourceId::new(7).unwrap(), &[info(103, 107, true)]).unwrap(),
    );
    assert!(r
        .begin(&mut s, stmt(101, 107, true, scope(61)))
        .unwrap()
        .proof
        .is_some());
    assert_eq!(r.stage(), Stage::Finished);
    assert!(r.allow_investigation().is_err());
    assert!(r.derive(&mut s).is_err());
    assert_eq!(r.source_calls(), 0);
    let mut s = base();
    for (i, a, b) in [(11, 101, 103), (12, 103, 101), (13, 109, 107)] {
        fact(&mut s, i, a, b, true);
    }
    let mut r = Investigation::new(LocalSource::new(SourceId::new(7).unwrap(), &[]).unwrap());
    let report = r.begin(&mut s, stmt(101, 107, true, scope(61))).unwrap();
    assert_eq!(
        report.episode.decision,
        EpisodeDecision::Abstain(Refusal::Monitor(
            neurons::metacognition::AbstainReason::ResourceLimited
        ))
    );
    assert!(
        report.review.unwrap().attempts[0]
            .inference
            .unwrap()
            .processed_events
            <= 32
    );
    assert!(r.allow_investigation().is_err());
    assert_eq!(r.source_calls(), 0);
}
#[test]
fn ambiguous_gap_and_opposed_base_do_not_choose_convenient_premise() {
    for opposed in [false, true] {
        let mut s = base();
        fact(&mut s, 11, 101, 103, true);
        if opposed {
            fact(&mut s, 14, 101, 103, false)
        } else {
            fact(&mut s, 14, 101, 109, true)
        }
        let mut r = Investigation::new(
            LocalSource::new(SourceId::new(7).unwrap(), &[info(103, 107, true)]).unwrap(),
        );
        r.begin(&mut s, stmt(101, 107, true, scope(61))).unwrap();
        r.allow_investigation().unwrap();
        let before = s.snapshot();
        assert!(r.derive(&mut s).is_err());
        assert_eq!(before, s.snapshot());
        assert_eq!(r.source_calls(), 0);
        assert!(r.history().contains("NoUniqueGap"));
    }
}
#[test]
fn absent_and_ambiguous_results_finish_readonly_without_fake_empty_admission() {
    for rows in [vec![], vec![info(103, 107, true), info(103, 107, false)]] {
        let (mut s, mut r) = begin(&rows);
        request(&mut s, &mut r);
        r.allow_source(&s).unwrap();
        let before = s.encode();
        r.fetch(&mut s).unwrap();
        assert_eq!(r.stage(), Stage::NoResult);
        assert_eq!(before, s.encode());
        let result = r.finish(&mut s).unwrap();
        assert!(result.proof.is_none());
        assert_eq!(before, s.encode());
        assert_eq!(r.source_calls(), 1);
        assert!(!s.snapshot().contains("Incorporated { ids: []"));
        assert!(r.fetch(&mut s).is_err());
    }
}
#[test]
fn refusal_of_admission_is_readonly_and_irrelevance_never_proves_query() {
    let (mut s, mut r) = begin(&[info(103, 107, true)]);
    fetch_assess(&mut s, &mut r);
    let before = s.encode();
    r.deny_admission().unwrap();
    assert!(r.allow_admission().is_err());
    assert!(r.admit(&mut s, EngramId::new(12).unwrap()).is_err());
    assert!(r.finish(&mut s).unwrap().proof.is_none());
    assert_eq!(before, s.encode());
    let (mut s, mut r) = begin(&[info(103, 109, true)]);
    fetch_assess(&mut s, &mut r);
    assert_eq!(r.assessments()[0].verdict, AdmissionVerdict::Admissible);
    assert!(!r.assessments()[0].addresses(r.request_value().unwrap()));
    r.allow_admission().unwrap();
    r.admit(&mut s, EngramId::new(12).unwrap()).unwrap();
    assert!(r.finish(&mut s).unwrap().proof.is_none());
    assert_eq!(s.memory().len(), 2);
}
#[test]
fn invalid_context_evidence_hypothesis_duplicate_and_source_are_not_admitted() {
    for mode in 0..6 {
        let mut item = info(103, 107, true);
        match mode {
            0 => item.statement = stmt(103, 107, true, scope(62)),
            1 => item.evidence = None,
            2 => item.nature = DeclaredNature::Hypothesis,
            3 => item.evidence = Some(EvidenceId::new(1011).unwrap()),
            4 => item.declared_source = Some(SourceId::new(99).unwrap()),
            _ => {
                item.statement = stmt(
                    103,
                    107,
                    true,
                    Scope {
                        rule: RuleRef {
                            id: RuleId::new(81).unwrap(),
                            version: RuleVersion::new(2).unwrap(),
                        },
                        ..scope(61)
                    },
                )
            }
        }
        let (mut s, mut r) = begin(&[item]);
        let before = s.factual_snapshot();
        request(&mut s, &mut r);
        r.allow_source(&s).unwrap();
        let fetched = r.fetch(&mut s);
        if r.stage() == Stage::Received {
            assert!(fetched.is_ok());
            r.interpret(&mut s).unwrap();
            r.assess(&mut s).unwrap();
            assert!(matches!(
                r.assessments()[0].verdict,
                AdmissionVerdict::Quarantined(_)
            ));
            r.allow_admission().unwrap();
            assert!(r.admit(&mut s, EngramId::new(12).unwrap()).is_err());
        } else {
            assert_eq!(r.stage(), Stage::NoResult);
        }
        assert!(r.finish(&mut s).unwrap().proof.is_none());
        assert_eq!(before, s.factual_snapshot());
        assert_eq!(r.source_calls(), 1);
    }
}
#[test]
fn contradiction_is_visible_before_and_after_explicit_admission() {
    let mut s = base();
    fact(&mut s, 11, 101, 103, true);
    fact(&mut s, 14, 103, 107, false);
    let mut r = Investigation::new(
        LocalSource::new(SourceId::new(7).unwrap(), &[info(103, 107, true)]).unwrap(),
    );
    r.begin(&mut s, stmt(101, 107, true, scope(61))).unwrap();
    fetch_assess(&mut s, &mut r);
    assert_eq!(r.opposition_records().len(), 1);
    r.allow_admission().unwrap();
    r.admit(&mut s, EngramId::new(12).unwrap()).unwrap();
    let before = s.factual_snapshot();
    let result = r.finish(&mut s).unwrap();
    assert!(result.proof.is_none());
    assert_eq!(
        result.episode.decision,
        EpisodeDecision::Abstain(Refusal::Monitor(
            neurons::metacognition::AbstainReason::ContradictoryEvidence
        ))
    );
    assert_eq!(before, s.factual_snapshot());
    assert_eq!(s.memory().len(), 3);
}
#[test]
fn budgets_are_fixed_and_eight_fact_capacity_is_preserved() {
    let (mut s, mut r) = begin(&[info(103, 107, true)]);
    fetch_assess(&mut s, &mut r);
    assert!(r.fetch(&mut s).is_err());
    assert!(r.derive(&mut s).is_err());
    r.allow_admission().unwrap();
    r.admit(&mut s, EngramId::new(12).unwrap()).unwrap();
    assert!(r.admit(&mut s, EngramId::new(13).unwrap()).is_err());
    r.finish(&mut s).unwrap();
    assert!(r.finish(&mut s).is_err());
    assert_eq!(r.source_calls(), 1);
    assert!(r.reports().len() <= 3);
    let mut s = base();
    for (i, a, b, pos) in [
        (11, 101, 103, true),
        (14, 103, 107, false),
        (15, 113, 109, true),
        (16, 109, 113, true),
        (17, 113, 101, true),
        (18, 109, 103, true),
        (19, 109, 107, false),
        (20, 113, 107, false),
    ] {
        fact(&mut s, i, a, b, pos);
    }
    let mut r = Investigation::new(
        LocalSource::new(SourceId::new(7).unwrap(), &[info(103, 107, true)]).unwrap(),
    );
    r.begin(&mut s, stmt(101, 107, true, scope(61))).unwrap();
    fetch_assess(&mut s, &mut r);
    r.allow_admission().unwrap();
    let before = s.encode();
    assert!(r.admit(&mut s, EngramId::new(12).unwrap()).is_err());
    assert_eq!(before, s.encode());
    assert_eq!(s.memory().len(), 8);
    assert!(r.finish(&mut s).unwrap().proof.is_none());
}
#[test]
fn independent_reference_only_receives_delivered_evidence() {
    for rows in [
        vec![info(103, 107, true)],
        vec![info(103, 109, true)],
        vec![],
    ] {
        let (mut s, mut r) = begin(&rows);
        request(&mut s, &mut r);
        r.allow_source(&s).unwrap();
        r.fetch(&mut s).unwrap();
        let mut delivered = vec![(101u32, 103u32)];
        if let Some(SourceCall {
            result: SourceResult::Delivered(p),
            ..
        }) = r.receipt()
        {
            if let FeedbackContent::One(i) = p.content {
                delivered.push((i.statement.subject().get(), i.statement.object().get()));
            }
            r.interpret(&mut s).unwrap();
            r.assess(&mut s).unwrap();
            r.allow_admission().unwrap();
            r.admit(&mut s, EngramId::new(12).unwrap()).unwrap();
        }
        let expected = delivered
            .iter()
            .any(|(a, b)| *a == 101 && delivered.iter().any(|(c, d)| b == c && *d == 107));
        assert_eq!(r.finish(&mut s).unwrap().proof.is_some(), expected);
    }
}

#[test]
fn materialized_f1_checkpoint_does_not_resume_permissions_or_reconsult_source() {
    let (mut s, mut r) = begin(&[info(103, 107, true)]);
    fetch_assess(&mut s, &mut r);
    r.allow_admission().unwrap();
    r.admit(&mut s, EngramId::new(12).unwrap()).unwrap();
    r.finish(&mut s).unwrap();
    let bytes = s.encode();
    let mut restored = Session::decode(&bytes).unwrap();
    assert_eq!(s.snapshot(), restored.snapshot());
    assert_eq!(r.source_calls(), 1);
    let mut other = Investigation::new(
        LocalSource::new(SourceId::new(7).unwrap(), &[info(103, 107, true)]).unwrap(),
    );
    assert!(other.fetch(&mut restored).is_err());
    assert!(other.allow_investigation().is_err());
    assert_eq!(other.source_calls(), 0);
    assert!(other
        .begin(&mut restored, stmt(101, 107, true, scope(61)))
        .is_err());
    assert_eq!(other.source_calls(), 0);
    assert_eq!(restored.encode(), bytes);
    assert!(r.processed_events() <= 96);
}
#[test]
fn repeated_episodes_are_isolated_with_fixed_inputs() {
    fn run() -> (String, Vec<u8>) {
        let (mut s, mut r) = begin(&[info(103, 107, true)]);
        fetch_assess(&mut s, &mut r);
        r.allow_admission().unwrap();
        r.admit(&mut s, EngramId::new(12).unwrap()).unwrap();
        r.finish(&mut s).unwrap();
        (r.history(), s.encode())
    }
    assert_eq!(run(), run());
    let (s, r) = begin(&[]);
    assert_eq!(s.memory().len(), 1);
    assert_eq!(r.source_calls(), 0);
    assert!(LocalSource::new(SourceId::new(7).unwrap(), &[info(103, 107, true); 9]).is_err());
}
#[test]
fn cli_runs_source_and_blocks_admission_bypass() {
    use neurons::personality::Style;
    use neurons::session::f1::run;
    use std::io::Cursor;
    let input="config 71 61 81 1 0 1000\nsimbolo 101 a\nsimbolo 103 b\nsimbolo 107 c\nautorizar 61\nreceber 11 61 101 103 + 7 1011 observacao\ninterpretar\navaliar\nadmitir\nfonte7 7\nevidencia7 61 103 107 + 1012 observacao\nfixar-fonte7\ninvestigar7 101 107\npermitir-investigacao7\npedido7\npermitir-fonte7\nconsultar-fonte7\ninterpretar7\navaliar7\nadmitir7 12\nadmitir-retorno 12\npermitir-admissao7\nadmitir7 12\nconcluir7\nhistorico7\nsair\n";
    let mut output = vec![];
    run(Cursor::new(input), &mut output).unwrap();
    let text = String::from_utf8(output).unwrap();
    assert_eq!(text.matches("Erro:").count(), 2);
    assert!(text.contains("MP07 final"));
    assert!(text.contains("TwoHop"));
    assert!(text.contains("source_calls=1"));
    assert!(text.contains("incorporação não autorizada"));
    assert!(text.contains("comandos paralelos bloqueados"));
    // Controle de API legada: sessão anterior continua disponível com o mesmo núcleo.
    let mut output = vec![];
    neurons::session::run(
        Cursor::new(b"consulta 61 101 107\nsair\n"),
        &mut output,
        Style::Concise,
    )
    .unwrap();
    assert!(String::from_utf8(output)
        .unwrap()
        .contains("MissingEvidence"));
}
#[test]
fn incompatible_catalog_is_preserved_as_refusal_not_reinterpreted() {
    let original = Information {
        statement: stmt(
            103,
            107,
            true,
            Scope {
                catalog: CatalogId::new(72).unwrap(),
                ..scope(61)
            },
        ),
        ..info(103, 107, true)
    };
    let (mut s, mut r) = begin(&[original]);
    request(&mut s, &mut r);
    r.allow_source(&s).unwrap();
    let before = s.encode();
    assert!(r.fetch(&mut s).is_err());
    assert_eq!(before, s.encode());
    assert_eq!(r.receipt().unwrap().original, Some(original));
    assert_eq!(r.source_calls(), 1);
    assert!(matches!(
        r.receipt().unwrap().result,
        SourceResult::HostRefused(_, _)
    ));
    assert!(r.finish(&mut s).unwrap().proof.is_none());
}

#[test]
fn temporal_provenance_is_checked_before_materializing_reception() {
    for mode in 0..3 {
        let mut item = info(103, 107, true);
        match mode {
            0 => item.received_at = 2000,
            1 => {
                item.statement = stmt(
                    103,
                    107,
                    true,
                    Scope {
                        validity: Validity::new(0, 1).unwrap(),
                        ..scope(61)
                    },
                )
            }
            _ => {
                item.statement = stmt(
                    103,
                    107,
                    true,
                    Scope {
                        validity: Validity::new(100, 1000).unwrap(),
                        ..scope(61)
                    },
                )
            }
        }
        let (mut s, mut r) = begin(&[item]);
        request(&mut s, &mut r);
        r.allow_source(&s).unwrap();
        let before = s.encode();
        r.fetch(&mut s).unwrap();
        assert_eq!(r.stage(), Stage::NoResult);
        assert!(matches!(
            r.receipt().unwrap().result,
            SourceResult::Rejected(_, ContractError::OutsideValidity)
        ));
        assert_eq!(r.receipt().unwrap().original, Some(item));
        assert_eq!(before, s.encode());
        assert!(r.finish(&mut s).unwrap().proof.is_none());
        assert_eq!(before, s.encode());
        assert_eq!(r.source_calls(), 1);
    }
}
