# Persona: Notebook Researcher

Focus:

- repr quality
- markdown rendering
- low-friction async APIs in notebook loops

Run:

```bash
cargo run -p cargo-ferryx -- generate-artifacts --input examples/notebook_experience/src/lib.rs --example-dir examples/notebook_experience --package ferryx_notebook
python3 verification/notebooks/validate_notebook_experience.py
```

