//! Diagnostico publico C2-P0: preserva e reproduz o comportamento anterior a correcao.
//! Copiar para tests/ncm_c2_p0.rs somente na arvore isolada descrita no manifesto.
//! PASS significa que o diagnostico reproduziu o resultado registrado, nao conformidade.
use neurons::cognition::concepts::{Attributes, Label, Observation};
use neurons::cognitive_contracts::*;
use neurons::curriculum::*;
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::NodeId;
use neurons::metacognition::concepts::{ConceptDecision, ConceptStrategy};
use neurons::session::f1::Session;

fn scope(context: u32) -> Scope {
    Scope {
        catalog: CatalogId::new(91).unwrap(),
        context: ContextId::new(context).unwrap(),
        rule: RuleRef {
            id: RuleId::new(51).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 10000).unwrap(),
    }
}
fn observation(bits: &str) -> Observation {
    Attributes::new(
        bits.as_bytes()
            .try_into()
            .map(|a: [u8; 6]| a.map(|b| b - b'0'))
            .unwrap(),
    )
    .unwrap()
    .into()
}
fn prepared(context: u32) -> Session {
    let mut session = Session::new();
    for command in [
        format!("config 91 {context} 51 1 0 10000"),
        format!("autorizar {context}"),
        "conceito 501 opaco".into(),
        format!("exemplo 401 901 {context} 100001 + 7 2401 observacao"),
        "interpretar-exemplo".into(),
        "avaliar-exemplo".into(),
        "admitir-exemplo 401".into(),
        "revisar".into(),
    ] {
        session.execute(&command).unwrap();
    }
    session
}
fn run(context: u32, teach: bool, already_supported: bool) -> (Session, ConceptGoalReport) {
    let mut session = prepared(context);
    let goal = ConceptGoal {
        id: EpisodeId::new(9001).unwrap(),
        concept: NodeId::new(501).unwrap(),
        scope: scope(context),
        observation: observation(if already_supported {
            "110001"
        } else {
            "100010"
        }),
        criterion: GoalCriterion::SupportedByCurrentEvidence,
    };
    let lessons: Vec<_> = [(402, "000001"), (403, "100000")]
        .iter()
        .map(|(id, bits)| Lesson {
            id: EngramId::new(*id).unwrap(),
            receipt: *id,
            exemplar: NodeId::new(*id + 500).unwrap(),
            attributes: observation(bits).complete().unwrap(),
            label: Label::Negative,
            source: SourceId::new(7).unwrap(),
            evidence: EvidenceId::new(*id + 2000).unwrap(),
        })
        .collect();
    let before = session.encode();
    let report = supervise_concept_goal(
        &mut session,
        goal,
        TeachingPlan::new(goal.concept, goal.scope, &lessons).unwrap(),
        PedagogyAssessment::new(
            &[observation(if already_supported {
                "100011"
            } else {
                "110001"
            })],
            &[observation("010101")],
        )
        .unwrap(),
        PedagogyPermissions {
            evaluation: Some(goal.scope),
            presentation: teach.then_some(goal.scope),
            admissions: lessons.iter().map(|lesson| lesson.id).collect(),
        },
        ConceptGoalBudget::default(),
    )
    .unwrap();
    if !teach || already_supported {
        assert_eq!(before, session.encode());
    }
    (session, report)
}

fn bindings_match(report: &ConceptGoalReport) -> bool {
    report
        .initial
        .iter()
        .chain(&report.final_query)
        .all(|query| {
            query.goal == report.goal
                && query.report.scope == report.goal.scope
                && query.report.observation == report.goal.observation
        })
}
fn row(id: &str, report: &ConceptGoalReport, expected: (bool, bool)) -> CpfEvidence {
    let observed = report.cpf_evidence();
    println!(
        "CASE|{id}|knowledge={}|learning={}|competence={}|expected_knowledge={}|expected_learning={}|bindings_match={}",
        observed.knowledge_supported,
        observed.learning_evidenced,
        observed.competence_demonstrated,
        expected.0,
        expected.1,
        bindings_match(report),
    );
    assert!(!observed.competence_demonstrated);
    observed
}
fn assert_reproduced_false_positive(
    id: &str,
    report: &ConceptGoalReport,
    expected_knowledge: bool,
) {
    assert!(!bindings_match(report));
    let observed = row(id, report, (expected_knowledge, false));
    // Pre-registro diagnostico: a implementacao atual aceita indevidamente.
    // Nao inverter esta assercao para disfarcar o resultado. C2-P1 tera testes proprios.
    assert!(observed.knowledge_supported && observed.learning_evidenced);
}

#[test]
fn genuine_learning_positive() {
    let (session, report) = run(41, true, false);
    assert!(bindings_match(&report));
    assert_eq!(report.strategy, ConceptStrategy::TeachOnce);
    assert_eq!(
        report
            .initial
            .as_ref()
            .unwrap()
            .report
            .version
            .as_ref()
            .unwrap()
            .number,
        1
    );
    assert_eq!(
        report
            .final_query
            .as_ref()
            .unwrap()
            .report
            .version
            .as_ref()
            .unwrap()
            .number,
        3
    );
    assert_eq!(
        report.final_query.as_ref().unwrap().report.decision,
        ConceptDecision::Supported(Label::Negative)
    );
    let bytes = session.encode();
    let result = row("positive_taught", &report, (true, true));
    assert!(result.knowledge_supported && result.learning_evidenced);
    assert_eq!(session.encode(), bytes);
}

#[test]
fn sufficient_knowledge_is_not_new_learning() {
    let (_, report) = run(41, true, true);
    assert_eq!(report.strategy, ConceptStrategy::Respond);
    assert!(report.lessons.is_empty());
    let result = row("control_already_supported", &report, (true, false));
    assert!(result.knowledge_supported && !result.learning_evidenced);
}

#[test]
fn without_teaching_control_stays_unsupported() {
    let (_, report) = run(41, false, false);
    assert_eq!(report.outcome, ConceptGoalResult::Interrupted);
    assert!(report.lessons.is_empty());
    let result = row("control_no_teaching", &report, (false, false));
    assert_eq!(result, CpfEvidence::default());
}

#[test]
fn incompatible_internal_observations_and_scope_fields_are_accepted() {
    let (session, original) = run(41, true, false);
    let bytes = session.encode();
    for initial in [true, false] {
        let phase = if initial { "initial" } else { "final" };
        for field in [
            "observation",
            "catalog",
            "context",
            "rule_id",
            "rule_version",
            "validity",
        ] {
            let mut altered = original.clone();
            let query = if initial {
                altered.initial.as_mut().unwrap()
            } else {
                altered.final_query.as_mut().unwrap()
            };
            match field {
                "observation" => query.report.observation = observation("110001"),
                "catalog" => query.report.scope.catalog = CatalogId::new(92).unwrap(),
                "context" => query.report.scope.context = ContextId::new(42).unwrap(),
                "rule_id" => query.report.scope.rule.id = RuleId::new(52).unwrap(),
                "rule_version" => query.report.scope.rule.version = RuleVersion::new(2).unwrap(),
                "validity" => query.report.scope.validity = Validity::new(1, 9999).unwrap(),
                _ => unreachable!(),
            }
            // Uma consulta final valida ainda sustenta conhecimento, mas nao aprendizagem.
            assert_reproduced_false_positive(&format!("{phase}_{field}"), &altered, initial);
        }
    }
    assert_eq!(session.encode(), bytes);
    assert_eq!(
        row("control_original_after_mutations", &original, (true, true)),
        original.cpf_evidence()
    );
}

#[test]
fn genuine_other_query_or_context_can_be_transplanted_into_the_wrapper() {
    let (session, original) = run(41, true, false);
    let bytes = session.encode();
    let other =
        PedagogyHost::pedagogy_query(&session, EpisodeId::new(99).unwrap(), observation("110001"))
            .unwrap();
    assert_eq!(other.decision, ConceptDecision::Supported(Label::Positive));
    assert_ne!(other.observation, original.goal.observation);
    let mut transplanted = original.clone();
    transplanted.final_query.as_mut().unwrap().report = other;
    assert_reproduced_false_positive("real_other_observation", &transplanted, false);

    let (_, other_context) = run(42, true, false);
    let mut transplanted = original.clone();
    transplanted.final_query.as_mut().unwrap().report = other_context.final_query.unwrap().report;
    assert_reproduced_false_positive("real_other_context", &transplanted, false);
    assert_eq!(session.encode(), bytes);
}

#[test]
fn existing_identity_and_execution_guards_remain_negative_controls() {
    let (_, original) = run(41, true, false);
    for field in [
        "final_goal_id",
        "initial_goal_id",
        "revision",
        "outcome",
        "execution",
    ] {
        let mut altered = original.clone();
        match field {
            "final_goal_id" => {
                altered.final_query.as_mut().unwrap().goal.id = EpisodeId::new(9002).unwrap()
            }
            "initial_goal_id" => {
                altered.initial.as_mut().unwrap().goal.id = EpisodeId::new(9002).unwrap()
            }
            "revision" => altered
                .lessons
                .iter_mut()
                .for_each(|lesson| lesson.record.revision = None),
            "outcome" => altered.outcome = ConceptGoalResult::Partial,
            "execution" => altered.execution = ConceptExecution::Interrupted,
            _ => unreachable!(),
        }
        let expected_knowledge = matches!(field, "initial_goal_id" | "revision");
        let evidence = row(
            &format!("guard_{field}"),
            &altered,
            (expected_knowledge, false),
        );
        assert_eq!(evidence.knowledge_supported, expected_knowledge);
        assert!(!evidence.learning_evidenced);
    }
}
