---
created_at: "2026-06-19T19:39:24.709461241+00:00"
id: "atelier-bd8j"
issue_type: "spike"
labels:
- "audit"
- "prune"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Inventory current artifact and branch growth"
updated_at: "2026-06-19T19:39:24.709461241+00:00"
---

## Description

Inventory the current repository's accumulated artifacts so the retention
contract and implementation are grounded in real growth patterns, not only
abstract cleanup categories.

## Outcome

- Inventory counts canonical issues, missions, evidence records, activity
  sidecars, native review records if present, ignored runtime/cache/diagnostic
  files, local Git branches, and Git worktrees.
- The spike classifies which observed items are protected, eligible only for
  local cleanup, eligible for Git cleanup, or candidates for future canonical
  archive/compaction.
- Follow-up risks are linked to implementation or contract work instead of
  silently widening scope.

## Evidence

- Evidence record includes the inventory commands, bounded output summaries,
  and classification notes.
- `atelier lint atelier-bd8j` passes.
