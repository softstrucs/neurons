use neurons::languages::contextual::{Context, ContextLexicon, ContextOutcome};
use neurons::languages::{Direction, Language};

#[derive(Clone, Copy)]
pub struct Fact {
    pub language: Language,
    pub text: &'static str,
    pub context: Context,
    pub concept: u32,
    pub positive: bool,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Simple {
    Supported(String, u32, Context),
    Unknown,
    Ambiguous,
    Conflict,
}
pub fn build(facts: &[Fact]) -> ContextLexicon {
    let mut lex = ContextLexicon::new();
    let mut concepts = Vec::new();
    for f in facts {
        if !concepts.contains(&f.concept) {
            lex.add_concept(f.concept, &format!("sentido fornecido {}", f.concept))
                .unwrap();
            concepts.push(f.concept);
        }
    }
    for f in facts {
        lex.associate(f.language, f.text, f.context, f.concept, f.positive)
            .unwrap();
    }
    lex
}
pub fn from_pairs(pairs: &[(&'static str, &'static str, Context)]) -> Vec<Fact> {
    let mut facts = Vec::new();
    for (index, (pt, en, cx)) in pairs.iter().copied().enumerate() {
        facts.push(Fact {
            language: Language::PtBr,
            text: pt,
            context: cx,
            concept: index as u32 + 1,
            positive: true,
        });
        facts.push(Fact {
            language: Language::En,
            text: en,
            context: cx,
            concept: index as u32 + 1,
            positive: true,
        });
    }
    facts
}
pub fn simplify(out: ContextOutcome) -> Simple {
    match out {
        ContextOutcome::Supported {
            text,
            concept,
            context,
            ..
        } => Simple::Supported(text, concept, context),
        ContextOutcome::Unknown => Simple::Unknown,
        ContextOutcome::Ambiguous => Simple::Ambiguous,
        ContextOutcome::Conflict => Simple::Conflict,
        other => panic!("resultado não esperado {other:?}"),
    }
}
/// Controle independente percorre exatamente os mesmos fatos e contextos.
pub fn symbolic(
    facts: &[Fact],
    direction: Direction,
    text: &str,
    context: Option<Context>,
) -> Simple {
    let (source, target) = match direction {
        Direction::PtToEn => (Language::PtBr, Language::En),
        Direction::EnToPt => (Language::En, Language::PtBr),
    };
    let mut result = None;
    for first in facts.iter().filter(|f| {
        f.language == source
            && f.text == text
            && f.positive
            && context.is_none_or(|cx| cx == f.context)
    }) {
        if facts.iter().any(|f| {
            f.language == source
                && f.text == text
                && f.context == first.context
                && f.concept == first.concept
                && !f.positive
        }) {
            return Simple::Conflict;
        }
        for second in facts.iter().filter(|f| {
            f.language == target
                && f.context == first.context
                && f.concept == first.concept
                && f.positive
        }) {
            if facts.iter().any(|f| {
                f.language == target
                    && f.text == second.text
                    && f.context == second.context
                    && f.concept == second.concept
                    && !f.positive
            }) {
                return Simple::Conflict;
            }
            if result.is_some() {
                return Simple::Ambiguous;
            }
            result = Some(Simple::Supported(
                second.text.to_owned(),
                second.concept,
                second.context,
            ));
        }
    }
    result.unwrap_or(Simple::Unknown)
}
