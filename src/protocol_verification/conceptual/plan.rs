//! Matriz prospectiva de metadados, sem vetores/rótulos V/F e sem execução.
use super::audit::AuditError;
use crate::protocol_verification::Verdict;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mechanism {
    Candidate,
    Symbolic,
    Literal,
    Trivial,
    Frozen,
    Provided,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    S0,
    S1,
    S2,
    Novel,
    Partial,
    ContextCold,
    ContextLearned,
    Retention,
    Empty,
    Conflict,
    OutsideFamily,
}
impl Block {
    pub const ALL: [Self; 11] = [
        Self::S0,
        Self::S1,
        Self::S2,
        Self::Novel,
        Self::Partial,
        Self::ContextCold,
        Self::ContextLearned,
        Self::Retention,
        Self::Empty,
        Self::Conflict,
        Self::OutsideFamily,
    ];
    pub fn count(self) -> usize {
        match self {
            Self::S0 | Self::S1 | Self::S2 | Self::ContextLearned | Self::Retention => 8,
            Self::Novel => 16,
            _ => 4,
        }
    }
    pub fn logical_at(self) -> u64 {
        match self {
            Self::S0 => 20,
            Self::S1 => 40,
            Self::S2 => 60,
            Self::Novel => 70,
            Self::Partial => 80,
            Self::ContextCold => 90,
            Self::ContextLearned => 110,
            Self::Retention => 120,
            Self::Conflict => 201,
            Self::Empty | Self::OutsideFamily => 20,
        }
    }
    pub(crate) fn ceiling(self) -> bool {
        matches!(
            self,
            Self::S0 | Self::S1 | Self::S2 | Self::Novel | Self::Partial | Self::Retention
        )
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
    pub repetition: u8,
    pub mechanism: Mechanism,
    pub block: Block,
    pub count: usize,
}
pub fn matrix() -> Vec<Cell> {
    let mut cells = vec![];
    for repetition in 1..=2 {
        for mechanism in [
            Mechanism::Candidate,
            Mechanism::Symbolic,
            Mechanism::Literal,
            Mechanism::Trivial,
            Mechanism::Frozen,
            Mechanism::Provided,
        ] {
            for block in Block::ALL {
                if mechanism != Mechanism::Provided || block.ceiling() {
                    cells.push(Cell {
                        repetition,
                        mechanism,
                        block,
                        count: block.count(),
                    });
                }
            }
        }
    }
    cells
}
pub const CLASSIFICATIONS: usize = 864;
pub const SEPARATE_RETRIEVALS: usize = 24;
pub const SEPARATE_ADMISSION_CHECKS: usize = 10;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Key {
    pub repetition: u8,
    pub mechanism: Mechanism,
    pub block: Block,
    pub index: usize,
}
impl Key {
    pub fn check(self) -> Result<(), AuditError> {
        if !(1..=2).contains(&self.repetition) || self.index >= self.block.count() {
            return Err(AuditError::Bounds);
        }
        if self.mechanism == Mechanism::Provided && !self.block.ceiling() {
            return Err(AuditError::OutsideRecut);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScoredRow {
    pub key: Key,
    pub verdict: Verdict,
    pub classification: super::controls::Classification,
}

/// Aplicar por célula/mecanismo, sem somar controles ao sucesso do candidato.
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Metrics {
    pub attempts: usize,
    pub passed: usize,
    pub failed: usize,
    pub not_evaluable: usize,
    pub raw_tp: usize,
    pub raw_tn: usize,
    pub raw_fp: usize,
    pub raw_fn: usize,
    pub verified_correct: usize,
    pub abstained: usize,
    pub no_association: usize,
    pub reasons: Vec<(crate::metacognition::concepts::Abstention, usize)>,
}
impl Metrics {
    pub fn record(
        &mut self,
        verdict: &Verdict,
        decision: Option<crate::metacognition::concepts::ConceptDecision>,
        truth: Option<crate::cognition::concepts::Label>,
    ) -> Result<(), AuditError> {
        use crate::cognition::concepts::Label;
        use crate::metacognition::concepts::ConceptDecision;
        if self.attempts >= CLASSIFICATIONS {
            return Err(AuditError::Bounds);
        }
        self.attempts += 1;
        match verdict {
            Verdict::Pass => self.passed += 1,
            Verdict::Fail(_) => {
                self.failed += 1;
            }
            Verdict::NotEvaluable(_) => {
                self.not_evaluable += 1;
            }
        }
        match decision {
            Some(ConceptDecision::Supported(label)) => match (label, truth) {
                (Label::Positive, Some(Label::Positive)) => {
                    self.raw_tp += 1;
                    if verdict.is_pass() {
                        self.verified_correct += 1;
                    }
                }
                (Label::Negative, Some(Label::Negative)) => {
                    self.raw_tn += 1;
                    if verdict.is_pass() {
                        self.verified_correct += 1;
                    }
                }
                (Label::Positive, Some(Label::Negative)) => self.raw_fp += 1,
                (Label::Negative, Some(Label::Positive)) => self.raw_fn += 1,
                _ => {}
            },
            Some(ConceptDecision::Abstain(reason)) => {
                self.abstained += 1;
                if let Some((_, n)) = self.reasons.iter_mut().find(|(r, _)| *r == reason) {
                    *n += 1;
                } else {
                    self.reasons.push((reason, 1));
                }
            }
            None => {
                self.abstained += 1;
                self.no_association += 1;
            }
        }
        Ok(())
    }
    /// Acurácia BRUTA sobre respostas com verdade definida; não substitui os vereditos.
    pub fn raw_response_accuracy(&self) -> Option<(usize, usize)> {
        let total = self.raw_tp + self.raw_tn + self.raw_fp + self.raw_fn;
        (total > 0).then_some((self.raw_tp + self.raw_tn, total))
    }
}
#[derive(Default)]
pub struct Ledger {
    rows: Vec<ScoredRow>,
    stopped: bool,
}
impl Ledger {
    pub fn rows(&self) -> &[ScoredRow] {
        &self.rows
    }
    pub fn append(&mut self, row: ScoredRow) -> Result<(), AuditError> {
        if self.stopped {
            return Err(AuditError::History);
        }
        row.key.check()?;
        if self.rows.len() >= CLASSIFICATIONS {
            return Err(AuditError::Bounds);
        }
        if self.rows.iter().any(|r| r.key == row.key) {
            return Err(AuditError::Duplicate);
        }
        self.stopped = !row.verdict.is_pass();
        self.rows.push(row);
        Ok(())
    }
    pub fn complete(&self) -> bool {
        !self.stopped && self.rows.len() == CLASSIFICATIONS
    }
}
/// Comparar payloads normalizados completos; tempos/IDs de execução ficam fora.
pub fn deterministic<T: PartialEq>(first: &T, second: &T) -> Result<(), AuditError> {
    if first == second {
        Ok(())
    } else {
        Err(AuditError::History)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gate {
    Integrity,
    Authorization,
    Partitions,
    Tests,
    Reference,
    Custody,
    History,
    Resources,
    Manifest,
}
#[derive(Default)]
pub struct Gates {
    passed: Vec<Gate>,
    stopped: bool,
}
impl Gates {
    pub const ORDER: [Gate; 9] = [
        Gate::Integrity,
        Gate::Authorization,
        Gate::Partitions,
        Gate::Tests,
        Gate::Reference,
        Gate::Custody,
        Gate::History,
        Gate::Resources,
        Gate::Manifest,
    ];
    pub fn record(&mut self, gate: Gate, verdict: Verdict) -> Result<(), AuditError> {
        if self.stopped || Self::ORDER.get(self.passed.len()) != Some(&gate) {
            return Err(AuditError::History);
        }
        if verdict.is_pass() {
            self.passed.push(gate);
        } else {
            self.stopped = true;
        }
        Ok(())
    }
    pub fn ready(&self) -> bool {
        !self.stopped && self.passed.len() == Self::ORDER.len()
    }
}
