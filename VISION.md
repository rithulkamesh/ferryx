# VISION

ferryx aims to become a semantic SDK infrastructure layer: write systems in Rust once, project native SDKs across ecosystems.

## Language Targets

Planned projection targets beyond Python:

- TypeScript
- WASM host bindings
- Julia
- R
- OpenAPI schema generation
- gRPC interface generation

All targets derive from semantic IR, never direct AST emitters.

## Notebook-native Compute

Long term, ferryx should make notebook workflows first-class:

- generated Python APIs with discoverable docs,
- Jupyter-ready packaging,
- zero-copy data interchange for numerical workloads.

## Distributed Runtime Ideas

Future runtime layer may include:

- distributed registry synchronization,
- remote capability discovery,
- projection-aware service boundaries.

## Plugin Ecosystem

ferryx should support pluggable emitters and policy modules:

- emitter plugins for new languages,
- lint/policy plugins for compatibility constraints,
- docs and benchmark reporter plugins.

## IR Stabilization Roadmap

1. 0.x: rapid IR evolution with migration notes.
2. Pre-1.0: freeze candidate, introduce schema conformance tests.
3. 1.0: IR v1 stabilization with compatibility contract.

## Semantic SDK Ecosystem

Target state:

- a shared ecosystem of IR-aware tools,
- reproducible language projections,
- compatibility-aware release automation,
- strong contributor community around a stable semantic core.

Small today, inevitable tomorrow.

