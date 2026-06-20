---
created_at: "2026-06-20T04:17:09.570108923+00:00"
id: "atelier-y8cs"
issue_type: "task"
labels:
- "doctor"
- "forgejo"
- "review"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-l7i6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Unify Forgejo review config and doctor readiness checks"
updated_at: "2026-06-20T04:17:09.570108923+00:00"
---

## Description

Imported bundle issue.

## Outcome

- Provider review commands load the same Forgejo role-author configuration as workflow review actions.
- `atelier doctor` checks provider review readiness before missions hit review transitions.
- Failures name the missing configuration or permission and the repair path.

## Evidence

- Review commands and workflow actions use one Forgejo config path for role authors; doctor reports missing token/config/role-author/repo-access failures with remediation.
