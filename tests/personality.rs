use neurons::personality::Style;
use neurons::session::{run, MAX_COMMANDS, MAX_LINE_BYTES, MAX_RECORDS};
use std::io::Cursor;
use std::io::Write;
use std::process::{Command, Stdio};

fn session(input: &[u8], style: Style) -> String {
    let mut output = Vec::new();
    run(Cursor::new(input), &mut output, style).unwrap();
    String::from_utf8(output).unwrap()
}

fn reports(output: &str) -> Vec<&str> {
    output
        .lines()
        .filter(|line| line.starts_with("ReviewReport"))
        .collect()
}

#[test]
fn style_control_preserves_decisions_evidence_and_repetition_in_six_cases() {
    let cases = [
        ("fato 1 1 1 3 1\nconsulta 1 1 3\n", "Respond"),
        (
            "fato 1 1 1 2 1\nfato 2 1 2 3 1\nconsulta 1 1 3\n",
            "Respond",
        ),
        ("consulta 1 1 3\n", "MissingEvidence"),
        (
            "fato 1 1 1 3 1\nfato 2 1 1 3 -1\nconsulta 1 1 3\n",
            "ContradictoryEvidence",
        ),
        (
            "fato 1 2 1 3 1\nconsulta 1 1 3 2\n",
            "revision: Some(MissingEvidence)",
        ),
        (
            "consulta 1 1 3\nfato 1 1 1 3 1\nconsulta 1 1 3\n",
            "Respond",
        ),
    ];
    for (input, expected) in cases {
        let concise = session(input.as_bytes(), Style::Concise);
        let explanatory = session(input.as_bytes(), Style::Explanatory);
        assert!(concise.contains(expected));
        assert_eq!(reports(&concise), reports(&explanatory));
        assert_ne!(concise, explanatory);
        assert_eq!(concise, session(input.as_bytes(), Style::Concise));
        assert_eq!(explanatory, session(input.as_bytes(), Style::Explanatory));
    }
    println!("stage08: 6/6 cenários com decisões e evidências idênticas entre estilos; 12/12 repetições exatas; 0 divergências");
}

#[test]
fn malformed_commands_do_not_create_facts_and_exit_stops_input() {
    let output = session(b"fato 1 1 1 3 2\nfato 0 1 1 3 1\nfato 1 0 1 3 1\nfato 1 1 1 2147483648 1\nconsulta 1 1 1\nconsulta 1 1 3\nsair\nfato 1 1 1 3 1\n", Style::Concise);
    assert_eq!(output.matches("Erro:").count(), 5);
    assert!(output.contains("MissingEvidence"));
    assert!(!output.contains("Registro observado;"));
}

#[test]
fn bounded_lines_commands_and_utf8_do_not_panic() {
    let output = session(&vec![b'x'; MAX_LINE_BYTES + 1], Style::Concise);
    assert!(output.contains("linha excede"));
    let output = session(&[255, b'\n'], Style::Concise);
    assert!(output.contains("UTF-8 inválido"));
    let output = session(
        "invalido\n".repeat(MAX_COMMANDS + 2).as_bytes(),
        Style::Concise,
    );
    assert_eq!(output.matches("Erro:").count(), MAX_COMMANDS);
    assert!(output.contains("Limite de comandos"));
}

#[test]
fn record_capacity_and_duplicate_ids_are_explicit() {
    let mut input = String::from("fato 1 1 1 2 1\nfato 1 1 1 3 1\n");
    for id in 2..=MAX_RECORDS + 1 {
        input.push_str(&format!("fato {id} 1 1 {} 1\n", id + 1));
    }
    let output = session(input.as_bytes(), Style::Concise);
    assert_eq!(output.matches("Registro observado;").count(), MAX_RECORDS);
    assert!(output.contains("DuplicateId"));
    assert!(output.contains("CapacityFull"));
}

#[test]
fn sessions_do_not_share_memory() {
    session(b"fato 1 1 1 3 1\n", Style::Concise);
    assert!(session(b"consulta 1 1 3\n", Style::Concise).contains("MissingEvidence"));
}

#[test]
fn cli_routes_both_styles_to_same_session() {
    for command in ["session", "session-explain"] {
        let mut child = Command::new(env!("CARGO_BIN_EXE_neurons"))
            .arg(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"consulta 1 1 3\nsair\n")
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success());
        let output = String::from_utf8(output.stdout).unwrap();
        assert!(output.contains("gramática restrita"));
        assert!(output.contains("MissingEvidence"));
        if command == "session" {
            assert!(output.contains("Resultado experimental:"));
        } else {
            assert!(output.contains("Avaliei os registros"));
        }
    }
}
