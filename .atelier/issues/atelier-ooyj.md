---
created_at: "2026-06-14T21:43:54.615953604+00:00"
id: "atelier-ooyj"
issue_type: "epic"
labels:
- "migration"
- "worktree"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-hrmj"
  - kind: "issue"
    id: "atelier-lfwg"
  - kind: "issue"
    id: "atelier-zbd4"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Migrate and clean existing issue worktrees"
updated_at: "2026-06-14T21:45:38.869255767+00:00"
---

## Description

Classify and migrate the current codex/atelier-* issue-worktree state into the mission-worktree/epic-branch model. Outcome: clean contained issue worktrees are removed, dirty worktrees are reviewed or salvaged, unmerged branches are folded into appropriate epic branches or explicitly retained, and stale Git registrations no longer break status. Evidence: migration transcript, classification table, cleanup commands, and final git worktree/branch status.

## Outcome

- Existing issue worktrees and `codex/atelier-*` branches are classified under the new mission-worktree/epic-branch model.
- Safe clean contained worktrees and stale registrations are removed.
- Dirty or uncontained work is preserved, folded into epic branches, or explicitly abandoned with evidence.

## Evidence

- Child issue proof from atelier-zbd4, atelier-lfwg, and atelier-hrmj maps to classification, removal, and branch folding.
- Before/after worktree, branch, and disk-usage transcripts show cleanup results.
- Migration evidence records any retained branches and their mission or epic owner.
