# Internals: Async Bridge

ferryx captures async semantics in IR (`is_async`, runtime hints).

Current architecture is tokio-compatible and designed for future `asyncio` bridge layers.

## Async Projection Goals

- preserve cancellation semantics where feasible.
- expose Pythonic awaitable APIs.
- avoid hidden runtime initialization side effects.

Any async model change requires RFC due to ergonomics and runtime compatibility impact.

