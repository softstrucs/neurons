//! Experimento delimitado: composição de duas relações por propagação de eventos.

use std::collections::BTreeSet;

use crate::circuit::{Circuit, CircuitError};
use crate::engram::{EngramId, EngramStore, RecordStatus};
use crate::event::NodeId;
use crate::neuron::{Neuron, NeuronError};
use crate::synapse::{Synapse, SynapseError, SynapseIdError};

/// Fato direto codificado em um engrama: pista `(contexto, origem, destino)`
/// e valor `1`. Apenas registros `Observed` ou `Validated` entram no ensaio.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RelationFact {
    id: EngramId,
    from: NodeId,
    to: NodeId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CognitionError {
    InvalidContext,
    ReflexiveQuery,
    ZeroFactLimit,
    ZeroQueueCapacity,
    ZeroEventBudget,
    FactLimitExceeded,
    DuplicateRelation { from: NodeId, to: NodeId },
    InvalidRelationTarget(EngramId),
    InvalidSynapseId,
    Neuron(NeuronError),
    Synapse(SynapseError),
    Circuit(CircuitError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Hypothesis {
    pub context: u32,
    pub from: NodeId,
    pub to: NodeId,
}

impl Hypothesis {
    pub fn new(context: u32, from: NodeId, to: NodeId) -> Result<Self, CognitionError> {
        if context == 0 {
            return Err(CognitionError::InvalidContext);
        }
        if from == to {
            return Err(CognitionError::ReflexiveQuery);
        }
        Ok(Self { context, from, to })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Proof {
    pub via: NodeId,
    pub first_fact: EngramId,
    pub second_fact: EngramId,
    pub via_activations: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    /// Consulta direta; não representa composição de relações.
    DirectRecall(EngramId),
    /// Dois fatos diretos e ativações observadas sustentam a composição.
    Inferred(Proof),
    /// Nenhuma composição de dois passos foi sustentada neste ensaio.
    Unsupported,
    /// Eventos restantes impedem uma conclusão dentro do orçamento.
    InconclusiveBudget,
    /// Uma falha de propagação impede uma conclusão confiável.
    InconclusivePropagation(CircuitError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InferenceReport {
    pub hypothesis: Hypothesis,
    pub outcome: Outcome,
    pub facts_loaded: usize,
    pub processed_events: usize,
    pub source_activations: u64,
    pub target_activations: u64,
}

/// Configuração limitada do ensaio. Nenhum peso é aprendido aqui; fatos
/// fornecidos são traduzidos em conexões de peso e atraso unitários.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TwoHopTask {
    max_facts: usize,
    queue_capacity: usize,
    event_budget: usize,
}

impl TwoHopTask {
    pub fn new(
        max_facts: usize,
        queue_capacity: usize,
        event_budget: usize,
    ) -> Result<Self, CognitionError> {
        if max_facts == 0 {
            return Err(CognitionError::ZeroFactLimit);
        }
        if queue_capacity == 0 {
            return Err(CognitionError::ZeroQueueCapacity);
        }
        if event_budget == 0 {
            return Err(CognitionError::ZeroEventBudget);
        }
        Ok(Self {
            max_facts,
            queue_capacity,
            event_budget,
        })
    }

    pub fn evaluate(
        self,
        store: &EngramStore,
        hypothesis: Hypothesis,
    ) -> Result<InferenceReport, CognitionError> {
        self.run(store, hypothesis, true)
    }

    /// Controle experimental: mesmos fatos e circuito, pesos de conexão zero.
    pub fn evaluate_without_synapses(
        self,
        store: &EngramStore,
        hypothesis: Hypothesis,
    ) -> Result<InferenceReport, CognitionError> {
        self.run(store, hypothesis, false)
    }

    fn run(
        self,
        store: &EngramStore,
        hypothesis: Hypothesis,
        propagate: bool,
    ) -> Result<InferenceReport, CognitionError> {
        let mut facts = Vec::new();
        for record in store.iter() {
            let cue = record.cue();
            if cue.context() != hypothesis.context
                || cue.pattern() <= 0
                || record.value() != 1
                || record.status() == RecordStatus::Hypothesis
            {
                continue;
            }
            let to = NodeId::new(cue.pattern() as u32)
                .map_err(|_| CognitionError::InvalidRelationTarget(record.id()))?;
            if facts
                .iter()
                .any(|fact: &RelationFact| fact.from == cue.source() && fact.to == to)
            {
                return Err(CognitionError::DuplicateRelation {
                    from: cue.source(),
                    to,
                });
            }
            if facts.len() >= self.max_facts {
                return Err(CognitionError::FactLimitExceeded);
            }
            facts.push(RelationFact {
                id: record.id(),
                from: cue.source(),
                to,
            });
        }
        facts.sort_by_key(|fact| (fact.from, fact.to, fact.id));

        let mut report = InferenceReport {
            hypothesis,
            outcome: Outcome::Unsupported,
            facts_loaded: facts.len(),
            processed_events: 0,
            source_activations: 0,
            target_activations: 0,
        };
        if let Some(direct) = facts
            .iter()
            .find(|fact| fact.from == hypothesis.from && fact.to == hypothesis.to)
        {
            report.outcome = Outcome::DirectRecall(direct.id);
            return Ok(report);
        }

        let mut nodes = BTreeSet::new();
        for fact in &facts {
            nodes.insert(fact.from);
            nodes.insert(fact.to);
        }
        if !nodes.contains(&hypothesis.from) || !nodes.contains(&hypothesis.to) {
            return Ok(report);
        }

        let mut circuit = Circuit::new(self.queue_capacity, self.event_budget)
            .map_err(CognitionError::Circuit)?;
        for node in nodes {
            circuit
                .add_neuron(Neuron::new(node, 1, 1).map_err(CognitionError::Neuron)?)
                .map_err(CognitionError::Circuit)?;
        }
        for fact in &facts {
            let id = crate::synapse::SynapseId::new(fact.id.get())
                .map_err(|_: SynapseIdError| CognitionError::InvalidSynapseId)?;
            let weight = i32::from(propagate);
            let synapse =
                Synapse::new(id, fact.from, fact.to, weight, 1).map_err(CognitionError::Synapse)?;
            circuit.connect(synapse).map_err(CognitionError::Circuit)?;
        }
        circuit
            .stimulate(hypothesis.from, 1, 0)
            .map_err(CognitionError::Circuit)?;
        let cycle = circuit.run_cycle();
        report.processed_events = cycle.cycle.processed;
        report.source_activations = circuit
            .neuron_state(hypothesis.from)
            .map(|state| state.activations)
            .unwrap_or(0);
        report.target_activations = circuit
            .neuron_state(hypothesis.to)
            .map(|state| state.activations)
            .unwrap_or(0);
        if let Some(error) = cycle.first_error {
            report.outcome = Outcome::InconclusivePropagation(error);
            return Ok(report);
        }
        if cycle.cycle.limit_reached {
            report.outcome = Outcome::InconclusiveBudget;
            return Ok(report);
        }
        if report.source_activations == 0 || report.target_activations == 0 {
            return Ok(report);
        }

        for first in facts.iter().filter(|fact| fact.from == hypothesis.from) {
            if first.to == hypothesis.from || first.to == hypothesis.to {
                continue;
            }
            let via_activations = circuit
                .neuron_state(first.to)
                .map(|state| state.activations)
                .unwrap_or(0);
            if via_activations == 0 {
                continue;
            }
            if let Some(second) = facts
                .iter()
                .find(|fact| fact.from == first.to && fact.to == hypothesis.to)
            {
                report.outcome = Outcome::Inferred(Proof {
                    via: first.to,
                    first_fact: first.id,
                    second_fact: second.id,
                    via_activations,
                });
                break;
            }
        }
        Ok(report)
    }
}
