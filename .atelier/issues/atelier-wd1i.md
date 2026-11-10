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
    id: "atelier-6sti"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-t747"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Cut this host over: convert the five repositories and install the new binary and skill"
updated_at: "2026-11-10T00:56:09.915045297+00:00"
---

## Goal

Run the runbook in the architecture specification's Transition section on this host.

## Done when

- [ ] Provisional: the mission criteria `five-converted` and `no-old-surface`; the rollback was tried once on a scratch clone beforehand.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- When no work is in flight in any of the five repositories.
