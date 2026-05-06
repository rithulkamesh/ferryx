## What changed

Describe the technical change and subsystem impact.

## Why

State motivation and design tradeoffs.

## Architecture boundary check

- [ ] No direct AST -> Python codegen introduced.
- [ ] Change preserves Rust -> IR -> emitter contract.

## Compatibility

- [ ] Rust API compatibility assessed.
- [ ] Generated Python API compatibility assessed.
- [ ] ABI impact assessed (`ferryx-ffi` / runtime boundary).

## Safety and Performance

- [ ] Unsafe code reviewed with invariants documented.
- [ ] Relevant benchmarks run or explicitly not applicable.

## Validation

- [ ] `cargo fmt --all`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`

## Docs

- [ ] Updated docs/README/RFC notes as needed.

