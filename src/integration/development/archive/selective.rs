//! Seleção indexada de segmentos existentes; custos de indexação são separados.
//! Não muda NRNARC01 nem os tetos de quatro fontes/32 inspeções/oito evidências.
use super::*;
use std::collections::{BTreeMap, VecDeque};

const MAX_SEGMENTS: usize = 16;
type Context = (u32, u32, u32, u32);
fn context(g: PlanningGoal) -> Context {
    (g.world, g.revision, g.context, g.source.get())
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Location {
    segment: u32,
    episode: usize,
    trial: usize,
}
#[derive(Clone, Copy)]
struct Posting {
    location: Location,
    to: i32,
    before_at: u64,
    after_at: u64,
}
#[derive(Clone, Debug, Default)]
pub struct CatalogCost {
    pub retained_images: usize,
    pub retained_payload_bytes: usize,
    pub retained_payload_capacity: usize,
    pub unique_postings: usize,
    pub segments: usize,
    pub episodes: usize,
    pub trials: usize,
    pub image_bytes: usize,
    pub construction_nanos: u128,
}
#[derive(PartialEq, Eq)]
struct ModelKey {
    contract: ConsequenceContract,
    version: HypothesisVersion,
    examples: Vec<crate::integration::concepts::BoundExample>,
    history: Vec<HypothesisVersion>,
}
struct ModelGroup {
    key: ModelKey,
    origins: Vec<Location>,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SnapshotStorage {
    #[default]
    Coalesced,
    Separate,
}
#[derive(Default)]
pub struct EpisodeCatalog {
    storage: SnapshotStorage,
    canonical: BTreeMap<u32, Location>,
    snapshot_origins: BTreeMap<u32, Vec<EpisodeId>>,
    segments: BTreeMap<u32, EpisodeArchive>,
    outgoing: BTreeMap<(Context, i32), Vec<Posting>>,
    models: BTreeMap<Context, Vec<ModelGroup>>,
    context_counts: BTreeMap<Context, usize>,
    used_permits: Vec<u32>,
    used_trials: Vec<u32>,
    cost: CatalogCost,
}
impl EpisodeCatalog {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_storage(storage: SnapshotStorage) -> Self {
        Self {
            storage,
            ..Self::default()
        }
    }
    pub fn storage(&self) -> SnapshotStorage {
        self.storage
    }
    fn canonical_location(&self, l: Location) -> Location {
        self.canonical[&self.episode(l).id.get()]
    }
    pub fn cost(&self) -> &CatalogCost {
        &self.cost
    }
    pub fn segment_ids(&self) -> Vec<u32> {
        self.segments.keys().copied().collect()
    }
    pub fn segment(&self, id: u32) -> Option<&EpisodeArchive> {
        self.segments.get(&id)
    }
    /// Armazenamento segmentado explícito: até 16 arquivos existentes, cada um
    /// ainda com quatro episódios. Não é ampliação de orçamento de consulta.
    pub fn insert(&mut self, id: u32, mut archive: EpisodeArchive) -> Result<(), String> {
        let started = std::time::Instant::now();
        if id == 0 || self.segments.contains_key(&id) || self.segments.len() >= MAX_SEGMENTS {
            return Err("segmento duplicado/inválido ou catálogo cheio".into());
        }
        if archive.episodes.iter().any(|e| {
            self.segments
                .values()
                .any(|a| a.episodes.iter().any(|old| old.id == e.id))
        }) {
            return Err("identidade de episódio repetida entre segmentos".into());
        }
        let mut keys = vec![];
        for (i, e) in archive.episodes.iter().enumerate() {
            if let Some(c) = e.runtime.consequence_contract {
                let coord = e.session.concepts().ok_or("conceito ausente")?;
                let key = ModelKey {
                    contract: c,
                    version: e.session.pedagogy_version()?,
                    examples: coord.examples().map_err(|e| format!("{e:?}"))?,
                    history: coord
                        .models()
                        .iter()
                        .find(|m| m.scope() == c.scope)
                        .ok_or("histórico ausente")?
                        .versions()
                        .to_vec(),
                };
                keys.push((i, key));
            }
        }
        for i in 0..archive.episodes.len() {
            let (prior, rest) = archive.episodes.split_at_mut(i);
            let e = &mut rest[0];
            let local = Location {
                segment: id,
                episode: i,
                trial: 0,
            };
            let same = if self.storage == SnapshotStorage::Coalesced {
                self.segments
                    .values()
                    .flat_map(|a| a.episodes.iter())
                    .chain(prior.iter())
                    .find(|old| old.bytes == e.bytes)
                    .map(|old| (old.id, old.image.clone()))
            } else {
                None
            };
            let representative = if let Some((origin, image)) = same {
                e.image = image;
                self.canonical[&origin.get()]
            } else {
                local
            };
            let canonical_id = if representative == local {
                e.id
            } else if representative.segment == id {
                prior[representative.episode].id
            } else {
                self.episode(representative).id
            };
            self.canonical.insert(e.id.get(), representative);
            self.snapshot_origins
                .entry(canonical_id.get())
                .or_default()
                .push(e.id);
            self.cost.episodes += 1;
            self.cost.image_bytes += e.bytes.len();
            self.cost.trials += e.runtime.action_trials.len();
            self.used_permits.extend(&e.runtime.action_permits);
            self.used_trials
                .extend(e.runtime.action_trials.iter().map(|t| t.id));
            if representative != local {
                continue;
            }
            for (j, t) in e.runtime.action_trials.iter().enumerate() {
                self.cost.unique_postings += 1;
                let c = (t.world, t.revision, t.context, t.source.get());
                self.outgoing.entry((c, t.from)).or_default().push(Posting {
                    location: Location {
                        segment: id,
                        episode: i,
                        trial: j,
                    },
                    to: t.to,
                    before_at: e
                        .runtime
                        .experiences
                        .iter()
                        .find(|v| v.record == t.before)
                        .expect("snapshot validado")
                        .reading
                        .event
                        .logical_time(),
                    after_at: e
                        .runtime
                        .experiences
                        .iter()
                        .find(|v| v.record == t.after)
                        .expect("snapshot validado")
                        .reading
                        .event
                        .logical_time(),
                });
                *self.context_counts.entry(c).or_default() += 1;
            }
        }
        for (i, key) in keys {
            let c = key.contract;
            let groups = self
                .models
                .entry((c.world, c.revision, c.scope.context.get(), c.source.get()))
                .or_default();
            let loc = Location {
                segment: id,
                episode: i,
                trial: 0,
            };
            if let Some(group) = groups.iter_mut().find(|g| g.key == key) {
                group.origins.push(loc)
            } else {
                groups.push(ModelGroup {
                    key,
                    origins: vec![loc],
                })
            }
        }
        self.segments.insert(id, archive);
        let segments = &self.segments;
        for postings in self.outgoing.values_mut() {
            postings.sort_by_key(|p| {
                let l = p.location;
                (segments[&l.segment].episodes[l.episode].id.get(), l.trial)
            });
        }
        for groups in self.models.values_mut() {
            for group in groups {
                group
                    .origins
                    .sort_by_key(|l| segments[&l.segment].episodes[l.episode].id.get());
            }
        }
        for origins in self.snapshot_origins.values_mut() {
            origins.sort_by_key(|e| e.get());
            origins.dedup();
        }
        self.used_permits.sort_unstable();
        self.used_permits.dedup();
        self.used_trials.sort_unstable();
        self.used_trials.dedup();
        let mut pointers = std::collections::BTreeSet::new();
        self.cost.retained_images = 0;
        self.cost.retained_payload_bytes = 0;
        self.cost.retained_payload_capacity = 0;
        for e in self.segments.values().flat_map(|a| a.episodes.iter()) {
            if pointers.insert(std::sync::Arc::as_ptr(&e.image) as usize) {
                self.cost.retained_images += 1;
                self.cost.retained_payload_bytes += e.bytes.len();
                self.cost.retained_payload_capacity += e.bytes.capacity();
            }
        }
        self.cost.segments += 1;
        self.cost.construction_nanos += started.elapsed().as_nanos();
        Ok(())
    }
    fn episode(&self, l: Location) -> &ArchivedEpisode {
        &self.segments[&l.segment].episodes[l.episode]
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectiveQuery {
    pub goal: PlanningGoal,
    pub conceptual_seed: Option<i32>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchCoverage {
    pub limit: Option<SearchLimit>,
    pub indexed_episodes: usize,
    pub indexed_trials: usize,
    pub context_candidates: usize,
    pub considered_trials: usize,
    pub closed_states: Vec<i32>,
    pub pending_states: usize,
    pub pending_models: usize,
    pub pending_postings: usize,
    pub selected_episodes: usize,
    pub index_scans: usize,
    pub materialization_scans: usize,
    pub materialization_pending: bool,
    pub complete_for_goal: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchLimit {
    PageBudget,
    Attention,
    QueryBudget,
    SourceSlots,
    Entries,
}
/// O empréstimo impede mutar o catálogo enquanto a vista está em uso. Só
/// memory() empresta o resultado; planos/referências não são checkpoints ativos.
pub struct SelectiveSearch<'a> {
    limit: Option<SearchLimit>,
    catalog: &'a EpisodeCatalog,
    query: SelectiveQuery,
    memory: RetrievedMemory,
    models_done: usize,
    frontier: VecDeque<i32>,
    seen: Vec<i32>,
    closed: Vec<i32>,
    current: Option<(i32, usize)>,
    sources: Vec<Location>,
    entries_limit: usize,
    index_scans: usize,
    core_scans: usize,
    considered: usize,
    finished: bool,
}
impl SelectiveSearch<'_> {
    pub fn memory(&self) -> &RetrievedMemory {
        &self.memory
    }
    pub fn coverage(&self) -> SearchCoverage {
        let groups = self.catalog.models.get(&context(self.query.goal));
        let pending_postings = self.current.map_or(0, |(s, i)| {
            self.catalog
                .outgoing
                .get(&(context(self.query.goal), s))
                .map_or(0, |p| p.len().saturating_sub(i))
        });
        SearchCoverage {
            limit: self.limit,
            indexed_episodes: self.catalog.cost.episodes,
            indexed_trials: self.catalog.cost.trials,
            context_candidates: *self
                .catalog
                .context_counts
                .get(&context(self.query.goal))
                .unwrap_or(&0),
            considered_trials: self.considered,
            closed_states: self.closed.clone(),
            pending_states: self.frontier.len() + usize::from(self.current.is_some()),
            pending_models: groups.map_or(0, |g| g.len().saturating_sub(self.models_done)),
            pending_postings,
            selected_episodes: self.sources.len(),
            index_scans: self.index_scans,
            materialization_scans: self.core_scans,
            materialization_pending: !self.finished,
            complete_for_goal: self.finished && self.memory.status == RetrievalStatus::Complete,
        }
    }
    fn add_source(&mut self, l: Location) -> bool {
        let l = self.catalog.canonical_location(l);
        if self
            .sources
            .iter()
            .any(|old| old.segment == l.segment && old.episode == l.episode)
        {
            return true;
        }
        if self.sources.len() >= MAX_EPISODES {
            self.limit = Some(SearchLimit::SourceSlots);
            return false;
        }
        self.sources.push(l);
        true
    }
}
impl Runtime {
    pub fn select_evidence<'a>(
        &mut self,
        catalog: &'a EpisodeCatalog,
        session: &Session,
        query: SelectiveQuery,
        budget: RetrievalBudget,
    ) -> Result<SelectiveSearch<'a>, String> {
        let mut memory = self.retrieve_episodes(&[], session, query.goal, budget)?;
        memory.status = RetrievalStatus::ResourceLimited;
        let mut frontier = VecDeque::from([query.goal.from]);
        let mut seen = vec![query.goal.from];
        if let Some(seed) = query.conceptual_seed {
            if !seen.contains(&seed) {
                frontier.push_back(seed);
                seen.push(seed)
            }
        }
        let mut search = SelectiveSearch {
            limit: None,
            catalog,
            query,
            memory,
            models_done: 0,
            frontier,
            seen,
            closed: vec![],
            current: None,
            sources: vec![],
            entries_limit: budget.entries,
            index_scans: 0,
            core_scans: 0,
            considered: 0,
            finished: false,
        };
        self.resume_selection(session, &mut search, budget)?;
        Ok(search)
    }
    /// Amplia uma busca pendente, sem devolver inspeções já consumidas. O teto
    /// total da busca e a atenção da instância permanecem em 32.
    pub fn resume_selection(
        &mut self,
        session: &Session,
        search: &mut SelectiveSearch<'_>,
        budget: RetrievalBudget,
    ) -> Result<(), String> {
        if !self.access.read_knowledge
            || budget.scans > 32
            || budget.entries != search.entries_limit
            || budget.entries > 8
            || search.memory.basis != session.encode()
            || search.finished
        {
            return Err(
                "continuação não autorizada, obsoleta, finalizada ou orçamento incompatível".into(),
            );
        }
        let mut left = budget.scans;
        search.limit = None;
        let consume = |r: &mut Runtime, s: &mut SelectiveSearch<'_>, left: &mut usize| -> bool {
            if *left == 0 {
                s.limit = Some(SearchLimit::PageBudget);
                return false;
            }
            if r.attention_scans >= r.limits.attention_scans {
                s.limit = Some(SearchLimit::Attention);
                return false;
            }
            if s.index_scans + s.core_scans >= 32 {
                s.limit = Some(SearchLimit::QueryBudget);
                return false;
            }
            *left -= 1;
            r.attention_scans += 1;
            s.index_scans += 1;
            true
        };
        let ctx = context(search.query.goal);
        loop {
            let groups = search.catalog.models.get(&ctx);
            if let Some(group) = groups.and_then(|g| g.get(search.models_done)) {
                if !consume(self, search, &mut left) {
                    return Ok(());
                }
                if group.key.version.at <= session.logical_time()
                    && !search.add_source(group.origins[0])
                {
                    return Ok(());
                }
                search.models_done += 1;
                continue;
            }
            if let Some((state, index)) = search.current {
                let postings = search.catalog.outgoing.get(&(ctx, state));
                if let Some(post) = postings.and_then(|p| p.get(index)).copied() {
                    if !consume(self, search, &mut left) {
                        return Ok(());
                    }
                    search.considered += 1;
                    let scope = session.pedagogy_identity().ok().map(|(_, s)| s);
                    if post.after_at <= session.logical_time()
                        && scope.is_none_or(|s| {
                            s.validity.contains(post.after_at)
                                && s.validity.contains(post.before_at)
                        })
                    {
                        if !search.add_source(post.location) {
                            return Ok(());
                        }
                        if !search.seen.contains(&post.to) {
                            search.seen.push(post.to);
                            search.frontier.push_back(post.to)
                        }
                    }
                    search.current = Some((state, index + 1));
                    continue;
                }
                search.closed.push(state);
                search.current = None;
                continue;
            }
            if let Some(state) = search.frontier.front().copied() {
                if !consume(self, search, &mut left) {
                    return Ok(());
                }
                search.frontier.pop_front();
                if state != search.query.goal.to {
                    search.current = Some((state, 0))
                }
                continue;
            }
            // Reserva integral da materialização compartilhada, sem ler uma
            // parte e fingir cobertura completa. Não há leitor cognitivo novo.
            let needed: usize = search
                .sources
                .iter()
                .map(|l| 1 + search.catalog.episode(*l).runtime.action_trials.len())
                .sum();
            if needed > left {
                search.limit = Some(SearchLimit::PageBudget);
                return Ok(());
            }
            if self.attention_scans + needed > self.limits.attention_scans {
                search.limit = Some(SearchLimit::Attention);
                return Ok(());
            }
            if search.index_scans + search.core_scans + needed > 32 {
                search.limit = Some(SearchLimit::QueryBudget);
                return Ok(());
            }
            let mut episodes: Vec<_> = search
                .sources
                .iter()
                .map(|l| search.catalog.episode(*l))
                .collect();
            episodes.sort_by_key(|e| e.id.get());
            let mut memory = self.retrieve_episodes(
                &episodes,
                session,
                search.query.goal,
                RetrievalBudget {
                    scans: needed,
                    entries: budget.entries,
                },
            )?;
            search.core_scans += memory.scans;
            if memory.status == RetrievalStatus::ResourceLimited {
                search.limit = Some(SearchLimit::Entries)
            }
            // Índice completo dos consumos inclui segmentos omitidos. Não são
            // concessões nem são copiados para a autoridade do novo Runtime.
            memory.used_permits = search.catalog.used_permits.clone();
            memory.used_trials = search.catalog.used_trials.clone();
            if memory.status == RetrievalStatus::Complete {
                if let Some(groups) = search.catalog.models.get(&ctx) {
                    for group in groups {
                        if group.key.version.at > session.logical_time() {
                            continue;
                        }
                        let representative = search.catalog.episode(group.origins[0]);
                        memory.add_model(representative, session)?;
                        if let Some(v) = memory
                            .versions
                            .iter()
                            .find(|v| v.episode == representative.id)
                            .cloned()
                        {
                            for loc in &group.origins {
                                let id = search.catalog.episode(*loc).id;
                                if !memory.versions.iter().any(|old| old.episode == id) {
                                    memory.versions.push(RecoveredVersion {
                                        episode: id,
                                        ..v.clone()
                                    })
                                }
                            }
                        }
                    }
                }
            }
            // Origens são aliases de armazenamento, jamais votos adicionais.
            for entry in memory.entries.iter_mut().chain(memory.rejected.iter_mut()) {
                let original = std::mem::take(&mut entry.origins);
                for id in original {
                    if let Some(aliases) = search.catalog.snapshot_origins.get(&id.get()) {
                        entry.origins.extend(aliases)
                    } else {
                        entry.origins.push(id)
                    }
                }
                entry.origins.sort_by_key(|id| id.get());
                entry.origins.dedup();
            }
            memory.coverage = Some(GoalCoverage {
                closed: search.closed.clone(),
                goal: search.query.goal,
            });
            search.memory = memory;
            search.finished = true;
            return Ok(());
        }
    }
    /// Expansão explícita de estado não coberto após uma observação/divergência.
    /// Reutiliza o mesmo catálogo, cursor e saldo; nunca reinicia o orçamento.
    pub fn extend_selection(
        &mut self,
        session: &Session,
        search: &mut SelectiveSearch<'_>,
        state: i32,
        budget: RetrievalBudget,
    ) -> Result<(), String> {
        if search.memory.basis != session.encode()
            || !self.access.read_knowledge
            || budget.scans > 32
            || budget.entries > 8
            || budget.entries != search.entries_limit
            || !search.finished
            || search.memory.status != RetrievalStatus::Complete
            || search.seen.contains(&state)
            || state == search.query.goal.to
        {
            return Err("expansão incompatível/obsoleta".into());
        }
        search.finished = false;
        search.memory.status = RetrievalStatus::ResourceLimited;
        search.frontier.push_back(state);
        search.seen.push(state);
        self.resume_selection(session, search, budget)
    }
}
