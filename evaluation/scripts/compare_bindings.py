#!/usr/bin/env python3
import json
import subprocess
import sys
from pathlib import Path


def run_json_command(command: str):
    proc = subprocess.run(command, shell=True, check=True, capture_output=True, text=True)
    return json.loads(proc.stdout)


def main() -> int:
    if len(sys.argv) < 3:
        print("usage: compare_bindings.py <output.json> <name=command> [name=command...]")
        return 1

    output = Path(sys.argv[1])
    adapters = {}
    for pair in sys.argv[2:]:
        name, command = pair.split("=", 1)
        adapters[name] = run_json_command(command)

    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(adapters, indent=2))
    print(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

