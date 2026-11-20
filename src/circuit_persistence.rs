//! Persistencia independente do circuito dendritico.
//! Checkpoints v1 permanecem sob engine::persistence.

use crate::circuit::{Circuit, CircuitSnapshotV2};
use crate::dendrite::{DendriteAddress, DendriteId, DendriteSnapshot};
use crate::engine::persistence::PersistenceError;
use crate::event::NodeId;
use crate::neuron::{NeuronSnapshot, NeuronState};
use crate::synapse::SynapseId;

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const MAGIC: &[u8; 8] = b"NEURCIR\0";
const VERSION: u32 = 2;
const HEADER: usize = 24;
const MAX_BYTES: usize = 256 * 1024;
const MAX_ITEMS: usize = 1024;

fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

fn put(body: &mut Vec<u8>, value: u64) {
    body.extend_from_slice(&value.to_le_bytes());
}

fn count(body: &mut Vec<u8>, value: usize) -> Result<(), PersistenceError> {
    if value > MAX_ITEMS {
        return Err(PersistenceError::LimitExceeded);
    }
    put(body, value as u64);
    Ok(())
}

struct Reader<'a> {
    bytes: &'a [u8],
}

impl Reader<'_> {
    fn word(&mut self) -> Result<u64, PersistenceError> {
        if self.bytes.len() < 8 {
            return Err(PersistenceError::InvalidFormat);
        }
        let (head, tail) = self.bytes.split_at(8);
        self.bytes = tail;
        Ok(u64::from_le_bytes(head.try_into().unwrap()))
    }

    fn count(&mut self) -> Result<usize, PersistenceError> {
        let value = self.word()?;
        if value > MAX_ITEMS as u64 {
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

    fn dendrite(&mut self) -> Result<DendriteId, PersistenceError> {
        DendriteId::new(self.u32()?).ok_or(PersistenceError::InvalidState)
    }

    fn synapse(&mut self) -> Result<SynapseId, PersistenceError> {
        SynapseId::new(self.u32()?).map_err(|_| PersistenceError::InvalidState)
    }

    fn address(&mut self) -> Result<DendriteAddress, PersistenceError> {
        Ok(DendriteAddress {
            neuron: self.node()?,
            dendrite: self.dendrite()?,
        })
    }

    fn bytes(&mut self, length: usize) -> Result<Vec<u8>, PersistenceError> {
        if length > self.bytes.len() {
            return Err(PersistenceError::InvalidFormat);
        }
        let (head, tail) = self.bytes.split_at(length);
        self.bytes = tail;
        Ok(head.to_vec())
    }
}

pub fn encode_v2(circuit: &Circuit) -> Result<Vec<u8>, PersistenceError> {
    let snapshot = circuit.snapshot_v2()?;
    let mut body = Vec::new();

    count(&mut body, snapshot.neurons.len())?;
    for neuron in snapshot.neurons {
        put(&mut body, u64::from(neuron.id.get()));
        put(&mut body, neuron.threshold as u32 as u64);
        put(&mut body, neuron.state_limit as u32 as u64);
        put(&mut body, neuron.state.potential as u32 as u64);
        put(&mut body, neuron.state.received);
        put(&mut body, neuron.state.activations);
    }

    count(&mut body, snapshot.dendrites.len())?;
    for (address, dendrite) in snapshot.dendrites {
        put(&mut body, u64::from(address.neuron.get()));
        put(&mut body, u64::from(address.dendrite.get()));
        put(&mut body, dendrite.potential as u32 as u64);
        put(&mut body, dendrite.threshold as u32 as u64);
        put(&mut body, dendrite.limit as u32 as u64);
    }

    count(&mut body, snapshot.synapses.len())?;
    for (id, source, destination, weight, delay) in snapshot.synapses {
        put(&mut body, u64::from(id.get()));
        put(&mut body, u64::from(source.get()));
        put(&mut body, u64::from(destination.get()));
        put(&mut body, weight as u32 as u64);
        put(&mut body, delay);
    }

    count(&mut body, snapshot.dendritic_routes.len())?;
    for (id, address) in snapshot.dendritic_routes {
        put(&mut body, u64::from(id.get()));
        put(&mut body, u64::from(address.neuron.get()));
        put(&mut body, u64::from(address.dendrite.get()));
    }

    count(&mut body, snapshot.pending_dendritic.len())?;
    for (sequence, address) in snapshot.pending_dendritic {
        put(&mut body, sequence);
        put(&mut body, u64::from(address.neuron.get()));
        put(&mut body, u64::from(address.dendrite.get()));
    }

    if snapshot.engine.len() > MAX_BYTES {
        return Err(PersistenceError::LimitExceeded);
    }
    put(&mut body, snapshot.engine.len() as u64);
    body.extend_from_slice(&snapshot.engine);

    if body.len() + HEADER > MAX_BYTES {
        return Err(PersistenceError::LimitExceeded);
    }

    let mut bytes = Vec::with_capacity(HEADER + body.len());
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&VERSION.to_le_bytes());
    bytes.extend_from_slice(&(body.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&checksum(&body).to_le_bytes());
    bytes.extend_from_slice(&body);

    Ok(bytes)
}

pub fn decode_v2(bytes: &[u8]) -> Result<Circuit, PersistenceError> {
    if bytes.len() > MAX_BYTES {
        return Err(PersistenceError::LimitExceeded);
    }
    if bytes.len() < HEADER || &bytes[..8] != MAGIC {
        return Err(PersistenceError::InvalidFormat);
    }

    let version = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
    if version != VERSION {
        return Err(PersistenceError::UnsupportedVersion);
    }

    let length = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    if length != bytes.len() - HEADER {
        return Err(PersistenceError::InvalidFormat);
    }

    let expected = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
    if checksum(&bytes[HEADER..]) != expected {
        return Err(PersistenceError::ChecksumMismatch);
    }

    let mut reader = Reader {
        bytes: &bytes[HEADER..],
    };

    let mut neurons = Vec::new();
    for _ in 0..reader.count()? {
        neurons.push(NeuronSnapshot {
            id: reader.node()?,
            threshold: reader.u32()? as i32,
            state_limit: reader.u32()? as i32,
            state: NeuronState {
                potential: reader.u32()? as i32,
                received: reader.word()?,
                activations: reader.word()?,
            },
        });
    }

    let mut dendrites = Vec::new();
    for _ in 0..reader.count()? {
        let address = reader.address()?;
        let state = DendriteSnapshot {
            potential: reader.u32()? as i32,
            threshold: reader.u32()? as i32,
            limit: reader.u32()? as i32,
        };
        dendrites.push((address, state));
    }

    let mut synapses = Vec::new();
    for _ in 0..reader.count()? {
        synapses.push((
            reader.synapse()?,
            reader.node()?,
            reader.node()?,
            reader.u32()? as i32,
            reader.word()?,
        ));
    }

    let mut dendritic_routes = Vec::new();
    for _ in 0..reader.count()? {
        dendritic_routes.push((reader.synapse()?, reader.address()?));
    }

    let mut pending_dendritic = Vec::new();
    for _ in 0..reader.count()? {
        pending_dendritic.push((reader.word()?, reader.address()?));
    }

    let engine_length = reader.word()?;
    if engine_length > MAX_BYTES as u64 {
        return Err(PersistenceError::LimitExceeded);
    }
    let engine = reader.bytes(engine_length as usize)?;

    if !reader.bytes.is_empty() {
        return Err(PersistenceError::InvalidFormat);
    }

    Circuit::from_snapshot_v2(CircuitSnapshotV2 {
        neurons,
        dendrites,
        synapses,
        dendritic_routes,
        pending_dendritic,
        engine,
    })
}

pub fn save_v2(path: &Path, circuit: &Circuit) -> Result<(), PersistenceError> {
    let bytes = encode_v2(circuit)?;

    let filename = path.file_name().ok_or(PersistenceError::InvalidFormat)?;

    let parent = path.parent().unwrap_or_else(|| Path::new("."));

    let mut temporary = None;

    for attempt in 0..128_u32 {
        let mut name = filename.to_os_string();
        name.push(format!(
            ".neurons-v2-{}-{}.tmp",
            std::process::id(),
            attempt
        ));

        let candidate = parent.join(name);

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => {
                temporary = Some((candidate, file));
                break;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                continue;
            }
            Err(error) => return Err(PersistenceError::Io(error)),
        }
    }

    let (temporary_path, mut file) = temporary.ok_or(PersistenceError::LimitExceeded)?;

    let result = (|| -> std::io::Result<()> {
        file.write_all(&bytes)?;
        file.sync_all()?;
        drop(file);

        fs::rename(&temporary_path, path)?;

        // Persistir a alteracao do diretorio quando suportado.
        #[cfg(unix)]
        {
            fs::File::open(parent)?.sync_all()?;
        }

        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temporary_path);
    }

    result.map_err(PersistenceError::Io)
}

pub fn load_v2(path: &Path) -> Result<Circuit, PersistenceError> {
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take((MAX_BYTES + 1) as u64)
        .read_to_end(&mut bytes)?;

    decode_v2(&bytes)
}
