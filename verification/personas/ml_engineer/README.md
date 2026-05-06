# Persona: ML Engineer

Workflow:

1. Generate tensor runtime artifacts.
2. Validate zero-copy metadata and notebook repr quality.
3. Run throughput benchmark suite.

Commands:

```bash
cargo run -p cargo-ferryx -- generate-artifacts --input examples/high_performance_tensor/src/lib.rs --example-dir examples/high_performance_tensor --package ferryx_hpt
python3 verification/notebooks/validate_notebook_experience.py
cargo run -p cargo-ferryx -- benchmark --suite zero_copy_throughput --output verification/benchmarks/ml_engineer.json
```

