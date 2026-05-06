#!/usr/bin/env python3
import json
from pathlib import Path


def main():
    root = Path(__file__).resolve().parents[1]
    src = root / "verification" / "snapshot" / "prompt_eval.json"
    dst = root / "verification" / "snapshot" / "prompt_eval.md"
    data = json.loads(src.read_text())
    lines = [
        "# Prompt Verification Report",
        "",
        f"Generated at: `{data['generated_at']}`",
        "",
        f"Prompt count: `{data['prompt_count']}`",
        "",
        "## Prompts",
    ]
    lines.extend(f"- {name}" for name in data["prompts"])
    dst.write_text("\n".join(lines))
    print(dst)


if __name__ == "__main__":
    main()

