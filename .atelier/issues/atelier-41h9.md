---
created_at: "2026-06-20T04:17:09.550006890+00:00"
id: "atelier-41h9"
issue_type: "epic"
labels:
- "cli"
- "review"
- "workflow"
review:
  kind: pull_request
  number: 13
  provider: forgejo
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-2sma"
  - kind: "issue"
    id: "atelier-4i5f"
  - kind: "issue"
    id: "atelier-6jjm"
  - kind: "issue"
    id: "atelier-eomn"
  - kind: "issue"
    id: "atelier-l7i6"
  - kind: "issue"
    id: "atelier-o5a9"
  - kind: "issue"
    id: "atelier-sgav"
  - kind: "issue"
    id: "atelier-wehh"
  - kind: "issue"
    id: "atelier-y8cs"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-20T05:35:24.544215908+00:00"
status: "done"
title: "Epic: Workflow transition authority and merge integration"
updated_at: "2026-06-20T05:35:24.544215908+00:00"
---

## Description

Imported bundle issue.

## Outcome

- Workflow transitions are the only lifecycle authority for issue closure and merge/integration behavior.
- Provider-backed review uses provider merge authority; local review rooms keep an explicit local branch integration action.
- Operator surfaces derive next steps from workflow state instead of static prompts.

## Evidence

- Mission terminal proof shows provider-backed workflows no longer perform local merges, room workflows still have an explicit local integration path, and status/doctor surfaces give actionable workflow-derived guidance.
