---
created_at: "2026-07-01T06:01:37.689048177+00:00"
id: "atelier-xso3"
evidence_type: "test"
captured_at: "2026-07-01T06:01:37.386864809+00:00"
command: "bash -lc 'set -euo pipefail; for page in abandon graph mission note plan repair search start worktree; do test $(wc -l < docs/product/command-audit/$page.md) -le 10; done; ! rg -n '\"'\"'^##|^\\|'\"'\"' docs/product/command-audit/{abandon,graph,mission,note,plan,repair,search,start,worktree}.md; rg -n '\"'\"'^# (Retired|Deferred)'\"'\"' docs/product/command-audit/{abandon,graph,mission,note,plan,repair,search,start,worktree}.md'"
exit_status: "0"
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
title: "Retired command audit files have compact headings and no walkthrough tables or secondary sections."
updated_at: "2026-07-01T06:01:43.564891328+00:00"
---

## Summary

Retired command audit files have compact headings and no walkthrough tables or secondary sections.

## Command

```console
bash -lc 'set -euo pipefail; for page in abandon graph mission note plan repair search start worktree; do test $(wc -l < docs/product/command-audit/$page.md) -le 10; done; ! rg -n '"'"'^##|^\|'"'"' docs/product/command-audit/{abandon,graph,mission,note,plan,repair,search,start,worktree}.md; rg -n '"'"'^# (Retired|Deferred)'"'"' docs/product/command-audit/{abandon,graph,mission,note,plan,repair,search,start,worktree}.md'
```

Exit status: 0

## Stdout

Bytes: 591
Truncated: no

```text
docs/product/command-audit/worktree.md:1:# Retired `atelier worktree`
docs/product/command-audit/start.md:1:# Retired `atelier start`
docs/product/command-audit/search.md:1:# Retired `atelier search`
docs/product/command-audit/repair.md:1:# Retired `atelier repair`
docs/product/command-audit/plan.md:1:# Deferred `atelier plan`
docs/product/command-audit/note.md:1:# Retired `atelier note`
docs/product/command-audit/graph.md:1:# Retired `atelier graph`
docs/product/command-audit/mission.md:1:# Retired `atelier mission`
docs/product/command-audit/abandon.md:1:# Retired `atelier abandon`
```

## Stderr

Bytes: 0
Truncated: no

```text
```
