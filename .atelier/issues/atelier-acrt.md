---
created_at: "2026-10-17T17:25:59.520021809+00:00"
id: "atelier-acrt"
issue_type: "feature"
labels:
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "issue"
    id: "atelier-rx22"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Review a mission before work starts with atelier brief"
updated_at: "2026-10-17T17:25:59.520021809+00:00"
---

## Goal

S5 of docs/strategies/review-and-introspection.md: `atelier brief <mission>` reviews a mission before work starts, in both forms, and shows the human's plan approval (a review event on the mission). The debrief shows the approval too.

## Done when

The S5 criterion holds, shown by a fixture test of both forms.

## Unknowns

Whether review events can target a mission today, or need a change to the review gate. Sizes: which existing field gives an item's size.
