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
title: "Add missions and the planning commands: `new`, `set`, `ready`, `drop`, `close`, `list`, `show`, `lead` and `start --new`"
updated_at: "2026-11-10T02:49:55.360277533+00:00"
---

## Goal

Let a planner write and ready work, and a lead run a mission, on the new records.

## Done when

- [ ] Provisional: a scenario test takes a mission from `new --mission` through two plan reviews made in separate checkouts and cherry-picked without conflict, `ready` on the mission and on one of two items, `lead`, and the item's work. The other item stays draft and cannot be started, and `close` on the mission is refused while it is draft; after `drop` on it, `close` succeeds. A started item dropped on the base shows as dropped from the base and from its branch, and `merge` on it is refused. `start --new` creates an item already building on its branch. Validation rejects an item whose `mission` names no mission. `show <mission>` prints the mission on one page: outcome, criteria, items grouped by status with who holds each and what blocks each, open questions and reviews. `list` with no flags prints all open work grouped by mission and then label, and `--ready`, `--blocked`, `--needs-human`, `--mission`, `--label` and `--mine` narrow it.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- What step A's scenario runs change in `status`, which `lead` and `list` share.
- Which flags of `list` show finished missions, answered questions and work in progress, so that today's `work missions --all`, `work decisions` and `work active` lose nothing (planning finding atelier-50br).
