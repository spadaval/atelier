---
created_at: "2026-11-10T00:56:10.176751966+00:00"
id: "atelier-2mlh"
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
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-wd1i"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Delete the old library code, and run a first item in a product repository"
updated_at: "2026-11-10T01:14:42.300877655+00:00"
---

## Goal

Remove what the cutover left unreachable, and find what the rehearsal could not.

## Done when

- [ ] Provisional: the workflow crate, the old record types, the gate code, evidence, bundle, migrate and prune are deleted, with non-test Rust reported before and after; one item in a product repository has gone from ready to done; each fix made along the way is inside the specifications or was escalated.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- What first use breaks.
