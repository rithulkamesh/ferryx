#!/usr/bin/env python3
import json
import subprocess
from pathlib import Path


def run(cmd):
    proc = subprocess.run(cmd, shell=True, check=True, capture_output=True, text=True)
    return proc.stdout.strip()


def main():
    root = Path(__file__).resolve().parents[2]
    src = root / "examples" / "tensor" / "src" / "lib.rs"
    out = root / "examples" / "tensor" / "generated"
    run(
        f"cargo run -p cargo-ferryx -- generate-artifacts --input {src} --example-dir {out.parent} --package ferryx_tensor"
    )
    metadata = json.loads((out / "metadata.json").read_text())
    assert metadata["hashes"], "metadata hashes missing"
    print("smoke ok")


if __name__ == "__main__":
    main()

