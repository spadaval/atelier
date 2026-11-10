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
    id: "atelier-oci2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-pz1y"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Convert and rehearse on all five repositories, then delete the old commands and rename the binary"
updated_at: "2026-11-10T00:56:08.858805140+00:00"
---

## Goal

Prove the new binary on real data, then remove the old command layer and its tests from the tree.

## Done when

- [ ] Provisional: `scripts/convert_records.py` and the new binary pass `check` and complete one item on a scratch clone of each of the five repositories (the mission criterion `rehearsal`); the old modules, commands and tests are deleted; `atelier-next` is renamed `atelier`; CI's `check` step pins 0.4.0; the Report gives non-test Rust before and after.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- What the conversion does with records the new model has no place for, such as evidence and decision records on open items.
