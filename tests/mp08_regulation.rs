//! Cenários sintéticos públicos MP08, independentes das partições científicas.
use neurons::cognitive_contracts::*;
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::NodeId;
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

use neurons::metacognition::{select_strategy, GoalResult, RegulationDiagnostic as D, Strategy};
fn grants() -> RegulationPermissions {
    RegulationPermissions {
        evaluation: Some(scope(61)),
        investigation: Some(scope(61)),
        source: Some((scope(61), SourceId::new(7).unwrap())),
        admission: Some((scope(61), EngramId::new(12).unwrap())),
        mp06: None,
    }
}
fn run(
    s: &mut Session,
    rows: &[Information],
    p: RegulationPermissions,
    b: RegulationBudget,
) -> RegulationReport {
    regulate(
        s,
        LocalSource::new(SourceId::new(7).unwrap(), rows).unwrap(),
        stmt(101, 107, true, scope(61)),
        p,
        b,
    )
    .unwrap()
}
fn seeded() -> Session {
    let mut s = base();
    fact(&mut s, 11, 101, 103, true);
    s
}
#[test]
fn supported_answers_without_source_or_mutation() {
    let mut s = base();
    fact(&mut s, 11, 101, 107, true);
    let before = s.factual_snapshot();
    let r = run(&mut s, &[], grants(), RegulationBudget::default());
    assert_eq!(r.strategy, Strategy::Respond);
    assert_eq!(r.goal, GoalResult::Met);
    assert_eq!(r.source_calls, 0);
    assert_eq!(r.queries, 1);
    assert_eq!(before, s.factual_snapshot());
}
#[test]
fn three_premises_require_specific_authorization() {
    for allowed in [false, true] {
        let mut s = base();
        fact(&mut s, 11, 101, 103, true);
        fact(&mut s, 13, 103, 109, true);
        fact(&mut s, 14, 109, 107, true);
        let mut p = grants();
        p.investigation = None;
        p.mp06 = allowed
            .then_some(neurons::integration::query::PathAuthorization::inclusion_three(scope(61)));
        let before = s.factual_snapshot();
        let r = run(&mut s, &[], p, RegulationBudget::default());
        assert_eq!(r.goal == GoalResult::Met, allowed);
        assert_eq!(r.source_calls, 0);
        assert_eq!(before, s.factual_snapshot());
        assert_eq!(r.queries, if allowed { 2 } else { 1 });
    }
}
#[test]
fn investigation_admits_only_explicitly_and_meets_goal_with_proof() {
    let mut s = seeded();
    let r = run(
        &mut s,
        &[info(103, 107, true)],
        grants(),
        RegulationBudget::default(),
    );
    assert_eq!(r.strategy, Strategy::Investigate);
    assert_eq!(r.goal, GoalResult::Met);
    assert_eq!(r.source_calls, 1);
    assert!(r.admitted);
    assert_eq!(r.evaluations, 2);
    assert_eq!(s.memory().len(), 2);
    assert!(r
        .evidence
        .last()
        .is_some_and(|e| matches!(e,RegulationEvidence::Query(q) if q.proof.is_some())));
    assert!(!s
        .metadata()
        .iter()
        .any(|m| m.information.statement.subject().get() == 101
            && m.information.statement.object().get() == 107));
}
#[test]
fn missing_permissions_never_execute_source_or_admit() {
    for mode in 0..4 {
        let mut s = seeded();
        let before = s.factual_snapshot();
        let mut p = grants();
        match mode {
            0 => p.evaluation = None,
            1 => p.investigation = None,
            2 => p.source = None,
            _ => p.source = Some((scope(62), SourceId::new(7).unwrap())),
        };
        let r = run(
            &mut s,
            &[info(103, 107, true)],
            p,
            RegulationBudget::default(),
        );
        assert_eq!(r.strategy, Strategy::Abstain);
        assert_eq!(r.source_calls, 0);
        assert!(!r.admitted);
        assert_eq!(before, s.factual_snapshot());
        assert_eq!(r.queries, if mode == 0 { 0 } else { 1 });
    }
}
#[test]
fn no_admission_grant_preserves_facts_and_partial_is_not_success() {
    let mut s = seeded();
    let before = s.factual_snapshot();
    let mut p = grants();
    p.admission = None;
    let r = run(
        &mut s,
        &[info(103, 107, true)],
        p,
        RegulationBudget::default(),
    );
    assert_eq!(r.goal, GoalResult::Partial);
    assert_eq!(r.source_calls, 1);
    assert!(!r.admitted);
    assert_eq!(before, s.factual_snapshot());
}
#[test]
fn ambiguous_gap_never_investigates() {
    let mut s = seeded();
    fact(&mut s, 14, 101, 109, true);
    let before = s.factual_snapshot();
    let r = run(
        &mut s,
        &[info(103, 107, true)],
        grants(),
        RegulationBudget::default(),
    );
    assert_eq!(r.strategy, Strategy::Abstain);
    assert!(r.diagnostics.contains(&D::GapUnavailable));
    assert_eq!(r.source_calls, 0);
    assert_eq!(before, s.factual_snapshot());
}
#[test]
fn opposition_never_hidden_by_strategy() {
    let mut s = base();
    fact(&mut s, 11, 101, 107, true);
    fact(&mut s, 14, 101, 107, false);
    let before = s.factual_snapshot();
    let r = run(&mut s, &[], grants(), RegulationBudget::default());
    assert_eq!(r.strategy, Strategy::Abstain);
    assert!(r.diagnostics.contains(&D::Opposition));
    assert_eq!(r.goal, GoalResult::NotMet);
    assert_eq!(r.source_calls, 0);
    assert_eq!(before, s.factual_snapshot());
}
#[test]
fn contract_impediment_is_not_missing_knowledge() {
    let mut s = seeded();
    let r = regulate(
        &mut s,
        LocalSource::new(SourceId::new(7).unwrap(), &[]).unwrap(),
        stmt(101, 107, false, scope(61)),
        grants(),
        RegulationBudget::default(),
    )
    .unwrap();
    assert!(r.diagnostics.contains(&D::ContractBlocked));
    assert_eq!(r.strategy, Strategy::Abstain);
    assert_eq!(r.source_calls, 0);
}
#[test]
fn source_absence_irrelevance_invalidity_and_contradiction_do_not_fake_success() {
    for mode in 0..5 {
        let mut s = seeded();
        if mode == 4 {
            fact(&mut s, 14, 103, 107, false);
        }
        let rows = match mode {
            0 => vec![],
            1 => vec![info(103, 109, true)],
            2 => vec![Information {
                evidence: None,
                ..info(103, 107, true)
            }],
            3 => vec![info(103, 107, false)],
            _ => vec![info(103, 107, true)],
        };
        let r = run(&mut s, &rows, grants(), RegulationBudget::default());
        assert_ne!(r.goal, GoalResult::Met);
        assert_eq!(r.source_calls, 1);
        assert!(r.queries <= 3 && r.events <= 96 && r.history.len() <= 8);
        if mode == 4 {
            assert!(r.diagnostics.contains(&D::Opposition));
        }
    }
}
#[test]
fn budgets_stop_before_execution_and_never_reset() {
    for b in [
        RegulationBudget {
            queries: 0,
            events: 96,
        },
        RegulationBudget {
            queries: 3,
            events: 0,
        },
        RegulationBudget {
            queries: 1,
            events: 96,
        },
        RegulationBudget {
            queries: 3,
            events: 31,
        },
    ] {
        let mut s = seeded();
        let r = run(&mut s, &[info(103, 107, true)], grants(), b);
        assert_eq!(r.strategy, Strategy::StopAtLimit);
        assert!(r.reservation_blocked);
        assert_eq!(r.source_calls, 0);
        assert!(r.events <= b.events && r.queries <= b.queries);
    }
    let mut s = seeded();
    fact(&mut s, 14, 103, 101, true);
    fact(&mut s, 15, 109, 107, true);
    let r = run(&mut s, &[], grants(), RegulationBudget::default());
    assert_eq!(r.strategy, Strategy::StopAtLimit);
    assert!(r.diagnostics.contains(&D::ResourceLimited));
}
#[test]
fn policy_precedence_preserves_multiple_diagnostics() {
    assert_eq!(
        select_strategy(
            &[D::Supported, D::Opposition, D::ResourceLimited],
            true,
            true,
            true
        ),
        Strategy::Abstain
    );
    assert_eq!(
        select_strategy(&[D::Supported, D::ResourceLimited], true, true, true),
        Strategy::StopAtLimit
    );
    assert_eq!(
        select_strategy(&[D::MissingEvidence], false, true, false),
        Strategy::Abstain
    );
    assert_eq!(
        select_strategy(&[D::InvalidEvidence], false, true, true),
        Strategy::Abstain
    );
}
#[test]
fn independent_symbolic_policy_matches_observed_cases() {
    let mut mismatches = 0;
    for mode in 0..5 {
        let mut s = seeded();
        let mut p = grants();
        if mode == 0 {
            fact(&mut s, 13, 103, 107, true);
        }
        if mode == 2 {
            p.source = None;
        }
        if mode == 3 {
            fact(&mut s, 14, 101, 109, true);
        }
        let b = if mode == 4 {
            RegulationBudget {
                queries: 0,
                events: 96,
            }
        } else {
            RegulationBudget::default()
        };
        // Référence recebe os mesmos fatos admitidos, permissões e orçamento; não enumera a fonte.
        let facts = s
            .metadata()
            .iter()
            .map(|m| m.information.statement)
            .collect::<Vec<_>>();
        let q = stmt(101, 107, true, scope(61));
        let support = facts.iter().any(|a| {
            a.subject() == q.subject()
                && facts
                    .iter()
                    .any(|c| a.object() == c.subject() && c.object() == q.object())
        });
        let bases = facts.iter().filter(|a| a.subject() == q.subject()).count();
        let expected = if b.queries == 0 {
            Strategy::StopAtLimit
        } else if support {
            Strategy::Respond
        } else if bases == 1 && p.source.is_some() {
            Strategy::Investigate
        } else {
            Strategy::Abstain
        };
        let r = run(&mut s, &[], p, b);
        mismatches += usize::from(r.strategy != expected);
    }
    assert_eq!(mismatches, 0, "falsos alarmes/falhas na matriz pública");
}
#[test]
fn determinism_and_checkpoint_materialization_do_not_resume_supervision() {
    let execute = || {
        let mut s = seeded();
        let r = run(
            &mut s,
            &[info(103, 107, true)],
            grants(),
            RegulationBudget::default(),
        );
        let bytes = s.encode();
        let restored = Session::decode(&bytes).unwrap();
        assert_eq!(restored.encode(), bytes);
        (r, bytes)
    };
    assert_eq!(execute(), execute());
}
#[test]
fn cli_permissions_and_single_execution_are_transient() {
    use std::io::Cursor;
    let input="config 71 61 81 1 0 1000\nsimbolo 101 a\nsimbolo 103 b\nsimbolo 107 c\nautorizar 61\nreceber 11 61 101 103 + 7 1011 observacao\ninterpretar\navaliar\nadmitir\nfonte7 7\nevidencia7 61 103 107 + 1012 observacao\nfixar-fonte7\npermissoes8 61 sim nao sim 7 12\nregular8 101 107 3 96\nregular8 101 107 3 96\nhistorico8\nsair\n";
    let mut out = vec![];
    neurons::session::f1::run(Cursor::new(input), &mut out).unwrap();
    let text = String::from_utf8(out).unwrap();
    assert!(text.contains("goal: Met"));
    assert_eq!(text.matches("Erro:").count(), 1);
    assert!(text.contains("um episódio supervisor"));
}

#[test]
fn aggregate_queries_include_mp06_and_do_not_restart_at_investigation() {
    for limit in [2, 3] {
        let mut s = seeded();
        let mut p = grants();
        p.mp06 = Some(neurons::integration::query::PathAuthorization::inclusion_three(scope(61)));
        let r = run(
            &mut s,
            &[],
            p,
            RegulationBudget {
                queries: limit,
                events: 96,
            },
        );
        assert_eq!(r.queries, limit);
        assert_eq!(r.source_calls, usize::from(limit == 3));
        assert_eq!(
            r.strategy,
            if limit == 3 {
                Strategy::Investigate
            } else {
                Strategy::StopAtLimit
            }
        );
        assert!(r.events <= 96 && r.evaluations <= 2 && r.history.len() <= 8);
        assert_eq!(r.permissions, p);
    }
}
#[test]
fn restore_clears_supervisor_permissions_history_and_never_requeries_source() {
    use std::io::Cursor;
    let mut s = seeded();
    let r = run(
        &mut s,
        &[info(103, 107, true)],
        grants(),
        RegulationBudget::default(),
    );
    assert_eq!(r.goal, GoalResult::Met);
    let dir = std::env::temp_dir().join(format!(
        "neurons-mp08-restore-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&dir).unwrap();
    let file = dir.join("public.f1");
    s.save(&file).unwrap();
    let input = format!(
        "restaurar {}\nhistorico8\nregular8 101 107 3 96\nconsultar-fonte7\nestado\nsair\n",
        file.display()
    );
    let mut output = vec![];
    neurons::session::f1::run(Cursor::new(input), &mut output).unwrap();
    let text = String::from_utf8(output).unwrap();
    assert_eq!(text.matches("Erro:").count(), 3);
    assert!(
        text.contains("sem relatório MP08")
            && text.contains("declare permissoes8")
            && text.contains("sem investigação")
    );
    assert_eq!(Session::load(&file).unwrap().encode(), s.encode());
}
#[test]
fn invalid_budget_is_rejected_before_any_host_mutation() {
    for budget in [
        RegulationBudget {
            queries: 4,
            events: 96,
        },
        RegulationBudget {
            queries: 3,
            events: 97,
        },
    ] {
        let mut s = seeded();
        let before = s.encode();
        assert!(regulate(
            &mut s,
            LocalSource::new(SourceId::new(7).unwrap(), &[]).unwrap(),
            stmt(101, 107, true, scope(61)),
            grants(),
            budget
        )
        .is_err());
        assert_eq!(before, s.encode());
    }
}

#[test]
fn final_goal_matches_symbolic_reference_using_only_admitted_delivered_facts() {
    let mut false_successes = 0;
    let mut missed_successes = 0;
    for mode in 0..6 {
        let mut s = seeded();
        let mut p = grants();
        if mode == 4 {
            fact(&mut s, 14, 103, 107, false);
        }
        if mode == 5 {
            p.admission = None;
        }
        let rows = match mode {
            0 | 4 | 5 => vec![info(103, 107, true)],
            1 => vec![],
            2 => vec![info(103, 109, true)],
            _ => vec![info(103, 107, false)],
        };
        let r = run(&mut s, &rows, p, RegulationBudget::default());
        // Não lê rows/catálogo da fonte: somente memória efetivamente admitida no episódio.
        let facts = s
            .metadata()
            .iter()
            .map(|m| m.information.statement)
            .collect::<Vec<_>>();
        let opposition = facts.iter().any(|n| {
            n.polarity() == Polarity::Negative
                && ((n.subject().get() == 101 && n.object().get() == 107)
                    || (n.subject().get() == 101 && n.object().get() == 103)
                    || (n.subject().get() == 103 && n.object().get() == 107))
        });
        let supported = facts.iter().any(|a| {
            a.polarity() == Polarity::Positive
                && a.subject().get() == 101
                && facts.iter().any(|b| {
                    b.polarity() == Polarity::Positive
                        && b.subject() == a.object()
                        && b.object().get() == 107
                })
        });
        let expected = supported && !opposition;
        false_successes += usize::from(r.goal == GoalResult::Met && !expected);
        missed_successes += usize::from(r.goal != GoalResult::Met && expected);
    }
    assert_eq!((false_successes, missed_successes), (0, 0));
}
