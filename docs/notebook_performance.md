# Notebook Performance Notes

- Prefer pre-generated artifacts for interactive startup speed.
- Use `verify-artifacts` in CI to prevent notebook/runtime drift.
- Large tensors should expose compact repr summaries to avoid notebook UI stalls.

