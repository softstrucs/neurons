//! Development-only check of independently implemented EC-04 induction.
#[path = "support/ec04_candidate.rs"]
mod candidate;
#[allow(dead_code)]
#[path = "support/ec04_generator.rs"]
mod data;
#[path = "support/ec04_independent.rs"]
mod independent;
#[allow(dead_code)]
#[path = "support/ec04_types.rs"]
mod types;
use types::{Episode, Learner, Outcome};
fn label(t: data::Target, x: u8, revised: bool) -> bool {
    let old = (x >> t.p1) & 1 == u8::from(t.v1) && (x >> t.p2) & 1 == u8::from(t.v2);
    old && (!revised || t.p3.is_some_and(|(p, v)| (x >> p) & 1 == u8::from(v)))
}
#[test]
fn development_induction_and_revision_remain_independent() {
    let a = data::Target {
        p1: 1,
        v1: true,
        p2: 4,
        v2: false,
        p3: Some((7, true)),
    };
    let b = data::Target {
        p1: 0,
        v1: true,
        p2: 6,
        v2: true,
        p3: None,
    };
    let av = data::generate(a).unwrap().0;
    let bv = data::generate(b).unwrap().0;
    let mut m = candidate::Candidate::new();
    let mut s = independent::Independent::new();
    for (ctx, t, vals) in [(2001, a, &av), (2002, b, &bv)] {
        let mut acquired = vals[..32].to_vec();
        acquired.sort_by_key(|x| data::rank(*x));
        for (i, x) in acquired.into_iter().enumerate() {
            let e = Episode {
                acquisition: true,
                id: i as u32 + 1,
                x,
                label: label(t, x, false),
            };
            m.acquire(ctx, e).unwrap();
            s.acquire(ctx, e).unwrap();
        }
        for &x in &vals[32..48] {
            let ma = m.query(ctx, x).unwrap().0;
            let sa = s.query(ctx, x).unwrap().0;
            assert_eq!(ma, sa);
            assert_eq!(
                ma.outcome,
                if label(t, x, false) {
                    Outcome::Positive
                } else {
                    Outcome::Negative
                }
            );
        }
    }
    let b_before: Vec<_> = bv[32..48]
        .iter()
        .map(|x| m.query(2002, *x).unwrap().0)
        .collect();
    let mut revised = av[80..96].to_vec();
    revised.sort_by_key(|x| data::rank(*x));
    for (i, x) in revised.into_iter().enumerate() {
        let e = Episode {
            acquisition: true,
            id: 33 + i as u32,
            x,
            label: label(a, x, true),
        };
        m.acquire(2001, e).unwrap();
        s.acquire(2001, e).unwrap();
    }
    for &x in &av[96..104] {
        let ma = m.query(2001, x).unwrap().0;
        let sa = s.query(2001, x).unwrap().0;
        assert_eq!(ma, sa);
        assert_eq!(
            ma.outcome,
            if label(a, x, true) {
                Outcome::Positive
            } else {
                Outcome::Negative
            }
        );
    }
    assert_eq!(
        b_before,
        bv[32..48]
            .iter()
            .map(|x| m.query(2002, *x).unwrap().0)
            .collect::<Vec<_>>()
    );
}

#[path = "support/ec04_controls.rs"]
mod controls;
#[test]
fn development_controls_do_not_recover_unacquired_vectors() {
    use controls::{Condition, Control};
    let mut literal = Control::new(Condition::Literal);
    literal
        .acquire(
            2001,
            Episode {
                acquisition: true,
                id: 1,
                x: 3,
                label: true,
            },
        )
        .unwrap();
    assert_eq!(literal.query(2001, 3).unwrap().0.outcome, Outcome::Positive);
    assert_eq!(literal.query(2001, 4).unwrap().0.outcome, Outcome::Missing);
    let candidate = Control::new(Condition::Candidate);
    let frozen = Control::frozen(&candidate);
    assert_eq!(
        candidate.query(2001, 4).unwrap().0,
        frozen.query(2001, 4).unwrap().0
    );
    for condition in [
        Condition::AlwaysPositive,
        Condition::AlwaysNegative,
        Condition::AlwaysAbstain,
        Condition::NoAcquisition,
        Condition::NoUpdate,
        Condition::Independent,
        Condition::FixedP7,
    ] {
        let model = Control::new(condition);
        let copy = model.clone();
        model.query(2001, 4).unwrap();
        assert_eq!(model, copy);
    }
}
