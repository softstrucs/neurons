use neurons::languages::contextual::Context;
// Conjunto substituto congelado após descartar consultas que coincidiam com desenvolvimento.
// Dados manuais em sentidos restritos, sem revisão linguística independente.
pub const PAIRS: [(&str, &str, Context); 24] = [
    ("pasta", "briefcase", Context::Finance),
    ("cheque", "check", Context::Finance),
    ("saldo", "balance", Context::Finance),
    ("bolsa", "stock exchange", Context::Finance),
    ("ação", "share", Context::Finance),
    ("conta corrente", "checking account", Context::Finance),
    ("planta", "floor plan", Context::Furniture),
    ("aparador", "sideboard", Context::Furniture),
    ("cômoda", "dresser", Context::Furniture),
    ("prateleira", "shelf", Context::Furniture),
    ("mesa lateral", "side table", Context::Furniture),
    ("cama dobrável", "folding bed", Context::Furniture),
    ("planta", "plant", Context::Nature),
    ("cascata", "waterfall", Context::Nature),
    ("deserto", "desert", Context::Nature),
    ("vulcão", "volcano", Context::Nature),
    ("pôr do sol", "sunset", Context::Nature),
    ("chuva forte", "heavy rain", Context::Nature),
    ("pasta", "folder", Context::Technology),
    ("processador", "processor", Context::Technology),
    ("celular", "cellphone", Context::Technology),
    ("impressora", "printer", Context::Technology),
    ("tecla enter", "enter key", Context::Technology),
    ("tela inicial", "home screen", Context::Technology),
];
