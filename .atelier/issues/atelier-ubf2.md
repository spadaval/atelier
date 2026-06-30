---
created_at: "2026-06-29T18:21:24.681981303+00:00"
id: "atelier-ubf2"
issue_type: "validation"
labels:
- "cli"
- "dashboard"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Validate repaired issue list and mission dashboard CLI behavior"
updated_at: "2026-06-30T15:11:21.120439119+00:00"
---

## Description

Independently validate the restored issue inventory command, resolved work-view contract, and repaired mission dashboard after the implementation and documentation tasks land. The validator starts from the epic outcome and public CLI behavior, not from private chat context.

## Outcome

Validation records pass, fail, blocked, or deferred for the command claims in this epic. The record includes transcripts or test output for `atelier issue list` default and filtered inventory behavior, `atelier work ready` top-level picker behavior, `atelier work blocked` blocker-triage behavior, absence of normal role guidance that routes through `work queue --ready` or `work queue --blocked`, any surviving `atelier work queue` behavior if it remains public, `atelier work mission <mission-id>` default dashboard output, mission-scoped filter output, blocked-row blocker IDs, closeout-ready dashboard output, `atelier issue transition <id>` default output showing failed requirements without validator/action firehose detail, updated help/docs guidance, and the standard checks `cargo fmt -- --check`, `cargo nextest run`, `git diff --check`, and `atelier check`.
