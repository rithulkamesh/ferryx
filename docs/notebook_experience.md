# Notebook Experience

ferryx-generated Python classes include notebook-friendly representation hooks.

## Renderer Infrastructure

- `__repr__` for concise textual previews
- `_repr_markdown_` for rich markdown display
- plugin extension point for MIME bundles via `NotebookRendererPlugin`

## Design Goals

- PyTorch-like tensor previews
- Polars-like table summaries
- async-compatible APIs for notebook execution loops

## Example

See `examples/notebook_experience/`.

