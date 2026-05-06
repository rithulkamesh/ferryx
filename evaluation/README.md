# Evaluation

This directory contains reproducible evaluation artifacts and methodology.

## Structure

- `matrix.md`: comparison matrix design for ferryx vs PyO3/cffi/ctypes/pybind11.
- `results/`: machine-generated benchmark outputs.
- `scripts/run_benchmarks.sh`: reproducible local benchmark runner.
- `scripts/generate_report.py`: JSON -> Markdown report generator.
- `scripts/check_regression.py`: threshold regression gate.
- `scripts/compare_bindings.py`: adapter collector for external binding stacks.

## Rules

- no manually edited benchmark numbers.
- every reported metric must map to a reproducible command.
- CI uploads benchmark artifacts for traceability.

