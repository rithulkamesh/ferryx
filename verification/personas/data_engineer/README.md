# Persona: Data Engineer

Focus:

- Arrow-oriented schema projection
- columnar APIs
- serialization and import-time behavior

Run:

```bash
cargo run -p cargo-ferryx -- generate-artifacts --input examples/arrow_bridge/src/lib.rs --example-dir examples/arrow_bridge --package ferryx_arrow
cargo run -p cargo-ferryx -- benchmark --suite serialization --output verification/benchmarks/data_engineer.json
```

