//! Controle de teste: recebe fatos, não recebe destino esperado ou oráculo.
use neurons::languages::{ConceptLexicon, Language};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fact {
    pub language: Language,
    pub form: &'static str,
    pub concept: u32,
    pub positive: bool,
    pub id: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BlindDecision {
    Supported {
        text: &'static str,
        concept: u32,
        source_evidence: u32,
        target_evidence: u32,
    },
    Unknown,
    Ambiguous,
    Conflict,
    LimitExceeded,
    Inconclusive,
    Invalid,
}

/// Relações são todas forma→conceito. Conflitos de origem são examinados
/// antes da enumeração de candidatos; informação e orçamento são explícitos.
pub fn select(
    facts: &[Fact],
    source_language: Language,
    input: &str,
    target_language: Language,
    budget: usize,
) -> BlindDecision {
    if source_language == target_language
        || budget > 2
        || input.is_empty()
        || input.len() > 256
        || input.chars().any(|c| c.is_whitespace() || c.is_control())
    {
        return BlindDecision::Invalid;
    }
    if facts.len() > 256 {
        return BlindDecision::LimitExceeded;
    }
    if budget < 2 {
        return BlindDecision::Inconclusive;
    }
    let sources: Vec<_> = facts
        .iter()
        .filter(|f| f.language == source_language && f.form == input && f.positive)
        .collect();
    if sources.iter().any(|p| {
        facts.iter().any(|n| {
            !n.positive && n.language == p.language && n.form == p.form && n.concept == p.concept
        })
    }) {
        return BlindDecision::Conflict;
    }
    let mut candidates = Vec::new();
    for first in sources {
        for second in facts
            .iter()
            .filter(|f| f.language == target_language && f.concept == first.concept && f.positive)
        {
            if candidates.len() == 8 {
                return BlindDecision::LimitExceeded;
            }
            if facts.iter().any(|n| {
                !n.positive
                    && n.language == second.language
                    && n.form == second.form
                    && n.concept == second.concept
            }) {
                return BlindDecision::Conflict;
            }
            candidates.push((first, second));
        }
    }
    match candidates.as_slice() {
        [] => BlindDecision::Unknown,
        [(first, second)] => BlindDecision::Supported {
            text: second.form,
            concept: first.concept,
            source_evidence: first.id,
            target_evidence: second.id,
        },
        _ => BlindDecision::Ambiguous,
    }
}
/// Monta o mecanismo avaliado e o controle a partir da mesma lista de fatos.
pub fn neurons(facts: &[Fact]) -> ConceptLexicon {
    let mut lex = ConceptLexicon::new();
    let mut seen = Vec::new();
    for f in facts {
        if !seen.contains(&f.concept) {
            lex.add_concept(f.concept, &format!("sentido {}", f.concept))
                .unwrap();
            seen.push(f.concept);
        }
    }
    for (index, f) in facts.iter().enumerate() {
        assert_eq!(f.id, index as u32 + 1);
        lex.associate(f.language, f.form, f.concept, f.positive)
            .unwrap();
    }
    lex
}
pub fn fact(id: u32, language: Language, form: &'static str, concept: u32, positive: bool) -> Fact {
    Fact {
        id,
        language,
        form,
        concept,
        positive,
    }
}
