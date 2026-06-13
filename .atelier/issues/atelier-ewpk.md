---
created_at: "2026-06-13T17:29:11.074433010+00:00"
id: "atelier-ewpk"
issue_type: "task"
labels:
- "cleanup"
- "docs"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-09sx"
  - kind: "issue"
    id: "atelier-fyms"
  - kind: "issue"
    id: "atelier-q5r6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove misleading legacy workflow surfaces"
updated_at: "2026-06-13T17:36:58.862190950+00:00"
---

## Description

Remove the stale command and documentation surfaces that made repo-defined workflow policy look implemented before it was enforced. This cleanup should happen after the configured transition and operator command replacements exist.

## Outcome

- Current atelier workflow validate is removed from the public and hidden command surface, with transition gate inspection owned by issue transition --options.
- Root atelier.workflow.yaml worktree hook behavior is removed in favor of the explicit .atelier/workflow.yaml policy contract and deferred hook design.
- finish-oriented help, docs, Agent Factory binding, status next actions, and product docs are replaced with close or abandon language.
- Docs no longer present deferred workflow config features as implemented normal behavior.

## Evidence

- Residue scan classifies remaining workflow validate, finish, and atelier.workflow.yaml references as removed, historical, or explicitly deferred.
- Help transcript proves normal commands teach workflow init/check/migrate, issue transition, start, close, and abandon.
- atelier lint, atelier export --check, and docs whitespace checks pass.
