//! cargo run --example cpu_profile --release -- 10 (também 100 ou 1000).
#[path = "support/profile.rs"]
mod profile;

fn main() -> std::process::ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let units = match args.as_slice() {
        [value] => value.parse().ok(),
        _ => None,
    };
    let Some(units) = units else {
        eprintln!("Uso: cpu_profile [10|100|1000]");
        return std::process::ExitCode::FAILURE;
    };
    match profile::run(units) {
        Ok(sample) => {
            println!("{sample:?}");
            println!("units={} rounds={} dispatches={} activations={} weight={} recovered={} addressed_ns_per_event={} addressed_events_per_second={:.0} scan_ns_per_event={} chain_ns_per_event={} update_ns_per_call={} record_ns_per_item={} recall_ns_per_query={} rss_kib={:?}/{:?}/{:?} peak_kib={:?}",sample.units,profile::ROUNDS,sample.dispatches,sample.activations,sample.final_weight,sample.recovered,sample.addressed_ns/profile::ROUNDS as u128,profile::ROUNDS as f64*1e9/sample.addressed_ns.max(1) as f64,sample.scan_ns/profile::ROUNDS as u128,sample.chain_ns/sample.chain_events as u128,sample.update_ns/profile::ROUNDS as u128,sample.record_ns/sample.units as u128,sample.recall_ns/sample.units as u128,sample.rss_before_kib,sample.rss_loaded_kib,sample.rss_after_kib,sample.rss_peak_kib);
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}
