//! Testes públicos MP05, sem fixtures ou avaliadores reservados.
use neurons::integration::synaptic::{SynapticEpisode, MAX_FILE};
use neurons::synapse::SynapseId;
use std::io::Write;
use std::process::{Command, Stdio};
fn apply(e: &mut SynapticEpisode, c: &str) -> String {
    e.execute(c).unwrap_or_else(|x| panic!("{c}: {x}"))
}
fn setup_with(max: i32, step: i32, cycle: usize) -> SynapticEpisode {
    let mut e = SynapticEpisode::new();
    for c in [
        format!("novo 41 61 81 1 0 {max} {step} 16 {cycle}"),
        "neuronio 101 1 4".into(),
        "neuronio 103 2 4".into(),
        "sinapse 11 101 103 0 1".into(),
        "autorizar 7 0 1000".into(),
    ] {
        apply(&mut e, &c);
    }
    e
}
fn setup() -> SynapticEpisode {
    setup_with(3, 2, 64)
}
fn experience(e: &mut SynapticEpisode, id: u32, at: u64) {
    apply(e, &format!("experiencia {id} 61 101 1 {at}"));
    apply(e, "executar");
}
fn feedback(e: &mut SynapticEpisode, id: u32, exp: u32, signal: &str, at: u64) -> String {
    apply(
        e,
        &format!("feedback {id} {exp} 61 7 {} 11 {signal} {at}", 1000 + id),
    )
}
fn weight(e: &SynapticEpisode) -> i32 {
    e.circuit()
        .unwrap()
        .synapse_weight(SynapseId::new(11).unwrap())
        .unwrap()
}
fn destination(e: &SynapticEpisode) -> u64 {
    e.circuit()
        .unwrap()
        .neuron_state(neurons::event::NodeId::new(103).unwrap())
        .unwrap()
        .activations
}
#[test]
fn functional_effect_against_no_update_and_symbolic_reference() {
    let mut learner = setup();
    let mut control = setup();
    assert_eq!(weight(&learner), 0);
    for e in [&mut learner, &mut control] {
        experience(e, 1, 0);
        assert_eq!(destination(e), 0);
    }
    feedback(&mut learner, 201, 1, "+", 1);
    assert_eq!(weight(&learner), 2);
    assert_eq!(weight(&control), 0);
    assert_eq!(destination(&learner), 0);
    for e in [&mut learner, &mut control] {
        experience(e, 2, 2);
    }
    assert_eq!(destination(&learner), 1);
    assert_eq!(destination(&control), 0);
    // Referência independente: soma limitada do passo e comparação com limiar 2.
    let reference_weight = (0i32 + 2).clamp(0, 3);
    assert_eq!(reference_weight >= 2, destination(&learner) > 0);
    let r = &learner.updates()[0];
    assert_eq!(
        (
            r.update.before,
            r.update.after,
            r.bounds,
            r.step,
            r.circuit,
            r.context,
            r.rule,
            r.version
        ),
        (0, 2, (0, 3), 2, 41, 61, 81, 1)
    );
    assert!(r.evidence > 0);
    assert!(!learner.trace().is_empty());
}
#[test]
fn saturation_and_explicit_reversal_are_not_opposite_feedback() {
    let mut e = setup();
    experience(&mut e, 1, 0);
    feedback(&mut e, 201, 1, "+", 1);
    experience(&mut e, 2, 2);
    feedback(&mut e, 202, 2, "+", 4);
    assert_eq!(weight(&e), 3);
    assert!(e.updates()[1].update.saturated);
    let would_be = neurons::learning::PlasticityRule::new(0, 3, 2)
        .unwrap()
        .adjust(3, Some(neurons::learning::Feedback::Negative))
        .unwrap()
        .after;
    assert_eq!(would_be, 1);
    apply(&mut e, "reverter 202 7 5");
    assert_eq!(weight(&e), 2);
    assert!(e.state().contains("Reverted"));
    let before = e.state();
    assert!(e.execute("reverter 202 7 6").is_err());
    assert_eq!(before, e.state());
    assert!(e.execute("feedback 202 2 61 7 1202 11 + 6").is_err());
}
#[test]
fn negative_feedback_has_a_functional_effect() {
    let mut e = setup();
    experience(&mut e, 1, 0);
    feedback(&mut e, 201, 1, "+", 1);
    experience(&mut e, 2, 2);
    assert_eq!(destination(&e), 1);
    feedback(&mut e, 202, 2, "-", 4);
    experience(&mut e, 3, 5);
    assert_eq!(destination(&e), 1);
    assert_eq!(weight(&e), 0);
}
#[test]
fn invalid_absent_expired_context_missing_evidence_duplicate_do_not_mutate() {
    let mut e = setup();
    experience(&mut e, 1, 0);
    let before = e.state();
    for cmd in [
        "feedback 201 1 61 99 1201 11 + 1",
        "feedback 201 1 62 7 1201 11 + 1",
        "feedback 201 999 61 7 1201 11 + 1",
        "feedback 201 1 61 7 0 11 + 1",
        "feedback 201 1 61 7 1201 11 + 1001",
        "feedback 201 1 61 7 1201 99 + 1",
        "feedback 201 1 61 7 1201 11 none 1",
        "reverter 201 99 1",
    ] {
        assert!(e.execute(cmd).is_err(), "{cmd}");
        assert_eq!(before, e.state());
    }
    assert_eq!(weight(&e), 0);
    feedback(&mut e, 201, 1, "+", 1);
    let updated = e.state();
    for cmd in [
        "feedback 201 1 61 7 1201 11 + 2",
        "feedback 202 1 61 7 1202 11 + 2",
    ] {
        assert!(e.execute(cmd).is_err());
        assert_eq!(updated, e.state());
    }
}
#[test]
fn queue_checkpoint_is_rejected_without_drain_or_discard() {
    let mut e = setup_with(3, 2, 1);
    apply(&mut e, "experiencia 1 61 101 1 0");
    let before = e.state();
    assert!(e.encode().unwrap_err().contains("pendentes"));
    assert_eq!(before, e.state());
    assert_eq!(e.circuit().unwrap().queue_len(), 1);
    apply(&mut e, "executar");
    feedback(&mut e, 201, 1, "+", 1);
    apply(&mut e, "experiencia 2 61 101 1 2");
    apply(&mut e, "executar");
    assert_eq!(e.circuit().unwrap().queue_len(), 1);
    let before = e.state();
    assert!(e.encode().is_err());
    assert!(e.execute("feedback 202 2 61 7 1202 11 + 2").is_err());
    assert_eq!(before, e.state());
    apply(&mut e, "executar");
    assert!(e.encode().is_ok());
    assert!(e.execute("feedback 202 2 61 7 1202 11 + 2").is_err()); // destino despachado no instante 3
}
#[test]
fn topology_parameters_and_limits_fail_without_losing_prefix() {
    let mut e = setup();
    let before = e.state();
    for cmd in [
        "neuronio 101 1 4",
        "neuronio 109 0 4",
        "sinapse 12 103 101 1 1",
        "sinapse 12 101 999 1 1",
        "sinapse 12 101 103 4 1",
        "sinapse 12 101 103 1 0",
        "novo 1 1 1 1 0 2 1 16 64",
    ] {
        assert!(e.execute(cmd).is_err(), "{cmd}");
        assert_eq!(before, e.state());
    }
    apply(&mut e, "neuronio 107 1 4");
    apply(&mut e, "neuronio 109 1 4");
    assert!(e.execute("neuronio 113 1 4").is_err());
    for i in 12..=14 {
        apply(&mut e, &format!("sinapse {i} 101 103 0 1"));
    }
    assert!(e.execute("sinapse 15 101 103 0 1").is_err());
    let mut e = setup();
    for i in 1..=8 {
        experience(&mut e, i, (i * 2) as u64);
    }
    let before = e.state();
    assert!(e.execute("experiencia 9 61 101 1 20").is_err());
    assert_eq!(before, e.state());
    assert!(e.circuit().unwrap().engine_metrics().dispatched <= 128);
    let mut e = setup_with(4, 1, 64);
    for i in 1..=4 {
        experience(&mut e, i, (i * 3) as u64);
        feedback(&mut e, 200 + i, i, "+", (i * 3 + 2) as u64);
    }
    let before = e.state();
    assert!(e.execute("reverter 204 7 20").is_err());
    assert_eq!(before, e.state());
}
#[test]
fn local_update_never_touches_unselected_connection_or_memory() {
    let mut e = setup();
    apply(&mut e, "neuronio 107 1 4");
    apply(&mut e, "sinapse 12 101 107 0 1");
    experience(&mut e, 1, 0);
    feedback(&mut e, 201, 1, "+", 1);
    assert_eq!(weight(&e), 2);
    assert_eq!(
        e.circuit()
            .unwrap()
            .synapse_weight(SynapseId::new(12).unwrap())
            .unwrap(),
        0
    );
}
#[test]
fn snapshot_roundtrip_and_continuation_are_identical_including_consumed_feedback() {
    let mut e = setup();
    experience(&mut e, 1, 0);
    feedback(&mut e, 201, 1, "+", 1);
    let bytes = e.encode().unwrap();
    let mut restored = SynapticEpisode::decode(&bytes).unwrap();
    assert_eq!(e.state(), restored.state());
    assert_eq!(
        e.circuit().unwrap().snapshot().unwrap(),
        restored.circuit().unwrap().snapshot().unwrap()
    );
    assert!(restored.execute("feedback 201 1 61 7 1201 11 + 2").is_err());
    for cmd in [
        "experiencia 2 61 101 1 2",
        "executar",
        "feedback 202 2 61 7 1202 11 + 4",
        "reverter 202 7 5",
    ] {
        assert_eq!(apply(&mut e, cmd), apply(&mut restored, cmd));
    }
    assert_eq!(e.encode().unwrap(), restored.encode().unwrap());
}
#[test]
fn every_byte_corruption_truncation_and_version_are_rejected() {
    let mut e = setup();
    experience(&mut e, 1, 0);
    feedback(&mut e, 201, 1, "+", 1);
    let bytes = e.encode().unwrap();
    for i in 0..bytes.len() {
        let mut altered = bytes.clone();
        altered[i] ^= 1;
        assert!(SynapticEpisode::decode(&altered).is_err(), "byte {i}");
        assert!(SynapticEpisode::decode(&bytes[..i]).is_err());
    }
    assert!(SynapticEpisode::decode(&vec![0; MAX_FILE + 1]).is_err());
    assert!(SynapticEpisode::decode(b"NRNF1\0\0\0").is_err());
}
fn forged(journal: &str) -> Vec<u8> {
    let h = journal
        .as_bytes()
        .iter()
        .fold(0xcbf29ce484222325u64, |h, b| {
            (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
        });
    let mut b = b"NRNMP05\0".to_vec();
    b.extend(1u32.to_le_bytes());
    b.extend((journal.len() as u32).to_le_bytes());
    b.extend(h.to_le_bytes());
    b.extend(0u64.to_le_bytes());
    b.extend(journal.as_bytes());
    b
}
#[test]
fn checksum_valid_invalid_topology_history_and_embedded_io_are_rejected() {
    for journal in [
        "novo 1 1 1 1 0 3 2 16 64\nneuronio 101 1 4\nsinapse 1 101 101 0 1",
        "novo 1 1 1 1 0 3 2 17 64",
        "novo 1 1 1 1 0 3 2 16 65",
        "novo 1 1 1 1 0 3 2 16 64\nneuronio 101 -1 4",
        "salvar /tmp/forged-never-created",
    ] {
        assert!(SynapticEpisode::decode(&forged(journal)).is_err());
    }
}
#[test]
fn disk_failures_do_not_overwrite_and_process_resume_matches_uninterrupted() {
    let root = std::env::temp_dir().join(format!("neurons-mp05-io-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let path = root.join("snapshot.mp05");
    let mut e = setup();
    experience(&mut e, 1, 0);
    feedback(&mut e, 201, 1, "+", 1);
    e.save(&path).unwrap();
    let disk = std::fs::read(&path).unwrap();
    assert!(e.save(&path).is_err());
    assert_eq!(disk, std::fs::read(&path).unwrap());
    let pending = root.join("other.mp05.mp05-pending");
    std::fs::write(&pending, b"occupied").unwrap();
    assert!(e.save(&root.join("other.mp05")).is_err());
    assert_eq!(std::fs::read(&pending).unwrap(), b"occupied");
    assert!(e.save(&root.join("absent/snapshot")).is_err());
    assert!(SynapticEpisode::load(&root.join("absent")).is_err());
    std::fs::write(root.join("bad"), b"corrupt").unwrap();
    assert!(SynapticEpisode::load(&root.join("bad")).is_err());
    let expected = [
        apply(&mut e, "experiencia 2 61 101 1 2"),
        apply(&mut e, "executar"),
        apply(&mut e, "estado"),
    ];
    let input = format!(
        "restaurar {}\nexperiencia 2 61 101 1 2\nexecutar\nestado\nsair\n",
        path.display()
    );
    let mut child = Command::new(env!("CARGO_BIN_EXE_neurons"))
        .arg("session-mp05")
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
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(!text.contains("Erro:"));
    for x in expected {
        assert!(text.contains(&x));
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn saturated_no_change_feedback_is_consumed_and_wrong_source_connection_is_rejected() {
    let mut e = setup_with(2, 2, 64);
    experience(&mut e, 1, 0);
    feedback(&mut e, 201, 1, "+", 1);
    experience(&mut e, 2, 2);
    feedback(&mut e, 202, 2, "+", 4);
    assert_eq!(weight(&e), 2);
    assert!(e.updates()[1].update.saturated);
    assert!(!e.updates()[1].update.changed());
    let restored = SynapticEpisode::decode(&e.encode().unwrap()).unwrap();
    assert_eq!(restored.updates(), e.updates());
    let before = e.state();
    assert!(e.execute("feedback 202 2 61 7 1202 11 + 5").is_err());
    assert_eq!(before, e.state());
    let mut e = setup();
    apply(&mut e, "neuronio 107 1 4");
    apply(&mut e, "sinapse 12 107 103 0 1");
    experience(&mut e, 1, 0);
    let before = e.state();
    assert!(e.execute("feedback 201 1 61 7 1201 12 + 1").is_err());
    assert_eq!(before, e.state());
}
#[test]
fn propagation_errors_and_invalid_grants_never_authorize_feedback() {
    let mut e = setup();
    let before = e.state();
    for cmd in [
        "autorizar 7 0 1000",
        "autorizar 8 10 0",
        "autorizar 0 0 1000",
    ] {
        assert!(e.execute(cmd).is_err());
        assert_eq!(before, e.state());
    }
    let mut e = SynapticEpisode::new();
    for c in [
        "novo 41 61 81 1 0 3 2 1 64",
        "neuronio 101 1 4",
        "neuronio 103 2 4",
        "neuronio 107 2 4",
        "sinapse 11 101 103 1 1",
        "sinapse 12 101 107 1 1",
        "autorizar 7 0 1000",
    ] {
        apply(&mut e, c);
    }
    experience(&mut e, 1, 0);
    assert!(!e.experiences()[0].valid);
    let before = e.state();
    assert!(e.execute("feedback 201 1 61 7 1201 11 + 2").is_err());
    assert_eq!(before, e.state());
    assert_eq!(
        SynapticEpisode::decode(&e.encode().unwrap())
            .unwrap()
            .state(),
        before
    );
}
#[test]
fn malformed_utf8_line_and_exit_preserve_runner_bounds() {
    use neurons::session::synaptic::run;
    use std::io::Cursor;
    for (data, expected) in [
        (vec![255, b'\n'], "UTF-8 inválido"),
        (vec![b'x'; 257], "linha excedida"),
        (
            b"invalido\nsair\nnovo 1 1 1 1 0 3 2 16 64\n".to_vec(),
            "Erro:",
        ),
    ] {
        let mut out = vec![];
        run(Cursor::new(data), &mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        assert!(text.contains(expected));
        assert!(!text.contains("Circuito e regra declarados"));
    }
}

#[test]
fn nonzero_potential_is_retained_and_changes_the_next_response_after_restore() {
    let mut original = setup_with(3, 1, 64);
    experience(&mut original, 1, 0);
    feedback(&mut original, 201, 1, "+", 1);
    experience(&mut original, 2, 2);
    let target = neurons::event::NodeId::new(103).unwrap();
    assert_eq!(
        original
            .circuit()
            .unwrap()
            .neuron_state(target)
            .unwrap()
            .potential,
        1
    );
    assert_eq!(destination(&original), 0);
    let mut restored = SynapticEpisode::decode(&original.encode().unwrap()).unwrap();
    assert_eq!(
        restored.circuit().unwrap().neuron_state(target),
        original.circuit().unwrap().neuron_state(target)
    );
    for e in [&mut original, &mut restored] {
        experience(e, 3, 4);
        assert_eq!(destination(e), 1);
        assert_eq!(
            e.circuit().unwrap().neuron_state(target).unwrap().potential,
            0
        );
    }
    assert_eq!(original.state(), restored.state());
    assert_eq!(original.encode().unwrap(), restored.encode().unwrap());
}
