# Architecture Overview

ferryx pipeline:

1. Parse Rust AST with `syn`.
2. Convert into semantic IR.
3. Register reflection descriptors at runtime.
4. Emit target-language SDKs from IR.
5. Package artifacts (wheels, metadata, docs).

```mermaid
flowchart LR
  source[RustSource] --> ast[SynAst]
  ast --> parser[FerryxParser]
  parser --> ir[SemanticIR]
  ir --> runtime[Registry]
  ir --> emitter[PythonEmitter]
  runtime --> build[FerryxBuild]
  emitter --> build
  build --> output[WheelsAndArtifacts]
```

Design invariant: parser and emitter remain decoupled through IR.

