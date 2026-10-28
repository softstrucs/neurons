//! Contratos compactos para comunicação por eventos.

/// Identificador inválido fornecido pelo chamador.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeIdError;

/// Identifica um destino ou uma origem interna. O valor zero é reservado.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId(u32);

impl NodeId {
    pub fn new(value: u32) -> Result<Self, NodeIdError> {
        if value == 0 {
            return Err(NodeIdError);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Categoria estrutural do evento; não define comportamento do destino.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum EventKind {
    Stimulus,
    Signal,
    Control,
}

/// Evento pequeno, sem texto livre ou referência a objetos externos.
/// `origin = None` representa um estímulo externo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Event {
    origin: Option<NodeId>,
    destination: NodeId,
    kind: EventKind,
    payload: i32,
    logical_time: u64,
}

impl Event {
    pub const fn new(
        origin: Option<NodeId>,
        destination: NodeId,
        kind: EventKind,
        payload: i32,
        logical_time: u64,
    ) -> Self {
        Self {
            origin,
            destination,
            kind,
            payload,
            logical_time,
        }
    }

    pub const fn origin(self) -> Option<NodeId> {
        self.origin
    }

    pub const fn destination(self) -> NodeId {
        self.destination
    }

    pub const fn kind(self) -> EventKind {
        self.kind
    }

    pub const fn payload(self) -> i32 {
        self.payload
    }

    pub const fn logical_time(self) -> u64 {
        self.logical_time
    }
}
