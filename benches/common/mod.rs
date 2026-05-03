use {criterion::Criterion, std::time::Duration};

pub fn criterion_config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(20))
        .sample_size(50)
}
