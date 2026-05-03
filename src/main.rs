use workers_benchmarks_rs::{
    BenchConfig, WorkProfile, run_async_tokio_workers, run_mixed_sync_workers_tokio_collector,
    run_sync_thread_workers,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    for profile in WorkProfile::ALL {
        let config = BenchConfig::for_profile(profile);

        let async_checksum = run_async_tokio_workers(config).await;
        let sync_checksum = run_sync_thread_workers(config);
        let mixed_checksum = run_mixed_sync_workers_tokio_collector(config).await;

        println!(
            "{profile:?}: async={async_checksum:#x}, sync={sync_checksum:#x}, mixed={mixed_checksum:#x}"
        );
    }
}
