# Roadmap

ferryx is in foundation phase: small footprint, strict architecture, high ambition.

## 0.x: Foundation Hardening

- Stabilize IR schema shape and serialization semantics.
- Expand Python emitter quality (typing, protocols, exceptions, docs).
- Harden ABI boundary contracts in `ferryx-ffi`.
- Add deterministic codegen and benchmark gates in CI.
- Publish first end-to-end wheel flow with reproducible artifacts.

## 0.x: Developer Experience

- `cargo ferryx build/inspect/dev/docs` workflow maturity.
- Better diagnostics for macro/parser errors.
- Snapshot tests for generated Python APIs.
- Rich examples for async, NumPy, trait projection, and errors.

## 1.0 Stabilization Goals

- Lock IR v1 schema.
- API compatibility policy enforced by CI checks.
- ABI compatibility matrix for supported platforms.
- Performance baseline and regression budgets documented.
- Production-grade docs and migration playbooks.

## Post-1.0 Targets

- Additional language targets (TypeScript, WASM host bindings).
- Advanced async bridging (`asyncio` interop improvements).
- Optional distributed metadata/runtime model.

