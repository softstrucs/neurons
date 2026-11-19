//! Aceitação pelo executável real; apenas ambientes públicos sintéticos.
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
static NEXT: AtomicU32 = AtomicU32::new(1);
struct Sandbox(PathBuf);
impl Sandbox {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "neurons-runtime-cli-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn store(&self) -> PathBuf {
        self.0.join("store")
    }
    fn call(&self, script: &str) -> Output {
        call(&format!(
            "armazenamento {}\n{script}",
            self.store().display()
        ))
    }
    fn generation(&self) -> PathBuf {
        fs::read_dir(self.store())
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "bin"))
            .max()
            .unwrap()
    }
}
impl Drop for Sandbox {
    fn drop(&mut self) {
        // Só o diretório exclusivo criado pelo próprio teste; nunca entrada recebida.
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn call(script: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_neurons"))
        .arg("session-runtime")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(script.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}
fn text(o: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    )
}
fn ok(o: &Output) {
    assert!(o.status.success(), "{}", text(o));
}
const CONFIG:&str="configurar 501 1 61 7 7 20 601 8 128 32\nambiente 11\ntransicao 11 7 20\ntransicao 20 7 77\ntransicao 7777 7 20\ntransicao 500 7 20\ntransicao 600 7 77\natributos 11 111000\natributos 20 000000\natributos 7777 101001\natributos 500 100000\natributos 600 000001\nfixar\n";
const RIGHTS: &str = "autorizar observar\nautorizar consultar\nautorizar agir\n";
const TEACHING: &str = "autorizar avaliar\nautorizar apresentar\nautorizar admitir\n";
fn acquired() -> String {
    format!("{CONFIG}{RIGHTS}{TEACHING}autorizar acao 100\nobservar\nagir\nautorizar acao 101\nobservar\nagir\nensinar 111100 001110\nepisodio 2 500\nobjetivo 90 20\n")
}
fn investigate() -> String {
    format!("{RIGHTS}{TEACHING}autorizar investigar\nautorizar acao 300\nautorizar acao 301\noferta 500\noferta 600\ninvestigar\n")
}
fn ready(s: &Sandbox) {
    let o = s.call(&format!(
        "{}{}episodio 3 7777\nobjetivo 91 20\nsair\n",
        acquired(),
        investigate()
    ));
    ok(&o);
    assert!(text(&o).contains("Some((Learned"), "{}", text(&o));
}
#[test]
fn binary_help_exposes_local_runtime_session() {
    let o = Command::new(env!("CARGO_BIN_EXE_neurons"))
        .arg("--help")
        .output()
        .unwrap();
    ok(&o);
    assert!(text(&o).contains("session-runtime"));
}
#[test]
fn real_binary_investigates_learns_restores_and_applies_to_novel_attributes() {
    let s = Sandbox::new();
    ready(&s);
    let o = s.call(&format!(
        "restaurar\nestado\n{RIGHTS}autorizar acao 400\nexecutar 1\nrecursos\nsair\n"
    ));
    ok(&o);
    let t = text(&o);
    assert!(t.contains("adapt: false"));
    assert!(t.contains("Aplicação: Reached"));
    assert!(t.contains("ações totais 4"));
    assert!(t.contains("conferência Some(Some(true))"));
    let o = s.call("restaurar\nestado\nrecursos\nsair\n");
    ok(&o);
    let t = text(&o);
    assert!(t.contains("estado 20; ações totais 4; versão 3"));
    assert!(t.contains("read_knowledge: false"));
    assert!(t.contains("reserved_conceptual_queries: 2"));
    assert!(t.contains("intervenções 1"));
}
#[test]
fn control_without_experience_does_not_reach_novel_goal() {
    let s = Sandbox::new();
    let o = s.call(&format!(
        "{CONFIG}objetivo 90 20\n{RIGHTS}autorizar acao 100\nexecutar 1\nestado\nsair\n"
    ));
    ok(&o);
    let t = text(&o);
    assert!(!t.contains("Aplicação: Reached"));
    assert!(t.contains("ações totais 0; versão 0"));
}
#[test]
fn ambiguous_experience_without_investigation_remains_insufficient() {
    let s = Sandbox::new();
    let o = s.call(&format!(
        "{}episodio 3 7777\nobjetivo 91 20\n{RIGHTS}autorizar acao 400\nexecutar 1\nestado\nsair\n",
        acquired()
    ));
    ok(&o);
    let t = text(&o);
    assert!(!t.contains("Aplicação: Reached"));
    assert!(t.contains("ações totais 2; versão 2"));
}
#[test]
fn selection_requires_independent_investigation_authority() {
    let s = Sandbox::new();
    ok(&s.call(&acquired()));
    let o = s.call(&format!(
        "restaurar\n{RIGHTS}oferta 500\nautorizar acao 300\ninvestigar\nestado\nsair\n"
    ));
    assert!(!o.status.success());
    assert!(text(&o).contains("investigação não autorizada"));
    assert!(text(&o).contains("ações totais 2; versão 2"));
}
#[test]
fn observation_requires_runtime_authority() {
    let s = Sandbox::new();
    let o = s.call(&format!("{CONFIG}observar\nestado\nsair\n"));
    assert!(!o.status.success());
    assert!(text(&o).contains("atributos sem autorização"));
    assert!(text(&o).contains("ações totais 0"));
}
#[test]
fn action_without_permit_does_not_change_environment() {
    let s = Sandbox::new();
    let o = s.call(&format!("{CONFIG}{RIGHTS}observar\nagir\nestado\nsair\n"));
    assert!(!o.status.success());
    assert!(text(&o).contains("ação sem concessão"));
    assert!(text(&o).contains("estado 11; ações totais 0"));
}
#[test]
fn teaching_requires_all_three_permissions() {
    for missing in ["avaliar", "apresentar", "admitir"] {
        let s = Sandbox::new();
        let rights = TEACHING
            .lines()
            .filter(|l| !l.ends_with(missing))
            .collect::<Vec<_>>()
            .join("\n");
        let o=s.call(&format!("{CONFIG}{RIGHTS}{rights}\nautorizar acao 100\nobservar\nagir\nensinar 111100 001110\nestado\nsair\n"));
        assert!(!o.status.success());
        assert!(text(&o).contains("concessões independentes"));
        assert!(text(&o).contains("ações totais 1; versão 0"));
    }
}
#[test]
fn consumed_and_unconsumed_issued_ids_are_not_regranted_after_restart() {
    let s = Sandbox::new();
    let o = s.call(&format!(
        "{CONFIG}{RIGHTS}autorizar acao 100\nobservar\nagir\nautorizar acao 101\nsair\n"
    ));
    ok(&o);
    let o = s.call("restaurar\nautorizar acao 100\nautorizar acao 101\nestado\nsair\n");
    assert!(!o.status.success());
    assert_eq!(text(&o).matches("Erro: concessão repetida").count(), 2);
    assert!(text(&o).contains("ações totais 1"));
}
#[test]
fn restore_does_not_reopen_consumed_predictive_budget() {
    let s = Sandbox::new();
    ready(&s);
    ok(&s.call(&format!(
        "restaurar\n{RIGHTS}autorizar acao 400\nexecutar 1\nsair\n"
    )));
    let o = s.call(&format!(
        "restaurar\n{RIGHTS}autorizar acao 401\nexecutar 1\nrecursos\nsair\n"
    ));
    assert!(!o.status.success());
    assert!(text(&o).contains("autoridade de planejamento incompatível"));
    assert!(text(&o).contains("intervenções 1"));
}
#[test]
fn insufficient_observation_budget_is_retained_after_restore() {
    let s = Sandbox::new();
    let config = CONFIG.replacen("601 8 128 32", "601 0 128 32", 1);
    let o = s.call(&format!("{config}{RIGHTS}observar\nrecursos\nsair\n"));
    assert!(!o.status.success());
    assert!(!text(&o).contains("Atributos observados com procedência"));
    let o = s.call("restaurar\nrecursos\nsair\n");
    ok(&o);
    assert!(text(&o).contains("observations: 0"));
}
#[test]
fn semantic_contradiction_suspends_transfer_without_erasing_history() {
    let s = Sandbox::new();
    let initial = acquired().replace("transicao 500 7 20", "transicao 500 7 77");
    let o=s.call(&format!("{initial}{}episodio 3 7777\nobjetivo 91 20\n{RIGHTS}autorizar acao 400\nexecutar 1\nestado\nsair\n",investigate()));
    ok(&o);
    let t = text(&o);
    assert!(!t.contains("Aplicação: Reached"));
    assert!(t.contains("ações totais 3"));
}
#[test]
fn mutation_is_forbidden_before_explicit_storage() {
    let o = call(CONFIG);
    assert!(!o.status.success());
    assert!(text(&o).contains("abra armazenamento"));
}
#[test]
fn pending_operation_blocks_restart_without_replaying() {
    let s = Sandbox::new();
    ok(&s.call(CONFIG));
    let generation = s.generation();
    let before = fs::read(&generation).unwrap();
    fs::write(s.store().join("pendente"), b"simulated crash before commit").unwrap();
    let o = s.call("restaurar\nsair\n");
    assert!(!o.status.success());
    assert!(text(&o).contains("execução incerta"));
    assert_eq!(before, fs::read(generation).unwrap());
    assert!(s.store().join("pendente").exists());
}
#[test]
fn a_stale_lock_is_not_removed_automatically() {
    let s = Sandbox::new();
    ok(&s.call(CONFIG));
    let lock = s.store().join("sessao.lock");
    fs::write(&lock, b"unknown previous owner").unwrap();
    let o = s.call("restaurar\nsair\n");
    assert!(!o.status.success());
    assert!(text(&o).contains("lock exclusivo"));
    assert_eq!(fs::read(lock).unwrap(), b"unknown previous owner");
}
#[test]
fn corrupt_or_truncated_latest_generation_never_falls_back_silently() {
    for truncate in [false, true] {
        let s = Sandbox::new();
        ok(&s.call(CONFIG));
        let path = s.generation();
        let mut b = fs::read(&path).unwrap();
        if truncate {
            b.truncate(b.len() / 2)
        } else {
            let last = b.len() - 1;
            b[last] ^= 1;
        }
        fs::write(&path, &b).unwrap();
        let o = s.call("restaurar\nsair\n");
        assert!(!o.status.success());
        assert!(text(&o).contains("integridade administrativa"));
        assert_eq!(fs::read(path).unwrap(), b);
    }
}
#[test]
fn existing_documents_and_unexpected_storage_entries_are_not_overwritten() {
    let s = Sandbox::new();
    fs::create_dir(s.store()).unwrap();
    let p = s.store().join("scientific.md");
    fs::write(&p, b"preserve").unwrap();
    let o = s.call(CONFIG);
    assert!(!o.status.success());
    assert!(text(&o).contains("arquivo inesperado"));
    assert_eq!(fs::read(p).unwrap(), b"preserve");
    assert!(!s.store().join("sessao.lock").exists());
}
#[test]
fn conflicting_generations_cannot_be_reconfigured_in_place() {
    let s = Sandbox::new();
    ok(&s.call(CONFIG));
    let p = s.generation();
    let b = fs::read(&p).unwrap();
    let o = s.call(CONFIG);
    assert!(!o.status.success());
    assert!(text(&o).contains("restaure; não reconfigure"));
    assert_eq!(fs::read(p).unwrap(), b);
}
#[test]
fn goal_and_fixed_environment_cannot_be_silently_replaced() {
    let s = Sandbox::new();
    let o = s.call(&format!(
        "{CONFIG}objetivo 90 20\nobjetivo 91 20\ntransicao 11 7 99\nestado\nsair\n"
    ));
    assert!(!o.status.success());
    assert!(text(&o).contains("objetivo já fixado"));
    assert!(text(&o).contains("configuração já fixada"));
    assert!(text(&o).contains("ações totais 0"));
}
#[cfg(unix)]
#[test]
fn storage_rejects_symlinks_and_creates_private_files() {
    use std::os::unix::{fs::symlink, fs::PermissionsExt};
    let s = Sandbox::new();
    ok(&s.call(CONFIG));
    assert_eq!(
        fs::metadata(s.generation()).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let link = s.0.join("alias");
    symlink(s.store(), &link).unwrap();
    let o = call(&format!("armazenamento {}\nrestaurar\n", link.display()));
    assert!(!o.status.success());
    assert!(text(&o).contains("link simbólico recusado"));
}
#[test]
fn two_step_observed_plan_reuses_existing_planner() {
    let s = Sandbox::new();
    let config="configurar 501 1 61 7 7 20 601 8 128 32\nambiente 0\ntransicao 0 7 10\ntransicao 10 7 20\natributos 0 000000\natributos 10 100000\nfixar\n";
    let o=s.call(&format!("{config}{RIGHTS}{TEACHING}autorizar acao 100\nobservar\nagir\nautorizar acao 101\nobservar\nagir\nensinar 111100 001110\nepisodio 2 0\nobjetivo 90 20\n{RIGHTS}autorizar acao 200\nautorizar acao 201\nplanejar 2\nexecutar 2\nestado\nsair\n"));
    ok(&o);
    let t = text(&o);
    assert!(t.contains("Plano observado: Supported"));
    assert!(t.contains("Aplicação: Reached"));
    assert!(t.contains("ações totais 4"));
}
#[test]
fn malformed_bounded_input_is_rejected_without_panic() {
    let s = Sandbox::new();
    let o = s.call("configurar 0 0 0 0 0 0 0 999 999 999\nsair\n");
    assert!(!o.status.success());
    assert!(!text(&o).contains("panicked"));
    let o = call(&format!("{}\n", "x".repeat(1025)));
    assert!(!o.status.success());
    assert!(text(&o).contains("linha excedida"));
}
#[test]
fn generation_gaps_are_rejected() {
    let s = Sandbox::new();
    ok(&s.call(CONFIG));
    let first = s.store().join("geracao-00000001.bin");
    fs::rename(&first, s.store().join("geracao-00000255.bin")).unwrap();
    let o = s.call("restaurar\n");
    assert!(!o.status.success());
    assert!(text(&o).contains("sequência de gerações incompleta"));
}

#[test]
fn changing_private_outcome_changes_learning_but_not_prior_selection() {
    let mut reports = vec![];
    for target in [20, 77] {
        let s = Sandbox::new();
        let initial =
            acquired().replace("transicao 500 7 20", &format!("transicao 500 7 {target}"));
        let o = s.call(&format!("{initial}{}estado\nsair\n", investigate()));
        ok(&o);
        let t = text(&o);
        let report = t
            .lines()
            .find(|l| l.starts_with("Sondas "))
            .unwrap()
            .to_owned();
        assert!(report.contains("Some(50)"));
        reports.push(report);
    }
    assert_ne!(
        reports[0], reports[1],
        "só consequência observada muda revisão"
    );
}
#[test]
fn investigation_without_admission_does_not_persist_its_provisional_filter_as_learning() {
    let s = Sandbox::new();
    ok(&s.call(&acquired()));
    let script = investigate().replace(TEACHING, "");
    let o = s.call(&format!("restaurar\n{script}estado\nsair\n"));
    ok(&o);
    let t = text(&o);
    assert!(t.contains("Some((Observed"));
    assert!(t.contains("ações totais 3; versão 2"));
    let o = s.call("restaurar\nestado\nsair\n");
    ok(&o);
    assert!(text(&o).contains("versão 2"));
}
#[test]
fn nondiscriminating_offer_preserves_ambiguity_and_cursor_between_processes() {
    let s = Sandbox::new();
    ok(&s.call(&acquired()));
    let o=s.call(&format!("restaurar\n{RIGHTS}autorizar investigar\nautorizar acao 300\noferta 600\ninvestigar\nestado\nsair\n"));
    ok(&o);
    let t = text(&o);
    assert!(t.contains("NoDiscriminatingOffer"));
    assert!(t.contains("ações totais 2; versão 2"));
    let o = s.call("restaurar\nsalvar\nestado\nsair\n");
    ok(&o);
    assert!(text(&o).contains("ações totais 2; versão 2"));
}
#[test]
fn aggregate_action_ceiling_survives_explicit_new_episodes() {
    let s = Sandbox::new();
    let initial = CONFIG.replace("11 111000", "11 100000");
    let mut script = initial;
    for e in 1..=4 {
        if e > 1 {
            script.push_str(&format!("episodio {e} 11\n"));
        }
        script.push_str(&format!("{RIGHTS}{TEACHING}autorizar acao {}\nobservar\nagir\nautorizar acao {}\nobservar\nagir\nensinar 111100 001110\n",100+e*10,101+e*10));
    }
    script.push_str("episodio 5 11\nsair\n");
    ok(&s.call(&script));
    let o = s.call(&format!(
        "restaurar\n{RIGHTS}autorizar acao 999\nobservar\nagir\nestado\nsair\n"
    ));
    assert!(!o.status.success());
    let t = text(&o);
    assert!(t.contains("limite agregado de ações atingido"));
    assert!(t.contains("estado 11; ações totais 8"));
}
#[test]
fn failed_generation_write_keeps_intent_and_prevents_continuation() {
    use std::io::BufRead;
    let s = Sandbox::new();
    let mut child = Command::new(env!("CARGO_BIN_EXE_neurons"))
        .arg("session-runtime")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    writeln!(
        input,
        "armazenamento {}\nconfigurar 501 1 61 7 7 20 601 8 128 32",
        s.store().display()
    )
    .unwrap();
    input.flush().unwrap();
    let mut output = std::io::BufReader::new(child.stdout.take().unwrap());
    loop {
        let mut line = String::new();
        assert!(output.read_line(&mut line).unwrap() > 0);
        if line.contains("Runtime configurado sem autoridade") {
            break;
        }
    }
    let original = fs::read(s.generation()).unwrap();
    // Falha real de create_new no destino exclusivo da próxima geração.
    fs::create_dir(s.store().join("geracao-00000002.bin")).unwrap();
    writeln!(input, "ambiente 11").unwrap();
    drop(input);
    let mut remainder = String::new();
    std::io::Read::read_to_string(&mut output, &mut remainder).unwrap();
    assert!(!child.wait().unwrap().success());
    assert!(remainder.contains("FATAL"));
    assert!(s.store().join("pendente").exists());
    assert_eq!(
        fs::read(s.store().join("geracao-00000001.bin")).unwrap(),
        original
    );
    let o = s.call("restaurar\nsair\n");
    assert!(!o.status.success());
    assert!(!text(&o).contains("Restaurado sem autoridade"));
}

#[test]
fn swapped_generations_are_rejected_even_when_each_packet_has_a_valid_checksum() {
    let s = Sandbox::new();
    ok(&s.call(CONFIG));
    let p = s.generation();
    let latest = fs::read(&p).unwrap();
    let first = s.store().join("geracao-00000001.bin");
    let old = fs::read(&first).unwrap();
    fs::write(&p, &old).unwrap();
    let o = s.call("restaurar\nsair\n");
    assert!(!o.status.success());
    assert!(text(&o).contains("encadeamento incompatível"));
    fs::write(&p, latest).unwrap();
    fs::write(first, &old[..old.len() / 2]).unwrap();
    let o = s.call("restaurar\nsair\n");
    assert!(!o.status.success());
    assert!(!text(&o).contains("Restaurado sem autoridade"));
}

#[test]
fn command_limit_reports_interruption_instead_of_silent_success() {
    let o = call(&"ajuda\n".repeat(257));
    assert!(!o.status.success());
    assert!(text(&o).contains("limite de 256 entradas atingido"));
}
