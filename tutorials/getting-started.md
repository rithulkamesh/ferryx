# Getting Started

## 1. Write Rust source

```rust
use ferryx_macros::ferryx;

#[ferryx]
pub struct Tensor {
    pub data: Vec<f32>,
}
```

## 2. Generate artifacts

```bash
cargo run -p cargo-ferryx -- build \
  --input examples/tensor/src/lib.rs \
  --out-dir target/ferryx-out \
  --package ferryx_tensor
```

## 3. Inspect IR

```bash
cargo run -p cargo-ferryx -- inspect \
  --input examples/tensor/src/lib.rs \
  --package ferryx_tensor
```

## 4. Use generated Python

Load generated package from output path and use typed APIs.

