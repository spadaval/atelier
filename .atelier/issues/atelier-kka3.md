---
created_at: "2026-06-21T16:37:30.770057010+00:00"
id: "atelier-kka3"
issue_type: "task"
labels:
- "cleanup"
- "mission-rework"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-76j0"
  - kind: "issue"
    id: "atelier-f9ci"
  - kind: "issue"
    id: "atelier-y3fj"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove mission-specific projection and storage branches"
updated_at: "2026-06-21T16:37:30.770057010+00:00"
---

## Description

Delete storage and projection code that treats missions as a separate lifecycle engine after migrated records use the target shape.

## Outcome

- Projection rebuild no longer needs special mission table behavior beyond generic relationship-derived objective views.
- Record kind routing no longer contains mission-only write/read paths unless the contract explicitly keeps first-class mission records.
- Lint and doctor report migration/config issues in product terms.

## Evidence

- Code search transcript shows removed mission-specific projection branches or documents the intentionally retained narrow boundary.
- `cargo nextest run` focused storage/projection tests and `atelier lint` pass.
