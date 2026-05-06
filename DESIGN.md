# DESIGN

ferryx exists to make generated SDKs feel native without sacrificing systems guarantees.

## Core Principles

### Rust-first

Rust source defines semantics. Anything not represented in Rust metadata is out of model scope.

### Semantic IR as the contract

IR is not an implementation detail; it is the projection boundary and compatibility artifact.

### No duplicated schemas

ferryx does not ask users to re-declare Rust contracts in Python metadata files.

### Zero wrapper drift

Handwritten wrappers rot. ferryx generates from IR so SDK shape tracks source truth.

### Python-native ergonomics

Generated APIs prioritize Python readability, typing richness, and natural exceptions.

### Predictable ownership

Ownership and borrowing semantics are explicit in IR and emitter decisions.

### Explicit performance

Performance claims require benchmarks and reproducible harnesses.

### Infrastructure over magic

Every stage is inspectable: parse, IR, runtime registry, emission, packaging.

## Why direct AST -> Python is forbidden

AST nodes are syntax-level facts, not stable semantic contracts. Emitting directly from AST:

- couples emitter to parser quirks,
- blocks multi-language projection reuse,
- makes compatibility guarantees fragile.

IR decouples syntax ingestion from language projections.

## Why semantic IR exists

IR provides:

- a stable schema for projections,
- serialization for tooling and diffing,
- deterministic code generation inputs,
- future target portability (TypeScript, WASM, Julia, R).

## Why metadata registries matter

Runtime metadata registries provide:

- reflection and inspection at runtime,
- emitter orchestration without recompilation hacks,
- consistent source of descriptor truth across tooling.

## Why generated APIs must feel handwritten

Generated SDKs are product surface. If they feel alien, users reject them.

ferryx generators must produce:

- idiomatic naming,
- precise typing,
- meaningful exceptions,
- high-quality docstrings.

