---
created_at: "2026-11-06T21:26:18.777067374+00:00"
id: "atelier-5822"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-8s1m"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Records ride with the commits they describe"
updated_at: "2026-11-06T21:26:18.777067374+00:00"
---

## Goal

Record-only commits fall to the floor that pinning forces, by letting records travel with the commit they describe or be batched, wherever pinning allows.

## Done when

Provisional: record-only commits in the lifecycle scenario are at most half the baseline (with the disposition and re-check changes counted); every command that writes records commits them in the way the new rule says; nothing a gate reads becomes unpinned.

## Unknowns

The floor measured by the first increment decides how much this item can reach.
