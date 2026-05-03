# Workers benchmarks, async vs sync for not I/O

A showcase repository for CPU-only workloads without I/O: long-lived workers receive jobs through `kanal`, execute synchronous CPU work, and return results. The goal is not to prove that async is "slow", but to isolate the cost of channels and scheduling from the cost of the work itself.

### What is being compared

- `async_kanal_tokio`: async `kanal::bounded_async`, workers are spawned with `tokio::spawn`, and the CPU loop runs directly inside the async task
- `sync_kanal_threads`: sync `kanal::bounded`, workers are dedicated `std::thread`
- `mixed_sync_workers_tokio_collector`: workers remain synchronous threads, while result collection is performed by an async Tokio task

### Workload profiles

All profiles use the same number of jobs. Only `work_iters` changes, so the comparison keeps the same queue length while varying the processing cost of a single job.

- `tiny`: almost no useful CPU work, making `.await` overhead, wakeups, executor scheduling, and channel costs much more visible.
- `medium`: channel overhead becomes less dominant, and the real CPU loop cost starts to matter
- `large`: CPU work dominates execution time, making the worker execution model more important than the micro-cost of sending messages

### Running

```bash
make run
make bench
```

Criterion is configured with `20s` measurement time, `3s` warmup, and `50` samples. A combined benchmark group is used to simplify visual comparison between implementations.

After running benchmarks, open: `target/criterion/report/index.html` or profile-specific grouped reports: `target/criterion/pipeline_comparison_<profile>/report/index.html`. In grouped reports, all implementations for the same workload profile are displayed side-by-side, making the throughput and latency leader immediately visible.

## Interpretation

For CPU-only workloads without I/O, using async does not provide an inherent advantage. Async becomes valuable when tasks spend significant time waiting on I/O. If workers are continuously executing CPU-bound work, dedicated threads with synchronous channels are often a simpler and more predictable execution model.

Using `tokio::spawn` for CPU-bound workers may appear acceptable on an otherwise idle runtime, but it becomes risky in real applications: a CPU loop without `.await` or explicit yielding occupies an executor worker thread and can negatively affect the latency of unrelated async tasks.

When CPU-bound work must coexist with Tokio, it is usually better to consider:
- a dedicated thread pool
- `spawn_blocking` with explicit concurrency limits
- or a separate synchronous worker layer integrated with the async runtime

| Scenario | Recommended Model | Why |
|---|---|---|
| High I/O wait, many idle tasks | Tokio + async channels | Async scales well when tasks spend most time waiting |
| CPU-bound long-lived workers | Dedicated threads + sync channels | Predictable scheduling, no executor contention |
| Tiny jobs with minimal CPU work | Async coroutine often win | Sync overhead becomes dominant |
| Medium CPU workloads | Depends on contention and worker count | Channel overhead matters less |
| Large CPU workloads | Dedicated threads usually win | CPU work dominates, scheduler overhead becomes negligible |
| Mixed async I/O + CPU work | Separate CPU worker layer | Prevents Tokio runtime starvation |
| Bursty short-lived blocking tasks | `spawn_blocking` | Good for integration with async systems |
| Infinite/always-running CPU workers | Dedicated threads | Better lifecycle and scheduler isolation |

```
throughput
^
|                           sync threads
|                         /
|                       /
|         async tokio /
|       /
|     /
+---------------------------------> CPU work per job
      tiny        medium      large
```

In practice, dedicated threads tend to become more favorable
as CPU work per job increases and async scheduling overhead
becomes less amortized.
