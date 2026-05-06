# Persona: Systems Engineer

Focus:

- IR consistency
- registry introspection
- ABI and safety constraints

Run:

```bash
cargo run -p cargo-ferryx -- inspect-registry
cargo run -p cargo-ferryx -- inspect-rewrite
python3 verification/compatibility/check_ir_compat.py
```

