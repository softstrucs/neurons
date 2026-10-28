#[path = "support/lexical_final.rs"]
mod lexical;
use neurons::languages::{Direction, Lexicon, Outcome};
#[test]
fn reserved_protocol_bidirectional_and_negative_controls() {
    let l = lexical::lexicon();
    let mut correct = [0; 2];
    let mut direct = [0; 2];
    for (pt, en) in lexical::PAIRS {
        for (index, direction, input, expected) in [
            (0, Direction::PtToEn, pt, en),
            (1, Direction::EnToPt, en, pt),
        ] {
            let result = l.translate(direction, input, 1).unwrap();
            assert_eq!(result, l.translate(direction, input, 1).unwrap());
            if matches!(&result,Outcome::Supported{text,..} if text==expected) {
                correct[index] += 1;
            }
            let lookup = lexical::PAIRS.iter().find_map(|(p, e)| {
                if index == 0 && *p == input {
                    Some(*e)
                } else if index == 1 && *e == input {
                    Some(*p)
                } else {
                    None
                }
            });
            if lookup == Some(expected) {
                direct[index] += 1;
            }
        }
    }
    let unknown = [
        (Direction::PtToEn, "barco"),
        (Direction::PtToEn, "sapato"),
        (Direction::PtToEn, "nuvem"),
        (Direction::PtToEn, "livros"),
        (Direction::EnToPt, "boat"),
        (Direction::EnToPt, "shoe"),
        (Direction::EnToPt, "cloud"),
        (Direction::EnToPt, "books"),
    ];
    let mut abstentions = 0;
    for (direction, text) in unknown {
        if l.translate(direction, text, 1).unwrap() == Outcome::Unknown {
            abstentions += 1;
        }
    }
    let ambiguous = [
        ("banco", "bank", "bench"),
        ("manga", "mango", "sleeve"),
        ("vela", "candle", "sail"),
        ("planta", "plant", "floorplan"),
    ];
    for (pt, en1, en2) in ambiguous {
        let mut x = Lexicon::new();
        x.add_pair(pt, en1).unwrap();
        x.add_pair(pt, en2).unwrap();
        if x.translate(Direction::PtToEn, pt, 1).unwrap() == Outcome::Ambiguous {
            abstentions += 1;
        }
    }
    let reverse = [
        ("banco", "margem", "bank"),
        ("luz", "leve", "light"),
        ("selo", "foca", "seal"),
        ("partida", "fósforo", "match"),
    ];
    for (pt1, pt2, en) in reverse {
        let mut x = Lexicon::new();
        x.add_pair(pt1, en).unwrap();
        x.add_pair(pt2, en).unwrap();
        if x.translate(Direction::EnToPt, en, 1).unwrap() == Outcome::Ambiguous {
            abstentions += 1;
        }
    }
    assert_eq!(correct, [24, 24]);
    assert_eq!(direct, [24, 24]);
    assert_eq!(abstentions, 16);
    println!("final pt_en={}/24 en_pt={}/24 direct_pt_en={}/24 direct_en_pt={}/24 negative_abstentions={}/16 false_alarms=0/48 missed_detection=0/16",correct[0],correct[1],direct[0],direct[1],abstentions);
}
