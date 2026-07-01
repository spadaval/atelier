---
created_at: "2026-07-01T05:57:21.974568160+00:00"
id: "atelier-nek7"
evidence_type: "validation"
captured_at: "2026-07-01T05:57:19.809458515+00:00"
command: "bash -lc 'set -euo pipefail; if rg -n -g \"*.md\" \"atelier (mission status|mission list|issue status|work queue --(ready|blocked|status|category))\" docs .agents AGENTS.md README.md PRODUCT_INTENT.md CONTEXT.md; then exit 1; fi; target/debug/atelier --help | rg -F \"atelier work ready\"; target/debug/atelier work --help | rg -F \"missions  List mission records by issue_type\"; target/debug/atelier issue --help | rg -F \"show        Show issue details\"; cargo fmt -- --check; git diff --check; target/debug/atelier check atelier-p0am'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-p0am"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p0am"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Current vocabulary check passed without stale command forms"
updated_at: "2026-07-01T05:57:28.254391408+00:00"
---

## Summary

Current vocabulary check passed without stale command forms

## Command

```console
bash -lc 'set -euo pipefail; if rg -n -g "*.md" "atelier (mission status|mission list|issue status|work queue --(ready|blocked|status|category))" docs .agents AGENTS.md README.md PRODUCT_INTENT.md CONTEXT.md; then exit 1; fi; target/debug/atelier --help | rg -F "atelier work ready"; target/debug/atelier work --help | rg -F "missions  List mission records by issue_type"; target/debug/atelier issue --help | rg -F "show        Show issue details"; cargo fmt -- --check; git diff --check; target/debug/atelier check atelier-p0am'
```

Exit status: 0

## Stdout

Bytes: 114
Truncated: no

```text
  atelier work ready
  missions  List mission records by issue_type
  show        Show issue details
Lint passed.
```

## Stderr

Bytes: 0
Truncated: no

```text
```
