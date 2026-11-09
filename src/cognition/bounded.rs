//! Extensão finita do ensaio existente; não altera TwoHopTask ou pesos aprendidos.
use super::{propagate_facts, CognitionError, Hypothesis, RelationFact};
use crate::engram::{EngramId, EngramStore, RecordStatus};
use crate::event::NodeId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathInference {
    pub alternatives: Vec<Vec<EngramId>>,
    pub processed_events: usize,
    pub activations: Vec<(NodeId, u64)>,
    pub resource_limited: bool,
    pub activation_supported: bool,
    pub propagation_error: Option<CognitionError>,
}
/// IDs desempatarão apenas a ordem de provas semanticamente verificadas.
/// Com oito arestas, enumera no máximo 8+64+512 extensões, profundidade três.
pub(crate) fn evaluate(
    store: &EngramStore,
    query: Hypothesis,
    budget: usize,
) -> Result<PathInference, CognitionError> {
    Hypothesis::new(query.context, query.from, query.to)?;
    if store.len() > 8 {
        return Err(CognitionError::FactLimitExceeded);
    }
    let mut result = PathInference {
        alternatives: vec![],
        processed_events: 0,
        activations: vec![],
        resource_limited: false,
        activation_supported: true,
        propagation_error: None,
    };
    if budget == 0 || budget > 64 {
        result.resource_limited = true;
        return Ok(result);
    }
    let mut facts = vec![];
    for r in store.iter() {
        if r.cue().context() == query.context
            && r.value() == 1
            && r.status() == RecordStatus::Observed
            && r.cue().pattern() > 0
        {
            facts.push(RelationFact {
                id: r.id(),
                from: r.cue().source(),
                to: NodeId::new(r.cue().pattern() as u32)
                    .map_err(|_| CognitionError::InvalidRelationTarget(r.id()))?,
            });
        }
    }
    fn paths(
        facts: &[RelationFact],
        at: NodeId,
        target: NodeId,
        visited: &mut Vec<NodeId>,
        prefix: &mut Vec<EngramId>,
        out: &mut Vec<Vec<EngramId>>,
    ) {
        if prefix.len() == 3 {
            return;
        }
        for f in facts.iter().filter(|f| f.from == at) {
            if visited.contains(&f.to) {
                continue;
            }
            prefix.push(f.id);
            if f.to == target {
                out.push(prefix.clone())
            } else {
                visited.push(f.to);
                paths(facts, f.to, target, visited, prefix, out);
                visited.pop();
            }
            prefix.pop();
        }
    }
    paths(
        &facts,
        query.from,
        query.to,
        &mut vec![query.from],
        &mut vec![],
        &mut result.alternatives,
    );
    result
        .alternatives
        .sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    // Recuperação literal não precisa de circuito, como no ensaio legado.
    if result.alternatives.first().is_some_and(|p| p.len() == 1) {
        return Ok(result);
    }
    if !facts.iter().any(|f| f.from == query.from) || !facts.iter().any(|f| f.to == query.to) {
        return Ok(result);
    }
    match propagate_facts(&facts, query, 16, budget, true) {
        Ok((circuit, r)) => {
            let mut nodes = std::collections::BTreeSet::new();
            for f in &facts {
                nodes.insert(f.from);
                nodes.insert(f.to);
            }
            result.activations = nodes
                .into_iter()
                .map(|n| (n, circuit.neuron_state(n).map_or(0, |s| s.activations)))
                .collect();
            result.activation_supported = result.alternatives.iter().all(|p| {
                p.iter().all(|id| {
                    facts.iter().find(|f| f.id == *id).is_some_and(|f| {
                        circuit
                            .neuron_state(f.from)
                            .is_some_and(|s| s.activations > 0)
                            && circuit
                                .neuron_state(f.to)
                                .is_some_and(|s| s.activations > 0)
                    })
                })
            });
            result.processed_events = r.cycle.processed;
            result.resource_limited = r.cycle.limit_reached;
            result.propagation_error = r.first_error.map(CognitionError::Circuit);
        }
        Err(e) => result.propagation_error = Some(e),
    }
    Ok(result)
}
