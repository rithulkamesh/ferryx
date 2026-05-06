# IR Versioning

`ferryx-ir` exposes `IR_VERSION` as the schema contract version.

## Evolution Policy

1. Additive fields: minor version bump.
2. Semantic reinterpretation: minor version bump plus migration guidance.
3. Breaking field removals/renames: major version bump.

## Compatibility Check

`validate_ir_compatibility()` currently enforces exact version match for deterministic toolchain behavior.

## Migration Policy

When IR changes:

- changelog entry with migration section
- parser + emitter update in same release
- fixtures/examples regenerated and validated

