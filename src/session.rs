//! Adaptador textual restrito, volátil e limitado; sem interpretação de linguagem natural.
use std::io::{self, BufRead, Read, Write};

use crate::cognition::{Hypothesis, TwoHopTask};
use crate::engram::{Cue, Engram, EngramId, EngramStore, RecordStatus};
use crate::event::NodeId;
use crate::metacognition::Monitor;
use crate::personality::Style;

pub const MAX_COMMANDS: usize = 32;
pub const MAX_LINE_BYTES: usize = 256;
pub const MAX_RECORDS: usize = 16;

/// Entradas e saídas são adaptadores; toda inferência usa o núcleo existente.
pub fn run<R: BufRead, W: Write>(mut input: R, mut output: W, style: Style) -> io::Result<()> {
    let mut store =
        EngramStore::new(MAX_RECORDS).map_err(|e| io::Error::other(format!("{e:?}")))?;
    let task =
        TwoHopTask::new(MAX_RECORDS, 32, 64).map_err(|e| io::Error::other(format!("{e:?}")))?;
    let monitor = Monitor::new(task, 2).map_err(|e| io::Error::other(format!("{e:?}")))?;
    writeln!(output, "Sessão experimental; gramática restrita, sem linguagem natural ou persistência.\nfato ID CONTEXTO ORIGEM DESTINO VALOR(1|-1)\nconsulta CONTEXTO ORIGEM DESTINO [CONTEXTO_ALTERNATIVO]\nsair\nLimites: 32 comandos, 256 bytes por linha, 16 registros, 2 tentativas por consulta.")?;
    for _ in 0..MAX_COMMANDS {
        let mut bytes = Vec::new();
        let count = input
            .by_ref()
            .take((MAX_LINE_BYTES + 1) as u64)
            .read_until(b'\n', &mut bytes)?;
        if count == 0 {
            return Ok(());
        }
        if count > MAX_LINE_BYTES {
            writeln!(output, "Erro: linha excede o limite; sessão encerrada.")?;
            return Ok(());
        }
        let Ok(line) = std::str::from_utf8(&bytes) else {
            writeln!(output, "Erro: UTF-8 inválido.")?;
            continue;
        };
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["sair"] => return Ok(()),
            ["fato", id, context, from, to, value] => {
                let record = (|| -> Result<Engram, String> {
                    let id = EngramId::new(number(id)?).map_err(|e| format!("{e:?}"))?;
                    let from = node(from)?;
                    let to = number(to)?;
                    if to == 0 || to > i32::MAX as u32 || to == from.get() {
                        return Err("destino inválido".into());
                    }
                    let value = match *value {
                        "1" => 1,
                        "-1" => -1,
                        _ => return Err("valor deve ser 1 ou -1".into()),
                    };
                    let cue = Cue::new(number(context)?, from, to as i32)
                        .map_err(|e| format!("{e:?}"))?;
                    Engram::new(id, cue, value, Some(from), 0, None, RecordStatus::Observed)
                        .map_err(|e| format!("{e:?}"))
                })();
                match record.and_then(|record| store.record(record).map_err(|e| format!("{e:?}"))) {
                    Ok(()) => writeln!(
                        output,
                        "Registro observado; não constitui verificação externa."
                    )?,
                    Err(error) => writeln!(output, "Erro: {error}")?,
                }
            }
            ["consulta", context, from, to, rest @ ..] if rest.len() <= 1 => {
                let result = (|| -> Result<_, String> {
                    let hypothesis = Hypothesis::new(number(context)?, node(from)?, node(to)?)
                        .map_err(|e| format!("{e:?}"))?;
                    let candidates = rest
                        .iter()
                        .map(|s| number(s))
                        .collect::<Result<Vec<_>, _>>()?;
                    monitor
                        .review(&store, hypothesis, &candidates)
                        .map_err(|e| format!("{e:?}"))
                })();
                match result {
                    Ok(report) => write!(output, "{}", style.render(&report))?,
                    Err(error) => writeln!(output, "Erro: {error}")?,
                }
            }
            _ => writeln!(output, "Erro: comando inválido; use a gramática informada.")?,
        }
    }
    writeln!(output, "Limite de comandos atingido; sessão encerrada.")
}

fn number(value: &str) -> Result<u32, String> {
    value.parse().map_err(|_| "inteiro inválido".into())
}

fn node(value: &str) -> Result<NodeId, String> {
    NodeId::new(number(value)?).map_err(|e| format!("{e:?}"))
}
