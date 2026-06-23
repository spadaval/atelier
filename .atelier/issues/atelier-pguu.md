---
created_at: "2026-06-23T20:17:03.769299859+00:00"
id: "atelier-pguu"
issue_type: "feature"
labels:
- "branch"
- "cli"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ih42"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Make branch.prepare explicit in workflow configuration"
updated_at: "2026-06-23T21:39:25.654179527+00:00"
---

## Description

Remove runtime auto-injection of branch preparation. Add explicit branch.prepare actions to the repository workflow transitions that start executable work, and reject or report missing branch preparation only when a configured branch/review action actually requires a branch.

Make branch context action-scoped instead of ambient transition state: transitions
with no branch or review action must not resolve an expected branch, branch owner,
or base branch just to render options, perform status changes, or record ordinary
activity.

Choose and document one public workflow action spelling for branch preparation
before broad docs/tests are written. The command, workflow YAML, product docs,
and diagnostics should not split between two spellings for branch preparation.

## Outcome

- `.atelier/workflow.yaml` visibly declares branch.prepare where branch setup belongs.
- Start transitions no longer receive hidden actions from the planner.
- Non-branch transitions do not compute or display expected branch, branch owner,
  or base branch details as a side effect of transition planning.
- Tests prove branch.prepare creates or switches to the epic branch for epic
  children and to the issue branch for standalone issues.
- Tests or before/after transcripts prove at least one transition with no
  branch/review action still renders options and applies without branch policy
  resolution.
