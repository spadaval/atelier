---
created_at: "2026-06-20T04:31:19.739896981+00:00"
id: "atelier-o5a9"
issue_type: "task"
labels:
- "sessions"
- "status"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-eomn"
  - kind: "issue"
    id: "atelier-l7i6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove validator attempts from session tracking"
updated_at: "2026-06-20T04:31:19.739896981+00:00"
---

## Description

Imported bundle issue.

## Outcome

- Validator attempts are removed from the derived session/role-attempt model.
- Evidence records and evidence links remain the validation audit trail.
- Worker and reviewer attempts remain available for implementation and review attribution.
- Status output no longer reports stale validator sessions from evidence attachment.

## Evidence

- `atelier status` and `atelier session list --active` no longer show validator attempts; evidence attachment records evidence activity without opening validator sessions; focused tests prove terminal issues do not retain active validator-derived attempts.
