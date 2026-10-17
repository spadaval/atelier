---
created_at: "2026-10-17T01:10:57.961551559+00:00"
id: "atelier-k55o"
issue_type: "bug"
labels:
- "cli-output"
- "issue-commands"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Refuse to create work items without a real Done when"
updated_at: "2026-10-17T01:10:57.961551559+00:00"
---

## Goal

`atelier issue create "x"` and `atelier issue create "x" --body "Plain goal text"` both succeed. They write placeholder sections: `## Done when\n\nDone when was not specified.` (and `Goal was not specified.` when there is no body). `atelier check <id>` then fails with `Issue section Done when must be present and non-empty`. The whole-repo `atelier check` passes, because placeholder checks only run when an ID is given (`crates/atelier-app/src/lint.rs:150-153`, `scoped`). `atelier start` then starts the empty item. `issue update --body` already runs the per-record `check` rules (`crates/atelier-cli/src/commands/issue.rs:916-934`), but `create` (`issue.rs:600-700`, `lifecycle_issue_sections` at 704) does not.

## Done when

- `issue create` refuses a work item without a real Goal and Done when (a mission without Outcome and Success criteria, a decision without Question). The error names the missing section and shows an example `--body`. Records created in `backlog` may be exempt only if the error/warning says so explicitly.
- Whole-repo `atelier check` reports placeholder acceptance sections on todo/active records, not only `check <id>`.
- `atelier start` refuses a record whose acceptance sections are placeholders.
- CLI tests cover create with no body, with plain text, and with full sections, plus `check` and `start` on a placeholder record.

## Unknowns

Whether backlog items may be filed with placeholders (capture now, refine later). If so, `plan`/`ready` out of backlog must enforce the sections.
