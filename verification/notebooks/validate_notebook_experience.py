#!/usr/bin/env python3
from pathlib import Path


def main():
    root = Path(__file__).resolve().parents[2]
    py_file = root / "examples" / "notebook_experience" / "generated" / "ferryx_notebook" / "__init__.py"
    if not py_file.exists():
        print("notebook generated package missing")
        raise SystemExit(2)
    text = py_file.read_text()
    assert "_repr_markdown_" in text
    print("notebook verification ok")


if __name__ == "__main__":
    main()

