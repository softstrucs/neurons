use neurons::circuit::Circuit;
use neurons::circuit_persistence::{decode_v2, encode_v2, load_v2, save_v2};
use neurons::dendrite::{Dendrite, DendriteAddress, DendriteId};
use neurons::event::NodeId;
use neurons::neuron::Neuron;
use neurons::synapse::{Synapse, SynapseId};

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn node(id: u32) -> NodeId {
    NodeId::new(id).unwrap()
}

fn circuit() -> Circuit {
    let mut circuit = Circuit::new(32, 1).unwrap();

    circuit
        .add_neuron(Neuron::new(node(1), 2, 10).unwrap())
        .unwrap();
    circuit
        .add_neuron(Neuron::new(node(2), 2, 10).unwrap())
        .unwrap();

    let address = DendriteAddress {
        neuron: node(2),
        dendrite: DendriteId::new(1).unwrap(),
    };

    circuit
        .add_dendrite(address, Dendrite::new(2, 10).unwrap())
        .unwrap();

    circuit
        .connect_dendrite(
            Synapse::new(SynapseId::new(1).unwrap(), node(1), node(2), 1, 1).unwrap(),
            address,
        )
        .unwrap();

    circuit
}

fn pending_circuit() -> Circuit {
    let mut circuit = circuit();
    circuit.stimulate(node(1), 2, 0).unwrap();
    circuit.run_cycle();
    assert_eq!(circuit.pending_dendritic_count(), 1);
    circuit
}

fn temporary_path() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    std::env::temp_dir().join(format!(
        "neurons-d44p2-{}-{}.bin",
        std::process::id(),
        stamp
    ))
}

#[test]
fn v2_roundtrip_preserves_pending_state() {
    let original = pending_circuit();

    let bytes = encode_v2(&original).unwrap();
    let restored = decode_v2(&bytes).unwrap();

    assert_eq!(
        original.snapshot_v2().unwrap(),
        restored.snapshot_v2().unwrap()
    );
}

#[test]
fn v2_file_roundtrip_and_continuation() {
    let mut original = pending_circuit();
    let path = temporary_path();

    save_v2(&path, &original).unwrap();
    let result = load_v2(&path);
    let _ = std::fs::remove_file(&path);

    let mut restored = result.unwrap();

    for _ in 0..8 {
        assert_eq!(
            original.snapshot_v2().unwrap(),
            restored.snapshot_v2().unwrap()
        );
        original.run_cycle();
        restored.run_cycle();
    }
}

#[test]
fn v2_rejects_modified_payload() {
    let bytes = encode_v2(&pending_circuit()).unwrap();
    let mut corrupted = bytes.clone();

    let last = corrupted.len() - 1;
    corrupted[last] ^= 0x01;

    assert!(decode_v2(&corrupted).is_err());
    assert!(decode_v2(&bytes[..bytes.len() - 1]).is_err());
}

#[test]
fn v2_rejects_wrong_version_and_magic() {
    let bytes = encode_v2(&circuit()).unwrap();

    let mut wrong_version = bytes.clone();
    wrong_version[8..12].copy_from_slice(&3_u32.to_le_bytes());
    assert!(decode_v2(&wrong_version).is_err());

    let mut wrong_magic = bytes;
    wrong_magic[0] ^= 0x01;
    assert!(decode_v2(&wrong_magic).is_err());
}

#[test]
fn v2_rejects_oversized_input() {
    let oversized = vec![0_u8; 256 * 1024 + 1];
    assert!(decode_v2(&oversized).is_err());
}

#[test]
fn v2_encoding_is_deterministic() {
    let original = pending_circuit();

    assert_eq!(encode_v2(&original).unwrap(), encode_v2(&original).unwrap());
}
