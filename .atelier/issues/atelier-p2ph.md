---
created_at: "2026-06-14T21:44:40.239656525+00:00"
id: "atelier-p2ph"
issue_type: "task"
labels:
- "migration"
- "product"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-1cib"
  - kind: "issue"
    id: "atelier-3q31"
  - kind: "issue"
    id: "atelier-3vzm"
  - kind: "issue"
    id: "atelier-6nqr"
  - kind: "issue"
    id: "atelier-8gum"
  - kind: "issue"
    id: "atelier-ccja"
  - kind: "issue"
    id: "atelier-l543"
  - kind: "issue"
    id: "atelier-noly"
  - kind: "issue"
    id: "atelier-o2z8"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Define command and migration target state for the new model"
updated_at: "2026-06-14T21:44:40.239656525+00:00"
---

## Description

Specify the public command shape and migration behavior before implementation. Outcome: docs identify mission worktree commands, epic branch commands, removed issue-worktree defaults, branch naming, cleanup behavior, and no compatibility aliases unless requested. Evidence: product command docs and issue examples are consistent and pass docs/help drift checks where available.

## Outcome

- Product command docs specify mission worktree commands, epic branch commands, branch/path naming, and cleanup behavior.
- Product docs name the issue-worktree default as removed or exceptional, not normal workflow.
- The migration target state identifies how existing issue worktrees and branches will be classified.

## Evidence

- Product docs diff shows the command surface and migration target state.
- Docs/help drift check or focused transcript shows documented commands match CLI help where implemented.
- `atelier lint atelier-p2ph` passes.
