---
created_at: "2026-06-14T21:45:38.869255767+00:00"
id: "atelier-hrmj"
issue_type: "task"
labels:
- "branch"
- "migration"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T04:33:59.725580892+00:00"
status: "done"
title: "Fold preserved issue branches into epic branches"
updated_at: "2026-06-15T04:33:59.725580892+00:00"
---

## Description

For dirty or uncontained issue branches, decide whether to fold changes into new epic branches, retain them temporarily, or abandon them with evidence. Outcome: no useful work is lost and remaining branches have an explicit epic/mission owner. Evidence: per-branch decision log, merge/cherry-pick transcript where applicable, and final branch status.

## Outcome

- Dirty or uncontained issue branches are folded into appropriate epic branches, explicitly retained, or abandoned with a recorded reason.
- Remaining branches have a mission or epic owner.
- No useful unmerged work is lost during migration.

## Evidence

- Evidence record or command transcript includes a per-branch decision log for each dirty or uncontained branch.
- Merge or cherry-pick transcript proves preserved changes moved into epic branches where applicable.
- Final `git branch` and `git status --short --branch` transcript shows remaining branch ownership and clean state.
