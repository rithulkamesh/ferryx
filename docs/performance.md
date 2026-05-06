# Performance

ferryx treats performance as a compatibility dimension.

## Measured Dimensions

- call overhead (Rust function -> generated Python surface)
- zero-copy throughput
- NumPy interop overhead
- async invocation latency
- serialization/deserialization throughput
- memory overhead under repeated calls
- wheel import startup time
- API generation speed

## Methodology

- Bench harnesses live in `benchmarks/`.
- Each benchmark run writes JSON artifacts under `evaluation/results/`.
- CI stores historical benchmark artifacts for trend analysis.
- Criterion benches run with `cargo bench --manifest-path tooling/ferryx-bench/Cargo.toml`.
- Markdown reports generated with `evaluation/scripts/generate_report.py`.

## Comparison Matrix Targets

Bench suites include compatibility runners for:

- PyO3
- cffi
- ctypes
- pybind11

ferryx does not publish synthetic numbers. All published metrics must point to reproducible scripts and raw outputs.

## Regression Budget Policy

- Parser + emitter generation speed: no >10% regression without explicit approval.
- Runtime call path: no >5% regression without explicit approval.
- Memory footprint: no >10% regression for baseline scenarios.

## Benchmark Reproducibility

Use:

```bash
cargo run -p cargo-ferryx -- build --input examples/tensor/src/lib.rs --out-dir target/ferryx-bench --package ferryx_tensor
cargo run --manifest-path tooling/ferryx-bench/Cargo.toml -- --suite all --output verification/benchmarks/latest.json
```

