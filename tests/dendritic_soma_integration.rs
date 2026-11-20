use neurons::circuit::Circuit;
use neurons::dendrite::{Dendrite, DendriteAddress, DendriteId};
use neurons::event::NodeId;
use neurons::neuron::Neuron;
use neurons::synapse::{Synapse, SynapseId};

fn nid(value: u32) -> NodeId {
    NodeId::new(value).unwrap()
}

fn address(neuron: u32, dendrite: u32) -> DendriteAddress {
    DendriteAddress {
        neuron: nid(neuron),
        dendrite: DendriteId::new(dendrite).unwrap(),
    }
}

fn build(soma_threshold: i32, dendrite_threshold: i32, capacity: usize) -> Circuit {
    let mut c = Circuit::new(capacity, 1).unwrap();

    for (id, threshold) in [(1, 1), (2, soma_threshold), (3, 1)] {
        c.add_neuron(Neuron::new(nid(id), threshold, 10).unwrap())
            .unwrap();
    }

    c.add_dendrite(
        address(2, 1),
        Dendrite::new(dendrite_threshold, 10).unwrap(),
    )
    .unwrap();

    c.connect_dendrite(
        Synapse::new(SynapseId::new(1).unwrap(), nid(1), nid(2), 1, 1).unwrap(),
        address(2, 1),
    )
    .unwrap();

    c
}

fn drain(c: &mut Circuit) {
    for _ in 0..20 {
        if c.queue_len() == 0 {
            return;
        }
        let report = c.run_cycle();
        assert_eq!(report.first_error, None);
    }
    assert_eq!(c.queue_len(), 0, "Fila nao esvaziou");
}

#[test]
fn below_dendritic_threshold_does_not_reach_soma() {
    let mut c = build(1, 2, 8);

    c.stimulate(nid(1), 1, 0).unwrap();
    drain(&mut c);

    assert_eq!(c.dendrite_potential(address(2, 1)), Ok(1));
    assert_eq!(c.neuron_state(nid(2)).unwrap().received, 0);
    assert_eq!(c.neuron_state(nid(2)).unwrap().activations, 0);
}

#[test]
fn dendritic_threshold_generates_one_somatic_event() {
    let mut c = build(1, 1, 8);

    c.stimulate(nid(1), 1, 0).unwrap();

    let first = c.run_cycle();
    assert_eq!(first.transmissions, 1);
    assert_eq!(c.pending_dendritic_count(), 1);

    let second = c.run_cycle();
    assert_eq!(second.activations, 0);
    assert_eq!(c.pending_dendritic_count(), 0);
    assert_eq!(c.queue_len(), 1);
    assert_eq!(c.neuron_state(nid(2)).unwrap().received, 0);

    let third = c.run_cycle();
    assert_eq!(third.activations, 1);
    assert_eq!(c.neuron_state(nid(2)).unwrap().received, 1);
    assert_eq!(c.neuron_state(nid(2)).unwrap().activations, 1);
    assert_eq!(c.dendrite_potential(address(2, 1)), Ok(0));
}

#[test]
fn somatic_threshold_remains_independent() {
    let mut c = build(2, 1, 8);

    c.stimulate(nid(1), 1, 0).unwrap();
    drain(&mut c);

    assert_eq!(c.neuron_state(nid(2)).unwrap().potential, 1);
    assert_eq!(c.neuron_state(nid(2)).unwrap().activations, 0);

    c.stimulate(nid(1), 1, 10).unwrap();
    drain(&mut c);

    assert_eq!(c.neuron_state(nid(2)).unwrap().received, 2);
    assert_eq!(c.neuron_state(nid(2)).unwrap().activations, 1);
    assert_eq!(c.neuron_state(nid(2)).unwrap().potential, 0);
}

#[test]
fn somatic_activation_propagates_to_next_neuron() {
    let mut c = build(1, 1, 8);

    c.connect(Synapse::new(SynapseId::new(2).unwrap(), nid(2), nid(3), 1, 1).unwrap())
        .unwrap();

    c.stimulate(nid(1), 1, 0).unwrap();
    drain(&mut c);

    assert_eq!(c.neuron_state(nid(2)).unwrap().activations, 1);
    assert_eq!(c.neuron_state(nid(3)).unwrap().activations, 1);
    assert_eq!(c.pending_dendritic_count(), 0);
}

#[test]
fn full_queue_reuses_released_slot_for_somatic_event() {
    let mut c = build(1, 1, 1);

    c.stimulate(nid(1), 1, 0).unwrap();

    let source = c.run_cycle();
    assert_eq!(source.transmissions, 1);
    assert_eq!(c.queue_len(), 1);

    let dendrite = c.run_cycle();
    assert_eq!(dendrite.rejected_transmissions, 0);
    assert_eq!(dendrite.first_error, None);
    assert_eq!(c.queue_len(), 1);

    let soma = c.run_cycle();
    assert_eq!(soma.activations, 1);
    assert_eq!(c.queue_len(), 0);
}
