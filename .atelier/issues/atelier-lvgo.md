---
created_at: "2026-06-18T16:20:42.979098526+00:00"
id: "atelier-lvgo"
issue_type: "validation"
labels:
- "session-pr-overhaul"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate session/PR attribution overhaul"
updated_at: "2026-06-18T16:33:54.421425630+00:00"
---

## Description

Validate the session/PR attribution overhaul end to end after the documentation, issue-event activity model, CLI behavior, and PR enforcement work lands.

## Outcome

The overhaul has focused regression coverage for session-as-issue-events, PR attribution, inspection-only session output, and PR merge behavior, and the full required validation command set passes.

## Evidence

Validation transcript records scenario proof for one-active-PR enforcement, PR issue inference, automatic worker/reviewer/validator issue-event attribution, `session list/show` projections, `pr merge` behavior without workflow side effects, docs/help/man parity, plus `cargo fmt -- --check`, `cargo nextest run`, `cargo nextest run --profile extended --run-ignored=only`, `atelier export --check`, `atelier lint`, and `atelier doctor`.
