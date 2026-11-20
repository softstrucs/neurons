use std::collections::{BTreeMap, BTreeSet};

use neurons::engine::{CycleReport, Engine, EngineError};
use neurons::event::{Event, EventKind, NodeId};
use neurons::neuron::{Neuron, NeuronError};
use neurons::synapse::{Synapse, SynapseError, SynapseId};

#[path = "dendritic_model.rs"]
mod dendritic_model;

use dendritic_model::DendriticNeuron;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterError {
    Engine(EngineError),
    Neuron(NeuronError),
    Synapse(SynapseError),
    DuplicateSynapse,
    DuplicateAddress,
    InvalidCompartment,
    WrongTarget,
}

pub struct DendriticEda {
    engine: Engine,
    source: Neuron,
    target: NodeId,
    dendrites: DendriticNeuron,
    routes: BTreeMap<NodeId, usize>,
    connections: Vec<(Synapse, NodeId)>,
    used_ids: BTreeSet<SynapseId>,
    activations: u64,
    delivered: Vec<(NodeId, i32, u64)>,
}

impl DendriticEda {
    pub fn new(
        source: Neuron,
        target: NodeId,
        queue_capacity: usize,
        cycle_limit: usize,
    ) -> Result<Self, AdapterError> {
        let engine = Engine::new(queue_capacity, cycle_limit).map_err(AdapterError::Engine)?;

        Ok(Self {
            engine,
            source,
            target,
            dendrites: DendriticNeuron::new(2, 10, 1, 10).unwrap(),
            routes: BTreeMap::new(),
            connections: Vec::new(),
            used_ids: BTreeSet::new(),
            activations: 0,
            delivered: Vec::new(),
        })
    }

    pub fn connect(
        &mut self,
        synapse: Synapse,
        address: NodeId,
        compartment: usize,
    ) -> Result<(), AdapterError> {
        if synapse.source() != self.source.id() || synapse.destination() != self.target {
            return Err(AdapterError::WrongTarget);
        }

        if compartment >= 2 {
            return Err(AdapterError::InvalidCompartment);
        }

        if self.used_ids.contains(&synapse.id()) {
            return Err(AdapterError::DuplicateSynapse);
        }

        if address == self.source.id()
            || address == self.target
            || self.routes.contains_key(&address)
        {
            return Err(AdapterError::DuplicateAddress);
        }

        self.engine.register_destination(address);
        self.routes.insert(address, compartment);
        self.used_ids.insert(synapse.id());
        self.connections.push((synapse, address));

        Ok(())
    }

    pub fn stimulate(&mut self, payload: i32, time: u64) -> Result<(), AdapterError> {
        // A fonte é um neurônio real de produção.
        let stimulus = Event::new(None, self.source.id(), EventKind::Stimulus, payload, time);

        let Some(emission) = self
            .source
            .receive(stimulus)
            .map_err(AdapterError::Neuron)?
        else {
            return Ok(());
        };

        // Cada sinapse possui um endereço dendrítico exclusivo.
        // A identidade da conexão não é inferida de Event.origin().
        for (synapse, address) in &self.connections {
            let transmitted = synapse.transmit(emission).map_err(AdapterError::Synapse)?;

            if let Some(event) = transmitted {
                let routed = Event::new(
                    event.origin(),
                    *address,
                    event.kind(),
                    event.payload(),
                    event.logical_time(),
                );

                self.engine.enqueue(routed).map_err(AdapterError::Engine)?;
            }
        }

        Ok(())
    }

    pub fn run_cycle(&mut self) -> CycleReport {
        let routes = &self.routes;
        let dendrites = &mut self.dendrites;
        let activations = &mut self.activations;
        let delivered = &mut self.delivered;

        self.engine.run_cycle(|event, _| {
            let compartment = routes[&event.destination()];

            let before = dendrites.activations();

            dendrites
                .receive(compartment, event.payload())
                .expect("rota validada na configuracao");

            *activations += dendrites.activations() - before;

            delivered.push((event.destination(), event.payload(), event.logical_time()));
        })
    }

    pub fn activations(&self) -> u64 {
        self.activations
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
}
