---
created_at: "2026-06-13T01:05:13.808839808+00:00"
id: "atelier-bpm0"
issue_type: "task"
labels:
- "rebuild"
- "tests"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v9id"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Repair rebuild ID-collision mission fixture"
updated_at: "2026-06-13T01:05:13.808839808+00:00"
---

## Description

Repair the broad-suite failure in commands::rebuild::tests::rebuild_rejects_global_id_collision_across_record_kinds. The test fixture creates a mission record without required mission sections, so rebuild fails on mission body parsing before it can assert the intended global ID collision behavior.

## Outcome

- The rebuild ID-collision fixture uses a valid sectioned mission body.
- The test still proves global IDs cannot collide across issue and domain record kinds.
- The default broad-suite fail-fast probe advances past the previous malformed mission fixture failure.

## Evidence

- cargo nextest run rebuild_rejects_global_id_collision_across_record_kinds passes.
- cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet advances past the previous malformed mission fixture failure.
- atelier lint, atelier export --check, and git diff --check pass.
