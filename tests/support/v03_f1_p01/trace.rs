//! Rastreio opcional para o único ponto de criação de filhos de restauração.
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::process::ExitStatusExt;
use std::process::{Child, Command, Output, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn now_ns() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if !matches!(self.0.try_wait(), Ok(Some(_))) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

fn identity(pid: u32) -> (u32, String, String) {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).expect("stat indisponível");
    let rest = stat.rsplit_once(')').expect("stat inválido").1;
    let fields: Vec<_> = rest.split_whitespace().collect();
    let ppid = fields[1].parse().expect("ppid inválido");
    let start = fields[19].to_owned();
    let group = std::fs::read_to_string(format!("/proc/{pid}/cgroup"))
        .expect("cgroup indisponível")
        .lines()
        .find_map(|x| x.strip_prefix("0::"))
        .expect("cgroup v2 ausente")
        .to_owned();
    assert!(!group.contains('|') && !group.contains('\n'));
    (ppid, start, group)
}

fn valid_ready(
    line: &str,
    run: &str,
    child: &str,
    pid: u32,
    ppid: u32,
    ticks: &str,
    group: &str,
) -> bool {
    let fields: Vec<_> = line.trim_end().split('|').collect();
    fields
        == [
            "V03TRACE_READY",
            run,
            child,
            &pid.to_string(),
            &ppid.to_string(),
            ticks,
            group,
        ]
}

fn valid_release(line: &str, run: &str, child: &str, pid: u32, ticks: &str) -> bool {
    line.trim_end() == format!("V03TRACE_GO|{run}|{child}|{pid}|{ticks}")
}

pub fn child_gate() {
    let Ok(run) = std::env::var("NEURONS_V03_TRACE_RUN") else {
        return;
    };
    let child_id = std::env::var("NEURONS_V03_TRACE_CHILD").expect("identidade ausente");
    let pid = std::process::id();
    let (ppid, start, group) = identity(pid);
    println!("V03TRACE_READY|{run}|{child_id}|{pid}|{ppid}|{start}|{group}");
    std::io::stdout()
        .flush()
        .expect("falha ao informar identidade");
    let mut command = String::new();
    std::io::stdin()
        .read_line(&mut command)
        .expect("falha no canal de liberação");
    assert!(
        valid_release(&command, &run, &child_id, pid, &start),
        "liberação inválida"
    );
}

pub fn test_gate() {
    let Ok(run) = std::env::var("NEURONS_V03_TRACE_RUN") else {
        return;
    };
    let pid = std::process::id();
    let (ppid, ticks, group) = identity(pid);
    println!("V03TEST_READY|{run}|{pid}|{ppid}|{ticks}|{group}");
    std::io::stdout()
        .flush()
        .expect("falha ao informar processo de teste");
    let mut command = String::new();
    std::io::stdin()
        .read_line(&mut command)
        .expect("falha no canal do teste");
    assert!(
        command.trim_end() == format!("V03TEST_GO|{run}|{pid}|{ticks}"),
        "liberação do teste inválida"
    );
}

pub fn run_child(cmd: &mut Command, partition: char, repetition: u8, case: u8) -> Output {
    let Ok(run) = std::env::var("NEURONS_V03_TRACE_RUN") else {
        return cmd.output().expect("filho não iniciado");
    };
    assert!(run.len() == 32 && run.bytes().all(|x| x.is_ascii_hexdigit()));
    let child_id = format!("{partition}-{repetition}-{case}");
    let expected = std::env::var("NEURONS_V03_TRACE_CGROUP").expect("cgroup esperado ausente");
    let parent_pid = std::process::id();
    let (_, _, parent_group) = identity(parent_pid);
    assert_eq!(parent_group, expected, "pai fora do cgroup");
    let begin = now_ns();
    let started = Instant::now();
    let mut guard = ChildGuard(
        cmd.env("NEURONS_V03_TRACE_CHILD", &child_id)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("filho não iniciado"),
    );
    let child = &mut guard.0;
    let pid = child.id();
    let mut reader = BufReader::new(child.stdout.take().unwrap());
    let result = (|| {
        let mut captured = String::new();
        loop {
            let mut line = String::new();
            assert!(
                reader.read_line(&mut line).expect("falha na identidade") > 0,
                "identidade ausente"
            );
            captured.push_str(&line);
            if let Some(pos) = line.find("V03TRACE_READY|") {
                let (ppid, ticks, group) = identity(pid);
                assert!(
                    valid_ready(
                        &line[pos..],
                        &run,
                        &child_id,
                        pid,
                        parent_pid,
                        &ticks,
                        &group
                    ),
                    "identidade divergente ou incompleta"
                );
                assert_eq!(ppid, parent_pid);
                assert_eq!(group, expected);
                let entered_ns = now_ns();
                writeln!(
                    child.stdin.take().unwrap(),
                    "V03TRACE_GO|{run}|{child_id}|{pid}|{ticks}"
                )
                .expect("falha na liberação");
                reader
                    .read_to_string(&mut captured)
                    .expect("falha na saída");
                assert_eq!(
                    captured.matches("V03TRACE_READY|").count(),
                    1,
                    "identidade duplicada"
                );
                let status = child.wait().expect("falha ao aguardar filho");
                let mut errors = Vec::new();
                child
                    .stderr
                    .take()
                    .unwrap()
                    .read_to_end(&mut errors)
                    .expect("falha na saída de erro");
                let exit_ns = now_ns();
                println!("V03TRACE_RECORD|{run}|{child_id}|restore|{pid}|{parent_pid}|{ticks}|{group}|{begin}|{entered_ns}|{exit_ns}|{}|{}|{}", started.elapsed().as_nanos(), status.code().unwrap_or(-1), status.signal().unwrap_or(0));
                return Output {
                    status,
                    stdout: captured.into_bytes(),
                    stderr: errors,
                };
            }
        }
    })();
    result
}

#[cfg(test)]
mod tests {
    use super::{valid_ready, valid_release};

    #[test]
    fn communication_rejects_missing_or_divergent_identity() {
        assert!(valid_ready(
            "V03TRACE_READY|run|D-0-1|12|7|99|/scope\n",
            "run",
            "D-0-1",
            12,
            7,
            "99",
            "/scope"
        ));
        assert!(!valid_ready(
            "V03TRACE_READY|run|D-0-1|12|7|99",
            "run",
            "D-0-1",
            12,
            7,
            "99",
            "/scope"
        ));
        assert!(!valid_ready(
            "V03TRACE_READY|run|D-0-1|12|7|98|/scope",
            "run",
            "D-0-1",
            12,
            7,
            "99",
            "/scope"
        ));
        assert!(!valid_ready(
            "V03TRACE_READY|run|D-0-1|12|8|99|/scope",
            "run",
            "D-0-1",
            12,
            7,
            "99",
            "/scope"
        ));
        assert!(valid_release(
            "V03TRACE_GO|run|D-0-1|12|99\n",
            "run",
            "D-0-1",
            12,
            "99"
        ));
        assert!(!valid_release("", "run", "D-0-1", 12, "99"));
        assert!(!valid_release(
            "V03TRACE_GO|run|D-0-1|12|98",
            "run",
            "D-0-1",
            12,
            "99"
        ));
    }
}
