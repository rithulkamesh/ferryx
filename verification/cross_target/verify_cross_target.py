#!/usr/bin/env python3
import json
import subprocess
from pathlib import Path


def run(cmd: str):
    subprocess.run(cmd, shell=True, check=True)


def main():
    root = Path(__file__).resolve().parents[2]
    src = root / "examples" / "typescript_sdk" / "src" / "lib.rs"
    tmp = root / "verification" / "cross_target" / "out"
    tmp.mkdir(parents=True, exist_ok=True)

    run(f"cargo run -p cargo-ferryx -- build --input {src} --out-dir {tmp / 'python'} --package ferryx_typescript_sdk")
    run(f"cargo run -p cargo-ferryx -- emit-typescript --input {src} --out-dir {tmp / 'ts'} --package ferryx_typescript_sdk")
    run(f"cargo run -p cargo-ferryx -- emit-openapi --input {src} --out-file {tmp / 'openapi.json'} --package ferryx_typescript_sdk")
    run(f"cargo run -p cargo-ferryx -- emit-grpc --input {src} --out-file {tmp / 'service.proto'} --package ferryx_typescript_sdk")

    pyi = (tmp / "python" / "ferryx_typescript_sdk" / "__init__.pyi").read_text()
    ts = (tmp / "ts" / "index.ts").read_text()
    openapi = json.loads((tmp / "openapi.json").read_text())
    proto = (tmp / "service.proto").read_text()

    assert "get_user" in pyi
    assert "get_user" in ts
    assert "/userapi/get_user" in openapi["paths"]
    assert "rpc get_user" in proto
    print("cross-target semantic verification ok")


if __name__ == "__main__":
    main()

