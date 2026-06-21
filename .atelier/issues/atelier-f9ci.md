---
created_at: "2026-06-21T16:37:30.770740525+00:00"
id: "atelier-f9ci"
issue_type: "epic"
labels:
- "mission-rework"
- "validation"
review:
  kind: pull_request
  number: 22
  provider: forgejo
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-76j0"
  - kind: "issue"
    id: "atelier-y3fj"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Validate mission rework end to end"
updated_at: "2026-06-21T20:03:31.351013662+00:00"
---

## Description

Prove the mission rework as a complete operator workflow, including docs/help parity, old-command removal, migration, and configured workflow behavior.

## Outcome

- A new mission/objective can be declared, created, linked to work, inspected, blocked, unblocked, validated, and closed through issue/status/evidence commands.
- Old mission-only commands are absent or rejected according to the no-alias policy.
- Existing repository mission data survives migration and rebuild.
- Docs, help, Agent Factory guidance, tests, and tracker state agree.

## Evidence

- Independent validation evidence maps each mission outcome to implementation issues and proof records.
- `cargo fmt -- --check`, focused `cargo nextest run`, `atelier lint`, `git diff --check`, and command-surface searches pass.
