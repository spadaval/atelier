---
created_at: "2026-06-29T20:13:43.814775362+00:00"
id: "atelier-eqq6"
issue_type: "epic"
labels:
- "admin"
- "cli"
- "complexity"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-vqhi"
  children:
  - kind: "issue"
    id: "atelier-g87o"
  - kind: "issue"
    id: "atelier-ie31"
  - kind: "issue"
    id: "atelier-mxnv"
  - kind: "issue"
    id: "atelier-vqdm"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-durs"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Hide or remove provider and recovery escape hatches"
updated_at: "2026-06-29T20:15:29.399052097+00:00"
---

## Description

Provider setup, branch recovery, destructive maintenance, and raw diagnostics are valuable only when explicitly routed by setup or recovery. They should not appear as normal workflow or role-guide paths.

## Outcome

Forgejo/provider setup, branch recovery, maintenance delete, and hidden diagnostic commands have explicit Hide, Fold, Keep, or Remove decisions. Root help, role guides, command audit, and tests agree that these surfaces are not normal workflow. Any surviving recovery command is reachable only from explicit setup, failed transition, check, or admin recovery guidance.
