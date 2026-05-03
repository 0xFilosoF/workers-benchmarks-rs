mod common;

use {
    criterion::{BenchmarkId, Criterion, criterion_group, criterion_main},
    workers_benchmarks_rs::{BenchConfig, WorkProfile, run_sync_thread_workers},
};

fn bench_sync_kanal_threads(c: &mut Criterion) {
    let mut group = c.benchmark_group("sync_kanal_threads");

    for profile in WorkProfile::ALL {
        let config = BenchConfig::for_profile(profile);

        group.bench_with_input(
            BenchmarkId::new(profile.name(), config.work_iters),
            &config,
            |bench, config| {
                bench.iter(|| run_sync_thread_workers(*config));
            },
        );
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = common::criterion_config();
    targets = bench_sync_kanal_threads
}
criterion_main!(benches);
