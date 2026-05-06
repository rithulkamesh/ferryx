#!/usr/bin/env python3
import subprocess
from pathlib import Path


def main():
    root = Path(__file__).resolve().parents[2]
    dist = root / "dist"
    dist.mkdir(exist_ok=True)
    subprocess.run("python3 -m pip --version", shell=True, check=True)
    print("wheel environment check ok")


if __name__ == "__main__":
    main()

