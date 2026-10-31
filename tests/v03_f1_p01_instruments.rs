#[path = "support/v03_f1_p01/canonical.rs"]
mod canonical;
#[path = "support/v03_f1_p01/reference.rs"]
mod reference;
#[path = "support/v03_f1_p01/trace.rs"]
mod trace;
use neurons::cognition::{Hypothesis, TwoHopTask};
use neurons::engine::Engine;
use neurons::engram::{Cue, Engram, EngramId, EngramStore, EvidenceId, RecordStatus};
use neurons::event::NodeId;
use neurons::integration::IntegratedState;
use neurons::metacognition::{Monitor, ReviewError};
use reference::{manifest, Manifest, Verdict};
use std::time::Instant;
fn node(x: u32) -> NodeId {
    NodeId::new(x).unwrap()
}
fn monitor() -> Monitor {
    Monitor::new(TwoHopTask::new(8, 8, 8).unwrap(), 1).unwrap()
}
fn state(m: &Manifest) -> IntegratedState {
    let mut store = EngramStore::new(8).unwrap();
    for r in &m.records {
        store
            .record(
                Engram::new(
                    EngramId::new(r.id).unwrap(),
                    Cue::new(r.context, node(r.from), r.to as i32).unwrap(),
                    r.sign,
                    r.origin.map(node),
                    r.observed_at,
                    r.evidence.map(|v| EvidenceId::new(v).unwrap()),
                    if r.validated {
                        RecordStatus::Validated
                    } else {
                        RecordStatus::Observed
                    },
                )
                .unwrap(),
            )
            .unwrap();
    }
    IntegratedState::new(Engine::new(8, 8).unwrap(), store, monitor())
}
fn run(
    m: &Manifest,
) -> (
    canonical::Fields,
    canonical::Fields,
    Vec<u8>,
    Vec<reference::Record>,
) {
    let s = state(m);
    let bytes = s.checkpoint().unwrap();
    let q = Hypothesis::new(m.context, node(m.from), node(m.to)).unwrap();
    let report = s.review(q, &[]);
    assert_eq!(s.checkpoint().unwrap(), bytes);
    let report_again = s.review(q, &[]);
    assert_eq!(s.checkpoint().unwrap(), bytes);
    let mut observed = vec![];
    for r in &m.records {
        let x = s.memory().get(EngramId::new(r.id).unwrap()).unwrap();
        let cue = x.cue();
        observed.push(reference::Record {
            id: x.id().get(),
            context: cue.context(),
            from: cue.source().get(),
            to: cue.pattern() as u32,
            sign: x.value(),
            origin: x.origin().map(|v| v.get()),
            observed_at: x.observed_at(),
            evidence: x.evidence().map(|v| v.get()),
            validated: x.status() == RecordStatus::Validated,
        });
    }
    (
        canonical::fields(&report),
        canonical::fields(&report_again),
        bytes,
        observed,
    )
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
#[test]
fn child_process() {
    let Ok(spec) = std::env::var("NEURONS_V03_F1_DV_CASE") else {
        return;
    };
    trace::child_gate();
    let mut c = spec.chars();
    let p = c.next().unwrap();
    let n = c.as_str().parse::<u8>().unwrap();
    let m = manifest(p, n).unwrap();
    let raw = std::env::var("NEURONS_V03_F1_DV_CHECKPOINT").unwrap();
    assert_eq!(raw.len() % 2, 0);
    let b: Vec<u8> = raw
        .as_bytes()
        .chunks_exact(2)
        .map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
        .collect();
    let restored = IntegratedState::restore(&b, monitor()).unwrap();
    let query = Hypothesis::new(m.context, node(m.from), node(m.to)).unwrap();
    let f = canonical::fields(&restored.review(query, &[]));
    assert_eq!(restored.checkpoint().unwrap(), b);
    println!(
        "V03F1_CHILD:{}{}:{}:{}",
        m.partition,
        m.case,
        canonical::wire(&f),
        hex(&b)
    );
}
fn execute(p: char) {
    let start = Instant::now();
    let mut decisions = 0;
    let mut comparisons = 0;
    let mut altered = 0;
    for rep in 0..3 {
        for n in 1..=6 {
            let m = manifest(p, n).unwrap();
            let (a, a2, b, e) = run(&m);
            assert!(canonical::compare(&a, &a2).is_empty());
            comparisons += 1;
            let diag = reference::verify(&m, &a, &e);
            assert_eq!(
                diag.verdict,
                if n == 5 {
                    Verdict::Incomplete
                } else {
                    Verdict::Valid
                },
                "{:?}",
                diag.differences
            );
            decisions += 2;
            let mut cmd = std::process::Command::new(std::env::current_exe().unwrap());
            cmd.args(["--exact", "child_process", "--nocapture"])
                .env("NEURONS_V03_F1_DV_CASE", format!("{p}{n}"))
                .env("NEURONS_V03_F1_DV_CHECKPOINT", hex(&b));
            let out = trace::run_child(&mut cmd, p, rep, n);
            assert!(out.status.success());
            let stdout = String::from_utf8(out.stdout).unwrap();
            assert!(
                stdout.contains("running 1 test"),
                "child test count missing: {stdout}"
            );
            let prefix = format!("V03F1_CHILD:{p}{n}:");
            let matches: Vec<_> = stdout
                .lines()
                .filter_map(|line| line.strip_prefix(&prefix))
                .collect();
            assert_eq!(matches.len(), 1, "child identity missing or duplicate");
            let (child_wire, child_checkpoint) =
                matches[0].rsplit_once(':').expect("child payload");
            let child_fields = canonical::parse_wire(child_wire).expect("child schema");
            let differences = canonical::compare(&a, &child_fields);
            assert!(differences.is_empty(), "child differences: {differences:?}");
            assert_eq!(child_checkpoint, hex(&b));
            assert_eq!(
                reference::verify(&m, &child_fields, &e).verdict,
                if n == 5 {
                    Verdict::Incomplete
                } else {
                    Verdict::Valid
                }
            );
            comparisons += 1;
            decisions += 1;
            if n == 1 {
                let restored = IntegratedState::restore(&b, monitor()).unwrap();
                let before = restored.checkpoint().unwrap();
                let q = Hypothesis::new(m.context, node(m.from), node(m.to)).unwrap();
                assert_eq!(
                    restored.review(q, &[m.context]),
                    Err(ReviewError::InvalidCandidateContext)
                );
                assert_eq!(restored.checkpoint().unwrap(), before);
            }
            if n == 1 {
                // Fato positivo adicional no mesmo contexto: não muda o suporte,
                // mas deve aparecer em facts_loaded. Usar apenas IDs D/V.
                let mut extra_case = manifest(p, 1).unwrap();
                let mut unrelated = extra_case.records[0].clone();
                unrelated.id += 100;
                unrelated.to += 1;
                extra_case.records.push(unrelated);
                let (extra_fields, _, _, extra_records) = run(&extra_case);
                assert_eq!(
                    reference::verify(&extra_case, &extra_fields, &extra_records).verdict,
                    Verdict::Valid
                );
                let mut wrong_count = extra_fields.clone();
                wrong_count
                    .iter_mut()
                    .find(|(k, _)| k == "attempts.0.inference.value.facts_loaded")
                    .unwrap()
                    .1 = "1".into();
                let count_diagnosis = reference::verify(&extra_case, &wrong_count, &extra_records);
                assert_eq!(count_diagnosis.verdict, Verdict::Invalid);
                assert!(count_diagnosis
                    .differences
                    .iter()
                    .any(|x| x.contains("facts_loaded")));
                let encoded = canonical::wire(&a);
                assert_eq!(canonical::parse_wire(&encoded).unwrap(), a);
                assert!(canonical::parse_wire("not-a-length").is_err());
                assert!(canonical::parse_wire(&encoded[..encoded.len() - 1]).is_err());
                let mut unknown_schema = a.clone();
                unknown_schema[0].1 = "unknown".into();
                assert!(canonical::parse_wire(&canonical::wire(&unknown_schema)).is_err());
                let mut duplicated = a.clone();
                duplicated.push(a[0].clone());
                assert!(canonical::parse_wire(&canonical::wire(&duplicated)).is_err());
                for i in 0..a.len() {
                    let mut missing_field = a.clone();
                    missing_field.remove(i);
                    assert_eq!(
                        reference::verify(&m, &missing_field, &e).verdict,
                        Verdict::Invalid
                    );
                    let mut altered_field = a.clone();
                    altered_field[i].1.push_str("_unknown");
                    assert_eq!(
                        reference::verify(&m, &altered_field, &e).verdict,
                        Verdict::Invalid
                    );
                }
                let mut wrong_origin = e.clone();
                wrong_origin[0].origin = None;
                let origin_diagnosis = reference::verify(&m, &a, &wrong_origin);
                assert_eq!(origin_diagnosis.verdict, Verdict::Invalid);
                assert!(origin_diagnosis
                    .differences
                    .iter()
                    .any(|x| x.contains(".origin:")));
                let mut invalid_reference = e.clone();
                invalid_reference[0].evidence = Some(999999);
                assert_eq!(
                    reference::verify(&m, &a, &invalid_reference).verdict,
                    Verdict::Invalid
                );
                for (key, value) in [("original.context", "0"), ("decision.evidence.id", "0")] {
                    let mut edit = a.clone();
                    edit.iter_mut().find(|(k, _)| k == key).unwrap().1 = value.into();
                    assert_eq!(reference::verify(&m, &edit, &e).verdict, Verdict::Invalid);
                    altered += 1;
                }
                let mut missing = e.clone();
                missing[0].evidence = None;
                assert_eq!(
                    reference::verify(&m, &a, &missing).verdict,
                    Verdict::Incomplete
                );
                altered += 1;
            }
        }
    }
    assert!(manifest('X', 1).is_err());
    assert_eq!((decisions, comparisons, altered), (54, 36, 9));
    println!("partition={p} decisions={decisions} comparisons={comparisons} alterations={altered} elapsed_us={} max_case_checkpoint_bytes={}",start.elapsed().as_micros(), state(&manifest(p,4).unwrap()).checkpoint().unwrap().len());
    assert!(start.elapsed().as_secs() < 30);
}
#[test]
fn development() {
    trace::test_gate();
    execute('D')
}
#[test]
fn validation() {
    trace::test_gate();
    execute('V')
}
