---
created_at: "2026-11-09T01:04:23.024399152+00:00"
id: "atelier-wo22"
issue_type: "validation"
labels:
- "agent-attribution"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-73vx"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-bnek"
    type: "advances"
  - kind: "issue"
    id: "atelier-wpj2"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Validate who-did-what on a real Claude-implemented, Codex-reviewed item"
updated_at: "2026-11-09T01:04:23.024399152+00:00"
---

## Goal

Independently show the attribution mission's criterion who-did-what holds on a real item.

## Done when

- [ ] On one real item implemented in a Claude thread and reviewed in a Codex thread after attribution landed, the item's page names each thread's provider, model, effort and role, and each thread link opens that thread in T3.
- [ ] Each role's token cost on the page matches a hand count from the provider transcripts within the unattributed share atelier-73vx reports.
- [ ] A receipt for who-did-what records the item, the threads, both counts and any differences.
