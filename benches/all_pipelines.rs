mod common;

use {
    criterion::{BenchmarkId, Criterion, criterion_group, criterion_main},
    workers_benchmarks_rs::{
        BenchConfig, WorkProfile, run_async_tokio_workers, run_mixed_sync_workers_tokio_collector,
        run_sync_thread_workers,
    },
};

fn bench_all_pipelines(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("tokio runtime must start");

    for profile in WorkProfile::ALL {
        let config = BenchConfig::for_profile(profile);
        let mut group = c.benchmark_group(format!("pipeline_comparison/{}", profile.name()));
        group.throughput(criterion::Throughput::Elements(config.jobs as u64));

        group.bench_with_input(
            BenchmarkId::new("async_kanal_tokio", config.work_iters),
            &config,
            |bench, config| {
                bench
                    .to_async(&runtime)
                    .iter(|| run_async_tokio_workers(*config));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("sync_kanal_threads", config.work_iters),
            &config,
            |bench, config| {
                bench.iter(|| run_sync_thread_workers(*config));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("mixed_sync_workers_tokio_collector", config.work_iters),
            &config,
            |bench, config| {
                bench
                    .to_async(&runtime)
                    .iter(|| run_mixed_sync_workers_tokio_collector(*config));
            },
        );

        group.finish();
    }
}

criterion_group! {
    name = benches;
    config = common::criterion_config();
    targets = bench_all_pipelines
}
criterion_main!(benches);
