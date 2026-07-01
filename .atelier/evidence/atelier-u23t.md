---
created_at: "2026-07-01T06:00:40.216218497+00:00"
id: "atelier-u23t"
evidence_type: "test"
captured_at: "2026-07-01T06:00:39.822091770+00:00"
command: "bash -lc 'set -euo pipefail; for page in abandon graph mission note plan repair search start worktree; do test \"$(wc -l < \"docs/product/command-audit/$page.md\")\" -le 10; done; rg -n \"^# (Retired|Deferred) `atelier (abandon|graph|mission|note|plan|repair|search|start|worktree)`$\" docs/product/command-audit/{abandon,graph,mission,note,plan,repair,search,start,worktree}.md'"
exit_status: "1"
target:
  kind: "issue"
  id: "atelier-fmb7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-fmb7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "All retired command audit pages are compact tombstones and point only to current owner surfaces."
updated_at: "2026-07-01T06:00:46.180110956+00:00"
---

## Summary

All retired command audit pages are compact tombstones and point only to current owner surfaces.

## Command

```console
bash -lc 'set -euo pipefail; for page in abandon graph mission note plan repair search start worktree; do test "$(wc -l < "docs/product/command-audit/$page.md")" -le 10; done; rg -n "^# (Retired|Deferred) `atelier (abandon|graph|mission|note|plan|repair|search|start|worktree)`$" docs/product/command-audit/{abandon,graph,mission,note,plan,repair,search,start,worktree}.md'
```

Exit status: 1

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 190
Truncated: no

```text
bash: command substitution: line 1: syntax error near unexpected token `abandon'
bash: command substitution: line 1: `atelier (abandon|graph|mission|note|plan|repair|search|start|worktree)'
```

