//! Composição experimental pequena de neurônios, sinapses e motor de eventos.

use std::collections::BTreeMap;

use crate::dendrite::{DeliveryTarget, Dendrite, DendriteAddress, DendriteError};

use crate::engine::{CycleReport, Engine, EngineError, EngineMetrics};
use crate::event::{Event, EventKind, NodeId};
use crate::neuron::{Neuron, NeuronError, NeuronState};
use crate::synapse::{Synapse, SynapseError, SynapseId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CircuitError {
    Engine(EngineError),
    Neuron(NeuronError),
    Synapse(SynapseError),
    DuplicateNeuron(NodeId),
    UnknownNeuron(NodeId),
    DuplicateSynapse(SynapseId),
    UnknownSynapse(SynapseId),
    Dendrite(DendriteError),
    UnknownDendrite(DendriteAddress),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CircuitReport {
    pub cycle: CycleReport,
    pub activations: u64,
    pub transmissions: u64,
    pub rejected_transmissions: u64,
    pub first_error: Option<CircuitError>,
}

/// Apenas os neurônios endereçados por eventos são processados.
pub struct Circuit {
    engine: Engine,
    neurons: BTreeMap<NodeId, Neuron>,
    synapses: Vec<Synapse>,
    dendrites: BTreeMap<DendriteAddress, Dendrite>,
    dendritic_routes: BTreeMap<SynapseId, DendriteAddress>,
    pending_dendritic: BTreeMap<u64, DendriteAddress>,
}

impl Circuit {
    pub fn new(queue_capacity: usize, max_events_per_cycle: usize) -> Result<Self, CircuitError> {
        Ok(Self {
            engine: Engine::new(queue_capacity, max_events_per_cycle)
                .map_err(CircuitError::Engine)?,
            neurons: BTreeMap::new(),
            synapses: Vec::new(),
            dendrites: BTreeMap::new(),
            dendritic_routes: BTreeMap::new(),
            pending_dendritic: BTreeMap::new(),
        })
    }

    pub fn add_neuron(&mut self, neuron: Neuron) -> Result<(), CircuitError> {
        let id = neuron.id();
        if self.neurons.contains_key(&id) {
            return Err(CircuitError::DuplicateNeuron(id));
        }
        self.engine.register_destination(id);
        self.neurons.insert(id, neuron);
        Ok(())
    }

    /// Registra um compartimento dendritico em um neuronio existente.
    pub fn add_dendrite(
        &mut self,
        address: DendriteAddress,
        dendrite: Dendrite,
    ) -> Result<(), CircuitError> {
        if !self.neurons.contains_key(&address.neuron) {
            return Err(CircuitError::UnknownNeuron(address.neuron));
        }

        if self.dendrites.contains_key(&address) {
            return Err(CircuitError::Dendrite(DendriteError::DuplicateDendrite));
        }

        self.dendrites.insert(address, dendrite);
        Ok(())
    }

    /// Consulta o potencial sem modificar o compartimento.
    pub fn dendrite_potential(&self, address: DendriteAddress) -> Result<i32, CircuitError> {
        self.dendrites
            .get(&address)
            .map(Dendrite::potential)
            .ok_or(CircuitError::UnknownDendrite(address))
    }

    pub fn dendrite_count(&self) -> usize {
        self.dendrites.len()
    }

    pub fn connect(&mut self, synapse: Synapse) -> Result<(), CircuitError> {
        for id in [synapse.source(), synapse.destination()] {
            if !self.neurons.contains_key(&id) {
                return Err(CircuitError::UnknownNeuron(id));
            }
        }
        if self.synapses.iter().any(|item| item.id() == synapse.id()) {
            return Err(CircuitError::DuplicateSynapse(synapse.id()));
        }
        self.synapses.push(synapse);
        Ok(())
    }

    /// Registra uma sinapse com entrega dendritica explicita.
    pub fn connect_dendrite(
        &mut self,
        synapse: Synapse,
        address: DendriteAddress,
    ) -> Result<(), CircuitError> {
        if synapse.destination() != address.neuron {
            return Err(CircuitError::Dendrite(DendriteError::WrongNeuron));
        }
        if !self.dendrites.contains_key(&address) {
            return Err(CircuitError::UnknownDendrite(address));
        }
        let id = synapse.id();
        self.connect(synapse)?;
        self.dendritic_routes.insert(id, address);
        Ok(())
    }

    /// Consulta o destino configurado para uma sinapse.
    pub fn delivery_target(&self, id: SynapseId) -> Result<DeliveryTarget, CircuitError> {
        let synapse = self
            .synapses
            .iter()
            .find(|item| item.id() == id)
            .ok_or(CircuitError::UnknownSynapse(id))?;

        Ok(match self.dendritic_routes.get(&id) {
            Some(address) => DeliveryTarget::Dendrite(*address),
            None => DeliveryTarget::Soma(synapse.destination()),
        })
    }

    /// Quantidade de entregas dendriticas ainda na fila.
    pub fn pending_dendritic_count(&self) -> usize {
        self.pending_dendritic.len()
    }

    /// Alteração de peso explícita, sem atualização implícita durante o despacho.
    pub fn set_weight(&mut self, id: SynapseId, weight: i32) -> Result<(), CircuitError> {
        let synapse = self
            .synapses
            .iter_mut()
            .find(|item| item.id() == id)
            .ok_or(CircuitError::UnknownSynapse(id))?;
        synapse.set_weight(weight);
        Ok(())
    }

    /// Consulta o peso atual para regras externas de atualização.
    pub fn synapse_weight(&self, id: SynapseId) -> Result<i32, CircuitError> {
        self.synapses
            .iter()
            .find(|item| item.id() == id)
            .map(Synapse::weight)
            .ok_or(CircuitError::UnknownSynapse(id))
    }

    pub fn stimulate(
        &mut self,
        destination: NodeId,
        payload: i32,
        logical_time: u64,
    ) -> Result<(), CircuitError> {
        self.engine
            .enqueue(Event::new(
                None,
                destination,
                EventKind::Stimulus,
                payload,
                logical_time,
            ))
            .map_err(CircuitError::Engine)
    }

    /// Processa um ciclo do motor. Falhas de transmissão são contadas e
    /// relatadas; os demais eventos do ciclo seguem em ordem determinística.
    pub fn run_cycle(&mut self) -> CircuitReport {
        let neurons = &mut self.neurons;
        let synapses = &self.synapses;
        let dendrites = &mut self.dendrites;
        let dendritic_routes = &self.dendritic_routes;
        let pending_dendritic = &mut self.pending_dendritic;
        let mut activations: u64 = 0;
        let mut transmissions: u64 = 0;
        let mut rejected_transmissions: u64 = 0;
        let mut first_error = None;

        let cycle = self.engine.run_cycle_identified(|sequence, event, sink| {
            if let Some(address) = pending_dendritic.remove(&sequence) {
                match dendrites.get_mut(&address) {
                    Some(dendrite) => {
                        if dendrite.receive(event.payload()) {
                            // Transferencia dendrito-soma pela fila do Engine.
                            // A identidade e atribuida somente se aceita.
                            let somatic_event = Event::new(
                                Some(address.neuron),
                                address.neuron,
                                EventKind::Signal,
                                1,
                                event.logical_time(),
                            );

                            if let Err(error) = sink.enqueue_identified(somatic_event) {
                                rejected_transmissions = rejected_transmissions.saturating_add(1);
                                first_error.get_or_insert(CircuitError::Engine(error));
                            }
                        }
                    }
                    None => {
                        first_error.get_or_insert(CircuitError::UnknownDendrite(address));
                    }
                }
                return;
            }

            let Some(neuron) = neurons.get_mut(&event.destination()) else {
                first_error.get_or_insert(CircuitError::UnknownNeuron(event.destination()));
                return;
            };
            let emission = match neuron.receive(event) {
                Ok(emission) => emission,
                Err(error) => {
                    first_error.get_or_insert(CircuitError::Neuron(error));
                    return;
                }
            };
            let Some(emission) = emission else {
                return;
            };
            activations = activations.saturating_add(1);
            for synapse in synapses
                .iter()
                .filter(|item| item.source() == emission.source())
            {
                match synapse.transmit(emission) {
                    Ok(Some(next)) => match sink.enqueue_identified(next) {
                        Ok(sequence) => {
                            if let Some(address) = dendritic_routes.get(&synapse.id()) {
                                pending_dendritic.insert(sequence, *address);
                            }
                            transmissions = transmissions.saturating_add(1);
                        }
                        Err(error) => {
                            rejected_transmissions = rejected_transmissions.saturating_add(1);
                            first_error.get_or_insert(CircuitError::Engine(error));
                        }
                    },
                    Ok(None) => {}
                    Err(error) => {
                        rejected_transmissions = rejected_transmissions.saturating_add(1);
                        first_error.get_or_insert(CircuitError::Synapse(error));
                    }
                }
            }
        });

        CircuitReport {
            cycle,
            activations,
            transmissions,
            rejected_transmissions,
            first_error,
        }
    }

    pub fn neuron_state(&self, id: NodeId) -> Option<NeuronState> {
        self.neurons.get(&id).map(Neuron::state)
    }

    pub fn queue_len(&self) -> usize {
        self.engine.queue_len()
    }

    pub fn engine_metrics(&self) -> EngineMetrics {
        self.engine.metrics()
    }
}

/// Snapshot observável completo do circuito e do Engine. Não é formato de arquivo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CircuitSnapshot {
    pub neurons: Vec<crate::neuron::NeuronSnapshot>,
    pub synapses: Vec<(SynapseId, NodeId, NodeId, i32, u64)>,
    pub engine: Vec<u8>,
}
impl Circuit {
    pub fn snapshot(&self) -> Result<CircuitSnapshot, crate::persistence::PersistenceError> {
        if !self.dendrites.is_empty() {
            return Err(crate::persistence::PersistenceError::DendriticSnapshotUnsupported);
        }

        let empty = crate::engram::EngramStore::new(1).expect("capacidade positiva");
        Ok(CircuitSnapshot {
            neurons: self.neurons.values().map(Neuron::snapshot).collect(),
            synapses: self
                .synapses
                .iter()
                .map(|s| (s.id(), s.source(), s.destination(), s.weight(), s.delay()))
                .collect(),
            engine: crate::persistence::encode(&self.engine, &empty)?,
        })
    }
}

impl Circuit {
    pub fn logical_time(&self) -> Option<u64> {
        self.engine.logical_time()
    }
}

/// Estado integral do circuito para persistencia v2.
/// A codificacao em arquivo sera implementada separadamente.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CircuitSnapshotV2 {
    pub neurons: Vec<crate::neuron::NeuronSnapshot>,
    pub dendrites: Vec<(DendriteAddress, crate::dendrite::DendriteSnapshot)>,
    pub synapses: Vec<(SynapseId, NodeId, NodeId, i32, u64)>,
    pub dendritic_routes: Vec<(SynapseId, DendriteAddress)>,
    pub pending_dendritic: Vec<(u64, DendriteAddress)>,
    pub engine: Vec<u8>,
}

impl Circuit {
    pub fn snapshot_v2(&self) -> Result<CircuitSnapshotV2, crate::persistence::PersistenceError> {
        use crate::persistence::PersistenceError;
        use std::collections::BTreeMap;

        let pending_events: BTreeMap<_, _> = self
            .engine
            .pending_events_identified()
            .into_iter()
            .collect();

        for (sequence, address) in &self.pending_dendritic {
            let event = pending_events
                .get(sequence)
                .ok_or(PersistenceError::InvalidState)?;

            if event.destination() != address.neuron || !self.dendrites.contains_key(address) {
                return Err(PersistenceError::InvalidState);
            }
        }

        for (synapse, address) in &self.dendritic_routes {
            if !self.dendrites.contains_key(address)
                || !self
                    .synapses
                    .iter()
                    .any(|item| item.id() == *synapse && item.destination() == address.neuron)
            {
                return Err(PersistenceError::InvalidState);
            }
        }

        let empty =
            crate::engram::EngramStore::new(1).map_err(|_| PersistenceError::InvalidState)?;

        Ok(CircuitSnapshotV2 {
            neurons: self.neurons.values().map(Neuron::snapshot).collect(),
            dendrites: self
                .dendrites
                .iter()
                .map(|(address, dendrite)| (*address, dendrite.snapshot()))
                .collect(),
            synapses: self
                .synapses
                .iter()
                .map(|s| (s.id(), s.source(), s.destination(), s.weight(), s.delay()))
                .collect(),
            dendritic_routes: self
                .dendritic_routes
                .iter()
                .map(|(id, address)| (*id, *address))
                .collect(),
            pending_dendritic: self
                .pending_dendritic
                .iter()
                .map(|(sequence, address)| (*sequence, *address))
                .collect(),
            engine: crate::persistence::encode(&self.engine, &empty)?,
        })
    }
}

impl Circuit {
    /// Reconstrói um circuito completo a partir de um snapshot v2.
    /// Nenhum estado parcialmente validado é disponibilizado.
    pub fn from_snapshot_v2(
        snapshot: CircuitSnapshotV2,
    ) -> Result<Self, crate::persistence::PersistenceError> {
        use crate::persistence::PersistenceError;
        use std::collections::{BTreeMap, BTreeSet};

        let (engine, memory) = crate::persistence::decode(&snapshot.engine)?;

        if memory.len() != 0 {
            return Err(PersistenceError::InvalidState);
        }

        let mut neurons = BTreeMap::new();

        for state in snapshot.neurons {
            let id = state.id;
            let neuron =
                Neuron::from_snapshot(state).map_err(|_| PersistenceError::InvalidState)?;

            if neurons.insert(id, neuron).is_some() {
                return Err(PersistenceError::InvalidState);
            }
        }

        let mut dendrites = BTreeMap::new();

        for (address, state) in snapshot.dendrites {
            if !neurons.contains_key(&address.neuron) {
                return Err(PersistenceError::InvalidState);
            }

            let dendrite =
                Dendrite::from_snapshot(state).map_err(|_| PersistenceError::InvalidState)?;

            if dendrites.insert(address, dendrite).is_some() {
                return Err(PersistenceError::InvalidState);
            }
        }

        let mut synapses = Vec::new();
        let mut synapse_ids = BTreeSet::new();

        for (id, source, destination, weight, delay) in snapshot.synapses {
            if !neurons.contains_key(&source)
                || !neurons.contains_key(&destination)
                || !synapse_ids.insert(id)
            {
                return Err(PersistenceError::InvalidState);
            }

            synapses.push(
                Synapse::new(id, source, destination, weight, delay)
                    .map_err(|_| PersistenceError::InvalidState)?,
            );
        }

        let mut dendritic_routes = BTreeMap::new();

        for (id, address) in snapshot.dendritic_routes {
            if !dendrites.contains_key(&address)
                || !synapses
                    .iter()
                    .any(|synapse| synapse.id() == id && synapse.destination() == address.neuron)
                || dendritic_routes.insert(id, address).is_some()
            {
                return Err(PersistenceError::InvalidState);
            }
        }

        let queued: BTreeMap<_, _> = engine.pending_events_identified().into_iter().collect();

        let mut pending_dendritic = BTreeMap::new();

        for (sequence, address) in snapshot.pending_dendritic {
            let event = queued
                .get(&sequence)
                .ok_or(PersistenceError::InvalidState)?;

            if event.destination() != address.neuron
                || !dendrites.contains_key(&address)
                || pending_dendritic.insert(sequence, address).is_some()
            {
                return Err(PersistenceError::InvalidState);
            }
        }

        // A topologia restaurada deve corresponder exatamente
        // aos destinos registrados no motor.
        let registered: BTreeSet<_> = engine.registered_destinations().into_iter().collect();

        let restored: BTreeSet<_> = neurons.keys().copied().collect();

        if registered != restored {
            return Err(PersistenceError::InvalidState);
        }

        Ok(Self {
            engine,
            neurons,
            synapses,
            dendrites,
            dendritic_routes,
            pending_dendritic,
        })
    }
}
