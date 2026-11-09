//! Fontes e estados sintéticos do harness. Não executa consultas ao preparar uma condição.
use super::{evaluator::Evaluator, PublicInput};
use crate::cognitive_contracts::*;
use crate::engram::{Cue, Engram, EngramId, EngramStore, EvidenceId, RecordStatus};
use crate::event::NodeId;
use crate::integration::{
    acquisition::{AcquisitionError, CognitiveEpisode},
    query::{RecordMetadata, RuleAuthorization},
};

pub const A: u32 = 11;
pub const B: u32 = 37;
pub const C: u32 = 83;
pub const D: u32 = 19;
pub const E: u32 = 61;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Condition {
    P,
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
}
pub const CONDITIONS: [Condition; 7] = [
    Condition::P,
    Condition::C0,
    Condition::C1,
    Condition::C2,
    Condition::C3,
    Condition::C4,
    Condition::C5,
];

pub fn scope(context: u32) -> Scope {
    Scope {
        catalog: CatalogId::new(1).unwrap(),
        context: ContextId::new(context).unwrap(),
        rule: RuleRef {
            id: RuleId::new(1).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 100).unwrap(),
    }
}
pub fn concept(id: u32) -> Concept {
    let name = match id {
        A => "conectores do lote Alfa",
        B => "conectores homologados",
        C => "componentes inspecionados",
        D => "conectores do lote Delta",
        E => "componentes com embalagem registrada",
        _ => "conceito de teste",
    };
    Concept::new(
        NodeId::new(id).unwrap(),
        CatalogId::new(1).unwrap(),
        name,
        name,
    )
    .unwrap()
}
pub fn statement(from: u32, to: u32, polarity: Polarity, context: u32) -> Statement {
    Statement::inclusion(&concept(from), &concept(to), polarity, scope(context)).unwrap()
}
pub fn source(n: u32) -> SourceId {
    SourceId::new(n).unwrap()
}
pub fn information(
    from: u32,
    to: u32,
    polarity: Polarity,
    evidence: u32,
    context: u32,
) -> Information {
    Information {
        statement: statement(from, to, polarity, context),
        declared_source: Some(source(1)),
        evidence: Some(EvidenceId::new(evidence).unwrap()),
        nature: DeclaredNature::Observation,
        received_at: 30,
    }
}

pub struct PreparedCondition {
    pub candidate: CognitiveEpisode,
    pub evaluator: Evaluator,
    condition: Condition,
    authorization: RuleAuthorization,
    sources: Vec<SourceId>,
}
impl PreparedCondition {
    /// Guarda de recorte no harness, antes de executar um mecanismo comparativo.
    pub fn control(
        &self,
        mechanism: super::Mechanism,
        query: Statement,
        at: u64,
    ) -> Result<super::ControlReport, super::Unavailable> {
        if mechanism == super::Mechanism::Candidate
            || !super::evaluator::in_recut(self.condition, mechanism)
        {
            return Err(super::Unavailable::OutsideRecut);
        }
        Ok(super::controls::evaluate(mechanism, &self.input(query, at)))
    }
    /// Apenas a consulta corrente. A consulta futura está sob custódia do avaliador.
    pub fn initial_query(&self) -> Statement {
        statement(
            A,
            C,
            Polarity::Positive,
            if self.condition == Condition::C3 {
                2102
            } else {
                2101
            },
        )
    }
    pub fn input(&self, query: Statement, at: u64) -> PublicInput<'_> {
        PublicInput {
            memory: self.candidate.memory(),
            metadata: self.candidate.metadata(),
            authorization: self.authorization,
            known_sources: &self.sources,
            query,
            at,
        }
    }
    /// Fonte docente externa ao candidato. O caso não é transportado no pacote.
    pub fn teacher_return(
        &self,
        request: InformationRequest,
    ) -> Result<FeedbackPackage, AcquisitionError> {
        if self.condition == Condition::C3 {
            return Err(AcquisitionError::WrongPhase);
        }
        let positive = information(B, C, Polarity::Positive, 3, 2101);
        let content = match self.condition {
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
        Ok(FeedbackPackage {
            id: FeedbackId::new(1).unwrap(),
            request: request.id,
            episode: request.episode,
            envelope_source: Some(source(1)),
            returned_at: 30,
            content,
        })
    }
}

pub fn prepare(condition: Condition) -> Result<PreparedCondition, AcquisitionError> {
    let context = if condition == Condition::C3 {
        2102
    } else {
        2101
    };
    let mut memory = EngramStore::new(8).unwrap();
    let mut metadata = Vec::new();
    let mut facts = vec![(1, A, B, 10), (2, D, B, 11)];
    if condition == Condition::C3 {
        facts.push((3, B, C, 11));
    }
    for (id, from, to, at) in facts {
        let mut info = information(from, to, Polarity::Positive, id, context);
        info.received_at = at;
        let id = EngramId::new(id).unwrap();
        memory
            .record(
                Engram::new(
                    id,
                    Cue::new(context, NodeId::new(from).unwrap(), to as i32).unwrap(),
                    1,
                    None,
                    at,
                    info.evidence,
                    RecordStatus::Observed,
                )
                .unwrap(),
            )
            .unwrap();
        metadata.push(RecordMetadata {
            id,
            information: info,
            state: EpistemicState::AdmittedObservation,
            disposition: None,
        });
    }
    let authorization = RuleAuthorization::inclusion_in(scope(2101));
    let sources = vec![source(1), source(2)];
    let candidate = CognitiveEpisode::new(
        memory,
        &metadata,
        &[concept(A), concept(B), concept(C), concept(D), concept(E)],
        authorization,
        &sources,
    )?;
    Ok(PreparedCondition {
        candidate,
        evaluator: Evaluator::new(condition),
        condition,
        authorization,
        sources,
    })
}
