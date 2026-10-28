use neurons::languages::Lexicon;
// Produzido manualmente para o experimento; pares isolados em sentido comum,
// sem fonte externa, revisão por especialista ou alegação de cobertura geral.
pub const PAIRS: [(&str, &str); 24] = [
    ("casa", "house"),
    ("gato", "cat"),
    ("cachorro", "dog"),
    ("livro", "book"),
    ("mesa", "table"),
    ("cadeira", "chair"),
    ("porta", "door"),
    ("janela", "window"),
    ("água", "water"),
    ("pão", "bread"),
    ("leite", "milk"),
    ("maçã", "apple"),
    ("sol", "sun"),
    ("lua", "moon"),
    ("flor", "flower"),
    ("árvore", "tree"),
    ("vermelho", "red"),
    ("azul", "blue"),
    ("verde", "green"),
    ("peixe", "fish"),
    ("pássaro", "bird"),
    ("cavalo", "horse"),
    ("hotel", "hotel"),
    ("animal", "animal"),
];
pub fn lexicon() -> Lexicon {
    let mut l = Lexicon::new();
    for (p, e) in PAIRS {
        l.add_pair(p, e).unwrap();
    }
    l
}
