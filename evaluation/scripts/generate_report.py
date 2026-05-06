#!/usr/bin/env python3
import json
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: generate_report.py <input.json> <output.md>")
        return 1

    src = Path(sys.argv[1])
    dst = Path(sys.argv[2])
    report = json.loads(src.read_text())

    lines = [
        "# Benchmark Report",
        "",
        f"Suite: `{report['suite']}`",
        "",
        "| Name | Unit | Value |",
        "|---|---|---:|",
    ]
    for rec in report.get("records", []):
        lines.append(f"| {rec['name']} | {rec['unit']} | {rec['value']:.6f} |")
    lines.append("")
    lines.append("## Notes")
    for note in report.get("notes", []):
        lines.append(f"- {note}")
    lines.append("")

    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.write_text("\n".join(lines))
    print(dst)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

