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
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Cut this host over: convert atelier and switch the binary in one item, then convert the other four repositories"
updated_at: "2026-11-10T01:14:42.029304934+00:00"
---

## Goal

Make the change of tool once, on this host, as the one item the architecture specification's Transition section describes.

## Done when

- [ ] Provisional: the mission criteria `five-converted` and `no-old-surface`. The item started under the old tool, the converter converted it in flight, and its review and merge ran under the binary built from its branch. The rollback was tried once on scratch clones beforehand.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- When no other work is in flight in any of the five repositories.
