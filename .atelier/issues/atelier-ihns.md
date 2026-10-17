---
created_at: "2026-10-16T23:04:57.934982250+00:00"
id: "atelier-ihns"
issue_type: "feature"
labels:
- "orientation"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-ihns
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1w8c"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Show the between-missions state in status and work missions"
updated_at: "2026-10-17T03:15:20.745451266+00:00"
---

## Goal

When no mission is open, `status` says "Current missions: 0" and `work missions` says "No missions match", and the open work collapses to "Unassigned: 13". An agent cannot tell that the last mission finished or what the remaining work is about. Show it.

## Done when

With no open mission, `status` and `work missions` name the most recently closed mission and its close date, and list unassigned open work grouped by label (unlabelled last), at most 5 items per group with a count of the rest. With open missions, unassigned work is still grouped the same way below them. Tests cover zero open missions, one open mission, and unlabelled work.

## Scope

The status and Mission Overview renderers and their tests. No new record fields.

## Context

Seen on master on 2026-10-16: mission atelier-kgcr done, 13 unassigned issues. `docs/product/issue-inventory-and-mission-overview.md`.
