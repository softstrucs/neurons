//! Disposições de apresentação explícitas; não alteram critérios de evidência.
use crate::metacognition::ReviewReport;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Style {
    Concise,
    Explanatory,
}

impl Style {
    /// Clareza e cautela são estáveis em ambos os estilos. O relatório completo
    /// permanece visível, inclusive erros, limites, revisões e abstenções.
    pub fn render(self, report: &ReviewReport) -> String {
        let introduction = match self {
            Self::Concise => "Resultado experimental:",
            Self::Explanatory => "Avaliei os registros fornecidos com critérios explícitos. A decisão e suas evidências são:",
        };
        format!("{introduction}\n{report:?}\n")
    }
}
