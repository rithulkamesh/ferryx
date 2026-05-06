# Emitter Architecture

Emitters consume semantic IR and optional rewrite pipelines.

## Stages

1. IR compatibility validation
2. semantic rewrite execution
3. type mapping
4. symbol/import planning
5. source generation

## Determinism

Emitter outputs are deterministic with:

- fixed pass ordering
- sorted registry traversal
- stable formatting conventions

