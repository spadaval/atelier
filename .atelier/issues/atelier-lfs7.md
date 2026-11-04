---
created_at: "2026-10-17T17:25:59.636065900+00:00"
id: "atelier-lfs7"
issue_type: "validation"
labels:
- "needs-human"
- "review"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2bjz"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-r0fj"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "issue"
    id: "atelier-rx22"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Validate a real mission's HTML debrief with the human"
updated_at: "2026-11-04T23:16:56.641533855+00:00"
---

## Goal

S7 of docs/strategies/review-and-introspection.md: a real mission that ran with decision capture and reports switched on produces an HTML debrief that the human judges good enough for post-mission review.

## Done when

The human opens the debrief in T3 Code and records a review event naming the commit it was generated from.

## Context

Waits for atelier-2bjz and atelier-r0fj, which change the debrief page the human judges.

## Unknowns

Which mission. atelier-9y66 kept the orientation mission atelier-1w8c, and the human judged its debrief "reasonably good" on 2026-10-22 without a review event; 1w8c has since been superseded into atelier-4pzj. atelier-vt8q (release distribution) closes with every item reported under the Close Reason report, so it is a fresh candidate; judging it instead changes 9y66's choice and needs the human.
