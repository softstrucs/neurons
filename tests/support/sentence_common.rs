use neurons::languages::sentences::{Gender, SentenceLexicon, SentenceOutcome};
use neurons::languages::Direction;
#[derive(Clone, Copy)]
pub struct Noun {
    pub pt: &'static str,
    pub en: &'static str,
    pub gender: Gender,
}
#[derive(Clone, Copy)]
pub struct Adj {
    pub m: &'static str,
    pub f: &'static str,
    pub en: &'static str,
}
pub const NOUNS: [Noun; 12] = [
    Noun {
        pt: "gato",
        en: "cat",
        gender: Gender::Masculine,
    },
    Noun {
        pt: "cachorro",
        en: "dog",
        gender: Gender::Masculine,
    },
    Noun {
        pt: "casa",
        en: "house",
        gender: Gender::Feminine,
    },
    Noun {
        pt: "mesa",
        en: "table",
        gender: Gender::Feminine,
    },
    Noun {
        pt: "barco",
        en: "boat",
        gender: Gender::Masculine,
    },
    Noun {
        pt: "jardim",
        en: "garden",
        gender: Gender::Masculine,
    },
    Noun {
        pt: "trem",
        en: "train",
        gender: Gender::Masculine,
    },
    Noun {
        pt: "casaco",
        en: "coat",
        gender: Gender::Masculine,
    },
    Noun {
        pt: "janela",
        en: "window",
        gender: Gender::Feminine,
    },
    Noun {
        pt: "árvore",
        en: "tree",
        gender: Gender::Feminine,
    },
    Noun {
        pt: "pedra",
        en: "stone",
        gender: Gender::Feminine,
    },
    Noun {
        pt: "estrela",
        en: "star",
        gender: Gender::Feminine,
    },
];
pub const ADJS: [Adj; 3] = [
    Adj {
        m: "branco",
        f: "branca",
        en: "white",
    },
    Adj {
        m: "preto",
        f: "preta",
        en: "black",
    },
    Adj {
        m: "novo",
        f: "nova",
        en: "new",
    },
];
pub fn build(nouns: &[Noun], adjs: &[Adj], rule: bool) -> SentenceLexicon {
    let mut x = SentenceLexicon::new(rule);
    for n in nouns {
        x.add_noun(n.pt, n.en, n.gender).unwrap();
    }
    for a in adjs {
        x.add_adjective(a.m, a.f, a.en).unwrap();
    }
    x
}
pub fn pair(n: Noun, a: Adj) -> (String, String) {
    (
        format!(
            "{} {}",
            n.pt,
            if n.gender == Gender::Masculine {
                a.m
            } else {
                a.f
            }
        ),
        format!("{} {}", a.en, n.en),
    )
}
#[derive(Debug, PartialEq, Eq)]
pub enum Simple {
    Supported(String),
    Unknown,
    Ambiguous,
    Conflict,
    OutOfGrammar,
    LimitExceeded,
    Inconclusive,
}
pub fn simple(out: SentenceOutcome) -> Simple {
    match out {
        SentenceOutcome::Supported { text, .. } => Simple::Supported(text),
        SentenceOutcome::Unknown => Simple::Unknown,
        SentenceOutcome::Ambiguous => Simple::Ambiguous,
        SentenceOutcome::Conflict => Simple::Conflict,
        SentenceOutcome::OutOfGrammar => Simple::OutOfGrammar,
        SentenceOutcome::LimitExceeded => Simple::LimitExceeded,
        SentenceOutcome::Inconclusive => Simple::Inconclusive,
    }
}
/// Controle independente com os mesmos fatos, categorias e regra gramatical.
pub fn symbolic(
    nouns: &[Noun],
    adjs: &[Adj],
    direction: Direction,
    input: &str,
    rule: bool,
    conflict: Option<(&str, &str)>,
) -> Simple {
    if input.len() > 256 || input.split_whitespace().count() > 12 {
        return Simple::LimitExceeded;
    }
    if !rule {
        return Simple::Inconclusive;
    }
    let words: Vec<_> = input.split_whitespace().collect();
    if words.len() != 2 {
        return Simple::OutOfGrammar;
    }
    let (nw, aw) = match direction {
        Direction::PtToEn => (words[0], words[1]),
        Direction::EnToPt => (words[1], words[0]),
    };
    let known = |w: &str| {
        nouns.iter().any(|n| {
            if direction == Direction::PtToEn {
                n.pt == w
            } else {
                n.en == w
            }
        }) || adjs.iter().any(|a| {
            if direction == Direction::PtToEn {
                a.m == w || a.f == w
            } else {
                a.en == w
            }
        })
    };
    let ns: Vec<_> = nouns
        .iter()
        .filter(|n| {
            if direction == Direction::PtToEn {
                n.pt == nw
            } else {
                n.en == nw
            }
        })
        .collect();
    if ns.is_empty() {
        return if known(nw) {
            Simple::OutOfGrammar
        } else {
            Simple::Unknown
        };
    }
    if ns.len() > 1 {
        return Simple::Ambiguous;
    }
    let n = ns[0];
    let as_: Vec<_> = adjs
        .iter()
        .filter(|a| {
            if direction == Direction::PtToEn {
                if n.gender == Gender::Masculine {
                    a.m == aw
                } else {
                    a.f == aw
                }
            } else {
                a.en == aw
            }
        })
        .collect();
    if as_.is_empty() {
        return if known(aw) {
            Simple::OutOfGrammar
        } else {
            Simple::Unknown
        };
    }
    if as_.len() > 1 {
        return Simple::Ambiguous;
    }
    if conflict.is_some_and(|(category, key)| {
        (category == "noun" && n.pt == key) || (category == "adjective" && as_[0].en == key)
    }) {
        return Simple::Conflict;
    }
    let (pt, en) = pair(*n, *as_[0]);
    Simple::Supported(if direction == Direction::PtToEn {
        en
    } else {
        pt
    })
}
