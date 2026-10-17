---
created_at: "2026-10-17T01:10:57.972018696+00:00"
id: "atelier-w4j2"
issue_type: "bug"
labels:
- "check"
- "cli-output"
priority: "P2"
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
title: "Stop atelier check reporting other record kinds as corrupt records"
updated_at: "2026-10-17T01:10:57.972018696+00:00"
---

## Goal

`atelier check <id>` assumes every full ID is an issue:

- `atelier check atelier-029b` (a review) prints `Lint found 1 issue(s): atelier-029b: Tracker record files are invalid: Missing record file .atelier/issues/atelier-029b.md` and exits 1, a false corruption report.
- `atelier check atelier-zzzz` (nonexistent) prints the same corruption message, while `atelier check zzzz` says `Error: Issue zzzz was not found`.
- `atelier check atelier-yjxb` (evidence) says `atelier-yjxb is a evidence record, not an issue record. Use atelier evidence show atelier-yjxb.` and does not check it. Short `check yjxb` says `Issue yjxb was not found`.

## Done when

- Unknown IDs, short or full, give one `atelier-zzzz not found` error and never claim the tracker is invalid.
- Review and evidence IDs, short or full, are checked like issues (or rejected with a correct message naming their kind).
- Tests cover a review ID, an evidence ID, and an unknown ID in both forms.

## Context

Entry point: crates/atelier-cli/src/main.rs:920-923 → commands::issue::lint (crates/atelier-cli/src/commands/issue.rs:1026) → atelier_app::lint.
