# Internals: Semantic IR

`ferryx-ir` models semantic units:

- package/module graph
- classes, traits, enums, impl blocks
- methods, params, receivers
- docs and visibility
- async metadata
- ownership metadata

IR is serializable (`serde`) and intended for deterministic diffing and cross-tooling contracts.

## Why this shape

- language-neutral but systems-aware
- expressive enough for Python typing and exception translation
- stable anchor for future emitters

## Compatibility Notes

- IR schema changes require RFC.
- Breaking field changes need migration mapping notes.

