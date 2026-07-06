---
created_at: "2026-06-29T20:11:54.461532604+00:00"
id: "atelier-ye11"
issue_type: "epic"
labels:
- "cli"
- "complexity"
- "review"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-vqhi"
  children:
  - kind: "issue"
    id: "atelier-8kv0"
  - kind: "issue"
    id: "atelier-odwi"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-durs"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Simplify review command surface"
updated_at: "2026-06-29T20:12:28.955830174+00:00"
---

## Description

Review commands are useful, but the current surface mirrors provider operations and asks for low-level branch/title/body plumbing in normal paths. Apply the complexity budget so review operations are issue/workflow-derived by default and provider machinery is recovery-only.

## Outcome

The review surface has explicit Keep/Simplify/Fold decisions. Routine review creation derives issue, branch, title, body, and provider context from workflow state. Submit-like operations are collapsed or justified. Help, docs, and tests prove normal review work does not require manual provider plumbing.
