#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

MODE="${1:---dry-run}"
shift || true

PUBLISH_RUST=false
PUBLISH_PYTHON=false
PUBLISH_TS=false
DRY=true

for arg in "$MODE" "$@"; do
  case "$arg" in
    --release) DRY=false ;;
    --dry-run) DRY=true ;;
    --rust) PUBLISH_RUST=true ;;
    --python) PUBLISH_PYTHON=true ;;
    --typescript) PUBLISH_TS=true ;;
  esac
done

if ! $PUBLISH_RUST && ! $PUBLISH_PYTHON && ! $PUBLISH_TS; then
  PUBLISH_RUST=true
  PUBLISH_PYTHON=true
  PUBLISH_TS=true
fi

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
bash scripts/verify_all_artifacts.sh

if $PUBLISH_RUST; then
  CRATES=(
    core/ferryx-ir
    core/ferryx-rewrite
    core/ferryx-target
    core/ferryx-runtime
    core/ferryx-macros
    core/ferryx-parser
    emitters/ferryx-python
    emitters/ferryx-plugin
    emitters/ferryx-typescript
    emitters/ferryx-wasm
    emitters/ferryx-openapi
    emitters/ferryx-grpc
    tooling/ferryx-build
    ffi/ferryx-ffi
    tooling/ferryx-cli
  )
  for crate in "${CRATES[@]}"; do
    if $DRY; then
      cargo publish --manifest-path "$crate/Cargo.toml" --dry-run
    else
      cargo publish --manifest-path "$crate/Cargo.toml"
    fi
  done
fi

if $PUBLISH_PYTHON; then
  if $DRY; then
    maturin build --release --out dist
  else
    maturin publish --skip-existing
  fi
fi

if $PUBLISH_TS; then
  cargo run -p cargo-ferryx -- emit-typescript \
    --input examples/typescript_sdk/src/lib.rs \
    --out-dir examples/typescript_sdk/generated \
    --package ferryx_typescript_sdk
  if [ -f examples/typescript_sdk/generated/package.json ]; then
    if $DRY; then
      (cd examples/typescript_sdk/generated && npm pack)
    else
      (cd examples/typescript_sdk/generated && npm publish --access public)
    fi
  fi
fi

