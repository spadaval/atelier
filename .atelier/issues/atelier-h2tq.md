---
created_at: "2026-06-13T02:35:57.892462110+00:00"
id: "atelier-h2tq"
issue_type: "feature"
labels:
- "cli"
- "evidence"
- "ux"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-rzsg"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Render compact evidence summaries with drill-down"
updated_at: "2026-06-13T02:35:57.892462110+00:00"
---

## Description

Reduce evidence-output noise while preserving inspectability. Mission and issue views should show concise evidence status by default and provide drill-down for full transcripts or audit details.

## Outcome

- Mission and issue output summarizes evidence by ID, result, kind, target, and covered claims.
- Full evidence bodies are available through an explicit drill-down command.
- Long evidence records no longer dominate default mission output.

## Evidence

- Snapshot or transcript tests cover default compact rendering and verbose drill-down.
- Manual transcript demonstrates a mission with long evidence remains scannable.
