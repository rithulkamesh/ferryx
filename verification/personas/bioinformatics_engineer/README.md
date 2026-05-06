# Persona: Bioinformatics Engineer

Focus:

- large vector datasets
- reproducible artifact generation
- CLI diagnostics for heterogeneous environments

Run:

```bash
cargo run -p cargo-ferryx -- doctor
cargo run -p cargo-ferryx -- generate-artifacts --input examples/tensor/src/lib.rs --example-dir examples/tensor --package ferryx_bio
python3 verification/golden/verify_golden.py
```

