# Generated Artifacts

ferryx commits deterministic generated artifacts for each example to make projection behavior inspectable and reviewable.

## Required Files

Each `examples/<name>/generated/` contains:

- `<package>/__init__.py`
- `<package>/__init__.pyi`
- `ir.json`
- `graph.mmd`
- `graph.dot`
- `docs.md`
- `metadata.json`

## Commands

Regenerate:

```bash
scripts/generate_all_artifacts.sh
```

Verify drift:

```bash
scripts/verify_all_artifacts.sh
```

## Reproducibility

- deterministic parser + rewrite + emitter pipeline
- SHA-256 hash manifest in `metadata.json`
- drift detection uses regenerated transient artifacts and hash comparison

## Commit Policy

- Commit generated artifacts under `examples/*/generated/`.
- Do not commit transient `.generated-verify` directories.

