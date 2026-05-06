# Async Bridge

ferryx captures async intent in IR and rewrites async semantics for target APIs.

## Current Guarantees

- method async flags preserved in IR
- rewrite pass marks Python awaitability metadata
- emitters can synthesize awaitable signatures

## Forward Work

- explicit asyncio runtime integration layer
- cancellation propagation policy
- cross-runtime scheduling diagnostics

