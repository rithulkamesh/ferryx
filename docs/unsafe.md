# Unsafe Rust Audit Strategy

ferryx currently keeps unsafe usage minimal and concentrated near ABI boundaries.

## Invariant Documentation Conventions

Each unsafe block must document:

- required preconditions
- ownership assumptions
- aliasing assumptions
- lifetime guarantees
- failure modes

## Review Requirements

- one domain reviewer + one safety reviewer for unsafe/ABI changes
- mandatory tests around boundary behavior
- no merge without explicit invariant comments

## Python and GIL Assumptions

- Python object interaction must respect GIL ownership in PyO3 integrations.
- FFI layers cannot assume Python thread-state without explicit acquisition.

## Send/Sync Expectations

- types crossing thread boundaries require explicit Send/Sync reasoning.
- runtime registries use thread-safe primitives with deterministic read semantics.

