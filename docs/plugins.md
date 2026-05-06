# Plugin and Emitter Ecosystem

ferryx plugins operate strictly on semantic IR.

## Plugin Types

- emitter plugins
- type mapper plugins
- serializer plugins
- notebook renderer plugins

Implemented in `crates/ferryx-plugin`.

## Contract

- no plugin receives Rust AST.
- plugins are deterministic for identical IR input.
- plugin output must be traceable to IR version + plugin id.

## Future Backends

- TypeScript
- WASM host bindings
- Julia
- R
- OpenAPI
- gRPC

All future targets integrate through emitter plugins.

