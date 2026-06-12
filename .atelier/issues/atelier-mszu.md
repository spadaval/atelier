---
created_at: "2026-06-11T18:22:53.237475165+00:00"
id: "atelier-mszu"
issue_type: "epic"
labels:
- "assignee:root"
- "issues"
- "markdown"
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
title: "Move issue lifecycle writes onto RecordStore"
updated_at: "2026-06-11T18:41:42.757697389+00:00"
---

## Description

Move issue lifecycle mutations onto RecordStore-owned Markdown writes before projection refresh. Scope includes issue create, subissue, update fields, status changes, close/reopen, delete, parent changes, and close-all behavior. Out of scope: labels, dependencies, typed links, missions, plans, and evidence. Acceptance: representative lifecycle commands remain recoverable from .atelier-state without export as normal writer; issue activity remains canonical; projection refresh follows the shared contract; tests cover create/update/close/reopen/delete/subissue and rebuild-from-checkout.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.
