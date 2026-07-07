---
created_at: "2026-06-23T16:21:20.071295422+00:00"
id: "atelier-pxxj"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-01T05:58:23.664744372+00:00"
status: "done"
title: "Document record-file, domain, and cache boundaries"
updated_at: "2026-07-01T05:58:23.664744372+00:00"
---

## Description

Update architecture and ADR material to replace canonical/projection phrasing with record-file/project-state/cache terminology and describe the target three-layer model.

## Outcome

- Architecture docs describe record files as project state, domain objects as concrete types, and SQLite as a lazy cache.

## Evidence

File changes in `CONTEXT.md`, `PRODUCT_INTENT.md`, and
`docs/architecture/markdown-first-record-store.md` define the record-file,
concrete-domain-type, and SQLite-domain-cache boundaries.
