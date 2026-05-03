use tokio::task::JoinSet;

use crate::{
    config::BenchConfig,
    work::{Job, cpu_work, job_for_index},
};

pub async fn run_async_tokio_workers(config: BenchConfig) -> u64 {
    let (job_tx, job_rx) = kanal::bounded_async::<Job>(config.channel_capacity);
    let (result_tx, result_rx) = kanal::bounded_async::<u64>(config.channel_capacity);
    let mut workers = JoinSet::new();

    for _ in 0..config.workers {
        let job_rx = job_rx.clone();
        let result_tx = result_tx.clone();

        workers.spawn(async move {
            while let Ok(job) = job_rx.recv().await {
                let result = cpu_work(job.seed, job.iters);

                if result_tx.send(result).await.is_err() {
                    break;
                }
            }
        });
    }

    drop(job_rx);
    drop(result_tx);

    let jobs = config.jobs;
    let collector = tokio::spawn(async move {
        let mut checksum = 0_u64;

        for _ in 0..jobs {
            checksum ^= result_rx
                .recv()
                .await
                .expect("async workers must send one result per job");
        }

        checksum
    });

    for index in 0..config.jobs {
        job_tx
            .send(job_for_index(index, config.work_iters))
            .await
            .expect("async workers must keep receiving jobs");
    }

    drop(job_tx);

    let checksum = collector.await.expect("collector task panicked");

    while let Some(result) = workers.join_next().await {
        result.expect("async worker task panicked");
    }

    checksum
}

pub async fn run_mixed_sync_workers_tokio_collector(config: BenchConfig) -> u64 {
    let (job_tx, job_rx) = kanal::bounded::<Job>(config.channel_capacity);
    let (result_tx, result_rx) = kanal::bounded_async::<u64>(config.channel_capacity);
    let mut workers = Vec::with_capacity(config.workers);

    for _ in 0..config.workers {
        let job_rx = job_rx.clone();
        let result_tx = result_tx.clone_sync();

        workers.push(std::thread::spawn(move || {
            while let Ok(job) = job_rx.recv() {
                let result = cpu_work(job.seed, job.iters);

                if result_tx.send(result).is_err() {
                    break;
                }
            }
        }));
    }

    drop(job_rx);
    drop(result_tx);

    let jobs = config.jobs;
    let collector = tokio::spawn(async move {
        let mut checksum = 0_u64;

        for _ in 0..jobs {
            checksum ^= result_rx
                .recv()
                .await
                .expect("sync workers must send one result per job");
        }

        checksum
    });

    // or std::thread::spawn
    let producer = tokio::task::spawn_blocking(move || {
        for index in 0..config.jobs {
            job_tx
                .send(job_for_index(index, config.work_iters))
                .expect("sync workers must keep receiving jobs");
        }
    });

    for worker in workers {
        worker.join().expect("sync worker thread panicked");
    }

    producer.await.expect("producer thread panicked");

    collector.await.expect("collector task panicked")
}
