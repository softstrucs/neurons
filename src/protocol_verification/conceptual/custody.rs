//! Custódia CIC-I2: somente D pode ser carregada/liberada. V/F não têm loader.
//! Não deriva Debug/Serialize de mundo, cofre, ticket ou fila privada.
use super::audit::{self, AuditError, AuditPolicy, PublicSnapshot};
use super::controls::{classification, Classification};
use crate::cognition::concepts::{Attribute, Attributes, Label, Observation};
use crate::cognitive_contracts::{EpisodeId, Scope};
use crate::integration::concepts::ConceptReport;
use crate::protocol_verification::{Verdict, VerificationIssue};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Partition {
    Development,
    Validation,
    Final,
}
/// Configuração exclusiva do docente/avaliador, não da API do candidato.
/// O mundo privado não pode ser despejado por depuração genérica.
/// ```compile_fail
/// use neurons::protocol_verification::conceptual::custody::WorldRule;
/// fn require_debug<T: std::fmt::Debug>() {}
/// require_debug::<WorldRule>();
/// ```
pub enum WorldRule {
    Constant(bool),
    Attribute(Attribute),
    Conjunction(Attribute, Attribute),
    Xor(Attribute, Attribute),
}
impl WorldRule {
    fn truth(&self, x: Observation) -> Option<Label> {
        let labels: Vec<_> = x
            .completions()
            .into_iter()
            .map(|a| {
                let v = a.values();
                let get = |i: Attribute| v[Attribute::ALL.iter().position(|a| *a == i).unwrap()];
                Label::from_bool(match *self {
                    Self::Constant(b) => b,
                    Self::Attribute(a) => get(a),
                    Self::Conjunction(a, b) => get(a) && get(b),
                    Self::Xor(a, b) => get(a) != get(b),
                })
            })
            .collect();
        if labels.iter().all(|l| *l == labels[0]) {
            Some(labels[0])
        } else {
            None
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicQuery {
    pub episode: EpisodeId,
    pub scope: Scope,
    pub observation: Observation,
    pub at: u64,
}
pub struct Ticket {
    vault: u64,
    index: usize,
}
static VAULT_IDS: AtomicU64 = AtomicU64::new(1);
/// Não existe Debug ou acesso público ao mundo/fila futura.
/// ```compile_fail
/// use neurons::protocol_verification::conceptual::custody::DevelopmentVault;
/// fn require_debug<T: std::fmt::Debug>() {}
/// require_debug::<DevelopmentVault>();
/// ```
/// ```compile_fail
/// use neurons::protocol_verification::conceptual::custody::DevelopmentVault;
/// fn leak(v: &DevelopmentVault) { let _ = &v.world; }
/// ```
pub struct DevelopmentVault {
    id: u64,
    world: WorldRule,
    queries: Vec<PublicQuery>,
    next: usize,
    pending: bool,
    stopped: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Assessment {
    pub verdict: Verdict,
    pub classification: Classification,
    pub truth: Option<Label>,
    pub instrument_issue: Option<AuditError>,
}
impl DevelopmentVault {
    pub fn new(world: WorldRule, queries: Vec<PublicQuery>) -> Result<Self, AuditError> {
        if queries.is_empty() || queries.len() > 80 {
            return Err(AuditError::Bounds);
        }
        for (i, q) in queries.iter().enumerate() {
            if !q.scope.validity.contains(q.at) || i > 0 && queries[i - 1].at > q.at {
                return Err(AuditError::Timing);
            }
            if queries[..i].iter().any(|old| old.episode == q.episode) {
                return Err(AuditError::Duplicate);
            }
        }
        let id = VAULT_IDS
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
            .map_err(|_| AuditError::Bounds)?;
        Ok(Self {
            id,
            world,
            queries,
            next: 0,
            pending: false,
            stopped: false,
        })
    }
    pub fn release(&mut self, partition: Partition) -> Result<(PublicQuery, Ticket), AuditError> {
        // Portão anterior a qualquer leitura da fila ou cálculo do mundo.
        if partition != Partition::Development {
            return Err(AuditError::Reserved);
        }
        if self.stopped {
            return Err(AuditError::History);
        }
        if self.pending {
            return Err(AuditError::History);
        }
        let query = self
            .queries
            .get(self.next)
            .ok_or(AuditError::Missing)?
            .clone();
        self.pending = true;
        Ok((
            query,
            Ticket {
                vault: self.id,
                index: self.next,
            },
        ))
    }
    pub fn submit(
        &mut self,
        ticket: Ticket,
        before: &PublicSnapshot,
        after: &PublicSnapshot,
        old_log: &[ConceptReport],
        new_log: &[ConceptReport],
        report: &ConceptReport,
        policy: &AuditPolicy,
    ) -> Result<Assessment, AuditError> {
        if !self.pending || ticket.vault != self.id || ticket.index != self.next {
            return Err(AuditError::Leakage);
        }
        let q = &self.queries[self.next];
        if report.episode != q.episode
            || report.scope != q.scope
            || report.observation != q.observation
            || report.at != q.at
        {
            return Err(AuditError::Leakage);
        }
        // Decisão já fixada e fiscalizada antes de qualquer liberação do veredito/verdade.
        let checked = audit::query(before, after, old_log, new_log, report, policy);
        let truth = self.world.truth(q.observation);
        let class = classification(Some(report.decision), truth);
        self.pending = false;
        self.next += 1;
        let instrument_issue = checked.err();
        let verdict = if instrument_issue.is_some() {
            Verdict::Fail(VerificationIssue::InvalidProof)
        } else if class == Classification::AmbiguousTruth {
            Verdict::NotEvaluable(crate::protocol_verification::Unavailable::ReferenceUnavailable)
        } else if class == Classification::Incorrect {
            Verdict::Fail(VerificationIssue::UnexpectedClass)
        } else {
            Verdict::Pass
        };
        self.stopped = !verdict.is_pass();
        Ok(Assessment {
            verdict,
            classification: class,
            truth,
            instrument_issue,
        })
    }
}

/// Verifica reutilização indevida sobre dados fornecidos pelo harness autorizado.
/// Testes I2 usam somente recortes D com sobreposição deliberada, nunca V/F reais.
pub fn disjoint(partitions: &[&[Attributes]]) -> Result<(), AuditError> {
    if partitions.len() > 3 || partitions.iter().any(|p| p.len() > 64) {
        return Err(AuditError::Bounds);
    }
    for (i, p) in partitions.iter().enumerate() {
        if p.iter().enumerate().any(|(j, a)| p[..j].contains(a)) {
            return Err(AuditError::Duplicate);
        }
        for old in &partitions[..i] {
            if p.iter().any(|a| old.contains(a)) {
                return Err(AuditError::Leakage);
            }
        }
    }
    Ok(())
}
