# Reflection Runtime

`ferryx-runtime` hosts inventory-backed metadata registration.

## Lifecycle

1. `#[ferryx]` macro emits `ReflectionRecord`.
2. registry collects records at startup.
3. tooling queries registry for inspection and generation helpers.

## Cost Model

- startup: inventory scan + JSON decode
- steady state: read-only lookups over sorted entries

## Benchmark Coverage

Reflection lookup overhead is measured in benchmark suites and tracked in CI artifacts.

