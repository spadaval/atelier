---
created_at: "2026-06-14T16:31:33.611558920+00:00"
id: "atelier-uran"
issue_type: "task"
labels:
- "docs"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Update command-surface docs for hidden diagnostics without workarounds"
updated_at: "2026-06-14T16:31:33.611558920+00:00"
---

## Description

Docs should be able to name hidden diagnostic commands in hidden/advanced contexts without pretending they are visible workflow surfaces.

## Outcome

- Command-surface docs can spell out hidden diagnostic commands in a clearly
  hidden/advanced section without avoiding command-shaped text as a workaround.
- The docs state that hidden diagnostics are not normal mission, issue,
  blocker, evidence, or closeout next actions.
- Drift checks pass with those explicit hidden command references.

## Evidence

- Docs diff includes at least one explicit hidden diagnostic command reference
  in an approved hidden/advanced context.
- `target/debug/atelier mission status <mission>` or the command-surface drift
  check reports Docs/Help Drift clear.
- `git diff --check` and `atelier lint` pass.
