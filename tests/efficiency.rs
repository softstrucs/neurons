#[path = "../examples/support/profile.rs"]
mod profile;

#[test]
fn bounded_profiles_preserve_functional_equivalence() {
    for units in [10, 100, 1000] {
        let sample = profile::run(units).unwrap();
        assert_eq!(sample.units, units);
        assert_eq!(sample.dispatches, profile::ROUNDS as u64);
        assert_eq!(sample.activations, profile::ROUNDS as u64);
        assert_eq!(sample.chain_events, 10 * units);
        assert_eq!(sample.final_weight, 1);
        assert_eq!(sample.recovered, units);
        // Durées não são critérios de correção; ler todos os campos do relatório.
        std::hint::black_box((
            sample.addressed_ns,
            sample.scan_ns,
            sample.chain_ns,
            sample.update_ns,
            sample.record_ns,
            sample.recall_ns,
            sample.rss_before_kib,
            sample.rss_loaded_kib,
            sample.rss_after_kib,
            sample.rss_peak_kib,
        ));
    }
}

#[test]
fn rejects_unbounded_or_unsupported_profiles() {
    for units in [0, 1, 11, 1001, usize::MAX] {
        assert!(profile::run(units).is_err());
    }
}
