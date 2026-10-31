#[allow(dead_code)]
#[path = "support/v03_f1_p01/canonical.rs"]
mod canonical;
use neurons::circuit::CircuitError;
use neurons::cognition::{CognitionError, Outcome, Proof};
use neurons::engine::EngineError;
use neurons::engram::EngramId;
use neurons::event::NodeId;
use neurons::metacognition::{
    AbstainReason, Assessment, Conflict, EvidenceRefs, ReviewError, RevisionReason,
};
use neurons::neuron::NeuronError;
use neurons::synapse::{SynapseError, SynapseId};
fn n(x: u32) -> NodeId {
    NodeId::new(x).unwrap()
}
fn id(x: u32) -> EngramId {
    EngramId::new(x).unwrap()
}
fn samples() -> Vec<canonical::Fields> {
    let mut v = vec![];
    for x in [
        AbstainReason::MissingEvidence,
        AbstainReason::ContradictoryEvidence,
        AbstainReason::ResourceLimited,
        AbstainReason::PropagationFailure,
        AbstainReason::InvalidEvidence,
        AbstainReason::ReviewBudget,
        AbstainReason::MultipleSupportedContexts,
    ] {
        v.push(canonical::inspect_abstain(x));
    }
    for x in [
        Assessment::Sufficient,
        Assessment::MissingEvidence,
        Assessment::ContradictoryEvidence,
        Assessment::ResourceLimited,
        Assessment::PropagationFailure,
        Assessment::InvalidEvidence,
    ] {
        v.push(canonical::inspect_assessment(x));
    }
    for x in [
        RevisionReason::MissingEvidence,
        RevisionReason::ContradictoryEvidence,
    ] {
        v.push(canonical::inspect_revision(x));
    }
    for x in [
        EvidenceRefs::Direct(id(21)),
        EvidenceRefs::TwoHop {
            first: id(21),
            second: id(22),
        },
    ] {
        v.push(canonical::inspect_evidence(x));
    }
    for x in [
        Conflict::NegativeConclusion { negative: id(22) },
        Conflict::NegativePremise {
            positive: id(21),
            negative: id(22),
        },
    ] {
        v.push(canonical::inspect_conflict(x));
    }
    let mut errors = vec![
        CognitionError::InvalidContext,
        CognitionError::ReflexiveQuery,
        CognitionError::ZeroFactLimit,
        CognitionError::ZeroQueueCapacity,
        CognitionError::ZeroEventBudget,
        CognitionError::FactLimitExceeded,
        CognitionError::DuplicateRelation {
            from: n(18),
            to: n(19),
        },
        CognitionError::InvalidRelationTarget(id(21)),
        CognitionError::InvalidSynapseId,
    ];
    for x in [
        NeuronError::InvalidThreshold,
        NeuronError::InvalidStateLimit,
        NeuronError::WrongDestination {
            expected: n(18),
            actual: n(19),
        },
        NeuronError::UnsupportedEventKind,
    ] {
        v.push(canonical::inspect_neuron(x));
        errors.push(CognitionError::Neuron(x));
        errors.push(CognitionError::Circuit(CircuitError::Neuron(x)));
    }
    for x in [
        SynapseError::ZeroDelay,
        SynapseError::WrongSource {
            expected: n(18),
            actual: n(19),
        },
        SynapseError::TimeOverflow,
    ] {
        v.push(canonical::inspect_synapse(x));
        errors.push(CognitionError::Synapse(x));
        errors.push(CognitionError::Circuit(CircuitError::Synapse(x)));
    }
    for x in [
        EngineError::ZeroQueueCapacity,
        EngineError::ZeroCycleLimit,
        EngineError::UnknownDestination(n(18)),
        EngineError::PastLogicalTime {
            attempted: 1,
            current: 2,
        },
        EngineError::QueueFull,
        EngineError::SequenceExhausted,
    ] {
        v.push(canonical::inspect_engine(x));
        errors.push(CognitionError::Circuit(CircuitError::Engine(x)));
    }
    for x in [
        CircuitError::DuplicateNeuron(n(18)),
        CircuitError::UnknownNeuron(n(18)),
        CircuitError::DuplicateSynapse(SynapseId::new(1).unwrap()),
        CircuitError::UnknownSynapse(SynapseId::new(1).unwrap()),
    ] {
        v.push(canonical::inspect_circuit(x));
        errors.push(CognitionError::Circuit(x));
    }
    for x in errors {
        v.push(canonical::inspect_cognition(x));
    }
    for outcome in [
        Outcome::DirectRecall(id(21)),
        Outcome::Inferred(Proof {
            via: n(20),
            first_fact: id(21),
            second_fact: id(22),
            via_activations: 1,
        }),
        Outcome::Unsupported,
        Outcome::InconclusiveBudget,
        Outcome::InconclusivePropagation(CircuitError::UnknownNeuron(n(20))),
    ] {
        v.push(canonical::inspect_outcome(outcome));
    }
    for err in [
        ReviewError::InvalidAttemptBudget,
        ReviewError::TooManyCandidates,
        ReviewError::InvalidCandidateContext,
        ReviewError::DuplicateCandidateContext,
        ReviewError::InvalidHypothesis(CognitionError::InvalidContext),
    ] {
        v.push(canonical::fields(&Err(err)));
    }
    v
}
#[test]
fn synthetic_child() {
    if std::env::var_os("NEURONS_V03_SCHEMA_CHILD").is_none() {
        return;
    }
    let v = samples();
    println!("SYNTH_CHILD:{}", canonical::wire(&v[0]));
}
#[test]
fn synthetic_coverage() {
    let v = samples();
    for fields in &v {
        let wire = canonical::wire(fields);
        assert_eq!(canonical::parse_wire(&wire).unwrap(), *fields);
        assert_eq!(canonical::wire(fields), wire);
        for i in 0..fields.len() {
            let mut missing = fields.clone();
            missing.remove(i);
            assert!(!canonical::compare(fields, &missing).is_empty());
            let mut changed = fields.clone();
            changed[i].1.push('_');
            assert!(!canonical::compare(fields, &changed).is_empty());
        }
        let mut extra = fields.clone();
        extra.push(("unexpected".into(), "x".into()));
        assert!(!canonical::compare(fields, &extra).is_empty());
    }
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "synthetic_child", "--nocapture"])
        .env("NEURONS_V03_SCHEMA_CHILD", "1")
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    let lines: Vec<_> = stdout
        .lines()
        .filter_map(|line| line.strip_prefix("SYNTH_CHILD:"))
        .collect();
    assert_eq!(lines.len(), 1);
    assert!(canonical::compare(&v[0], &canonical::parse_wire(lines[0]).unwrap()).is_empty());
    println!("synthetic_values={} cross_process=1", v.len());
}
