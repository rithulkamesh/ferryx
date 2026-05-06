# tensor example

Demonstrates class reflection + method projection for numeric structs.

## Run

```bash
cargo run -p cargo-ferryx -- build \
  --input examples/tensor/src/lib.rs \
  --out-dir examples/tensor/generated \
  --package ferryx_tensor
```

## Expected artifacts

- `generated/ferryx-ir.json`
- `generated/ferryx_tensor/__init__.py`
- `generated/ferryx_tensor/__init__.pyi`

