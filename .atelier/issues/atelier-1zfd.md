---
created_at: "2026-06-15T01:15:02.259400227+00:00"
id: "atelier-1zfd"
issue_type: "task"
labels:
- "product"
- "validation"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-jrtk"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T01:21:59.431473693+00:00"
status: "done"
title: "Define lightweight mission closeout contract"
updated_at: "2026-06-15T01:21:59.431473693+00:00"
---

## Description

Define the lightweight mission closeout contract in product and validation guidance. The contract should say when mission-level evidence is useful, when child issue/epic evidence is enough, and which failures mission status should still surface.

## Outcome

- `docs/product/zen.md`, `docs/product/work-model.md`, and `docs/architecture/quality/validation.md` agree that mission closeout is not a second validation pass over already-validated children.
- Mission closeout expectations name concrete checks: linked work complete, blockers clear, tracker/lint health current, mission intent covered, and any mission-specific risks explicitly addressed.
- Guidance distinguishes accountability expectations from mechanical gates, keeping routine mission closeout light unless the mission makes a cross-cutting claim that needs first-class evidence.

## Evidence

- File diff for `docs/product/work-model.md` and `docs/architecture/quality/validation.md` shows the simplified mission closeout contract.
- Command output from `rg -n "mission closeout|mission-level|independent validation" docs/product docs/architecture/quality` shows no remaining blanket requirement to revalidate all linked work at mission scope.
- `target/debug/atelier lint` passes after the documentation update.
