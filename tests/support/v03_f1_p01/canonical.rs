use neurons::circuit::CircuitError;
use neurons::cognition::{CognitionError, Hypothesis, InferenceReport, Outcome, Proof};
use neurons::engine::EngineError;
use neurons::metacognition::{
    AbstainReason, Assessment, AttemptLog, Conflict, Decision, EvidenceRefs, ReviewError,
    ReviewReport, RevisionReason,
};
use neurons::neuron::NeuronError;
use neurons::synapse::SynapseError;

pub type Fields = Vec<(String, String)>;
fn put(f: &mut Fields, p: &str, v: impl ToString) {
    f.push((p.into(), v.to_string()));
}
fn path(p: &str, s: &str) -> String {
    format!("{p}.{s}")
}
fn opt<T>(f: &mut Fields, p: &str, value: Option<T>, write: impl FnOnce(&mut Fields, &str, T)) {
    match value {
        None => put(f, p, "none"),
        Some(v) => {
            put(f, p, "some");
            write(f, &path(p, "value"), v);
        }
    }
}
fn hyp(f: &mut Fields, p: &str, h: Hypothesis) {
    let Hypothesis { context, from, to } = h;
    put(f, &path(p, "context"), context);
    put(f, &path(p, "from"), from.get());
    put(f, &path(p, "to"), to.get());
}
fn evidence(f: &mut Fields, p: &str, e: EvidenceRefs) {
    match e {
        EvidenceRefs::Direct(id) => {
            put(f, p, "Direct");
            put(f, &path(p, "id"), id.get());
        }
        EvidenceRefs::TwoHop { first, second } => {
            put(f, p, "TwoHop");
            put(f, &path(p, "first"), first.get());
            put(f, &path(p, "second"), second.get());
        }
    }
}
fn conflict(f: &mut Fields, p: &str, c: Conflict) {
    match c {
        Conflict::NegativeConclusion { negative } => {
            put(f, p, "NegativeConclusion");
            put(f, &path(p, "negative"), negative.get());
        }
        Conflict::NegativePremise { positive, negative } => {
            put(f, p, "NegativePremise");
            put(f, &path(p, "positive"), positive.get());
            put(f, &path(p, "negative"), negative.get());
        }
    }
}
fn neuron(f: &mut Fields, p: &str, e: NeuronError) {
    match e {
        NeuronError::InvalidThreshold => put(f, p, "InvalidThreshold"),
        NeuronError::InvalidStateLimit => put(f, p, "InvalidStateLimit"),
        NeuronError::UnsupportedEventKind => put(f, p, "UnsupportedEventKind"),
        NeuronError::WrongDestination { expected, actual } => {
            put(f, p, "WrongDestination");
            put(f, &path(p, "expected"), expected.get());
            put(f, &path(p, "actual"), actual.get());
        }
    }
}
fn synapse(f: &mut Fields, p: &str, e: SynapseError) {
    match e {
        SynapseError::ZeroDelay => put(f, p, "ZeroDelay"),
        SynapseError::TimeOverflow => put(f, p, "TimeOverflow"),
        SynapseError::WrongSource { expected, actual } => {
            put(f, p, "WrongSource");
            put(f, &path(p, "expected"), expected.get());
            put(f, &path(p, "actual"), actual.get());
        }
    }
}
fn engine(f: &mut Fields, p: &str, e: EngineError) {
    match e {
        EngineError::ZeroQueueCapacity => put(f, p, "ZeroQueueCapacity"),
        EngineError::ZeroCycleLimit => put(f, p, "ZeroCycleLimit"),
        EngineError::QueueFull => put(f, p, "QueueFull"),
        EngineError::SequenceExhausted => put(f, p, "SequenceExhausted"),
        EngineError::UnknownDestination(id) => {
            put(f, p, "UnknownDestination");
            put(f, &path(p, "id"), id.get());
        }
        EngineError::PastLogicalTime { attempted, current } => {
            put(f, p, "PastLogicalTime");
            put(f, &path(p, "attempted"), attempted);
            put(f, &path(p, "current"), current);
        }
    }
}
fn circuit(f: &mut Fields, p: &str, e: CircuitError) {
    match e {
        CircuitError::Engine(x) => {
            put(f, p, "Engine");
            engine(f, &path(p, "value"), x)
        }
        CircuitError::Neuron(x) => {
            put(f, p, "Neuron");
            neuron(f, &path(p, "value"), x)
        }
        CircuitError::Synapse(x) => {
            put(f, p, "Synapse");
            synapse(f, &path(p, "value"), x)
        }
        CircuitError::DuplicateNeuron(x) => {
            put(f, p, "DuplicateNeuron");
            put(f, &path(p, "id"), x.get());
        }
        CircuitError::UnknownNeuron(x) => {
            put(f, p, "UnknownNeuron");
            put(f, &path(p, "id"), x.get());
        }
        CircuitError::DuplicateSynapse(x) => {
            put(f, p, "DuplicateSynapse");
            put(f, &path(p, "id"), x.get());
        }
        CircuitError::UnknownSynapse(x) => {
            put(f, p, "UnknownSynapse");
            put(f, &path(p, "id"), x.get());
        }
    }
}
fn cognition(f: &mut Fields, p: &str, e: CognitionError) {
    match e {
        CognitionError::InvalidContext => put(f, p, "InvalidContext"),
        CognitionError::ReflexiveQuery => put(f, p, "ReflexiveQuery"),
        CognitionError::ZeroFactLimit => put(f, p, "ZeroFactLimit"),
        CognitionError::ZeroQueueCapacity => put(f, p, "ZeroQueueCapacity"),
        CognitionError::ZeroEventBudget => put(f, p, "ZeroEventBudget"),
        CognitionError::FactLimitExceeded => put(f, p, "FactLimitExceeded"),
        CognitionError::InvalidSynapseId => put(f, p, "InvalidSynapseId"),
        CognitionError::DuplicateRelation { from, to } => {
            put(f, p, "DuplicateRelation");
            put(f, &path(p, "from"), from.get());
            put(f, &path(p, "to"), to.get());
        }
        CognitionError::InvalidRelationTarget(x) => {
            put(f, p, "InvalidRelationTarget");
            put(f, &path(p, "id"), x.get());
        }
        CognitionError::Neuron(x) => {
            put(f, p, "Neuron");
            neuron(f, &path(p, "value"), x)
        }
        CognitionError::Synapse(x) => {
            put(f, p, "Synapse");
            synapse(f, &path(p, "value"), x)
        }
        CognitionError::Circuit(x) => {
            put(f, p, "Circuit");
            circuit(f, &path(p, "value"), x)
        }
    }
}
fn inference(f: &mut Fields, p: &str, i: InferenceReport) {
    let InferenceReport {
        hypothesis,
        outcome,
        facts_loaded,
        processed_events,
        source_activations,
        target_activations,
    } = i;
    hyp(f, &path(p, "hypothesis"), hypothesis);
    serialize_outcome(f, &path(p, "outcome"), outcome);
    put(f, &path(p, "facts_loaded"), facts_loaded);
    put(f, &path(p, "processed_events"), processed_events);
    put(f, &path(p, "source_activations"), source_activations);
    put(f, &path(p, "target_activations"), target_activations);
}
fn serialize_outcome(f: &mut Fields, q: &str, outcome: Outcome) {
    match outcome {
        Outcome::DirectRecall(x) => {
            put(f, q, "DirectRecall");
            put(f, &path(q, "id"), x.get());
        }
        Outcome::Inferred(Proof {
            via,
            first_fact,
            second_fact,
            via_activations,
        }) => {
            put(f, q, "Inferred");
            put(f, &path(q, "via"), via.get());
            put(f, &path(q, "first_fact"), first_fact.get());
            put(f, &path(q, "second_fact"), second_fact.get());
            put(f, &path(q, "via_activations"), via_activations);
        }
        Outcome::Unsupported => put(f, q, "Unsupported"),
        Outcome::InconclusiveBudget => put(f, q, "InconclusiveBudget"),
        Outcome::InconclusivePropagation(x) => {
            put(f, q, "InconclusivePropagation");
            circuit(f, &path(q, "error"), x);
        }
    }
}
fn revision(x: RevisionReason) -> &'static str {
    match x {
        RevisionReason::MissingEvidence => "MissingEvidence",
        RevisionReason::ContradictoryEvidence => "ContradictoryEvidence",
    }
}
fn abstain(x: AbstainReason) -> &'static str {
    match x {
        AbstainReason::MissingEvidence => "MissingEvidence",
        AbstainReason::ContradictoryEvidence => "ContradictoryEvidence",
        AbstainReason::ResourceLimited => "ResourceLimited",
        AbstainReason::PropagationFailure => "PropagationFailure",
        AbstainReason::InvalidEvidence => "InvalidEvidence",
        AbstainReason::ReviewBudget => "ReviewBudget",
        AbstainReason::MultipleSupportedContexts => "MultipleSupportedContexts",
    }
}
fn assessment(x: Assessment) -> &'static str {
    match x {
        Assessment::Sufficient => "Sufficient",
        Assessment::MissingEvidence => "MissingEvidence",
        Assessment::ContradictoryEvidence => "ContradictoryEvidence",
        Assessment::ResourceLimited => "ResourceLimited",
        Assessment::PropagationFailure => "PropagationFailure",
        Assessment::InvalidEvidence => "InvalidEvidence",
    }
}
pub fn fields(result: &Result<ReviewReport, ReviewError>) -> Fields {
    let mut f = vec![];
    put(&mut f, "schema", "v03-f1-p01/v1");
    match result {
        Err(e) => {
            put(&mut f, "result", "Err");
            match *e {
                ReviewError::InvalidAttemptBudget => put(&mut f, "error", "InvalidAttemptBudget"),
                ReviewError::TooManyCandidates => put(&mut f, "error", "TooManyCandidates"),
                ReviewError::InvalidCandidateContext => {
                    put(&mut f, "error", "InvalidCandidateContext")
                }
                ReviewError::DuplicateCandidateContext => {
                    put(&mut f, "error", "DuplicateCandidateContext")
                }
                ReviewError::InvalidHypothesis(x) => {
                    put(&mut f, "error", "InvalidHypothesis");
                    cognition(&mut f, "error.value", x);
                }
            }
        }
        Ok(report) => {
            put(&mut f, "result", "Ok");
            let ReviewReport {
                original,
                decision,
                attempts,
            } = report;
            hyp(&mut f, "original", *original);
            match *decision {
                Decision::Respond {
                    hypothesis,
                    evidence: ev,
                    revision: r,
                } => {
                    put(&mut f, "decision", "Respond");
                    hyp(&mut f, "decision.hypothesis", hypothesis);
                    evidence(&mut f, "decision.evidence", ev);
                    opt(&mut f, "decision.revision", r, |f, p, x| {
                        put(f, p, revision(x))
                    });
                }
                Decision::Abstain(x) => {
                    put(&mut f, "decision", "Abstain");
                    put(&mut f, "decision.reason", abstain(x));
                }
            }
            put(&mut f, "attempts.len", attempts.len());
            for (idx, a) in attempts.iter().enumerate() {
                let AttemptLog {
                    hypothesis,
                    inference: inf,
                    evaluation_error,
                    assessment: ass,
                    evidence: ev,
                    conflict: conf,
                } = *a;
                let p = format!("attempts.{idx}");
                hyp(&mut f, &path(&p, "hypothesis"), hypothesis);
                opt(&mut f, &path(&p, "inference"), inf, inference);
                opt(
                    &mut f,
                    &path(&p, "evaluation_error"),
                    evaluation_error,
                    cognition,
                );
                put(&mut f, &path(&p, "assessment"), assessment(ass));
                opt(&mut f, &path(&p, "evidence"), ev, evidence);
                opt(&mut f, &path(&p, "conflict"), conf, conflict);
            }
        }
    }
    f
}
pub fn wire(f: &Fields) -> String {
    let mut out = String::new();
    for (k, v) in f {
        out.push_str(&format!("{}:{}{}:{}", k.len(), k, v.len(), v));
    }
    out
}
pub fn compare(a: &Fields, b: &Fields) -> Vec<String> {
    let mut out = vec![];
    for i in 0..a.len().max(b.len()) {
        match (a.get(i), b.get(i)) {
            (Some(x), Some(y)) if x == y => {}
            (Some(x), Some(y)) => out.push(format!("{} != {}", x.0, y.0)),
            (Some(x), None) => out.push(format!("{} missing right", x.0)),
            (None, Some(y)) => out.push(format!("{} missing left", y.0)),
            (None, None) => {}
        }
    }
    out
}

/// Decodifica o formato por comprimento; rejeita truncamento, UTF-8 inválido,
/// caminhos repetidos e versão desconhecida. O verificador rejeita variantes
/// e campos não previstos para o manifesto avaliado.
pub fn parse_wire(wire: &str) -> Result<Fields, &'static str> {
    fn one<'a>(rest: &mut &'a str) -> Result<&'a str, &'static str> {
        let (n, tail) = rest.split_once(':').ok_or("length_missing")?;
        if n.is_empty() || !n.bytes().all(|x| x.is_ascii_digit()) {
            return Err("invalid_length");
        }
        let size: usize = n.parse().map_err(|_| "length_overflow")?;
        if !tail.is_char_boundary(size) {
            return Err("truncated_or_utf8");
        }
        let (value, following) = tail.split_at(size);
        *rest = following;
        Ok(value)
    }
    let mut rest = wire;
    let mut out = vec![];
    while !rest.is_empty() {
        let key = one(&mut rest)?.to_string();
        let value = one(&mut rest)?.to_string();
        if out.iter().any(|(k, _)| k == &key) {
            return Err("duplicate_path");
        }
        out.push((key, value));
    }
    if out.first().map(|(k, v)| (k.as_str(), v.as_str())) != Some(("schema", "v03-f1-p01/v1")) {
        return Err("unknown_schema");
    }
    Ok(out)
}

// Pontos de prova do codec: serializam valores públicos isolados, sem fabricar
// ReviewReport ou alegar que determinada variante seja alcançável em review.
#[allow(dead_code)]
pub fn inspect_neuron(x: NeuronError) -> Fields {
    let mut f = vec![("schema".into(), "v03-f1-p01/v1".into())];
    neuron(&mut f, "value", x);
    f
}
#[allow(dead_code)]
pub fn inspect_synapse(x: SynapseError) -> Fields {
    let mut f = vec![("schema".into(), "v03-f1-p01/v1".into())];
    synapse(&mut f, "value", x);
    f
}
#[allow(dead_code)]
pub fn inspect_engine(x: EngineError) -> Fields {
    let mut f = vec![("schema".into(), "v03-f1-p01/v1".into())];
    engine(&mut f, "value", x);
    f
}
#[allow(dead_code)]
pub fn inspect_circuit(x: CircuitError) -> Fields {
    let mut f = vec![("schema".into(), "v03-f1-p01/v1".into())];
    circuit(&mut f, "value", x);
    f
}
#[allow(dead_code)]
pub fn inspect_cognition(x: CognitionError) -> Fields {
    let mut f = vec![("schema".into(), "v03-f1-p01/v1".into())];
    cognition(&mut f, "value", x);
    f
}
#[allow(dead_code)]
pub fn inspect_evidence(x: EvidenceRefs) -> Fields {
    let mut f = vec![("schema".into(), "v03-f1-p01/v1".into())];
    evidence(&mut f, "value", x);
    f
}
#[allow(dead_code)]
pub fn inspect_conflict(x: Conflict) -> Fields {
    let mut f = vec![("schema".into(), "v03-f1-p01/v1".into())];
    conflict(&mut f, "value", x);
    f
}
#[allow(dead_code)]
pub fn inspect_assessment(x: Assessment) -> Fields {
    vec![
        ("schema".into(), "v03-f1-p01/v1".into()),
        ("value".into(), assessment(x).into()),
    ]
}
#[allow(dead_code)]
pub fn inspect_abstain(x: AbstainReason) -> Fields {
    vec![
        ("schema".into(), "v03-f1-p01/v1".into()),
        ("value".into(), abstain(x).into()),
    ]
}
#[allow(dead_code)]
pub fn inspect_revision(x: RevisionReason) -> Fields {
    vec![
        ("schema".into(), "v03-f1-p01/v1".into()),
        ("value".into(), revision(x).into()),
    ]
}

#[allow(dead_code)]
pub fn inspect_outcome(x: Outcome) -> Fields {
    let mut f = vec![("schema".into(), "v03-f1-p01/v1".into())];
    serialize_outcome(&mut f, "value", x);
    f
}
