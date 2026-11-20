use std::collections::{BTreeMap, BTreeSet};

use neurons::engine::{CycleReport, Engine, EngineError};
use neurons::event::{Event, EventKind, NodeId};
use neurons::neuron::{Neuron, NeuronError};
use neurons::synapse::{Synapse, SynapseError, SynapseId};

#[path = "dendritic_model.rs"]
mod dendritic_model;

use dendritic_model::DendriticNeuron;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegrityError {
    Engine(EngineError),
    Neuron(NeuronError),
    Synapse(SynapseError),
    InvalidTopology,
    InvalidCompartment,
    DuplicateAddress,
    DuplicateSynapse,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegrityReport {
    pub cycle: CycleReport,
    pub rejected_groups: u64,
    pub first_error: Option<IntegrityError>,
}

pub struct IntegrityAdapter {
    engine: Engine,
    source: Neuron,
    target: NodeId,
    dendrites: DendriticNeuron,
    routes: BTreeMap<NodeId, usize>,
    connections: Vec<(Synapse, NodeId)>,
    synapse_ids: BTreeSet<SynapseId>,
    capacity: usize,
    delivered: Vec<(NodeId, i32, u64)>,
    rejected_groups: u64,
    first_error: Option<IntegrityError>,
}

impl IntegrityAdapter {
    pub fn new(
        source: Neuron,
        target: NodeId,
        capacity: usize,
        cycle_limit: usize,
    ) -> Result<Self, IntegrityError> {
        if source.id() == target {
            return Err(IntegrityError::InvalidTopology);
        }

        let mut engine = Engine::new(capacity, cycle_limit).map_err(IntegrityError::Engine)?;

        engine.register_destination(source.id());

        Ok(Self {
            engine,
            source,
            target,
            dendrites: DendriticNeuron::new(2, 10, 1, 10)
                .expect("configuracao experimental valida"),
            routes: BTreeMap::new(),
            connections: Vec::new(),
            synapse_ids: BTreeSet::new(),
            capacity,
            delivered: Vec::new(),
            rejected_groups: 0,
            first_error: None,
        })
    }

    pub fn connect(
        &mut self,
        synapse: Synapse,
        address: NodeId,
        compartment: usize,
    ) -> Result<(), IntegrityError> {
        if synapse.source() != self.source.id() || synapse.destination() != self.target {
            return Err(IntegrityError::InvalidTopology);
        }

        if compartment >= 2 {
            return Err(IntegrityError::InvalidCompartment);
        }

        if self.synapse_ids.contains(&synapse.id()) {
            return Err(IntegrityError::DuplicateSynapse);
        }

        if address == self.source.id()
            || address == self.target
            || self.routes.contains_key(&address)
        {
            return Err(IntegrityError::DuplicateAddress);
        }

        self.engine.register_destination(address);
        self.routes.insert(address, compartment);
        self.synapse_ids.insert(synapse.id());
        self.connections.push((synapse, address));

        Ok(())
    }

    pub fn stimulate(&mut self, payload: i32, time: u64) -> Result<(), IntegrityError> {
        self.engine
            .enqueue(Event::new(
                None,
                self.source.id(),
                EventKind::Stimulus,
                payload,
                time,
            ))
            .map_err(IntegrityError::Engine)
    }

    pub fn run_cycle(&mut self) -> IntegrityReport {
        let source_id = self.source.id();
        let source = &mut self.source;
        let dendrites = &mut self.dendrites;
        let routes = &self.routes;
        let connections = &self.connections;
        let delivered = &mut self.delivered;

        // Contabilidade local: apenas este adaptador agenda eventos.
        let mut queued = self.engine.queue_len();
        let capacity = self.capacity;
        let mut rejected = 0_u64;
        let mut first_error = None;

        let cycle = self.engine.run_cycle(|event, sink| {
            // O Engine retirou um evento da fila.
            queued -= 1;

            if event.destination() == source_id {
                let emission = match source.receive(event) {
                    Ok(value) => value,
                    Err(error) => {
                        first_error.get_or_insert(IntegrityError::Neuron(error));
                        return;
                    }
                };

                let Some(emission) = emission else {
                    return;
                };

                // Preparacao integral antes de qualquer enqueue.
                let mut group = Vec::new();

                for (synapse, address) in connections {
                    let transmitted = match synapse.transmit(emission) {
                        Ok(value) => value,
                        Err(error) => {
                            first_error.get_or_insert(IntegrityError::Synapse(error));
                            rejected += 1;
                            return;
                        }
                    };

                    if let Some(signal) = transmitted {
                        group.push(Event::new(
                            signal.origin(),
                            *address,
                            signal.kind(),
                            signal.payload(),
                            signal.logical_time(),
                        ));
                    }
                }

                // Rejeita o grupo inteiro antes de alterar a fila.
                if group.len() > capacity - queued {
                    rejected += 1;
                    first_error.get_or_insert(IntegrityError::Engine(EngineError::QueueFull));
                    return;
                }

                // Todos os destinos foram registrados em connect().
                // Todos os tempos derivam de atrasos positivos.
                for signal in group {
                    sink.enqueue(signal)
                        .expect("grupo previamente validado e reservado");
                    queued += 1;
                }
            } else {
                let compartment = routes
                    .get(&event.destination())
                    .expect("destino dendritico registrado");

                assert_eq!(event.kind(), EventKind::Signal);
                assert_eq!(event.origin(), Some(source_id));

                dendrites
                    .receive(*compartment, event.payload())
                    .expect("compartimento validado");

                delivered.push((event.destination(), event.payload(), event.logical_time()));
            }
        });

        self.rejected_groups = self.rejected_groups.saturating_add(rejected);

        if self.first_error.is_none() {
            self.first_error = first_error;
        }

        IntegrityReport {
            cycle,
            rejected_groups: rejected,
            first_error,
        }
    }

    pub fn source_activations(&self) -> u64 {
        self.source.state().activations
    }

    pub fn activations(&self) -> u64 {
        self.dendrites.activations()
    }

    pub fn potentials(&self) -> [i32; 2] {
        self.dendrites.potentials()
    }

    pub fn delivered(&self) -> &[(NodeId, i32, u64)] {
        &self.delivered
    }

    pub fn queue_len(&self) -> usize {
        self.engine.queue_len()
    }

    pub fn rejected_groups(&self) -> u64 {
        self.rejected_groups
    }

    pub fn first_error(&self) -> Option<IntegrityError> {
        self.first_error
    }
}
