---
created_at: "2026-06-19T19:39:18.753462512+00:00"
id: "atelier-kpa1"
issue_type: "task"
labels:
- "artifact-update"
- "prune"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-f55w"
  - kind: "issue"
    id: "atelier-rgpl"
  - kind: "issue"
    id: "atelier-w1z8"
  - kind: "issue"
    id: "atelier-x3dy"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-23T21:23:59.228454261+00:00"
status: "done"
title: "Define retention classes and prune safety contract"
updated_at: "2026-06-23T21:23:59.228454261+00:00"
---

## Description

Define the retention and pruning contract for Atelier's generated and
long-lived artifacts. This is an artifact-update task and must land before
implementation slices choose deletion or compaction behavior.

## Outcome

- Retention classes are documented for canonical issues, missions, evidence
  records, evidence payload references, issue activity sidecars, native review
  rooms, local runtime/cache state, command diagnostics, Git branches, and Git
  worktrees.
- The docs name which classes can be deleted, archived, compacted, externally
  retained, or only reported in v1.
- Safety rules cover active work, non-terminal work, blockers, required proof,
  unmerged branches, dirty worktrees, protected base branches, and configured
  age thresholds.
- The public command shape and help expectations are documented without adding
  compatibility aliases or fallback commands.

## Evidence

- File changes in `SPEC.md`, `docs/product/`, `docs/architecture/`, or
  `docs/adr/` demonstrate the contract and link back to the mission validation
  criteria.
- `rg` or equivalent transcript shows old ambiguous cleanup guidance is not
  left as conflicting product guidance.
- `atelier lint atelier-kpa1` passes after the artifact update.
