//! EC-04-R1 deterministic data-construction preflight; no learning model.
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Target {
    pub(crate) p1: u8,
    pub(crate) v1: bool,
    pub(crate) p2: u8,
    pub(crate) v2: bool,
    pub(crate) p3: Option<(u8, bool)>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Domain {
    P,
    N1,
    N2,
    N3,
    T,
    F,
    O,
    W1,
    W2,
}
#[derive(Clone, Copy, Debug)]
struct Slot {
    domain: Domain,
    group: u8,
    forced: bool,
    smallest: bool,
}
fn test(t: Target, d: Domain, x: u8) -> bool {
    let a = (x >> t.p1) & 1 == t.v1 as u8;
    let b = (x >> t.p2) & 1 == t.v2 as u8;
    let c = t.p3.map(|(p, v)| (x >> p) & 1 == v as u8).unwrap_or(false);
    match d {
        Domain::P => a && b,
        Domain::N1 => !a && b,
        Domain::N2 => a && !b,
        Domain::N3 => !a && !b,
        Domain::T => a && b && c,
        Domain::F => a && b && !c,
        Domain::O => !a || !b,
        Domain::W1 => !a && b && c,
        Domain::W2 => a && !b && c,
    }
}
fn group(slots: &mut Vec<Slot>, d: Domain, n: usize, g: &mut u8, smallest: bool) {
    *g += 1;
    for _ in 0..n {
        slots.push(Slot {
            domain: d,
            group: *g,
            forced: false,
            smallest,
        });
    }
}
fn pairs(slots: &mut Vec<Slot>, d: Domain, n: usize, g: &mut u8) {
    *g += 1;
    for _ in 0..n {
        slots.push(Slot {
            domain: d,
            group: *g,
            forced: false,
            smallest: false,
        });
        slots.push(Slot {
            domain: d,
            group: *g,
            forced: true,
            smallest: false,
        });
    }
}
fn slots(changed: bool) -> Vec<Slot> {
    let mut s = Vec::new();
    let mut g = 0;
    pairs(&mut s, Domain::P, 4, &mut g);
    for d in [Domain::N1, Domain::N2, Domain::N3] {
        group(&mut s, d, 8, &mut g, false);
    }
    if changed {
        group(&mut s, Domain::T, 2, &mut g, false);
        group(&mut s, Domain::F, 2, &mut g, false);
    } else {
        group(&mut s, Domain::P, 4, &mut g, false);
    }
    for d in [Domain::N1, Domain::N2, Domain::N3] {
        group(&mut s, d, 4, &mut g, false);
    }
    if changed {
        group(&mut s, Domain::T, 4, &mut g, false);
        group(&mut s, Domain::F, 4, &mut g, false);
    } else {
        group(&mut s, Domain::P, 8, &mut g, false);
    }
    for d in [Domain::N1, Domain::N2, Domain::N3] {
        group(&mut s, d, 8, &mut g, false);
    }
    if changed {
        pairs(&mut s, Domain::T, 1, &mut g);
        group(&mut s, Domain::F, 8, &mut g, false);
        group(&mut s, Domain::W1, 1, &mut g, false);
        group(&mut s, Domain::W2, 1, &mut g, false);
        group(&mut s, Domain::O, 4, &mut g, true);
        group(&mut s, Domain::T, 1, &mut g, false);
        group(&mut s, Domain::F, 4, &mut g, false);
        group(&mut s, Domain::O, 3, &mut g, false);
        group(&mut s, Domain::T, 2, &mut g, false);
        group(&mut s, Domain::F, 8, &mut g, false);
        group(&mut s, Domain::O, 6, &mut g, false);
    }
    s
}
pub(crate) fn rank(x: u8) -> u8 {
    x.wrapping_mul(197).wrapping_add(101)
}
pub(crate) fn generate(t: Target) -> Result<(Vec<u8>, u64), &'static str> {
    let ss = slots(t.p3.is_some());
    let mut order: Vec<u8> = (0..=255).collect();
    order.sort_by_key(|&x| (rank(x), x));
    let mut chosen = vec![0; ss.len()];
    let mut used = BTreeSet::new();
    let mut expansions = 0u64;
    fn dfs(
        i: usize,
        ss: &[Slot],
        t: Target,
        order: &[u8],
        chosen: &mut [u8],
        used: &mut BTreeSet<u8>,
        n: &mut u64,
    ) -> Result<bool, &'static str> {
        if i == ss.len() {
            return Ok(true);
        }
        let sl = ss[i];
        let relevant = (1u8 << t.p1)
            | (1u8 << t.p2)
            | if i >= 80 {
                t.p3.map(|(p, _)| 1u8 << p).unwrap_or(0)
            } else {
                0
            };
        let options: Vec<u8> = if sl.forced {
            vec![chosen[i - 1] ^ !relevant]
        } else {
            order
                .iter()
                .copied()
                .filter(|&x| test(t, sl.domain, x))
                .collect()
        };
        let previous = (0..i)
            .rev()
            .find(|&j| ss[j].group == sl.group && !ss[j].forced)
            .map(|j| chosen[j]);
        let options: Vec<u8> = if sl.smallest {
            options
                .into_iter()
                .filter(|x| !used.contains(x))
                .take(1)
                .collect()
        } else {
            options
        };
        for x in options {
            if *n == 2_000_000 {
                return Err("DataConstructionBudget");
            };
            *n += 1;
            if used.contains(&x) || !test(t, sl.domain, x) {
                continue;
            }
            if sl.forced && rank(chosen[i - 1]) >= rank(x) {
                continue;
            }
            if !sl.forced && previous.is_some_and(|p| rank(p) >= rank(x)) {
                continue;
            }
            chosen[i] = x;
            used.insert(x);
            if dfs(i + 1, ss, t, order, chosen, used, n)? {
                return Ok(true);
            }
            used.remove(&x);
        }
        Ok(false)
    }
    match dfs(0, &ss, t, &order, &mut chosen, &mut used, &mut expansions)? {
        true => Ok((chosen, expansions)),
        false => Err("DataConstructionUnsatisfiable"),
    }
}
fn identifiable(t: Target, data: &[u8], start: usize, end: usize, revised: bool) -> usize {
    let mut count = 0;
    for mask in 1u16..=255 {
        if !(1..=3).contains(&mask.count_ones()) {
            continue;
        }
        for value in 0u16..=255 {
            if value & !mask != 0 {
                continue;
            }
            if data[start..end].iter().all(|&x| {
                let target = test(t, Domain::P, x)
                    && (!revised || t.p3.is_some_and(|(p, v)| (x >> p) & 1 == v as u8));
                (u16::from(x) & mask == value) == target
            }) {
                count += 1;
            }
        }
    }
    count
}
fn main() {
    let cases = [
        (
            "dev-A",
            Target {
                p1: 1,
                v1: true,
                p2: 4,
                v2: false,
                p3: Some((7, true)),
            },
        ),
        (
            "dev-B",
            Target {
                p1: 0,
                v1: true,
                p2: 6,
                v2: true,
                p3: None,
            },
        ),
        (
            "val-A",
            Target {
                p1: 3,
                v1: false,
                p2: 5,
                v2: true,
                p3: Some((0, false)),
            },
        ),
        (
            "val-B",
            Target {
                p1: 2,
                v1: true,
                p2: 4,
                v2: false,
                p3: None,
            },
        ),
        (
            "final-1-A",
            Target {
                p1: 0,
                v1: true,
                p2: 1,
                v2: true,
                p3: Some((4, true)),
            },
        ),
        (
            "final-1-B",
            Target {
                p1: 2,
                v1: true,
                p2: 3,
                v2: false,
                p3: None,
            },
        ),
        (
            "final-2-A",
            Target {
                p1: 2,
                v1: false,
                p2: 3,
                v2: true,
                p3: Some((6, false)),
            },
        ),
        (
            "final-2-B",
            Target {
                p1: 4,
                v1: true,
                p2: 5,
                v2: true,
                p3: None,
            },
        ),
        (
            "final-3-A",
            Target {
                p1: 4,
                v1: true,
                p2: 5,
                v2: false,
                p3: Some((0, true)),
            },
        ),
        (
            "final-3-B",
            Target {
                p1: 6,
                v1: false,
                p2: 7,
                v2: false,
                p3: None,
            },
        ),
        (
            "final-4-A",
            Target {
                p1: 6,
                v1: false,
                p2: 7,
                v2: false,
                p3: Some((2, false)),
            },
        ),
        (
            "final-4-B",
            Target {
                p1: 0,
                v1: false,
                p2: 1,
                v2: true,
                p3: None,
            },
        ),
    ];
    for (name, t) in cases {
        match generate(t) {
            Ok((data, n)) => {
                let initial = identifiable(t, &data, 0, 32, false);
                let revised = if t.p3.is_some() {
                    identifiable(t, &data, 80, 96, true)
                } else {
                    1
                };
                assert_eq!((initial, revised), (1, 1));
                println!("{name} slots={} expansions={n} identifiable_initial={initial} identifiable_revised={revised}",data.len())
            }
            Err(e) => {
                eprintln!("{name}: {e}");
                std::process::exit(1)
            }
        }
    }
}

#[cfg(test)]
mod preflight_tests {
    use super::*;
    fn cases() -> Vec<(&'static str, Target)> {
        vec![
            (
                "dev-A",
                Target {
                    p1: 1,
                    v1: true,
                    p2: 4,
                    v2: false,
                    p3: Some((7, true)),
                },
            ),
            (
                "dev-B",
                Target {
                    p1: 0,
                    v1: true,
                    p2: 6,
                    v2: true,
                    p3: None,
                },
            ),
            (
                "val-A",
                Target {
                    p1: 3,
                    v1: false,
                    p2: 5,
                    v2: true,
                    p3: Some((0, false)),
                },
            ),
            (
                "val-B",
                Target {
                    p1: 2,
                    v1: true,
                    p2: 4,
                    v2: false,
                    p3: None,
                },
            ),
            (
                "final-1-A",
                Target {
                    p1: 0,
                    v1: true,
                    p2: 1,
                    v2: true,
                    p3: Some((4, true)),
                },
            ),
            (
                "final-1-B",
                Target {
                    p1: 2,
                    v1: true,
                    p2: 3,
                    v2: false,
                    p3: None,
                },
            ),
            (
                "final-2-A",
                Target {
                    p1: 2,
                    v1: false,
                    p2: 3,
                    v2: true,
                    p3: Some((6, false)),
                },
            ),
            (
                "final-2-B",
                Target {
                    p1: 4,
                    v1: true,
                    p2: 5,
                    v2: true,
                    p3: None,
                },
            ),
            (
                "final-3-A",
                Target {
                    p1: 4,
                    v1: true,
                    p2: 5,
                    v2: false,
                    p3: Some((0, true)),
                },
            ),
            (
                "final-3-B",
                Target {
                    p1: 6,
                    v1: false,
                    p2: 7,
                    v2: false,
                    p3: None,
                },
            ),
            (
                "final-4-A",
                Target {
                    p1: 6,
                    v1: false,
                    p2: 7,
                    v2: false,
                    p3: Some((2, false)),
                },
            ),
            (
                "final-4-B",
                Target {
                    p1: 0,
                    v1: false,
                    p2: 1,
                    v2: true,
                    p3: None,
                },
            ),
        ]
    }
    fn digest(name: &str, partition: &str, bytes: &[u8]) {
        use std::{
            io::Write,
            process::{Command, Stdio},
        };
        let mut child = Command::new("sha256sum")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(bytes).unwrap();
        let result = child.wait_with_output().unwrap();
        assert!(result.status.success());
        println!(
            "MANIFEST {name} {partition} bytes={} sha256={}",
            bytes.len(),
            std::str::from_utf8(&result.stdout)
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
        );
    }
    #[test]
    fn all_manifests_obey_domains_quotas_pairs_and_repetition() {
        for (name, t) in cases() {
            let (data, n) = generate(t).expect("construction budget or unsatisfiable");
            let (again, n2) = generate(t).unwrap();
            assert_eq!((&data, n), (&again, n2), "{name} nondeterministic");
            assert!(n <= 2_000_000);
            assert_eq!(data.len(), if t.p3.is_some() { 120 } else { 80 });
            assert_eq!(
                data.iter().copied().collect::<BTreeSet<_>>().len(),
                data.len()
            );
            let def = slots(t.p3.is_some());
            for (i, (&x, slot)) in data.iter().zip(&def).enumerate() {
                assert!(test(t, slot.domain, x), "{name} wrong domain at {i}");
                if slot.forced {
                    let relevant = (1u8 << t.p1)
                        | (1u8 << t.p2)
                        | if i >= 80 {
                            t.p3.map(|(p, _)| 1u8 << p).unwrap_or(0)
                        } else {
                            0
                        };
                    assert_eq!(x, data[i - 1] ^ !relevant, "{name} forced pair {i}");
                    assert!(rank(data[i - 1]) < rank(x));
                }
                if !slot.forced {
                    if let Some(prev) = (0..i)
                        .rev()
                        .find(|&j| def[j].group == slot.group && !def[j].forced)
                    {
                        assert!(rank(data[prev]) < rank(x), "{name} group order {i}");
                    }
                }
            }
            for i in 92..if t.p3.is_some() { 96 } else { 92 } {
                let mut free: Vec<u8> = (0..=255)
                    .filter(|x| test(t, Domain::O, *x) && !data[..i].contains(x))
                    .collect();
                free.sort_by_key(|x| (rank(*x), *x));
                assert_eq!(Some(&data[i]), free.first(), "{name} O-livre {i}");
            }
            // Independent quota checks use predicates directly, not the generated slots.
            let positive =
                |x: u8| ((x >> t.p1) & 1 == u8::from(t.v1)) && ((x >> t.p2) & 1 == u8::from(t.v2));
            for (range, want) in [(0..32, 8), (32..48, 4), (48..80, 8)] {
                assert_eq!(
                    data[range].iter().filter(|x| positive(**x)).count(),
                    want,
                    "{name} positive quota"
                );
            }
            if let Some((p3, v3)) = t.p3 {
                let revised = |x: u8| positive(x) && (x >> p3) & 1 == u8::from(v3);
                for (range, want) in [(80..96, 2), (96..104, 1), (104..120, 2)] {
                    assert_eq!(
                        data[range].iter().filter(|x| revised(**x)).count(),
                        want,
                        "{name} revision quota"
                    );
                }
                assert_eq!(
                    data[104..120]
                        .iter()
                        .filter(|x| positive(**x) && !revised(**x))
                        .count(),
                    8
                );
            }
            assert_eq!(identifiable(t, &data, 0, 32, false), 1);
            if t.p3.is_some() {
                assert_eq!(identifiable(t, &data, 80, 96, true), 1);
            }
            assert!(data[..32].iter().all(|x| !data[32..80].contains(x)));
            if t.p3.is_some() {
                assert!(data[..80].iter().all(|x| !data[80..].contains(x)));
            }
            for (partition, range) in [
                ("acquisition", 0..32),
                ("validation", 32..48),
                ("final", 48..80),
            ] {
                digest(name, partition, &data[range]);
            }
            if t.p3.is_some() {
                for (partition, range) in [
                    ("revision-acquisition", 80..96),
                    ("revision-validation", 96..104),
                    ("revision-final", 104..120),
                ] {
                    digest(name, partition, &data[range]);
                }
            }
            println!(
                "{name}: slots={} expansions={n} unique=yes identified=yes",
                data.len()
            );
        }
    }
}
