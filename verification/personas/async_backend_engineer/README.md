# Persona: Async Backend Engineer

Focus:

- async projection semantics
- rewrite async annotations
- latency checks

Run:

```bash
cargo run -p cargo-ferryx -- generate-artifacts --input examples/async_inference/src/lib.rs --example-dir examples/async_inference --package ferryx_async
cargo run -p cargo-ferryx -- benchmark --suite async_latency --output verification/benchmarks/async_backend_engineer.json
```

