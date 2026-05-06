# Memory Model

## Ownership Boundaries

- Rust remains owner of canonical semantic state.
- generated language projections expose safe value-level APIs.
- zero-copy paths require explicit lifetime-safe bridges.

## ABI Guarantees

- `#[repr(C)]` for exported FFI structures.
- ABI version surfaced via `ferryx_ffi::ABI_VERSION`.

## Zero-copy Invariants

- contiguous backing memory
- immutable aliasing unless explicitly synchronized
- lifetime outlives foreign view

## Async Runtime Guarantees

- async metadata in IR indicates awaitability contracts.
- bridge layers must avoid hidden executor creation side effects.

