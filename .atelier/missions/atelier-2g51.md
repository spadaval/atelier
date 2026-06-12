---
created_at: "2026-06-11T22:14:53.209649393+00:00"
id: "atelier-2g51"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "evidence"
    id: "atelier-2yxv"
    type: "validates"
  - kind: "issue"
    id: "atelier-1e24"
    type: "advances"
  - kind: "issue"
    id: "atelier-3gki"
    type: "advances"
  - kind: "issue"
    id: "atelier-481n"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Make missions actually work"
updated_at: "2026-06-11T22:24:19.580403169+00:00"
---

## Intent

Turn missions from advisory records into enforced workflow objects. Validators must return enforceable results, failed validators must fail commands, and mission closeout must be impossible while required gates fail, including a dirty Git worktree.

## Constraints

- Mission closeout requires all linked work closed, evidence attached, workflow validators passing, and a clean Git worktree.
- Keep mission close out of v1; status=closed is the single enforced transition path.

## Risks

- None.

## Validation

- workflow validate fails nonzero on validator failure
- mission update --status closed rejects dirty worktree and other closeout blockers
- mission status reports concrete closeout blockers
- final closeout records cargo fmt, cargo nextest, export --check, lint, doctor, and clean git status
