//! Fila limitada e despacho determinístico, sem dependência de componentes neurais.

use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeSet, BinaryHeap};

use crate::event::{Event, NodeId};

/// Falhas de configuração ou agendamento. Eventos rejeitados não alteram a fila.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineError {
    ZeroQueueCapacity,
    ZeroCycleLimit,
    UnknownDestination(NodeId),
    PastLogicalTime { attempted: u64, current: u64 },
    QueueFull,
    SequenceExhausted,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EngineMetrics {
    pub enqueued: u64,
    pub dispatched: u64,
    pub rejected: u64,
    pub peak_queue_len: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CycleReport {
    pub processed: usize,
    pub remaining: usize,
    pub limit_reached: bool,
}

#[derive(Clone, Copy, Debug)]
struct QueuedEvent {
    event: Event,
    sequence: u64,
}

impl PartialEq for QueuedEvent {
    fn eq(&self, other: &Self) -> bool {
        self.event.logical_time() == other.event.logical_time() && self.sequence == other.sequence
    }
}

impl Eq for QueuedEvent {}

impl PartialOrd for QueuedEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.event.logical_time(), self.sequence)
            .cmp(&(other.event.logical_time(), other.sequence))
    }
}

/// Agenda eventos por `(tempo lógico, sequência de entrada)`.
///
/// A fila rejeita o evento novo quando atinge a capacidade. Destinos devem ser
/// registrados antes do agendamento. O despachante decide o comportamento de
/// cada destino e pode agendar novos eventos, inclusive no mesmo tempo lógico.
/// Eventos no passado são rejeitados depois do primeiro despacho.
pub struct Engine {
    queue: BinaryHeap<Reverse<QueuedEvent>>,
    destinations: BTreeSet<NodeId>,
    queue_capacity: usize,
    max_events_per_cycle: usize,
    next_sequence: u64,
    current_time: Option<u64>,
    metrics: EngineMetrics,
}

/// Acesso restrito do despachante à fila; impede ciclos aninhados.
pub struct EventSink<'a> {
    engine: &'a mut Engine,
}

impl EventSink<'_> {
    pub fn enqueue(&mut self, event: Event) -> Result<(), EngineError> {
        self.engine.enqueue(event)
    }
}

impl Engine {
    pub fn new(queue_capacity: usize, max_events_per_cycle: usize) -> Result<Self, EngineError> {
        if queue_capacity == 0 {
            return Err(EngineError::ZeroQueueCapacity);
        }
        if max_events_per_cycle == 0 {
            return Err(EngineError::ZeroCycleLimit);
        }
        Ok(Self {
            queue: BinaryHeap::new(),
            destinations: BTreeSet::new(),
            queue_capacity,
            max_events_per_cycle,
            next_sequence: 0,
            current_time: None,
            metrics: EngineMetrics::default(),
        })
    }

    /// Registra um endereço de despacho. Uma repetição retorna `false`.
    pub fn register_destination(&mut self, id: NodeId) -> bool {
        self.destinations.insert(id)
    }

    /// Agenda um evento. A sequência é atribuída apenas após a validação.
    pub fn enqueue(&mut self, event: Event) -> Result<(), EngineError> {
        let error = if !self.destinations.contains(&event.destination()) {
            Some(EngineError::UnknownDestination(event.destination()))
        } else if let Some(current) = self.current_time {
            (event.logical_time() < current).then_some(EngineError::PastLogicalTime {
                attempted: event.logical_time(),
                current,
            })
        } else {
            None
        };

        if let Some(error) = error {
            self.metrics.rejected = self.metrics.rejected.saturating_add(1);
            return Err(error);
        }
        if self.queue.len() >= self.queue_capacity {
            self.metrics.rejected = self.metrics.rejected.saturating_add(1);
            return Err(EngineError::QueueFull);
        }
        let sequence = match self.next_sequence.checked_add(1) {
            Some(next) => {
                let sequence = self.next_sequence;
                self.next_sequence = next;
                sequence
            }
            None => {
                self.metrics.rejected = self.metrics.rejected.saturating_add(1);
                return Err(EngineError::SequenceExhausted);
            }
        };
        self.queue.push(Reverse(QueuedEvent { event, sequence }));
        self.metrics.enqueued = self.metrics.enqueued.saturating_add(1);
        self.metrics.peak_queue_len = self.metrics.peak_queue_len.max(self.queue.len());
        Ok(())
    }

    /// Processa no máximo o limite configurado. O callback executa uma vez por
    /// evento e pode chamar `enqueue`; ele deve tratar eventual erro retornado.
    pub fn run_cycle(
        &mut self,
        mut dispatch: impl FnMut(Event, &mut EventSink<'_>),
    ) -> CycleReport {
        let mut processed = 0;
        while processed < self.max_events_per_cycle {
            let Some(Reverse(queued)) = self.queue.pop() else {
                break;
            };
            self.current_time = Some(queued.event.logical_time());
            self.metrics.dispatched = self.metrics.dispatched.saturating_add(1);
            processed += 1;
            dispatch(queued.event, &mut EventSink { engine: self });
        }
        CycleReport {
            processed,
            remaining: self.queue.len(),
            limit_reached: processed == self.max_events_per_cycle && !self.queue.is_empty(),
        }
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn metrics(&self) -> EngineMetrics {
        self.metrics
    }
}

/// Checkpoints locais opcionais do motor e da memória associativa.
#[path = "persistence.rs"]
pub mod persistence;
