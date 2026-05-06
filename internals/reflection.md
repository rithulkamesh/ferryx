# Internals: Reflection Registry

`ferryx-runtime` uses inventory-based registration records:

- macro expansion submits `ReflectionRecord`.
- runtime collects records deterministically.
- tools query registry for inspection and generation support.

## Guarantees

- deterministic ordering for reproducible output.
- malformed records ignored to preserve runtime stability.
- descriptor payload is semantic IR item JSON.

## Future Direction

- optional schema hash validation.
- registry integrity checks in CI.

