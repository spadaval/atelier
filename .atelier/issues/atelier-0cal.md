---
created_at: "2026-10-17T01:10:57.968045340+00:00"
id: "atelier-0cal"
issue_type: "task"
labels:
- "bundle"
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Point bundle apply's next step at the new graph and at committing"
updated_at: "2026-10-17T01:10:57.968045340+00:00"
---

## Goal

After a successful apply, `Next Commands` lists `atelier issue show <first created id>` (an arbitrary record) and `atelier check` (crates/atelier-cli/src/commands/bundle.rs:1555-1565). `check` is redundant because apply already validated the staged graph. The step that matters, committing the new records, is missing. docs/product/human-cli-output.md says next actions include "commit record changes".

## Done when

- When the bundle created or updated a mission, apply ends with `Next: atelier work mission <id>`; for a parent without a mission, `atelier work parent <id>`; otherwise `atelier issue show <id>` for the single top-level item.
- A `Commit: git add .atelier && git commit` line follows.
- `atelier check` is dropped from the footer.
- A test asserts the footer for a mission bundle.
