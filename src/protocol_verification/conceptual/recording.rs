//! Payload bruto público separado do veredito externo. Sem leitura de datasets.
use super::audit::{AuditError, PublicSnapshot};
use super::plan::Key;
use crate::integration::concepts::ConceptReport;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawFrame {
    pub key: Key,
    pub before: PublicSnapshot,
    pub after: PublicSnapshot,
    pub old_log: Vec<ConceptReport>,
    pub new_log: Vec<ConceptReport>,
    pub report: ConceptReport,
}
impl RawFrame {
    pub fn canonical(&self) -> Result<String, AuditError> {
        self.key.check()?;
        if self.before.memory.len() > 16
            || self.after.memory.len() > 16
            || self.old_log.len() > 80
            || self.new_log.len() > 80
            || self.before.trace.len() > 128
            || self.after.trace.len() > 128
        {
            return Err(AuditError::Bounds);
        }
        fn version(v: &crate::learning::concepts::HypothesisVersion) -> bool {
            v.criteria.len() <= 23 && v.eliminated.len() <= 23 && v.support.len() <= 16
        }
        fn snapshot(s: &PublicSnapshot) -> bool {
            s.metadata.len() <= 16
                && s.models.len() <= 16
                && s.models.iter().map(|m| m.versions.len()).sum::<usize>() <= 32
                && s.models.iter().all(|m| m.versions.iter().all(version))
        }
        fn report(r: &ConceptReport) -> bool {
            r.consulted.len() <= 16
                && r.candidate_predictions.len() <= 2
                && r.version.as_ref().is_none_or(version)
        }
        if !snapshot(&self.before)
            || !snapshot(&self.after)
            || !report(&self.report)
            || !self.old_log.iter().all(report)
            || !self.new_log.iter().all(report)
        {
            return Err(AuditError::Bounds);
        }
        // Debug deriva somente de tipos públicos de CIC; não inclui mundo/cofre/ticket.
        Ok(format!("{self:?}\n"))
    }
    pub fn write_new(&self, path: &Path) -> std::io::Result<()> {
        let raw = self.canonical().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid CIC frame")
        })?;
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        f.write_all(raw.as_bytes())
    }
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceObservation {
    pub wall_ns: Option<u128>,
    pub rss_bytes: Option<u64>,
    pub free_disk_bytes: Option<u64>,
    pub live_data_bytes: Option<u64>,
    pub method: String,
}
impl ResourceObservation {
    /// Ausência não vira zero ou conformidade; preenchimento exige instrumento real.
    pub fn complete_for_resource_claim(&self) -> bool {
        self.wall_ns.is_some()
            && self.rss_bytes.is_some()
            && self.free_disk_bytes.is_some()
            && self.live_data_bytes.is_some()
            && !self.method.is_empty()
    }
}
