# Internals: Codegen Pipeline

Codegen consumes only semantic IR.

Stages:

1. Type mapping.
2. Symbol/import resolution.
3. Source emission (`.py`, `.pyi`).
4. Packaging handoff (`ferryx-build`).

## Constraints

- no direct parser coupling.
- deterministic output for identical IR input.
- generated APIs must include typing and doc affordances.

