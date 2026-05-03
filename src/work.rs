#[derive(Debug, Clone, Copy)]
pub struct Job {
    pub seed: u64,
    pub iters: u64,
}

pub fn job_for_index(index: usize, work_iters: u64) -> Job {
    Job {
        seed: index as u64 ^ 0x9e37_79b9_7f4a_7c15,
        iters: work_iters,
    }
}

pub fn cpu_work(mut x: u64, iters: u64) -> u64 {
    for _ in 0..iters {
        x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        x ^= x >> 33;
    }

    std::hint::black_box(x)
}
