#!/usr/bin/env python3
import json
from pathlib import Path


def main():
    root = Path(__file__).resolve().parents[1]
    bundles = {}
    for ex in (root / "examples").iterdir():
        gen = ex / "generated"
        if not gen.exists():
            continue
        bundles[ex.name] = sorted(str(p.relative_to(root)) for p in gen.rglob("*") if p.is_file())
    out = root / "verification" / "snapshot" / "artifact_bundle.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(bundles, indent=2))
    print(out)


if __name__ == "__main__":
    main()

