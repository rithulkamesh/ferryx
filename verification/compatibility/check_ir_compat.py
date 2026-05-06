#!/usr/bin/env python3
import json
from pathlib import Path


def main():
    root = Path(__file__).resolve().parents[2]
    ir = root / "examples" / "tensor" / "generated" / "ir.json"
    if not ir.exists():
        raise SystemExit("ir.json not found")
    data = json.loads(ir.read_text())
    assert "ir_version" in data
    print(f"ir_version={data['ir_version']}")


if __name__ == "__main__":
    main()

