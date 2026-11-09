//! Fiscal puro de histórico e mudanças. Não gera decisões ou fatos.
use super::{
    fixtures::{A, B, C, D},
    VerificationIssue,
};
use crate::cognitive_contracts::{EpisodeDecision, EpistemicState, MAX_RECORDS};
use crate::engram::{Engram, RecordStatus};
use crate::integration::{
    acquisition::{AdmissionVerdict, CognitiveEpisode, TraceEntry, TraceKind, MAX_TRACE},
    query::QueryReport,
};

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct HistorySummary {
    pub decisions: usize,
    pub requests: usize,
    pub returns: usize,
    pub admissible: usize,
    pub quarantined: usize,
    pub inserted: usize,
    pub processed_events: usize,
    pub attempts: usize,
}
pub fn no_conclusions(records: &[Engram]) -> Result<(), VerificationIssue> {
    if records
        .iter()
        .any(|r| matches!(r.cue().source().get(), A | D) && r.cue().pattern() == C as i32)
    {
        return Err(VerificationIssue::StoredConclusion);
    }
    Ok(())
}
/// Aceita prefixo para verificações de engenharia; não declara uma execução M5 completa.
pub fn check_trace(
    trace: &[TraceEntry],
    reports: &[QueryReport],
) -> Result<HistorySummary, VerificationIssue> {
    if trace.len() > MAX_TRACE || reports.len() > 3 {
        return Err(VerificationIssue::Bounds);
    }
    let mut summary = HistorySummary::default();
    let mut request = None;
    let mut query_index = 0;
    let mut admitted = Vec::new();
    let gate_only = reports.first().is_some_and(|r| {
        r.episode.decision
            == EpisodeDecision::Abstain(crate::cognitive_contracts::Refusal::RuleOutsideContext)
    });
    for (index, entry) in trace.iter().enumerate() {
        let stage_ok = if gate_only {
            matches!(entry.kind, TraceKind::Query(_))
        } else {
            match index {
                0 | 6 | 7 => matches!(entry.kind, TraceKind::Query(_)),
                1 => matches!(entry.kind, TraceKind::Request(_)),
                2 => matches!(entry.kind, TraceKind::Received { .. }),
                3 => matches!(entry.kind, TraceKind::Interpreted),
                4 => matches!(entry.kind, TraceKind::Assessed { .. }),
                5 => matches!(entry.kind, TraceKind::Incorporated { .. }),
                _ => false,
            }
        };
        if !stage_ok {
            return Err(VerificationIssue::History);
        }
        if entry.before.len() > MAX_RECORDS || entry.after.len() > MAX_RECORDS {
            return Err(VerificationIssue::Bounds);
        }
        if index > 0 && (trace[index - 1].after != entry.before || trace[index - 1].at > entry.at) {
            return Err(VerificationIssue::History);
        }
        no_conclusions(&entry.before.iter().map(|r| r.engram).collect::<Vec<_>>())?;
        no_conclusions(&entry.after.iter().map(|r| r.engram).collect::<Vec<_>>())?;
        if !matches!(entry.kind, TraceKind::Incorporated { .. }) && entry.before != entry.after {
            return Err(VerificationIssue::Mutation);
        }
        match &entry.kind {
            TraceKind::Query(id) => {
                let report = reports.get(query_index).ok_or(VerificationIssue::History)?;
                if report.episode.id != *id
                    || report.episode.at != entry.at
                    || report.episode.before != report.episode.after
                    || report.episode.before.ids()
                        != entry
                            .before
                            .iter()
                            .map(|r| r.engram.id())
                            .collect::<Vec<_>>()
                {
                    return Err(VerificationIssue::History);
                }
                if report.episode.query.object().get() != C
                    || report.episode.query.subject().get() != if query_index == 2 { D } else { A }
                    || entry.at != [20, 40, 50][query_index]
                {
                    return Err(VerificationIssue::Leakage);
                }
                if report.episode.prior != EpistemicState::Hypothesis {
                    return Err(VerificationIssue::History);
                }
                match report.episode.decision {
                    EpisodeDecision::Provisional { .. }
                        if report.episode.posterior != EpistemicState::ProvisionalKnowledge =>
                    {
                        return Err(VerificationIssue::History)
                    }
                    EpisodeDecision::Abstain(_)
                        if report.episode.posterior != EpistemicState::Hypothesis =>
                    {
                        return Err(VerificationIssue::History)
                    }
                    _ => {}
                }
                if let Some(review) = &report.review {
                    if review.attempt_count() > 1 {
                        return Err(VerificationIssue::Bounds);
                    }
                    summary.attempts += review.attempt_count();
                    for a in &review.attempts {
                        if let Some(raw) = a.inference {
                            if raw.processed_events > 32 || raw.facts_loaded > 8 {
                                return Err(VerificationIssue::Bounds);
                            }
                            summary.processed_events += raw.processed_events;
                        }
                    }
                }
                query_index += 1;
                summary.decisions += 1;
            }
            TraceKind::Request(r) => {
                if request.is_some()
                    || query_index != 1
                    || entry.at != 21
                    || r.requested_at != entry.at
                    || r.missing.subject().get() != B
                    || r.missing.object().get() != C
                {
                    return Err(VerificationIssue::History);
                }
                request = Some(*r);
                summary.requests += 1;
            }
            TraceKind::Received {
                package,
                envelope_error,
            } => {
                let r = request.ok_or(VerificationIssue::History)?;
                if summary.returns != 0
                    || entry.at != 30
                    || (envelope_error.is_none() && package.check_link(r).is_err())
                {
                    return Err(VerificationIssue::History);
                }
                summary.returns += 1;
            }
            TraceKind::Interpreted => {
                if summary.returns != 1 || entry.at != 30 {
                    return Err(VerificationIssue::History);
                }
            }
            TraceKind::Assessed { items, policy } => {
                if items.len() > 2 || summary.returns != 1 {
                    return Err(VerificationIssue::Bounds);
                }
                if *policy != crate::integration::acquisition::ADMISSION_POLICY || entry.at != 30 {
                    return Err(VerificationIssue::History);
                }
                for item in items {
                    match item.verdict {
                        AdmissionVerdict::Admissible => {
                            summary.admissible += 1;
                            admitted.push(item.information);
                        }
                        AdmissionVerdict::Quarantined(_) => summary.quarantined += 1,
                    }
                }
            }
            TraceKind::Incorporated { ids } => {
                if entry.at != 31
                    || ids.len() != admitted.len()
                    || entry.after.len() != entry.before.len() + ids.len()
                    || entry.after.get(..entry.before.len()) != Some(entry.before.as_slice())
                {
                    return Err(VerificationIssue::Mutation);
                }
                for ((new, id), info) in entry.after[entry.before.len()..]
                    .iter()
                    .zip(ids)
                    .zip(&admitted)
                {
                    if new.engram.id() != *id
                        || new.metadata.information != *info
                        || new.metadata.state != EpistemicState::AdmittedObservation
                        || new.engram.status() != RecordStatus::Observed
                        || new.engram.observed_at() != entry.at
                    {
                        return Err(VerificationIssue::Mutation);
                    }
                }
                summary.inserted += ids.len();
            }
        }
    }
    if query_index != reports.len() || summary.requests > 1 || summary.returns > 1 {
        return Err(VerificationIssue::History);
    }
    Ok(summary)
}
pub fn audit(candidate: &CognitiveEpisode) -> Result<HistorySummary, VerificationIssue> {
    let result = check_trace(candidate.trace(), candidate.reports())?;
    if candidate.episodes().entries()
        != candidate
            .reports()
            .iter()
            .map(|r| r.episode.clone())
            .collect::<Vec<_>>()
    {
        return Err(VerificationIssue::History);
    }
    if let Some(last) = candidate.trace().last() {
        if last.after.iter().map(|r| r.engram).collect::<Vec<_>>()
            != candidate.memory().iter().collect::<Vec<_>>()
            || last.after.iter().map(|r| r.metadata).collect::<Vec<_>>()
                != candidate
                    .memory()
                    .iter()
                    .map(|r| {
                        *candidate
                            .metadata()
                            .iter()
                            .find(|m| m.id == r.id())
                            .expect("vínculo interno validado")
                    })
                    .collect::<Vec<_>>()
        {
            return Err(VerificationIssue::Mutation);
        }
    }
    Ok(result)
}
