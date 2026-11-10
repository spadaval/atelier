---
created_at: "2026-11-10T00:56:08.858805140+00:00"
id: "atelier-nafc"
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
    id: "atelier-axfa"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-pz1y"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Write the converter, and rehearse it and the new binary on all five repositories"
updated_at: "2026-11-10T01:14:41.224064285+00:00"
---

## Goal

Prove the converter and the new binary on real data before the cutover depends on them.

## Done when

- [ ] Provisional: `scripts/convert_records.py` and `atelier-next` pass `check` and complete one item on a scratch clone of each of the five repositories (the mission criterion `rehearsal`), and the converter converts one item in flight as `building`.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- What the conversion does with records the new model has no place for, such as evidence and decision records on open items.
