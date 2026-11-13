//! Cenário público próprio. Referência simbólica não recebe gabarito ou plano futuro.
use neurons::cognition::concepts::{Attributes, Label, Observation};
use neurons::cognitive_contracts::*;
use neurons::curriculum::{Lesson, PedagogyPermissions};
use neurons::engram::{EngramId, EvidenceId, RecordStatus};
use neurons::event::NodeId;
use neurons::integration::composed::Episode;
use neurons::integration::composed::*;
use neurons::integration::query::{PathAuthorization, QueryCoordinator, RuleAuthorization};
use neurons::metacognition::concepts::{Abstention, ConceptDecision};
use neurons::session::f1::Session;
fn node(n: u32) -> NodeId {
    NodeId::new(n).unwrap()
}
fn eid(n: u32) -> EpisodeId {
    EpisodeId::new(n).unwrap()
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
            .map(|a: [u8; 6]| a.map(|v| v - b'0'))
            .unwrap(),
    )
    .unwrap()
}
fn obs(s: &str) -> Observation {
    Observation::new(
        s.as_bytes()
            .try_into()
            .map(|a: [u8; 6]| a.map(|v| Some(v - b'0')))
            .unwrap(),
    )
    .unwrap()
}
fn apply(s: &mut Session, c: &str) {
    s.execute(c).unwrap_or_else(|e| panic!("{c}: {e}"));
}
fn example(s: &mut Session, id: u32, bits: &str, label: &str) {
    for c in [
        format!(
            "exemplo {id} {} 41 {bits} {label} 7 {} observacao",
            id + 500,
            id + 2000
        ),
        "interpretar-exemplo".into(),
        "avaliar-exemplo".into(),
        format!("admitir-exemplo {id}"),
        "revisar".into(),
    ] {
        apply(s, &c);
    }
}
fn relation(s: &mut Session, id: u32, from: u32, to: u32, positive: bool, evidence: u32) {
    for c in [
        format!(
            "receber {id} 41 {from} {to} {} 8 {evidence} observacao",
            if positive { "+" } else { "-" }
        ),
        "interpretar".into(),
        "avaliar".into(),
        "admitir".into(),
    ] {
        apply(s, &c);
    }
}
fn base(with_relation: bool) -> Session {
    let mut s = Session::new();
    for c in [
        "config 91 41 51 1 0 10000",
        "simbolo 101 classe-c",
        "simbolo 103 classe-d",
        "simbolo 105 ponte",
        "autorizar 41",
        "conceito 501 opaco",
    ] {
        apply(&mut s, c);
    }
    example(&mut s, 401, "111111", "+");
    example(&mut s, 402, "000001", "-");
    if with_relation {
        relation(&mut s, 11, 101, 103, true, 8011);
    }
    s
}
fn binding() -> Binding {
    Binding {
        id: eid(800),
        item: node(9010),
        observation: obs("100101"),
        concept: node(501),
        relational_concept: node(101),
        destination: node(103),
        scope: scope(),
        source: SourceId::new(9).unwrap(),
        evidence: EvidenceId::new(9900).unwrap(),
        origin: BindingOrigin::ExternalSyntheticObservation,
    }
}
fn registry() -> SyntheticRegistry {
    // Tabela pública congelada, independente do construtor de pedidos binding().
    // Não recebe Session, pergunta, hipótese, experiências ou rótulo esperado.
    let record = Binding {
        id: eid(800),
        item: node(9010),
        observation: Observation::new([Some(1), Some(0), Some(0), Some(1), Some(0), Some(1)])
            .unwrap(),
        concept: node(501),
        relational_concept: node(101),
        destination: node(103),
        scope: scope(),
        source: SourceId::new(9).unwrap(),
        evidence: EvidenceId::new(9900).unwrap(),
        origin: BindingOrigin::ExternalSyntheticObservation,
    };
    SyntheticRegistry::new(record, 1).unwrap()
}
fn permissions(b: &Binding) -> Permissions {
    Permissions {
        conceptual: Some(b.scope),
        relational: Some(PathAuthorization::inclusion_three(b.scope)),
        composition: Some(b.clone()),
    }
}
fn lesson(id: u32, bits: &str, label: Label) -> Lesson {
    Lesson {
        id: EngramId::new(id).unwrap(),
        receipt: id,
        exemplar: node(id + 500),
        attributes: attrs(bits),
        label,
        source: SourceId::new(7).unwrap(),
        evidence: EvidenceId::new(id + 2000).unwrap(),
    }
}
fn teaching_permissions(id: u32) -> PedagogyPermissions {
    PedagogyPermissions {
        evaluation: Some(scope()),
        presentation: Some(scope()),
        admissions: vec![EngramId::new(id).unwrap()],
    }
}
/// Verificador público do harness, independente da classificação do adaptador.
/// Recalcula H23 por tabelas booleanas e percorre somente evidências admitidas.
/// A tabela de identidade e as concessões são entradas independentes, sem rótulo alvo.
pub fn reference_verify(
    s: &Session,
    root: &SyntheticRegistry,
    r: &Report,
    p: &Permissions,
) -> Result<Status, String> {
    let b = root.record();
    if &r.binding != b
        || b.origin != BindingOrigin::ExternalSyntheticObservation
        || p.composition.as_ref() != Some(b)
        || p.conceptual != Some(b.scope)
        || p.relational != Some(PathAuthorization::inclusion_three(b.scope))
    {
        return Err("identidade/vínculo/permissões".into());
    }
    if r.dependencies.registry_revision != root.revision() || r.dependencies.state != s.encode() {
        return Err("dependência obsoleta".into());
    }
    let c = s.concepts().ok_or("sem conceito")?;
    if c.concept().id() != b.concept || c.models().len() != 1 || c.models()[0].scope() != b.scope {
        return Err("contexto/conceito".into());
    }
    let cr = r.conceptual.as_ref().ok_or("sem relatório conceitual")?;
    let examples = c.examples().map_err(|e| format!("{e:?}"))?;
    if cr.episode != r.objective
        || cr.scope != b.scope
        || cr.observation != b.observation
        || cr.at != s.logical_time()
        || cr.version.as_ref() != Some(c.models()[0].current())
        || cr.consulted != examples
    {
        return Err("relatório/versão/evidências conceituais adulterados".into());
    }
    let sources = [SourceId::new(7).unwrap(), SourceId::new(8).unwrap()];
    for ex in &examples {
        if ex.metadata.scope != b.scope
            || !sources.contains(&ex.metadata.source)
            || ex.metadata.incorporated_at > s.logical_time()
            || !b.scope.validity.contains(s.logical_time())
            || ex.record.evidence() != Some(ex.metadata.evidence)
            || ex.record.status() != RecordStatus::Observed
        {
            return Err("proveniência conceitual".into());
        }
    }
    let contradictory = examples.iter().enumerate().any(|(i, a)| {
        examples[..i].iter().any(|b| {
            a.example.attributes == b.example.attributes && a.example.label != b.example.label
        })
    });
    let mut rules: Vec<(u8, Option<bool>)> = (0..6).map(|i| (1 << i, None)).collect();
    for i in 0..6 {
        for j in i + 1..6 {
            rules.push(((1 << i) | (1 << j), None));
        }
    }
    rules.extend([(0, Some(false)), (0, Some(true))]);
    let predict = |rule: (u8, Option<bool>), a: Attributes| {
        rule.1.unwrap_or_else(|| {
            a.values()
                .iter()
                .enumerate()
                .all(|(i, v)| rule.0 & (1 << i) == 0 || *v)
        })
    };
    rules.retain(|rule| {
        examples.iter().all(|ex| {
            predict(*rule, ex.example.attributes) == (ex.example.label == Label::Positive)
        })
    });
    let votes = rules
        .iter()
        .map(|rule| predict(*rule, b.observation.complete().unwrap()))
        .collect::<Vec<_>>();
    let cd = if contradictory {
        ConceptDecision::Abstain(Abstention::ContradictoryEvidence)
    } else if examples.is_empty() {
        ConceptDecision::Abstain(Abstention::NoEvidence)
    } else if votes.is_empty() {
        ConceptDecision::Abstain(Abstention::ModelClassInadequate)
    } else if votes.iter().all(|v| *v == votes[0]) {
        ConceptDecision::Supported(if votes[0] {
            Label::Positive
        } else {
            Label::Negative
        })
    } else {
        ConceptDecision::Abstain(Abstention::InsufficientEvidence)
    };
    if cr.decision != cd {
        return Err("classificação adulterada".into());
    }
    let rr = r.relational.as_ref().ok_or("sem relatório relacional")?;
    if rr.id != r.objective
        || rr.rule != b.scope.rule
        || rr.query.scope() != b.scope
        || rr.query.subject() != b.relational_concept
        || rr.query.object() != b.destination
        || rr.at != s.logical_time()
    {
        return Err("consulta relacional".into());
    }
    let eligible = s
        .metadata()
        .iter()
        .filter_map(|m| {
            let e = s.memory().get(m.id).ok()?;
            (m.state == EpistemicState::AdmittedObservation
                && m.disposition.is_none()
                && e.status() == RecordStatus::Observed
                && e.observed_at() <= s.logical_time()
                && b.scope.validity.contains(e.observed_at())
                && check_observation(m.information, b.scope, &sources, s.logical_time()).is_ok())
            .then_some((e, *m))
        })
        .collect::<Vec<_>>();
    let consulted = eligible
        .iter()
        .map(
            |(engram, metadata)| neurons::integration::query::BoundRecord {
                engram: *engram,
                metadata: *metadata,
            },
        )
        .collect::<Vec<_>>();
    if rr.consulted != consulted {
        return Err("projeção relacional adulterada".into());
    }
    let snapshot = MemorySnapshot::new(&s.memory().iter().map(|e| e.id()).collect::<Vec<_>>())
        .map_err(|e| format!("{e:?}"))?;
    if rr.before != snapshot || rr.after != snapshot {
        return Err("snapshot relacional adulterado".into());
    }
    let positives = eligible
        .iter()
        .filter(|(_, m)| m.information.statement.polarity() == Polarity::Positive)
        .map(|(e, m)| {
            (
                m.information.statement.subject(),
                m.information.statement.object(),
                e.id(),
            )
        })
        .collect::<Vec<_>>();
    let negatives = eligible
        .iter()
        .filter(|(_, m)| m.information.statement.polarity() == Polarity::Negative)
        .map(|(_, m)| {
            (
                m.information.statement.subject(),
                m.information.statement.object(),
            )
        })
        .collect::<Vec<_>>();
    // Enumeração simbólica independente, finita: no máximo 8+64+512 extensões.
    fn paths(
        node: NodeId,
        target: NodeId,
        edges: &[(NodeId, NodeId, EngramId)],
        visited: &mut Vec<NodeId>,
        path: &mut Vec<(NodeId, NodeId)>,
        out: &mut Vec<Vec<(NodeId, NodeId)>>,
    ) {
        if path.len() == 3 {
            return;
        }
        for (a, b, _) in edges {
            if *a != node || visited.contains(b) {
                continue;
            }
            path.push((*a, *b));
            if *b == target {
                out.push(path.clone());
            } else {
                visited.push(*b);
                paths(*b, target, edges, visited, path, out);
                visited.pop();
            }
            path.pop();
        }
    }
    let mut alternatives = vec![];
    paths(
        b.relational_concept,
        b.destination,
        &positives,
        &mut vec![b.relational_concept],
        &mut vec![],
        &mut alternatives,
    );
    let opposed = !alternatives.is_empty()
        && (negatives.contains(&(b.relational_concept, b.destination))
            || alternatives
                .iter()
                .any(|path| path.iter().any(|edge| negatives.contains(edge))));
    if rr
        .review
        .as_ref()
        .is_some_and(|v| (!v.conflicts.is_empty()) != opposed || v.inference.processed_events > 32)
    {
        return Err("oposição ou eventos relatados inconsistentes".into());
    }
    let coordinator = QueryCoordinator::new(
        s.memory(),
        s.metadata(),
        RuleAuthorization::inclusion_in(b.scope),
        &sources,
    )
    .map_err(|e| format!("{e:?}"))?;
    if let Some(proof) = &rr.proof {
        if proof.premises.len() != 1 {
            return Err("fora do recorte de uma premissa".into());
        }
        let ids = proof
            .premises
            .iter()
            .map(|p| p.engram.id())
            .collect::<Vec<_>>();
        if coordinator
            .verify_path(rr.query, s.logical_time(), &ids)
            .map_err(|e| format!("{e:?}"))?
            != *proof
        {
            return Err("prova adulterada".into());
        }
    }
    if cr
        .consulted
        .iter()
        .any(|ex| ex.metadata.evidence == b.evidence)
        || rr.proof.as_ref().is_some_and(|proof| {
            proof.premises.iter().any(|p| {
                p.metadata.information.evidence == Some(b.evidence)
                    || examples
                        .iter()
                        .any(|e| Some(e.metadata.evidence) == p.metadata.information.evidence)
            })
        })
    {
        return Err("evidência circular/dependente".into());
    }
    let direct = positives
        .iter()
        .any(|(a, d, _)| *a == b.relational_concept && *d == b.destination);
    let expected = if contradictory || opposed {
        Status::Contradictory
    } else if cd == ConceptDecision::Supported(Label::Positive) && direct {
        Status::Supported
    } else {
        Status::Insufficient
    };
    if r.status != expected
        || (expected == Status::Supported && (rr.proof.is_none() || rr.refusal.is_some()))
    {
        return Err("conclusão não demonstrada".into());
    }
    Ok(expected)
}

fn request() -> TeachingRequest {
    TeachingRequest {
        lesson: lesson(403, "100001", Label::Positive),
        before: obs("110000"),
        after: obs("010101"),
        permissions: teaching_permissions(403),
    }
}
fn config() -> &'static str {
    "registro-composto 800 9010 100101 501 101 103 41 9 9900 1\nobjetivo-composto 9001 800 9010 100101 501 101 103 41 9 9900\n"
}
fn grants() -> &'static str {
    "permissoes-compostas 41 sim sim sim\n"
}
fn teaching() -> &'static str {
    "ensino-composto 403 403 903 100001 + 7 2403 110000 010101\npermissoes-ensino-composto 41 sim sim 403\n"
}
fn invoke(input: &str) -> String {
    use std::io::Write;
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_neurons"))
        .arg("session-f1")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}
fn input(s: &Session, tail: &str) -> String {
    format!("{}\n{tail}", s.history().join("\n"))
}
fn execute(s: &Session, extra: &str, teach: bool, budget: &str) -> String {
    invoke(&input(
        s,
        &format!(
            "{}{}{}{extra}acompanhar-composto {budget}\nsair\n",
            config(),
            grants(),
            if teach { teaching() } else { "" }
        ),
    ))
}
fn count_reports(text: &str) -> usize {
    text.lines().filter(|l| *l == "ObjetivoComposto").count()
}
fn directory() -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!(
        "neurons-composed-cli-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&p).unwrap();
    p
}
#[test]
fn real_cli_matches_core_and_independent_reference_for_full_cycle() {
    let mut s = base(true);
    let before = s.encode();
    let meta = s.metadata().to_vec();
    let output = execute(&s, "", true, "8 3 96");
    assert!(!output.contains("Erro:"), "{output}");
    let report = Episode::new(&s, eid(9001), binding()).supervise(
        &mut s,
        &registry(),
        permissions(&binding()),
        Some(request()),
    );
    assert_eq!(report.outcome, GoalOutcome::Supported);
    reference_verify(
        &s,
        &registry(),
        report.final_query.as_ref().unwrap(),
        &report.permissions,
    )
    .unwrap();
    assert!(output.contains(&neurons::session::f1::format_composed_report(&report)));
    assert_eq!(count_reports(&output), 1);
    assert!(output.contains("inicial.estado=Some(Insufficient)"));
    assert!(output.contains("reconsulta.estado=Some(Supported)"));
    assert!(output.contains("verificacao=Some(Verified(Supported))"));
    assert!(output.contains("inicial.versao=Some(2)"));
    assert!(output.contains("reconsulta.versao=Some(3)"));
    assert!(output.contains("teaching_attempts: 1"));
    assert!(output.contains("conceptual_queries: 5"));
    assert_eq!(meta, s.metadata());
    assert_ne!(s.encode(), before);
    assert!(!output.contains("dependencies") && !output.contains("NRNF1"));
}
#[test]
fn supported_initially_does_not_execute_teaching() {
    let mut s = base(true);
    example(&mut s, 403, "100001", "+");
    let output = execute(&s, "", true, "8 3 96");
    assert!(output.contains("resultado=Supported"));
    assert!(output.contains("tentativa_iniciada=false"));
    assert!(output.contains("reconsulta.estado=None"));
    assert!(output.contains("teaching_attempts: 0"));
}
#[test]
fn equivalent_control_without_teaching_remains_insufficient() {
    let s = base(true);
    let control = execute(&s, "", false, "8 3 96");
    let treatment = execute(&s, "", true, "8 3 96");
    assert!(control.contains("resultado=Abstention"));
    assert!(control.contains("tentativa_iniciada=false"));
    assert!(control.contains("inicial.versao=Some(2)"));
    assert!(treatment.contains("inicial.versao=Some(2)"));
    assert!(treatment.contains("resultado=Supported"));
}
#[test]
fn missing_relation_never_triggers_teaching() {
    let output = execute(&base(false), "", true, "8 3 96");
    assert!(output.contains("diagnostico=RelationalInsufficiency"));
    assert!(output.contains("resultado=Abstention"));
    assert!(output.contains("tentativa_iniciada=false"));
}
#[test]
fn opposing_evidence_in_either_domain_blocks_teaching() {
    for concept in [true, false] {
        let mut s = base(true);
        if concept {
            example(&mut s, 404, "111111", "-");
        } else {
            relation(&mut s, 12, 101, 103, false, 8012);
        }
        let output = execute(&s, "", true, "8 3 96");
        assert!(output.contains("diagnostico=Opposition"));
        assert!(output.contains("tentativa_iniciada=false"));
        assert!(output.contains("resultado=Abstention"));
    }
}
#[test]
fn each_query_permission_denied_separately_has_zero_consumption_and_no_evidence() {
    for flags in ["nao sim sim", "sim nao sim", "sim sim nao"] {
        let output = invoke(&input(
            &base(true),
            &format!(
                "{}permissoes-compostas 41 {flags}\n{}acompanhar-composto 8 3 96\nsair\n",
                config(),
                teaching()
            ),
        ));
        assert!(output.contains("diagnostico=PermissionMissing"));
        assert!(output.contains("resultado=Interrupted"));
        assert!(output.contains("conceptual_queries: 0"));
        assert!(output.contains("relational_queries: 0"));
        assert!(output.contains("inicial.evidencias=None"));
        assert!(output.contains("inicial.prova=None"));
        assert!(output.contains("inicial.consultados_conceituais=None"));
        assert!(output.contains("inicial.conflitos=None"));
        assert!(output.contains("inicial.revisao_registro=None"));
    }
}
#[test]
fn teaching_permissions_are_not_implied_by_composition() {
    for args in ["nao sim 403", "sim nao 403", "sim sim -", "sim sim 404"] {
        let t = teaching().replace("sim sim 403", args);
        let output = invoke(&input(
            &base(true),
            &format!(
                "{}{}{}acompanhar-composto 8 3 96\nsair\n",
                config(),
                grants(),
                t
            ),
        ));
        assert!(output.contains("resultado=Interrupted"));
        assert!(output.contains("diagnostico=PermissionMissing"));
        assert!(output.contains("tentativa_iniciada=false"));
        assert!(output.contains("teaching_attempts: 0"));
    }
}
#[test]
fn insufficient_aggregate_budget_never_starts_experience() {
    for budget in ["0 3 96", "4 3 96", "8 2 96", "8 3 63"] {
        let output = execute(&base(true), "", true, budget);
        assert!(output.contains("resultado=Interrupted"));
        assert!(output.contains("diagnostico=ResourceLimited"));
        assert!(output.contains("tentativa_iniciada=false"));
    }
}
#[test]
fn malformed_and_missing_fields_fail_before_any_execution() {
    let invalid = [
        "registro-composto",
        "registro-composto 0 9010 100101 501 101 103 41 9 9900 1",
        "registro-composto 800 9010 abcdef 501 101 103 41 9 9900 1",
        "registro-composto 800 9010 100101 501 101 103 41 9 - 1",
        "acompanhar-composto 8 3 96",
    ];
    for command in invalid {
        let output = invoke(&input(&base(true), &format!("{command}\nsair\n")));
        assert_eq!(count_reports(&output), 0);
        assert!(output.contains("Erro:"), "{output}");
    }
    for command in [
        "objetivo-composto",
        "ensino-composto 403",
        "permissoes-compostas 41 talvez sim sim",
        "acompanhar-composto 8 3",
        "acompanhar-composto -1 3 96",
        "acompanhar-composto 9 3 96",
    ] {
        let output = invoke(&input(
            &base(true),
            &format!("{}{command}\nsair\n", config()),
        ));
        assert_eq!(count_reports(&output), 0);
        assert!(output.contains("Erro:"));
    }
}
#[test]
fn omitted_concessions_and_incomplete_teaching_configuration_are_errors() {
    for config_tail in [
        "",
        "ensino-composto 403 403 903 100001 + 7 2403 110000 010101\n",
        "permissoes-ensino-composto 41 sim sim 403\n",
    ] {
        let output = invoke(&input(
            &base(true),
            &format!(
                "{}{}{config_tail}acompanhar-composto 8 3 96\nsair\n",
                config(),
                if config_tail.is_empty() { "" } else { grants() }
            ),
        ));
        assert_eq!(count_reports(&output), 0);
        assert!(output.contains("Erro:"));
    }
}
#[test]
fn incompatible_binding_is_not_silently_replaced_by_the_registry() {
    let cfg = config().replace(
        "objetivo-composto 9001 800 9010",
        "objetivo-composto 9001 800 9011",
    );
    let output = invoke(&input(
        &base(true),
        &format!(
            "{cfg}{}{}acompanhar-composto 8 3 96\nsair\n",
            grants(),
            teaching()
        ),
    ));
    assert!(output.contains("diagnostico=ContractBlocked"));
    assert!(output.contains("resultado=Interrupted"));
    assert!(output.contains("item: NodeId(9011)"));
    assert!(output.contains("conceptual_queries: 0"));
}
#[test]
fn fixed_goal_and_lesson_cannot_be_changed_or_reexecuted() {
    let extra="objetivo-composto 9002 800 9010 100101 501 101 103 41 9 9900\nensino-composto 404 404 904 100000 - 7 2404 110000 010101\n";
    let output = invoke(&input(
        &base(true),
        &format!(
            "{}{}{}{extra}acompanhar-composto 8 3 96\nacompanhar-composto 8 3 96\nensinar9\nsair\n",
            config(),
            grants(),
            teaching()
        ),
    ));
    assert_eq!(count_reports(&output), 1);
    assert!(output.contains("objetivo=9001"));
    assert!(!output.contains("objetivo=9002"));
    assert_eq!(output.matches("Erro:").count(), 4);
    assert!(output.contains("teaching_attempts: 1"));
}
#[test]
fn parallel_mutation_and_report_injection_are_not_cli_capabilities() {
    let output = execute(
        &base(true),
        "agora 9999\nrevisar\nrelatorio-composto Supported\n",
        true,
        "8 3 96",
    );
    assert_eq!(output.matches("Erro:").count(), 3);
    assert!(output.contains("resultado=Supported"));
    assert!(output.contains("reconsulta.versao=Some(3)"));
}
#[test]
fn history_is_read_only_and_repeats_the_same_report() {
    let output = invoke(&input(
        &base(true),
        &format!(
            "{}{}{}acompanhar-composto 8 3 96\nhistorico-composto\nsair\n",
            config(),
            grants(),
            teaching()
        ),
    ));
    let blocks = output
        .split("\nObjetivoComposto\n")
        .skip(1)
        .map(|b| b.split("\nFimObjetivoComposto").next().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0], blocks[1]);
}
#[test]
fn post_checkpoint_knowledge_is_preserved_without_transient_authority() {
    let dir = directory();
    let learned = dir.join("learned.f1");
    let after = dir.join("after.f1");
    let output = invoke(&input(
        &base(true),
        &format!(
            "{}{}{}acompanhar-composto 8 3 96\nsalvar {}\nsair\n",
            config(),
            grants(),
            teaching(),
            learned.display()
        ),
    ));
    assert!(!output.contains("Erro:"));
    let restored = Session::load(&learned).unwrap();
    let original = base(true);
    assert_eq!(
        restored.memory().iter().collect::<Vec<_>>(),
        original.memory().iter().collect::<Vec<_>>()
    );
    assert_eq!(restored.metadata(), original.metadata());
    assert_eq!(restored.concepts().unwrap().examples().unwrap().len(), 3);
    let restored_output=invoke(&format!("restaurar {}\nhistorico-composto\nacompanhar-composto 8 3 96\n{}{}acompanhar-composto 8 3 96\nsalvar {}\nsair\n",learned.display(),config(),grants(),after.display()));
    assert_eq!(restored_output.matches("Erro:").count(), 2);
    assert_eq!(count_reports(&restored_output), 1);
    assert!(restored_output.contains("resultado=Supported"));
    assert!(restored_output.contains("tentativa_iniciada=false"));
    assert_eq!(
        std::fs::read(learned).unwrap(),
        std::fs::read(after).unwrap()
    );
}
#[test]
fn restoration_in_same_process_clears_all_preparation_and_concessions() {
    let dir = directory();
    let path = dir.join("initial.f1");
    base(true).save(&path).unwrap();
    let output=invoke(&input(&base(true),&format!("{}{}{}restaurar {}\nhistorico-composto\nacompanhar-composto 8 3 96\n{}acompanhar-composto 8 3 96\nsair\n",config(),grants(),teaching(),path.display(),config())));
    assert_eq!(count_reports(&output), 0);
    assert_eq!(output.matches("Erro:").count(), 3);
    assert!(output.contains("declare permissoes-compostas"));
}
#[test]
fn failed_restore_does_not_reopen_a_consumed_episode() {
    let dir = directory();
    let output = invoke(&input(
        &base(true),
        &format!(
            "{}{}{}acompanhar-composto 8 3 96\nrestaurar {}\nacompanhar-composto 8 3 96\nsair\n",
            config(),
            grants(),
            teaching(),
            dir.join("missing.f1").display()
        ),
    ));
    assert_eq!(count_reports(&output), 1);
    assert_eq!(output.matches("Erro:").count(), 2);
}
#[test]
fn denied_episode_cannot_reuse_grants_after_failure() {
    let output=invoke(&input(&base(true),&format!("{}permissoes-compostas 41 nao sim sim\nacompanhar-composto 8 3 96\npermissoes-compostas 41 sim sim sim\nacompanhar-composto 8 3 96\nsair\n",config())));
    assert_eq!(count_reports(&output), 1);
    assert_eq!(output.matches("Erro:").count(), 2);
    assert!(output.contains("resultado=Interrupted"));
}
#[test]
fn existing_pedagogical_episode_cannot_be_bypassed() {
    let output = invoke(&input(&base(true), &format!("plano9\n{}sair\n", config())));
    assert_eq!(count_reports(&output), 0);
    assert!(output.contains("episódio paralelo"));
}
#[test]
fn execution_error_and_partial_outcomes_are_not_reported_as_success() {
    let fail = teaching()
        .replace("ensino-composto 403 403", "ensino-composto 401 403")
        .replace("sim sim 403", "sim sim 401");
    let output = invoke(&input(
        &base(true),
        &format!(
            "{}{}{}acompanhar-composto 8 3 96\nsair\n",
            config(),
            grants(),
            fail
        ),
    ));
    assert!(output.contains("resultado=Interrupted"));
    assert!(output.contains("falha_experiencia=Some("));
    let partial = teaching().replace("100001 + 7 2403 110000", "110000 - 7 2403 100001");
    let output = invoke(&input(
        &base(true),
        &format!(
            "{}{}{}acompanhar-composto 8 3 96\nsair\n",
            config(),
            grants(),
            partial
        ),
    ));
    assert!(output.contains("resultado=Partial"));
    assert!(output.contains("verificacao=Some(Verified(Insufficient))"));
}

#[test]
fn preparation_is_readonly_and_does_not_enter_the_checkpoint() {
    let dir = directory();
    let before = dir.join("before.f1");
    let after = dir.join("prepared.f1");
    let output = invoke(&input(
        &base(true),
        &format!(
            "salvar {}\n{}{}{}salvar {}\nsair\n",
            before.display(),
            config(),
            grants(),
            teaching(),
            after.display()
        ),
    ));
    assert!(!output.contains("Erro:"));
    assert_eq!(count_reports(&output), 0);
    assert_eq!(
        std::fs::read(before).unwrap(),
        std::fs::read(after).unwrap()
    );
}
#[test]
fn verification_budget_failure_is_explicit_and_never_claims_support() {
    let mut s = base(true);
    example(&mut s, 403, "100001", "+");
    let output = execute(&s, "", false, "8 1 96");
    assert!(output.contains("inicial.estado=Some(Supported)"));
    assert!(output.contains("verificacao=Some(Interrupted)"));
    assert!(output.contains("resultado=Interrupted"));
    assert!(!output.contains("resultado=Supported"));
}
#[test]
fn negative_classification_is_abstention_and_not_automatic_teaching() {
    let mut s = base(true);
    example(&mut s, 403, "100001", "+");
    let cfg = config().replace("100101", "010101");
    let output = invoke(&input(
        &s,
        &format!(
            "{cfg}{}{}acompanhar-composto 8 3 96\nsair\n",
            grants(),
            teaching()
        ),
    ));
    assert!(output.contains("diagnostico=NegativeClassification"));
    assert!(output.contains("resultado=Abstention"));
    assert!(output.contains("tentativa_iniciada=false"));
}
#[test]
fn invalid_parameter_does_not_fix_grants_or_consume_an_episode() {
    let output=invoke(&input(&base(true),&format!("{}permissoes-compostas 41 sim talvez sim\n{}{}acompanhar-composto 9 3 96\nacompanhar-composto 8 3 96\nsair\n",config(),grants(),teaching())));
    assert_eq!(output.matches("Erro:").count(), 2);
    assert_eq!(count_reports(&output), 1);
    assert!(output.contains("resultado=Supported"));
}
#[test]
fn byte_line_limits_and_controls_never_execute_an_episode() {
    use std::io::Cursor;
    for input in [vec![b'x'; 257], vec![255, b'\n']] {
        let mut out = vec![];
        neurons::session::f1::run(Cursor::new(input), &mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        assert_eq!(count_reports(&text), 0);
        assert!(text.contains("Erro:"));
    }
    let output = invoke(&input(
        &base(true),
        "registro-composto\t800 9010 100101 501 101 103 41 9 9900 1\nsair\n",
    ));
    assert_eq!(count_reports(&output), 0);
    assert!(output.contains("linha inválida"));
}
