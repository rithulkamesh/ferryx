#!/usr/bin/env python3
import json
import sys
from pathlib import Path


def load(path: Path):
    if not path.exists():
        return {}
    data = json.loads(path.read_text())
    return {r["name"]: r["value"] for r in data.get("records", [])}


def main() -> int:
    if len(sys.argv) != 4:
        print("usage: check_regression.py <baseline.json> <current.json> <max_ratio>")
        return 1

    baseline = load(Path(sys.argv[1]))
    current = load(Path(sys.argv[2]))
    max_ratio = float(sys.argv[3])

    for name, value in current.items():
        if name not in baseline:
            continue
        old = baseline[name]
        if old == 0:
            continue
        ratio = value / old
        if ratio > max_ratio:
            print(f"regression detected for {name}: ratio={ratio:.3f} > {max_ratio}")
            return 2
    print("no regression threshold breaches")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

