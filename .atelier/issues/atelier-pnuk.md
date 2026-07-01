---
created_at: "2026-07-01T15:36:20.930236626+00:00"
id: "atelier-pnuk"
issue_type: "bug"
labels:
- "bug"
- "tracker"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Avoid failed transition activity records blocking clean-worktree retries"
updated_at: "2026-07-01T15:36:20.930236626+00:00"
---

## Description

Failed transition attempts are intentionally recorded as canonical activity. When a transition requires git.worktree_clean, a prior failed attempt can leave an uncommitted activity record that causes the next retry to fail its clean-worktree validator. Preserve auditability while making retries ergonomic: either exempt generated failed-attempt activity from the clean validator or keep it in runtime state until explicitly checkpointed. Reproduced during atelier-411x epic close: the initial close was blocked by uncommitted validation evidence; its recorded transition_blocked activity then appeared as the sole dirty entry on the retry. Code path: crates/atelier-cli/src/commands/workflow.rs report_blocked_transition records activity after validators; crates/atelier-app/src/workflow_validation.rs git_worktree_clean considers all untracked files.

## Outcome

Outcome was not specified.
