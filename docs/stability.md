# Stability Model

ferryx stability classes are encoded by IR package metadata:

- `experimental`
- `beta`
- `stable`
- `internal`

## Rules

- Experimental: breaking changes allowed with migration notes.
- Beta: compatibility preferred, breakage requires explicit changelog migration.
- Stable: SemVer compatibility expected.
- Internal: not part of public compatibility contract.

## Enforcement

- CLI `inspect-ir` validates IR version compatibility.
- Build pipeline validates version with `validate_ir_compatibility`.

