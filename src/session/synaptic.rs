//! Adaptador textual MP05; usa o coordenador e o Circuit compartilhados.
use crate::integration::synaptic::SynapticEpisode;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
pub const HELP:&str="MP05 — adaptação sináptica assistida; sem aprendizagem autônoma ou generalização.
novo CIRCUITO CONTEXTO REGRA VERSAO MIN MAX PASSO FILA CICLO
neuronio ID LIMIAR LIMITE
sinapse ID ORIGEM DESTINO PESO ATRASO (topologia acíclica)
autorizar FONTE INICIO FIM (declaração sintética; não autenticação)
experiencia ID CONTEXTO ORIGEM CARGA INSTANTE
executar (até 64 eventos/ciclo; repetir apenas enquanto pendente)
feedback ID EXPERIENCIA CONTEXTO FONTE EVIDENCIA SINAPSE +|- INSTANTE
reverter FEEDBACK FONTE INSTANTE
estado / historico / salvar CAMINHO / restaurar CAMINHO / ajuda / sair
Limites: 4 neurônios, 4 sinapses, fila16, ciclo64, episódio128 eventos, 8 experiências, 4 alterações incluindo reversão, 8 feedbacks, 256 operações, checkpoint64KiB. Salvar exige repouso e destino novo.
Checkpoint MP05 v1: configuração, topologia, parâmetros/estados neuronais, pesos, motor/relógio/métricas, autorizações sintéticas com validade original, feedbacks consumidos e histórico. Replay validado, sem executar E/S do arquivo; checksum não autentica. Não salva memória epistêmica, F1 ou log MP06.
";
pub fn run<R: BufRead, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    let mut episode = SynapticEpisode::new();
    writeln!(output, "{HELP}")?;
    output.flush()?;
    for _ in 0..512 {
        let mut bytes = vec![];
        let count = input.by_ref().take(257).read_until(b'\n', &mut bytes)?;
        if count == 0 {
            return Ok(());
        }
        if count > 256 {
            writeln!(output, "Erro: linha excedida; encerrando")?;
            return Ok(());
        }
        let line = match std::str::from_utf8(&bytes) {
            Ok(s) => s.trim_end_matches(['\n', '\r']),
            Err(_) => {
                writeln!(output, "Erro: UTF-8 inválido")?;
                continue;
            }
        };
        let parts: Vec<_> = line.split_whitespace().collect();
        let result = match parts.as_slice() {
            ["sair"] => return Ok(()),
            ["ajuda"] => Ok(HELP.into()),
            ["salvar", p] => episode
                .save(Path::new(p))
                .map(|_| "Checkpoint MP05 salvo".into()),
            ["restaurar", p] => SynapticEpisode::load(Path::new(p)).map(|restored| {
                episode = restored;
                "Checkpoint MP05 restaurado; consumo e validade preservados".into()
            }),
            _ => episode.execute(line),
        };
        match result {
            Ok(x) => writeln!(output, "{x}")?,
            Err(e) => writeln!(output, "Erro: {e}")?,
        }
        output.flush()?;
    }
    writeln!(output, "Limite de entradas; encerrando")
}
