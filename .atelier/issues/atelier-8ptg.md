---
created_at: "2026-06-11T16:06:43.274863790+00:00"
id: "atelier-8ptg"
issue_type: "epic"
labels:
- "assignee:root"
- "cache"
- "markdown"
- "projection"
- "sqlite"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Convert SQLite projection to metadata-only index"
updated_at: "2026-06-11T16:16:38.419539034+00:00"
---

## Description

Reduce SQLite from a broad shadow copy of canonical Markdown into a metadata/index-only ProjectionIndex. The goal is to keep enough derived rows for fast lists, ready-work discovery, graph traversal, workflow checks, search inputs, and Mission Control summaries, while reading full record content directly from Markdown when detail views need it.

## Outcome

- ProjectionIndex ownership is documented as metadata/index-only, not a complete replication of Markdown content.

- A table/field audit classifies existing SQLite columns as projection metadata, runtime state, compatibility residue, or removal candidates.

- At least one representative query/detail path proves the pattern: use SQLite to locate/sort candidates, then read full Markdown content from RecordStore for detail rendering.

- Mission Control requirements are reflected: frequent polling can use small metadata projections without reparsing or mirroring every Markdown body.

- Rebuild, export --check, lint, doctor, and existing issue/mission workflows remain green.

- Follow-up issues are created for any content-heavy tables or command paths that cannot be migrated in this slice.

## Evidence

Evidence was not specified in the legacy issue record.

## Notes

### Scope

- Audit SQLite tables and command reads to identify which fields are true query/index metadata versus replicated canonical content.
- Define the minimal ProjectionIndex schema for records, labels, timestamps, relationship edges, source hashes, and derived readiness/workflow flags.
- Move rich canonical content such as bodies, descriptions, plan/evidence data, activity text, and detailed record payloads out of the projection path where practical.
- Update detail commands and future Mission Control panel assumptions to query metadata first, then load selected Markdown records from RecordStore.
- Preserve full rebuild as a compatibility/fallback path while preparing targeted per-record refresh.
