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

    /// Retorna a identidade interna de uma entrega aceita.
    pub(crate) fn enqueue_identified(&mut self, event: Event) -> Result<u64, EngineError> {
        self.engine.enqueue_identified(event)
    }

    /// Agenda um lote de eventos de maneira atomica.
    pub fn enqueue_batch(&mut self, events: &[Event]) -> Result<(), EngineError> {
        self.engine.enqueue_batch(events)
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
        self.enqueue_identified(event).map(|_| ())
    }

    /// Agenda um evento e retorna sua sequência somente após aceitação.
    pub(crate) fn enqueue_identified(&mut self, event: Event) -> Result<u64, EngineError> {
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
        Ok(sequence)
    }

    /// Agenda todos os eventos ou nenhum.
    ///
    /// Lote vazio: sucesso sem modificacoes.
    /// Lote rejeitado: incrementa rejected uma unica vez.
    /// Nao altera os formatos de persistencia existentes.
    pub fn enqueue_batch(&mut self, events: &[Event]) -> Result<(), EngineError> {
        if events.is_empty() {
            return Ok(());
        }

        let validation = (|| {
            for event in events {
                if !self.destinations.contains(&event.destination()) {
                    return Err(EngineError::UnknownDestination(event.destination()));
                }

                if let Some(current) = self.current_time {
                    if event.logical_time() < current {
                        return Err(EngineError::PastLogicalTime {
                            attempted: event.logical_time(),
                            current,
                        });
                    }
                }
            }

            if events.len() > self.queue_capacity - self.queue.len() {
                return Err(EngineError::QueueFull);
            }

            let count = u64::try_from(events.len()).map_err(|_| EngineError::SequenceExhausted)?;

            self.next_sequence
                .checked_add(count)
                .ok_or(EngineError::SequenceExhausted)?;

            Ok(())
        })();

        if let Err(error) = validation {
            self.metrics.rejected = self.metrics.rejected.saturating_add(1);
            return Err(error);
        }

        // A validacao completa antecede qualquer insercao.
        for event in events {
            let sequence = self.next_sequence;
            self.next_sequence += 1;

            self.queue.push(Reverse(QueuedEvent {
                event: *event,
                sequence,
            }));
        }

        self.metrics.enqueued += events.len() as u64;
        self.metrics.peak_queue_len = self.metrics.peak_queue_len.max(self.queue.len());

        Ok(())
    }

    /// Processa no máximo o limite configurado. O callback executa uma vez por
    /// evento e pode chamar `enqueue`; ele deve tratar eventual erro retornado.
    pub fn run_cycle(
        &mut self,
        mut dispatch: impl FnMut(Event, &mut EventSink<'_>),
    ) -> CycleReport {
        self.run_cycle_identified(|_, event, sink| dispatch(event, sink))
    }

    /// Despacho interno com identidade estável por evento.
    pub(crate) fn run_cycle_identified(
        &mut self,
        mut dispatch: impl FnMut(u64, Event, &mut EventSink<'_>),
    ) -> CycleReport {
        let mut processed = 0;
        while processed < self.max_events_per_cycle {
            let Some(Reverse(queued)) = self.queue.pop() else {
                break;
            };
            self.current_time = Some(queued.event.logical_time());
            self.metrics.dispatched = self.metrics.dispatched.saturating_add(1);
            processed += 1;
            dispatch(
                queued.sequence,
                queued.event,
                &mut EventSink { engine: self },
            );
        }
        CycleReport {
            processed,
            remaining: self.queue.len(),
            limit_reached: processed == self.max_events_per_cycle && !self.queue.is_empty(),
        }
    }

    /// Eventos pendentes em ordem deterministica, com suas identidades.
    /// Consulta interna, sem modificar a fila.
    pub(crate) fn pending_events_identified(&self) -> Vec<(u64, Event)> {
        let mut events: Vec<_> = self
            .queue
            .iter()
            .map(|entry| (entry.0.sequence, entry.0.event))
            .collect();

        events.sort_by_key(|(sequence, event)| (event.logical_time(), *sequence));

        events
    }

    /// Destinos registrados no motor, em ordem deterministica.
    pub(crate) fn registered_destinations(&self) -> Vec<NodeId> {
        self.destinations.iter().copied().collect()
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

impl Engine {
    /// Instante do último evento despachado; leitura sem avançar o motor.
    pub const fn logical_time(&self) -> Option<u64> {
        self.current_time
    }
}

#[cfg(test)]
mod den_int_007_sequence_tests {
    use super::*;
    use crate::event::EventKind;

    #[test]
    fn sequence_exhaustion_rejects_entire_batch() {
        let destination = NodeId::new(1).unwrap();
        let mut engine = Engine::new(4, 4).unwrap();
        engine.register_destination(destination);

        // Estado artificial para exercitar o limite de u64.
        engine.next_sequence = u64::MAX - 1;
        engine.metrics.enqueued = u64::MAX - 1;
        engine.metrics.dispatched = u64::MAX - 1;

        let event = Event::new(None, destination, EventKind::Stimulus, 7, 1);

        let before_queue = engine.queue_len();
        let before_sequence = engine.next_sequence;
        let before_metrics = engine.metrics();

        assert_eq!(
            engine.enqueue_batch(&[event, event]),
            Err(EngineError::SequenceExhausted)
        );

        assert_eq!(engine.queue_len(), before_queue);
        assert_eq!(engine.next_sequence, before_sequence);
        assert_eq!(engine.metrics().enqueued, before_metrics.enqueued);
        assert_eq!(engine.metrics().dispatched, before_metrics.dispatched);
        assert_eq!(engine.metrics().rejected, before_metrics.rejected + 1);

        // Uma insercao individual ainda cabe no limite.
        assert_eq!(engine.enqueue(event), Ok(()));
        assert_eq!(engine.next_sequence, u64::MAX);

        // Nenhuma sequencia adicional pode ser atribuida.
        assert_eq!(engine.enqueue(event), Err(EngineError::SequenceExhausted));
        assert_eq!(engine.queue_len(), 1);
    }
}

#[cfg(test)]
mod den_int_008_d42a_identity_tests {
    use super::*;
    use crate::event::EventKind;

    fn node(value: u32) -> NodeId {
        NodeId::new(value).unwrap()
    }

    fn event(destination: NodeId, time: u64) -> Event {
        Event::new(None, destination, EventKind::Stimulus, 1, time)
    }

    #[test]
    fn accepted_events_keep_identity_across_cycles() {
        let destination = node(1);
        let mut engine = Engine::new(4, 1).unwrap();
        engine.register_destination(destination);

        let first = engine.enqueue_identified(event(destination, 2)).unwrap();
        let second = engine.enqueue_identified(event(destination, 1)).unwrap();

        assert_ne!(first, second);

        let mut observed = Vec::new();

        let report = engine.run_cycle_identified(|sequence, event, _| {
            observed.push((sequence, event.logical_time()));
        });

        assert_eq!(report.processed, 1);
        assert_eq!(report.remaining, 1);
        assert_eq!(observed, vec![(second, 1)]);

        engine.run_cycle_identified(|sequence, event, _| {
            observed.push((sequence, event.logical_time()));
        });

        assert_eq!(observed, vec![(second, 1), (first, 2)]);
    }

    #[test]
    fn rejected_insertion_does_not_consume_identity() {
        let destination = node(1);
        let mut engine = Engine::new(1, 1).unwrap();
        engine.register_destination(destination);

        let first = engine.enqueue_identified(event(destination, 0)).unwrap();

        assert_eq!(
            engine.enqueue_identified(event(destination, 1)),
            Err(EngineError::QueueFull)
        );

        engine.run_cycle(|_, _| {});

        let second = engine.enqueue_identified(event(destination, 1)).unwrap();

        assert_eq!(second, first + 1);
    }

    #[test]
    fn sink_identity_matches_dispatched_identity() {
        let destination = node(1);
        let mut engine = Engine::new(3, 3).unwrap();
        engine.register_destination(destination);

        let first = engine.enqueue_identified(event(destination, 0)).unwrap();

        let mut created = None;
        let mut observed = Vec::new();

        engine.run_cycle_identified(|sequence, event, sink| {
            observed.push(sequence);

            if event.logical_time() == 0 {
                created = Some(sink.enqueue_identified(event_with_time(event, 1)).unwrap());
            }
        });

        assert_eq!(observed, vec![first, created.unwrap()]);
    }

    fn event_with_time(event: Event, time: u64) -> Event {
        Event::new(
            event.origin(),
            event.destination(),
            event.kind(),
            event.payload(),
            time,
        )
    }

    #[test]
    fn public_enqueue_and_batch_preserve_sequence_order() {
        let destination = node(1);
        let mut engine = Engine::new(4, 4).unwrap();
        engine.register_destination(destination);

        engine.enqueue(event(destination, 1)).unwrap();
        engine
            .enqueue_batch(&[event(destination, 1), event(destination, 1)])
            .unwrap();

        let next = engine.enqueue_identified(event(destination, 1)).unwrap();

        let mut observed = Vec::new();
        engine.run_cycle_identified(|sequence, _, _| {
            observed.push(sequence);
        });

        assert_eq!(observed.len(), 4);
        assert_eq!(observed[3], next);
        assert!(observed.windows(2).all(|pair| pair[0] < pair[1]));
    }
}
