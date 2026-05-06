# errors example

Shows `Result<T, E>` translation pipeline for projected Python exceptions.

Key behavior:

- Rust domain errors remain explicit.
- Emitter can synthesize Python exception classes from IR error metadata.

