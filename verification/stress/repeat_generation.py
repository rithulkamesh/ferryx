#!/usr/bin/env python3
import subprocess
from pathlib import Path


def run(cmd):
    subprocess.run(cmd, shell=True, check=True)


def main():
    root = Path(__file__).resolve().parents[2]
    src = root / "examples" / "high_performance_tensor" / "src" / "lib.rs"
    ex = root / "examples" / "high_performance_tensor"
    for _ in range(3):
        run(
            f"cargo run -p cargo-ferryx -- generate-artifacts --input {src} --example-dir {ex} --package ferryx_hpt"
        )
    print("stress generation ok")


if __name__ == "__main__":
    main()

