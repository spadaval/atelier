---
created_at: "2026-07-09T16:31:16.198659564+00:00"
id: "atelier-h4i8"
evidence_type: "test"
captured_at: "2026-07-09T16:31:15.925058034+00:00"
command: "bash -lc 'atelier work ready | tee /tmp/atelier-2uim-ready-open.txt; ! rg \"atelier-(amfw|wyxn|l5mw)\" /tmp/atelier-2uim-ready-open.txt'"
exit_status: "0"
agent_identity: "agent-factory.validate"
target:
  kind: "issue"
  id: "atelier-2uim"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-2uim"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'atelier work ready | tee /tmp/atelier-2uim-ready-open.txt; ! rg \"atelier-(amfw|wyxn|l5mw)\" /tmp/atelier-2uim-ready-open.txt'"
updated_at: "2026-07-09T16:31:20.618977270+00:00"
---

## Summary

bash -lc 'atelier work ready | tee /tmp/atelier-2uim-ready-open.txt; ! rg "atelier-(amfw|wyxn|l5mw)" /tmp/atelier-2uim-ready-open.txt'

## Command

```console
bash -lc 'atelier work ready | tee /tmp/atelier-2uim-ready-open.txt; ! rg "atelier-(amfw|wyxn|l5mw)" /tmp/atelier-2uim-ready-open.txt'
```

Exit status: 0

## Stdout

Bytes: 1417
Truncated: no

```text
Atelier Work
============

Ready Work
----------
  ready atelier-4fip [todo] high - Epic: Build the formatted Mission Overview
  ready atelier-ikuv [todo] high - Superseded umbrella: Command surface consolidation and removal
  ready atelier-nzu9 [todo] high - Epic: Rework issue list as simple inventory
  ready atelier-pnuk [todo] high - Avoid failed transition activity records blocking clean-worktree retries
  ready atelier-rgpl [todo] high - Archive or compact eligible canonical records
  ready atelier-tcai [todo] high - Make bundle preview and apply share a normalized graph plan
  ready atelier-tdgs [todo] high - Build the mission-to-epic overview projection
  ready atelier-txf6 [todo] high - Epic: Retention and prune contract
  ready atelier-u4gx [todo] high - Add Agent Factory references and an Atelier work-model man page
  ready atelier-w1z8 [todo] high - Prune stale branches and removable worktrees
  ready atelier-x3dy [todo] high - Add prune inventory and dry-run report
  ready atelier-yf06 [todo] high - Implement a flat issue inventory read and render path
  ready atelier-a0h0 [todo] medium - Clarify or remove generic issue fields storage
  ready atelier-bd8j [todo] medium - Inventory current artifact and branch growth
  ready atelier-f55w [todo] medium - Prune ignored local runtime, cache, and diagnostics
  ready atelier-24xn [ready] medium - Prune stale Atelier artifacts and branches
```

## Stderr

Bytes: 0
Truncated: no

```text
```
