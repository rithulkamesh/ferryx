# Julia Projection Foundations

ferryx Julia projection is designed as an IR-native emitter pipeline.

## Type Mapping Registry

- `String` -> `String`
- numeric scalars -> Julia scalar types
- `Vec<T>` -> `Vector{T}`
- `Option<T>` -> `Union{T, Nothing}`

## Ownership and Runtime Notes

- explicit host boundary ownership transfer
- zero-copy candidates via Arrow-compatible buffers
- async modeled through task/future interop adapter layer

