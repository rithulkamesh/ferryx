# Architecture Overview

ferryx multi-target pipeline:

1. Parse Rust AST with `syn`.
2. Convert into semantic IR.
3. Register reflection descriptors at runtime.
4. Execute rewrite pipeline with target compatibility checks.
5. Emit target-language SDKs/schemas from IR.
5. Package artifacts (wheels, metadata, docs).

```mermaid
flowchart LR
  source[RustSource] --> ast[SynAst]
  ast --> parser[FerryxParser]
  parser --> ir[SemanticIR]
  ir --> runtime[Registry]
  ir --> rewrite[RewritePipeline]
  rewrite --> pyEmitter[PythonEmitter]
  rewrite --> tsEmitter[TypeScriptEmitter]
  rewrite --> wasmEmitter[WasmEmitter]
  rewrite --> openapiEmitter[OpenApiEmitter]
  rewrite --> grpcEmitter[GrpcEmitter]
  runtime --> build[FerryxBuild]
  pyEmitter --> build
  tsEmitter --> build
  wasmEmitter --> build
  openapiEmitter --> build
  grpcEmitter --> build
  build --> output[LanguageArtifacts]
```

Design invariant: parser and emitter remain decoupled through IR.

