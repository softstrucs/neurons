//! Configuração privada do executor/avaliador. Nunca importada pelo aprendiz.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Category {
    Necessary,
    Dispensable,
    Unknown,
    Changed,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Condition {
    pub key: u16,
    pub category: Category,
    pub backup: i32,
}
use Category::*;
pub const PANEL: [Condition; 16] = [
    Condition {
        key: 47,
        category: Necessary,
        backup: 1,
    },
    Condition {
        key: 12,
        category: Unknown,
        backup: 1,
    },
    Condition {
        key: 83,
        category: Dispensable,
        backup: 2,
    },
    Condition {
        key: 26,
        category: Changed,
        backup: 1,
    },
    Condition {
        key: 59,
        category: Dispensable,
        backup: 2,
    },
    Condition {
        key: 94,
        category: Necessary,
        backup: 1,
    },
    Condition {
        key: 31,
        category: Changed,
        backup: 2,
    },
    Condition {
        key: 68,
        category: Unknown,
        backup: 2,
    },
    Condition {
        key: 15,
        category: Changed,
        backup: 1,
    },
    Condition {
        key: 72,
        category: Dispensable,
        backup: 2,
    },
    Condition {
        key: 39,
        category: Unknown,
        backup: 1,
    },
    Condition {
        key: 86,
        category: Necessary,
        backup: 1,
    },
    Condition {
        key: 53,
        category: Necessary,
        backup: 1,
    },
    Condition {
        key: 24,
        category: Changed,
        backup: 2,
    },
    Condition {
        key: 97,
        category: Dispensable,
        backup: 2,
    },
    Condition {
        key: 61,
        category: Unknown,
        backup: 2,
    },
];
