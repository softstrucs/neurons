use std::process::Command;

#[test]
fn help_lists_available_commands() {
    let output = Command::new(env!("CARGO_BIN_EXE_neurons"))
        .arg("--help")
        .output()
        .expect("a CLI deve iniciar");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("a saída deve ser UTF-8");
    assert!(stdout.contains("doctor"));
    assert!(stdout.contains("Uso: neurons"));
}

#[test]
fn doctor_reports_bootstrap_state() {
    let output = Command::new(env!("CARGO_BIN_EXE_neurons"))
        .arg("doctor")
        .output()
        .expect("a CLI deve iniciar");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("a saída deve ser UTF-8");
    assert!(stdout.contains("Neurons 0.1.0"));
    assert!(stdout.contains("Sistema:"));
    assert!(stdout.contains("núcleo ainda não implementado"));
}

#[test]
fn unknown_command_fails() {
    let output = Command::new(env!("CARGO_BIN_EXE_neurons"))
        .arg("desconhecido")
        .output()
        .expect("a CLI deve iniciar");

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8(output.stderr)
        .expect("a saída de erro deve ser UTF-8")
        .contains("Argumentos inválidos"));
}
