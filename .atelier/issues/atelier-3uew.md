---
created_at: "2026-06-24T20:37:10.879967751+00:00"
id: "atelier-3uew"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-8c91"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z0ll"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Expand command-surface drift checks across visible grouped commands"
updated_at: "2026-06-24T20:37:10.879967751+00:00"
---

## Description

Fix partial command-surface drift checking. `COMMAND_GROUP_ROOTS` only covers some grouped roots, so stale subcommands under visible groups such as `review` or `forgejo` can pass drift checks as long as the root exists. Audit evidence points to `crates/atelier-cli/src/command_surface.rs:16` and `crates/atelier-cli/src/command_surface.rs:552`.

Constraints:
- Do not hard-code only the latest observed missing groups if recursive help discovery is practical.
- Drift checks should support the z0ll help/man shared command model and removed-command validation.

Risks:
- Docs/help drift can silently preserve removed review/provider subcommands and weaken command-surface cleanup proof.

## Outcome

Command-surface drift checking covers all visible grouped command roots, either by recursively discovering them from help or by explicitly listing every visible grouped root with tests. Removed grouped commands cannot survive in docs, tests, man guidance, or Agent Factory references merely because their root command still exists.

## Evidence

- Focused tests fail on stale references to removed review subcommands such as `review approve --body` and provider commands such as `forgejo roles check` when those surfaces are no longer visible.
- Root help, focused help, man guidance, docs, and Agent Factory references are checked consistently for removed grouped commands.
- `atelier lint` or the successor `atelier check` exercises the strengthened drift check.
