//! EC-02.4-R1: executor, avaliador e oráculo ficam fora do aprendiz.
#[path = "support/expectation.rs"]
mod expectation;
#[path = "support/expectation_symbolic.rs"]
mod independent;
#[path = "support/expectation_locked.rs"]
mod locked;
#[allow(dead_code)]
#[path = "support/causal_diagnostic.rs"]
mod model;
#[path = "support/causal_symbolic.rs"]
mod symbolic;
use expectation::{
    Acquisition, Answer, Decision, Entry, Learner, Observation, Packet, Purpose, Reason,
};
use locked::{Category, Condition, PANEL};
use model::{Intervention, Scenario};
use std::{mem::size_of, time::Instant};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Metrics {
    correct: u16,
    predictions: u16,
    predicted_correct: u16,
    wrong_allow: u16,
    unneeded_protect: u16,
    abstentions: u16,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct QueryLog {
    phase: u8,
    key: u16,
    answers: [Answer; 7],
    entry: Option<Entry>,
    evaluation_pair: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct UpdateLog {
    pair: u32,
    key: u16,
    before: Option<Entry>,
    after: Option<Entry>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct ResultLog {
    packets: Vec<Packet>,
    queries: Vec<QueryLog>,
    updates: Vec<UpdateLog>,
    metrics: Vec<[Metrics; 7]>,
    final_entries: Vec<Entry>,
    events: usize,
    reproductions: usize,
    peak_data_bytes: usize,
}

struct Executor {
    log: ResultLog,
    started: Instant,
}
impl Executor {
    fn new() -> Self {
        Self {
            log: ResultLog {
                packets: Vec::with_capacity(136),
                queries: Vec::with_capacity(96),
                updates: Vec::with_capacity(40),
                metrics: Vec::with_capacity(6),
                final_entries: Vec::new(),
                events: 0,
                reproductions: 0,
                peak_data_bytes: 0,
            },
            started: Instant::now(),
        }
    }
    fn guard(&self) {
        assert!(
            self.started.elapsed().as_secs_f64() < 30.0,
            "STOP: 30 segundos"
        );
        let status = std::fs::read_to_string("/proc/self/status").unwrap();
        let rss: usize = status
            .lines()
            .find(|l| l.starts_with("VmRSS:"))
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        assert!(rss < 256 * 1024, "STOP: RSS");
        let output = std::process::Command::new("df")
            .args(["-B1", "--output=avail", "/home"])
            .output()
            .unwrap();
        assert!(output.status.success());
        let free: u64 = std::str::from_utf8(&output.stdout)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        assert!(free >= 3 * 1024 * 1024 * 1024, "STOP: disco");
    }
    fn memory(
        &mut self,
        m: &Learner,
        no: &Learner,
        s: &independent::Summary,
        temporary_bytes: usize,
    ) {
        let bytes = size_of::<Self>()
            + self.log.packets.capacity() * size_of::<Packet>()
            + self.log.queries.capacity() * size_of::<QueryLog>()
            + self.log.updates.capacity() * size_of::<UpdateLog>()
            + self.log.metrics.capacity() * size_of::<[Metrics; 7]>()
            + self.log.final_entries.capacity() * size_of::<Entry>()
            + size_of::<Learner>() * 2
            + (m.episodes.capacity() + no.episodes.capacity()) * size_of::<Acquisition>()
            + (m.entries.capacity() + no.entries.capacity() + s.rows.capacity())
                * size_of::<Entry>()
            + size_of::<independent::Summary>()
            + temporary_bytes
            + 16 * 1024;
        self.log.peak_data_bytes = self.log.peak_data_bytes.max(bytes);
        assert!(bytes <= 256 * 1024, "STOP: dados >256 KiB");
    }
    fn pair(&mut self, key: u16, backup: i32, purpose: Purpose) -> Packet {
        assert!(self.log.packets.len() < 136);
        let scenario = match backup {
            1 => Scenario::N,
            2 => Scenario::R,
            _ => panic!("parâmetro não congelado"),
        };
        let mut rows = Vec::with_capacity(2);
        for intervention in [Intervention::F, Intervention::X] {
            let actual = model::execute(scenario, intervention);
            let comparison = symbolic::execute(scenario, intervention);
            assert_eq!(actual, comparison, "STOP: divergência entre executores");
            self.log.reproductions += 2;
            self.log.events += actual.processed + comparison.processed;
            assert!(self.log.reproductions <= 544 && self.log.events <= 4352);
            assert!(
                !actual.error
                    && actual.remaining == 0
                    && actual.rejected == 0
                    && actual.rejected_transmissions == 0
                    && !actual.limit_reached
            );
            assert_eq!(actual.after, actual.effective);
            rows.push(actual);
        }
        assert_eq!(rows[0].initial, rows[1].initial);
        assert_eq!(rows[0].original, rows[1].original);
        assert_eq!(rows[0].effective, [1, 0, backup, 0]);
        assert_eq!(rows[1].effective, [0, 0, backup, 0]);
        let id = self.log.packets.len() as u32 + 1;
        let project = |r: &model::Record, blocked: bool| Observation {
            id: 2 * id - u32::from(!blocked),
            initial: r.initial,
            states: r.states,
            action: r.action,
            blocked,
        };
        let packet = Packet {
            id,
            key,
            purpose,
            connection: 1,
            factual: project(&rows[0], false),
            blocked: project(&rows[1], true),
            complete: true,
            comparable: true,
        };
        self.log.packets.push(packet.clone());
        packet
    }
}
fn entry(m: &Learner, key: u16) -> Option<Entry> {
    m.entries.iter().find(|e| e.key == key).copied()
}
fn ingest(
    x: &mut Executor,
    m: &mut Learner,
    no: &mut Learner,
    s: &mut independent::Summary,
    key: u16,
    backup: i32,
) {
    let p = x.pair(key, backup, Purpose::Acquisition);
    let before = entry(m, key);
    let id = p.id;
    let a = Acquisition::checked(p).unwrap();
    s.observe(&a);
    m.acquire(a.clone()).unwrap();
    no.acquire(a).unwrap();
    assert_eq!(m.entries, s.rows, "STOP: divergência de resumos");
    assert!(no.entries.is_empty());
    x.log.updates.push(UpdateLog {
        pair: id,
        key,
        before,
        after: entry(m, key),
    });
    assert!(x.log.updates.len() <= 40);
    x.memory(m, no, s, 0);
}
fn backup(c: Condition, changed: bool) -> i32 {
    if changed && c.category == Category::Changed {
        3 - c.backup
    } else {
        c.backup
    }
}
fn grade(metrics: &mut Metrics, a: Answer, p: &Packet, unknown: bool) {
    let change = p.factual.action != p.blocked.action;
    let expected = if unknown {
        Decision::Investigate
    } else if change {
        Decision::Protect
    } else {
        Decision::Allow
    };
    metrics.correct += u16::from(a.decision == expected);
    match a.decision {
        Decision::Investigate => metrics.abstentions += 1,
        d => {
            metrics.predictions += 1;
            let right = (d == Decision::Protect) == change;
            metrics.predicted_correct += u16::from(right);
            metrics.wrong_allow += u16::from(d == Decision::Allow && change);
            metrics.unneeded_protect += u16::from(d == Decision::Protect && !change);
        }
    }
}
fn evaluate(
    x: &mut Executor,
    m: &Learner,
    no: &Learner,
    s: &independent::Summary,
    phase: u8,
    changed: bool,
) {
    x.guard();
    let before = m.clone();
    let no_before = no.clone();
    let symbolic_before = s.clone();
    let mut scores = [Metrics::default(); 7];
    x.memory(
        m,
        no,
        s,
        before.episodes.capacity() * size_of::<Acquisition>()
            + no_before.episodes.capacity() * size_of::<Acquisition>()
            + (before.entries.capacity()
                + no_before.entries.capacity()
                + symbolic_before.rows.capacity())
                * size_of::<Entry>()
            + size_of::<Packet>() * 3
            + size_of::<[Metrics; 7]>(),
    );
    for c in PANEL {
        // Todas as decisões são produzidas antes do par de avaliação e seu contraste.
        let answers = [
            m.query(c.key, true),
            no.query(c.key, true),
            m.query(c.key, false),
            m.literal(c.key),
            Answer {
                decision: Decision::Allow,
                refs: [None; 2],
                reason: Reason::Fixed,
            },
            Answer {
                decision: Decision::Protect,
                refs: [None; 2],
                reason: Reason::Fixed,
            },
            Answer {
                decision: Decision::Investigate,
                refs: [None; 2],
                reason: Reason::Fixed,
            },
        ];
        assert_eq!(answers[0], s.decide(c.key));
        for a in answers {
            for id in a.refs.into_iter().flatten() {
                assert!(m
                    .episodes
                    .iter()
                    .any(|e| e.packet().id == id && e.packet().key == c.key));
            }
        }
        let p = x.pair(c.key, backup(c, changed), Purpose::Evaluation);
        // A separação é verificável pelo tipo/capacidade: este pacote não pode ser adquirido.
        assert!(Acquisition::checked(p.clone()).is_err());
        for (score, answer) in scores.iter_mut().zip(answers) {
            grade(score, answer, &p, c.category == Category::Unknown);
        }
        x.log.queries.push(QueryLog {
            phase,
            key: c.key,
            answers,
            entry: entry(m, c.key),
            evaluation_pair: p.id,
        });
    }
    assert_eq!(*m, before, "STOP: avaliação alterou aprendiz");
    assert_eq!(*no, no_before);
    assert_eq!(*s, symbolic_before);
    for (control, score) in scores.iter().enumerate() {
        println!("P{phase} control={control} {score:?}");
    }
    x.log.metrics.push(scores);
}
fn sequence() -> ResultLog {
    let mut x = Executor::new();
    let mut m = Learner::new(true);
    let mut no = Learner::new(false);
    let mut s = independent::Summary::new();
    evaluate(&mut x, &m, &no, &s, 0, false);
    for _ in 0..2 {
        x.guard();
        for c in PANEL {
            if c.category != Category::Unknown {
                ingest(&mut x, &mut m, &mut no, &mut s, c.key, c.backup);
            }
        }
    }
    evaluate(&mut x, &m, &no, &s, 1, false);
    let retained: Vec<_> = PANEL.iter().map(|c| entry(&m, c.key)).collect();
    for _ in 0..2 {
        x.guard();
        for (key, b) in [(101, 1), (102, 2), (103, 1), (104, 2)] {
            ingest(&mut x, &mut m, &mut no, &mut s, key, b);
        }
    }
    assert_eq!(
        retained,
        PANEL.iter().map(|c| entry(&m, c.key)).collect::<Vec<_>>()
    );
    evaluate(&mut x, &m, &no, &s, 2, false);
    evaluate(&mut x, &m, &no, &s, 3, true);
    x.guard();
    for c in PANEL {
        if c.category == Category::Changed {
            ingest(&mut x, &mut m, &mut no, &mut s, c.key, backup(c, true));
            assert_eq!(entry(&m, c.key).unwrap().confirmed, None);
        }
    }
    evaluate(&mut x, &m, &no, &s, 4, true);
    x.guard();
    for c in PANEL {
        if c.category == Category::Changed {
            ingest(&mut x, &mut m, &mut no, &mut s, c.key, backup(c, true));
            assert_eq!(
                entry(&m, c.key).unwrap().confirmed,
                Some(backup(c, true) == 1)
            );
        }
    }
    evaluate(&mut x, &m, &no, &s, 5, true);
    assert_eq!(m.episodes.len(), 40);
    assert_eq!(no.episodes.len(), 40);
    assert_eq!(m.entries.len(), 16);
    for a in &m.episodes {
        assert_eq!(a.packet().purpose, Purpose::Acquisition);
    }
    assert_eq!(x.log.packets.len(), 136);
    assert_eq!(x.log.queries.len(), 96);
    assert_eq!(x.log.updates.len(), 40);
    assert_eq!(x.log.reproductions, 544);
    let expected = [
        (4, 0, 0, 0, 0, 16),
        (16, 12, 12, 0, 0, 4),
        (16, 12, 12, 0, 0, 4),
        (12, 12, 8, 2, 2, 4),
        (12, 8, 8, 0, 0, 8),
        (16, 12, 12, 0, 0, 4),
    ];
    for (phase, e) in expected.into_iter().enumerate() {
        let a = x.log.metrics[phase][0];
        assert_eq!(
            (
                a.correct,
                a.predictions,
                a.predicted_correct,
                a.wrong_allow,
                a.unneeded_protect,
                a.abstentions
            ),
            e
        );
        for control in [1, 2, 6] {
            assert_eq!(
                x.log.metrics[phase][control],
                Metrics {
                    correct: 4,
                    abstentions: 16,
                    ..Metrics::default()
                }
            );
        }
        assert_eq!(
            x.log.metrics[phase][4],
            Metrics {
                correct: 6,
                predictions: 16,
                predicted_correct: 8,
                wrong_allow: 8,
                ..Metrics::default()
            }
        );
        assert_eq!(
            x.log.metrics[phase][5],
            Metrics {
                correct: 6,
                predictions: 16,
                predicted_correct: 8,
                unneeded_protect: 8,
                ..Metrics::default()
            }
        );
        let literal = x.log.metrics[phase][3];
        let reference = if phase == 4 {
            x.log.metrics[5][0]
        } else {
            x.log.metrics[phase][0]
        };
        assert_eq!(literal, reference);
    }
    x.log.final_entries = m.entries.clone();
    x.memory(&m, &no, &s, 0);
    x.guard();
    println!("sequence pairs={} reproductions={} events={} peak_accounted_bytes={} elapsed_seconds={:.6}",x.log.packets.len(),x.log.reproductions,x.log.events,x.log.peak_data_bytes,x.started.elapsed().as_secs_f64());
    x.log
}

#[test]
fn development_rejects_invalid_pairs_without_mutation() {
    let mut x = Executor::new();
    let p = x.pair(700, 1, Purpose::Acquisition);
    let mut m = Learner::new(true);
    let before = m.clone();
    let mut bad = p.clone();
    bad.complete = false;
    assert!(Acquisition::checked(bad).is_err());
    assert_eq!(m, before);
    let a = Acquisition::checked(p).unwrap();
    m.acquire(a.clone()).unwrap();
    let after = m.clone();
    assert!(m.acquire(a).is_err());
    assert_eq!(m, after);
}
#[test]
fn validation_confirmation_and_revision_on_separate_keys() {
    let mut x = Executor::new();
    let mut m = Learner::new(true);
    let mut no = Learner::new(false);
    let mut s = independent::Summary::new();
    for key in [801, 802] {
        let b = if key == 801 { 1 } else { 2 };
        ingest(&mut x, &mut m, &mut no, &mut s, key, b);
        assert_eq!(m.query(key, true).decision, Decision::Investigate);
        ingest(&mut x, &mut m, &mut no, &mut s, key, b);
        assert_eq!(
            m.query(key, true).decision,
            if b == 1 {
                Decision::Protect
            } else {
                Decision::Allow
            }
        );
        ingest(&mut x, &mut m, &mut no, &mut s, key, 3 - b);
        assert_eq!(m.query(key, true).decision, Decision::Investigate);
        ingest(&mut x, &mut m, &mut no, &mut s, key, 3 - b);
        assert_eq!(
            m.query(key, true).decision,
            if b == 1 {
                Decision::Allow
            } else {
                Decision::Protect
            }
        );
    }
}
#[test]
fn frozen_acquisition_retention_revision_panel() {
    let first = sequence();
    let second = sequence();
    assert_eq!(first, second, "STOP: repetição divergente");
    assert_eq!(first.reproductions + second.reproductions, 1088);
    assert!(first.events + second.events <= 8704);
    println!(
        "EC-02.4-R2 repeat_equal=true total_reproductions=1088 total_events={} event_ceiling=8704",
        first.events + second.events
    );
}
