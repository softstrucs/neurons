//! Dados públicos inventados para o produto F1. Não importa fixtures ou avaliadores.
use neurons::session::f1::{self, Session, MAX_CHECKPOINT_BYTES, MAX_OPERATIONS};
use std::io::{Cursor, Write};
use std::process::{Command, Stdio};

fn apply(s: &mut Session, command: &str) -> String {
    s.execute(command)
        .unwrap_or_else(|e| panic!("{command}: {e}"))
}
fn base() -> Session {
    let mut s = Session::new();
    for c in [
        "config 91 41 51 1 0 10000",
        "simbolo 101 cobre",
        "simbolo 103 prisma",
        "simbolo 107 fosco",
        "simbolo 109 tecido",
        "simbolo 113 leve",
        "autorizar 41",
    ] {
        apply(&mut s, c);
    }
    s
}
fn fact(s: &mut Session, id: u32, from: u32, to: u32, pol: &str) {
    apply(
        s,
        &format!(
            "receber {id} 41 {from} {to} {pol} 7 {} observacao",
            id + 1000
        ),
    );
    apply(s, "interpretar");
    apply(s, "avaliar");
    apply(s, "admitir");
}
fn acquisition() -> Session {
    let mut s = base();
    fact(&mut s, 201, 101, 103, "+");
    fact(&mut s, 202, 109, 103, "+");
    let out = apply(&mut s, "iniciar 101 107");
    assert!(out.contains("MissingEvidence"));
    let out = apply(&mut s, "pedido");
    assert!(out.contains("NodeId(103)"));
    assert!(out.contains("NodeId(107)"));
    s
}
fn feedback(
    s: &mut Session,
    source: u32,
    evidence: &str,
    nature: &str,
    context: u32,
    to: u32,
) -> String {
    apply(
        s,
        &format!("retorno 301 1 {context} 103 {to} + {source} {evidence} {nature}"),
    );
    apply(s, "interpretar-retorno");
    apply(s, "avaliar-retorno")
}
fn concept() -> Session {
    let mut s = base();
    apply(&mut s, "conceito 501 opaco");
    s
}
fn example(s: &mut Session, id: u32, vector: &str, label: &str) -> String {
    apply(
        s,
        &format!(
            "exemplo {id} {} 41 {vector} {label} 7 {} observacao",
            id + 500,
            id + 2000
        ),
    );
    apply(s, "interpretar-exemplo");
    let out = apply(s, "avaliar-exemplo");
    assert!(out.contains("Admissible"));
    apply(s, &format!("admitir-exemplo {id}"));
    apply(s, "revisar")
}
#[test]
fn explicit_acquisition_proves_two_queries_without_storing_conclusions() {
    let mut s = acquisition();
    let original = s.history().to_vec();
    let before = s.factual_snapshot();
    assert!(feedback(&mut s, 7, "1203", "observacao", 41, 107).contains("Admissible"));
    assert_eq!(s.factual_snapshot(), before);
    apply(&mut s, "admitir-retorno 203");
    assert_eq!(s.memory().len(), 3);
    let before = s.factual_snapshot();
    let output = apply(&mut s, "reavaliar");
    assert!(output.contains("TwoHop"));
    assert!(output.contains("EngramId(201)"));
    assert!(output.contains("EngramId(203)"));
    let output = apply(&mut s, "consultar-final 109 107");
    assert!(output.contains("TwoHop"));
    assert!(output.contains("EngramId(202)"));
    assert_eq!(before, s.factual_snapshot());
    assert_eq!(&s.history()[..original.len()], &original);
    assert!(!s
        .metadata()
        .iter()
        .any(|m| m.information.statement.object().get() == 107
            && [101, 109].contains(&m.information.statement.subject().get())));
    assert!(s
        .metadata()
        .iter()
        .all(|m| m.state == neurons::cognitive_contracts::EpistemicState::AdmittedObservation));
}
#[test]
fn literal_missing_conflict_context_and_query_capacity_are_explicit() {
    let mut s = base();
    fact(&mut s, 201, 101, 107, "+");
    fact(&mut s, 202, 101, 107, "-");
    let before = s.factual_snapshot();
    assert!(apply(&mut s, "consultar 41 101 107").contains("ContradictoryEvidence"));
    assert!(apply(&mut s, "consultar 42 101 107").contains("RuleOutsideContext"));
    assert!(apply(&mut s, "consultar 41 109 107").contains("MissingEvidence"));
    assert_eq!(before, s.factual_snapshot());
    let before = s.snapshot();
    assert!(s.execute("consultar 41 101 107").is_err());
    assert_eq!(before, s.snapshot());
    let mut positive = base();
    fact(&mut positive, 201, 101, 107, "+");
    assert!(apply(&mut positive, "consultar 41 101 107").contains("Direct"));
}
#[test]
fn seed_admission_rejects_unknown_source_missing_evidence_hypothesis_context_and_duplicates() {
    for tail in [
        "41 101 103 + 99 1201 observacao",
        "41 101 103 + 7 - observacao",
        "41 101 103 + 7 1201 hipotese",
        "42 101 103 + 7 1201 observacao",
    ] {
        let mut s = base();
        apply(&mut s, &format!("receber 201 {tail}"));
        apply(&mut s, "interpretar");
        assert!(apply(&mut s, "avaliar").contains("Err("));
        let before = s.snapshot();
        assert!(s.execute("admitir").is_err());
        assert_eq!(before, s.snapshot());
        assert_eq!(s.memory().len(), 0);
    }
    let mut s = base();
    fact(&mut s, 201, 101, 103, "+");
    apply(&mut s, "receber 201 41 103 107 + 7 1202 observacao");
    apply(&mut s, "interpretar");
    assert!(apply(&mut s, "avaliar").contains("Duplicate"));
}
#[test]
fn malformed_commands_and_wrong_phase_are_atomic() {
    let mut s = base();
    for bad in [
        "admitir",
        "pedido",
        "reavaliar",
        "salvar /tmp/never-created-by-execute",
        "restaurar /tmp/no-file",
        "receber 0 41 101 103 + 7 1 observacao",
        "receber 1 41 101 101 + 7 1 observacao",
        "simbolo 101 duplicado",
        "agora 0",
        "config 1 1 1 1 0 10000",
    ] {
        let before = s.snapshot();
        assert!(s.execute(bad).is_err(), "{bad}");
        assert_eq!(before, s.snapshot(), "{bad}");
    }
    assert!(s.execute(&"x".repeat(257)).is_err());
    assert!(s.execute("config\n1").is_err());
}
#[test]
fn feedback_quarantine_wrong_link_irrelevance_and_temporal_limits_do_not_create_proofs() {
    for (src, ev, nat, ctx) in [
        (99, "1203", "observacao", 41),
        (7, "-", "observacao", 41),
        (7, "1203", "hipotese", 41),
        (7, "1203", "observacao", 42),
    ] {
        let mut s = acquisition();
        let before = s.factual_snapshot();
        assert!(feedback(&mut s, src, ev, nat, ctx, 107).contains("Quarantined"));
        assert_eq!(before, s.factual_snapshot());
        assert!(s.execute("admitir-retorno 203").is_err());
        apply(&mut s, "admitir-retorno");
        assert!(apply(&mut s, "reavaliar").contains("MissingEvidence"));
    }
    let mut s = acquisition();
    assert!(feedback(&mut s, 7, "1203", "observacao", 41, 113).contains("Admissible"));
    apply(&mut s, "admitir-retorno 203");
    assert!(apply(&mut s, "reavaliar").contains("MissingEvidence"));
    let mut s = acquisition();
    apply(&mut s, "retorno 301 2 41 103 107 + 7 1203 observacao");
    apply(&mut s, "interpretar-retorno");
    assert!(apply(&mut s, "avaliar-retorno").contains("InvalidLink"));
    let mut s = base();
    fact(&mut s, 201, 101, 107, "+");
    apply(&mut s, "agora 10001");
    let before = s.snapshot();
    assert!(s.execute("consultar 41 101 107").is_err());
    assert_eq!(before, s.snapshot());
}
#[test]
fn concept_revision_is_explained_by_public_counterexamples_and_preserves_versions() {
    let mut s = concept();
    assert_eq!(
        s.concepts().unwrap().models()[0].current().criteria.len(),
        23
    );
    assert!(apply(&mut s, "classificar 41 100001").contains("NoEvidence"));
    // Família H23 intacta; sequência pública própria deste teste de produto.
    example(&mut s, 401, "100001", "+");
    let first = s.concepts().unwrap().models()[0].versions().to_vec();
    assert!(apply(&mut s, "classificar 41 100000").contains("InsufficientEvidence"));
    example(&mut s, 402, "000001", "-");
    example(&mut s, 403, "100000", "-");
    let c = s.concepts().unwrap();
    let versions = c.models()[0].versions();
    assert_eq!(&versions[..first.len()], &first);
    assert_eq!(versions.len(), 4);
    assert_eq!(versions[3].criteria.len(), 1);
    assert!(!versions[3].eliminated.is_empty());
    assert_eq!(versions[3].support.len(), 3);
    let before = s.factual_snapshot();
    assert!(apply(&mut s, "classificar 41 100001").contains("Supported(Positive)"));
    assert!(apply(&mut s, "classificar 41 ?00001").contains("IncompleteAttributes"));
    assert!(apply(&mut s, "classificar 42 100001").contains("NoModel"));
    assert_eq!(before, s.factual_snapshot());
    assert_eq!(s.concepts().unwrap().memory().len(), 3);
}
#[test]
fn concept_conflicts_are_preserved_not_resolved_by_source_or_recency() {
    let mut s = concept();
    example(&mut s, 401, "100001", "+");
    example(&mut s, 402, "100001", "-");
    let before = s.factual_snapshot();
    assert!(apply(&mut s, "classificar 41 100001").contains("ContradictoryEvidence"));
    assert_eq!(before, s.factual_snapshot());
    assert_eq!(s.concepts().unwrap().memory().len(), 2);
    let mut s = concept();
    example(&mut s, 401, "000000", "+");
    example(&mut s, 402, "111111", "-");
    assert!(apply(&mut s, "classificar 41 100001").contains("ModelClassInadequate"));
}
#[test]
fn concepts_quarantine_and_pending_revision_never_promote_predictions() {
    for tail in ["99 2401 observacao", "7 - observacao", "7 2401 hipotese"] {
        let mut s = concept();
        apply(&mut s, &format!("exemplo 401 901 41 100001 + {tail}"));
        apply(&mut s, "interpretar-exemplo");
        assert!(apply(&mut s, "avaliar-exemplo").contains("Quarantined"));
        assert!(s.execute("admitir-exemplo 401").is_err());
        assert_eq!(s.concepts().unwrap().memory().len(), 0);
    }
    let mut s = concept();
    apply(&mut s, "exemplo 401 901 41 100001 + 7 2401 observacao");
    apply(&mut s, "interpretar-exemplo");
    apply(&mut s, "avaliar-exemplo");
    apply(&mut s, "admitir-exemplo 401");
    let before = s.snapshot();
    assert!(s
        .execute("classificar 41 100001")
        .unwrap_err()
        .contains("PendingRevision"));
    assert_eq!(before, s.snapshot());
    apply(&mut s, "revisar");
    apply(&mut s, "exemplo 402 902 41 100001 + 7 2402 observacao");
    apply(&mut s, "interpretar-exemplo");
    assert!(apply(&mut s, "avaliar-exemplo").contains("Duplicate"));
}
#[test]
fn round_trip_preserves_pending_phases_provenance_versions_queries_and_continuation() {
    let mut s = acquisition();
    feedback(&mut s, 7, "1203", "observacao", 41, 107);
    let mut restored = Session::decode(&s.encode()).unwrap();
    assert_eq!(s.snapshot(), restored.snapshot());
    for c in [
        "admitir-retorno 203",
        "reavaliar",
        "consultar-final 109 107",
        "conceito 501 opaco",
    ] {
        assert_eq!(apply(&mut s, c), apply(&mut restored, c));
    }
    example(&mut s, 401, "100001", "+");
    let mut restored = Session::decode(&s.encode()).unwrap();
    assert_eq!(s.snapshot(), restored.snapshot());
    assert_eq!(
        apply(&mut s, "classificar 41 100001"),
        apply(&mut restored, "classificar 41 100001")
    );
    assert_eq!(s.encode(), restored.encode());
}
#[test]
fn all_checkpoint_bytes_truncations_versions_and_excess_are_checked() {
    let mut s = concept();
    example(&mut s, 401, "100001", "+");
    let bytes = s.encode();
    for i in 0..bytes.len() {
        let mut bad = bytes.clone();
        bad[i] ^= 1;
        assert!(Session::decode(&bad).is_err(), "byte {i}");
        assert!(Session::decode(&bytes[..i]).is_err(), "truncated {i}");
    }
    assert!(Session::decode(&vec![0; MAX_CHECKPOINT_BYTES + 1]).is_err());
    assert!(Session::decode(b"NRNlegacy").is_err());
    let mut forged = b"NRNF1\0\0\0".to_vec();
    forged.extend(1u32.to_le_bytes());
    let payload = b"salvar /tmp/forged-never-created";
    forged.extend((payload.len() as u32).to_le_bytes());
    let h = payload.iter().fold(0xcbf29ce484222325u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    });
    forged.extend(h.to_le_bytes());
    forged.extend(0u64.to_le_bytes());
    forged.extend(payload);
    assert!(Session::decode(&forged).is_err());
}
#[test]
fn capacity_determinism_and_read_views_preserve_prefix() {
    let mut s = Session::new();
    for i in 1..=MAX_OPERATIONS {
        apply(&mut s, &format!("agora {i}"));
    }
    let before = s.snapshot();
    assert!(s.execute("agora 129").is_err());
    assert_eq!(s.snapshot(), before);
    apply(&mut s, "estado");
    apply(&mut s, "historico");
    assert_eq!(s.snapshot(), before);
    assert_eq!(Session::decode(&s.encode()).unwrap().snapshot(), before);
    let mut s = base();
    for (i, (from, to)) in [
        (101, 103),
        (101, 107),
        (101, 109),
        (101, 113),
        (103, 101),
        (103, 107),
        (103, 109),
        (103, 113),
    ]
    .into_iter()
    .enumerate()
    {
        fact(&mut s, 600 + i as u32, from, to, "+");
    }
    assert_eq!(s.memory().len(), 8);
    apply(&mut s, "receber 608 41 107 101 + 7 1608 observacao");
    apply(&mut s, "interpretar");
    apply(&mut s, "avaliar");
    let before = s.snapshot();
    assert!(s.execute("admitir").unwrap_err().contains("CapacityFull"));
    assert_eq!(before, s.snapshot());
}
#[test]
fn runner_and_cli_preserve_legacy_and_handle_errors_eof_and_invalid_utf8() {
    let mut output = vec![];
    f1::run(
        Cursor::new(
            b"config 91 41 51 1 0 10000\nconceito 501 opaco\nclassificar 41 100001\nsair\n",
        ),
        &mut output,
    )
    .unwrap();
    let out = String::from_utf8(output).unwrap();
    assert!(out.contains("NoEvidence"));
    assert!(!out.contains("Erro:"));
    let mut output = vec![];
    f1::run(Cursor::new(vec![255, b'\n']), &mut output).unwrap();
    assert!(String::from_utf8(output)
        .unwrap()
        .contains("UTF-8 inválido"));
    let mut output = vec![];
    f1::run(Cursor::new(vec![b'x'; 257]), &mut output).unwrap();
    assert!(String::from_utf8(output).unwrap().contains("linha excede"));
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
        .write_all(b"config 91 41 51 1 0 10000\nconceito 501 opaco\nestado\nsair\n")
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8(out.stdout)
        .unwrap()
        .contains("H23 inicial"));
}
#[test]
fn disk_failure_exclusive_creation_and_failed_restore_preserve_live_session() {
    let dir = std::env::temp_dir().join(format!("neurons-f1-files-{}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("session.f1");
    let mut s = concept();
    example(&mut s, 401, "100001", "+");
    s.save(&path).unwrap();
    assert_eq!(Session::load(&path).unwrap().snapshot(), s.snapshot());
    let bytes = std::fs::read(&path).unwrap();
    assert!(s.save(&path).is_err());
    assert_eq!(bytes, std::fs::read(&path).unwrap());
    let pending = dir.join("other.f1.f1-pending");
    std::fs::write(&pending, b"preexisting").unwrap();
    assert!(s.save(&dir.join("other.f1")).is_err());
    assert_eq!(std::fs::read(&pending).unwrap(), b"preexisting");
    assert!(s.save(&dir.join("missing/session")).is_err());
    let before = s.snapshot();
    assert!(Session::load(&dir.join("absent")).is_err());
    std::fs::write(dir.join("corrupt"), b"bad").unwrap();
    assert!(Session::load(&dir.join("corrupt")).is_err());
    assert_eq!(before, s.snapshot());
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&path, dir.join("link")).unwrap();
        assert!(Session::load(&dir.join("link")).is_err());
    }
    // Apenas o diretório temporário criado pelo próprio teste.
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn checkpoint_valid_checksum_cannot_change_derived_state_or_replay_invalid_acts() {
    let mut s = concept();
    example(&mut s, 401, "100001", "+");
    let mut bytes = s.encode();
    // Journal íntegro, mas checksum do estado derivado divergente.
    bytes[24] ^= 1;
    assert!(Session::decode(&bytes)
        .err()
        .unwrap()
        .contains("estado reconstruído incompatível"));
}

#[test]
fn concept_record_context_and_query_limits_fail_without_losing_prefix() {
    let mut s = concept();
    for i in 0..16 {
        let vector = format!("{i:06b}");
        example(&mut s, 700 + i, &vector, "+");
    }
    assert_eq!(s.concepts().unwrap().memory().len(), 16);
    apply(&mut s, "exemplo 716 1216 41 010000 + 7 2716 observacao");
    apply(&mut s, "interpretar-exemplo");
    apply(&mut s, "avaliar-exemplo");
    let before = s.snapshot();
    assert!(s.execute("admitir-exemplo 716").is_err());
    assert_eq!(before, s.snapshot());
    let mut s = concept();
    for ctx in 42..57 {
        apply(&mut s, &format!("contexto-conceitual {ctx}"));
    }
    assert_eq!(s.concepts().unwrap().models().len(), 16);
    let before = s.snapshot();
    assert!(s.execute("contexto-conceitual 57").is_err());
    assert_eq!(before, s.snapshot());
    let mut s = concept();
    for _ in 0..80 {
        assert!(apply(&mut s, "classificar 41 100001").contains("NoEvidence"));
    }
    let before = s.snapshot();
    assert!(s.execute("classificar 41 100001").is_err());
    assert_eq!(before, s.snapshot());
}

#[test]
fn acquisition_preserves_opposition_and_never_picks_a_convenient_source() {
    let mut s = base();
    fact(&mut s, 201, 101, 103, "+");
    fact(&mut s, 204, 103, 107, "-");
    let out = apply(&mut s, "iniciar 101 107");
    assert!(out.contains("MissingEvidence"));
    apply(&mut s, "pedido");
    assert!(feedback(&mut s, 8, "1203", "observacao", 41, 107).contains("Admissible"));
    apply(&mut s, "admitir-retorno 203");
    let before = s.factual_snapshot();
    assert!(apply(&mut s, "reavaliar").contains("ContradictoryEvidence"));
    assert_eq!(before, s.factual_snapshot());
    assert_eq!(s.memory().len(), 3);
}
