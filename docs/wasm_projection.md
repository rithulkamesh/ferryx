# WASM Projection Model

WASM emission is driven from semantic IR via `emitters/ferryx-wasm`.

## Host Boundary Architecture

- generated host bridge interfaces in TypeScript
- explicit pointer-handle model for host references
- memory transfer policy emitted as `memory-model.json`

## Ownership Safety

- no implicit transfer of ownership across boundary
- borrowed views require explicit validation
- async interactions are promise-based bridge calls

