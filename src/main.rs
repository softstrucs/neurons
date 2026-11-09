use std::env;
use std::process::ExitCode;
use std::thread;

const HELP: &str = concat!(
    "Neurons ",
    env!("CARGO_PKG_VERSION"),
    "\n\nUso: neurons [--help | doctor | session | session-explain | session-f1 | session-mp05]\n\nComandos:\n  --help, -h  Mostra esta ajuda\n  doctor      Mostra informações básicas do ambiente\n  session     Sessão curta de gramática restrita, estilo conciso\n  session-explain  Mesma sessão, estilo explicativo\n  session-f1  Sessão integrada limitada: evidências, aquisição, conceitos e checkpoint\n  session-mp05 Episódio sináptico assistido e persistente\n"
);

fn main() -> ExitCode {
    let mut args = env::args().skip(1);

    match (args.next().as_deref(), args.next()) {
        (Some("--help" | "-h"), None) => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        (Some("doctor"), None) => {
            println!("Neurons {}", env!("CARGO_PKG_VERSION"));
            println!("Sistema: {} ({})", env::consts::OS, env::consts::ARCH);
            match thread::available_parallelism() {
                Ok(threads) => println!("Paralelismo disponível: {}", threads.get()),
                Err(error) => println!("Paralelismo disponível: indisponível ({error})"),
            }
            println!("Estado: núcleo local experimental implementado: eventos, circuitos, memória, inferência limitada e metacognição; sem compreensão geral demonstrada.");
            ExitCode::SUCCESS
        }
        (Some("session-mp05"), None) => {
            match neurons::session::synaptic::run(std::io::stdin().lock(), std::io::stdout().lock())
            {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("Erro de sessão MP05: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        (Some("session-f1"), None) => {
            match neurons::session::f1::run(std::io::stdin().lock(), std::io::stdout().lock()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("Erro de sessão F1: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        (Some(command @ ("session" | "session-explain")), None) => {
            let style = if command == "session" {
                neurons::personality::Style::Concise
            } else {
                neurons::personality::Style::Explanatory
            };
            match neurons::session::run(std::io::stdin().lock(), std::io::stdout().lock(), style) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("Erro de sessão: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        _ => {
            eprint!("Argumentos inválidos.\n\n{HELP}");
            ExitCode::FAILURE
        }
    }
}
