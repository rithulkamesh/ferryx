# Internals: Memory Model

ferryx memory goals:

- explicit ownership boundaries
- minimal copying across Rust/Python boundaries
- ABI-safe FFI contracts for long-term stability

## Zero-copy Direction

Examples and benchmark harnesses model future zero-copy paths via:

- borrowed buffers
- contiguous numeric vectors
- predictable lifetime boundaries

No implicit unsafe borrowing across language boundaries is accepted without explicit invariants.

