#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

declare -a EXAMPLES=(
  "tensor ferryx_tensor"
  "basic_structs ferryx_basic"
  "traits_protocols ferryx_traits"
  "errors ferryx_errors"
  "async_inference ferryx_async"
  "numpy_zero_copy ferryx_numpy"
  "polars_bridge ferryx_polars"
  "jupyter_demo ferryx_jupyter"
  "arrow_bridge ferryx_arrow"
  "notebook_experience ferryx_notebook"
  "llm_runtime ferryx_llm"
  "high_performance_tensor ferryx_hpt"
  "ferryx_tensor_runtime ferryx_tensor_runtime"
  "typescript_sdk ferryx_typescript_sdk"
  "wasm_runtime ferryx_wasm_runtime"
  "openapi_service ferryx_openapi_service"
  "grpc_service ferryx_grpc_service"
)

for entry in "${EXAMPLES[@]}"; do
  ex="$(echo "$entry" | awk '{print $1}')"
  pkg="$(echo "$entry" | awk '{print $2}')"
  cargo run -p cargo-ferryx -- verify-artifacts \
    --input "examples/$ex/src/lib.rs" \
    --example-dir "examples/$ex" \
    --package "$pkg"
done

