#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

cargo run --manifest-path tooling/ferryx-bench/Cargo.toml -- --suite all --output verification/benchmarks/latest.json
cargo bench --manifest-path tooling/ferryx-bench/Cargo.toml
python3 evaluation/scripts/generate_report.py evaluation/results/latest.json evaluation/results/latest.md

