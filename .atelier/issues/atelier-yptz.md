---
created_at: "2026-11-10T00:56:07.800857010+00:00"
id: "atelier-yptz"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-67nn"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add missions and the planning commands: `new`, `set`, `ready`, `drop`, `close`, `list`, `show` and `lead`"
updated_at: "2026-11-10T00:56:07.800857010+00:00"
---

## Goal

Let a planner write and ready work, and a lead run a mission, on the new records.

## Done when

- [ ] Provisional: a scenario test takes a mission from `new --mission` through plan review, `ready`, `lead`, its items and `close`, and each command behaves as the product specification's Commands section says.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- What step A's scenario runs change in `status`, which `lead` and `list` share.
