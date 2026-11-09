//! MP05: replay validado de atos explícitos sobre o Circuit existente.
//! Formato próprio; F1 e o codec Engine+EngramStore não são alterados.
use crate::circuit::{Circuit, CircuitSnapshot};
use crate::event::NodeId;
use crate::learning::{Feedback, PlasticityRule, WeightUpdate};
use crate::neuron::Neuron;
use crate::synapse::{Synapse, SynapseId};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub const MAX_FILE: usize = 64 * 1024;
pub const MAX_OPERATIONS: usize = 256;
fn e(x: impl std::fmt::Debug) -> String {
    format!("{x:?}")
}
fn number(s: &str) -> Result<u32, String> {
    let n = s.parse().map_err(|_| "inteiro inválido")?;
    if n == 0 {
        Err("ID zero".into())
    } else {
        Ok(n)
    }
}
fn signed(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| "valor inválido".into())
}
fn time(s: &str) -> Result<u64, String> {
    s.parse().map_err(|_| "instante inválido".into())
}
fn node(s: &str) -> Result<NodeId, String> {
    NodeId::new(number(s)?).map_err(e)
}
fn synapse(s: &str) -> Result<SynapseId, String> {
    SynapseId::new(number(s)?).map_err(e)
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub circuit: u32,
    pub context: u32,
    pub rule: u32,
    pub version: u32,
    pub min: i32,
    pub max: i32,
    pub step: i32,
    pub queue: usize,
    pub cycle: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Experience {
    pub id: u32,
    pub context: u32,
    pub source: NodeId,
    pub payload: i32,
    pub at: u64,
    pub before: CircuitSnapshot,
    pub after: Option<CircuitSnapshot>,
    pub complete: bool,
    pub valid: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grant {
    pub source: u32,
    pub start: u64,
    pub end: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateRecord {
    pub circuit: u32,
    pub context: u32,
    pub feedback: u32,
    pub experience: u32,
    pub source: u32,
    pub evidence: u32,
    pub synapse: SynapseId,
    pub rule: u32,
    pub version: u32,
    pub bounds: (i32, i32),
    pub step: i32,
    pub at: u64,
    pub update: WeightUpdate,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Trace {
    Configured(Config),
    Node(NodeId),
    Link(SynapseId),
    Authorized(Grant),
    Experience(Experience),
    Cycle {
        experience: u32,
        processed: usize,
        remaining: usize,
        error: Option<crate::circuit::CircuitError>,
    },
    Updated(UpdateRecord),
    Reverted {
        feedback: u32,
        source: u32,
        at: u64,
        before: i32,
        after: i32,
    },
}
/// Campos fechados: somente operações validadas podem alterar pesos, fila e histórico.
pub struct SynapticEpisode {
    config: Option<Config>,
    circuit: Option<Circuit>,
    rule: Option<PlasticityRule>,
    nodes: Vec<NodeId>,
    links: Vec<(SynapseId, NodeId, NodeId)>,
    grants: Vec<Grant>,
    experiences: Vec<Experience>,
    updates: Vec<UpdateRecord>,
    trace: Vec<Trace>,
    journal: Vec<String>,
    last_change: Option<UpdateRecord>,
    changes: usize,
    clock: u64,
}
impl Default for SynapticEpisode {
    fn default() -> Self {
        Self::new()
    }
}
impl SynapticEpisode {
    pub fn new() -> Self {
        Self {
            config: None,
            circuit: None,
            rule: None,
            nodes: vec![],
            links: vec![],
            grants: vec![],
            experiences: vec![],
            updates: vec![],
            trace: vec![],
            journal: vec![],
            last_change: None,
            changes: 0,
            clock: 0,
        }
    }
    pub fn circuit(&self) -> Option<&Circuit> {
        self.circuit.as_ref()
    }
    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }
    pub fn experiences(&self) -> &[Experience] {
        &self.experiences
    }
    pub fn updates(&self) -> &[UpdateRecord] {
        &self.updates
    }
    pub fn trace(&self) -> &[Trace] {
        &self.trace
    }
    pub fn state(&self) -> String {
        format!("config={:?}\ncircuit={:?}\ngrants={:?}\nexperiences={:?}\nupdates={:?}\ntrace={:?}\nchanges={} clock={} last={:?}\njournal={:?}",self.config,self.circuit.as_ref().map(Circuit::snapshot),self.grants,self.experiences,self.updates,self.trace,self.changes,self.clock,self.last_change,self.journal)
    }
    pub fn execute(&mut self, line: &str) -> Result<String, String> {
        if line.len() > 256 || line.chars().any(char::is_control) {
            return Err("linha inválida".into());
        }
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        if line == "estado" || line == "historico" {
            return Ok(self.state());
        }
        if self.journal.len() >= MAX_OPERATIONS {
            return Err("limite de operações".into());
        }
        let mut staged = Self::replay(&self.journal)?;
        let result = staged.apply(&line)?;
        staged.journal.push(line);
        *self = staged;
        Ok(result)
    }
    fn replay(lines: &[String]) -> Result<Self, String> {
        let mut x = Self::new();
        for l in lines {
            x.apply(l)?;
            x.journal.push(l.clone());
        }
        Ok(x)
    }
    fn at(&mut self, at: u64) -> Result<(), String> {
        if at < self.clock {
            return Err("instante regressivo".into());
        }
        self.clock = at;
        Ok(())
    }
    fn grant(&self, source: u32, at: u64) -> Result<(), String> {
        if self
            .grants
            .iter()
            .any(|g| g.source == source && g.start <= at && at <= g.end)
        {
            Ok(())
        } else {
            Err("feedback sem autorização sintética válida".into())
        }
    }
    fn resting(&self) -> Result<(), String> {
        if self.circuit.as_ref().ok_or("sem circuito")?.queue_len() != 0 {
            Err("eventos pendentes".into())
        } else {
            Ok(())
        }
    }
    fn apply(&mut self, line: &str) -> Result<String, String> {
        let p: Vec<_> = line.split_whitespace().collect();
        match p.as_slice() {
            ["novo", cid, ctx, rule, version, min, max, step, queue, cycle] => {
                if self.config.is_some() {
                    return Err("configuração fixada".into());
                }
                let config = Config {
                    circuit: number(cid)?,
                    context: number(ctx)?,
                    rule: number(rule)?,
                    version: number(version)?,
                    min: signed(min)?,
                    max: signed(max)?,
                    step: signed(step)?,
                    queue: number(queue)? as usize,
                    cycle: number(cycle)? as usize,
                };
                if config.queue > 16 || config.cycle > 64 {
                    return Err("limite de fila/ciclo".into());
                }
                self.rule =
                    Some(PlasticityRule::new(config.min, config.max, config.step).map_err(e)?);
                self.circuit = Some(Circuit::new(config.queue, config.cycle).map_err(e)?);
                self.config = Some(config.clone());
                self.trace.push(Trace::Configured(config));
                Ok("Circuito e regra declarados".into())
            }
            ["neuronio", id, threshold, limit] => {
                if !self.experiences.is_empty() || self.nodes.len() >= 4 {
                    return Err("topologia fixada ou limite de neurônios".into());
                }
                let id = node(id)?;
                self.circuit
                    .as_mut()
                    .ok_or("sem circuito")?
                    .add_neuron(Neuron::new(id, signed(threshold)?, signed(limit)?).map_err(e)?)
                    .map_err(e)?;
                self.nodes.push(id);
                self.trace.push(Trace::Node(id));
                Ok("Neurônio cadastrado".into())
            }
            ["sinapse", id, from, to, weight, delay] => {
                if !self.experiences.is_empty() || self.links.len() >= 4 {
                    return Err("topologia fixada ou limite de sinapses".into());
                }
                let id = synapse(id)?;
                let from = node(from)?;
                let to = node(to)?;
                let weight = signed(weight)?;
                let c = self.config.as_ref().ok_or("sem circuito")?;
                if weight < c.min || weight > c.max {
                    return Err("peso fora dos limites".into());
                }
                // DAG de até quatro arestas: no máximo 2^4 eventos por experiência.
                // Permite provar teto total 8*16=128 sem modificar o Engine geral.
                fn reaches(
                    edges: &[(SynapseId, NodeId, NodeId)],
                    a: NodeId,
                    b: NodeId,
                    depth: usize,
                ) -> bool {
                    a == b
                        || (depth > 0
                            && edges
                                .iter()
                                .any(|(_, f, t)| *f == a && reaches(edges, *t, b, depth - 1)))
                }
                if reaches(&self.links, to, from, 4) {
                    return Err("ciclo fora do recorte MP05".into());
                }
                self.circuit
                    .as_mut()
                    .unwrap()
                    .connect(Synapse::new(id, from, to, weight, time(delay)?).map_err(e)?)
                    .map_err(e)?;
                self.links.push((id, from, to));
                self.trace.push(Trace::Link(id));
                Ok("Sinapse cadastrada".into())
            }
            ["autorizar", source, start, end] => {
                self.resting()?;
                let g = Grant {
                    source: number(source)?,
                    start: time(start)?,
                    end: time(end)?,
                };
                if g.start > g.end
                    || self.grants.len() >= 2
                    || self.grants.iter().any(|x| x.source == g.source)
                {
                    return Err("autorização inválida/duplicada ou capacidade".into());
                }
                self.grants.push(g.clone());
                self.trace.push(Trace::Authorized(g));
                Ok("Fonte sintética declarada; não autenticada".into())
            }
            ["experiencia", id, context, source, payload, at] => {
                self.resting()?;
                if self.experiences.len() >= 8 {
                    return Err("limite de experiências".into());
                }
                let id = number(id)?;
                let context = number(context)?;
                let source = node(source)?;
                let payload = signed(payload)?;
                let at = time(at)?;
                if context != self.config.as_ref().unwrap().context
                    || self.nodes.is_empty()
                    || self.experiences.iter().any(|x| x.id == id)
                {
                    return Err("contexto/identidade inválida".into());
                }
                self.at(at)?;
                let c = self.circuit.as_mut().unwrap();
                let before = c.snapshot().map_err(e)?;
                c.stimulate(source, payload, at).map_err(e)?;
                let x = Experience {
                    id,
                    context,
                    source,
                    payload,
                    at,
                    before,
                    after: None,
                    complete: false,
                    valid: true,
                };
                self.trace.push(Trace::Experience(x.clone()));
                self.experiences.push(x);
                Ok("Experiência recebida; evento pendente; nenhum ajuste".into())
            }
            ["executar"] => {
                let x = self.experiences.last_mut().ok_or("sem experiência")?;
                if x.complete {
                    return Err("experiência já concluída".into());
                }
                let c = self.circuit.as_mut().unwrap();
                let r = c.run_cycle();
                self.clock = self.clock.max(c.logical_time().unwrap_or(self.clock));
                if c.engine_metrics().dispatched > 128 {
                    return Err("limite total de eventos".into());
                }
                x.valid &= r.first_error.is_none();
                x.complete = c.queue_len() == 0;
                x.after = Some(c.snapshot().map_err(e)?);
                self.trace.push(Trace::Cycle {
                    experience: x.id,
                    processed: r.cycle.processed,
                    remaining: r.cycle.remaining,
                    error: r.first_error,
                });
                if x.complete {
                    self.trace.push(Trace::Experience(x.clone()));
                }
                Ok(format!(
                    "Execução {:?}; antes={:?}; depois={:?}",
                    r,
                    x.before.neurons,
                    x.after.as_ref().unwrap().neurons
                ))
            }
            ["feedback", fid, experience, context, source, evidence, syn, signal, at] => {
                self.resting()?;
                let fid = number(fid)?;
                let experience = number(experience)?;
                let context = number(context)?;
                let source = number(source)?;
                let evidence = number(evidence)?;
                let syn = synapse(syn)?;
                let at = time(at)?;
                if self.updates.len() >= 8
                    || self.updates.iter().any(|x| {
                        x.feedback == fid || x.evidence == evidence || x.experience == experience
                    })
                {
                    return Err("feedback/evidência/experiência consumidos ou capacidade".into());
                }
                self.grant(source, at)?;
                let x = self.experiences.last().ok_or("sem experiência")?;
                if x.id != experience
                    || x.context != context
                    || context != self.config.as_ref().unwrap().context
                    || !x.complete
                    || !x.valid
                    || at < x.at
                {
                    return Err("vínculo ou experiência inválida".into());
                }
                let (_, from, _) = self
                    .links
                    .iter()
                    .find(|(id, _, _)| *id == syn)
                    .ok_or("sinapse ausente")?;
                let after = x
                    .after
                    .as_ref()
                    .unwrap()
                    .neurons
                    .iter()
                    .find(|n| n.id == *from)
                    .ok_or("origem ausente")?
                    .state
                    .activations;
                let before = x
                    .before
                    .neurons
                    .iter()
                    .find(|n| n.id == *from)
                    .unwrap()
                    .state
                    .activations;
                if *from != x.source || after <= before {
                    return Err("sinapse sem ativação da origem desta experiência".into());
                }
                let signal = match *signal {
                    "+" => Feedback::Positive,
                    "-" => Feedback::Negative,
                    _ => return Err("feedback inválido".into()),
                };
                self.at(at)?;
                let rule = self.rule.unwrap();
                let c = self.circuit.as_ref().unwrap();
                let planned = rule
                    .adjust(c.synapse_weight(syn).map_err(e)?, Some(signal))
                    .map_err(e)?;
                if planned.changed() && self.changes >= 4 {
                    return Err("limite de alterações".into());
                }
                let update = rule
                    .apply(self.circuit.as_mut().unwrap(), syn, Some(signal))
                    .map_err(e)?;
                let config = self.config.as_ref().unwrap();
                let record = UpdateRecord {
                    circuit: config.circuit,
                    context,
                    feedback: fid,
                    experience,
                    source,
                    evidence,
                    synapse: syn,
                    rule: config.rule,
                    version: config.version,
                    bounds: rule.bounds(),
                    step: rule.step(),
                    at,
                    update,
                };
                if update.changed() {
                    self.changes += 1;
                    self.last_change = Some(record.clone());
                }
                self.trace.push(Trace::Updated(record.clone()));
                self.updates.push(record.clone());
                Ok(format!("Atualização explícita {record:?}"))
            }
            ["reverter", fid, source, at] => {
                self.resting()?;
                let fid = number(fid)?;
                let source = number(source)?;
                let at = time(at)?;
                self.grant(source, at)?;
                let last = self
                    .last_change
                    .clone()
                    .ok_or("sem atualização reversível")?;
                if last.feedback != fid || self.changes >= 4 {
                    return Err("não é a última alteração ou limite".into());
                }
                let c = self.circuit.as_ref().unwrap();
                if c.synapse_weight(last.synapse).map_err(e)? != last.update.after {
                    return Err("peso divergente".into());
                }
                self.at(at)?;
                self.circuit
                    .as_mut()
                    .unwrap()
                    .set_weight(last.synapse, last.update.before)
                    .map_err(e)?;
                self.changes += 1;
                self.last_change = None;
                self.trace.push(Trace::Reverted {
                    feedback: fid,
                    source,
                    at,
                    before: last.update.after,
                    after: last.update.before,
                });
                Ok(format!(
                    "Reversão explícita: {} -> {}; feedback permanece consumido",
                    last.update.after, last.update.before
                ))
            }
            _ => Err("comando MP05 inválido".into()),
        }
    }
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        self.resting()?;
        let body = self.journal.join("\n");
        let mut out = b"NRNMP05\0".to_vec();
        out.extend(1u32.to_le_bytes());
        out.extend((body.len() as u32).to_le_bytes());
        out.extend(checksum(body.as_bytes()).to_le_bytes());
        out.extend(checksum(self.state().as_bytes()).to_le_bytes());
        out.extend(body.as_bytes());
        if out.len() > MAX_FILE {
            return Err("checkpoint excedido".into());
        }
        Ok(out)
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 32 || bytes.len() > MAX_FILE || &bytes[..8] != b"NRNMP05\0" {
            return Err("formato MP05 inválido".into());
        }
        if u32::from_le_bytes(bytes[8..12].try_into().unwrap()) != 1 {
            return Err("versão MP05 incompatível".into());
        }
        let body = &bytes[32..];
        if body.len() != u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize
            || checksum(body) != u64::from_le_bytes(bytes[16..24].try_into().unwrap())
        {
            return Err("corrupção MP05".into());
        }
        let body = std::str::from_utf8(body).map_err(e)?;
        let lines: Vec<_> = if body.is_empty() {
            vec![]
        } else {
            body.split('\n').map(str::to_string).collect()
        };
        if lines.len() > MAX_OPERATIONS
            || lines.iter().any(|l| {
                l.is_empty()
                    || l.len() > 256
                    || l.chars().any(char::is_control)
                    || *l != l.split_whitespace().collect::<Vec<_>>().join(" ")
            })
        {
            return Err("histórico inválido".into());
        }
        let state = Self::replay(&lines)?;
        if state.encode()?.as_slice() != bytes {
            return Err("estado reconstruído incompatível".into());
        }
        Ok(state)
    }
    pub fn save(&self, path: &Path) -> Result<(), String> {
        let bytes = self.encode()?;
        let name = path
            .file_name()
            .ok_or("destino inválido")?
            .to_string_lossy();
        let tmp = path.with_file_name(format!("{name}.mp05-pending"));
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp)
            .map_err(e)?;
        let result = (|| -> std::io::Result<()> {
            f.write_all(&bytes)?;
            f.sync_all()?;
            fs::hard_link(&tmp, path)?;
            Ok(())
        })();
        let clean = fs::remove_file(&tmp);
        result.map_err(e)?;
        clean.map_err(e)?;
        Ok(())
    }
    pub fn load(path: &Path) -> Result<Self, String> {
        let meta = fs::symlink_metadata(path).map_err(e)?;
        if !meta.is_file() || meta.len() > MAX_FILE as u64 {
            return Err("arquivo MP05 inválido".into());
        }
        let mut bytes = vec![];
        fs::File::open(path)
            .map_err(e)?
            .take((MAX_FILE + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(e)?;
        Self::decode(&bytes)
    }
}
fn checksum(b: &[u8]) -> u64 {
    b.iter().fold(0xcbf29ce484222325, |h, x| {
        (h ^ u64::from(*x)).wrapping_mul(0x100000001b3)
    })
}
