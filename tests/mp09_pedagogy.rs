//! Casos públicos próprios; nenhuma fixture, partição ou avaliador científico.
use neurons::cognition::concepts::{Attributes, Label, Observation};
use neurons::cognitive_contracts::*;
use neurons::curriculum::*;
use neurons::engram::{EngramId, EvidenceId};
use neurons::event::NodeId;
use neurons::metacognition::concepts::{Abstention, ConceptDecision, PedagogicalDiagnosis};
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
    PedagogyAssessment::new(
        &[obs("100010"), obs("110001")],
        &[obs("010101"), obs("100011")],
    )
    .unwrap()
}
fn permissions() -> PedagogyPermissions {
    PedagogyPermissions {
        evaluation: Some(scope()),
        presentation: Some(scope()),
        admissions: vec![id(402), id(403)],
    }
}
fn score(reports: &[neurons::integration::concepts::ConceptReport], key: &[Label]) -> usize {
    assert_eq!(reports.len(), key.len());
    reports
        .iter()
        .zip(key)
        .filter(|(r, k)| r.decision == ConceptDecision::Supported(**k))
        .count()
}
#[test]
fn actual_progress_versions_and_no_experience_control() {
    let mut s = base();
    let versions = s.concepts().unwrap().models()[0].versions().to_vec();
    let r = teach_concept(&mut s, plan(&lessons()), assessment(), permissions()).unwrap();
    assert_eq!(
        r.initial
            .iter()
            .chain(&r.posterior)
            .map(|r| r.episode.get())
            .collect::<Vec<_>>(),
        vec![1, 2, 3, 4]
    );
    assert_eq!(r.diagnosis, PedagogicalDiagnosis::ExperienceNeeded);
    assert_eq!(score(&r.initial, &[Label::Negative, Label::Positive]), 1);
    assert_eq!(score(&r.posterior, &[Label::Negative, Label::Positive]), 2);
    assert_eq!(
        r.lessons.iter().map(|r| r.lesson.id).collect::<Vec<_>>(),
        vec![id(402), id(403)]
    );
    assert!(r.lessons.iter().all(|r| r.admitted));
    let c = s.concepts().unwrap();
    assert_eq!(&c.models()[0].versions()[..versions.len()], &versions);
    assert_eq!(c.models()[0].versions().len(), 4);
    assert_eq!(r.final_version.criteria.len(), 1);
    assert_eq!(r.final_version.support.len(), 3);
    assert_eq!(c.memory().len(), 3);
    let mut control = base();
    let mut p = permissions();
    p.presentation = None;
    let before = control.encode();
    let ctl = teach_concept(&mut control, plan(&lessons()), assessment(), p).unwrap();
    assert_eq!(
        score(&ctl.posterior, &[Label::Negative, Label::Positive]),
        1
    );
    assert_eq!(control.encode(), before);
    assert!(r.history.len() <= 8);
}
#[test]
fn permissions_are_independent_and_refusal_preserves_knowledge() {
    for mode in 0..3 {
        let mut s = base();
        let c = s.concepts().unwrap();
        let before = (c.examples().unwrap(), c.models().to_vec());
        let mut p = permissions();
        match mode {
            0 => p.evaluation = None,
            1 => p.presentation = None,
            _ => p.admissions.clear(),
        };
        let r = teach_concept(&mut s, plan(&lessons()), assessment(), p);
        if mode == 0 {
            assert!(r.is_err());
        } else {
            let r = r.unwrap();
            assert!(r.lessons.iter().all(|l| !l.admitted));
            if mode == 2 {
                assert_eq!(r.lessons.len(), 1);
            }
        }
        let c = s.concepts().unwrap();
        assert_eq!(before, (c.examples().unwrap(), c.models().to_vec()));
        let bytes = s.encode();
        assert_eq!(Session::decode(&bytes).unwrap().encode(), bytes);
    }
}
#[test]
fn overlap_is_logged_before_any_execution() {
    let mut s = base();
    let before = s.encode();
    let l = lesson(402, "010101", Label::Negative);
    let r = teach_concept(&mut s, plan(&[l]), assessment(), permissions()).unwrap();
    assert!(r.leak_detected);
    assert!(r.initial.is_empty() && r.lessons.is_empty());
    assert_eq!(before, s.encode());
    let mut s = base();
    let before = s.encode();
    let eval = PedagogyAssessment::new(&[obs("100001")], &[obs("100011")]).unwrap();
    let r = teach_concept(&mut s, plan(&lessons()), eval, permissions()).unwrap();
    assert!(r.leak_detected);
    assert_eq!(s.encode(), before);
}
#[test]
fn evaluation_labels_are_not_an_input_and_cannot_influence_selection() {
    let mut s1 = base();
    let mut s2 = base();
    let r1 = teach_concept(&mut s1, plan(&lessons()), assessment(), permissions()).unwrap();
    let r2 = teach_concept(&mut s2, plan(&lessons()), assessment(), permissions()).unwrap();
    assert_eq!(r1, r2);
    assert_eq!(s1.encode(), s2.encode());
    assert_ne!(
        score(&r1.posterior, &[Label::Negative, Label::Positive]),
        score(&r1.posterior, &[Label::Positive, Label::Negative])
    );
    // Alterar a chave externa muda a nota, nunca as experiências já escolhidas.
    assert_eq!(r1.lessons, r2.lessons);
}
#[test]
fn bad_source_never_changes_hypothesis_or_admits() {
    let mut s = base();
    let before = s.concepts().unwrap().models().to_vec();
    let mut l = lessons()[0];
    l.source = SourceId::new(99).unwrap();
    let r = teach_concept(&mut s, plan(&[l]), assessment(), permissions()).unwrap();
    assert!(!r.lessons[0].admitted);
    assert!(matches!(
        r.lessons[0].admissibility,
        neurons::integration::concepts::Admission::Quarantined(_)
    ));
    assert_eq!(before, s.concepts().unwrap().models());
}
#[test]
fn contradiction_blocks_teaching_conservatively() {
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
    let r = teach_concept(&mut s, plan(&lessons()), assessment(), permissions()).unwrap();
    assert_eq!(r.diagnosis, PedagogicalDiagnosis::Blocked);
    assert!(r.lessons.is_empty());
    assert!(r
        .posterior
        .iter()
        .all(|r| r.decision == ConceptDecision::Abstain(Abstention::ContradictoryEvidence)));
    assert_eq!(before, s.encode());
}
#[test]
fn insufficient_experiences_preserve_abstention() {
    let mut s = base();
    let r = teach_concept(&mut s, plan(&[]), assessment(), permissions()).unwrap();
    assert!(r.lessons.is_empty());
    assert!(r
        .posterior
        .iter()
        .any(|r| matches!(r.decision, ConceptDecision::Abstain(_))));
    assert_eq!(r.initial_version, r.final_version);
}
#[test]
fn queries_and_diagnosis_are_readonly() {
    let s = base();
    let before = s.encode();
    let r = PedagogyHost::pedagogy_query(&s, EpisodeId::new(1).unwrap(), obs("100010")).unwrap();
    assert_eq!(
        r.decision,
        ConceptDecision::Abstain(Abstention::InsufficientEvidence)
    );
    let _ = neurons::metacognition::concepts::diagnose_pedagogy(&[r]);
    assert_eq!(s.encode(), before);
}
#[test]
fn limits_and_identity_fail_closed() {
    assert!(TeachingPlan::new(NodeId::new(501).unwrap(), scope(), &[lessons()[0]; 4]).is_err());
    assert!(plan_try_duplicate().is_err());
    assert!(PedagogyAssessment::new(&[obs("100010")], &[obs("100010")]).is_err());
    let mut s = base();
    let before = s.encode();
    let foreign = TeachingPlan::new(NodeId::new(502).unwrap(), scope(), &lessons()).unwrap();
    assert!(teach_concept(&mut s, foreign, assessment(), permissions()).is_err());
    assert_eq!(before, s.encode());
    apply(&mut s, "agora 9999");
    let before = s.encode();
    assert!(teach_concept(&mut s, plan(&lessons()), assessment(), permissions()).is_err());
    assert_eq!(before, s.encode());
}
fn plan_try_duplicate() -> Result<TeachingPlan, String> {
    TeachingPlan::new(NodeId::new(501).unwrap(), scope(), &[lessons()[0]; 2])
}
#[test]
fn symbolic_reference_uses_only_admitted_examples() {
    let mut s = base();
    let r = teach_concept(&mut s, plan(&lessons()), assessment(), permissions()).unwrap();
    let examples = s.concepts().unwrap().examples().unwrap();
    // Referência separada: tabelas booleanas por máscara, sem chamar family/revise/evaluate do núcleo.
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
        examples
            .iter()
            .all(|e| pred(*rule, e.example.attributes) == (e.example.label == Label::Positive))
    });
    assert_eq!(rules.len(), 1);
    for item in &r.posterior {
        let a = item.observation.complete().unwrap();
        let values = rules.iter().map(|rule| pred(*rule, a)).collect::<Vec<_>>();
        assert!(values.iter().all(|x| *x == values[0]));
        assert_eq!(
            item.decision,
            ConceptDecision::Supported(if values[0] {
                Label::Positive
            } else {
                Label::Negative
            })
        );
    }
}
#[test]
fn checkpoint_restores_versions_examples_and_predictions_without_teaching() {
    let mut s = base();
    let r = teach_concept(&mut s, plan(&lessons()), assessment(), permissions()).unwrap();
    let bytes = s.encode();
    let restored = Session::decode(&bytes).unwrap();
    assert_eq!(restored.encode(), bytes);
    assert_eq!(
        s.concepts().unwrap().models(),
        restored.concepts().unwrap().models()
    );
    assert_eq!(
        s.concepts().unwrap().examples().unwrap(),
        restored.concepts().unwrap().examples().unwrap()
    );
    for item in r.posterior {
        assert_eq!(
            PedagogyHost::pedagogy_query(&restored, EpisodeId::new(1).unwrap(), item.observation)
                .unwrap()
                .decision,
            item.decision
        );
    }
    assert_eq!(restored.encode(), bytes);
}

#[test]
fn incomplete_attributes_request_better_observation_not_more_teaching() {
    let mut s = base();
    teach_concept(&mut s, plan(&lessons()), assessment(), permissions()).unwrap();
    let before = s.encode();
    let probes = PedagogyAssessment::new(&[obs("?00101")], &[obs("110011")]).unwrap();
    let r = teach_concept(&mut s, plan(&[]), probes, permissions()).unwrap();
    assert_eq!(r.diagnosis, PedagogicalDiagnosis::Blocked);
    assert_eq!(
        r.initial[0].decision,
        ConceptDecision::Abstain(Abstention::IncompleteAttributes)
    );
    assert!(r.lessons.is_empty());
    assert_eq!(before, s.encode());
}
fn public_input(presentation: bool) -> String {
    format!("config 91 41 51 1 0 10000\nautorizar 41\nconceito 501 opaco\nexemplo 401 901 41 100001 + 7 2401 observacao\ninterpretar-exemplo\navaliar-exemplo\nadmitir-exemplo 401\nrevisar\nplano9\nlicao9 402 402 902 000001 - 7 2402\nlicao9 403 403 903 100000 - 7 2403\navaliacoes9 100010 110001 010101 100011\nfixar9\npermissoes9 41 sim {} 402,403\nensinar9\nhistorico9\n",if presentation {"sim"} else {"nao"})
}
#[test]
fn cli_supervision_is_single_and_permissions_are_explicit() {
    use std::io::Cursor;
    let mut output = vec![];
    neurons::session::f1::run(
        Cursor::new(public_input(true) + "ensinar9\nsair\n"),
        &mut output,
    )
    .unwrap();
    let text = String::from_utf8(output).unwrap();
    assert_eq!(text.matches("Erro:").count(), 1);
    assert!(text.contains("ExperienceNeeded"));
    assert!(text.contains("admitted: true"));
    assert!(text.contains("episódio MP09 encerrado"));
    let mut output = vec![];
    neurons::session::f1::run(Cursor::new(public_input(false) + "sair\n"), &mut output).unwrap();
    let text = String::from_utf8(output).unwrap();
    assert!(!text.contains("Erro:") && !text.contains("admitted: true"));
}
#[test]
fn process_restore_retains_predictions_and_clears_teaching_supervision() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let dir = std::env::temp_dir().join(format!(
        "neurons-mp09-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&dir).unwrap();
    let original = dir.join("taught.f1");
    let restored = dir.join("restored.f1");
    let execute = |input: String| {
        let mut child = Command::new(env!("CARGO_BIN_EXE_neurons"))
            .arg("session-f1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        let out = child.wait_with_output().unwrap();
        assert!(out.status.success());
        String::from_utf8(out.stdout).unwrap()
    };
    let first = execute(public_input(true) + &format!("salvar {}\nsair\n", original.display()));
    assert!(!first.contains("Erro:"));
    let second = execute(format!(
        "restaurar {}\nsalvar {}\nclassificar 41 010101\nclassificar 41 100011\nhistorico9\nsair\n",
        original.display(),
        restored.display()
    ));
    assert_eq!(
        std::fs::read(&original).unwrap(),
        std::fs::read(&restored).unwrap()
    );
    assert!(second.contains("Supported(Negative)") && second.contains("Supported(Positive)"));
    assert_eq!(second.matches("Erro:").count(), 1);
    assert!(second.contains("sem relatório MP09"));
    assert!(!second.contains("MP09 PedagogyReport"));
}
#[test]
fn teaching_order_is_canonical_and_refused_ids_are_not_replaced() {
    let mut s = base();
    let mut reverse = lessons();
    reverse.reverse();
    let r = teach_concept(&mut s, plan(&reverse), assessment(), permissions()).unwrap();
    assert_eq!(r.lessons[0].lesson.id, id(402));
    let mut s = base();
    let before = s.concepts().unwrap().models().to_vec();
    let mut p = permissions();
    p.admissions = vec![id(403)];
    let r = teach_concept(&mut s, plan(&lessons()), assessment(), p).unwrap();
    assert_eq!(r.lessons.len(), 1);
    assert!(!r.lessons[0].admitted);
    assert_eq!(s.concepts().unwrap().models(), before);
}
