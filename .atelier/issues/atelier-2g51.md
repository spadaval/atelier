---
created_at: '2026-06-11T22:14:53.209649393+00:00'
id: atelier-2g51
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments:
  - kind: evidence
    id: atelier-2yxv
    role: validates
  relates:
  - kind: issue
    id: atelier-1e24
    type: advances
  - kind: issue
    id: atelier-3gki
    type: advances
  - kind: issue
    id: atelier-481n
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-11T22:24:19.580403169+00:00'
status: closed
title: Make missions actually work
updated_at: '2026-06-11T22:24:19.580403169+00:00'
---

## Description

Turn missions from advisory records into enforced workflow objects. Validators must return enforceable results, failed validators must fail commands, and mission closeout must be impossible while required gates fail, including a dirty Git worktree.

## Outcome

### Constraints

- Mission closeout requires all linked work closed, evidence attached, workflow validators passing, and a clean Git worktree.
- Keep mission close out of v1; status=closed is the single enforced transition path.

### Risks

- None.

## Evidence

- Manual check: workflow validate fails nonzero on validator failure
- Manual check: mission update --status closed rejects dirty worktree and other closeout blockers
- Manual check: mission status reports concrete closeout blockers
- Manual check: final closeout records cargo fmt, cargo nextest, export --check, lint, doctor, and clean git status

## Notes

Migrated from `.atelier/missions/atelier-2g51.md` as a declared mission objective issue.
