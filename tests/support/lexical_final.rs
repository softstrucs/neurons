use neurons::languages::Lexicon;
// Inventário final independente, elaborado manualmente; sem revisão externa.
pub const PAIRS: [(&str, &str); 24] = [
    ("rua", "street"),
    ("pedra", "stone"),
    ("chuva", "rain"),
    ("neve", "snow"),
    ("estrela", "star"),
    ("folha", "leaf"),
    ("fruta", "fruit"),
    ("queijo", "cheese"),
    ("arroz", "rice"),
    ("ovo", "egg"),
    ("café", "coffee"),
    ("chá", "tea"),
    ("escola", "school"),
    ("cidade", "city"),
    ("jardim", "garden"),
    ("campo", "field"),
    ("pato", "duck"),
    ("abelha", "bee"),
    ("coelho", "rabbit"),
    ("urso", "bear"),
    ("amarelo", "yellow"),
    ("preto", "black"),
    ("rosa", "pink"),
    ("cinza", "gray"),
];
pub fn lexicon() -> Lexicon {
    let mut l = Lexicon::new();
    for (p, e) in PAIRS {
        l.add_pair(p, e).unwrap();
    }
    l
}
