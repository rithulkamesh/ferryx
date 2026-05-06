# Contributing to ferryx

ferryx is a Rust-first semantic projection framework. Contributions must preserve one hard boundary:

`Rust AST -> semantic IR -> language emitters`

Direct AST-to-language output is out of scope and will be rejected.

## Development Environment

- Rust stable (latest), plus nightly for selected checks.
- Python 3.9+ for generated SDK validation.
- `cargo`, `rustfmt`, `clippy`, `maturin`.

Recommended local loop:

1. `cargo fmt --all`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. Run relevant benchmarks when touching runtime/codegen/ffi.

## Pull Request Lifecycle

1. Open an issue (bug, feature, design concern) or link to an existing one.
2. For non-trivial design changes, create an RFC before implementation.
3. Submit PR with:
   - Problem statement.
   - Design summary and tradeoffs.
   - Compatibility impact.
   - Test plan and benchmark evidence (if perf-sensitive).
4. Maintainer review:
   - Correctness.
   - IR architecture compliance.
   - API stability and migration impact.
   - Safety and ABI review where relevant.
5. Squash or rebase is fine; maintain clear commit semantics.

## Review Standards

Changes are accepted when they include:

- Tests for happy path and failure path.
- Updated docs for any user-facing behavior.
- Benchmarks for claimed performance improvements.
- Safety notes for `unsafe` blocks.
- Compatibility notes for API/ABI changes.

## Unsafe Rust Policy

Any `unsafe` requires:

- Dedicated safety comment explaining invariants.
- Why safe Rust alternatives are not viable.
- At least one maintainer with unsafe/ABI review context.

## API and Compatibility Policy

- Public Rust API follows SemVer.
- Python generated API follows projection compatibility guarantees defined in `CHANGELOG.md`.
- ABI compatibility rules are defined in `SECURITY.md` and `GOVERNANCE.md`.

## Benchmarks and Performance Regressions

If change touches parser, runtime registry, emitter, FFI, or generated call path:

- Run benchmark harnesses in `benchmarks/`.
- Include before/after artifacts in PR.
- Regressions >5% in core paths require explicit maintainer approval and rationale.

## Documentation Expectations

Contributors must update docs in the same PR when behavior changes:

- `README.md` for high-level UX.
- `docs/` for deep design.
- `rfcs/` when architecture direction changes.

## RFC Process

RFC required for:

- ABI changes.
- IR schema changes.
- Macro syntax changes.
- Async ownership/bridging model shifts.
- New language targets.

Use `rfcs/RFC_TEMPLATE.md`.

## Issue Triage

Maintainers label issues by:

- `kind:bug`, `kind:feature`, `kind:docs`, `kind:perf`, `kind:security`
- `area:macros`, `area:ir`, `area:runtime`, `area:python`, `area:ffi`, `area:cli`, `area:build`
- `priority:P0..P3`

## Release Policy Summary

- Stable releases: from `main` with green CI, changelog entry, and migration notes.
- Patch releases: bug/security/perf fixes without breaking API contracts.
- Minors/majors: follow SemVer + stabilization policy in `CHANGELOG.md`.

