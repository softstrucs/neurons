//! Conformidade publica C2-P1; nao importa nem altera o diagnostico historico P0.
//! As expectativas sao fixadas pelo contrato de vinculo, nunca pelo classificador.
use neurons::cognition::concepts::{Attributes, Label, Observation};
use neurons::cognitive_contracts::{
    CatalogId, ContextId, EpisodeId, RuleId, RuleRef, RuleVersion, Scope, SourceId, Validity,
};
use neurons::curriculum::{
    supervise_concept_goal, ConceptExecution, ConceptGoal, ConceptGoalBudget, ConceptGoalReport,
    ConceptGoalResult, CpfEvidence, GoalCriterion, Lesson, PedagogyAssessment, PedagogyHost,
    PedagogyPermissions, TeachingPlan,
};
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::NodeId;
use neurons::session::f1::Session;

fn bits(value: &str) -> Observation {
    let raw: [u8; 6] = value.as_bytes().try_into().unwrap();
    Attributes::new(raw.map(|byte| byte - b'0')).unwrap().into()
}

fn public_episode(
    context: u32,
    presentation: bool,
    sufficient: bool,
) -> (Session, ConceptGoalReport) {
    let scope = Scope {
        catalog: CatalogId::new(91).unwrap(),
        context: ContextId::new(context).unwrap(),
        rule: RuleRef {
            id: RuleId::new(51).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 10000).unwrap(),
    };
    let mut host = Session::new();
    let commands = [
        format!("config 91 {context} 51 1 0 10000"),
        format!("autorizar {context}"),
        "conceito 501 opaco".to_owned(),
        format!("exemplo 401 901 {context} 100001 + 7 2401 observacao"),
        "interpretar-exemplo".to_owned(),
        "avaliar-exemplo".to_owned(),
        "admitir-exemplo 401".to_owned(),
        "revisar".to_owned(),
    ];
    for command in commands {
        host.execute(&command).unwrap();
    }
    let goal = ConceptGoal {
        id: EpisodeId::new(9001).unwrap(),
        concept: NodeId::new(501).unwrap(),
        scope,
        observation: bits(if sufficient { "110001" } else { "100010" }),
        criterion: GoalCriterion::SupportedByCurrentEvidence,
    };
    let rows = [(402, "000001"), (403, "100000")].map(|(number, observation)| Lesson {
        id: EngramId::new(number).unwrap(),
        receipt: number,
        exemplar: NodeId::new(number + 500).unwrap(),
        attributes: bits(observation).complete().unwrap(),
        label: Label::Negative,
        source: SourceId::new(7).unwrap(),
        evidence: EvidenceId::new(number + 2000).unwrap(),
    });
    let before = host.encode();
    let result = supervise_concept_goal(
        &mut host,
        goal,
        TeachingPlan::new(goal.concept, scope, &rows).unwrap(),
        PedagogyAssessment::new(
            &[bits(if sufficient { "100011" } else { "110001" })],
            &[bits("010101")],
        )
        .unwrap(),
        PedagogyPermissions {
            evaluation: Some(scope),
            presentation: presentation.then_some(scope),
            admissions: rows.iter().map(|lesson| lesson.id).collect(),
        },
        ConceptGoalBudget::default(),
    )
    .unwrap();
    if !presentation || sufficient {
        assert_eq!(host.encode(), before);
    }
    (host, result)
}

#[derive(Clone, Copy)]
enum ScopeField {
    Catalog,
    Context,
    RuleId,
    RuleVersion,
    Validity,
}

fn incompatible_scope(scope: &mut Scope, field: ScopeField) {
    match field {
        ScopeField::Catalog => scope.catalog = CatalogId::new(92).unwrap(),
        ScopeField::Context => scope.context = ContextId::new(42).unwrap(),
        ScopeField::RuleId => scope.rule.id = RuleId::new(52).unwrap(),
        ScopeField::RuleVersion => scope.rule.version = RuleVersion::new(2).unwrap(),
        ScopeField::Validity => scope.validity = Validity::new(1, 9999).unwrap(),
    }
}

enum Case {
    Taught,
    AlreadySupported,
    NoTeaching,
    OriginalAfterMutations,
    Observation { initial: bool },
    Scope { initial: bool, field: ScopeField },
    OtherObservation,
    OtherContext,
    GuardFinalGoal,
    GuardInitialGoal,
    GuardRevision,
    GuardOutcome,
    GuardExecution,
    FallbackObservation,
    FallbackScope,
    FallbackGoal,
}

fn expected(case: &Case) -> CpfEvidence {
    let (knowledge, learning) = match case {
        Case::Taught | Case::OriginalAfterMutations => (true, true),
        Case::AlreadySupported
        | Case::Observation { initial: true }
        | Case::Scope { initial: true, .. }
        | Case::GuardInitialGoal
        | Case::GuardRevision => (true, false),
        _ => (false, false),
    };
    CpfEvidence {
        knowledge_supported: knowledge,
        learning_evidenced: learning,
        competence_demonstrated: false,
    }
}

fn check(name: &str, case: Case) {
    let sufficient = matches!(
        case,
        Case::AlreadySupported
            | Case::FallbackObservation
            | Case::FallbackScope
            | Case::FallbackGoal
    );
    let presentation = !matches!(case, Case::NoTeaching);
    let (host, original) = public_episode(41, presentation, sufficient);
    let mut report = original.clone();
    let state = host.encode();
    match case {
        Case::Taught | Case::AlreadySupported | Case::NoTeaching => {}
        Case::OriginalAfterMutations => {
            let mut discarded = report.clone();
            discarded.final_query.as_mut().unwrap().report.observation = bits("110001");
            assert_ne!(discarded, original);
        }
        Case::Observation { initial } => {
            let query = if initial {
                report.initial.as_mut()
            } else {
                report.final_query.as_mut()
            }
            .unwrap();
            query.report.observation = bits("110001");
        }
        Case::Scope { initial, field } => {
            let query = if initial {
                report.initial.as_mut()
            } else {
                report.final_query.as_mut()
            }
            .unwrap();
            incompatible_scope(&mut query.report.scope, field);
        }
        Case::OtherObservation => {
            let query =
                PedagogyHost::pedagogy_query(&host, EpisodeId::new(99).unwrap(), bits("110001"))
                    .unwrap();
            assert_ne!(query.observation, report.goal.observation);
            report.final_query.as_mut().unwrap().report = query;
        }
        Case::OtherContext => {
            let (_, other) = public_episode(42, true, false);
            let query = other.final_query.unwrap().report;
            assert_ne!(query.scope, report.goal.scope);
            report.final_query.as_mut().unwrap().report = query;
        }
        Case::GuardFinalGoal => {
            report.final_query.as_mut().unwrap().goal.id = EpisodeId::new(9002).unwrap()
        }
        Case::GuardInitialGoal => {
            report.initial.as_mut().unwrap().goal.id = EpisodeId::new(9002).unwrap()
        }
        Case::GuardRevision => report
            .lessons
            .iter_mut()
            .for_each(|lesson| lesson.record.revision = None),
        Case::GuardOutcome => report.outcome = ConceptGoalResult::Partial,
        Case::GuardExecution => report.execution = ConceptExecution::Interrupted,
        Case::FallbackObservation | Case::FallbackScope | Case::FallbackGoal => {
            assert!(report.final_query.is_none());
            let mut final_query = report.initial.as_ref().unwrap().clone();
            match case {
                Case::FallbackObservation => final_query.report.observation = bits("100010"),
                Case::FallbackScope => {
                    incompatible_scope(&mut final_query.report.scope, ScopeField::Context)
                }
                Case::FallbackGoal => final_query.goal.id = EpisodeId::new(9002).unwrap(),
                _ => unreachable!(),
            }
            report.final_query = Some(final_query);
        }
    }
    let required = expected(&case);
    let actual = report.cpf_evidence();
    println!("CASE|{name}|knowledge={}|learning={}|competence={}|expected_knowledge={}|expected_learning={}",
        actual.knowledge_supported, actual.learning_evidenced, actual.competence_demonstrated,
        required.knowledge_supported, required.learning_evidenced);
    assert_eq!(
        host.encode(),
        state,
        "classificacao nao deve alterar Session"
    );
    assert_eq!(actual, required, "vinculo de evidencia incorreto: {name}");
}

macro_rules! conformance {
    ($name:ident, $case:expr) => {
        #[test]
        fn $name() {
            check(stringify!($name), $case);
        }
    };
}
conformance!(positive_taught, Case::Taught);
conformance!(control_already_supported, Case::AlreadySupported);
conformance!(control_no_teaching, Case::NoTeaching);
conformance!(
    control_original_after_mutations,
    Case::OriginalAfterMutations
);
conformance!(initial_observation, Case::Observation { initial: true });
conformance!(final_observation, Case::Observation { initial: false });
conformance!(
    initial_catalog,
    Case::Scope {
        initial: true,
        field: ScopeField::Catalog
    }
);
conformance!(
    final_catalog,
    Case::Scope {
        initial: false,
        field: ScopeField::Catalog
    }
);
conformance!(
    initial_context,
    Case::Scope {
        initial: true,
        field: ScopeField::Context
    }
);
conformance!(
    final_context,
    Case::Scope {
        initial: false,
        field: ScopeField::Context
    }
);
conformance!(
    initial_rule_id,
    Case::Scope {
        initial: true,
        field: ScopeField::RuleId
    }
);
conformance!(
    final_rule_id,
    Case::Scope {
        initial: false,
        field: ScopeField::RuleId
    }
);
conformance!(
    initial_rule_version,
    Case::Scope {
        initial: true,
        field: ScopeField::RuleVersion
    }
);
conformance!(
    final_rule_version,
    Case::Scope {
        initial: false,
        field: ScopeField::RuleVersion
    }
);
conformance!(
    initial_validity,
    Case::Scope {
        initial: true,
        field: ScopeField::Validity
    }
);
conformance!(
    final_validity,
    Case::Scope {
        initial: false,
        field: ScopeField::Validity
    }
);
conformance!(real_other_observation, Case::OtherObservation);
conformance!(real_other_context, Case::OtherContext);
conformance!(guard_final_goal_id, Case::GuardFinalGoal);
conformance!(guard_initial_goal_id, Case::GuardInitialGoal);
conformance!(guard_revision, Case::GuardRevision);
conformance!(guard_outcome, Case::GuardOutcome);
conformance!(guard_execution, Case::GuardExecution);
conformance!(fallback_final_observation, Case::FallbackObservation);
conformance!(fallback_final_scope, Case::FallbackScope);
conformance!(fallback_final_goal, Case::FallbackGoal);
