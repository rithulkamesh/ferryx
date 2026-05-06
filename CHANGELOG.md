# Changelog

All notable changes to ferryx are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and versioning follows Semantic Versioning with additional pre-1.0 rules.

## Release Policy

- **Stable channel**: tagged releases from `main` with green CI.
- **Nightly channel**: pre-release tags (`-nightly.YYYYMMDD`) for rapid validation.
- **Security releases**: patch lines with focused fixes and advisory links.

## Versioning Strategy

### Workspace and Crates

- Workspace version tracks top-level project milestones.
- Crates are versioned together during 0.x for simplicity and compatibility tracking.
- After 1.0, crate-level independent versioning may be introduced where justified.

### Python Wheel Compatibility

- Wheels are generated from IR and runtime metadata derived from Rust source.
- Pre-1.0: generated Python API may evolve rapidly; breaking changes require migration notes.
- Post-1.0: generated API follows semantic compatibility guarantees per release notes.

## ABI Evolution Policy

- ABI-affecting changes require RFC and changelog callout.
- Additive ABI changes preferred.
- Breaking ABI changes require major version bump (post-1.0) or explicit 0.x migration notes.
- `ferryx-ffi` ABI version constants must be updated for contract changes.

## Deprecation and Migration Policy

- Deprecated behavior remains available for at least one minor cycle when feasible.
- Every deprecation must include:
  - Replacement guidance.
  - Timeline.
  - Migration examples.

## 0.x Instability Statement

ferryx is currently in 0.x. Architecture is stable, APIs are intentionally evolving.

- We optimize for correctness and model quality over short-term API permanence.
- Breaking changes are allowed, but never silent: each one requires rationale and migration guidance.

## 1.0 Stabilization Goals

- IR v1 schema lock.
- Stable Rust and Python public APIs.
- Documented ABI compatibility matrix.
- Performance budgets and regression checks.

## [Unreleased]

### Added

- Workspace architecture with dedicated crates for macros, IR, parser, runtime, Python emitter, build, CLI, and FFI.
- Inventory-backed runtime reflection registry.
- `#[ferryx]` macro support for structs and impl blocks.
- Syn-based AST-to-IR parser pipeline.
- Python emitter generating `.py` and `.pyi` artifacts.
- Build crate orchestration for parse -> IR -> Python emission.
- CLI commands: `build`, `inspect`, `dev`, `docs`.
- ABI-safe FFI base layer with explicit version surface.
- Governance, RFC, docs, examples, benchmarking, and CI infrastructure.

