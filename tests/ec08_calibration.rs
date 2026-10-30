// Reuse the unchanged experimental fixtures and independent symbolic control.
include!("ec08_decisions.rs");
fn quality_panel(offset: u32, reps: usize, final_run: bool) {
    let start = std::time::Instant::now();
    let mut previous = None;
    for rep in 0..reps {
        let mut cs = cases();
        let mut unverified = cs[1].clone();
        unverified.name = "sem_referencia";
        cs.push(unverified);
        let mut rows = Vec::new();
        let (mut mtime, mut ctime) = (0u128, 0u128);
        let (mut events, mut attempts) = (0usize, 0usize);
        let (mut good, mut wrong, mut no_ref, mut responses, mut revisions, mut abstain) =
            (0, 0, 0, 0, 0, 0);
        for c in cs {
            let s = store(&c, offset);
            let before: Vec<_> = s.iter().collect();
            let q = Hypothesis::new(1, node(1), node(3)).unwrap();
            let monitor = Monitor::new(TwoHopTask::new(16, 32, 64).unwrap(), c.budget).unwrap();
            let t = std::time::Instant::now();
            let report = monitor.review(&s, q, &c.candidates).unwrap();
            mtime += t.elapsed().as_nanos();
            let t = std::time::Instant::now();
            let control = symbol(&c);
            ctime += t.elapsed().as_nanos();
            assert_eq!(before, s.iter().collect::<Vec<_>>());
            let view = match report.decision {
                Decision::Respond { hypothesis, .. } => {
                    responses += 1;
                    if hypothesis.context == 1 {
                        View::Original
                    } else {
                        revisions += 1;
                        View::Revised
                    }
                }
                Decision::Abstain(AbstainReason::MissingEvidence) => {
                    abstain += 1;
                    View::Missing
                }
                Decision::Abstain(AbstainReason::ContradictoryEvidence) => {
                    abstain += 1;
                    View::Conflict
                }
                Decision::Abstain(AbstainReason::MultipleSupportedContexts) => {
                    abstain += 1;
                    View::Multiple
                }
                Decision::Abstain(AbstainReason::ReviewBudget) => {
                    abstain += 1;
                    View::Budget
                }
                x => panic!("unexpected {x:?}"),
            };
            assert_eq!(view, control);
            if c.name == "sem_referencia" {
                no_ref += 1
            } else if view == c.expected {
                good += 1
            } else {
                wrong += 1
            }
            attempts += report.attempt_count();
            events += report
                .attempts
                .iter()
                .filter_map(|x| x.inference.map(|v| v.processed_events))
                .sum::<usize>();
            if final_run {
                println!("QUALITY rep={rep} case={} view={view:?} control={control:?} expected={} report={report:?}",c.name,if c.name=="sem_referencia" {"none"}else{"synthetic"});
            }
            rows.push((c.name, view, control, report));
        }
        assert_eq!((good, wrong, no_ref), (14, 0, 1));
        assert_eq!((responses, revisions, abstain), (3, 1, 12));
        if let Some(old) = &previous {
            assert_eq!(old, &rows)
        }
        previous = Some(rows);
        println!("QUALITY_SUM rep={rep} correct={good} wrong={wrong} no_ref={no_ref} responses={responses} revisions={revisions} abstain={abstain} attempts={attempts} events={events} monitor_ns={mtime} control_ns={ctime}");
        assert!(start.elapsed().as_secs_f64() < 30.0);
        let st = std::fs::read_to_string("/proc/self/status").unwrap();
        let h: usize = st
            .lines()
            .find(|x| x.starts_with("VmHWM:"))
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        assert!(h < 256 * 1024);
        let o = std::process::Command::new("df")
            .args(["-B1", "--output=avail", "/home"])
            .output()
            .unwrap();
        let free: u64 = String::from_utf8(o.stdout)
            .unwrap()
            .lines()
            .last()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        assert!(free >= 3 * 1024 * 1024 * 1024);
    }
    println!("QUALITY_SECONDS {:.6}", start.elapsed().as_secs_f64());
}
#[test]
fn quality_development() {
    quality_panel(400, 1, false)
}
#[test]
fn quality_validation() {
    quality_panel(500, 1, false)
}
#[test]
fn quality_frozen_final() {
    quality_panel(600, 2, true)
}
