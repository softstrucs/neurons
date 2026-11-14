//! Ponte operacional: atributos observados → MP09/H23 → previsão de consequência.
//! O significado do conceito (ação/destino) é configuração explícita, não uma
//! relação descoberta. Rótulos de aquisição vêm de ações já observadas; nenhuma
//! tabela de consequências futuras entra na consulta H23. Relatórios transitórios.
use super::*;
use crate::cognition::concepts::{Attributes, Label};
use crate::cognitive_contracts::Scope;
use crate::curriculum::{Lesson, PedagogyHost, PedagogyReport};
use crate::integration::concepts::ConceptReport;
use crate::metacognition::concepts::ConceptDecision;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttributeFrame {
    query: TransitionQuery,
    reading: Reading,
    record: EngramId,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConsequenceContract {
    pub concept: NodeId,
    pub scope: Scope,
    pub world: u32,
    pub revision: u32,
    pub source: SourceId,
    pub action: u32,
    pub target: i32,
}
impl ConsequenceContract {
    fn matches(self, q: TransitionQuery) -> bool {
        self.world == q.world
            && self.revision == q.revision
            && self.scope.context.get() == q.context
            && self.source == q.source
            && self.action == q.action
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsequenceForecast {
    contract: ConsequenceContract,
    frame: AttributeFrame,
    report: ConceptReport,
    transitions: Prediction,
}
impl ConsequenceForecast {
    pub(super) fn frame(&self) -> &AttributeFrame {
        &self.frame
    }
    pub fn report(&self) -> &ConceptReport {
        &self.report
    }
    pub fn contract(&self) -> ConsequenceContract {
        self.contract
    }
    pub fn transitions(&self) -> &Prediction {
        &self.transitions
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsequenceTraining {
    pub contract: ConsequenceContract,
    pub trials: Vec<u32>,
    pub pedagogy: PedagogyReport,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsequenceReview {
    pub forecast: ConsequenceForecast,
    pub trial: ActionTrial,
    pub actual: Label,
    /// None significa abstenção, nunca acerto presumido.
    pub agreement: Option<bool>,
    /// Estado exato, distinto do rótulo conceitual de destino.
    pub transition_agreement: Option<bool>,
    pub review_required: bool,
}
impl Runtime {
    /// Somente configuração sem autoridade. Uma nova instância pode continuar
    /// a mesma hipótese; não transfere recursos, fontes, concessões ou episódios.
    pub fn inherit_consequence_contract(&mut self, previous: &Runtime) -> Result<(), String> {
        if self.consequence_contract.is_some()
            || self.conceptual_attempted
            || self.verification_attempted
        {
            return Err("contrato da instância já fixado".into());
        }
        self.consequence_contract = Some(
            previous
                .consequence_contract
                .ok_or("contrato anterior ausente")?,
        );
        Ok(())
    }
    fn check_consequence_contract(
        &self,
        session: &Session,
        c: ConsequenceContract,
    ) -> Result<(), String> {
        match self.consequence_contract {
            Some(old) if old != c => Err("mudança de significado da hipótese de consequência".into()),
            None if session.pedagogy_version()?.number != 0 =>
                Err("hipótese adquirida exige seu contrato original, por restauração ou herança explícita".into()),
            _ => Ok(()),
        }
    }
    /// Leitura de atributos de um estado ATUAL. O sensor é a origem declarada
    /// dos seis valores; IDs escalares não são convertidos em atributos.
    pub fn observe_attributes(
        &mut self,
        world: &World,
        action: u32,
        reading: Reading,
    ) -> Result<AttributeFrame, String> {
        let query = world.attribute_query(action);
        let sensor = self
            .sensors
            .iter()
            .find(|s| s.id == reading.sensor)
            .ok_or("sensor de atributos ausente")?;
        if !self.access.explore
            || action == 0
            || reading.item.get() != query.world
            || reading.source != query.source
            || sensor.context != query.context
            || sensor.encoding != Encoding::BooleanAttributes
        {
            return Err("atributos sem autorização ou vínculo de estado".into());
        }
        let experience = self.observe(reading)?;
        Ok(AttributeFrame {
            query,
            reading,
            record: experience.record,
        })
    }
    fn attribute_values(
        &self,
        frame: &AttributeFrame,
        c: ConsequenceContract,
        session: &Session,
    ) -> Result<Attributes, String> {
        let e = self
            .experiences
            .iter()
            .find(|e| e.record == frame.record)
            .ok_or("atributos não registrados nesta instância")?;
        self.check_consequence_contract(session, c)?;
        if e.reading != frame.reading
            || !c.matches(frame.query)
            || session.pedagogy_identity()? != (c.concept, c.scope)
            || !c
                .scope
                .validity
                .contains(frame.reading.event.logical_time())
            || frame.reading.event.logical_time() > session.logical_time()
        {
            return Err("atributos, contrato ou versão temporal incompatíveis".into());
        }
        Attributes::decode(e.reading.event.payload()).map_err(|e| format!("{e:?}"))
    }
    fn frame_trial(&self, frame: &AttributeFrame, id: u32) -> Result<ActionTrial, String> {
        let t = self
            .action_trials
            .iter()
            .find(|t| t.id == id)
            .ok_or("ação não observada")?;
        let before = self
            .experiences
            .iter()
            .position(|e| e.record == t.before)
            .ok_or("observação anterior ausente")?;
        // Adjacência impede atribuir retrospectivamente outro estado à ação.
        if before == 0
            || self.experiences[before - 1].record != frame.record
            || !frame.query.matches_trial(t)
        {
            return Err("atributos não precedem esta intervenção".into());
        }
        Ok(t.clone())
    }
    /// Um episódio MP09 existente; no máximo duas experiências (seis leituras
    /// MP05). Escolha informativa, diagnóstico e revisão permanecem no currículo.
    pub fn teach_consequences(
        &mut self,
        session: &mut Session,
        c: ConsequenceContract,
        samples: &[(AttributeFrame, u32, EngramId, u32)],
        assessment: PedagogyAssessment,
        permissions: PedagogyPermissions,
    ) -> Result<ConsequenceTraining, String> {
        if !self.access.learn_concepts
            || !self.access.read_knowledge
            || self.conceptual_attempted
            || samples.is_empty()
            || samples.len() > 2
            || permissions.evaluation != Some(c.scope)
            || permissions.presentation != Some(c.scope)
            || samples
                .iter()
                .any(|s| !permissions.admissions.contains(&s.2))
        {
            return Err("ensino de consequências negado, consumido ou excessivo".into());
        }
        let mut lessons = vec![];
        for (frame, trial, id, receipt) in samples {
            let attributes = self.attribute_values(frame, c, session)?;
            let t = self.frame_trial(frame, *trial)?;
            let after = self
                .experiences
                .iter()
                .find(|e| e.record == t.after)
                .ok_or("consequência ausente")?;
            if after.reading.event.logical_time() > session.logical_time()
                || !c
                    .scope
                    .validity
                    .contains(after.reading.event.logical_time())
            {
                return Err("consequência fora da validade da experiência".into());
            }
            lessons.push(Lesson {
                id: *id,
                receipt: *receipt,
                exemplar: NodeId::new(*receipt).map_err(|e| format!("{e:?}"))?,
                attributes,
                label: Label::from_bool(t.to == c.target),
                source: t.source,
                evidence: after.reading.evidence,
            });
        }
        let plan = TeachingPlan::new(c.concept, c.scope, &lessons)?;
        session.pedagogy_capacity(lessons.len())?;
        self.conceptual_attempted = true;
        self.consequence_contract = Some(c);
        self.cognitive_resources.reserve(5, 0, 0);
        let before = session.pedagogy_events();
        let report = curriculum::teach_concept(session, plan, assessment, permissions)?;
        self.cognitive_resources.report(
            report.initial.len() + report.posterior.len(),
            0,
            0,
            session.pedagogy_events() - before,
        );
        Ok(ConsequenceTraining {
            contract: c,
            trials: samples.iter().map(|s| s.1).collect(),
            pedagogy: report,
        })
    }
    /// Compartilha a reserva de consulta+verificação com a porta F3-01; não cria
    /// um novo orçamento cumulativo nem usa a tabela privada do ambiente.
    pub fn forecast_consequence(
        &mut self,
        session: &Session,
        c: ConsequenceContract,
        frame: &AttributeFrame,
        episode: EpisodeId,
    ) -> Result<ConsequenceForecast, String> {
        self.forecast_using(session, c, frame, episode, &self.action_trials.clone())
    }
    pub(super) fn forecast_using(
        &mut self,
        session: &Session,
        c: ConsequenceContract,
        frame: &AttributeFrame,
        episode: EpisodeId,
        trials: &[ActionTrial],
    ) -> Result<ConsequenceForecast, String> {
        if !self.access.read_knowledge || self.verification_attempted {
            return Err("previsão negada ou consulta/verificação consumida".into());
        }
        if self.experiences.last().map(|e| e.record) != Some(frame.record) {
            return Err("previsão exige atributos atuais, antes da ação".into());
        }
        let attributes = self.attribute_values(frame, c, session)?;
        let transitions = self.predict_using(frame.query, trials)?;
        if transitions.status == PredictionStatus::ResourceLimited {
            return Err("orçamento do modelo de transições esgotado".into());
        }
        self.verification_attempted = true;
        self.consequence_contract = Some(c);
        self.cognitive_resources.reserve(2, 2, 64);
        let report = session.pedagogy_query(episode, attributes.into())?;
        // A segunda consulta fica reservada até a conferência, inclusive falha.
        self.cognitive_resources.conceptual_queries += 1;
        let forecast = ConsequenceForecast {
            contract: c,
            frame: frame.clone(),
            report,
            transitions,
        };
        self.consequence_pending = Some(forecast.clone());
        Ok(forecast)
    }
    /// Adaptador de execução, sem nova política de ensino/planejamento. O
    /// chamador fornece uma concessão fresca; conflitos suspendem a aplicação.
    pub fn execute_forecast(
        &mut self,
        session: &Session,
        world: &mut World,
        forecast: ConsequenceForecast,
        request: &Intervention,
    ) -> Result<ConsequenceReview, String> {
        if self.consequence_pending.as_ref() != Some(&forecast)
            || world.attribute_query(request.permit.action) != forecast.frame.query
            || session.pedagogy_version()?
                != *forecast.report.version.as_ref().ok_or("modelo ausente")?
            || forecast.transitions.status == PredictionStatus::Conflicting
            || forecast.transitions.outcomes.iter().any(|outcome| {
                matches!(forecast.report.decision, ConceptDecision::Supported(label)
                    if label != Label::from_bool(outcome.to == forecast.contract.target))
            })
        {
            return Err("aplicação suspensa: conflito, versão ou objetivo incompatível".into());
        }
        let trial = self.intervene(world, request)?;
        self.review_consequence(session, forecast, trial.id)
    }
    pub fn review_consequence(
        &mut self,
        session: &Session,
        forecast: ConsequenceForecast,
        trial: u32,
    ) -> Result<ConsequenceReview, String> {
        if !self.access.read_knowledge || self.consequence_pending.as_ref() != Some(&forecast) {
            return Err("conferência sem reserva ou já consumida".into());
        }
        // Consome antes de validar: adulteração/falha não devolve autoridade.
        self.consequence_pending.take();
        self.cognitive_resources.operations_without_report -= 1;
        let attrs = self.attribute_values(&forecast.frame, forecast.contract, session)?;
        let current = session.pedagogy_query(forecast.report.episode, attrs.into())?;
        self.cognitive_resources.conceptual_queries += 1;
        if current != forecast.report {
            return Err("relatório conceitual obsoleto ou incompatível".into());
        }
        let actual_trial = self.frame_trial(&forecast.frame, trial)?;
        let after = self
            .experiences
            .iter()
            .find(|e| e.record == actual_trial.after)
            .ok_or("consequência ausente")?;
        if after.reading.event.logical_time() > session.logical_time()
            || !forecast
                .contract
                .scope
                .validity
                .contains(after.reading.event.logical_time())
        {
            return Err("consequência ainda não elegível no tempo da sessão".into());
        }
        let actual = Label::from_bool(actual_trial.to == forecast.contract.target);
        let agreement = match forecast.report.decision {
            ConceptDecision::Supported(label) => Some(label == actual),
            ConceptDecision::Abstain(_) => None,
        };
        let transition_agreement = (forecast.transitions.status == PredictionStatus::Observed)
            .then(|| {
                forecast
                    .transitions
                    .outcomes
                    .iter()
                    .any(|o| o.to == actual_trial.to)
            });
        let review_required = agreement != Some(true)
            || transition_agreement == Some(false)
            || forecast.transitions.status == PredictionStatus::Conflicting;
        Ok(ConsequenceReview {
            forecast,
            trial: actual_trial,
            actual,
            agreement,
            transition_agreement,
            review_required,
        })
    }
}
impl TransitionQuery {
    fn matches_trial(self, t: &ActionTrial) -> bool {
        self.world == t.world
            && self.revision == t.revision
            && self.context == t.context
            && self.source == t.source
            && self.from == t.from
            && self.action == t.action
    }
}
