#!/usr/bin/env bash
set -euo pipefail

LEVEL="${1:-patch}"
PRERELEASE="${2:-false}"
NIGHTLY="${3:-false}"
DRY_RUN="${4:-true}"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

CURRENT="$(python3 - <<'PY'
import tomllib
with open("Cargo.toml","rb") as f:
    d=tomllib.load(f)
print(d["workspace"]["package"]["version"])
PY
)"

python3 - "$CURRENT" "$LEVEL" "$PRERELEASE" "$NIGHTLY" "$DRY_RUN" <<'PY'
import sys, re, datetime, pathlib
current, level, prerelease, nightly, dry = sys.argv[1:]
m = re.match(r"^(\d+)\.(\d+)\.(\d+)", current)
if not m:
    raise SystemExit(f"invalid version: {current}")
major, minor, patch = map(int, m.groups())
if level == "major":
    major += 1; minor = 0; patch = 0
elif level == "minor":
    minor += 1; patch = 0
else:
    patch += 1
base = f"{major}.{minor}.{patch}"
if nightly == "true":
    stamp = datetime.datetime.utcnow().strftime("%Y%m%d")
    nxt = f"{base}-nightly.{stamp}"
elif prerelease == "true":
    nxt = f"{base}-beta.1"
else:
    nxt = base
print(nxt)
if dry == "true":
    raise SystemExit(0)
path = pathlib.Path("Cargo.toml")
text = path.read_text()
text = re.sub(r'version = "[^"]+"', f'version = "{nxt}"', text, count=1)
path.write_text(text)
print(f"updated Cargo.toml version to {nxt}")
PY

