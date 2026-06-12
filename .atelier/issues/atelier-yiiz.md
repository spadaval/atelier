---
created_at: "2026-06-11T18:22:54.124647136+00:00"
id: "atelier-yiiz"
issue_type: "epic"
labels:
- "assignee:root"
- "markdown"
- "records"
- "recordstore"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-afir"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Move first-class record writes onto RecordStore"
updated_at: "2026-06-11T18:47:16.692189436+00:00"
---

## Description

Move first-class record mutations onto RecordStore-owned Markdown writes before projection refresh. Scope includes missions, plans, evidence, record links, mission blockers/work links, plan revisions, evidence attachments, and bulk plan apply durable outputs. Out of scope: issue lifecycle and issue-only relationship tables. Acceptance: mission/plan/evidence command writes update Markdown first; record_links are projected from canonical relationships; detail/list/workflow validation survives rebuild; tests cover create/update/link/attach/revise/bulk apply and fresh checkout rebuild.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.
