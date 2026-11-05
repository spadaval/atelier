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
status: "backlog"
title: "Make the debrief's verdict links and More toggles visibly interactive"
updated_at: "2026-11-05T05:17:15.478853596+00:00"
---

## Goal

The .verdict links are text-coloured with a 1.2:1 border (WCAG F73), and the details.more toggle hides its marker and differs from body text by colour alone (2.14:1 light, 1.54:1 dark).

## Done when

Both controls are distinguishable without colour (underline, marker or 3:1 contrast against surrounding text) in both palettes.

## Context

Found by the S7 agent re-review of atelier-vt8q's debrief at 7518f516 (FOLLOW-UP).
