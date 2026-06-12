---
created_at: "2026-06-11T16:14:26.191466697+00:00"
id: "atelier-b3gc"
issue_type: "feature"
labels:
- "markdown"
- "recordstore"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Move public durable mutations onto RecordStore"
updated_at: "2026-06-11T18:23:25.686942626+00:00"
---

## Description

Normal durable mutation commands are recoverable because they write .atelier-state through the compatibility export path, but most public mutations still use SQLite as the mutation engine. Move issue lifecycle, labels/dependencies, typed links, missions, plans, and evidence onto RecordStore-owned Markdown writes before projection refresh; retire export-as-normal-writer once covered.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.
