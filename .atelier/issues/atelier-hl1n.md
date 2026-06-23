---
created_at: "2026-06-23T16:21:20.073661392+00:00"
id: "atelier-hl1n"
issue_type: "epic"
labels: []
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-ax1g"
  - kind: "issue"
    id: "atelier-muzq"
  - kind: "issue"
    id: "atelier-sdqy"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mska"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Unify record-file storage and domain models"
updated_at: "2026-06-23T16:21:20.267807317+00:00"
---

## Description

Refactor storage and domain boundaries so shared record-file mechanics do not become the primary domain abstraction.

## Outcome

- The codebase has unified record-file storage mechanics and concrete domain boundaries for issue, evidence, and review behavior.

## Evidence

- Issue, evidence, and review domain logic operate on concrete domain types.
- Record-file parsing, rendering, discovery, and atomic writes are shared at the storage layer.
