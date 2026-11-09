//! Núcleo da Neurons. Componentes são adicionados conforme cada etapa.

pub mod circuit;
pub mod cognition;
pub mod cognitive_contracts;
pub mod curriculum;
pub mod engine;
pub mod engram;
pub mod event;
pub mod languages;
pub mod learning;
pub mod metacognition;
pub mod neuron;
pub mod personality;
/// Instrumentos M4 separados do candidato; sem executor do painel M5.
pub mod protocol_verification;
pub mod session;
pub mod synapse;
pub use engine::persistence;
pub mod integration;
