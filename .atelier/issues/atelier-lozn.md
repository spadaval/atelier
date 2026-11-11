---
created_at: "2026-11-05T05:17:15.478853596+00:00"
id: "atelier-lozn"
issue_type: "task"
labels:
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-lfs7"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:55.383319434+00:00"
status: "superseded"
title: "Make the debrief's verdict links and More toggles visibly interactive"
updated_at: "2026-11-11T03:55:55.383319434+00:00"
---

## Goal

The .verdict links are text-coloured with a 1.2:1 border (WCAG F73), and the details.more toggle hides its marker and differs from body text by colour alone (2.14:1 light, 1.54:1 dark).

## Done when

Both controls are distinguishable without colour (underline, marker or 3:1 contrast against surrounding text) in both palettes.

## Context

Found by the S7 agent re-review of atelier-vt8q's debrief at 7518f516 (FOLLOW-UP).

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes the site's debrief page, which the What is removed table removes with `serve`; the Desk (mission atelier-z4cr) shows the debrief.
