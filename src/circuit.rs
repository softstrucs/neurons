//! Composição experimental pequena de neurônios, sinapses e motor de eventos.

use std::collections::BTreeMap;

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
}

impl Circuit {
    pub fn new(queue_capacity: usize, max_events_per_cycle: usize) -> Result<Self, CircuitError> {
        Ok(Self {
            engine: Engine::new(queue_capacity, max_events_per_cycle)
                .map_err(CircuitError::Engine)?,
            neurons: BTreeMap::new(),
            synapses: Vec::new(),
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
        let mut activations: u64 = 0;
        let mut transmissions: u64 = 0;
        let mut rejected_transmissions: u64 = 0;
        let mut first_error = None;

        let cycle = self.engine.run_cycle(|event, sink| {
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
                    Ok(Some(next)) => match sink.enqueue(next) {
                        Ok(()) => transmissions = transmissions.saturating_add(1),
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
