//! Integração pública de objetivo conceitual; sem fixtures científicas.
use neurons::cognition::concepts::{Attributes, Label, Observation};
use neurons::cognitive_contracts::*;
use neurons::curriculum::*;
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::NodeId;
use neurons::metacognition::concepts::{Abstention, ConceptDecision, ConceptStrategy};
use neurons::session::f1::Session;
fn id(n: u32) -> EngramId {
    EngramId::new(n).unwrap()
}
fn scope() -> Scope {
    Scope {
        catalog: CatalogId::new(91).unwrap(),
        context: ContextId::new(41).unwrap(),
        rule: RuleRef {
            id: RuleId::new(51).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 10000).unwrap(),
    }
}
fn attrs(s: &str) -> Attributes {
    Attributes::new(
        s.as_bytes()
            .try_into()
            .map(|a: [u8; 6]| a.map(|x| x - b'0'))
            .unwrap(),
    )
    .unwrap()
}
fn obs(s: &str) -> Observation {
    Observation::new(
        s.as_bytes()
            .try_into()
            .map(|a: [u8; 6]| a.map(|x| if x == b'?' { None } else { Some(x - b'0') }))
            .unwrap(),
    )
    .unwrap()
}
fn apply(s: &mut Session, c: &str) {
    s.execute(c).unwrap_or_else(|e| panic!("{c}: {e}"));
}
fn base() -> Session {
    let mut s = Session::new();
    for c in [
        "config 91 41 51 1 0 10000",
        "autorizar 41",
        "conceito 501 opaco",
        "exemplo 401 901 41 100001 + 7 2401 observacao",
        "interpretar-exemplo",
        "avaliar-exemplo",
        "admitir-exemplo 401",
        "revisar",
    ] {
        apply(&mut s, c);
    }
    s
}
fn lesson(n: u32, bits: &str, label: Label) -> Lesson {
    Lesson {
        id: id(n),
        receipt: n,
        exemplar: NodeId::new(n + 500).unwrap(),
        attributes: attrs(bits),
        label,
        source: SourceId::new(7).unwrap(),
        evidence: EvidenceId::new(n + 2000).unwrap(),
    }
}
fn lessons() -> Vec<Lesson> {
    vec![
        lesson(402, "000001", Label::Negative),
        lesson(403, "100000", Label::Negative),
    ]
}
fn plan(rows: &[Lesson]) -> TeachingPlan {
    TeachingPlan::new(NodeId::new(501).unwrap(), scope(), rows).unwrap()
}
fn assessment() -> PedagogyAssessment {
    PedagogyAssessment::new(&[obs("110001")], &[obs("010101")]).unwrap()
}
fn permissions() -> PedagogyPermissions {
    PedagogyPermissions {
        evaluation: Some(scope()),
        presentation: Some(scope()),
        admissions: vec![id(402), id(403)],
    }
}

fn goal() -> ConceptGoal {
    ConceptGoal {
        id: EpisodeId::new(9001).unwrap(),
        concept: NodeId::new(501).unwrap(),
        scope: scope(),
        observation: obs("100010"),
        criterion: GoalCriterion::SupportedByCurrentEvidence,
    }
}
fn run(
    s: &mut Session,
    rows: &[Lesson],
    p: PedagogyPermissions,
    b: ConceptGoalBudget,
) -> ConceptGoalReport {
    supervise_concept_goal(s, goal(), plan(rows), assessment(), p, b).unwrap()
}
#[test]
fn identified_goal_is_reconsulted_after_explained_revisions() {
    let mut s = base();
    let original = s.concepts().unwrap().models()[0].versions().to_vec();
    let r = run(
        &mut s,
        &lessons(),
        permissions(),
        ConceptGoalBudget::default(),
    );
    assert_eq!(r.goal, goal());
    assert_eq!(r.strategy, ConceptStrategy::TeachOnce);
    assert_eq!(r.outcome, ConceptGoalResult::Resolved);
    assert_eq!(r.execution, ConceptExecution::Completed);
    let first = &r.initial.as_ref().unwrap().report;
    let last = &r.final_query.as_ref().unwrap().report;
    assert_eq!(first.observation, last.observation);
    assert_eq!(first.observation, goal().observation);
    assert_eq!(
        first.decision,
        ConceptDecision::Abstain(Abstention::InsufficientEvidence)
    );
    assert_eq!(last.decision, ConceptDecision::Supported(Label::Negative));
    assert_eq!(first.version.as_ref().unwrap().number, 1);
    assert_eq!(last.version.as_ref().unwrap().number, 3);
    assert_eq!(first.consulted.len(), 1);
    assert_eq!(last.consulted.len(), 3);
    assert!(r
        .initial
        .iter()
        .chain(&r.final_query)
        .chain(&r.independent_before)
        .chain(&r.independent_after)
        .all(|q| q.goal == goal()));
    assert!(r.lessons.iter().all(|l| l.goal == goal()));
    assert_eq!(r.queries, 4);
    assert_eq!(r.concept_events, 10);
    assert!(r.history.len() <= 8);
    assert_eq!(
        &s.concepts().unwrap().models()[0].versions()[..original.len()],
        &original
    );
    assert!(first.at < r.lessons[0].record.revision.as_ref().unwrap().at);
    assert_eq!(
        last.version
            .as_ref()
            .unwrap()
            .support
            .iter()
            .map(|e| e.evidence)
            .collect::<Vec<_>>(),
        vec![
            EvidenceId::new(2401).unwrap(),
            EvidenceId::new(2402).unwrap(),
            EvidenceId::new(2403).unwrap()
        ]
    );
    assert_eq!(s.memory().len(), 0);
}
#[test]
fn sufficient_goal_answers_without_teaching_or_duplicate_query() {
    let mut s = base();
    let before = s.encode();
    let g = ConceptGoal {
        observation: obs("110001"),
        ..goal()
    };
    let a = PedagogyAssessment::new(&[obs("100011")], &[obs("010101")]).unwrap();
    let r = supervise_concept_goal(
        &mut s,
        g,
        plan(&lessons()),
        a,
        permissions(),
        ConceptGoalBudget::default(),
    )
    .unwrap();
    assert_eq!(r.strategy, ConceptStrategy::Respond);
    assert_eq!(r.outcome, ConceptGoalResult::Resolved);
    assert_eq!(r.queries, 1);
    assert!(r.final_query.is_none() && r.lessons.is_empty());
    assert_eq!(before, s.encode());
}
#[test]
fn permissions_and_control_never_incorporate_implicitly() {
    for mode in 0..3 {
        let mut s = base();
        let before = s.encode();
        let mut p = permissions();
        match mode {
            0 => p.evaluation = None,
            1 => p.presentation = None,
            _ => p.admissions.clear(),
        };
        let r = run(&mut s, &lessons(), p, ConceptGoalBudget::default());
        assert_eq!(r.outcome, ConceptGoalResult::Interrupted);
        assert!(r.lessons.is_empty());
        assert_eq!(r.concept_events, 0);
        assert_eq!(before, s.encode());
        assert_eq!(r.queries, if mode == 0 { 0 } else { 1 });
    }
}
#[test]
fn missing_second_admission_does_not_authorize_it_by_feedback() {
    let mut s = base();
    let mut p = permissions();
    p.admissions = vec![id(402)];
    let r = run(&mut s, &lessons(), p, ConceptGoalBudget::default());
    assert_eq!(r.outcome, ConceptGoalResult::Interrupted);
    assert!(r.lessons[0].record.admitted);
    assert!(!r.lessons[1].record.admitted);
    assert_eq!(r.execution, ConceptExecution::Interrupted);
    assert_eq!(s.concepts().unwrap().memory().len(), 2);
    assert_eq!(s.concepts().unwrap().models()[0].current().number, 2);
}
#[test]
fn partial_progress_is_not_resolution() {
    let mut s = base();
    let r = run(
        &mut s,
        &lessons()[..1],
        permissions(),
        ConceptGoalBudget::default(),
    );
    assert_eq!(r.outcome, ConceptGoalResult::Partial);
    assert_eq!(r.execution, ConceptExecution::Completed);
    assert_eq!(
        r.final_query.unwrap().report.decision,
        ConceptDecision::Abstain(Abstention::InsufficientEvidence)
    );
    let mut s = base();
    let r = run(&mut s, &[], permissions(), ConceptGoalBudget::default());
    assert_eq!(r.outcome, ConceptGoalResult::Abstention);
    assert!(r.lessons.is_empty());
}
#[test]
fn unknown_source_is_quarantined_not_promoted() {
    let mut s = base();
    let before = s.concepts().unwrap().models().to_vec();
    let mut l = lessons()[0];
    l.source = SourceId::new(99).unwrap();
    let r = run(&mut s, &[l], permissions(), ConceptGoalBudget::default());
    assert_eq!(r.outcome, ConceptGoalResult::Abstention);
    assert!(matches!(
        r.lessons[0].record.admissibility,
        neurons::integration::concepts::Admission::Quarantined(_)
    ));
    assert!(!r.lessons[0].record.admitted);
    assert_eq!(s.concepts().unwrap().models(), before);
}
#[test]
fn incompatible_context_and_budget_inputs_fail_before_mutation() {
    let mut s = base();
    let before = s.encode();
    let g = ConceptGoal {
        scope: Scope {
            context: ContextId::new(42).unwrap(),
            ..scope()
        },
        ..goal()
    };
    assert!(supervise_concept_goal(
        &mut s,
        g,
        plan(&lessons()),
        assessment(),
        permissions(),
        ConceptGoalBudget::default()
    )
    .is_err());
    assert_eq!(before, s.encode());
    for b in [
        ConceptGoalBudget {
            queries: 6,
            experiences: 2,
        },
        ConceptGoalBudget {
            queries: 5,
            experiences: 3,
        },
    ] {
        assert!(supervise_concept_goal(
            &mut s,
            goal(),
            plan(&lessons()),
            assessment(),
            permissions(),
            b
        )
        .is_err());
        assert_eq!(before, s.encode());
    }
}
#[test]
fn contradiction_prevents_teaching_and_remains_visible() {
    let mut s = base();
    for c in [
        "exemplo 450 950 41 100001 - 7 2450 observacao",
        "interpretar-exemplo",
        "avaliar-exemplo",
        "admitir-exemplo 450",
        "revisar",
    ] {
        apply(&mut s, c);
    }
    let before = s.encode();
    let r = run(
        &mut s,
        &lessons(),
        permissions(),
        ConceptGoalBudget::default(),
    );
    assert_eq!(r.outcome, ConceptGoalResult::Abstention);
    assert_eq!(r.strategy, ConceptStrategy::Abstain);
    assert_eq!(
        r.initial.unwrap().report.decision,
        ConceptDecision::Abstain(Abstention::ContradictoryEvidence)
    );
    assert!(r.lessons.is_empty());
    assert_eq!(s.encode(), before);
}
#[test]
fn aggregated_budget_cannot_be_refilled_between_phases() {
    for b in [
        ConceptGoalBudget {
            queries: 0,
            experiences: 2,
        },
        ConceptGoalBudget {
            queries: 1,
            experiences: 2,
        },
        ConceptGoalBudget {
            queries: 3,
            experiences: 2,
        },
        ConceptGoalBudget {
            queries: 5,
            experiences: 1,
        },
    ] {
        let mut s = base();
        let before = s.encode();
        let r = run(&mut s, &lessons(), permissions(), b);
        assert_eq!(r.outcome, ConceptGoalResult::Interrupted);
        assert_eq!(r.strategy, ConceptStrategy::Stop);
        assert_eq!(r.concept_events, 0);
        assert!(r.queries <= b.queries);
        assert!(r.lessons.is_empty());
        assert_eq!(before, s.encode());
    }
    let mut s = base();
    let a = PedagogyAssessment::new(
        &[obs("110001"), obs("100011")],
        &[obs("010101"), obs("110000")],
    )
    .unwrap();
    let r = supervise_concept_goal(
        &mut s,
        goal(),
        plan(&lessons()),
        a,
        permissions(),
        ConceptGoalBudget::default(),
    )
    .unwrap();
    assert_eq!(r.strategy, ConceptStrategy::Stop);
    assert_eq!(r.queries, 1);
}
#[test]
fn goal_and_independent_assessments_are_separate_and_label_free() {
    let mut s = base();
    let before = s.encode();
    let a = PedagogyAssessment::new(&[goal().observation], &[obs("010101")]).unwrap();
    let r = supervise_concept_goal(
        &mut s,
        goal(),
        plan(&lessons()),
        a,
        permissions(),
        ConceptGoalBudget::default(),
    )
    .unwrap();
    assert!(r.reason.contains("Sobreposição"));
    assert_eq!(r.queries, 0);
    assert_eq!(before, s.encode());
    let mut l = lessons()[0];
    l.attributes = goal().observation.complete().unwrap();
    let r = run(&mut s, &[l], permissions(), ConceptGoalBudget::default());
    assert!(r.reason.contains("Sobreposição"));
    assert_eq!(r.queries, 0);
    assert_eq!(before, s.encode());
    assert!(PedagogyAssessment::new(&[obs("110001")], &[obs("110001")]).is_err());
}
#[test]
fn repeated_inputs_and_external_keys_do_not_change_selection() {
    let execute = || {
        let mut s = base();
        let r = run(
            &mut s,
            &lessons(),
            permissions(),
            ConceptGoalBudget::default(),
        );
        (r, s.encode())
    };
    let (a, bytes) = execute();
    assert_eq!((a.clone(), bytes), execute());
    let final_decision = a.final_query.as_ref().unwrap().report.decision;
    let correct_external_key = Label::Negative;
    let wrong_external_key = Label::Positive;
    assert_eq!(
        final_decision,
        ConceptDecision::Supported(correct_external_key)
    );
    assert_ne!(
        final_decision,
        ConceptDecision::Supported(wrong_external_key)
    );
    // Nenhuma dessas chaves é argumento de supervise_concept_goal.
}
#[test]
fn independent_symbolic_reference_only_sees_admitted_evidence() {
    let mut s = base();
    let r = run(
        &mut s,
        &lessons(),
        permissions(),
        ConceptGoalBudget::default(),
    );
    for q in r
        .initial
        .iter()
        .chain(&r.final_query)
        .chain(&r.independent_before)
        .chain(&r.independent_after)
    {
        let mut rules: Vec<(u8, Option<bool>)> = (0..6).map(|i| (1 << i, None)).collect();
        for i in 0..6 {
            for j in i + 1..6 {
                rules.push(((1 << i) | (1 << j), None));
            }
        }
        rules.extend([(0, Some(false)), (0, Some(true))]);
        let pred = |rule: (u8, Option<bool>), a: Attributes| {
            rule.1.unwrap_or_else(|| {
                a.values()
                    .iter()
                    .enumerate()
                    .all(|(i, b)| rule.0 & (1 << i) == 0 || *b)
            })
        };
        rules.retain(|rule| {
            q.report
                .consulted
                .iter()
                .all(|e| pred(*rule, e.example.attributes) == (e.example.label == Label::Positive))
        });
        let decisions = rules
            .iter()
            .map(|rule| pred(*rule, q.report.observation.complete().unwrap()))
            .collect::<Vec<_>>();
        let expected = if !decisions.is_empty() && decisions.iter().all(|v| *v == decisions[0]) {
            ConceptDecision::Supported(if decisions[0] {
                Label::Positive
            } else {
                Label::Negative
            })
        } else {
            ConceptDecision::Abstain(Abstention::InsufficientEvidence)
        };
        assert_eq!(q.report.decision, expected);
    }
}
#[test]
fn admission_failure_preserves_auditable_partial_execution() {
    let mut s = base();
    for c in [
        "exemplo 5000 9500 41 000000 - 7 7000 observacao",
        "interpretar-exemplo",
        "avaliar-exemplo",
        "admitir-exemplo 5000",
        "revisar",
    ] {
        apply(&mut s, c);
    }
    let mut rows = lessons();
    rows[1].id = id(5000);
    let mut p = permissions();
    p.admissions = vec![id(402), id(5000)];
    let r = run(&mut s, &rows, p, ConceptGoalBudget::default());
    assert_eq!(r.outcome, ConceptGoalResult::Interrupted);
    assert_eq!(r.execution, ConceptExecution::Interrupted);
    assert_eq!(r.lessons.len(), 2);
    assert!(r.lessons[0].record.admitted);
    assert!(!r.lessons[1].record.admitted);
    assert_eq!(r.concept_events, 8);
    assert!(r.reason.contains("interrompido"));
    let bytes = s.encode();
    assert_eq!(Session::decode(&bytes).unwrap().encode(), bytes);
}

fn public_input(present: bool) -> String {
    format!("config 91 41 51 1 0 10000\nautorizar 41\nconceito 501 opaco\nexemplo 401 901 41 100001 + 7 2401 observacao\ninterpretar-exemplo\navaliar-exemplo\nadmitir-exemplo 401\nrevisar\nobjetivo9 9001 100010\nplano9\nlicao9 402 402 902 000001 - 7 2402\nlicao9 403 403 903 100000 - 7 2403\navaliacoes-objetivo9 110001 010101\nfixar9\npermissoes9 41 sim {} 402,403\nacompanhar9 5 2\nhistorico-objetivo9\n",if present{"sim"}else{"nao"})
}
#[test]
fn cli_runs_one_identified_episode_and_blocks_repetition() {
    use std::io::Cursor;
    let mut output = vec![];
    neurons::session::f1::run(
        Cursor::new(public_input(true) + "acompanhar9 5 2\nensinar9\nsair\n"),
        &mut output,
    )
    .unwrap();
    let text = String::from_utf8(output).unwrap();
    assert_eq!(text.matches("Erro:").count(), 2);
    assert!(
        text.contains("outcome: Resolved")
            && text.contains("TeachOnce")
            && text.contains("EpisodeId(9001)")
    );
    assert!(text.contains("objetivo conceitual encerrado"));
}
#[test]
fn independent_control_does_not_improve_without_authorized_teaching() {
    let mut taught = base();
    let r = run(
        &mut taught,
        &lessons(),
        permissions(),
        ConceptGoalBudget::default(),
    );
    let mut control = base();
    let before = control.encode();
    let mut p = permissions();
    p.presentation = None;
    let c = run(&mut control, &lessons(), p, ConceptGoalBudget::default());
    assert_eq!(c.outcome, ConceptGoalResult::Interrupted);
    assert_eq!(before, control.encode());
    let goal_after_control =
        PedagogyHost::pedagogy_query(&control, EpisodeId::new(99).unwrap(), goal().observation)
            .unwrap();
    assert_eq!(
        goal_after_control.decision,
        ConceptDecision::Abstain(Abstention::InsufficientEvidence)
    );
    assert_eq!(
        r.final_query.unwrap().report.decision,
        ConceptDecision::Supported(Label::Negative)
    );
    let key_before = [Label::Positive];
    let key_after = [Label::Negative];
    assert_eq!(
        r.independent_before
            .iter()
            .zip(key_before)
            .filter(|(r, k)| r.report.decision == ConceptDecision::Supported(*k))
            .count(),
        1
    );
    assert_eq!(
        r.independent_after
            .iter()
            .zip(key_after)
            .filter(|(r, k)| r.report.decision == ConceptDecision::Supported(*k))
            .count(),
        1
    );
    let independent_control =
        PedagogyHost::pedagogy_query(&control, EpisodeId::new(100).unwrap(), obs("010101"))
            .unwrap();
    assert_eq!(
        independent_control.decision,
        ConceptDecision::Abstain(Abstention::InsufficientEvidence)
    );
}
#[test]
fn five_queries_are_a_single_aggregate_and_capacity_reservation_is_checked() {
    let mut s = base();
    let a = PedagogyAssessment::new(&[obs("110001"), obs("100011")], &[obs("010101")]).unwrap();
    let r = supervise_concept_goal(
        &mut s,
        goal(),
        plan(&lessons()),
        a,
        permissions(),
        ConceptGoalBudget::default(),
    )
    .unwrap();
    assert_eq!(r.queries, 5);
    assert_eq!(r.outcome, ConceptGoalResult::Resolved);
    assert!(r.history.len() <= 8);
    let mut s = base();
    apply(&mut s, "agora 9999");
    let before = s.encode();
    let r = run(
        &mut s,
        &lessons(),
        permissions(),
        ConceptGoalBudget::default(),
    );
    assert_eq!(r.strategy, ConceptStrategy::Stop);
    assert_eq!(r.queries, 1);
    assert_eq!(before, s.encode());
}
#[test]
fn independent_process_restores_knowledge_but_requires_new_goal_and_grants() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let dir = std::env::temp_dir().join(format!(
        "neurons-goal-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&dir).unwrap();
    let first = dir.join("learned.f1");
    let restored = dir.join("restored.f1");
    let after_query = dir.join("queried.f1");
    let execute = |input: String| {
        let mut p = Command::new(env!("CARGO_BIN_EXE_neurons"))
            .arg("session-f1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        p.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
        let out = p.wait_with_output().unwrap();
        assert!(out.status.success());
        String::from_utf8(out.stdout).unwrap()
    };
    let initial = execute(public_input(true) + &format!("salvar {}\nsair\n", first.display()));
    assert!(!initial.contains("Erro:"));
    let following=execute(format!("restaurar {}\nsalvar {}\nhistorico-objetivo9\nacompanhar9 5 2\nobjetivo9 9002 100010\nplano9\navaliacoes-objetivo9 110001 010101\nfixar9\npermissoes9 41 sim nao -\nacompanhar9 1 0\nsalvar {}\nsair\n",first.display(),restored.display(),after_query.display()));
    assert_eq!(following.matches("Erro:").count(), 2);
    assert!(
        following.contains("sem supervisão de objetivo conceitual")
            && following.contains("defina objetivo9")
    );
    assert!(
        following.contains("outcome: Resolved")
            && following.contains("strategy: Respond")
            && following.contains("queries: 1")
    );
    assert!(!following.contains("strategy: TeachOnce") && !following.contains("EpisodeId(9001)"));
    assert_eq!(
        std::fs::read(&first).unwrap(),
        std::fs::read(&restored).unwrap()
    );
    assert_eq!(
        std::fs::read(&first).unwrap(),
        std::fs::read(&after_query).unwrap()
    );
}
