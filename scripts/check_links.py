#!/usr/bin/env python3
import re
from pathlib import Path


def main():
    root = Path(__file__).resolve().parents[1]
    bad = []
    for md in root.rglob("*.md"):
        text = md.read_text(encoding="utf-8")
        for rel in re.findall(r"\[[^\]]+\]\(([^)]+)\)", text):
            if rel.startswith("http") or rel.startswith("#"):
                continue
            target = (md.parent / rel).resolve()
            if not target.exists():
                bad.append((md, rel))
    if bad:
        for md, rel in bad:
            print(f"broken link: {md} -> {rel}")
        raise SystemExit(2)
    print("link check ok")


if __name__ == "__main__":
    main()

