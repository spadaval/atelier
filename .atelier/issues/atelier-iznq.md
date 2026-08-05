---
created_at: "2026-07-28T16:39:31.749240322+00:00"
id: "atelier-iznq"
issue_type: "task"
labels:
- "architecture"
- "planning"
- "product-contract"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2a1t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Publish the lightweight strategy and planning-layer contract"
updated_at: "2026-08-05T16:43:27.127727601+00:00"
---

## Description

Update the owning product, domain, architecture, and command-surface sources using the audit evidence. Record an ADR only if the rationale must survive beyond those contracts.

## Outcome

- Durable repository guidance cleanly separates strategic product intent, outcome maps, current implementation tasks, and proof.
- The contract specifies how missions reference strategy, how draft epics become ready through expansion, and how Managers replan within adaptation authority.
- The documented workflow remains ordinary Markdown plus typed issue relationships and introduces no approval or identity gate.

## Evidence

- Strategy revision 2, ADR 0018, product intent, domain language, mission authoring, validation, workflow, and persistence guidance agree on the adaptive model.
- Documentation residue searches find no requirement for expansion flags, validation profiles, work dispositions, or replanning records.
