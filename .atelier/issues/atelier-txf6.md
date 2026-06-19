---
created_at: "2026-06-19T19:38:44.648948114+00:00"
id: "atelier-txf6"
issue_type: "epic"
labels:
- "artifact-update"
- "prune"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t3h3"
  children:
  - kind: "issue"
    id: "atelier-bd8j"
  - kind: "issue"
    id: "atelier-kpa1"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Retention and prune contract"
updated_at: "2026-06-19T19:39:24.710266179+00:00"
---

## Description

Settle the product and architecture contract for pruning before any command can
delete or compact records, local state, branches, or worktrees. This epic owns
the terminology, retention classes, eligibility rules, and proof obligations
for the prune workflow.

## Outcome

- SPEC.md, product docs, architecture docs, and any needed ADR state how Atelier
  classifies tracked canonical records, evidence metadata, evidence payloads,
  issue activity sidecars, ignored runtime/cache data, diagnostics, branches,
  and worktrees for pruning.
- The command contract distinguishes dry-run reporting from destructive apply
  behavior and names the safety defaults for protected records and Git state.
- Implementation work under `atelier-t3h3` can proceed without guessing which
  artifacts may be deleted, archived, compacted, or only reported.

## Evidence

- Documentation diff and review notes map each retention class to an observable
  prune behavior.
- `atelier lint atelier-txf6` passes after child work lands.
- Attached evidence names unresolved policy risks before the implementation epic
  is unblocked.
