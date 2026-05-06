# R Projection Foundations

ferryx R projection is modeled as semantic IR -> R package emitter.

## Type Mapping Registry

- `String` -> `character`
- numeric scalars -> `numeric`/`integer`
- `Vec<T>` -> vector/list projection by element shape
- `Option<T>` -> nullable mapping (`NA` / list-none)

## Dataframe and Arrow Interop

- IR metadata used to generate data-frame oriented adapters.
- Arrow buffers are preferred for large tabular exchange.
- ownership boundaries remain explicit for safety.

