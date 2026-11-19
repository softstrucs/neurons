//! Coordenador de investigação conceitual limitada, sobre Runtime/MP09/H23.
//! Ofertas contêm atributos atuais declarados por sensor e autoridade, não rótulos.
use super::*;
use crate::cognition::concepts::{Attributes, Criterion, Label};
use crate::curriculum::PedagogyHost;
use crate::learning::concepts::HypothesisVersion;

mod search;
pub use search::{ContinuedInvestigation, ProbeCorrespondence, ProbeFeasibility, ProbeSearch};

pub struct InvestigationOffer<'a> {
    pub world: &'a mut World,
    pub attributes: Reading,
    pub intervention: Intervention,
}

fn validate_offer_identities(offers: &[InvestigationOffer<'_>]) -> Result<(), String> {
    if offers.iter().enumerate().any(|(i, o)| {
        offers[..i].iter().any(|old| {
            old.attributes.id == o.attributes.id
                || old.attributes.evidence == o.attributes.evidence
                || old.intervention.id == o.intervention.id
                || old.intervention.permit.id == o.intervention.permit.id
        })
    }) {
        return Err("identidades de oferta ambíguas".into());
    }
    Ok(())
}

/// Suborçamento da atenção existente, não uma nova reserva independente.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProbeBudget {
    pub patterns: usize,
    pub evaluations: usize,
}
impl Default for ProbeBudget {
    fn default() -> Self {
        Self {
            patterns: 32,
            evaluations: 32 * 23,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProbeStatus {
    Ready,
    NoDisagreement,
    RepresentationLimited,
    ResourceLimited,
    Exhausted,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeSelection {
    pub version: HypothesisVersion,
    pub status: ProbeStatus,
    /// Propostas hipotéticas sem rótulos; não são observações do ambiente.
    pub patterns: Vec<Attributes>,
    pub inspected: usize,
    pub evaluations: usize,
    pub excluded: usize,
    pub space_size: usize,
    /// Ready pode cobrir somente um prefixo, sem alegar ótimo ou exaustividade.
    pub exhaustive: bool,
    pub round_inspected: usize,
    pub round_evaluations: usize,
}
/// Autoridade pedagógica explícita, independente da proposta de sondas.
pub struct ProbeTeaching {
    pub record: EngramId,
    pub receipt: u32,
    pub permissions: PedagogyPermissions,
}
pub struct OperationalInvestigation {
    pub probes: ProbeSelection,
    /// Diagnóstico existente, inclusive quando nenhuma investigação é liberada.
    /// Uma versão unânime pode estar refutada por experiências posteriores.
    pub epistemic: EpistemicReport,
    /// Ausente quando a busca não forneceu duas sondas utilizáveis.
    /// Não equivale a objetivo resolvido ou evidência suficiente.
    pub investigation: Option<DiscriminatingInvestigation>,
}

impl Runtime {
    /// Ordem fixa dos 64 padrões completos, sem regra-alvo ou consequência.
    /// Duas sondas separadas das experiências conhecidas/ofertadas. Cada padrão
    /// inspecionado consome atenção, mesmo quando excluído; cada avaliação é
    /// contabilizada. Cobertura parcial sem duas sondas nunca libera avaliação.
    pub fn select_discriminating_probes(
        &mut self,
        session: &Session,
        offered: &[Attributes],
        budget: ProbeBudget,
    ) -> Result<ProbeSelection, String> {
        let mut search = ProbeSearch::make(session, None, offered)?;
        self.advance_probe_search(session, &mut search, budget)
    }

    /// Usa SOMENTE atributos atuais das ofertas; destinos de World continuam
    /// privados. Propõe sondas de avaliação, depois delega diagnóstico, seleção
    /// da experiência real, permissões, execução e admissão ao fluxo existente.
    /// Sem concessão pedagógica pode observar, mas nunca incorporar por seleção.
    pub fn investigate_with_selected_probes(
        &mut self,
        memory: &impl EpisodeMemory,
        session: &mut Session,
        contract: ConsequenceContract,
        question: Attributes,
        offers: &mut [InvestigationOffer<'_>],
        teaching: Option<ProbeTeaching>,
        budget: ProbeBudget,
    ) -> Result<OperationalInvestigation, String> {
        if offers.len() > 4 {
            return Err("máximo de quatro ofertas".into());
        }
        let offered = offers
            .iter()
            .map(|o| Attributes::decode(o.attributes.event.payload()).map_err(|e| format!("{e:?}")))
            .collect::<Result<Vec<_>, _>>()?;
        let probes = self.select_discriminating_probes(session, &offered, budget)?;
        self.finish_probe_investigation(
            memory, session, contract, question, offers, teaching, probes,
        )
    }

    fn finish_probe_investigation(
        &mut self,
        memory: &impl EpisodeMemory,
        session: &mut Session,
        contract: ConsequenceContract,
        question: Attributes,
        offers: &mut [InvestigationOffer<'_>],
        teaching: Option<ProbeTeaching>,
        probes: ProbeSelection,
    ) -> Result<OperationalInvestigation, String> {
        if probes.status != ProbeStatus::Ready {
            let epistemic = self.assess_epistemic(memory, session, contract, question)?;
            return Ok(OperationalInvestigation {
                probes,
                epistemic,
                investigation: None,
            });
        }
        let revision = teaching
            .map(|grant| -> Result<PredictiveRevision, String> {
                Ok(PredictiveRevision {
                    record: grant.record,
                    receipt: grant.receipt,
                    permissions: grant.permissions,
                    assessment: PedagogyAssessment::new(
                        &[probes.patterns[0].into()],
                        &[probes.patterns[1].into()],
                    )?,
                })
            })
            .transpose()?;
        let investigation =
            self.investigate_disagreement(memory, session, contract, question, offers, revision)?;
        Ok(OperationalInvestigation {
            probes,
            epistemic: investigation.origin.clone(),
            investigation: Some(investigation),
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscriminatingOption {
    pub reading: Reading,
    pub predictions: Vec<(Criterion, Label)>,
    /// Menor grupo separado, sem probabilidades ou confiança inventadas.
    pub separation: usize,
    /// Controle habilitado no simulador; ainda sujeito a permissão e reserva.
    pub realizable: bool,
    pub authorized: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvestigationResult {
    NotNeeded,
    NoDiscriminatingOffer,
    Denied,
    ResourceLimited,
    RepresentationLimited,
    Observed,
    Learned,
    ConflictRetained,
    Interrupted,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscriminatingInvestigation {
    pub origin: EpistemicReport,
    pub version: HypothesisVersion,
    pub options: Vec<DiscriminatingOption>,
    pub selected: Option<u32>,
    pub question: String,
    pub trial: Option<ActionTrial>,
    pub review: Option<ConsequenceReview>,
    pub selection_scans: usize,
    pub criterion_evaluations: usize,
    /// Filtro diagnóstico transitório; a revisão autoritativa é só a de MP09.
    pub retained: Vec<Criterion>,
    pub refuted: Vec<Criterion>,
    pub teaching: Option<ConsequenceTraining>,
    pub after: Option<EpistemicReport>,
    pub requires_retrieval: bool,
    pub result: InvestigationResult,
    pub failure: Option<String>,
}
impl Runtime {
    fn assess_investigation_offer(
        &self,
        memory: &impl EpisodeMemory,
        session: &Session,
        c: ConsequenceContract,
        offer: &InvestigationOffer<'_>,
        version: &HypothesisVersion,
    ) -> Result<DiscriminatingOption, String> {
        let q = offer.world.attribute_query(c.action);
        let sensor = self
            .sensors
            .iter()
            .find(|s| s.id == offer.attributes.sensor)
            .ok_or("sensor da oferta ausente")?;
        if q.world != c.world
            || q.revision != c.revision
            || q.context != c.scope.context.get()
            || q.source != c.source
            || offer.attributes.item.get() != q.world
            || offer.attributes.source != q.source
            || sensor.encoding != Encoding::BooleanAttributes
            || sensor.context != q.context
            || sensor.source != q.source
            || !c
                .scope
                .validity
                .contains(offer.attributes.event.logical_time())
            || offer.attributes.event.logical_time() > session.logical_time()
            || offer.attributes.event.logical_time() > offer.intervention.at
            || !c.scope.validity.contains(offer.intervention.at)
            || offer.intervention.at >= session.logical_time()
        {
            return Err("oferta fora do vínculo/contexto/validade".into());
        }
        let attributes =
            Attributes::decode(offer.attributes.event.payload()).map_err(|e| format!("{e:?}"))?;
        let predictions: Vec<_> = version
            .criteria
            .iter()
            .map(|h| (*h, h.evaluate(attributes)))
            .collect();
        let positive = predictions
            .iter()
            .filter(|(_, l)| *l == Label::Positive)
            .count();
        let separation = positive.min(predictions.len() - positive);
        let realizable =
            offer.world.available_actions().contains(&c.action) && offer.world.action_count() < 8;
        let authorized = realizable
            && self.world_observable(offer.world)
            && self.access.sensors.contains(&sensor.id)
            && self.connected(sensor)
            && self.permit_matches(offer.world, &offer.intervention)
            && offer.intervention.permit.action == c.action
            && !memory.memory_used(&offer.intervention)
            && self.valid_requests(offer.world, std::slice::from_ref(&offer.intervention));
        Ok(DiscriminatingOption {
            reading: offer.attributes,
            predictions,
            separation,
            realizable,
            authorized,
        })
    }

    /// Até quatro ofertas, uma experiência e o episódio MP09 já existente.
    /// Todos os destinos permanecem privados até intervene. Sem investigação
    /// implícita, escolha por gabarito, nova família ou admissão relacional.
    pub fn investigate_disagreement(
        &mut self,
        memory: &impl EpisodeMemory,
        session: &mut Session,
        c: ConsequenceContract,
        question_attributes: Attributes,
        offers: &mut [InvestigationOffer<'_>],
        teaching: Option<PredictiveRevision>,
    ) -> Result<DiscriminatingInvestigation, String> {
        if !self.access.read_knowledge
            || !self.access.explore
            || !self.access.record
            || self.verification_attempted
            || offers.len() > 4
        {
            return Err("investigação negada, consumida ou excessiva".into());
        }
        validate_offer_identities(offers)?;
        if let Some(g) = &teaching {
            if !self.access.learn_concepts
                || self.conceptual_attempted
                || g.receipt == 0
                || g.permissions.evaluation != Some(c.scope)
                || g.permissions.presentation != Some(c.scope)
                || !g.permissions.admissions.contains(&g.record)
            {
                return Err("autoridade pedagógica independente ausente".into());
            }
            session.pedagogy_capacity(1)?;
        }
        let origin = self.assess_epistemic(memory, session, c, question_attributes)?;
        let version = session.pedagogy_version()?;
        let mut out = DiscriminatingInvestigation {
            origin,
            version: version.clone(),
            options: vec![],
            selected: None,
            question:
                "Qual experiência autorizada separa previsões dos critérios H23 ainda elegíveis?"
                    .into(),
            trial: None,
            review: None,
            selection_scans: 0,
            criterion_evaluations: 0,
            retained: version.criteria.clone(),
            refuted: vec![],
            teaching: None,
            after: None,
            requires_retrieval: false,
            result: InvestigationResult::NotNeeded,
            failure: None,
        };
        match out.origin.state {
            EpistemicDiagnosis::ResourceLimited => {
                out.result = InvestigationResult::ResourceLimited;
                return Ok(out);
            }
            EpistemicDiagnosis::ModelExpressiveness => {
                out.result = InvestigationResult::RepresentationLimited;
                return Ok(out);
            }
            EpistemicDiagnosis::NoCounterexample => return Ok(out),
            _ => {}
        }
        let mut selected = None;
        let mut best = 0;
        for (i, offer) in offers.iter().enumerate() {
            if self.attention_scans >= self.limits.attention_scans {
                out.result = InvestigationResult::ResourceLimited;
                return Ok(out);
            }
            self.attention_scans += 1;
            out.selection_scans += 1;
            let option = self.assess_investigation_offer(memory, session, c, offer, &version)?;
            out.criterion_evaluations += option.predictions.len();
            let authorized = option.authorized;
            let separation = option.separation;
            out.options.push(option);
            // Desempate estável por ID de observação, nunca consequência oculta.
            if authorized
                && separation > 0
                && (separation > best
                    || (separation == best
                        && selected
                            .is_some_and(|j: usize| offer.attributes.id < offers[j].attributes.id)))
            {
                best = separation;
                selected = Some(i);
            }
        }
        let Some(i) = selected else {
            out.result = if out.options.iter().any(|o| o.separation > 0) {
                InvestigationResult::Denied
            } else {
                InvestigationResult::NoDiscriminatingOffer
            };
            return Ok(out);
        };
        let trials = self.remembered_trials(memory)?;
        let neural = self
            .neural
            .circuit()
            .ok_or("circuito ausente")?
            .engine_metrics()
            .dispatched as usize;
        if self.planning_queries >= 4
            || self.planning_scans + trials.len() > 32
            || self.experiences.len() + 3 > self.limits.observations
            || self.room(3).is_err()
            || self.intervention_room(1).is_err()
            || self.neural.experiences().len() + 3 > 8
            || neural + self.action_reserved_events + 48 > self.limits.neural_events
        {
            out.result = InvestigationResult::ResourceLimited;
            return Ok(out);
        }
        if self.consequence_contract.is_none() && memory.memory_contract().is_some() {
            self.inherit_consequence_contract(memory)?;
        }
        let offer = &mut offers[i];
        out.selected = Some(offer.attributes.id);
        let frame = match self.observe_attributes(offer.world, c.action, offer.attributes) {
            Ok(frame) => frame,
            Err(e) => {
                out.result = InvestigationResult::Interrupted;
                out.failure = Some(e);
                return Ok(out);
            }
        };
        let forecast = match self.forecast_using(
            session,
            c,
            &frame,
            EpisodeId::new(offer.intervention.id).map_err(|e| format!("{e:?}"))?,
            &trials,
        ) {
            Ok(f) => f,
            Err(e) => {
                out.result = InvestigationResult::Interrupted;
                out.failure = Some(e);
                return Ok(out);
            }
        };
        out.question=format!("Observação {} no estado {}: a ação {} atingirá {}? As alternativas registradas discordam.",offer.attributes.id,offer.world.state(),c.action,c.target);
        // Investigar uma hipótese não é executar sua previsão como fato.
        let trial = match self.intervene(offer.world, &offer.intervention) {
            Ok(t) => t,
            Err(e) => {
                out.result = InvestigationResult::Interrupted;
                out.failure = Some(e);
                return Ok(out);
            }
        };
        out.trial = Some(trial.clone());
        let review = match self.review_consequence(session, forecast, trial.id) {
            Ok(review) => review,
            Err(e) => {
                out.result = InvestigationResult::Interrupted;
                out.failure = Some(e);
                return Ok(out);
            }
        };
        let actual = Label::from_bool(trial.to == c.target);
        out.retained = out.options[i]
            .predictions
            .iter()
            .filter(|(_, label)| *label == actual)
            .map(|(h, _)| *h)
            .collect();
        out.refuted = version
            .criteria
            .iter()
            .filter(|h| !out.retained.contains(h))
            .copied()
            .collect();
        out.review = Some(review);
        out.result = if out.retained.is_empty() {
            InvestigationResult::RepresentationLimited
        } else {
            InvestigationResult::Observed
        };
        if let Some(grant) = teaching {
            let report = match self.teach_consequences(
                session,
                c,
                &[(frame, trial.id, grant.record, grant.receipt)],
                grant.assessment,
                grant.permissions,
            ) {
                Ok(report) => report,
                Err(e) => {
                    out.result = InvestigationResult::Interrupted;
                    out.failure = Some(e);
                    return Ok(out);
                }
            };
            out.requires_retrieval = report.pedagogy.final_version != version;
            if report.pedagogy.lessons.iter().any(|l| l.admitted) {
                out.result = InvestigationResult::Learned;
            }
            out.teaching = Some(report);
        }
        if !out.requires_retrieval {
            let after = self.assess_epistemic(memory, session, c, question_attributes)?;
            if after.state == EpistemicDiagnosis::ResourceLimited {
                out.result = InvestigationResult::ResourceLimited;
            }
            out.after = Some(after);
        }
        // Uma observação adicional não apaga conflito histórico nem comprova
        // qual fonte errou; os critérios retidos continuam provisórios.
        if out.origin.state == EpistemicDiagnosis::ObservationConflict
            && out.result != InvestigationResult::ResourceLimited
        {
            out.result = InvestigationResult::ConflictRetained;
        }
        Ok(out)
    }
}
