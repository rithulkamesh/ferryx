# Semantic Rewrite Engine

`core/ferryx-rewrite` implements composable semantic passes.

Pipeline shape:

`Rust semantics -> rewrite pipeline -> target-native API`

## Built-in Python Passes

- naming normalization
- ownership projection
- exception projection
- iterator projection
- async projection

## Determinism

- pass execution order is explicit.
- identical IR input + pass list yields identical output.

## Extensibility

The rewrite engine is emitter-independent and can be reused by future TypeScript/WASM emitters.

