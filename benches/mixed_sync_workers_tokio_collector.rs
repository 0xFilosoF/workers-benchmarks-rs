mod common;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use workers_benchmarks_rs::{BenchConfig, WorkProfile, run_mixed_sync_workers_tokio_collector};

fn bench_mixed_sync_workers_tokio_collector(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("tokio runtime must start");
    let mut group = c.benchmark_group("mixed_sync_workers_tokio_collector");

    for profile in WorkProfile::ALL {
        let config = BenchConfig::for_profile(profile);

        group.bench_with_input(
            BenchmarkId::new(profile.name(), config.work_iters),
            &config,
            |bench, config| {
                bench
                    .to_async(&runtime)
                    .iter(|| run_mixed_sync_workers_tokio_collector(*config));
            },
        );
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = common::criterion_config();
    targets = bench_mixed_sync_workers_tokio_collector
}
criterion_main!(benches);
