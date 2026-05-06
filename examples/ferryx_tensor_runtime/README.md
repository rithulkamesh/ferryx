# ferryx_tensor_runtime

Self-hosting stress example for ferryx.

It validates:

- shape-checked tensor constructors
- error projection from `Result`
- async scaling path
- generated artifacts + drift checks

## Generate

```bash
cargo run -p cargo-ferryx -- generate-artifacts \
  --input examples/ferryx_tensor_runtime/src/lib.rs \
  --example-dir examples/ferryx_tensor_runtime \
  --package ferryx_tensor_runtime
```

