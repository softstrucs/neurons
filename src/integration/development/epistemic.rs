//! Limitações derivadas de experiências persistidas; não revisa a família H23.
//! Avaliação usa Criterion::evaluate e a versão admitida, sem chave do ambiente.
use super::*;
use crate::cognition::concepts::{Attributes, Label};
use crate::curriculum::PedagogyHost;
use crate::learning::concepts::HypothesisVersion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsequenceSample {
    pub origins: Vec<EpisodeId>,
    pub contract: ConsequenceContract,
    pub attributes: Reading,
    pub trial: ActionTrial,
    pub before: Reading,
    pub after: Reading,
    /// Versão da sessão no snapshot, não alegação de relatório histórico salvo.
    pub recorded_version: HypothesisVersion,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EpistemicDiagnosis {
    NoCounterexample,
    InsufficientEvidence,
    HypothesisRefuted,
    ObservationConflict,
    ModelExpressiveness,
    ResourceLimited,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EpistemicQuestion {
    AcquireIndependentEvidence,
    CheckSourceAndContext,
    ReviewAuthorizedRepresentation,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EpistemicReport {
    pub state: EpistemicDiagnosis,
    pub contract: ConsequenceContract,
    pub evaluated_version: HypothesisVersion,
    pub evidence: Vec<ConsequenceSample>,
    pub question: Option<EpistemicQuestion>,
    pub scans: usize,
    pub criterion_evaluations: usize,
    pub justification: String,
}
impl EpistemicReport {
    pub fn blocks_application(&self) -> bool {
        matches!(
            self.state,
            EpistemicDiagnosis::HypothesisRefuted
                | EpistemicDiagnosis::ObservationConflict
                | EpistemicDiagnosis::ModelExpressiveness
                | EpistemicDiagnosis::ResourceLimited
        )
    }
}
impl Runtime {
    /// Reconstrução sem escrita a partir do envelope existente, inclusive depois
    /// de restauração com Access negado. Usada só internamente pela memória.
    pub(super) fn recorded_samples(&self, version: HypothesisVersion) -> Vec<ConsequenceSample> {
        let Some(c) = self.consequence_contract else {
            return vec![];
        };
        self.action_trials
            .iter()
            .filter_map(|t| {
                if t.action != c.action
                    || t.world != c.world
                    || t.revision != c.revision
                    || t.context != c.scope.context.get()
                    || t.source != c.source
                {
                    return None;
                }
                let pos = self.experiences.iter().position(|e| e.record == t.before)?;
                let attributes = &self.experiences.get(pos.checked_sub(1)?)?.reading;
                if !self.sensors.iter().any(|s| {
                    s.id == attributes.sensor
                        && s.encoding == Encoding::BooleanAttributes
                        && s.source == c.source
                        && s.context == t.context
                }) || attributes.source != t.source
                    || attributes.event.logical_time()
                        > self.experiences[pos].reading.event.logical_time()
                {
                    return None;
                }
                Attributes::decode(attributes.event.payload()).ok()?;
                Some(ConsequenceSample {
                    origins: vec![],
                    contract: c,
                    attributes: *attributes,
                    trial: t.clone(),
                    before: self.experiences[pos].reading,
                    after: self
                        .experiences
                        .iter()
                        .find(|e| e.record == t.after)?
                        .reading,
                    recorded_version: version.clone(),
                })
            })
            .collect()
    }
    /// Uma inspeção de atenção por registro, até oito registros e H23 finita.
    /// Resultado parcial é ResourceLimited, nunca ausência de contraexemplo.
    pub fn assess_epistemic(
        &mut self,
        memory: &impl EpisodeMemory,
        session: &Session,
        contract: ConsequenceContract,
        attributes: Attributes,
    ) -> Result<EpistemicReport, String> {
        if !self.access.read_knowledge
            || session.pedagogy_identity()? != (contract.concept, contract.scope)
            || !contract.scope.validity.contains(session.logical_time())
            || memory.memory_contract().is_some_and(|c| c != contract)
            || self.consequence_contract.is_some_and(|c| c != contract)
        {
            return Err("avaliação epistemológica não autorizada/incompatível".into());
        }
        let version = session.pedagogy_version()?;
        let mut samples = memory.memory_samples(session)?;
        samples.extend(self.recorded_samples(version.clone()));
        let mut unique: Vec<ConsequenceSample> = vec![];
        for sample in samples {
            if sample.contract != contract {
                continue;
            }
            if let Some(old) = unique.iter_mut().find(|o| {
                o.trial == sample.trial
                    && o.attributes == sample.attributes
                    && o.before == sample.before
                    && o.after == sample.after
                    && o.recorded_version == sample.recorded_version
            }) {
                old.origins.extend(sample.origins);
                old.origins.sort_by_key(|i| i.get());
                old.origins.dedup();
            } else {
                unique.push(sample)
            }
        }
        let mut report = EpistemicReport {
            state: EpistemicDiagnosis::NoCounterexample,
            contract,
            evaluated_version: version.clone(),
            evidence: vec![],
            question: None,
            scans: 0,
            criterion_evaluations: 0,
            justification:
                "Nenhum contraexemplo elegível nas evidências examinadas; não é prova universal."
                    .into(),
        };
        let predictions: Vec<_> = version
            .criteria
            .iter()
            .map(|h| (*h, h.evaluate(attributes)))
            .collect();
        report.criterion_evaluations = predictions.len();
        let mut refuted = false;
        let mut expressive = false;
        let mut conflict = false;
        let mut labels = vec![];
        // Equivalência empírica é relativa à versão vinculada no snapshot.
        // Não reinterpretar conflitos históricos de versões anteriores como
        // refutação automática de uma revisão posterior autorizada.
        let mut same_version: Vec<(Attributes, Vec<Label>, Label, ConsequenceSample)> = vec![];
        let mut joint_witnesses: Vec<ConsequenceSample> = vec![];
        for sample in unique {
            if report.scans == 8 || self.attention_scans >= self.limits.attention_scans {
                report.state = EpistemicDiagnosis::ResourceLimited;
                report.question = Some(EpistemicQuestion::AcquireIndependentEvidence);
                report.justification =
                    "Cobertura epistemológica incompleta por limite; aplicação suspensa.".into();
                return Ok(report);
            }
            self.attention_scans += 1;
            report.scans += 1;
            if sample.after.event.logical_time() > session.logical_time()
                || !contract
                    .scope
                    .validity
                    .contains(sample.attributes.event.logical_time())
                || !contract
                    .scope
                    .validity
                    .contains(sample.after.event.logical_time())
            {
                continue;
            }
            let observed = Attributes::decode(sample.attributes.event.payload())
                .map_err(|e| format!("{e:?}"))?;
            let actual = Label::from_bool(sample.trial.to == contract.target);
            if sample.recorded_version == version {
                let signature: Vec<_> = version
                    .criteria
                    .iter()
                    .map(|h| h.evaluate(observed))
                    .collect();
                report.criterion_evaluations += signature.len();
                for (old_attributes, old_signature, old_actual, old_sample) in &same_version {
                    if *old_attributes != observed
                        && *old_signature == signature
                        && *old_actual != actual
                    {
                        for witness in [old_sample, &sample] {
                            if !joint_witnesses.contains(witness) {
                                joint_witnesses.push(witness.clone());
                            }
                        }
                    }
                }
                same_version.push((observed, signature, actual, sample.clone()));
            }
            if observed != attributes {
                continue;
            }
            labels.push(actual);
            let compatible = predictions
                .iter()
                .filter(|(_, label)| *label == actual)
                .count();
            expressive |= compatible == 0;
            refuted |= predictions
                .iter()
                .find(|(h, _)| Some(*h) == version.candidate)
                .is_some_and(|(_, label)| *label != actual);
            report.evidence.push(sample);
        }
        if !joint_witnesses.is_empty() && !expressive {
            expressive = true;
            // Mesmas previsões de TODOS os critérios e efeitos opostos: nenhum
            // critério da versão acomoda conjuntamente essas duas experiências.
            // Uma refutação local já suficiente conserva seu relatório/prova.
            for witness in joint_witnesses {
                if !report.evidence.contains(&witness) {
                    report.evidence.push(witness);
                }
            }
        }
        // Mesmos atributos com rótulos opostos: conflito observado, não prova
        // de que uma nova família determinística resolveria a contradição.
        conflict |= labels.iter().any(|l| Some(l) != labels.first());
        if let Some(coord) = session.concepts() {
            for known in coord.examples().map_err(|e| format!("{e:?}"))? {
                if known.example.attributes == attributes
                    && labels.iter().any(|l| *l != known.example.label)
                {
                    conflict = true
                }
            }
        }
        report.state = if conflict {
            EpistemicDiagnosis::ObservationConflict
        } else if expressive {
            EpistemicDiagnosis::ModelExpressiveness
        } else if refuted {
            EpistemicDiagnosis::HypothesisRefuted
        } else if predictions
            .iter()
            .any(|(_, label)| *label == Label::Positive)
            && predictions
                .iter()
                .any(|(_, label)| *label == Label::Negative)
        {
            EpistemicDiagnosis::InsufficientEvidence
        } else {
            EpistemicDiagnosis::NoCounterexample
        };
        report.question = match report.state {
            EpistemicDiagnosis::ObservationConflict => {
                Some(EpistemicQuestion::CheckSourceAndContext)
            }
            EpistemicDiagnosis::ModelExpressiveness => {
                Some(EpistemicQuestion::ReviewAuthorizedRepresentation)
            }
            EpistemicDiagnosis::HypothesisRefuted | EpistemicDiagnosis::InsufficientEvidence => {
                Some(EpistemicQuestion::AcquireIndependentEvidence)
            }
            _ => None,
        };
        report.justification=match report.state {
            EpistemicDiagnosis::ObservationConflict=>"Observações/admissões elegíveis divergem para os mesmos atributos; investigar procedência/contexto.",
            EpistemicDiagnosis::ModelExpressiveness=>"Nenhum critério H23 remanescente acomoda o contraexemplo junto ao conhecimento admitido; limitação relativa à base, não nova verdade ou família.",
            EpistemicDiagnosis::HypothesisRefuted=>"O candidato é refutado, mas há critérios alternativos; nova experiência/revisão exige autorização independente.",
            EpistemicDiagnosis::InsufficientEvidence=>"Critérios discordam; insuficiência não equivale a refutação.",
            _=>"Nenhum contraexemplo elegível nas evidências examinadas; não é prova universal.",
        }.into();
        Ok(report)
    }
}

/// A avaliação é anterior ao ensino e permanece histórica depois da revisão.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EpistemicRevision {
    pub diagnosis: EpistemicReport,
    pub teaching: Option<ConsequenceTraining>,
}
impl Runtime {
    /// Reaproveita uma consequência REAL, sem nova ação ou concessão herdada.
    /// Não tenta adaptar um caso incompatível à família ou apagar exemplos.
    pub fn revise_recovered_counterexample(
        &mut self,
        memory: &impl EpisodeMemory,
        session: &mut Session,
        c: ConsequenceContract,
        attributes: Attributes,
        grant: PredictiveRevision,
    ) -> Result<EpistemicRevision, String> {
        if !self.access.learn_concepts
            || !self.access.read_knowledge
            || self.conceptual_attempted
            || grant.permissions.evaluation != Some(c.scope)
            || grant.permissions.presentation != Some(c.scope)
            || !grant.permissions.admissions.contains(&grant.record)
            || grant.receipt == 0
        {
            return Err("revisão recuperada não autorizada ou consumida".into());
        }
        let diagnosis = self.assess_epistemic(memory, session, c, attributes)?;
        if diagnosis.state != EpistemicDiagnosis::HypothesisRefuted {
            return Ok(EpistemicRevision {
                diagnosis,
                teaching: None,
            });
        }
        let sample = diagnosis
            .evidence
            .iter()
            .find(|s| {
                diagnosis.evaluated_version.candidate.is_some_and(|h| {
                    h.evaluate(attributes) != Label::from_bool(s.trial.to == c.target)
                })
            })
            .ok_or("contraexemplo recuperado ausente")?;
        let lesson = crate::curriculum::Lesson {
            id: grant.record,
            receipt: grant.receipt,
            exemplar: NodeId::new(grant.receipt).map_err(|e| format!("{e:?}"))?,
            attributes,
            label: Label::from_bool(sample.trial.to == c.target),
            source: sample.trial.source,
            evidence: sample.after.evidence,
        };
        let teaching = self.teach_observed_lessons(
            session,
            c,
            vec![sample.trial.id],
            vec![lesson],
            grant.assessment,
            grant.permissions,
        )?;
        Ok(EpistemicRevision {
            diagnosis,
            teaching: Some(teaching),
        })
    }
}
