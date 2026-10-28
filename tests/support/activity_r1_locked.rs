// Congelado antes da avaliação R1. Rótulos novos; somente padrões numéricos conhecidos.
pub const FORMS: [&str; 24] = [
    "r1 araucaria",
    "r1 cedro",
    "r1 pinheiro",
    "r1 ipê",
    "r1 jacaranda",
    "r1 peroba",
    "r1 aroeira",
    "r1 imbuia",
    "r1 angico",
    "r1 jatoba",
    "r1 pau-brasil",
    "r1 jequitiba",
    "r1 garça",
    "r1 tucano",
    "r1 sabiá",
    "r1 arara",
    "r1 coruja",
    "r1 ema",
    "r1 seriema",
    "r1 curió",
    "r1 canário",
    "r1 bem-te-vi",
    "r1 pardal",
    "r1 joão-de-barro",
];
pub fn input(i: usize) -> [bool; 2] {
    if i < 12 {
        [true, false]
    } else {
        [false, true]
    }
}
pub fn expected(i: usize) -> usize {
    usize::from(i >= 12)
}
