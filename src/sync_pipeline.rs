use crate::{
    config::BenchConfig,
    work::{Job, cpu_work, job_for_index},
};

pub fn run_sync_thread_workers(config: BenchConfig) -> u64 {
    let (job_tx, job_rx) = kanal::bounded::<Job>(config.channel_capacity);
    let (result_tx, result_rx) = kanal::bounded::<u64>(config.channel_capacity);
    let mut workers = Vec::with_capacity(config.workers);

    for _ in 0..config.workers {
        let job_rx = job_rx.clone();
        let result_tx = result_tx.clone();

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

    let producer = std::thread::spawn(move || {
        for index in 0..config.jobs {
            job_tx
                .send(job_for_index(index, config.work_iters))
                .expect("sync workers must keep receiving jobs");
        }
    });

    let mut checksum = 0_u64;

    for _ in 0..config.jobs {
        checksum ^= result_rx
            .recv()
            .expect("sync workers must send one result per job");
    }

    producer.join().expect("producer thread panicked");

    for worker in workers {
        worker.join().expect("sync worker thread panicked");
    }

    checksum
}
