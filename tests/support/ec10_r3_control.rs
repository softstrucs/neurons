//! Separate implementation: literal-combination enumeration and fresh suffix search.
use std::collections::BTreeMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Output {
    Positive,
    Negative,
    Ambiguous,
    Missing,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Entry {
    id: u32,
    x: u16,
    y: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Context {
    log: Vec<Entry>,
    start: usize,
    patterns: Vec<[i8; 9]>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Independent {
    states: BTreeMap<u32, Context>,
}
fn enumerate(pos: usize, used: usize, p: &mut [i8; 9], out: &mut Vec<[i8; 9]>, log: &[Entry]) {
    if pos == 9 {
        if used > 0 && log.iter().all(|e| matches(p, e.x) == e.y) {
            out.push(*p)
        }
        return;
    }
    p[pos] = -1;
    enumerate(pos + 1, used, p, out, log);
    if used < 3 {
        for v in 0..=1 {
            p[pos] = v;
            enumerate(pos + 1, used + 1, p, out, log)
        }
    }
    p[pos] = -1;
}
fn matches(p: &[i8; 9], x: u16) -> bool {
    p.iter()
        .enumerate()
        .all(|(i, &v)| v < 0 || ((x >> i) & 1) == v as u16)
}
impl Independent {
    pub fn new() -> Self {
        Self {
            states: BTreeMap::new(),
        }
    }
    pub fn acquire(&mut self, ctx: u32, id: u32, x: u16, y: bool) {
        assert!(ctx > 0 && id > 0 && x < 512);
        let s = self.states.entry(ctx).or_insert_with(|| Context {
            log: Vec::new(),
            start: 0,
            patterns: Vec::new(),
        });
        assert!(!s.log.iter().any(|e| e.id == id || e.x == x));
        s.log.push(Entry { id, x, y });
        for start in 0..s.log.len() {
            let mut p = [-1i8; 9];
            let mut out = Vec::new();
            enumerate(0, 0, &mut p, &mut out, &s.log[start..]);
            if !out.is_empty() {
                s.start = start;
                s.patterns = out;
                return;
            }
        }
        panic!("no consistent suffix")
    }
    pub fn query(&self, ctx: u32, x: u16) -> (Output, usize, usize, usize) {
        let Some(s) = self.states.get(&ctx) else {
            return (Output::Missing, 0, 0, 0);
        };
        let (mut yes, mut no) = (false, false);
        for p in &s.patterns {
            if matches(p, x) {
                yes = true
            } else {
                no = true
            }
        }
        let d = match (yes, no) {
            (true, false) => Output::Positive,
            (false, true) => Output::Negative,
            (true, true) => Output::Ambiguous,
            _ => Output::Missing,
        };
        (d, s.log.len(), s.start, s.patterns.len())
    }
}
