---
created_at: "2026-06-29T20:13:07.527643415+00:00"
id: "atelier-3g1y"
issue_type: "task"
labels:
- "cli"
- "evidence"
- "relationships"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Decide and implement evidence attach ownership"
updated_at: "2026-06-29T20:13:07.527643415+00:00"
---

## Description

`evidence attach` may be a duplicate relationship mutation rather than a distinct proof job. Decide whether it earns its own command or should fold into the general link/relationship model.

## Outcome

Evidence reuse has one clear owner. If `evidence attach` survives, docs and tests prove it is simpler than a general relationship mutation for cross-kind proof links. If it does not survive, help/docs route reuse through the chosen relationship command and rejected old forms do not remain compatibility aliases.
