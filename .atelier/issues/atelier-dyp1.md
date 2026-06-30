---
created_at: "2026-06-29T17:47:31.411840618+00:00"
id: "atelier-dyp1"
issue_type: "task"
labels:
- "reliability"
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T04:08:58.708485586+00:00"
status: "done"
title: "Implement closeout failure classification and waiver rules"
updated_at: "2026-06-30T04:08:58.708485586+00:00"
---

## Description

Mission closeout cannot bury failing default checks as unrelated background noise. Closeout behavior must classify failures as fixed, linked to an owner/blocker, or explicitly waived by a human with the reason recorded.

## Outcome

Mission closeout implements failure classification for default checks. Closeout output and stored state classify each failing required check as fixed, linked blocker, or human-approved waiver. A failed suite cannot be dismissed as outside the mission without a durable owner or recorded waiver reason.
