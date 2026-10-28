//! Dados finais congelados após validação do controle; fatos fornecidos manualmente.
//! O avaliador usa os pares como oráculo após ambas as decisões; não é teste de significado adquirido.
pub const PAIRS: [(&str, &str); 16] = [
    ("céu", "sky"),
    ("mar", "sea"),
    ("areia", "sand"),
    ("nuvem", "cloud"),
    ("chuva", "rain"),
    ("vento", "wind"),
    ("neve", "snow"),
    ("sol", "sun"),
    ("lua", "moon"),
    ("estrela", "star"),
    ("fogo", "fire"),
    ("terra", "earth"),
    ("madeira", "wood"),
    ("ferro", "iron"),
    ("vidro", "glass"),
    ("papel", "paper"),
];
pub const MISSING_PT: [&str; 4] = [
    "faltante alfa",
    "faltante beta",
    "faltante gama",
    "faltante delta",
];
pub const MISSING_EN: [&str; 4] = [
    "missing alpha",
    "missing beta",
    "missing gamma",
    "missing delta",
];
// Não contêm espaços: A01.2 aceita uma forma por consulta.
pub const ABSENT_PT: [&str; 4] = [
    "faltante_alfa",
    "faltante_beta",
    "faltante_gama",
    "faltante_delta",
];
pub const ABSENT_EN: [&str; 4] = [
    "missing_alpha",
    "missing_beta",
    "missing_gamma",
    "missing_delta",
];
pub const EXTRA_EN: [&str; 2] = ["alternative_alpha", "alternative_beta"];
pub const EXTRA_PT: [&str; 2] = ["alternativa_gama", "alternativa_delta"];
