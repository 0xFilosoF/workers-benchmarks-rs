use {criterion::Criterion, std::time::Duration};

pub fn criterion_config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(30))
        .warm_up_time(Duration::from_secs(5))
        .sample_size(50)
}
