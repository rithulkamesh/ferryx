#!/usr/bin/env python3
import json
from pathlib import Path
from datetime import datetime, timezone


def main():
    root = Path(__file__).resolve().parents[1]
    prompts = sorted((root / "verification" / "prompts").glob("*.md"))
    output = root / "verification" / "snapshot" / "prompt_eval.json"
    report = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "prompt_count": len(prompts),
        "prompts": [p.name for p in prompts],
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(report, indent=2))
    print(output)


if __name__ == "__main__":
    main()

