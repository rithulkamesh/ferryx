# Governance

ferryx is maintained as infrastructure, not as a demo project. Governance exists to keep technical direction coherent while scaling contributor throughput.

## Roles

- **Founding Maintainer**: final technical arbitration, release sign-off, long-term architecture stewardship.
- **Core Maintainers**: own subsystem roadmaps, review and merge changes, run release trains.
- **Subsystem Maintainers**: delegated ownership for crate-specific areas.
- **Contributors**: propose and implement changes through issues, PRs, and RFCs.

Current maintainers are listed in `MAINTAINERS.md`.

## Decision Model

- Routine changes: maintainer consensus in PR review.
- Architecture changes: RFC required; approved by founding maintainer + at least one relevant core maintainer.
- Emergency security/perf hotfixes: expedited review with postmortem follow-up.

## Technical Guardrails

1. Rust is the source of truth.
2. Semantic IR is the only projection boundary.
3. No direct Rust AST -> Python codegen.
4. ABI stability is explicit, versioned, and reviewed.
5. Generated APIs must feel handwritten and Pythonic.

## API Stability and Versioning

- Pre-1.0: rapid iteration allowed, but breaking changes still require migration notes.
- Post-1.0: SemVer enforced across public crates and generated Python surfaces.

## Performance Governance

- Performance regressions in core paths require benchmark evidence and sign-off.
- Benchmarks are tracked in `benchmarks/` and `docs/performance.md`.
- Claims without reproducible scripts are treated as unverified.

## Review Expectations

- Every PR needs one domain reviewer.
- Unsafe/ABI changes require two reviewers, one with FFI experience.
- User-facing changes require docs update in same PR.

## Release Governance

- Release readiness requires green CI, updated changelog, and migration notes where applicable.
- Version bumps follow policy in `CHANGELOG.md`.

## Conflict Resolution

When consensus fails:

1. Collect concrete alternatives and constraints.
2. Time-box discussion.
3. Founding maintainer decides based on project principles and long-term maintainability.

