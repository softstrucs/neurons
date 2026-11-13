//! Adaptador transitório: interpreta entradas, encaminha tipos e apresenta resultados.
//! Não diagnostica, seleciona estratégias ou admite conhecimento.
use super::{bits, err, evidence, node, num, polarity, record, source, Session};
use crate::cognition::concepts::{Label, Observation};
use crate::cognitive_contracts::{EpisodeId, Polarity};
use crate::curriculum::{Lesson, PedagogyPermissions};
use crate::integration::composed::{
    Binding, BindingOrigin, Episode, GoalReport, Limits, Permissions, Report, SyntheticRegistry,
    TeachingRequest,
};
use crate::integration::query::PathAuthorization;

pub(super) const HELP: &str = "Composição supervisionada transitória (preparar conhecimento F1 antes do registro):
registro-composto VINCULO ITEM BITS CONCEITO ORIGEM DESTINO CONTEXTO FONTE EVIDENCIA REVISAO
objetivo-composto OBJETIVO VINCULO ITEM BITS CONCEITO ORIGEM DESTINO CONTEXTO FONTE EVIDENCIA
permissoes-compostas CONTEXTO CONCEITUAL(sim|nao) RELACIONAL(sim|nao) COMPOSICAO(sim|nao)
ensino-composto ID RECIBO EXEMPLAR BITS ROTULO(+|-) FONTE EVIDENCIA PRE POS (opcional, uma experiência)
permissoes-ensino-composto CONTEXTO AVALIAR(sim|nao) APRESENTAR(sim|nao) ADMISSAO_ID|-
acompanhar-composto CONSULTAS_CONCEITUAIS CONSULTAS_RELACIONAIS EVENTOS_NEURAIS (máximos8/3/96)
historico-composto
Registro sintético explícito e independente do pedido; não autentica identidade real.
Sem concessões implícitas, dados de prova rotulados ou persistência de supervisor/autoridade transitória.
Após o registro, comandos cognitivos paralelos ficam bloqueados; restaurar limpa a preparação e as concessões.
";

#[derive(Default)]
pub(super) struct Adapter {
    registry: Option<SyntheticRegistry>,
    goal: Option<(EpisodeId, Binding)>,
    permissions: Option<Permissions>,
    lesson: Option<(Lesson, Observation, Observation)>,
    pedagogy: Option<PedagogyPermissions>,
    report: Option<GoalReport>,
}
pub(super) fn handles(op: &str) -> bool {
    matches!(
        op,
        "registro-composto"
            | "objetivo-composto"
            | "permissoes-compostas"
            | "ensino-composto"
            | "permissoes-ensino-composto"
            | "acompanhar-composto"
            | "historico-composto"
    )
}
fn flag(value: &str) -> Result<bool, String> {
    match value {
        "sim" => Ok(true),
        "nao" => Ok(false),
        _ => Err("use sim ou nao".into()),
    }
}
fn observation(value: &str) -> Result<Observation, String> {
    Observation::new(bits(value)?).map_err(err)
}
fn binding(session: &Session, values: &[&str]) -> Result<Binding, String> {
    let [id, item, vector, concept, from, to, context, src, ev] = values else {
        return Err("campos do vínculo composto ausentes".into());
    };
    Ok(Binding {
        id: EpisodeId::new(num(id)?).map_err(err)?,
        item: node(item)?,
        observation: observation(vector)?,
        concept: node(concept)?,
        relational_concept: node(from)?,
        destination: node(to)?,
        scope: session.scope_in(context)?,
        source: source(src)?,
        evidence: evidence(ev)?.ok_or("evidência do vínculo ausente")?,
        origin: BindingOrigin::ExternalSyntheticObservation,
    })
}
impl Adapter {
    pub(super) fn active(&self) -> bool {
        self.registry.is_some()
    }
    pub(super) fn execute(
        &mut self,
        session: &mut Session,
        values: &[&str],
    ) -> Result<String, String> {
        if values == ["historico-composto"] {
            return self
                .report
                .as_ref()
                .map(render)
                .ok_or("sem relatório composto".into());
        }
        if self.report.is_some() {
            return Err(
                "episódio composto encerrado; sem repetição ou autoridade reaproveitada".into(),
            );
        }
        match values {
            ["registro-composto", rest @ ..] if rest.len() == 10 => {
                if self.registry.is_some() {
                    return Err("registro composto já fixado".into());
                }
                let record = binding(session, &rest[..9])?;
                let registry = SyntheticRegistry::new(record, num(rest[9])?)?;
                self.registry = Some(registry);
                Ok("Registro sintético fixado antes do objetivo; nenhuma concessão".into())
            }
            ["objetivo-composto", id, rest @ ..] if rest.len() == 9 => {
                if self.registry.is_none() || self.goal.is_some() {
                    return Err("registro ausente ou objetivo já fixado".into());
                }
                let objective = EpisodeId::new(num(id)?).map_err(err)?;
                let request = binding(session, rest)?;
                self.goal = Some((objective, request));
                Ok("Objetivo composto fixado; conhecimento não alterado".into())
            }
            ["permissoes-compostas", context, concept, relation, composition] => {
                if self.permissions.is_some() {
                    return Err("concessões compostas já fixadas".into());
                }
                let (_, goal) = self.goal.as_ref().ok_or("defina objetivo-composto")?;
                let scope = session.scope_in(context)?;
                let permissions = Permissions {
                    conceptual: flag(concept)?.then_some(scope),
                    relational: flag(relation)?
                        .then_some(PathAuthorization::inclusion_three(scope)),
                    composition: flag(composition)?.then_some(goal.clone()),
                };
                self.permissions = Some(permissions);
                Ok("Três concessões independentes fixadas explicitamente".into())
            }
            ["ensino-composto", id, receipt, exemplar, vector, label, src, ev, pre, post] => {
                if self.goal.is_none() || self.lesson.is_some() {
                    return Err("objetivo ausente ou experiência já fixada".into());
                }
                let lesson = Lesson {
                    id: record(id)?,
                    receipt: num(receipt)?,
                    exemplar: node(exemplar)?,
                    attributes: observation(vector)?.complete().map_err(err)?,
                    label: if polarity(label)? == Polarity::Positive {
                        Label::Positive
                    } else {
                        Label::Negative
                    },
                    source: source(src)?,
                    evidence: evidence(ev)?.ok_or("evidência didática ausente")?,
                };
                if lesson.receipt == 0 {
                    return Err("recibo inválido".into());
                }
                let before = observation(pre)?;
                let after = observation(post)?;
                crate::curriculum::PedagogyAssessment::new(&[before], &[after])?;
                self.lesson = Some((lesson, before, after));
                Ok(
                    "Uma experiência preparada; avaliações sem rótulos; nenhuma apresentação"
                        .into(),
                )
            }
            ["permissoes-ensino-composto", context, evaluation, presentation, admission] => {
                if self.goal.is_none() || self.pedagogy.is_some() {
                    return Err("objetivo ausente ou concessões pedagógicas já fixadas".into());
                }
                let scope = session.scope_in(context)?;
                let permissions = PedagogyPermissions {
                    evaluation: flag(evaluation)?.then_some(scope),
                    presentation: flag(presentation)?.then_some(scope),
                    admissions: if *admission == "-" {
                        vec![]
                    } else {
                        vec![record(admission)?]
                    },
                };
                self.pedagogy = Some(permissions);
                Ok("Concessões pedagógicas independentes fixadas explicitamente".into())
            }
            ["acompanhar-composto", cq, rq, events] => {
                let (objective, goal) = self.goal.as_ref().ok_or("defina objetivo-composto")?;
                let registry = self.registry.as_ref().ok_or("defina registro-composto")?;
                let permissions = self
                    .permissions
                    .as_ref()
                    .ok_or("declare permissoes-compostas")?
                    .clone();
                let teaching = match (&self.lesson, &self.pedagogy) {
                    (Some((lesson, before, after)), Some(permissions)) => Some(TeachingRequest {
                        lesson: *lesson,
                        before: *before,
                        after: *after,
                        permissions: permissions.clone(),
                    }),
                    (None, None) => None,
                    _ => {
                        return Err(
                            "dados e concessões pedagógicas devem ser declarados separadamente"
                                .into(),
                        )
                    }
                };
                let limits = Limits {
                    conceptual_queries: num(cq)? as usize,
                    relational_queries: num(rq)? as usize,
                    neural_events: num(events)? as usize,
                };
                let episode = Episode::with_limits(session, *objective, goal.clone(), limits)?;
                let report = episode.supervise(session, registry, permissions, teaching);
                let text = render(&report);
                self.report = Some(report);
                // Concessões não ficam disponíveis para outra execução.
                self.permissions = None;
                self.pedagogy = None;
                self.lesson = None;
                Ok(text)
            }
            _ => Err("entrada composta inválida ou campos obrigatórios ausentes; use ajuda".into()),
        }
    }
}

/// Projeção de apresentação: não inclui bytes do checkpoint/dependências ou estado
/// não consultado. Dados ausentes por recusa continuam ausentes na saída.
pub(super) fn render(report: &GoalReport) -> String {
    fn query(prefix: &str, report: Option<&Report>) -> String {
        let conceptual = report.and_then(|r| r.conceptual.as_ref());
        let relational = report.and_then(|r| r.relational.as_ref());
        format!("{prefix}.estado={:?}\n{prefix}.conceitual={:?}\n{prefix}.versao={:?}\n{prefix}.evidencias={:?}\n{prefix}.consultados_conceituais={:?}\n{prefix}.prova={:?}\n{prefix}.conflitos={:?}\n{prefix}.recusa={:?}\n{prefix}.revisao_registro={:?}\n",
            report.map(|r| r.status), conceptual.map(|c| c.decision),
            conceptual.and_then(|c| c.version.as_ref()).map(|v| v.number),
            conceptual.map(|c| c.consulted.iter().map(|e| e.metadata.evidence).collect::<Vec<_>>()),
            conceptual.map(|c| &c.consulted),
            relational.and_then(|r| r.proof.as_ref()), relational.and_then(|r| r.review.as_ref()).map(|r| &r.conflicts), relational.and_then(|r| r.refusal),
            report.filter(|r| !r.dependencies.state.is_empty()).map(|r| r.dependencies.registry_revision))
    }
    format!("ObjetivoComposto\nobjetivo={}\nvinculo={:?}\n{}diagnostico={:?}\nestrategia={:?}\ntentativa_iniciada={}\nexperiencia={:?}\nfalha_experiencia={:?}\n{}verificacao={:?}\nresultado={:?}\npermissoes_compostas={:?}\npermissoes_pedagogicas={:?}\nlimites={:?}\nconsumo={:?}\nmotivo={:?}\nFimObjetivoComposto",
        report.objective.get(), report.binding, query("inicial", report.initial.as_ref()),
        report.diagnosis, report.strategy, report.action_started,
        report.action.as_ref().map(|a| &a.lessons), report.action_failure,
        query("reconsulta", report.final_query.as_ref()), report.verification, report.outcome,
        report.permissions, report.teaching.as_ref().map(|t| &t.permissions), report.limits, report.usage, report.reason)
}
