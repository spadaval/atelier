---
created_at: "2026-06-29T17:37:32.255559024+00:00"
id: "atelier-kzu2"
issue_type: "epic"
labels:
- "cli"
- "mission"
- "workflow"
review:
  kind: pull_request
  number: 33
  provider: forgejo
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2uim"
  children:
  - kind: "issue"
    id: "atelier-97p7"
  - kind: "issue"
    id: "atelier-i0ze"
  - kind: "issue"
    id: "atelier-see0"
  - kind: "issue"
    id: "atelier-yr2v"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1mga"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Make mission lifecycle workflow-configured"
updated_at: "2026-06-30T15:18:29.343285331+00:00"
---

## Description

Mission commands derive lifecycle behavior from .atelier/workflow.yaml instead of hardcoded mission status lists. Changing mission statuses, initial state, transitions, terminal state, or display categories in workflow policy changes mission behavior without Rust lifecycle edits.

## Outcome

Mission lifecycle behavior is controlled by workflow policy. Mission commands no longer hardcode a separate lifecycle vocabulary for valid statuses, initial status, current versus terminal status, transition availability, or dashboard ordering.
