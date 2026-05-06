#!/usr/bin/env python3
from pathlib import Path
import json


def main():
    root = Path(__file__).resolve().parents[2]
    examples = [p for p in (root / "examples").iterdir() if p.is_dir()]
    for ex in examples:
        gen = ex / "generated"
        if not gen.exists():
            continue
        meta = gen / "metadata.json"
        assert meta.exists(), f"missing metadata for {ex.name}"
        json.loads(meta.read_text())
    print("golden verification ok")


if __name__ == "__main__":
    main()

