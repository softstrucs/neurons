//! Recuperação contextual finita sobre checkpoints operacionais existentes.
//! Não mescla modelos H23 nem restaura autoridade. Checksum não autentica origem.
use super::*;
use crate::curriculum::PedagogyHost;
use crate::learning::concepts::HypothesisVersion;

mod selective;
pub use selective::{
    CatalogCost, EpisodeCatalog, SearchCoverage, SearchLimit, SelectiveQuery, SelectiveSearch,
};

const MAX_EPISODES: usize = 4;
const MAX_IMAGE: usize = 128 * 1024;
const MAX_ARCHIVE: usize = MAX_EPISODES * MAX_IMAGE + 128;
const MAGIC: &[u8; 8] = b"NRNARC01";
mod sealed {
    pub trait Sealed {}
}
/// Uma porta para a memória existente, não um segundo raciocinador.
pub trait EpisodeMemory: sealed::Sealed {
    /// Projeção para o planejador. Em RetrievedMemory, id é referência
    /// transitória resolvível; não substitui o ID original da evidência.
    fn memory_trials(&self) -> Result<Vec<ActionTrial>, String>;
    fn memory_contract(&self) -> Option<ConsequenceContract>;
    fn memory_used(&self, request: &Intervention) -> bool;
    fn validate_memory(&self, session: &Session, goal: PlanningGoal) -> Result<(), String>;
    fn validate_plan_goal(&self, _: PlanningGoal) -> Result<(), String> {
        Ok(())
    }
    fn validate_prediction(&self, session: &Session, query: TransitionQuery) -> Result<(), String> {
        self.validate_memory(
            session,
            PlanningGoal {
                world: query.world,
                revision: query.revision,
                context: query.context,
                source: query.source,
                from: query.from,
                to: query.from,
            },
        )
    }
}
impl sealed::Sealed for Runtime {}
impl EpisodeMemory for Runtime {
    fn memory_trials(&self) -> Result<Vec<ActionTrial>, String> {
        Ok(self.action_trials.clone())
    }
    fn memory_contract(&self) -> Option<ConsequenceContract> {
        self.consequence_contract
    }
    fn memory_used(&self, r: &Intervention) -> bool {
        self.action_permits.contains(&r.permit.id)
            || self.action_trials.iter().any(|t| t.id == r.id)
    }
    fn validate_memory(&self, _: &Session, _: PlanningGoal) -> Result<(), String> {
        Ok(())
    }
}
struct ArchivedEpisode {
    id: EpisodeId,
    bytes: Vec<u8>,
    runtime: Runtime,
    session: Session,
}
#[derive(Default)]
pub struct EpisodeArchive {
    episodes: Vec<ArchivedEpisode>,
}
impl EpisodeArchive {
    /// Gravação explícita, sem sobrescrita; nenhuma persistência automática.
    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
        use std::io::Write;
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(path).map_err(|e| e.to_string())?;
        file.write_all(&self.encode()).map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())
    }
    pub fn load(
        path: &std::path::Path,
        interfaces: &[(EpisodeId, Vec<Sensor>)],
    ) -> Result<Self, String> {
        use std::io::Read;
        let metadata = std::fs::symlink_metadata(path).map_err(|e| e.to_string())?;
        if !metadata.is_file() || metadata.len() > MAX_ARCHIVE as u64 {
            return Err("arquivo regular limitado exigido".into());
        }
        let mut bytes = vec![];
        std::fs::File::open(path)
            .map_err(|e| e.to_string())?
            .take(MAX_ARCHIVE as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        Self::decode(&bytes, interfaces)
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.episodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.episodes.is_empty()
    }
    /// Captura imutável validada; reinsere a mesma imagem sem duplicá-la.
    pub fn insert(
        &mut self,
        id: EpisodeId,
        runtime: &Runtime,
        session: &Session,
    ) -> Result<(), String> {
        let bytes = runtime.checkpoint(session)?;
        if let Some(old) = self.episodes.iter().find(|e| e.id == id) {
            return if old.bytes == bytes {
                Ok(())
            } else {
                Err("episódio já registrado com outro conteúdo".into())
            };
        }
        if self.episodes.len() >= MAX_EPISODES {
            return Err("arquivo limitado a quatro episódios".into());
        }
        let (runtime, session) = Runtime::restore(&bytes, &runtime.sensors, Access::default())?;
        self.episodes.push(ArchivedEpisode {
            id,
            bytes,
            runtime,
            session,
        });
        self.episodes.sort_by_key(|e| e.id.get());
        Ok(())
    }
    pub fn encode(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        put(&mut out, self.len() as u64);
        for e in &self.episodes {
            put(&mut out, u64::from(e.id.get()));
            put(&mut out, e.bytes.len() as u64);
            out.extend(&e.bytes);
        }
        let sum = checkpoint::checksum(&out);
        put(&mut out, sum);
        out
    }
    /// Interfaces são reinstaladas pelo chamador. Access dos arquivos é sempre
    /// negado; nenhum episódio ativo, fonte executável ou concessão é restaurado.
    pub fn decode(bytes: &[u8], interfaces: &[(EpisodeId, Vec<Sensor>)]) -> Result<Self, String> {
        if bytes.len() < 24 || bytes.len() > MAX_ARCHIVE || bytes.get(..8) != Some(MAGIC.as_slice())
        {
            return Err("arquivo de episódios inválido".into());
        }
        let end = bytes.len() - 8;
        if checkpoint::checksum(&bytes[..end])
            != u64::from_le_bytes(bytes[end..].try_into().unwrap())
        {
            return Err("checksum do arquivo inválido".into());
        }
        let mut r = Reader {
            bytes: &bytes[..end],
            at: 8,
        };
        let count = r.number()?;
        if count > MAX_EPISODES as u64 || interfaces.len() != count as usize {
            return Err("interfaces/quantidade incompatíveis".into());
        }
        let mut out = Self::new();
        for _ in 0..count {
            let id = EpisodeId::new(u32::try_from(r.number()?).map_err(|_| "ID excedido")?)
                .map_err(|e| format!("{e:?}"))?;
            let size = usize::try_from(r.number()?).map_err(|_| "imagem excedida")?;
            if size > MAX_IMAGE {
                return Err("imagem excedida".into());
            }
            let image = r.take(size)?;
            let ports: Vec<_> = interfaces.iter().filter(|(i, _)| *i == id).collect();
            if ports.len() != 1 || out.episodes.iter().any(|e| e.id == id) {
                return Err("identidade de episódio duplicada/ausente".into());
            }
            let (runtime, session) = Runtime::restore(image, &ports[0].1, Access::default())?;
            out.episodes.push(ArchivedEpisode {
                id,
                bytes: image.to_vec(),
                runtime,
                session,
            });
        }
        out.episodes.sort_by_key(|e| e.id.get());
        if r.at != end || out.encode() != bytes {
            return Err("arquivo não canônico".into());
        }
        Ok(out)
    }
}
fn put(out: &mut Vec<u8>, v: u64) {
    out.extend(v.to_le_bytes())
}
struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}
impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self.at.checked_add(n).ok_or("offset excedido")?;
        let bytes = self.bytes.get(self.at..end).ok_or("arquivo truncado")?;
        self.at = end;
        Ok(bytes)
    }
    fn number(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetrievalBudget {
    pub scans: usize,
    pub entries: usize,
}
impl Default for RetrievalBudget {
    fn default() -> Self {
        Self {
            scans: 32,
            entries: 8,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetrievalStatus {
    Complete,
    ResourceLimited,
    AmbiguousIdentity,
    ConflictingVersions,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VersionRelation {
    Current,
    Ancestor,
    Incompatible,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveredVersion {
    pub episode: EpisodeId,
    pub version: HypothesisVersion,
    pub relation: VersionRelation,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveredTransition {
    /// Referência transitória da consulta; não é identidade de fato ou concessão.
    pub reference: u32,
    pub origins: Vec<EpisodeId>,
    pub trial: ActionTrial,
    pub before: Reading,
    pub after: Reading,
}
/// Resultado completo ou recusa explícita; uma busca truncada nunca é prova.
struct GoalCoverage {
    closed: Vec<i32>,
    goal: PlanningGoal,
}
pub struct RetrievedMemory {
    coverage: Option<GoalCoverage>,
    status: RetrievalStatus,
    goal: PlanningGoal,
    entries: Vec<RecoveredTransition>,
    versions: Vec<RecoveredVersion>,
    contract: Option<ConsequenceContract>,
    basis: Vec<u8>,
    scans: usize,
    omitted: usize,
    used_permits: Vec<u32>,
    used_trials: Vec<u32>,
    rejected: Option<RecoveredTransition>,
}
impl RetrievedMemory {
    fn validate_basis(&self, s: &Session, g: PlanningGoal) -> Result<(), String> {
        self.memory_trials()?;
        if s.encode() != self.basis
            || g.world != self.goal.world
            || g.revision != self.goal.revision
            || g.context != self.goal.context
            || g.source != self.goal.source
        {
            return Err("recuperação obsoleta ou de outro contexto".into());
        }
        Ok(())
    }
    fn add_model(&mut self, e: &ArchivedEpisode, session: &Session) -> Result<(), String> {
        if self.versions.iter().any(|v| v.episode == e.id) {
            return Ok(());
        }
        if let Some(c) = e.runtime.consequence_contract {
            let version = e.session.pedagogy_version()?;
            let active = session.pedagogy_version()?;
            let old_examples = e
                .session
                .concepts()
                .ok_or("conceito de origem ausente")?
                .examples()
                .map_err(|e| format!("{e:?}"))?;
            let active_examples = session
                .concepts()
                .ok_or("conceito atual ausente")?
                .examples()
                .map_err(|e| format!("{e:?}"))?;
            // IDs/contadores iguais não bastam para provar ancestralidade:
            // conteúdo, procedência e histórico da versão precisam coincidir.
            let lineage = old_examples.iter().all(|old| active_examples.contains(old))
                && session
                    .concepts()
                    .unwrap()
                    .models()
                    .iter()
                    .any(|m| m.scope() == c.scope && m.versions().contains(&version));
            let relation = if lineage && version == active {
                VersionRelation::Current
            } else if lineage
                && version.number < active.number
                && version.support.iter().all(|x| active.support.contains(x))
                && active.criteria.iter().all(|x| version.criteria.contains(x))
            {
                VersionRelation::Ancestor
            } else {
                VersionRelation::Incompatible
            };
            if relation == VersionRelation::Incompatible
                || session.pedagogy_identity()? != (c.concept, c.scope)
                || self.contract.is_some_and(|old| old != c)
            {
                self.status = RetrievalStatus::ConflictingVersions;
            }
            self.versions.push(RecoveredVersion {
                episode: e.id,
                version,
                relation,
            });
            self.contract = Some(c);
        }
        Ok(())
    }
    pub fn resolve_reference(&self, reference: u32) -> Option<&RecoveredTransition> {
        self.entries
            .iter()
            .find(|entry| entry.reference == reference)
    }
    pub fn rejected_evidence(&self) -> Option<&RecoveredTransition> {
        self.rejected.as_ref()
    }
    pub fn status(&self) -> RetrievalStatus {
        self.status
    }
    pub fn entries(&self) -> &[RecoveredTransition] {
        &self.entries
    }
    pub fn versions(&self) -> &[RecoveredVersion] {
        &self.versions
    }
    pub fn scans(&self) -> usize {
        self.scans
    }
    pub fn omitted(&self) -> usize {
        self.omitted
    }
}
impl sealed::Sealed for RetrievedMemory {}
impl EpisodeMemory for RetrievedMemory {
    fn memory_trials(&self) -> Result<Vec<ActionTrial>, String> {
        if self.status != RetrievalStatus::Complete {
            return Err(format!("recuperação não utilizável: {:?}", self.status));
        }
        // A projeção para o planejador conserva o original em entries. Nenhum
        // ID de origem é reescrito/persistido; referências só valem nesta vista.
        Ok(self
            .entries
            .iter()
            .map(|e| {
                let mut projected = e.trial.clone();
                projected.id = e.reference;
                projected
            })
            .collect())
    }
    fn memory_contract(&self) -> Option<ConsequenceContract> {
        self.contract
    }
    fn memory_used(&self, r: &Intervention) -> bool {
        self.used_permits.contains(&r.permit.id)
            || self.used_trials.contains(&r.id)
            || self.resolve_reference(r.id).is_some()
    }
    fn validate_memory(&self, s: &Session, g: PlanningGoal) -> Result<(), String> {
        self.validate_basis(s, g)?;
        self.validate_plan_goal(g)
    }
    fn validate_plan_goal(&self, g: PlanningGoal) -> Result<(), String> {
        if let Some(c) = &self.coverage {
            if g.to != c.goal.to || (!c.closed.contains(&g.from) && g.from != g.to) {
                return Err("cobertura seletiva insuficiente para este objetivo/estado".into());
            }
        }
        Ok(())
    }
    fn validate_prediction(&self, s: &Session, q: TransitionQuery) -> Result<(), String> {
        self.validate_basis(
            s,
            PlanningGoal {
                world: q.world,
                revision: q.revision,
                context: q.context,
                source: q.source,
                from: q.from,
                to: q.from,
            },
        )?;
        if self
            .coverage
            .as_ref()
            .is_some_and(|c| !c.closed.contains(&q.from))
        {
            return Err("estado fora da cobertura da recuperação seletiva".into());
        }
        Ok(())
    }
}
impl Runtime {
    /// Atenção compartilhada com seleção de ações, inclusive para episódios
    /// irrelevantes. Não altera arquivos de origem nem funde hipóteses H23.
    pub fn retrieve_context(
        &mut self,
        archive: &EpisodeArchive,
        session: &Session,
        goal: PlanningGoal,
        budget: RetrievalBudget,
    ) -> Result<RetrievedMemory, String> {
        let episodes: Vec<_> = archive.episodes.iter().collect();
        self.retrieve_episodes(&episodes, session, goal, budget)
    }
    fn retrieve_episodes(
        &mut self,
        episodes: &[&ArchivedEpisode],
        session: &Session,
        goal: PlanningGoal,
        budget: RetrievalBudget,
    ) -> Result<RetrievedMemory, String> {
        if !self.access.read_knowledge
            || budget.scans > 32
            || budget.entries > 8
            || goal.world == 0
            || goal.revision == 0
            || goal.context == 0
            || goal.context == u32::MAX
        {
            return Err("recuperação não autorizada/inválida".into());
        }
        let scope = session.pedagogy_identity().ok().map(|(_, scope)| scope);
        if scope.is_some_and(|s| {
            s.context.get() != goal.context || !s.validity.contains(session.logical_time())
        }) {
            return Err("contexto/validade da sessão incompatível".into());
        }
        let mut out = RetrievedMemory {
            status: RetrievalStatus::Complete,
            goal,
            entries: vec![],
            versions: vec![],
            contract: None,
            basis: session.encode(),
            scans: 0,
            omitted: 0,
            used_permits: vec![],
            used_trials: vec![],
            rejected: None,
            coverage: None,
        };
        // Somente tombstones de consumo, não permissões restauráveis. Incluem
        // fontes irrelevantes para impedir reciclar autoridade omitida na busca.
        for e in episodes {
            out.used_permits.extend(&e.runtime.action_permits);
            out.used_trials
                .extend(e.runtime.action_trials.iter().map(|t| t.id));
        }
        for e in episodes {
            if !self.retrieval_scan(&mut out, budget) {
                return Ok(out);
            }
            let mut relevant = false;
            for trial in &e.runtime.action_trials {
                if !self.retrieval_scan(&mut out, budget) {
                    return Ok(out);
                }
                let before = e
                    .runtime
                    .experiences
                    .iter()
                    .find(|v| v.record == trial.before)
                    .ok_or("premissa anterior ausente")?
                    .reading;
                let after = e
                    .runtime
                    .experiences
                    .iter()
                    .find(|v| v.record == trial.after)
                    .ok_or("consequência ausente")?
                    .reading;
                if trial.world != goal.world
                    || trial.revision != goal.revision
                    || trial.context != goal.context
                    || trial.source != goal.source
                    || after.event.logical_time() > session.logical_time()
                    || scope.is_some_and(|s| {
                        !s.validity.contains(before.event.logical_time())
                            || !s.validity.contains(after.event.logical_time())
                    })
                {
                    out.omitted += 1;
                    continue;
                }
                relevant = true;
                if let Some(old) = out.entries.iter_mut().find(|p| {
                    p.trial.permit == trial.permit
                        || p.before.evidence == before.evidence
                        || p.after.evidence == after.evidence
                        || p.before.evidence == after.evidence
                        || p.after.evidence == before.evidence
                }) {
                    if old.trial == *trial && old.before == before && old.after == after {
                        old.origins.push(e.id);
                        continue;
                    }
                    out.status = RetrievalStatus::AmbiguousIdentity;
                    out.rejected = Some(RecoveredTransition {
                        reference: u32::MAX - out.entries.len() as u32,
                        origins: vec![e.id],
                        trial: trial.clone(),
                        before,
                        after,
                    });
                    return Ok(out);
                }
                if out.entries.len() >= budget.entries {
                    out.status = RetrievalStatus::ResourceLimited;
                    return Ok(out);
                }
                out.entries.push(RecoveredTransition {
                    reference: u32::MAX - out.entries.len() as u32,
                    origins: vec![e.id],
                    trial: trial.clone(),
                    before,
                    after,
                });
            }
            if relevant {
                out.add_model(e, session)?;
            }
        }
        Ok(out)
    }
    fn retrieval_scan(&mut self, out: &mut RetrievedMemory, budget: RetrievalBudget) -> bool {
        if out.scans >= budget.scans || self.attention_scans >= self.limits.attention_scans {
            out.status = RetrievalStatus::ResourceLimited;
            return false;
        }
        out.scans += 1;
        self.attention_scans += 1;
        true
    }
    /// IDs de suporte são referências desta vista: use resolve_reference.
    pub fn predict_retrieved(
        &mut self,
        session: &Session,
        query: TransitionQuery,
        memory: &RetrievedMemory,
    ) -> Result<Prediction, String> {
        memory.validate_prediction(session, query)?;
        self.predict_using(query, &memory.memory_trials()?)
    }
    /// As provas usam referências desta vista e preservam as origens em entries.
    pub fn plan_retrieved(
        &mut self,
        session: &Session,
        goal: PlanningGoal,
        budget: PlanningBudget,
        memory: &RetrievedMemory,
    ) -> Result<PlanOutcome, String> {
        memory.validate_memory(session, goal)?;
        self.plan_using(goal, budget, &memory.memory_trials()?)
    }
    /// Continuação explícita com permissões frescas e saldo corrente. O plano
    /// continua sendo o do motor existente; oposição local invalida a execução.
    pub fn execute_retrieved_plan(
        &mut self,
        session: &Session,
        world: &mut World,
        plan: &Plan,
        requests: &[Intervention],
        memory: &RetrievedMemory,
    ) -> Result<Execution, String> {
        if !self.access.read_knowledge
            || !self.access.explore
            || !self.access.record
            || requests.len() > 4
        {
            return Err("execução recuperada não autorizada/inválida".into());
        }
        memory.validate_memory(session, plan.goal())?;
        if requests.iter().any(|r| memory.memory_used(r)) {
            return Err("identidade/concessão recuperada não é autoridade nova".into());
        }
        let combined = self.remembered_trials(memory)?;
        for step in plan.steps() {
            let q = TransitionQuery {
                world: step.world,
                revision: step.revision,
                context: step.context,
                source: step.source,
                from: step.from,
                action: step.action,
            };
            if Self::outcomes_in(&combined, q).len() > 1 {
                return Err("oposição local invalida o plano recuperado".into());
            }
        }
        let actions: Vec<_> = plan.steps().iter().map(|s| s.action).collect();
        let chosen = self
            .choose_requests(world, &actions, requests)
            .map_err(|e| format!("{e:?}"))?;
        Ok(self.execute_plan_using(world, plan, &chosen, &memory.memory_trials()?))
    }
}
