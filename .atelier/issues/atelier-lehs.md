---
created_at: "2026-10-17T01:10:57.962696285+00:00"
id: "atelier-lehs"
issue_type: "task"
labels:
- "cli-output"
- "issue-commands"
priority: "P3"
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
title: "Drop or fix the Impact section in issue show"
updated_at: "2026-10-17T01:10:57.962696285+00:00"
---

## Goal

`issue show` always prints an Impact section (`render_impact_section`, `crates/atelier-cli/src/commands/issue.rs:440-476`), built from `downstream_impact` (`crates/atelier-app/src/record_index.rs:1001`). That function follows children plus `derived`/`caused-by`/`falsifies` links, but `caused-by` is not an accepted link role, so that branch can never match. On the done parent atelier-1ioo it relists its 11 children, already shown under Subissues, with `11 downstream issues may need review before changing or closing this issue.` That is advice that makes no sense for a closed record. On most items it prints `No downstream issues found.`

## Done when

One of:
- (a) The section is removed, along with `downstream_impact` if nothing else uses it; or
- (b) It shows only non-child dependents (blocked_by inverse is already "Blocking"; derived/falsifies links) on open records, under a clear label, and prints nothing when empty.

Dead role names (`caused-by`) are removed either way. Tests cover a done parent and an item with a `derived` link.

## Unknowns

Whether any reader uses Impact. Nothing in the transcripts references it.
