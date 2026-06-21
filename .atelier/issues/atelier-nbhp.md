---
created_at: "2026-06-21T16:37:30.768607040+00:00"
id: "atelier-nbhp"
issue_type: "epic"
labels:
- "migration"
- "mission-rework"
review:
  kind: pull_request
  number: 20
  provider: forgejo
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-iv2x"
  - kind: "issue"
    id: "atelier-kka3"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Migrate mission records and projections"
updated_at: "2026-06-21T19:39:27.140347484+00:00"
---

## Description

Move canonical mission state and projection behavior to the target declared-policy model without preserving duplicate mission storage or command shims.

## Outcome

- Existing mission records migrate directly to the target canonical shape.
- Projection rebuild derives mission/objective views from canonical issue/objective records and relationships.
- Legacy mission storage and projection paths are removed once migration is complete.

## Evidence

- Migration tests cover current repository fixtures and malformed legacy mission records.
- `atelier rebuild`, `atelier lint`, and focused projection tests pass.
