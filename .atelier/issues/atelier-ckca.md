---
created_at: "2026-06-23T16:21:20.079723985+00:00"
id: "atelier-ckca"
issue_type: "epic"
labels: []
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0p7e"
  - kind: "issue"
    id: "atelier-idwz"
  - kind: "issue"
    id: "atelier-nxq9"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mska"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Rewrite domain-shaped cache schema"
updated_at: "2026-06-23T16:21:20.582583953+00:00"
---

## Description

Rewrite the SQLite cache schema and rebuild code to use domain-shaped tables instead of the current hybrid issue tables plus generic non-issue record tables.

## Outcome

- SQLite is a domain-shaped cache, not a mirror of the record-file storage abstraction.

## Evidence

- The cache schema contains domain query tables and cache source metadata.
- The generic records table is removed or demoted from the target cache model.
