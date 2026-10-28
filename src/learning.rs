//! Regra experimental e explícita de plasticidade sináptica incremental.

use crate::circuit::{Circuit, CircuitError};
use crate::synapse::SynapseId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Feedback {
    Positive,
    Negative,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LearningError {
    InvalidBounds,
    InvalidStep,
    WeightOutOfBounds { weight: i32, min: i32, max: i32 },
    Circuit(CircuitError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WeightUpdate {
    pub before: i32,
    pub after: i32,
    pub feedback: Option<Feedback>,
    pub saturated: bool,
}

impl WeightUpdate {
    pub const fn changed(self) -> bool {
        self.before != self.after
    }
}

/// Soma ou subtrai um passo fixo do peso, limitando o resultado a `[min, max]`.
/// Não avalia a resposta nem escolhe a conexão: o feedback vem do experimento.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlasticityRule {
    min: i32,
    max: i32,
    step: i32,
}

impl PlasticityRule {
    pub fn new(min: i32, max: i32, step: i32) -> Result<Self, LearningError> {
        if min > max {
            return Err(LearningError::InvalidBounds);
        }
        if step <= 0 {
            return Err(LearningError::InvalidStep);
        }
        Ok(Self { min, max, step })
    }

    pub const fn bounds(self) -> (i32, i32) {
        (self.min, self.max)
    }

    pub const fn step(self) -> i32 {
        self.step
    }

    /// Cálculo puro: a mesma entrada sempre produz o mesmo ajuste.
    pub fn adjust(
        self,
        current: i32,
        feedback: Option<Feedback>,
    ) -> Result<WeightUpdate, LearningError> {
        if current < self.min || current > self.max {
            return Err(LearningError::WeightOutOfBounds {
                weight: current,
                min: self.min,
                max: self.max,
            });
        }
        let Some(signal) = feedback else {
            return Ok(WeightUpdate {
                before: current,
                after: current,
                feedback: None,
                saturated: false,
            });
        };

        let candidate = match signal {
            Feedback::Positive => i64::from(current) + i64::from(self.step),
            Feedback::Negative => i64::from(current) - i64::from(self.step),
        };
        let bounded = candidate.clamp(i64::from(self.min), i64::from(self.max));
        Ok(WeightUpdate {
            before: current,
            after: bounded as i32,
            feedback: Some(signal),
            saturated: candidate != bounded,
        })
    }

    /// Aplica o ajuste somente quando o peso realmente muda. Não toca em
    /// estado neuronal, engramas ou fila de eventos.
    pub fn apply(
        self,
        circuit: &mut Circuit,
        synapse: SynapseId,
        feedback: Option<Feedback>,
    ) -> Result<WeightUpdate, LearningError> {
        let current = circuit
            .synapse_weight(synapse)
            .map_err(LearningError::Circuit)?;
        let update = self.adjust(current, feedback)?;
        if update.changed() {
            circuit
                .set_weight(synapse, update.after)
                .map_err(LearningError::Circuit)?;
        }
        Ok(update)
    }
}
