//! Rastreio opcional para o único ponto de criação de filhos de restauração.
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::process::ExitStatusExt;
use std::process::{Child, Command, Output, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const CHILD_TIMEOUT: Duration = Duration::from_secs(3);
const MAX_CHANNEL_BYTES: usize = 1_048_576;

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
    let expected = std::env::var("NEURONS_V03_TRACE_CGROUP").expect("cgroup esperado ausente");
    run_child_traced(cmd, partition, repetition, case, &run, &expected)
}

fn run_child_traced(
    cmd: &mut Command,
    partition: char,
    repetition: u8,
    case: u8,
    run: &str,
    expected: &str,
) -> Output {
    assert!(run.len() == 32 && run.bytes().all(|x| x.is_ascii_hexdigit()));
    let child_id = format!("{partition}-{repetition}-{case}");
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
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let (tx, rx) = mpsc::sync_channel(16);
    let output_reader = thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    if tx.send(line).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
    let (error_tx, error_rx) = mpsc::sync_channel(1);
    thread::spawn(move || {
        let mut errors = Vec::new();
        stderr
            .take((MAX_CHANNEL_BYTES + 1) as u64)
            .read_to_end(&mut errors)
            .expect("falha na saída de erro");
        let _ = error_tx.send(errors);
    });
    let result = (|| {
        let mut captured = String::new();
        let mut identity_data = None;
        loop {
            let remaining = CHILD_TIMEOUT
                .checked_sub(started.elapsed())
                .expect("timeout do filho");
            let line = match rx.recv_timeout(remaining) {
                Ok(line) => line,
                Err(mpsc::RecvTimeoutError::Timeout) => panic!("timeout do filho"),
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            };
            assert!(
                captured.len() + line.len() <= MAX_CHANNEL_BYTES,
                "saída excede limite"
            );
            captured.push_str(&line);
            if let Some(pos) = line.find("V03TRACE_READY|") {
                assert!(identity_data.is_none(), "identidade duplicada");
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
                identity_data = Some((ticks, group, entered_ns));
            }
        }
        let (ticks, group, entered_ns) = identity_data.expect("identidade ausente");
        let status = loop {
            if let Some(status) = child.try_wait().expect("falha ao aguardar filho") {
                break status;
            }
            assert!(
                started.elapsed() < CHILD_TIMEOUT,
                "timeout ao aguardar filho"
            );
            thread::sleep(Duration::from_millis(5));
        };
        output_reader.join().expect("falha no leitor de saída");
        let remaining = CHILD_TIMEOUT
            .checked_sub(started.elapsed())
            .expect("timeout ao ler erro do filho");
        let errors = error_rx
            .recv_timeout(remaining)
            .expect("erro ausente ou timeout");
        assert!(
            errors.len() <= MAX_CHANNEL_BYTES,
            "saída de erro excede limite"
        );
        let exit_ns = now_ns();
        println!("V03TRACE_RECORD|{run}|{child_id}|restore|{pid}|{parent_pid}|{ticks}|{group}|{begin}|{entered_ns}|{exit_ns}|{}|{}|{}", started.elapsed().as_nanos(), status.code().unwrap_or(-1), status.signal().unwrap_or(0));
        Output {
            status,
            stdout: captured.into_bytes(),
            stderr: errors,
        }
    })();
    result
}

#[cfg(test)]
mod tests {
    use super::{identity, run_child_traced, valid_ready, valid_release, ChildGuard};
    use std::io::{BufRead, BufReader, Write};
    use std::process::{Command, Stdio};
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

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

    #[test]
    fn synthetic_gate_child() {
        if std::env::var_os("NEURONS_SYNTHETIC_GATE").is_some() {
            super::child_gate();
            println!("SYNTHETIC_WORK_ALLOWED");
        }
    }

    #[test]
    fn safe_gate_rejects_invalid_and_closed_channel() {
        for send_invalid in [true, false] {
            let mut guard = ChildGuard(
                Command::new(std::env::current_exe().unwrap())
                    .args([
                        "--exact",
                        "trace::tests::synthetic_gate_child",
                        "--nocapture",
                    ])
                    .env("NEURONS_SYNTHETIC_GATE", "1")
                    .env("NEURONS_V03_TRACE_RUN", "synthetic")
                    .env("NEURONS_V03_TRACE_CHILD", "synthetic-child")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap(),
            );
            let child = &mut guard.0;
            let stdout = child.stdout.take().unwrap();
            let (tx, rx) = mpsc::channel();
            let reader = std::thread::spawn(move || {
                for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                    if tx.send(line).is_err() {
                        break;
                    }
                }
            });
            let mut seen = String::new();
            while !seen.contains("V03TRACE_READY|") {
                seen.push_str(
                    &rx.recv_timeout(Duration::from_secs(3))
                        .expect("identidade ausente ou timeout"),
                );
            }
            if send_invalid {
                writeln!(child.stdin.as_mut().unwrap(), "invalid").unwrap();
            }
            drop(child.stdin.take());
            let until = Instant::now() + Duration::from_secs(3);
            let status = loop {
                if let Some(status) = child.try_wait().unwrap() {
                    break status;
                }
                assert!(
                    Instant::now() < until,
                    "timeout ao aguardar processo sintético"
                );
                std::thread::sleep(Duration::from_millis(5));
            };
            reader.join().unwrap();
            for line in rx.try_iter() {
                seen.push_str(&line);
            }
            assert!(!status.success());
            assert!(!seen.contains("SYNTHETIC_WORK_ALLOWED"));
            assert!(!std::path::Path::new(&format!("/proc/{}", child.id())).exists());
        }
    }

    #[test]
    fn safe_live_handshake_and_failure_cleanup() {
        let (_, _, group) = identity(std::process::id());
        let run = "0123456789abcdef0123456789abcdef";
        let program = r#"import os,sys,time
p=os.getpid(); parent=os.getppid(); s=open('/proc/self/stat').read().rsplit(')',1)[1].split(); tick=s[19]
cg=next(x[3:] for x in open('/proc/self/cgroup').read().splitlines() if x.startswith('0::'))
open(os.environ['NEURONS_SYNTHETIC_PIDFILE'],'w').write(str(p))
mode=os.environ['NEURONS_SYNTHETIC_MODE']; run=os.environ['NEURONS_SYNTHETIC_RUN']; child='D-0-1'
if mode=='timeout': time.sleep(10); sys.exit(0)
if mode=='early': sys.exit(0)
if mode=='missing': print('no identity',flush=True); sys.exit(0)
if mode=='wrong': child='D-0-2'
if mode=='closed': os.close(0)
ready=f'V03TRACE_READY|{run}|{child}|{p}|{parent}|{tick}|{cg}'
print(ready,flush=True)
if mode=='closed': time.sleep(0.2); sys.exit(0)
answer=sys.stdin.readline()
if answer.strip()!=f'V03TRACE_GO|{run}|{child}|{p}|{tick}': sys.exit(2)
if mode=='duplicate': print(ready,flush=True)
if mode=='stderr': sys.stderr.write('x'*131072); sys.stderr.flush()
print('SYNTHETIC_OK',flush=True)
"#;
        for mode in [
            "valid",
            "stderr",
            "missing",
            "wrong",
            "early",
            "timeout",
            "duplicate",
            "closed",
        ] {
            let pidfile =
                std::env::temp_dir().join(format!("neurons-trace-{}-{mode}", std::process::id()));
            let mut cmd = Command::new("python3");
            cmd.args(["-u", "-c", program])
                .env("NEURONS_SYNTHETIC_PIDFILE", &pidfile)
                .env("NEURONS_SYNTHETIC_MODE", mode)
                .env("NEURONS_SYNTHETIC_RUN", run);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run_child_traced(&mut cmd, 'D', 0, 1, run, &group)
            }));
            let pid: u32 = std::fs::read_to_string(&pidfile).unwrap().parse().unwrap();
            std::fs::remove_file(&pidfile).unwrap();
            assert!(
                !std::path::Path::new(&format!("/proc/{pid}")).exists(),
                "filho não recolhido: {mode}"
            );
            if mode == "valid" || mode == "stderr" {
                assert!(result.unwrap().status.success(), "{mode}");
            } else {
                assert!(result.is_err(), "falha não detectada: {mode}");
            }
        }
    }
}
