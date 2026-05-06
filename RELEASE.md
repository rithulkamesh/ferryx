# Release Policy

## Channels

- **stable**: semantic releases from `main` after full CI, changelog, and migration checks.
- **nightly**: pre-release artifacts for fast feedback on unstable changes.

## Versioning

- SemVer for public APIs.
- Pre-1.0 (`0.x`) allows breaking changes with mandatory migration notes.
- Post-1.0 requires explicit deprecation windows and compatibility guarantees.

## Release Checklist

1. `cargo fmt --all`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. Benchmarks run and no unapproved budget regressions.
5. Changelog updated.
6. Migration notes published for any breaking changes.
7. Security advisory notes included if applicable.

## Crate Versioning Strategy

During 0.x, core crates ship with synchronized versions.
After 1.0, decoupled crate releases may be introduced for low-risk components.

## Python Wheel Strategy

- Build wheels with maturin matrix in CI.
- Validate import and smoke tests across supported Python versions.
- Track generated API compatibility per release.

