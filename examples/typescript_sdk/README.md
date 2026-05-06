# typescript_sdk example

Demonstrates deterministic IR -> TypeScript SDK generation with async and error semantics.

```bash
cargo run -p cargo-ferryx -- emit-typescript \
  --input examples/typescript_sdk/src/lib.rs \
  --out-dir examples/typescript_sdk/generated \
  --package ferryx_typescript_sdk
```

