//! M1: vocabulário e validação estrutural, sem coordenador ou política de aquisição.
//! IDs e fontes são declarados; nenhum construtor verifica verdade ou autentica fonte.
use crate::engram::{EngramId, EvidenceId, RecordStatus};
use crate::event::NodeId;
use crate::metacognition::{AbstainReason, EvidenceRefs};

pub const MAX_RECORDS: usize = 8;
pub const MAX_EPISODES: usize = 3;
pub const MAX_FEEDBACK_ITEMS: usize = 2;
pub const MAX_REQUESTS: usize = 1;
pub const MAX_LABEL_BYTES: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractError {
    InvalidFormat,
    UnknownSource,
    IncompatibleScope,
    InsufficientEvidence,
    Duplicate,
    OutsideValidity,
    InvalidLink,
    Capacity,
    ReservedValidation,
}

macro_rules! id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name(u32);
        impl $name {
            pub fn new(value: u32) -> Result<Self, ContractError> {
                if value == 0 {
                    return Err(ContractError::InvalidFormat);
                }
                Ok(Self(value))
            }
            pub const fn get(self) -> u32 {
                self.0
            }
        }
    };
}
id!(CatalogId);
id!(ContextId);
id!(RuleId);
id!(RuleVersion);
id!(SourceId);
id!(EpisodeId);
id!(RequestId);
id!(FeedbackId);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuleRef {
    pub id: RuleId,
    pub version: RuleVersion,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Validity {
    start: u64,
    end: u64,
}
impl Validity {
    pub fn new(start: u64, end: u64) -> Result<Self, ContractError> {
        if start > end {
            return Err(ContractError::InvalidFormat);
        }
        Ok(Self { start, end })
    }
    pub const fn contains(self, time: u64) -> bool {
        self.start <= time && time <= self.end
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Scope {
    pub catalog: CatalogId,
    pub context: ContextId,
    pub rule: RuleRef,
    pub validity: Validity,
}

/// Classe fornecida pelo catálogo; NodeId é endereço, sem ordem semântica.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Concept {
    node: NodeId,
    catalog: CatalogId,
    name: String,
    definition: String,
}
fn label(value: &str) -> Result<(), ContractError> {
    if value.trim().is_empty()
        || value.len() > MAX_LABEL_BYTES
        || value.chars().any(char::is_control)
    {
        return Err(ContractError::InvalidFormat);
    }
    Ok(())
}
impl Concept {
    pub fn new(
        node: NodeId,
        catalog: CatalogId,
        name: &str,
        definition: &str,
    ) -> Result<Self, ContractError> {
        label(name)?;
        label(definition)?;
        Ok(Self {
            node,
            catalog,
            name: name.into(),
            definition: definition.into(),
        })
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn definition(&self) -> &str {
        &self.definition
    }
    pub const fn node(&self) -> NodeId {
        self.node
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Predicate {
    Inclusion,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Polarity {
    Positive,
    Negative,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Statement {
    subject: NodeId,
    object: NodeId,
    predicate: Predicate,
    polarity: Polarity,
    scope: Scope,
}
impl Statement {
    pub fn inclusion(
        subject: &Concept,
        object: &Concept,
        polarity: Polarity,
        scope: Scope,
    ) -> Result<Self, ContractError> {
        if subject.catalog != scope.catalog || object.catalog != scope.catalog {
            return Err(ContractError::IncompatibleScope);
        }
        if subject.node == object.node {
            return Err(ContractError::InvalidFormat);
        }
        Ok(Self {
            subject: subject.node,
            object: object.node,
            predicate: Predicate::Inclusion,
            polarity,
            scope,
        })
    }
    pub const fn scope(self) -> Scope {
        self.scope
    }
    pub const fn subject(self) -> NodeId {
        self.subject
    }
    pub const fn object(self) -> NodeId {
        self.object
    }
    pub const fn polarity(self) -> Polarity {
        self.polarity
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeclaredNature {
    Observation,
    Hypothesis,
}
/// `declared_source` é o informante externo, nunca Cue.source ou Engram.origin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Information {
    pub statement: Statement,
    pub declared_source: Option<SourceId>,
    pub evidence: Option<EvidenceId>,
    pub nature: DeclaredNature,
    pub received_at: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EpistemicState {
    Received,
    Hypothesis,
    AdmittedObservation,
    Inference,
    ProvisionalKnowledge,
    /// Vocabulário reservado; não há produtor autorizado em M1.
    ExplicitlyValidated,
}
impl EpistemicState {
    /// Só estados de entrada explicitamente admitidos possuem representação factual.
    pub const fn record_status(self) -> Option<RecordStatus> {
        match self {
            Self::Hypothesis => Some(RecordStatus::Hypothesis),
            Self::AdmittedObservation => Some(RecordStatus::Observed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InformationRequest {
    pub id: RequestId,
    pub episode: EpisodeId,
    pub requested_at: u64,
    pub missing: Statement,
    pub basis: EvidenceRefs,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeedbackContent {
    Unavailable,
    One(Information),
    Two(Information, Information),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FeedbackPackage {
    pub id: FeedbackId,
    pub request: RequestId,
    pub episode: EpisodeId,
    pub envelope_source: Option<SourceId>,
    pub returned_at: u64,
    pub content: FeedbackContent,
}
impl FeedbackPackage {
    /// Apenas vínculo estrutural. Conteúdo irrelevante pode ser relatado; não ganha suporte.
    pub fn check_link(self, request: InformationRequest) -> Result<(), ContractError> {
        if self.request != request.id
            || self.episode != request.episode
            || self.returned_at < request.requested_at
        {
            return Err(ContractError::InvalidLink);
        }
        Ok(())
    }
}

/// Checagem estrutural pura; sucesso não é admissão, autenticação ou promoção.
/// Cada item é examinado com sua própria fonte, nunca a do envelope.
pub fn check_observation(
    info: Information,
    scope: Scope,
    known_sources: &[SourceId],
    at: u64,
) -> Result<(), ContractError> {
    let source = info.declared_source.ok_or(ContractError::UnknownSource)?;
    if !known_sources.contains(&source) {
        return Err(ContractError::UnknownSource);
    }
    if info.statement.scope != scope {
        return Err(ContractError::IncompatibleScope);
    }
    if !scope.validity.contains(at)
        || !scope.validity.contains(info.received_at)
        || info.received_at > at
    {
        return Err(ContractError::OutsideValidity);
    }
    if info.nature != DeclaredNature::Observation || info.evidence.is_none() {
        return Err(ContractError::InsufficientEvidence);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Refusal {
    Contract(ContractError),
    Monitor(AbstainReason),
    RuleOutsideContext,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Disposition {
    Refused,
    Quarantined,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rejection {
    pub feedback: FeedbackId,
    pub item: Option<Information>,
    pub disposition: Disposition,
    pub reason: ContractError,
}
/// Declaração de ato futuro, sem função que o aplique ou produza Validated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationAct {
    pub record: EngramId,
    pub evidence: EvidenceId,
    pub verifier: SourceId,
    pub criterion: RuleRef,
    pub scope: Scope,
    pub at: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EpisodeDecision {
    Abstain(Refusal),
    Provisional {
        premises: EvidenceRefs,
        rule: RuleRef,
    },
}
/// Snapshot exato e limitado de IDs. Conteúdo/proveniência ficam no registro histórico.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemorySnapshot(Vec<EngramId>);
impl MemorySnapshot {
    pub fn new(ids: &[EngramId]) -> Result<Self, ContractError> {
        if ids.len() > MAX_RECORDS {
            return Err(ContractError::Capacity);
        }
        for (i, id) in ids.iter().enumerate() {
            if ids[..i].contains(id) {
                return Err(ContractError::Duplicate);
            }
        }
        Ok(Self(ids.to_vec()))
    }
    pub fn ids(&self) -> &[EngramId] {
        &self.0
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Episode {
    pub id: EpisodeId,
    pub at: u64,
    pub query: Statement,
    pub before: MemorySnapshot,
    pub after: MemorySnapshot,
    pub prior: EpistemicState,
    pub posterior: EpistemicState,
    pub decision: EpisodeDecision,
}
/// Ledger append-only. Não guarda memória factual, não executa consultas/inferências.
#[derive(Default)]
pub struct EpisodeLog {
    entries: Vec<Episode>,
}
impl EpisodeLog {
    pub fn append(&mut self, episode: Episode) -> Result<(), ContractError> {
        if episode.prior == EpistemicState::ExplicitlyValidated
            || episode.posterior == EpistemicState::ExplicitlyValidated
        {
            return Err(ContractError::ReservedValidation);
        }
        if episode.before != episode.after {
            return Err(ContractError::InvalidFormat);
        }
        if !episode.query.scope.validity.contains(episode.at) {
            return Err(ContractError::OutsideValidity);
        }
        if matches!(episode.decision, EpisodeDecision::Abstain(_))
            && episode.prior != episode.posterior
        {
            return Err(ContractError::InvalidFormat);
        }
        if let EpisodeDecision::Provisional { premises, rule } = episode.decision {
            if rule != episode.query.scope.rule {
                return Err(ContractError::IncompatibleScope);
            }
            let present = match premises {
                EvidenceRefs::Direct(id) => episode.before.0.contains(&id),
                EvidenceRefs::TwoHop { first, second } => {
                    first != second
                        && episode.before.0.contains(&first)
                        && episode.before.0.contains(&second)
                }
            };
            if !present {
                return Err(ContractError::InsufficientEvidence);
            }
            if !matches!(
                episode.posterior,
                EpistemicState::Inference | EpistemicState::ProvisionalKnowledge
            ) {
                return Err(ContractError::InvalidFormat);
            }
        }
        if self.entries.iter().any(|entry| entry.id == episode.id) {
            return Err(ContractError::Duplicate);
        }
        if self.entries.len() == MAX_EPISODES {
            return Err(ContractError::Capacity);
        }
        if self.entries.last().is_some_and(|last| last.at > episode.at) {
            return Err(ContractError::OutsideValidity);
        }
        self.entries.push(episode);
        Ok(())
    }
    pub fn entries(&self) -> &[Episode] {
        &self.entries
    }
}

/// Entrada candidata deliberadamente sem extensões, condição experimental,
/// resposta esperada ou consulta futura. O avaliador implementará interface separada.
pub struct CandidateInput<'a> {
    pub query: Statement,
    pub at: u64,
    pub records: &'a [Information],
}
impl CandidateInput<'_> {
    pub fn check_bounds(&self) -> Result<(), ContractError> {
        if self.records.len() > MAX_RECORDS {
            return Err(ContractError::Capacity);
        }
        Ok(())
    }
}

/// Histórico de entradas e recusas, sem métodos de promoção ou remoção.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InformationEntry {
    Received {
        id: EngramId,
        information: Information,
    },
    Rejected {
        id: EngramId,
        rejection: Rejection,
    },
}
impl InformationEntry {
    fn id(self) -> EngramId {
        match self {
            Self::Received { id, .. } | Self::Rejected { id, .. } => id,
        }
    }
}
#[derive(Default)]
pub struct InformationLog {
    entries: Vec<InformationEntry>,
}
impl InformationLog {
    pub fn append(&mut self, entry: InformationEntry) -> Result<(), ContractError> {
        if self.entries.iter().any(|old| old.id() == entry.id()) {
            return Err(ContractError::Duplicate);
        }
        if self.entries.len() == MAX_RECORDS {
            return Err(ContractError::Capacity);
        }
        self.entries.push(entry);
        Ok(())
    }
    pub fn entries(&self) -> &[InformationEntry] {
        &self.entries
    }
}

/// Saída exclusiva do avaliador. Não é entrada de admissão nem ato de validação.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvaluationVerdict {
    pub episode: EpisodeId,
    pub correct_in_reference: bool,
    pub supported_by_public_records: bool,
}
pub trait EpisodeEvaluator {
    fn evaluate(&self, fixed_episode: &Episode) -> EvaluationVerdict;
}

#[cfg(test)]
mod tests {
    use super::*;
    fn scope() -> Scope {
        Scope {
            catalog: CatalogId::new(1).unwrap(),
            context: ContextId::new(2101).unwrap(),
            rule: RuleRef {
                id: RuleId::new(1).unwrap(),
                version: RuleVersion::new(1).unwrap(),
            },
            validity: Validity::new(0, 100).unwrap(),
        }
    }
    fn statement() -> Statement {
        let s = scope();
        // Ordem numérica invertida é válida e não altera o significado da relação.
        let a = Concept::new(NodeId::new(83).unwrap(), s.catalog, "A", "classe A").unwrap();
        let b = Concept::new(NodeId::new(11).unwrap(), s.catalog, "B", "classe B").unwrap();
        Statement::inclusion(&a, &b, Polarity::Positive, s).unwrap()
    }
    fn info() -> Information {
        Information {
            statement: statement(),
            declared_source: Some(SourceId::new(1).unwrap()),
            evidence: Some(EvidenceId::new(1).unwrap()),
            nature: DeclaredNature::Observation,
            received_at: 10,
        }
    }
    fn episode(id: u32) -> Episode {
        let memory = MemorySnapshot::new(&[EngramId::new(1).unwrap()]).unwrap();
        Episode {
            id: EpisodeId::new(id).unwrap(),
            at: 20,
            query: statement(),
            before: memory.clone(),
            after: memory,
            prior: EpistemicState::Hypothesis,
            posterior: EpistemicState::Hypothesis,
            decision: EpisodeDecision::Abstain(Refusal::Monitor(AbstainReason::MissingEvidence)),
        }
    }
    #[test]
    fn valid_construction_without_numeric_semantics() {
        assert_eq!(statement().subject().get(), 83);
        assert_eq!(statement().object().get(), 11);
        assert_eq!(
            check_observation(info(), scope(), &[SourceId::new(1).unwrap()], 20),
            Ok(())
        );
        assert!(Validity::new(0, 100).unwrap().contains(100));
    }
    #[test]
    fn malformed_data() {
        assert_eq!(ContextId::new(0), Err(ContractError::InvalidFormat));
        assert_eq!(Validity::new(2, 1), Err(ContractError::InvalidFormat));
        assert!(Concept::new(NodeId::new(1).unwrap(), scope().catalog, "\n", "classe").is_err());
        assert!(Concept::new(
            NodeId::new(1).unwrap(),
            scope().catalog,
            "A",
            &"x".repeat(129)
        )
        .is_err());
    }
    #[test]
    fn incompatible_catalog_and_reflexivity() {
        let a = Concept::new(
            NodeId::new(1).unwrap(),
            CatalogId::new(2).unwrap(),
            "A",
            "classe",
        )
        .unwrap();
        assert_eq!(
            Statement::inclusion(&a, &a, Polarity::Positive, scope()),
            Err(ContractError::IncompatibleScope)
        );
        let a = Concept::new(NodeId::new(1).unwrap(), scope().catalog, "A", "classe").unwrap();
        assert_eq!(
            Statement::inclusion(&a, &a, Polarity::Positive, scope()),
            Err(ContractError::InvalidFormat)
        );
    }
    #[test]
    fn evidence_id_is_not_source_authentication() {
        assert_eq!(
            check_observation(info(), scope(), &[], 20),
            Err(ContractError::UnknownSource)
        );
        let mut i = info();
        i.declared_source = None;
        assert_eq!(
            check_observation(i, scope(), &[SourceId::new(1).unwrap()], 20),
            Err(ContractError::UnknownSource)
        );
    }
    #[test]
    fn missing_evidence_and_hypothesis_never_admitted_by_check() {
        let sources = [SourceId::new(1).unwrap()];
        let mut i = info();
        i.evidence = None;
        assert_eq!(
            check_observation(i, scope(), &sources, 20),
            Err(ContractError::InsufficientEvidence)
        );
        i = info();
        i.nature = DeclaredNature::Hypothesis;
        assert_eq!(
            check_observation(i, scope(), &sources, 20),
            Err(ContractError::InsufficientEvidence)
        );
        assert_eq!(i.nature, DeclaredNature::Hypothesis);
    }
    #[test]
    fn context_rule_version_and_time_are_checked() {
        let sources = [SourceId::new(1).unwrap()];
        let mut s = scope();
        s.context = ContextId::new(2102).unwrap();
        assert_eq!(
            check_observation(info(), s, &sources, 20),
            Err(ContractError::IncompatibleScope)
        );
        s = scope();
        s.rule.version = RuleVersion::new(2).unwrap();
        assert_eq!(
            check_observation(info(), s, &sources, 20),
            Err(ContractError::IncompatibleScope)
        );
        for at in [9, 101] {
            assert_eq!(
                check_observation(info(), scope(), &sources, at),
                Err(ContractError::OutsideValidity)
            );
        }
    }
    #[test]
    fn feedback_link_and_per_item_source() {
        let request = InformationRequest {
            id: RequestId::new(1).unwrap(),
            episode: EpisodeId::new(1).unwrap(),
            requested_at: 21,
            missing: statement(),
            basis: EvidenceRefs::Direct(EngramId::new(1).unwrap()),
        };
        let mut other = info();
        other.declared_source = Some(SourceId::new(2).unwrap());
        let mut p = FeedbackPackage {
            id: FeedbackId::new(1).unwrap(),
            request: request.id,
            episode: request.episode,
            envelope_source: info().declared_source,
            returned_at: 30,
            content: FeedbackContent::Two(info(), other),
        };
        assert_eq!(p.check_link(request), Ok(()));
        assert_eq!(
            check_observation(other, scope(), &[SourceId::new(1).unwrap()], 40),
            Err(ContractError::UnknownSource)
        );
        p.episode = EpisodeId::new(2).unwrap();
        assert_eq!(p.check_link(request), Err(ContractError::InvalidLink));
        p.episode = request.episode;
        p.returned_at = 20;
        assert_eq!(p.check_link(request), Err(ContractError::InvalidLink));
    }
    #[test]
    fn no_implicit_promotion_or_fact_conversion() {
        for state in [
            EpistemicState::Received,
            EpistemicState::Inference,
            EpistemicState::ProvisionalKnowledge,
            EpistemicState::ExplicitlyValidated,
        ] {
            assert_eq!(state.record_status(), None);
        }
        assert_eq!(
            EpistemicState::Hypothesis.record_status(),
            Some(RecordStatus::Hypothesis)
        );
        let mut log = EpisodeLog::default();
        let mut e = episode(1);
        e.posterior = EpistemicState::ExplicitlyValidated;
        assert_eq!(log.append(e), Err(ContractError::ReservedValidation));
        assert!(log.entries().is_empty());
    }
    #[test]
    fn history_preserved_and_bounded() {
        let mut log = EpisodeLog::default();
        let first = episode(1);
        log.append(first.clone()).unwrap();
        assert_eq!(log.append(first.clone()), Err(ContractError::Duplicate));
        log.append(episode(2)).unwrap();
        log.append(episode(3)).unwrap();
        assert_eq!(log.append(episode(4)), Err(ContractError::Capacity));
        assert_eq!(log.entries()[0], first);
        let mut entries = InformationLog::default();
        for n in 1..=8 {
            entries
                .append(InformationEntry::Received {
                    id: EngramId::new(n).unwrap(),
                    information: info(),
                })
                .unwrap();
        }
        let prefix = entries.entries().to_vec();
        assert_eq!(
            entries.append(InformationEntry::Received {
                id: EngramId::new(9).unwrap(),
                information: info()
            }),
            Err(ContractError::Capacity)
        );
        assert_eq!(entries.entries(), prefix);
    }
    #[test]
    fn refusal_and_quarantine_preserve_original() {
        let mut log = InformationLog::default();
        let i = info();
        let rejected = InformationEntry::Rejected {
            id: EngramId::new(1).unwrap(),
            rejection: Rejection {
                feedback: FeedbackId::new(1).unwrap(),
                item: Some(i),
                disposition: Disposition::Quarantined,
                reason: ContractError::IncompatibleScope,
            },
        };
        log.append(rejected).unwrap();
        assert_eq!(log.entries(), &[rejected]);
        assert_eq!(log.append(rejected), Err(ContractError::Duplicate));
    }
    #[test]
    fn decision_requires_existing_distinct_premise_ids_and_read_only_snapshot() {
        let mut log = EpisodeLog::default();
        let mut e = episode(1);
        e.posterior = EpistemicState::Inference;
        e.decision = EpisodeDecision::Provisional {
            premises: EvidenceRefs::Direct(EngramId::new(2).unwrap()),
            rule: scope().rule,
        };
        assert_eq!(
            log.append(e.clone()),
            Err(ContractError::InsufficientEvidence)
        );
        e.decision = EpisodeDecision::Provisional {
            premises: EvidenceRefs::Direct(EngramId::new(1).unwrap()),
            rule: scope().rule,
        };
        log.append(e.clone()).unwrap();
        assert_eq!(log.entries()[0].before, log.entries()[0].after);
        e.id = EpisodeId::new(2).unwrap();
        e.after = MemorySnapshot::new(&[]).unwrap();
        assert_eq!(log.append(e), Err(ContractError::InvalidFormat));
    }
    #[test]
    fn evaluation_data_has_separate_interface_and_does_not_validate() {
        struct PrivateEvaluator {
            expected: bool,
            extensions: Vec<u32>,
        }
        impl EpisodeEvaluator for PrivateEvaluator {
            fn evaluate(&self, e: &Episode) -> EvaluationVerdict {
                EvaluationVerdict {
                    episode: e.id,
                    correct_in_reference: self.expected && !self.extensions.is_empty(),
                    supported_by_public_records: false,
                }
            }
        }
        let e = episode(1);
        let saved = e.clone();
        let evaluator = PrivateEvaluator {
            expected: true,
            extensions: vec![42],
        };
        let records = [info()];
        let input = CandidateInput {
            query: e.query,
            at: e.at,
            records: &records,
        };
        assert_eq!(input.check_bounds(), Ok(()));
        assert!(evaluator.evaluate(&e).correct_in_reference);
        assert_eq!(e, saved);
        assert_eq!(input.records, records);
        assert_eq!(e.posterior, EpistemicState::Hypothesis);
    }

    #[test]
    fn exchange_is_bounded_and_preserves_request_and_return() {
        let request = InformationRequest {
            id: RequestId::new(1).unwrap(),
            episode: EpisodeId::new(1).unwrap(),
            requested_at: 21,
            missing: statement(),
            basis: EvidenceRefs::Direct(EngramId::new(1).unwrap()),
        };
        let feedback = FeedbackPackage {
            id: FeedbackId::new(1).unwrap(),
            request: request.id,
            episode: request.episode,
            envelope_source: None,
            returned_at: 30,
            content: FeedbackContent::Unavailable,
        };
        let mut log = ExchangeLog::default();
        assert_eq!(
            log.record_feedback(feedback),
            Err(ContractError::InvalidLink)
        );
        log.record_request(request).unwrap();
        assert_eq!(log.record_request(request), Err(ContractError::Capacity));
        log.record_feedback(feedback).unwrap();
        assert_eq!(log.record_feedback(feedback), Err(ContractError::Capacity));
        assert_eq!(log.request(), Some(request));
        assert_eq!(log.feedback(), Some(feedback));
    }

    #[test]
    fn abstention_cannot_promote_hypothesis_and_snapshots_reject_duplicates() {
        let mut e = episode(1);
        e.posterior = EpistemicState::AdmittedObservation;
        assert_eq!(
            EpisodeLog::default().append(e),
            Err(ContractError::InvalidFormat)
        );
        let id = EngramId::new(1).unwrap();
        assert_eq!(
            MemorySnapshot::new(&[id, id]),
            Err(ContractError::Duplicate)
        );
        assert_eq!(MemorySnapshot::new(&[id; 9]), Err(ContractError::Capacity));
    }
}

/// Registro explícito limitado a um pedido e um retorno; não cria pedidos,
/// não seleciona fonte, não admite itens e não atualiza memória factual.
#[derive(Default)]
pub struct ExchangeLog {
    request: Option<InformationRequest>,
    feedback: Option<FeedbackPackage>,
}
impl ExchangeLog {
    pub fn record_request(&mut self, request: InformationRequest) -> Result<(), ContractError> {
        if self.request.is_some() {
            return Err(ContractError::Capacity);
        }
        if !request
            .missing
            .scope
            .validity
            .contains(request.requested_at)
        {
            return Err(ContractError::OutsideValidity);
        }
        self.request = Some(request);
        Ok(())
    }
    pub fn record_feedback(&mut self, feedback: FeedbackPackage) -> Result<(), ContractError> {
        if self.feedback.is_some() {
            return Err(ContractError::Capacity);
        }
        feedback.check_link(self.request.ok_or(ContractError::InvalidLink)?)?;
        self.feedback = Some(feedback);
        Ok(())
    }
    pub const fn request(&self) -> Option<InformationRequest> {
        self.request
    }
    pub const fn feedback(&self) -> Option<FeedbackPackage> {
        self.feedback
    }
}
