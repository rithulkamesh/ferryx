# Target Capability System

Capability metadata is defined in `crates/ferryx-target`.

Each target declares support across:

- async semantics
- exception projection
- protocol support
- notebook repr
- zero-copy buffers
- browser compatibility
- schema generation
- ownership restrictions

The validation engine emits compatibility warnings before target emission.

