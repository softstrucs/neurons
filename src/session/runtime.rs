//! Adaptador local supervisionado. Não contém inferência, seleção ou regra H23.
//! Gerações administrativas não substituem codecs científicos nem autenticam o operador.
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};

use crate::cognition::concepts::Attributes;
use crate::cognitive_contracts::{EpisodeId, SourceId};
use crate::curriculum::{PedagogyAssessment, PedagogyHost, PedagogyPermissions};
use crate::engram::{EngramId, EvidenceId};
use crate::event::{Event, EventKind, NodeId};
use crate::integration::{development::*, synaptic::SynapticEpisode};
use crate::session::f1::Session;

const MAX_BUNDLE: usize = 1024 * 1024;
const MAX_GENERATIONS: u32 = 256;
const HELP: &str = "Runtime local supervisionado; sem linguagem natural ou ações externas.
armazenamento DIRETORIO (exclusivo; sem sobrescrita)
configurar ENTIDADE REVISAO CONTEXTO FONTE ACAO ALVO CONCEITO OBS EVENTOS ATENCAO
ambiente INICIAL / transicao DE ACAO PARA / atributos ESTADO BITS / fixar
restaurar (sem autoridade; preparações transitórias não retomadas)
autorizar observar|consultar|agir|investigar|avaliar|apresentar|admitir|inspecionar
autorizar acao ID (concessão nova e descartável)
objetivo ID DESTINO / observar / agir / ensinar PRE_BITS POS_BITS
oferta ESTADO / investigar / planejar PASSOS / executar PASSOS
episodio NOVO_ID INICIAL (arquiva o anterior; recursos novos explicitamente)
estado / recursos / inspecionar / salvar / sair
Cada mutação é persistida. Até 256 gerações, 4 episódios arquivados, 8 ações totais.
Interrupção com marcador pendente bloqueia retomada; não há replay automático.";

type Result<T> = std::result::Result<T, String>;
fn num<T: std::str::FromStr>(s: &str) -> Result<T> {
    s.parse().map_err(|_| "número inválido".into())
}
fn node(n: u32) -> Result<NodeId> {
    NodeId::new(n).map_err(|e| format!("{e:?}"))
}
fn source(n: u32) -> Result<SourceId> {
    SourceId::new(n).map_err(|e| format!("{e:?}"))
}
fn episode(n: u32) -> Result<EpisodeId> {
    EpisodeId::new(n).map_err(|e| format!("{e:?}"))
}
fn bits(s: &str) -> Result<Attributes> {
    if s.len() != 6 || !s.bytes().all(|b| b == b'0' || b == b'1') {
        return Err("seis bits exigidos".into());
    }
    Attributes::new(std::array::from_fn(|i| s.as_bytes()[i] - b'0')).map_err(|e| format!("{e:?}"))
}
fn checksum(b: &[u8]) -> u64 {
    b.iter().fold(0xcbf29ce484222325, |h, v| {
        (h ^ u64::from(*v)).wrapping_mul(0x100000001b3)
    })
}
fn restricted(path: &Path) -> io::Result<File> {
    let mut o = OpenOptions::new();
    o.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600);
    }
    o.open(path)
}
fn read_limited(path: &Path) -> Result<Vec<u8>> {
    let m = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !m.is_file() || m.len() > MAX_BUNDLE as u64 {
        return Err("arquivo regular limitado exigido".into());
    }
    let mut b = vec![];
    File::open(path)
        .map_err(|e| e.to_string())?
        .take(MAX_BUNDLE as u64 + 1)
        .read_to_end(&mut b)
        .map_err(|e| e.to_string())?;
    if b.len() > MAX_BUNDLE {
        return Err("arquivo excedido".into());
    }
    Ok(b)
}
struct Storage {
    path: PathBuf,
    generation: u32,
    lock: File,
    digest: u64,
}
impl Storage {
    fn open(path: &Path) -> Result<Self> {
        // Recusa links em todos os componentes já existentes, não só no destino.
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(|e| e.to_string())?
                .join(path)
        };
        let mut check = PathBuf::new();
        for c in absolute.components() {
            if matches!(c, std::path::Component::ParentDir) {
                return Err("caminho com .. recusado".into());
            }
            check.push(c);
            if let Ok(m) = fs::symlink_metadata(&check) {
                if m.file_type().is_symlink() {
                    return Err("link simbólico recusado".into());
                }
            }
        }
        if !absolute.exists() {
            let mut builder = fs::DirBuilder::new();
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                builder.mode(0o700);
            }
            builder.create(&absolute).map_err(|e| e.to_string())?;
        }
        if !fs::symlink_metadata(&absolute)
            .map_err(|e| e.to_string())?
            .is_dir()
        {
            return Err("diretório exigido".into());
        }
        // Valida nomes antes de criar lock. Fontes e diretórios preexistentes não são removidos.
        let generation = Self::scan(&absolute)?;
        if absolute.join("pendente").exists() {
            return Err("execução incerta: marcador pendente; retomada bloqueada".into());
        }
        let mut digest = 0;
        for id in 1..=generation {
            let b = read_limited(&absolute.join(format!("geracao-{id:08}.bin")))?;
            Self::payload(&b, id, Some(digest))?;
            digest = checksum(&b);
        }
        let mut lock = restricted(&absolute.join("sessao.lock"))
            .map_err(|e| format!("lock exclusivo: {e}"))?;
        writeln!(lock, "{}", std::process::id()).map_err(|e| e.to_string())?;
        lock.sync_all().map_err(|e| e.to_string())?;
        Ok(Self {
            path: absolute,
            generation,
            lock,
            digest,
        })
    }
    fn scan(path: &Path) -> Result<u32> {
        let mut ids = vec![];
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let e = entry.map_err(|e| e.to_string())?;
            let n = e.file_name();
            let n = n.to_str().ok_or("nome inválido")?;
            if n == "sessao.lock" || n == "pendente" {
                continue;
            }
            let id = num::<u32>(
                n.strip_prefix("geracao-")
                    .and_then(|v| v.strip_suffix(".bin"))
                    .ok_or("arquivo inesperado no armazenamento")?,
            )?;
            if id == 0
                || id > MAX_GENERATIONS
                || n != format!("geracao-{id:08}.bin")
                || !e.file_type().map_err(|e| e.to_string())?.is_file()
            {
                return Err("geração inválida".into());
            }
            ids.push(id);
        }
        ids.sort_unstable();
        if ids.iter().enumerate().any(|(i, n)| *n != i as u32 + 1) {
            return Err("sequência de gerações incompleta".into());
        }
        Ok(ids.last().copied().unwrap_or(0))
    }
    fn payload(bytes: &[u8], id: u32, parent: Option<u64>) -> Result<&[u8]> {
        if bytes.len() < 28 || bytes.len() > MAX_BUNDLE || &bytes[..8] != b"NRNGEN01" {
            return Err("integridade administrativa: geração inválida".into());
        }
        let end = bytes.len() - 8;
        if checksum(&bytes[..end]) != u64::from_le_bytes(bytes[end..].try_into().unwrap()) {
            return Err("integridade administrativa: checksum de geração inválido".into());
        }
        let actual = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let previous = u64::from_le_bytes(bytes[12..20].try_into().unwrap());
        if actual != id || parent.is_some_and(|p| p != previous) {
            return Err("integridade administrativa: geração ou encadeamento incompatível".into());
        }
        Ok(&bytes[20..end])
    }
    fn sync(&self) -> Result<()> {
        File::open(&self.path)
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())
    }
    fn begin(&self) -> Result<()> {
        if self.generation >= MAX_GENERATIONS {
            return Err("limite de gerações atingido".into());
        }
        let mut f = restricted(&self.path.join("pendente")).map_err(|e| e.to_string())?;
        writeln!(f, "intencao {}", self.generation + 1).map_err(|e| e.to_string())?;
        f.sync_all().map_err(|e| e.to_string())?;
        self.sync()
    }
    fn commit(&mut self, state: &State) -> Result<()> {
        let payload = state.encode()?;
        let next = self.generation + 1;
        let mut b = b"NRNGEN01".to_vec();
        b.extend(next.to_le_bytes());
        b.extend(self.digest.to_le_bytes());
        b.extend(payload);
        b.extend(checksum(&b).to_le_bytes());
        if b.len() > MAX_BUNDLE {
            return Err("geração administrativa excedida".into());
        }
        let mut f = restricted(&self.path.join(format!("geracao-{next:08}.bin")))
            .map_err(|e| e.to_string())?;
        f.write_all(&b)
            .and_then(|_| f.sync_all())
            .map_err(|e| e.to_string())?;
        self.sync()?;
        fs::remove_file(self.path.join("pendente")).map_err(|e| e.to_string())?;
        self.sync()?;
        self.generation = next;
        self.digest = checksum(&b);
        Ok(())
    }
    fn restore(&self) -> Result<State> {
        if self.generation == 0 {
            return Err("nenhuma geração disponível".into());
        }
        let b = read_limited(
            &self
                .path
                .join(format!("geracao-{:08}.bin", self.generation)),
        )?;
        if checksum(&b) != self.digest {
            return Err("geração mudou durante abertura".into());
        }
        State::decode(Self::payload(&b, self.generation, None)?)
    }
}
impl Drop for Storage {
    fn drop(&mut self) {
        let _ = &self.lock;
        let _ = fs::remove_file(self.path.join("sessao.lock"));
    }
}

#[derive(Clone)]
struct Config {
    world: u32,
    revision: u32,
    context: u32,
    source: u32,
    action: u32,
    target: i32,
    concept: u32,
    observations: usize,
    events: usize,
    attention: usize,
}
impl Config {
    fn parse(v: &[&str]) -> Result<Self> {
        if v.len() != 10 {
            return Err("configurar exige dez números".into());
        }
        let c = Self {
            world: num(v[0])?,
            revision: num(v[1])?,
            context: num(v[2])?,
            source: num(v[3])?,
            action: num(v[4])?,
            target: num(v[5])?,
            concept: num(v[6])?,
            observations: num(v[7])?,
            events: num(v[8])?,
            attention: num(v[9])?,
        };
        node(c.world)?;
        node(c.concept)?;
        source(c.source)?;
        if c.revision == 0
            || c.context == 0
            || c.context == u32::MAX
            || c.action == 0
            || c.observations > 8
            || c.events > 128
            || c.attention > 32
        {
            return Err("configuração fora dos limites".into());
        }
        Ok(c)
    }
    fn text(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {} {}",
            self.world,
            self.revision,
            self.context,
            self.source,
            self.action,
            self.target,
            self.concept,
            self.observations,
            self.events,
            self.attention
        )
    }
    fn sensors(&self) -> Result<Vec<Sensor>> {
        Ok([Encoding::Scalar, Encoding::BooleanAttributes]
            .into_iter()
            .enumerate()
            .map(|(i, encoding)| {
                Ok(Sensor {
                    id: i as u32 + 1,
                    source: source(self.source)?,
                    context: self.context,
                    input: node(101)?,
                    encoding,
                })
            })
            .collect::<Result<Vec<_>>>()?)
    }
    fn neural(&self) -> Result<SynapticEpisode> {
        let mut n = SynapticEpisode::new();
        for cmd in [
            format!("novo 41 {} 81 1 0 3 2 16 64", self.context),
            "neuronio 101 1 4".into(),
            "neuronio 103 2 4".into(),
            "sinapse 11 101 103 0 1".into(),
        ] {
            n.execute(&cmd)?;
        }
        Ok(n)
    }
    fn limits(&self) -> Limits {
        Limits {
            observations: self.observations,
            neural_events: self.events,
            attention_scans: self.attention,
            ..Limits::default()
        }
    }
}
struct State {
    config: Config,
    runtime: Runtime,
    knowledge: Session,
    archive: EpisodeArchive,
    archived: Vec<u32>,
    episode: u32,
    nonce: u32,
    current: i32,
    total_actions: usize,
    fixed: bool,
    transitions: Vec<(i32, u32, i32)>,
    attributes: Vec<(i32, String)>,
    goal: Option<(u32, i32)>,
    issued: Vec<u32>,
    access: Access,
    pedagogy: [bool; 3],
    investigation: bool,
    permits: Vec<u32>,
    frame: Option<AttributeFrame>,
    samples: Vec<(AttributeFrame, u32, EngramId, u32)>,
    offers: Vec<i32>,
    cursor: Option<ProbeSearch>,
}
impl State {
    fn new(config: Config) -> Result<Self> {
        let runtime = Runtime::new(
            config.neural()?,
            &config.sensors()?,
            Access::default(),
            config.limits(),
        )?;
        let mut knowledge = Session::new();
        for cmd in [
            format!("config 91 {} 81 1 0 10000", config.context),
            format!("conceito {} adquirido", config.concept),
            "agora 6000".into(),
        ] {
            knowledge.execute(&cmd)?;
        }
        Ok(Self {
            config,
            runtime,
            knowledge,
            archive: EpisodeArchive::new(),
            archived: vec![],
            episode: 1,
            nonce: 0,
            current: 0,
            total_actions: 0,
            fixed: false,
            transitions: vec![],
            attributes: vec![],
            goal: None,
            issued: vec![],
            access: Access::default(),
            pedagogy: [false; 3],
            investigation: false,
            permits: vec![],
            frame: None,
            samples: vec![],
            offers: vec![],
            cursor: None,
        })
    }
    fn contract(&self) -> Result<ConsequenceContract> {
        Ok(ConsequenceContract {
            concept: node(self.config.concept)?,
            scope: self.knowledge.pedagogy_identity()?.1,
            world: self.config.world,
            revision: self.config.revision,
            source: source(self.config.source)?,
            action: self.config.action,
            target: self.config.target,
        })
    }
    fn world(&self, initial: i32) -> Result<World> {
        if !self.fixed {
            return Err("fixe o ambiente antes de operar".into());
        }
        World::new(
            self.config.world,
            self.config.revision,
            self.config.sensors()?[0].clone(),
            initial,
            &self.transitions,
        )
    }
    fn task(&self) -> Result<PlanningGoal> {
        let (_, to) = self.goal.ok_or("declare objetivo")?;
        Ok(PlanningGoal {
            world: self.config.world,
            revision: self.config.revision,
            context: self.config.context,
            source: source(self.config.source)?,
            from: self.current,
            to,
        })
    }
    fn next(&mut self) -> Result<u32> {
        if self.nonce >= 512 {
            return Err("limite de identidades atingido".into());
        }
        self.nonce += 1;
        Ok(self.nonce)
    }
    fn reading(&mut self, state: i32) -> Result<Reading> {
        let value = self
            .attributes
            .iter()
            .find(|a| a.0 == state)
            .ok_or("atributos atuais indisponíveis")?
            .1
            .clone();
        let id = self.next()?;
        Ok(Reading {
            id: 10 * id,
            item: node(self.config.world)?,
            sensor: 2,
            source: source(self.config.source)?,
            evidence: EvidenceId::new(10 * id).map_err(|e| format!("{e:?}"))?,
            event: Event::new(
                None,
                node(101)?,
                EventKind::Stimulus,
                bits(&value)?.encode(),
                u64::from(id) * 10,
            ),
        })
    }
    fn request(&mut self, permit: u32) -> Result<Intervention> {
        let id = self.next()?;
        Ok(Intervention {
            id,
            permit: ActionPermit {
                id: permit,
                world: self.config.world,
                revision: self.config.revision,
                context: self.config.context,
                source: source(self.config.source)?,
                action: self.config.action,
            },
            before_id: 10 * id + 1,
            after_id: 10 * id + 2,
            before_evidence: EvidenceId::new(10 * id + 1).map_err(|e| format!("{e:?}"))?,
            after_evidence: EvidenceId::new(10 * id + 2).map_err(|e| format!("{e:?}"))?,
            at: u64::from(id) * 10,
        })
    }
    fn grant(&mut self, v: &[&str]) -> Result<String> {
        match v {
            ["acao", id] => {
                let id = num::<u32>(id)?;
                if id == 0
                    || self.issued.contains(&id)
                    || self.issued.len() >= 256
                    || self.permits.len() >= 4
                {
                    return Err("concessão repetida ou excessiva".into());
                }
                self.issued.push(id);
                self.permits.push(id);
            }
            [kind] => {
                match *kind {
                    "observar" => {
                        self.access.record = true;
                        self.access.sensors = vec![1, 2];
                    }
                    "consultar" => self.access.read_knowledge = true,
                    "agir" => self.access.explore = true,
                    "inspecionar" => self.access.inspect = true,
                    "investigar" => self.investigation = true,
                    "avaliar" => self.pedagogy[0] = true,
                    "apresentar" => self.pedagogy[1] = true,
                    "admitir" => {
                        self.pedagogy[2] = true;
                        self.access.learn_concepts = true;
                    }
                    _ => return Err("autoridade desconhecida".into()),
                }
                let bytes = self.runtime.checkpoint(&self.knowledge)?;
                let (r, s) =
                    Runtime::restore(&bytes, &self.config.sensors()?, self.access.clone())?;
                self.runtime = r;
                self.knowledge = s;
            }
            _ => return Err("autorizar inválido".into()),
        }
        Ok("Autorização explícita registrada; não é autenticação de produção".into())
    }
    fn permissions(&self, ids: Vec<EngramId>) -> Result<PedagogyPermissions> {
        if self.pedagogy != [true; 3] {
            return Err(
                "avaliação, apresentação e admissão exigem concessões independentes".into(),
            );
        }
        let scope = self.contract()?.scope;
        Ok(PedagogyPermissions {
            evaluation: Some(scope),
            presentation: Some(scope),
            admissions: ids,
        })
    }
    fn command(&mut self, v: &[&str]) -> Result<String> {
        match v {
            ["ambiente", initial] if !self.fixed && self.transitions.is_empty() => {
                self.current = num(initial)?;
                Ok("Estado inicial configurado".into())
            }
            ["transicao", from, action, to] if !self.fixed => {
                let t = (num(from)?, num(action)?, num(to)?);
                if self.transitions.len() >= 8
                    || t.1 == 0
                    || self
                        .transitions
                        .iter()
                        .any(|old| old.0 == t.0 && old.1 == t.1)
                {
                    return Err("transição duplicada ou excessiva".into());
                }
                self.transitions.push(t);
                Ok("Transição privada do ambiente registrada".into())
            }
            ["atributos", state, value] if !self.fixed => {
                let state = num(state)?;
                bits(value)?;
                if self.attributes.len() >= 8 || self.attributes.iter().any(|a| a.0 == state) {
                    return Err("atributos duplicados ou excessivos".into());
                }
                self.attributes.push((state, value.to_string()));
                Ok("Atributos observáveis registrados".into())
            }
            ["fixar"] if !self.fixed => {
                self.fixed = true;
                self.world(self.current)?;
                Ok("Ambiente fixado; consequências não são entradas do planejador".into())
            }
            ["autorizar", rest @ ..] => self.grant(rest),
            ["objetivo", id, target] => {
                if self.goal.is_some() {
                    return Err("objetivo já fixado para este episódio".into());
                }
                let id = num(id)?;
                episode(id)?;
                let target = num(target)?;
                if target != self.config.target {
                    return Err("destino incompatível com contrato conceitual".into());
                }
                self.goal = Some((id, target));
                Ok("Objetivo fixado".into())
            }
            ["inspecionar"] => {
                let report = self.runtime.discover(Some(&self.knowledge))?;
                Ok(format!("Descoberta operacional: {report:?}"))
            }
            ["observar"] => {
                if self.frame.is_some() || self.samples.len() >= 2 {
                    return Err("preparação anterior ou limite de amostras".into());
                }
                let w = self.world(self.current)?;
                let reading = self.reading(self.current)?;
                let frame = self
                    .runtime
                    .observe_attributes(&w, self.config.action, reading)?;
                self.frame = Some(frame);
                Ok("Atributos observados com procedência".into())
            }
            ["agir"] => {
                if self.total_actions >= 8 {
                    return Err("limite agregado de ações atingido".into());
                }
                let frame = self.frame.take().ok_or("observe antes de agir")?;
                if self.permits.is_empty() {
                    self.frame = Some(frame);
                    return Err("ação sem concessão".into());
                }
                let permit = self.permits.remove(0);
                let request = self.request(permit)?;
                let mut w = self.world(self.current)?;
                let result = self.runtime.intervene(&mut w, &request);
                self.current = w.state();
                self.total_actions += w.action_count();
                let trial = result?;
                let record = EngramId::new(20000 + request.id).map_err(|e| format!("{e:?}"))?;
                self.samples
                    .push((frame, trial.id, record, 30000 + request.id));
                Ok(format!(
                    "Experiência observada: {} -> {}; ensino ainda não admitido",
                    trial.from, trial.to
                ))
            }
            ["ensinar", pre, post] => {
                let assessment =
                    PedagogyAssessment::new(&[bits(pre)?.into()], &[bits(post)?.into()])?;
                let permissions = self.permissions(self.samples.iter().map(|s| s.2).collect())?;
                let c = self.contract()?;
                let samples = std::mem::take(&mut self.samples);
                let report = self.runtime.teach_consequences(
                    &mut self.knowledge,
                    c,
                    &samples,
                    assessment,
                    permissions,
                )?;
                self.cursor = None;
                Ok(format!(
                    "Aprendizagem autorizada: versão {} critérios {:?}",
                    report.pedagogy.final_version.number, report.pedagogy.final_version.criteria
                ))
            }
            ["oferta", state] => {
                let state = num(state)?;
                self.world(state)?;
                if !self.attributes.iter().any(|a| a.0 == state)
                    || self.offers.len() >= 4
                    || self.offers.contains(&state)
                {
                    return Err("oferta indisponível, repetida ou excessiva".into());
                }
                if self.cursor.is_some() {
                    return Err("busca já vinculada às ofertas".into());
                }
                self.offers.push(state);
                Ok("Oferta sem rótulo registrada; não autoriza execução".into())
            }
            ["investigar"] => self.investigate(),
            ["planejar", steps] => {
                let goal = self.task()?;
                let memory = self.runtime.retrieve_context(
                    &self.archive,
                    &self.knowledge,
                    goal,
                    RetrievalBudget::default(),
                )?;
                let out = self.runtime.plan_retrieved(
                    &self.knowledge,
                    goal,
                    PlanningBudget {
                        steps: num(steps)?,
                        scans: 32,
                    },
                    &memory,
                )?;
                Ok(format!(
                    "Plano observado: {:?}; cobertura {:?}; nenhuma ação autorizada por seleção",
                    out.reason,
                    memory.status()
                ))
            }
            ["executar", steps] => self.execute(num(steps)?),
            ["episodio", id, initial] => {
                let id = num(id)?;
                episode(id)?;
                if id <= self.episode || self.archived.contains(&id) || self.archive.len() >= 4 {
                    return Err("episódio repetido, fora de ordem ou arquivo cheio".into());
                }
                if self.frame.is_some() || !self.samples.is_empty() {
                    return Err("conclua aprendizagem antes de encerrar episódio".into());
                }
                let initial = num(initial)?;
                self.world(initial)?;
                let mut next = Runtime::new(
                    self.config.neural()?,
                    &self.config.sensors()?,
                    Access::default(),
                    self.config.limits(),
                )?;
                if self.knowledge.pedagogy_version()?.number > 0 {
                    next.inherit_consequence_contract(&self.runtime)?;
                }
                self.archive
                    .insert(episode(self.episode)?, &self.runtime, &self.knowledge)?;
                self.archived.push(self.episode);
                self.runtime = next;
                self.episode = id;
                self.current = initial;
                self.access = Access::default();
                self.pedagogy = [false; 3];
                self.investigation = false;
                self.permits.clear();
                self.goal = None;
                self.offers.clear();
                self.cursor = None;
                Ok("Novo episódio: memória retida, autoridade negada e reservas próprias explícitas; contador ambiental agregado preservado".into())
            }
            ["salvar"] => Ok(
                "Geração coerente solicitada; preparações e concessões não são restauráveis".into(),
            ),
            _ => Err("comando inválido ou configuração já fixada".into()),
        }
    }
    fn investigate(&mut self) -> Result<String> {
        if !self.investigation {
            return Err("investigação não autorizada".into());
        }
        if self.offers.is_empty() || self.permits.len() < self.offers.len() {
            return Err("ofertas e concessões independentes exigidas".into());
        }
        if self.total_actions >= 8 {
            return Err("limite agregado de ações atingido".into());
        }
        if self.frame.is_some() || !self.samples.is_empty() {
            return Err("preparações anteriores pendentes".into());
        }
        let c = self.contract()?;
        let task = self.task()?;
        let memory = self.runtime.retrieve_context(
            &self.archive,
            &self.knowledge,
            task,
            RetrievalBudget::default(),
        )?;
        let offered = self
            .offers
            .iter()
            .map(|state| {
                bits(
                    &self
                        .attributes
                        .iter()
                        .find(|a| a.0 == *state)
                        .ok_or("atributos ausentes")?
                        .1,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let mut search = match self.cursor.take() {
            Some(s) => s,
            None => ProbeSearch::new(&self.knowledge, c, &offered)?,
        };
        let probes = self.runtime.continue_probe_search(
            &self.knowledge,
            c,
            &offered,
            &mut search,
            ProbeBudget::default(),
        )?;
        let Some(question) = probes.patterns.first().copied() else {
            self.cursor = Some(search);
            return Ok(format!("Investigação suspensa: {:?}", probes.status));
        };
        let states = self.offers.clone();
        let mut worlds = states
            .iter()
            .map(|s| self.world(*s))
            .collect::<Result<Vec<_>>>()?;
        let mut entries = vec![];
        for state in &states {
            let r = self.reading(*state)?;
            let permit = self.permits.remove(0);
            let request = self.request(permit)?;
            entries.push((r, request));
        }
        let mut offers = worlds
            .iter_mut()
            .zip(entries)
            .map(|(w, (attributes, intervention))| InvestigationOffer {
                world: w,
                attributes,
                intervention,
            })
            .collect::<Vec<_>>();
        let teaching = if self.pedagogy == [true; 3] {
            let id = self.next()?;
            let record = EngramId::new(20000 + id).map_err(|e| format!("{e:?}"))?;
            Some(ProbeTeaching {
                record,
                receipt: 30000 + id,
                permissions: self.permissions(vec![record])?,
            })
        } else {
            None
        };
        let out = self.runtime.investigate_continued_probes(
            &memory,
            &mut self.knowledge,
            c,
            question,
            &mut offers,
            teaching,
            &mut search,
            ProbeBudget {
                patterns: 0,
                evaluations: 0,
            },
        );
        drop(offers);
        for w in &worlds {
            if w.action_count() > 0 {
                self.current = w.state();
                self.total_actions += w.action_count();
            }
        }
        if self.knowledge.pedagogy_version()? == probes.version {
            self.cursor = Some(search);
        }
        let out = out?;
        Ok(format!(
            "Sondas {:?}; cobertura exaustiva {}; diagnóstico {:?}; pergunta {}; investigação {:?}",
            out.outcome.probes.status,
            out.outcome.probes.exhaustive,
            out.outcome.epistemic.state,
            out.outcome
                .investigation
                .as_ref()
                .map(|r| r.question.as_str())
                .unwrap_or("evidência insuficiente; manter ambiguidade"),
            out.outcome
                .investigation
                .as_ref()
                .map(|r| (r.result, r.selected, &r.retained))
        ))
    }
    fn execute(&mut self, steps: usize) -> Result<String> {
        if steps == 0 || steps > 4 || self.total_actions + steps > 8 || self.permits.len() < steps {
            return Err("passos, concessões ou orçamento agregado insuficientes".into());
        }
        if self.frame.is_some() || !self.samples.is_empty() {
            return Err("preparações anteriores pendentes".into());
        }
        let task = self.task()?;
        let objective = episode(self.goal.ok_or("objetivo ausente")?.0)?;
        let c = self.contract()?;
        let memory = self.runtime.retrieve_context(
            &self.archive,
            &self.knowledge,
            task,
            RetrievalBudget::default(),
        )?;
        let reading = self.reading(self.current)?;
        let mut requests = vec![];
        for _ in 0..steps {
            let permit = self.permits.remove(0);
            requests.push(self.request(permit)?);
        }
        let mut w = self.world(self.current)?;
        let out = self.runtime.pursue_with_consequences(
            &memory,
            &self.knowledge,
            &mut w,
            PursuitGoal {
                objective,
                task,
                max_actions: steps,
            },
            c,
            reading,
            &requests,
        );
        self.current = w.state();
        self.total_actions += w.action_count();
        let out = out?;
        Ok(format!(
            "Aplicação: {:?}; escolha {:?}; justificativa {}; conferência {:?}; estado {}; ações totais {}; cobertura {:?}",
            out.reason, out.choice, out.justification, out.review.as_ref().map(|r|r.agreement),
            self.current,
            self.total_actions,
            memory.status()
        ))
    }
    fn encode(&self) -> Result<Vec<u8>> {
        let mut meta = format!(
            "{}\n{} {} {} {} {}\n{}\n{}\n{}\n",
            self.config.text(),
            self.episode,
            self.nonce,
            self.current,
            self.total_actions,
            u8::from(self.fixed),
            self.goal
                .map(|g| format!("{} {}", g.0, g.1))
                .unwrap_or_else(|| "-".into()),
            self.issued
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            self.archived
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        meta.push_str(&format!("{}\n", self.transitions.len()));
        for (f, a, t) in &self.transitions {
            meta.push_str(&format!("{f} {a} {t}\n"));
        }
        meta.push_str(&format!("{}\n", self.attributes.len()));
        for (state, bits) in &self.attributes {
            meta.push_str(&format!("{state} {bits}\n"));
        }
        meta.push_str(&format!(
            "{}\n",
            self.offers
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        ));
        let mut out = b"NRNCLI01".to_vec();
        for section in [
            meta.into_bytes(),
            self.runtime.checkpoint(&self.knowledge)?,
            self.archive.encode(),
            self.cursor
                .as_ref()
                .map(|s| s.encode())
                .transpose()?
                .unwrap_or_default(),
        ] {
            out.extend((section.len() as u32).to_le_bytes());
            out.extend(section);
        }
        out.extend(checksum(&out).to_le_bytes());
        if out.len() > MAX_BUNDLE {
            return Err("pacote administrativo excedido".into());
        }
        Ok(out)
    }
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 32 || bytes.len() > MAX_BUNDLE || &bytes[..8] != b"NRNCLI01" {
            return Err("pacote administrativo inválido".into());
        }
        let end = bytes.len() - 8;
        if checksum(&bytes[..end]) != u64::from_le_bytes(bytes[end..].try_into().unwrap()) {
            return Err("integridade administrativa inválida".into());
        }
        let mut at = 8;
        let mut sections = vec![];
        for _ in 0..4 {
            let size = bytes.get(at..at + 4).ok_or("pacote truncado")?;
            at += 4;
            let size = u32::from_le_bytes(size.try_into().unwrap()) as usize;
            let next = at.checked_add(size).ok_or("tamanho inválido")?;
            sections.push(
                bytes
                    .get(at..next)
                    .filter(|_| next <= end)
                    .ok_or("seção truncada")?,
            );
            at = next;
        }
        if at != end {
            return Err("conteúdo administrativo extra".into());
        }
        let text = std::str::from_utf8(sections[0]).map_err(|e| e.to_string())?;
        let mut lines = text.lines();
        let mut line = || {
            lines
                .next()
                .map(|l| l.split_whitespace().collect::<Vec<_>>())
                .ok_or("metadados incompletos".to_string())
        };
        let c = Config::parse(&line()?)?;
        let mut s = Self::new(c)?;
        let v = line()?;
        if v.len() != 5 {
            return Err("estado administrativo inválido".into());
        }
        s.episode = num(v[0])?;
        episode(s.episode)?;
        s.nonce = num(v[1])?;
        s.current = num(v[2])?;
        s.total_actions = num(v[3])?;
        s.fixed = match v[4] {
            "0" => false,
            "1" => true,
            _ => return Err("flag inválida".into()),
        };
        let v = line()?;
        s.goal = match v.as_slice() {
            ["-"] => None,
            [id, target] => {
                let id = num(id)?;
                episode(id)?;
                Some((id, num(target)?))
            }
            _ => return Err("objetivo inválido".into()),
        };
        s.issued = line()?.iter().map(|v| num(v)).collect::<Result<Vec<_>>>()?;
        s.archived = line()?.iter().map(|v| num(v)).collect::<Result<Vec<_>>>()?;
        let v = line()?;
        let count = if v.len() == 1 {
            num::<usize>(v[0])?
        } else {
            return Err("contagem inválida".into());
        };
        if count > 8 {
            return Err("transições excedidas".into());
        }
        for _ in 0..count {
            let v = line()?;
            if v.len() != 3 {
                return Err("transição inválida".into());
            }
            s.transitions.push((num(v[0])?, num(v[1])?, num(v[2])?));
        }
        let v = line()?;
        let count = if v.len() == 1 {
            num::<usize>(v[0])?
        } else {
            return Err("contagem inválida".into());
        };
        if count > 8 {
            return Err("atributos excedidos".into());
        }
        for _ in 0..count {
            let v = line()?;
            if v.len() != 2 {
                return Err("atributo inválido".into());
            }
            let state = num(v[0])?;
            bits(v[1])?;
            if s.attributes.iter().any(|a| a.0 == state) {
                return Err("atributos duplicados".into());
            }
            s.attributes.push((state, v[1].into()));
        }
        s.offers = line()?.iter().map(|v| num(v)).collect::<Result<Vec<_>>>()?;
        if lines.next().is_some()
            || s.nonce > 512
            || s.total_actions > 8
            || s.issued.len() > 256
            || s.issued
                .iter()
                .enumerate()
                .any(|(i, id)| *id == 0 || s.issued[..i].contains(id))
            || s.archived.len() > 4
            || s.archived
                .iter()
                .enumerate()
                .any(|(i, id)| *id == 0 || *id >= s.episode || s.archived[..i].contains(id))
            || s.offers.len() > 4
            || s.offers.iter().enumerate().any(|(i, id)| {
                s.offers[..i].contains(id) || !s.attributes.iter().any(|a| a.0 == *id)
            })
            || s.goal.is_some_and(|g| g.1 != s.config.target)
        {
            return Err("metadados incoerentes".into());
        }
        let ports = s.config.sensors()?;
        (s.runtime, s.knowledge) = Runtime::restore(sections[1], &ports, Access::default())?;
        if s.knowledge.pedagogy_identity()?.0 != node(s.config.concept)?
            || s.knowledge.pedagogy_identity()?.1.context.get() != s.config.context
        {
            return Err("identidade conceitual incompatível".into());
        }
        if s.runtime
            .memory_contract()
            .is_some_and(|c| Some(c) != s.contract().ok())
        {
            return Err("contrato incompatível".into());
        }
        let interfaces = s
            .archived
            .iter()
            .map(|id| Ok((episode(*id)?, ports.clone())))
            .collect::<Result<Vec<_>>>()?;
        s.archive = EpisodeArchive::decode(sections[2], &interfaces)?;
        if s.archive.len() != s.archived.len() {
            return Err("inventário episódico incompatível".into());
        }
        if !sections[3].is_empty() {
            s.cursor = Some(ProbeSearch::restore(
                sections[3],
                &s.knowledge,
                s.contract()?,
            )?);
        }
        if s.fixed {
            s.world(s.current)?;
        }
        if s.encode()? != bytes {
            return Err("pacote não canônico".into());
        }
        Ok(s)
    }
}

pub fn run<R: BufRead, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    writeln!(output, "{HELP}")?;
    output.flush()?;
    let mut storage: Option<Storage> = None;
    let mut state: Option<State> = None;
    let mut errors = false;
    let mut ended = false;
    for _ in 0..256 {
        let mut buf = vec![];
        let n = input.by_ref().take(1025).read_until(b'\n', &mut buf)?;
        if n == 0 {
            ended = true;
            break;
        }
        if n > 1024 {
            return Err(io::Error::other("linha excedida"));
        }
        let text = std::str::from_utf8(&buf).map_err(io::Error::other)?;
        let v = text.split_whitespace().collect::<Vec<_>>();
        if v.is_empty() {
            continue;
        }
        let result: Result<String> = (|| match v.as_slice() {
            ["sair"] => Ok("Sessão encerrada; nenhuma ação repetida automaticamente".into()),
            ["ajuda"] => Ok(HELP.into()),
            ["armazenamento", path] => {
                if storage.is_some() || state.is_some() {
                    return Err("armazenamento já aberto".into());
                }
                storage = Some(Storage::open(Path::new(path))?);
                Ok("Armazenamento exclusivo aberto".into())
            }
            ["restaurar"] => {
                if state.is_some() {
                    return Err("estado já inicializado".into());
                }
                let disk = storage.as_ref().ok_or("abra armazenamento")?;
                state = Some(disk.restore()?);
                Ok("Restaurado sem autoridade; preparações descartadas; reservas consumidas preservadas".into())
            }
            ["estado"] => {
                let s = state.as_ref().ok_or("configure ou restaure")?;
                Ok(format!("Episódio {}; estado {}; ações totais {}; versão {}; autoridade {:?}; arquivo {}",s.episode,s.current,s.total_actions,s.knowledge.pedagogy_version()?.number,s.access,s.archive.len()))
            }
            ["recursos"] => {
                let s = state.as_ref().ok_or("configure ou restaure")?;
                Ok(format!("Reservas {:?}; atenção {}; intervenções {}; concessões emitidas {}; limites {:?}",s.runtime.cognitive_resources(),s.runtime.attention_scans(),s.runtime.action_attempts(),s.issued.len(),s.config.limits()))
            }
            _ => {
                let disk = storage
                    .as_mut()
                    .ok_or("abra armazenamento antes de mutar")?;
                if state.is_none() {
                    if disk.generation != 0 {
                        return Err("gerações existentes: restaure; não reconfigure".into());
                    }
                    let ["configurar", args @ ..] = v.as_slice() else {
                        return Err("configure primeiro".into());
                    };
                    let new = State::new(Config::parse(args)?)?;
                    disk.begin().map_err(|e| format!("FATAL: {e}"))?;
                    state = Some(new);
                    disk.commit(state.as_ref().unwrap())
                        .map_err(|e| format!("FATAL: {e}"))?;
                    return Ok("Runtime configurado sem autoridade".into());
                }
                disk.begin().map_err(|e| format!("FATAL: {e}"))?;
                let s = state.as_mut().unwrap();
                let result = s.command(&v);
                disk.commit(s).map_err(|e| format!("FATAL: {e}"))?;
                result
            }
        })();
        match result {
            Ok(text) => writeln!(output, "{text}")?,
            Err(e) => {
                writeln!(output, "Erro: {e}")?;
                errors = true;
                if e.starts_with("FATAL:") {
                    return Err(io::Error::other(e));
                }
            }
        }
        output.flush()?;
        if v == ["sair"] {
            ended = true;
            break;
        }
    }
    if !ended {
        return Err(io::Error::other(
            "limite de 256 entradas atingido; sessão interrompida",
        ));
    }
    if errors {
        Err(io::Error::other("sessão terminou com operações recusadas"))
    } else {
        Ok(())
    }
}
