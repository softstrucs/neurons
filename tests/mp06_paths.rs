//! Cenários públicos próprios MP06, sem fixtures ou avaliadores científicos.
use neurons::cognitive_contracts::*;
use neurons::engram::*;
use neurons::event::NodeId;
use neurons::integration::query::*;
fn n(v: u32) -> NodeId {
    NodeId::new(v).unwrap()
}
fn id(v: u32) -> EngramId {
    EngramId::new(v).unwrap()
}
fn scope(ctx: u32) -> Scope {
    Scope {
        catalog: CatalogId::new(71).unwrap(),
        context: ContextId::new(ctx).unwrap(),
        rule: RuleRef {
            id: RuleId::new(81).unwrap(),
            version: RuleVersion::new(1).unwrap(),
        },
        validity: Validity::new(0, 100).unwrap(),
    }
}
fn stmt(a: u32, b: u32, positive: bool, s: Scope) -> Statement {
    let c = |v| Concept::new(n(v), s.catalog, "publico", "símbolo de teste público").unwrap();
    Statement::inclusion(
        &c(a),
        &c(b),
        if positive {
            Polarity::Positive
        } else {
            Polarity::Negative
        },
        s,
    )
    .unwrap()
}
struct Fixture {
    memory: EngramStore,
    metadata: Vec<RecordMetadata>,
}
impl Fixture {
    fn new(edges: &[(u32, u32, u32, bool)]) -> Self {
        let mut f = Self {
            memory: EngramStore::new(8).unwrap(),
            metadata: vec![],
        };
        for &(i, a, b, pos) in edges {
            let evidence = Some(EvidenceId::new(1000 + i).unwrap());
            f.memory
                .record(
                    Engram::new(
                        id(i),
                        Cue::new(61, n(a), b as i32).unwrap(),
                        if pos { 1 } else { -1 },
                        None,
                        1,
                        evidence,
                        RecordStatus::Observed,
                    )
                    .unwrap(),
                )
                .unwrap();
            f.metadata.push(RecordMetadata {
                id: id(i),
                information: Information {
                    statement: stmt(a, b, pos, scope(61)),
                    declared_source: Some(SourceId::new(7).unwrap()),
                    evidence,
                    nature: DeclaredNature::Observation,
                    received_at: 0,
                },
                state: EpistemicState::AdmittedObservation,
                disposition: None,
            });
        }
        f
    }
    fn coordinator(&self) -> QueryCoordinator<'_> {
        QueryCoordinator::new(
            &self.memory,
            &self.metadata,
            RuleAuthorization::inclusion_in(scope(61)),
            &[SourceId::new(7).unwrap()],
        )
        .unwrap()
    }
    fn run(&self, a: u32, b: u32, budget: usize) -> PathQueryReport {
        self.coordinator()
            .query_three(
                EpisodeId::new(1).unwrap(),
                stmt(a, b, true, scope(61)),
                5,
                PathAuthorization::inclusion_three(scope(61)),
                budget,
                &mut PathEpisodeLog::default(),
            )
            .unwrap()
    }
}
fn ids(p: &PathProof) -> Vec<u32> {
    p.premises.iter().map(|r| r.engram.id().get()).collect()
}
#[test]
fn direct_two_three_and_no_fact_promotion() {
    for edges in [
        vec![(11, 101, 109, true)],
        vec![(11, 101, 103, true), (12, 103, 109, true)],
        vec![
            (11, 101, 103, true),
            (12, 103, 107, true),
            (13, 107, 109, true),
        ],
    ] {
        let f = Fixture::new(&edges);
        let before = f.memory.iter().collect::<Vec<_>>();
        let metadata = f.metadata.clone();
        let r = f.run(101, 109, 64);
        assert!(r.refusal.is_none());
        assert_eq!(r.proof.as_ref().unwrap().premises.len(), edges.len());
        assert_eq!(r.before, r.after);
        assert_eq!(r.posterior, EpistemicState::ProvisionalKnowledge);
        assert_eq!(before, f.memory.iter().collect::<Vec<_>>());
        assert_eq!(metadata, f.metadata);
        assert!(r.review.unwrap().inference.processed_events <= 64);
    }
}
#[test]
fn alternatives_shortest_first_then_canonical_and_order_independent() {
    let edges = [
        (19, 101, 103, true),
        (18, 103, 109, true),
        (11, 101, 107, true),
        (12, 107, 109, true),
        (13, 107, 113, true),
        (14, 113, 109, true),
    ];
    let f = Fixture::new(&edges);
    let r = f.run(101, 109, 64);
    assert_eq!(ids(r.proof.as_ref().unwrap()), vec![11, 12]);
    assert_eq!(r.alternatives.len(), 3);
    let reversed = Fixture::new(&edges.into_iter().rev().collect::<Vec<_>>());
    let q = reversed.run(101, 109, 64);
    assert_eq!(r.alternatives, q.alternatives);
    assert_eq!(r.proof, q.proof);
}
#[test]
fn opposition_on_nonselected_chain_and_conclusion_is_never_hidden() {
    for edges in [
        vec![
            (11, 101, 109, true),
            (12, 101, 103, true),
            (13, 103, 109, true),
            (14, 103, 109, false),
        ],
        vec![
            (11, 101, 103, true),
            (12, 103, 107, true),
            (13, 107, 109, true),
            (14, 101, 109, false),
        ],
    ] {
        let r = Fixture::new(&edges).run(101, 109, 64);
        assert!(r.proof.is_none());
        assert_eq!(
            r.refusal,
            Some(Refusal::Monitor(
                neurons::metacognition::AbstainReason::ContradictoryEvidence
            ))
        );
        assert!(!r.review.unwrap().conflicts.is_empty());
        assert!(!r.alternatives.is_empty());
    }
}
#[test]
fn malformed_disconnected_missing_negative_and_repeated_proofs_rejected() {
    let f = Fixture::new(&[
        (11, 101, 103, true),
        (12, 103, 107, true),
        (13, 107, 109, true),
        (14, 113, 109, true),
        (15, 103, 109, false),
    ]);
    let c = f.coordinator();
    let q = stmt(101, 109, true, scope(61));
    for p in [
        vec![],
        vec![id(11), id(14)],
        vec![id(11), id(12)],
        vec![id(11), id(15)],
        vec![id(11), id(11), id(13)],
        vec![id(999)],
        vec![id(11), id(12), id(13), id(14)],
    ] {
        assert!(c.verify_path(q, 5, &p).is_err());
    }
    assert!(c
        .verify_path(
            stmt(101, 109, false, scope(61)),
            5,
            &[id(11), id(12), id(13)]
        )
        .is_err());
    assert_eq!(
        ids(&c.verify_path(q, 5, &[id(11), id(12), id(13)]).unwrap()),
        vec![11, 12, 13]
    );
}
#[test]
fn received_hypothesis_quarantine_and_unknown_sources_are_excluded() {
    for mode in 0..5 {
        let mut f = Fixture::new(&[
            (11, 101, 103, true),
            (12, 103, 107, true),
            (13, 107, 109, true),
        ]);
        match mode {
            0 => f.metadata[1].state = EpistemicState::Received,
            1 => f.metadata[1].state = EpistemicState::Hypothesis,
            2 => f.metadata[1].disposition = Some(Disposition::Quarantined),
            3 => f.metadata[1].information.declared_source = Some(SourceId::new(99).unwrap()),
            _ => f.metadata[1].information.nature = DeclaredNature::Hypothesis,
        }
        let r = f.run(101, 109, 64);
        assert!(r.proof.is_none());
        assert_eq!(r.excluded.len(), 1);
        assert!(r.consulted.iter().all(|b| b.engram.id() != id(12)));
    }
}
#[test]
fn explicit_scope_authorization_and_validity_fail_closed() {
    let f = Fixture::new(&[
        (11, 101, 103, true),
        (12, 103, 107, true),
        (13, 107, 109, true),
    ]);
    let c = f.coordinator();
    let r = c
        .query_three(
            EpisodeId::new(1).unwrap(),
            stmt(101, 109, true, scope(61)),
            5,
            PathAuthorization::inclusion_three(scope(62)),
            64,
            &mut PathEpisodeLog::default(),
        )
        .unwrap();
    assert_eq!(r.refusal, Some(Refusal::RuleOutsideContext));
    assert!(r.review.is_none());
    let r = c
        .query_three(
            EpisodeId::new(1).unwrap(),
            stmt(101, 109, true, scope(62)),
            5,
            PathAuthorization::inclusion_three(scope(62)),
            64,
            &mut PathEpisodeLog::default(),
        )
        .unwrap();
    assert_eq!(r.refusal, Some(Refusal::RuleOutsideContext));
    assert!(c
        .query_three(
            EpisodeId::new(1).unwrap(),
            stmt(101, 109, true, scope(61)),
            101,
            PathAuthorization::inclusion_three(scope(61)),
            64,
            &mut PathEpisodeLog::default()
        )
        .is_err());
    assert!(c
        .verify_path(
            stmt(101, 109, true, scope(61)),
            101,
            &[id(11), id(12), id(13)]
        )
        .is_err());
}
#[test]
fn insufficient_four_premises_cycles_and_event_budget_abstain() {
    for edges in [
        vec![(11, 101, 103, true), (12, 107, 109, true)],
        vec![
            (11, 101, 103, true),
            (12, 103, 107, true),
            (13, 107, 113, true),
            (14, 113, 109, true),
        ],
    ] {
        let r = Fixture::new(&edges).run(101, 109, 64);
        assert!(r.proof.is_none());
        assert_eq!(
            r.refusal,
            Some(Refusal::Monitor(
                neurons::metacognition::AbstainReason::MissingEvidence
            ))
        );
    }
    let f = Fixture::new(&[
        (11, 101, 103, true),
        (12, 103, 107, true),
        (13, 107, 109, true),
    ]);
    for budget in [0, 1, 65] {
        let r = f.run(101, 109, budget);
        assert!(r.proof.is_none());
        assert_eq!(
            r.refusal,
            Some(Refusal::Monitor(
                neurons::metacognition::AbstainReason::ResourceLimited
            ))
        );
    }
    let cyc = Fixture::new(&[
        (11, 101, 103, true),
        (12, 103, 101, true),
        (13, 103, 107, true),
        (14, 107, 109, true),
    ]);
    let r = cyc.run(101, 109, 64);
    assert!(r.proof.is_none());
    assert_eq!(
        r.refusal,
        Some(Refusal::Monitor(
            neurons::metacognition::AbstainReason::ResourceLimited
        ))
    );
}
#[test]
fn log_limit_duplicate_and_chronology_preserve_prefix() {
    let f = Fixture::new(&[(11, 101, 109, true)]);
    let c = f.coordinator();
    let mut log = PathEpisodeLog::default();
    for i in 1..=3 {
        c.query_three(
            EpisodeId::new(i).unwrap(),
            stmt(101, 109, true, scope(61)),
            i as u64,
            PathAuthorization::inclusion_three(scope(61)),
            64,
            &mut log,
        )
        .unwrap();
    }
    let before = log.entries().to_vec();
    assert!(c
        .query_three(
            EpisodeId::new(4).unwrap(),
            stmt(101, 109, true, scope(61)),
            4,
            PathAuthorization::inclusion_three(scope(61)),
            64,
            &mut log
        )
        .is_err());
    assert_eq!(before, log.entries());
}
#[test]
fn independent_symbolic_reference_agrees_on_public_cases() {
    // Referência por multiplicação booleana explícita de matrizes, sem usar motor/prova do candidato.
    let cases = [
        vec![(11, 101, 109, true)],
        vec![(11, 101, 103, true), (12, 103, 109, true)],
        vec![
            (11, 101, 103, true),
            (12, 103, 107, true),
            (13, 107, 109, true),
        ],
        vec![(11, 101, 103, true), (12, 107, 109, true)],
    ];
    let nodes = [101, 103, 107, 109];
    for edges in cases {
        let mut adjacency = [[false; 4]; 4];
        for &(_, a, b, pos) in &edges {
            if pos {
                adjacency[nodes.iter().position(|n| *n == a).unwrap()]
                    [nodes.iter().position(|n| *n == b).unwrap()] = true;
            }
        }
        let mut power = adjacency;
        let mut expected = adjacency[0][3];
        for _ in 2..=3 {
            let mut next = [[false; 4]; 4];
            for a in 0..4 {
                for b in 0..4 {
                    next[a][b] = (0..4).any(|k| power[a][k] && adjacency[k][b]);
                }
            }
            power = next;
            expected |= power[0][3];
        }
        assert_eq!(
            Fixture::new(&edges).run(101, 109, 64).proof.is_some(),
            expected
        );
    }
}
#[test]
fn session_extension_is_transient_and_does_not_change_f1_checkpoint() {
    use neurons::session::f1::{run, Session};
    use std::io::Cursor;
    let setup="config 71 61 81 1 0 1000\nsimbolo 101 a\nsimbolo 103 b\nsimbolo 107 c\nsimbolo 109 d\nautorizar 61\nreceber 11 61 101 103 + 7 1011 observacao\ninterpretar\navaliar\nadmitir\nreceber 12 61 103 107 + 7 1012 observacao\ninterpretar\navaliar\nadmitir\nreceber 13 61 107 109 + 7 1013 observacao\ninterpretar\navaliar\nadmitir\n";
    let mut s = Session::new();
    for line in setup.lines() {
        s.execute(line).unwrap();
    }
    let bytes = s.encode();
    let snapshot = s.snapshot();
    let auth = PathAuthorization::inclusion_three(Scope {
        validity: Validity::new(0, 1000).unwrap(),
        ..scope(61)
    });
    assert!(s
        .query_three("61", "101", "109", auth, &mut PathEpisodeLog::default())
        .unwrap()
        .proof
        .is_some());
    let mut output = vec![];
    run(
        Cursor::new(format!(
            "{setup}autorizar3 61\nconsultar3 61 101 109\nsair\n"
        )),
        &mut output,
    )
    .unwrap();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("PathProof"));
    assert!(text.contains("EngramId(13)"));
    assert!(!text.contains("Erro:"));
    assert_eq!(bytes, s.encode());
    assert_eq!(snapshot, s.snapshot());
    assert_eq!(snapshot, Session::decode(&bytes).unwrap().snapshot());
}

#[test]
fn missing_evidence_future_record_and_rule_version_are_individually_rejected() {
    for mode in 0..3 {
        let mut f = Fixture::new(&[
            (11, 101, 103, true),
            (12, 103, 107, true),
            (13, 107, 109, true),
        ]);
        if mode == 0 {
            f.metadata[1].information.evidence = None;
        }
        if mode == 2 {
            f.metadata[1].information.statement = stmt(
                103,
                107,
                true,
                Scope {
                    rule: RuleRef {
                        id: RuleId::new(81).unwrap(),
                        version: RuleVersion::new(2).unwrap(),
                    },
                    ..scope(61)
                },
            );
        }
        let mut memory = EngramStore::new(8).unwrap();
        for r in f.memory.iter() {
            memory
                .record(
                    Engram::new(
                        r.id(),
                        r.cue(),
                        r.value(),
                        r.origin(),
                        if mode == 1 && r.id() == id(12) {
                            10
                        } else {
                            r.observed_at()
                        },
                        if mode == 0 && r.id() == id(12) {
                            None
                        } else {
                            r.evidence()
                        },
                        r.status(),
                    )
                    .unwrap(),
                )
                .unwrap();
        }
        f.memory = memory;
        let r = f.run(101, 109, 64);
        assert!(r.proof.is_none());
        assert_eq!(r.excluded.len(), 1);
        assert!(f
            .coordinator()
            .verify_path(
                stmt(101, 109, true, scope(61)),
                5,
                &[id(11), id(12), id(13)]
            )
            .is_err());
    }
}
#[test]
fn duplicate_and_past_episode_errors_preserve_log_and_eight_facts_are_bounded() {
    let f = Fixture::new(&[
        (11, 101, 103, true),
        (12, 103, 107, true),
        (13, 107, 109, true),
        (14, 113, 127, true),
        (15, 127, 131, true),
        (16, 131, 137, true),
        (17, 137, 139, true),
        (18, 139, 149, true),
    ]);
    let c = f.coordinator();
    let mut log = PathEpisodeLog::default();
    c.query_three(
        EpisodeId::new(1).unwrap(),
        stmt(101, 109, true, scope(61)),
        5,
        PathAuthorization::inclusion_three(scope(61)),
        64,
        &mut log,
    )
    .unwrap();
    let prefix = log.entries().to_vec();
    for (i, at) in [(1, 5), (2, 4)] {
        assert!(c
            .query_three(
                EpisodeId::new(i).unwrap(),
                stmt(101, 109, true, scope(61)),
                at,
                PathAuthorization::inclusion_three(scope(61)),
                64,
                &mut log
            )
            .is_err());
        assert_eq!(prefix, log.entries());
    }
    assert!(prefix[0].proof.is_some());
    assert!(
        prefix[0]
            .review
            .as_ref()
            .unwrap()
            .inference
            .processed_events
            <= 64
    );
}

#[test]
fn three_premises_exceed_literal_and_two_hop_controls_without_changing_them() {
    let f = Fixture::new(&[
        (11, 101, 103, true),
        (12, 103, 107, true),
        (13, 107, 109, true),
    ]);
    assert!(f.memory.recall(Cue::new(61, n(101), 109).unwrap()).is_err());
    let legacy = neurons::cognition::TwoHopTask::new(8, 16, 64)
        .unwrap()
        .evaluate(
            &f.memory,
            neurons::cognition::Hypothesis::new(61, n(101), n(109)).unwrap(),
        )
        .unwrap();
    assert_eq!(legacy.outcome, neurons::cognition::Outcome::Unsupported);
    assert_eq!(f.run(101, 109, 64).proof.unwrap().premises.len(), 3);
}
