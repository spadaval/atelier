---
created_at: "2026-07-01T05:56:58.913533997+00:00"
id: "atelier-h0tc"
evidence_type: "validation"
captured_at: "2026-07-01T05:56:56.372663016+00:00"
command: "bash -lc 'set -euo pipefail; ! rg -n -g \"*.md\" \"atelier (mission status|mission list|issue status|work queue --(ready|blocked|status|category))\" docs .agents AGENTS.md README.md PRODUCT_INTENT.md CONTEXT.md SPEC.md; target/debug/atelier --help | rg -F \"atelier work ready\"; target/debug/atelier work --help | rg -F \"missions  List mission records by issue_type\"; target/debug/atelier issue --help | rg -F \"show        Show issue details\"; cargo fmt -- --check; git diff --check; target/debug/atelier check atelier-p0am'"
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
title: "Current command vocabulary and stale-form scan passed"
updated_at: "2026-07-01T05:57:04.810453819+00:00"
---

## Summary

Current command vocabulary and stale-form scan passed

## Command

```console
bash -lc 'set -euo pipefail; ! rg -n -g "*.md" "atelier (mission status|mission list|issue status|work queue --(ready|blocked|status|category))" docs .agents AGENTS.md README.md PRODUCT_INTENT.md CONTEXT.md SPEC.md; target/debug/atelier --help | rg -F "atelier work ready"; target/debug/atelier work --help | rg -F "missions  List mission records by issue_type"; target/debug/atelier issue --help | rg -F "show        Show issue details"; cargo fmt -- --check; git diff --check; target/debug/atelier check atelier-p0am'
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

Bytes: 52
Truncated: no

```text
rg: SPEC.md: No such file or directory (os error 2)
```
