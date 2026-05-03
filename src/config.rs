#[derive(Debug, Clone, Copy)]
pub struct BenchConfig {
    pub workers: usize,
    pub jobs: usize,
    pub work_iters: u64,
    pub channel_capacity: usize,
}

const DEFAULT_JOBS: usize = 10_000;

impl BenchConfig {
    pub fn new(workers: usize, jobs: usize, work_iters: u64, channel_capacity: usize) -> Self {
        assert!(workers > 0, "workers must be greater than zero");
        assert!(jobs > 0, "jobs must be greater than zero");

        Self {
            workers,
            jobs,
            work_iters,
            channel_capacity,
        }
    }

    pub fn for_profile(profile: WorkProfile) -> Self {
        let workers = num_cpus::get().max(1);
        Self::new(workers, profile.jobs(), profile.work_iters(), workers * 4)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WorkProfile {
    Tiny,
    Medium,
    Large,
}

impl WorkProfile {
    pub const ALL: [Self; 3] = [Self::Tiny, Self::Medium, Self::Large];

    pub fn name(self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }

    pub fn jobs(self) -> usize {
        DEFAULT_JOBS
    }

    pub fn work_iters(self) -> u64 {
        match self {
            Self::Tiny => 4,
            Self::Medium => 512,
            Self::Large => 8_192,
        }
    }
}
