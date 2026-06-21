---
created_at: "2026-06-21T16:37:30.765735744+00:00"
id: "atelier-vays"
issue_type: "epic"
labels:
- "mission-rework"
review:
  kind: pull_request
  number: 19
  provider: forgejo
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-2kfb"
  - kind: "issue"
    id: "atelier-62po"
  - kind: "issue"
    id: "atelier-e7t1"
  - kind: "issue"
    id: "atelier-fyc9"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-21T19:20:16.177083507+00:00"
status: "done"
title: "Epic: Collapse mission command surface into issue commands"
updated_at: "2026-06-21T19:20:16.177083507+00:00"
---

## Description

Remove mission-only command surfaces after issue commands own rich objective reads, focused status, links, blockers, notes, and transitions.

## Outcome

- No visible `atelier mission` root command remains unless the contract identifies a mission-only job that cannot live under `issue` or `status`.
- Issue commands provide the mission/objective operator path with corrective wrong-kind guidance.
- Help, role guides, docs, and tests no longer teach removed mission-only commands.

## Evidence

- Root help and command-surface lint show no removed mission namespace.
- Focused CLI tests cover replacement commands and old-command rejection.
