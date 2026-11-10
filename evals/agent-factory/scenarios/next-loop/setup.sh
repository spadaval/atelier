#!/usr/bin/env bash
# A small Python repository tracked by the redesigned atelier (next/): a
# config that names no forge, one ready item asking for a small real change,
# and no instructions or skill beyond what `atelier status` prints. work/bin
# holds the new binary, which the harness puts first on PATH; NEXT_ATELIER
# names it (the round's copy of next/target/debug/atelier).
set -euo pipefail
work=$1
: "${NEXT_ATELIER:?set NEXT_ATELIER to the new atelier binary}"
mkdir -p "$work/bin"
cp "$NEXT_ATELIER" "$work/bin/atelier"
export GIT_AUTHOR_NAME="Dana Dev" GIT_AUTHOR_EMAIL=dana@example.com
export GIT_COMMITTER_NAME="Dana Dev" GIT_COMMITTER_EMAIL=dana@example.com
git init -q -b main "$work/repo"
cd "$work/repo"
# Agents commit too; give the clone an identity that is nobody's.
git config user.name "Eval Agent"
git config user.email agent@example.com
cat > slug.py <<'PY'
"""Turn titles into URL slugs."""
import re


def slugify(title: str) -> str:
    """Lowercase `title` and replace each run of other characters with a hyphen."""
    return re.sub(r"[^a-z0-9]", "-", title.lower())
PY
cat > test_slug.py <<'PY'
import unittest

from slug import slugify


class SlugifyTest(unittest.TestCase):
    def test_lowercases_and_hyphenates(self):
        self.assertEqual(slugify("Hello World"), "hello-world")


if __name__ == "__main__":
    unittest.main()
PY
cat > README.md <<'MD'
# slugs

`slug.slugify(title)` turns a post title into the slug in its URL.

Tests: `python3 -m unittest`
MD
printf '__pycache__/\n' > .gitignore
mkdir -p .atelier/items/slug-k3fa
cat > .atelier/config.toml <<'TOML'
schema_version = 3
slug = "slug"
base_branch = "main"

[[merge_check]]
run = "python3 -m unittest -q"
TOML
cat > .atelier/items/slug-k3fa/item.md <<'MD'
---
schema: atelier.item
schema_version: 3
id: slug-k3fa
title: Make slugify collapse runs of punctuation into one hyphen
status: ready
mission: null
labels: []
priority: high
blocked_by: []
implementers: []
ready:
  by: dana
  at: 2026-11-10T09:00:00Z
trivial: null
caused_by: null
created_at: 2026-11-10T09:00:00Z
---

## Goal

`slugify("Hello,  World!")` returns `hello---world-`, which makes ugly URLs.
A slug should have one hyphen between words and none at either end.

## Done when

- [ ] `slugify("Hello,  World!")` returns `hello-world`, and
  `slugify("  --Rust & Go--  ")` returns `rust-go`.
- [ ] `test_slug.py` has a test for each of those two cases, and
  `python3 -m unittest` passes.

## Scope

`slug.py` and `test_slug.py` only.

## Report
MD
git add . && git commit -qm "Slugify titles, and track work in Atelier"

# Grader inputs, outside the repository.
git rev-parse HEAD > "$work/.setup_head"
