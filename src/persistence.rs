//! Checkpoint opcional e limitado de Engine + EngramStore. Não salva Circuit.
//! FNV-1a detecta corrupção acidental; não autentica arquivos não confiáveis.
use super::{Engine, EngineMetrics, QueuedEvent};
use crate::engram::{Cue, Engram, EngramId, EngramStore, EvidenceId, RecordStatus};
use crate::event::{Event, EventKind, NodeId};
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

pub const MAX_FILE_BYTES: usize = 256 * 1024;
pub const MAX_ITEMS: usize = 1024;
const MAX_CYCLE_LIMIT: usize = 1_000_000;
const MAGIC: &[u8; 8] = b"NEURONS\0";
const VERSION: u32 = 1;
const HEADER: usize = 24;

#[derive(Debug)]
pub enum PersistenceError {
    Io(io::Error),
    LimitExceeded,
    UnsupportedVersion,
    InvalidFormat,
    ChecksumMismatch,
    InvalidState,
}
impl From<io::Error> for PersistenceError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Escreve um único checkpoint, com temporário exclusivo ao lado do destino.
/// Não há autosave, histórico ou log crescente. Não substitui arquivo em erro
/// de codificação, escrita ou sync. A durabilidade após queda depende do FS.
pub fn save(path: &Path, engine: &Engine, memory: &EngramStore) -> Result<(), PersistenceError> {
    let bytes = encode(engine, memory)?;
    let mut name = path
        .file_name()
        .ok_or(PersistenceError::InvalidFormat)?
        .to_os_string();
    name.push(".neurons-tmp");
    let temporary = path.with_file_name(name);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    let result = (|| -> io::Result<()> {
        file.write_all(&bytes)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temporary, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result.map_err(PersistenceError::Io)
}

/// Constrói objetos novos apenas após validação. O chamador decide substituir
/// seu estado atual; uma leitura inválida não recebe referência mutável à sessão.
pub fn load(path: &Path) -> Result<(Engine, EngramStore), PersistenceError> {
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take((MAX_FILE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)?;
    decode(&bytes)
}

/// Codificação canônica, little-endian, sem depender do layout Rust.
pub fn encode(engine: &Engine, memory: &EngramStore) -> Result<Vec<u8>, PersistenceError> {
    validate_engine(engine)?;
    if memory.capacity() > MAX_ITEMS {
        return Err(PersistenceError::LimitExceeded);
    }
    let mut body = Vec::new();
    let mut put = |value: u64| body.extend_from_slice(&value.to_le_bytes());
    for value in [
        engine.queue_capacity as u64,
        engine.max_events_per_cycle as u64,
        engine.next_sequence,
        u64::from(engine.current_time.is_some()),
        engine.current_time.unwrap_or(0),
        engine.metrics.enqueued,
        engine.metrics.dispatched,
        engine.metrics.rejected,
        engine.metrics.peak_queue_len as u64,
        engine.destinations.len() as u64,
    ] {
        put(value);
    }
    for id in &engine.destinations {
        put(u64::from(id.get()));
    }
    let mut queue: Vec<_> = engine.queue.iter().map(|entry| entry.0).collect();
    queue.sort_by_key(|entry| (entry.event.logical_time(), entry.sequence));
    put(queue.len() as u64);
    for queued in queue {
        let event = queued.event;
        for value in [
            queued.sequence,
            event.origin().map_or(0, |id| u64::from(id.get())),
            u64::from(event.destination().get()),
            event.kind() as u64,
            u64::from(event.payload() as u32),
            event.logical_time(),
        ] {
            put(value);
        }
    }
    put(memory.capacity() as u64);
    put(memory.len() as u64);
    for record in memory.iter() {
        let status = match record.status() {
            RecordStatus::Observed => 0,
            RecordStatus::Hypothesis => 1,
            RecordStatus::Validated => 2,
        };
        for value in [
            u64::from(record.id().get()),
            u64::from(record.cue().context()),
            u64::from(record.cue().source().get()),
            u64::from(record.cue().pattern() as u32),
            u64::from(record.value() as u32),
            record.origin().map_or(0, |id| u64::from(id.get())),
            record.observed_at(),
            record.evidence().map_or(0, |id| u64::from(id.get())),
            status,
        ] {
            put(value);
        }
    }
    if body.len() + HEADER > MAX_FILE_BYTES {
        return Err(PersistenceError::LimitExceeded);
    }
    let mut bytes = Vec::with_capacity(body.len() + HEADER);
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&VERSION.to_le_bytes());
    bytes.extend_from_slice(&(body.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&checksum(&body).to_le_bytes());
    bytes.extend_from_slice(&body);
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<(Engine, EngramStore), PersistenceError> {
    if bytes.len() > MAX_FILE_BYTES {
        return Err(PersistenceError::LimitExceeded);
    }
    if bytes.len() < HEADER || &bytes[..8] != MAGIC {
        return Err(PersistenceError::InvalidFormat);
    }
    let version = u32::from_le_bytes(
        bytes[8..12]
            .try_into()
            .map_err(|_| PersistenceError::InvalidFormat)?,
    );
    if version != VERSION {
        return Err(PersistenceError::UnsupportedVersion);
    }
    let length = u32::from_le_bytes(
        bytes[12..16]
            .try_into()
            .map_err(|_| PersistenceError::InvalidFormat)?,
    ) as usize;
    if length != bytes.len() - HEADER {
        return Err(PersistenceError::InvalidFormat);
    }
    let expected = u64::from_le_bytes(
        bytes[16..24]
            .try_into()
            .map_err(|_| PersistenceError::InvalidFormat)?,
    );
    if checksum(&bytes[HEADER..]) != expected {
        return Err(PersistenceError::ChecksumMismatch);
    }
    let mut reader = Reader {
        bytes: &bytes[HEADER..],
    };
    let queue_capacity = reader.count(MAX_ITEMS)?;
    let max_events_per_cycle = reader.count(MAX_CYCLE_LIMIT)?;
    if queue_capacity == 0 || max_events_per_cycle == 0 {
        return Err(PersistenceError::InvalidState);
    }
    let next_sequence = reader.word()?;
    let time_flag = reader.word()?;
    let time = reader.word()?;
    let current_time = match (time_flag, time) {
        (0, 0) => None,
        (1, time) => Some(time),
        _ => return Err(PersistenceError::InvalidState),
    };
    let metrics = EngineMetrics {
        enqueued: reader.word()?,
        dispatched: reader.word()?,
        rejected: reader.word()?,
        peak_queue_len: reader.count(MAX_ITEMS)?,
    };
    let destination_count = reader.count(MAX_ITEMS)?;
    let mut destinations = BTreeSet::new();
    for _ in 0..destination_count {
        if !destinations.insert(reader.node()?) {
            return Err(PersistenceError::InvalidState);
        }
    }
    let queue_len = reader.count(MAX_ITEMS)?;
    let mut queue = BinaryHeap::new();
    for _ in 0..queue_len {
        let sequence = reader.word()?;
        let origin = reader.optional_node()?;
        let destination = reader.node()?;
        let kind = match reader.word()? {
            0 => EventKind::Stimulus,
            1 => EventKind::Signal,
            2 => EventKind::Control,
            _ => return Err(PersistenceError::InvalidState),
        };
        let payload = reader.u32()? as i32;
        let time = reader.word()?;
        queue.push(Reverse(QueuedEvent {
            sequence,
            event: Event::new(origin, destination, kind, payload, time),
        }));
    }
    let capacity = reader.count(MAX_ITEMS)?;
    let count = reader.count(MAX_ITEMS)?;
    if count > capacity {
        return Err(PersistenceError::InvalidState);
    }
    let mut memory = EngramStore::new(capacity).map_err(|_| PersistenceError::InvalidState)?;
    for _ in 0..count {
        let id = EngramId::new(reader.u32()?).map_err(|_| PersistenceError::InvalidState)?;
        let context = reader.u32()?;
        let source = reader.node()?;
        let pattern = reader.u32()? as i32;
        let cue = Cue::new(context, source, pattern).map_err(|_| PersistenceError::InvalidState)?;
        let value = reader.u32()? as i32;
        let origin = reader.optional_node()?;
        let time = reader.word()?;
        let evidence = match reader.u32()? {
            0 => None,
            id => Some(EvidenceId::new(id).map_err(|_| PersistenceError::InvalidState)?),
        };
        let status = match reader.word()? {
            0 => RecordStatus::Observed,
            1 => RecordStatus::Hypothesis,
            2 => RecordStatus::Validated,
            _ => return Err(PersistenceError::InvalidState),
        };
        let record = Engram::new(id, cue, value, origin, time, evidence, status)
            .map_err(|_| PersistenceError::InvalidState)?;
        memory
            .record(record)
            .map_err(|_| PersistenceError::InvalidState)?;
    }
    if !reader.bytes.is_empty() {
        return Err(PersistenceError::InvalidFormat);
    }
    let engine = Engine {
        queue,
        destinations,
        queue_capacity,
        max_events_per_cycle,
        next_sequence,
        current_time,
        metrics,
    };
    validate_engine(&engine)?;
    Ok((engine, memory))
}

fn validate_engine(engine: &Engine) -> Result<(), PersistenceError> {
    if engine.queue_capacity > MAX_ITEMS
        || engine.max_events_per_cycle > MAX_CYCLE_LIMIT
        || engine.destinations.len() > MAX_ITEMS
        || engine.queue.len() > MAX_ITEMS
    {
        return Err(PersistenceError::LimitExceeded);
    }
    if engine.queue_capacity == 0
        || engine.max_events_per_cycle == 0
        || engine.queue.len() > engine.queue_capacity
        || engine.metrics.peak_queue_len > engine.queue_capacity
        || engine.metrics.peak_queue_len < engine.queue.len()
        || engine.metrics.peak_queue_len as u64 > engine.metrics.enqueued
        || (engine.metrics.enqueued > 0 && engine.metrics.peak_queue_len == 0)
        || engine.metrics.enqueued != engine.next_sequence
        || engine.metrics.dispatched > engine.metrics.enqueued
        || engine.metrics.enqueued - engine.metrics.dispatched != engine.queue.len() as u64
        || (engine.metrics.dispatched > 0) != engine.current_time.is_some()
    {
        return Err(PersistenceError::InvalidState);
    }
    let mut sequences = BTreeSet::new();
    for queued in &engine.queue {
        let queued = queued.0;
        if !engine.destinations.contains(&queued.event.destination())
            || queued.sequence >= engine.next_sequence
            || !sequences.insert(queued.sequence)
            || engine
                .current_time
                .is_some_and(|time| queued.event.logical_time() < time)
        {
            return Err(PersistenceError::InvalidState);
        }
    }
    Ok(())
}

fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}
struct Reader<'a> {
    bytes: &'a [u8],
}
impl Reader<'_> {
    fn word(&mut self) -> Result<u64, PersistenceError> {
        let chunk = self.bytes.get(..8).ok_or(PersistenceError::InvalidFormat)?;
        let value = u64::from_le_bytes(
            chunk
                .try_into()
                .map_err(|_| PersistenceError::InvalidFormat)?,
        );
        self.bytes = &self.bytes[8..];
        Ok(value)
    }
    fn count(&mut self, max: usize) -> Result<usize, PersistenceError> {
        let value = self.word()?;
        if value > max as u64 {
            return Err(PersistenceError::LimitExceeded);
        }
        Ok(value as usize)
    }
    fn u32(&mut self) -> Result<u32, PersistenceError> {
        self.word()?
            .try_into()
            .map_err(|_| PersistenceError::InvalidState)
    }
    fn node(&mut self) -> Result<NodeId, PersistenceError> {
        NodeId::new(self.u32()?).map_err(|_| PersistenceError::InvalidState)
    }
    fn optional_node(&mut self) -> Result<Option<NodeId>, PersistenceError> {
        match self.u32()? {
            0 => Ok(None),
            id => Ok(Some(
                NodeId::new(id).map_err(|_| PersistenceError::InvalidState)?,
            )),
        }
    }
}
