---
created_at: "2026-11-10T00:56:10.442628646+00:00"
id: "atelier-6w8d"
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
    id: "atelier-2mlh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Replace the product docs, `CONTEXT.md`, `PRODUCT_INTENT.md` and the ADRs with the design record"
updated_at: "2026-11-10T00:56:10.442628646+00:00"
---

## Goal

Leave one current description of the system, edited in place, with the list of what was tried and removed.

## Done when

- [ ] Provisional: `docs/adr/` and `docs/strategies/` are gone, open missions' documents are in their directories, and no doc describes a removed command.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- Which pages under `docs/product/` survive as they are.
