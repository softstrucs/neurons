//! Referência independente para os seis casos diretos; não chama o núcleo.
use super::canonical::Fields;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub context: u32,
    pub from: u32,
    pub to: u32,
    pub sign: i32,
    pub origin: Option<u32>,
    pub observed_at: u64,
    pub evidence: Option<u32>,
    pub validated: bool,
}
pub struct Manifest {
    pub partition: char,
    pub case: u8,
    pub context: u32,
    pub from: u32,
    pub to: u32,
    pub records: Vec<Record>,
}
pub fn manifest(partition: char, case: u8) -> Result<Manifest, &'static str> {
    let b = match partition {
        'D' => 1000,
        'V' => 2000,
        _ => return Err("partition_not_authorized"),
    };
    if !(1..=6).contains(&case) {
        return Err("unknown_case");
    }
    let a = b + 1;
    let other = b + 2;
    let from = b + 101;
    let to = b + 102;
    let id = b + 10 * u32::from(case);
    let ev = b + 500 + 10 * u32::from(case);
    let rec = |id, context, sign, evidence, validated| Record {
        id,
        context,
        from,
        to,
        sign,
        origin: Some(from),
        observed_at: 7,
        evidence,
        validated,
    };
    let mut records = vec![];
    if case != 6 {
        records.push(rec(
            id,
            a,
            1,
            if case == 5 { None } else { Some(ev) },
            case != 5,
        ));
    }
    if case == 2 || case == 3 {
        records.push(rec(id + 1, other, -1, None, false));
    }
    if case == 4 {
        records.push(rec(id + 1, a, -1, None, false));
    }
    Ok(Manifest {
        partition,
        case,
        context: if case == 3 { other } else { a },
        from,
        to,
        records,
    })
}
#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Valid,
    Incomplete,
    Invalid,
}
#[derive(Debug)]
pub struct Diagnosis {
    pub verdict: Verdict,
    pub differences: Vec<String>,
}
fn add(v: &mut Fields, k: &str, x: impl ToString) {
    v.push((k.into(), x.to_string()));
}
pub fn verify(m: &Manifest, f: &Fields, observed: &[Record]) -> Diagnosis {
    let mut expected = vec![];
    let mut errors = vec![];
    let mut incomplete = false;
    let relevant: Vec<_> = m
        .records
        .iter()
        .filter(|r| r.context == m.context && r.from == m.from && r.to == m.to)
        .collect();
    let pos: Vec<_> = relevant.iter().filter(|r| r.sign == 1).collect();
    let neg: Vec<_> = relevant.iter().filter(|r| r.sign == -1).collect();
    if pos.len() > 1
        || neg.len() > 1
        || m.records
            .iter()
            .any(|r| r.validated && r.evidence.is_none())
    {
        return Diagnosis {
            verdict: Verdict::Invalid,
            differences: vec!["manifest.invalid_or_ambiguous".into()],
        };
    }
    add(&mut expected, "schema", "v03-f1-p01/v1");
    add(&mut expected, "result", "Ok");
    for (suffix, value) in [("context", m.context), ("from", m.from), ("to", m.to)] {
        add(&mut expected, &format!("original.{suffix}"), value);
    }
    if let Some(p) = pos.first() {
        if neg.is_empty() {
            add(&mut expected, "decision", "Respond");
            for (suffix, value) in [("context", m.context), ("from", m.from), ("to", m.to)] {
                add(
                    &mut expected,
                    &format!("decision.hypothesis.{suffix}"),
                    value,
                );
            }
            add(&mut expected, "decision.evidence", "Direct");
            add(&mut expected, "decision.evidence.id", p.id);
            add(&mut expected, "decision.revision", "none");
        } else {
            add(&mut expected, "decision", "Abstain");
            add(&mut expected, "decision.reason", "ContradictoryEvidence");
        }
    } else {
        add(&mut expected, "decision", "Abstain");
        add(&mut expected, "decision.reason", "MissingEvidence");
    }
    add(&mut expected, "attempts.len", 1);
    for (suffix, value) in [("context", m.context), ("from", m.from), ("to", m.to)] {
        add(
            &mut expected,
            &format!("attempts.0.hypothesis.{suffix}"),
            value,
        );
    }
    add(&mut expected, "attempts.0.inference", "some");
    for (suffix, value) in [("context", m.context), ("from", m.from), ("to", m.to)] {
        add(
            &mut expected,
            &format!("attempts.0.inference.value.hypothesis.{suffix}"),
            value,
        );
    }
    if let Some(p) = pos.first() {
        add(
            &mut expected,
            "attempts.0.inference.value.outcome",
            "DirectRecall",
        );
        add(&mut expected, "attempts.0.inference.value.outcome.id", p.id);
    } else {
        add(
            &mut expected,
            "attempts.0.inference.value.outcome",
            "Unsupported",
        );
    }
    add(
        &mut expected,
        "attempts.0.inference.value.facts_loaded",
        m.records
            .iter()
            .filter(|r| r.context == m.context && r.sign == 1 && r.to > 0)
            .count(),
    );
    add(
        &mut expected,
        "attempts.0.inference.value.processed_events",
        0,
    );
    add(
        &mut expected,
        "attempts.0.inference.value.source_activations",
        0,
    );
    add(
        &mut expected,
        "attempts.0.inference.value.target_activations",
        0,
    );
    add(&mut expected, "attempts.0.evaluation_error", "none");
    add(
        &mut expected,
        "attempts.0.assessment",
        if pos.is_empty() {
            "MissingEvidence"
        } else if neg.is_empty() {
            "Sufficient"
        } else {
            "ContradictoryEvidence"
        },
    );
    if let Some(p) = pos.first() {
        add(&mut expected, "attempts.0.evidence", "some");
        add(&mut expected, "attempts.0.evidence.value", "Direct");
        add(&mut expected, "attempts.0.evidence.value.id", p.id);
    } else {
        add(&mut expected, "attempts.0.evidence", "none");
    }
    if !pos.is_empty() && !neg.is_empty() {
        add(&mut expected, "attempts.0.conflict", "some");
        add(
            &mut expected,
            "attempts.0.conflict.value",
            "NegativeConclusion",
        );
        add(
            &mut expected,
            "attempts.0.conflict.value.negative",
            neg[0].id,
        );
    } else {
        add(&mut expected, "attempts.0.conflict", "none");
    }
    for (i, (key, value)) in expected.iter().enumerate() {
        match f.get(i) {
            Some((k, v)) if k == key && v == value => {}
            Some((k, _)) => errors.push(format!("{key}:mismatch_or_order:{k}")),
            None => errors.push(format!("{key}:missing")),
        }
    }
    if f.len() > expected.len() {
        errors.push("report:unexpected_or_unknown_fields".into());
    }
    if observed.len() != m.records.len() {
        errors.push("records:missing_or_extra".into());
    }
    let mut incomplete_fields = vec![];
    for (actual, want) in observed.iter().zip(&m.records) {
        macro_rules! field {
            ($name:ident) => {
                if actual.$name != want.$name {
                    errors.push(format!("record.{}.{}:mismatch", want.id, stringify!($name)));
                }
            };
        }
        field!(id);
        field!(context);
        field!(from);
        field!(to);
        field!(sign);
        field!(origin);
        field!(observed_at);
        field!(validated);
        if actual.evidence != want.evidence {
            if actual.evidence.is_none() && want.evidence.is_some() {
                incomplete = true;
                incomplete_fields.push(format!("record.{}.evidence:missing", want.id));
            } else {
                errors.push(format!("record.{}.evidence:mismatch", want.id));
            }
        }
        if want.evidence.is_none() && want.sign == 1 {
            incomplete = true;
            incomplete_fields.push(format!("record.{}.evidence:not_provided", want.id));
        }
    }
    let verdict = if !errors.is_empty() {
        Verdict::Invalid
    } else if incomplete {
        Verdict::Incomplete
    } else {
        Verdict::Valid
    };
    errors.extend(incomplete_fields);
    Diagnosis {
        verdict,
        differences: errors,
    }
}
