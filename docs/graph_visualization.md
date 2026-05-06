# IR Graph Visualization

`cargo-ferryx graph` renders semantic IR into:

- Mermaid flowchart output
- DOT graph output

## Usage

```bash
cargo run -p cargo-ferryx -- graph --input examples/tensor/src/lib.rs --package ferryx_tensor --format mermaid
cargo run -p cargo-ferryx -- graph --input examples/tensor/src/lib.rs --package ferryx_tensor --format dot --output target/ir.dot
```

## Purpose

- design review of API surfaces
- projection debugging
- docs and architecture communication

