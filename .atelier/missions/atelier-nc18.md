---
created_at: "2026-06-11T00:06:58.557537388+00:00"
id: "atelier-nc18"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-000i"
    type: "advances"
  - kind: "issue"
    id: "atelier-001m"
    type: "advances"
  - kind: "issue"
    id: "atelier-001p"
    type: "advances"
  - kind: "issue"
    id: "atelier-001q"
    type: "advances"
  - kind: "issue"
    id: "atelier-001r"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Local diagnostics and telemetry"
updated_at: "2026-06-11T19:54:27.791432915+00:00"
---

## Intent

Superseded by mission atelier-n8ag, Autonomous mission operations.

Original intent preserved: add local-only command telemetry and diagnostics so operators can identify slow commands and performance issues without committing runtime logs into durable project state.

The completed diagnostics foundation work is now linked directly to atelier-n8ag as supporting done work.

## Constraints

- Diagnostics data stays outside .atelier-state by default, supports disable/override controls, and documents redaction and retention behavior.

## Risks

- Telemetry can leak sensitive command context or create nondeterministic tracker state if storage and redaction boundaries are not explicit.

## Validation

- Linked issues prove storage/redaction policy, command instrumentation, disabled telemetry behavior, failed-command telemetry, and stable slow-command query output.
