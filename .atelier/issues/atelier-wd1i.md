---
created_at: "2026-11-10T00:56:09.915045297+00:00"
id: "atelier-wd1i"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5zre"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-oci2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-scx2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-t747"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-u1ko"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Run the cutover on this host"
updated_at: "2026-11-10T01:39:35.366711469+00:00"
---

## Goal

Make the change of tool once, on this host, by running the reviewed runbook.

## Done when

- [ ] Provisional: `scripts/cutover/cutover.sh` ran to its end on this host: all five repositories are converted and pushed, `atelier check` passes in each under the installed new binary, and the `atelier-serve` service is stopped and disabled, since the new binary has no `serve`. This item was closed with `atelier close`, the first act the new tool recorded.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Protocol: `docs/strategies/redesign-architecture.md`, Transition, part 4, with its rollback table. Do not run `atelier start` on this item: the runbook stops while any item is in progress, and `atelier close` ends an item that was never started. Run it on the base. This item changes no file by hand: the runbook's conversion commits are the only direct pushes, and the mission's Authority allows them.

## Unknowns

- When no work is in flight in any of the five repositories and no other thread is running in them.
