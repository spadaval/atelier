---
created_at: "2026-06-14T05:59:03.571673734+00:00"
id: "atelier-vj08"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-4yrt"
  - kind: "issue"
    id: "atelier-a85s"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Add init import-beads migration flag"
updated_at: "2026-06-14T07:49:17.930858215+00:00"
---

## Description

Add `atelier init --import-beads` to import repo-local
`.beads/issues.manual.jsonl` explicitly during setup while leaving
`import-beads` callable only until the init path is implemented.

## Outcome

`atelier init --import-beads` imports the standard repo-local Beads migration
input `.beads/issues.manual.jsonl` explicitly during setup. The standalone
`import-beads` command may remain callable until this init path is complete,
but it is not shown in general root help.

## Evidence

- `atelier init --help` documents `--import-beads` and the standard
  `.beads/issues.manual.jsonl` source.
- Fixture test or transcript proves init detects repo-local Beads migration
  input and imports only when `--import-beads` is supplied.
- Root help does not list `import-beads` as a general command.
- Existing `import-beads` tests still pass while the transitional command
  remains available.
- `git diff --check`, `atelier lint`, and focused import/init tests pass.

## Notes

This issue owns the Beads import flag. Broader init/workflow setup work should
depend on this issue instead of reimplementing the same migration flag.
