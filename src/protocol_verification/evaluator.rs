//! Universo, expectativas e custódia exclusivos do avaliador. Sem acesso mutável ao candidato.
use super::{
    controls, fixtures::*, history, Class, ControlReport, Mechanism, PublicInput, Unavailable,
    Verdict, VerificationIssue,
};
use crate::cognitive_contracts::{EpisodeDecision, Polarity, Refusal, Statement};
use crate::engram::{EngramId, EngramStore};
use crate::integration::{
    acquisition::{CognitiveEpisode, Phase},
    query::{QueryReport, RuleAuthorization},
};
use crate::metacognition::{AbstainReason, EvidenceRefs};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecisionPhase {
    Initial,
    AfterFeedback,
    Reserved,
}
impl DecisionPhase {
    pub const fn index(self) -> usize {
        match self {
            Self::Initial => 0,
            Self::AfterFeedback => 1,
            Self::Reserved => 2,
        }
    }
    pub const fn at(self) -> u64 {
        match self {
            Self::Initial => 20,
            Self::AfterFeedback => 40,
            Self::Reserved => 50,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evaluation {
    pub verdict: Verdict,
    pub expected: Class,
    pub actual: Option<Class>,
    pub reference_truth: Option<bool>,
}
/// Não avaliável e falha nunca incrementam `passed`.
#[derive(Default, Debug, Eq, PartialEq)]
pub struct EngineeringTally {
    pub passed: usize,
    pub failed: usize,
    pub not_evaluable: usize,
}
impl EngineeringTally {
    pub fn record(&mut self, e: &Evaluation) {
        match e.verdict {
            Verdict::Pass => self.passed += 1,
            Verdict::Fail(_) => self.failed += 1,
            Verdict::NotEvaluable(_) => self.not_evaluable += 1,
        }
    }
}
pub fn in_recut(condition: Condition, mechanism: Mechanism) -> bool {
    match mechanism {
        Mechanism::NoPropagation => matches!(condition, Condition::P | Condition::C0),
        Mechanism::Unmonitored => matches!(condition, Condition::P | Condition::C2),
        _ => true,
    }
}
fn expected(condition: Condition, phase: DecisionPhase, mechanism: Mechanism) -> Class {
    if mechanism == Mechanism::NoPropagation {
        return Class::Unsupported;
    }
    if mechanism == Mechanism::Unmonitored {
        return if phase == DecisionPhase::Initial {
            Class::Unsupported
        } else {
            Class::Inferred
        };
    }
    if condition == Condition::C3 {
        return Class::OutsideRule;
    }
    if mechanism == Mechanism::Literal {
        return Class::NoAssociation;
    }
    if phase == DecisionPhase::Initial {
        return Class::MissingEvidence;
    }
    match condition {
        Condition::P => Class::Provisional,
        Condition::C2 => Class::Conflict,
        _ => Class::MissingEvidence,
    }
}
fn verdict(result: Result<(), VerificationIssue>, truth: Option<bool>, actual: Class) -> Verdict {
    if truth.is_none() {
        return Verdict::NotEvaluable(Unavailable::ReferenceUnavailable);
    }
    if result == Err(VerificationIssue::UnexpectedClass) {
        let unavailable = match actual {
            Class::ResourceLimited => Some(Unavailable::BudgetExhausted),
            Class::PropagationFailure => Some(Unavailable::PropagationFailure),
            Class::InvalidEvidence => Some(Unavailable::InvalidEvidence),
            _ => None,
        };
        if let Some(reason) = unavailable {
            return Verdict::NotEvaluable(reason);
        }
    }
    match result {
        Ok(()) => Verdict::Pass,
        Err(issue) => Verdict::Fail(issue),
    }
}
pub fn classify(report: &QueryReport) -> Class {
    match report.episode.decision {
        EpisodeDecision::Provisional { .. } => Class::Provisional,
        EpisodeDecision::Abstain(Refusal::RuleOutsideContext) => Class::OutsideRule,
        EpisodeDecision::Abstain(Refusal::Monitor(reason)) => match reason {
            AbstainReason::MissingEvidence => Class::MissingEvidence,
            AbstainReason::ContradictoryEvidence => Class::Conflict,
            AbstainReason::ResourceLimited | AbstainReason::ReviewBudget => Class::ResourceLimited,
            AbstainReason::PropagationFailure => Class::PropagationFailure,
            _ => Class::InvalidEvidence,
        },
        _ => Class::InvalidEvidence,
    }
}
/// Fiscal de relatório contra memória pública fixada; utilizável com cópias adulteradas.
pub fn check_candidate_report(
    report: &QueryReport,
    input: &PublicInput<'_>,
) -> Result<(), VerificationIssue> {
    if report.episode.query != input.query
        || report.episode.at != input.at
        || report.episode.before != report.episode.after
        || report.episode.before.ids() != input.memory.iter().map(|r| r.id()).collect::<Vec<_>>()
    {
        return Err(VerificationIssue::InvalidProof);
    }
    history::no_conclusions(&input.memory.iter().collect::<Vec<_>>())?;
    if input.authorization.scope() != input.query.scope() {
        if classify(report) != Class::OutsideRule
            || report.review.is_some()
            || !report.consulted.is_empty()
            || report.proof.is_some()
        {
            return Err(VerificationIssue::InvalidScope);
        }
        return Ok(());
    }
    if report.consulted != controls::public_records(input)? {
        return Err(VerificationIssue::Metadata);
    }
    let review = report
        .review
        .as_ref()
        .ok_or(VerificationIssue::InvalidProof)?;
    if review.attempts.len() != 1
        || review.original.context != input.query.scope().context.get()
        || review.original.from != input.query.subject()
        || review.original.to != input.query.object()
    {
        return Err(VerificationIssue::InvalidProof);
    }
    match (review.decision, report.episode.decision) {
        (
            crate::metacognition::Decision::Respond {
                evidence,
                hypothesis,
                revision: None,
            },
            EpisodeDecision::Provisional { premises, rule },
        ) if evidence == premises
            && hypothesis == review.original
            && rule == input.query.scope().rule =>
        {
            if report.proof != Some(controls::check_proof(input, premises, false)?) {
                return Err(VerificationIssue::InvalidProof);
            }
        }
        (
            crate::metacognition::Decision::Abstain(a),
            EpisodeDecision::Abstain(Refusal::Monitor(b)),
        ) if a == b && report.proof.is_none() => {}
        _ => return Err(VerificationIssue::InvalidProof),
    }
    let attempt = review.attempts[0];
    if let Some(raw) = attempt.inference {
        if raw.hypothesis != review.original || raw.processed_events > 32 || raw.facts_loaded > 8 {
            return Err(VerificationIssue::InvalidProof);
        }
        if let Some(refs) = attempt.evidence {
            let checked = controls::check_proof(input, refs, true)?;
            let valid = match (raw.outcome, checked) {
                (
                    crate::cognition::Outcome::DirectRecall(id),
                    crate::integration::query::VerifiedProof::Direct(r),
                ) => id == r.engram.id(),
                (
                    crate::cognition::Outcome::Inferred(p),
                    crate::integration::query::VerifiedProof::TwoHop { first, second, via },
                ) => {
                    p.first_fact == first.engram.id()
                        && p.second_fact == second.engram.id()
                        && p.via == via
                        && p.via_activations > 0
                }
                _ => false,
            };
            if !valid {
                return Err(VerificationIssue::InvalidProof);
            }
            if classify(report) == Class::Conflict
                && controls::check_proof(input, refs, false) != Err(VerificationIssue::Opposition)
            {
                return Err(VerificationIssue::Opposition);
            }
        }
    }
    Ok(())
}
pub struct Evaluator {
    condition: Condition,
    /// Máscaras dos seis indivíduos do mundo de referência. Não saem pela API candidata.
    extensions: [(u32, u8); 5],
    reserved_released: bool,
}
impl Evaluator {
    pub fn new(condition: Condition) -> Self {
        Self {
            condition,
            extensions: [
                (A, 0b000011),
                (D, 0b000100),
                (B, 0b001111),
                (C, 0b011111),
                (E, 0b101111),
            ],
            reserved_released: false,
        }
    }
    fn truth(&self, q: Statement) -> Option<bool> {
        if q.scope().catalog != scope(2101).catalog {
            return None;
        }
        let a = self
            .extensions
            .iter()
            .find(|(id, _)| *id == q.subject().get())?
            .1;
        let b = self
            .extensions
            .iter()
            .find(|(id, _)| *id == q.object().get())?
            .1;
        let inclusion = a & !b == 0;
        Some(if q.polarity() == Polarity::Positive {
            inclusion
        } else {
            !inclusion
        })
    }
    fn check_control_input(
        &self,
        phase: DecisionPhase,
        input: &PublicInput<'_>,
    ) -> Result<(), VerificationIssue> {
        use crate::cognitive_contracts::EpistemicState;
        use crate::engram::{Cue, Engram, RecordStatus};
        let context = if self.condition == Condition::C3 {
            2102
        } else {
            2101
        };
        let mut facts = vec![
            (1, A, B, Polarity::Positive, 10),
            (2, D, B, Polarity::Positive, 11),
        ];
        if self.condition == Condition::C3 {
            facts.push((3, B, C, Polarity::Positive, 11));
        } else if phase != DecisionPhase::Initial {
            match self.condition {
                Condition::P => facts.push((3, B, C, Polarity::Positive, 30)),
                Condition::C1 => facts.push((3, B, E, Polarity::Positive, 30)),
                Condition::C2 => {
                    facts.push((3, B, C, Polarity::Positive, 30));
                    facts.push((4, B, C, Polarity::Negative, 30));
                }
                _ => {}
            }
        }
        if input.memory.len() != facts.len()
            || input.metadata.len() != facts.len()
            || input.authorization.scope() != scope(2101)
            || input.known_sources != [source(1), source(2)]
        {
            return Err(VerificationIssue::UnexpectedMemory);
        }
        for (id, from, to, polarity, received) in facts {
            let id = EngramId::new(id).unwrap();
            let m = input
                .metadata
                .iter()
                .find(|m| m.id == id)
                .ok_or(VerificationIssue::Metadata)?;
            let mut info = information(from, to, polarity, id.get(), context);
            info.received_at = received;
            if polarity == Polarity::Negative {
                info.declared_source = Some(source(2));
            }
            let observed = if received == 30 { 31 } else { received };
            let expected = Engram::new(
                id,
                Cue::new(context, concept(from).node(), to as i32).unwrap(),
                if polarity == Polarity::Positive {
                    1
                } else {
                    -1
                },
                None,
                observed,
                info.evidence,
                RecordStatus::Observed,
            )
            .unwrap();
            if m.information != info
                || m.state != EpistemicState::AdmittedObservation
                || m.disposition.is_some()
                || input.memory.get(id) != Ok(expected)
            {
                return Err(VerificationIssue::UnexpectedMemory);
            }
        }
        Ok(())
    }
    fn check_exchange(&self, candidate: &CognitiveEpisode) -> Result<(), VerificationIssue> {
        use crate::cognitive_contracts::{ContractError, DeclaredNature, FeedbackContent};
        if self.condition == Condition::C3 {
            return if candidate.request().is_none() && candidate.received().is_none() {
                Ok(())
            } else {
                Err(VerificationIssue::History)
            };
        }
        let request = candidate.request().ok_or(VerificationIssue::History)?;
        if request.id.get() != 1
            || request.episode != candidate.reports()[0].episode.id
            || request.requested_at != 21
            || request.missing != statement(B, C, Polarity::Positive, 2101)
            || request.basis != EvidenceRefs::Direct(EngramId::new(1).unwrap())
        {
            return Err(VerificationIssue::History);
        }
        let positive = information(B, C, Polarity::Positive, 3, 2101);
        let expected_content = match self.condition {
            Condition::C0 => FeedbackContent::Unavailable,
            Condition::C1 => FeedbackContent::One(information(B, E, Polarity::Positive, 3, 2101)),
            Condition::C2 => {
                let mut negative = information(B, C, Polarity::Negative, 4, 2101);
                negative.declared_source = Some(source(2));
                FeedbackContent::Two(positive, negative)
            }
            Condition::C4 => {
                let mut hypothesis = positive;
                hypothesis.nature = DeclaredNature::Hypothesis;
                hypothesis.evidence = None;
                FeedbackContent::One(hypothesis)
            }
            Condition::C5 => FeedbackContent::One(information(B, C, Polarity::Positive, 3, 2102)),
            _ => FeedbackContent::One(positive),
        };
        let returned = candidate.received().ok_or(VerificationIssue::History)?;
        if returned.content != expected_content
            || returned.request != request.id
            || returned.episode != request.episode
            || returned.id.get() != 1
            || returned.returned_at != 30
            || returned.envelope_source != Some(source(1))
        {
            return Err(VerificationIssue::History);
        }
        let rejected = match self.condition {
            Condition::C4 => Some(ContractError::InsufficientEvidence),
            Condition::C5 => Some(ContractError::IncompatibleScope),
            _ => None,
        };
        if let Some(reason) = rejected {
            if candidate.quarantine().len() != 1 || candidate.quarantine()[0].reason != reason {
                return Err(VerificationIssue::History);
            }
        } else if !candidate.quarantine().is_empty() {
            return Err(VerificationIssue::History);
        }
        Ok(())
    }
    /// Liberação única somente após a segunda decisão fixada e no instante previsto.
    pub fn release_reserved(
        &mut self,
        candidate: &CognitiveEpisode,
        at: u64,
    ) -> Result<Statement, VerificationIssue> {
        if self.reserved_released
            || at != 50
            || candidate.phase() != Phase::Reassessed
            || candidate.reports().len() != 2
            || candidate.reports()[1].episode.at != 40
        {
            return Err(VerificationIssue::Leakage);
        }
        self.reserved_released = true;
        Ok(statement(
            D,
            C,
            Polarity::Positive,
            if self.condition == Condition::C3 {
                2102
            } else {
                2101
            },
        ))
    }
    pub fn assess_candidate(
        &self,
        phase: DecisionPhase,
        candidate: &CognitiveEpisode,
    ) -> Evaluation {
        let expected = expected(self.condition, phase, Mechanism::Candidate);
        let Some(report) = candidate.reports().get(phase.index()) else {
            return Evaluation {
                verdict: Verdict::NotEvaluable(Unavailable::MissingDecision),
                expected,
                actual: None,
                reference_truth: None,
            };
        };
        let actual = classify(report);
        let truth = self.truth(report.episode.query);
        let result = (|| -> Result<(), VerificationIssue> {
            history::audit(candidate)?;
            if phase != DecisionPhase::Initial {
                self.check_exchange(candidate)?;
            }
            if phase == DecisionPhase::Reserved && !self.reserved_released {
                return Err(VerificationIssue::Leakage);
            }
            let context = if self.condition == Condition::C3 {
                2102
            } else {
                2101
            };
            let query = statement(
                if phase == DecisionPhase::Reserved {
                    D
                } else {
                    A
                },
                C,
                Polarity::Positive,
                context,
            );
            if report.episode.query != query || report.episode.at != phase.at() {
                return Err(VerificationIssue::Timing);
            }
            if actual != expected {
                return Err(VerificationIssue::UnexpectedClass);
            }
            if actual == Class::Provisional && truth == Some(false) {
                return Err(VerificationIssue::UnexpectedClass);
            }
            let entry=candidate.trace().iter().find(|e|matches!(e.kind,crate::integration::acquisition::TraceKind::Query(id) if id==report.episode.id)).ok_or(VerificationIssue::History)?;
            let mut memory = EngramStore::new(8).unwrap();
            let mut metadata = Vec::new();
            for r in &entry.before {
                memory
                    .record(r.engram)
                    .map_err(|_| VerificationIssue::Metadata)?;
                metadata.push(r.metadata);
            }
            let sources = [source(1), source(2)];
            let input = PublicInput {
                memory: &memory,
                metadata: &metadata,
                authorization: RuleAuthorization::inclusion_in(scope(2101)),
                known_sources: &sources,
                query,
                at: phase.at(),
            };
            let count = if self.condition == Condition::C3 {
                3
            } else if phase == DecisionPhase::Initial {
                2
            } else {
                match self.condition {
                    Condition::P | Condition::C1 => 3,
                    Condition::C2 => 4,
                    _ => 2,
                }
            };
            if memory.len() != count {
                return Err(VerificationIssue::UnexpectedMemory);
            }
            // Inclusive K2: o gate não pode ocultar um caminho factual inicial incompleto.
            if controls::public_records(&input)?.len() != count {
                return Err(VerificationIssue::UnexpectedMemory);
            }
            // A configuração factual inicial não pode ser substituída por dois IDs quaisquer.
            for (id, from, received_at) in [(1, A, 10), (2, D, 11)] {
                let m = metadata
                    .iter()
                    .find(|m| m.id == EngramId::new(id).unwrap())
                    .ok_or(VerificationIssue::UnexpectedMemory)?;
                let r = memory.get(m.id).map_err(|_| VerificationIssue::Metadata)?;
                if m.information.statement != statement(from, B, Polarity::Positive, context)
                    || m.information.declared_source != Some(source(1))
                    || m.information.evidence != Some(crate::engram::EvidenceId::new(id).unwrap())
                    || m.information.received_at != received_at
                    || r.observed_at() != received_at
                    || m.information.nature
                        != crate::cognitive_contracts::DeclaredNature::Observation
                    || m.state != crate::cognitive_contracts::EpistemicState::AdmittedObservation
                    || m.disposition.is_some()
                {
                    return Err(VerificationIssue::UnexpectedMemory);
                }
            }
            for (id, from, to, polarity) in if count == 4 {
                vec![(3, B, C, Polarity::Positive), (4, B, C, Polarity::Negative)]
            } else if count == 3 {
                vec![(
                    3,
                    B,
                    if self.condition == Condition::C1 {
                        E
                    } else {
                        C
                    },
                    Polarity::Positive,
                )]
            } else {
                vec![]
            } {
                let r = metadata
                    .iter()
                    .find(|m| m.id == EngramId::new(id).unwrap())
                    .ok_or(VerificationIssue::UnexpectedMemory)?;
                if r.information.statement != statement(from, to, polarity, context) {
                    return Err(VerificationIssue::UnexpectedMemory);
                }
                let observed = memory.get(r.id).map_err(|_| VerificationIssue::Metadata)?;
                let received = if self.condition == Condition::C3 {
                    11
                } else {
                    30
                };
                if r.information.declared_source != Some(source(if id == 4 { 2 } else { 1 }))
                    || r.information.evidence != Some(crate::engram::EvidenceId::new(id).unwrap())
                    || r.information.received_at != received
                    || observed.observed_at()
                        != if self.condition == Condition::C3 {
                            11
                        } else {
                            31
                        }
                {
                    return Err(VerificationIssue::UnexpectedMemory);
                }
            }
            if let EpisodeDecision::Provisional { premises, rule } = report.episode.decision {
                if rule != query.scope().rule {
                    return Err(VerificationIssue::InvalidScope);
                }
                let verified = controls::check_proof(&input, premises, false)?;
                if report.proof != Some(verified) {
                    return Err(VerificationIssue::InvalidProof);
                }
                let expected_refs = EvidenceRefs::TwoHop {
                    first: EngramId::new(if phase == DecisionPhase::Reserved {
                        2
                    } else {
                        1
                    })
                    .unwrap(),
                    second: EngramId::new(3).unwrap(),
                };
                if premises != expected_refs {
                    return Err(VerificationIssue::UnexpectedProof);
                }
            } else if report.proof.is_some() {
                return Err(VerificationIssue::InvalidProof);
            }
            check_candidate_report(report, &input)?;
            Ok(())
        })();
        Evaluation {
            verdict: verdict(result, truth, actual),
            expected,
            actual: Some(actual),
            reference_truth: truth,
        }
    }
    pub fn assess_control(
        &self,
        phase: DecisionPhase,
        report: Option<&ControlReport>,
        input: &PublicInput<'_>,
        mechanism: Mechanism,
    ) -> Evaluation {
        let expected = expected(self.condition, phase, mechanism);
        let absent = if mechanism == Mechanism::Candidate || !in_recut(self.condition, mechanism) {
            Some(Unavailable::OutsideRecut)
        } else if report.is_none() {
            Some(Unavailable::MissingDecision)
        } else {
            None
        };
        if let Some(reason) = absent {
            return Evaluation {
                verdict: Verdict::NotEvaluable(reason),
                expected,
                actual: None,
                reference_truth: None,
            };
        }
        let report = report.unwrap();
        let truth = self.truth(report.query);
        let result = (|| -> Result<(), VerificationIssue> {
            let context = if self.condition == Condition::C3 {
                2102
            } else {
                2101
            };
            if report.query
                != statement(
                    if phase == DecisionPhase::Reserved {
                        D
                    } else {
                        A
                    },
                    C,
                    Polarity::Positive,
                    context,
                )
                || (phase == DecisionPhase::Reserved && !self.reserved_released)
            {
                return Err(VerificationIssue::Leakage);
            }
            if report.mechanism != mechanism
                || report.query != input.query
                || report.at != phase.at()
                || report.at != input.at
            {
                return Err(VerificationIssue::Timing);
            }
            if report.class != expected {
                return Err(VerificationIssue::UnexpectedClass);
            }
            if matches!(report.class, Class::Provisional | Class::Inferred) && truth == Some(false)
            {
                return Err(VerificationIssue::UnexpectedClass);
            }
            self.check_control_input(phase, input)?;
            history::no_conclusions(&input.memory.iter().collect::<Vec<_>>())?;
            if report.class != Class::OutsideRule
                && report.eligible != controls::public_records(input)?
            {
                return Err(VerificationIssue::Metadata);
            }
            if let Some(refs) = report.proof {
                controls::check_proof(
                    input,
                    refs,
                    matches!(report.class, Class::Conflict | Class::Inferred),
                )?;
                if report.class == Class::Conflict
                    && controls::check_proof(input, refs, false)
                        != Err(VerificationIssue::Opposition)
                {
                    return Err(VerificationIssue::Opposition);
                }
            } else if matches!(report.class, Class::Provisional | Class::Inferred) {
                return Err(VerificationIssue::InvalidProof);
            }
            if matches!(mechanism, Mechanism::NoPropagation | Mechanism::Unmonitored) {
                let raw = report.raw.ok_or(VerificationIssue::InvalidProof)?;
                if raw.hypothesis.context != input.query.scope().context.get()
                    || raw.hypothesis.from != input.query.subject()
                    || raw.hypothesis.to != input.query.object()
                    || raw.processed_events > 32
                {
                    return Err(VerificationIssue::InvalidProof);
                }
                let matched = match (raw.outcome, report.proof, report.class) {
                    (crate::cognition::Outcome::Unsupported, None, Class::Unsupported) => true,
                    (
                        crate::cognition::Outcome::DirectRecall(a),
                        Some(EvidenceRefs::Direct(b)),
                        Class::Provisional,
                    ) => a == b,
                    (
                        crate::cognition::Outcome::Inferred(p),
                        Some(refs @ EvidenceRefs::TwoHop { first, second }),
                        Class::Inferred,
                    ) => {
                        let via = match controls::check_proof(input, refs, true)? {
                            crate::integration::query::VerifiedProof::TwoHop { via, .. } => via,
                            _ => return Err(VerificationIssue::InvalidProof),
                        };
                        p.first_fact == first
                            && p.second_fact == second
                            && p.via == via
                            && p.via_activations > 0
                    }
                    _ => false,
                };
                if !matched {
                    return Err(VerificationIssue::InvalidProof);
                }
            } else if report.raw.is_some() {
                return Err(VerificationIssue::InvalidProof);
            }
            Ok(())
        })();
        Evaluation {
            verdict: verdict(result, truth, report.class),
            expected,
            actual: Some(report.class),
            reference_truth: truth,
        }
    }
}
