---
schema: atelier.item
schema_version: 3
id: atelier-lozn
title: Make the debrief's verdict links and More toggles visibly interactive
status: draft
mission: null
labels:
- review
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-05T05:17:15.478853596Z
---

## Goal

The .verdict links are text-coloured with a 1.2:1 border (WCAG F73), and the details.more toggle hides its marker and differs from body text by colour alone (2.14:1 light, 1.54:1 dark).

## Done when

- [ ] Both controls are distinguishable without colour (underline, marker or 3:1 contrast against surrounding text) in both palettes.

## Context

Found by the S7 agent re-review of atelier-vt8q's debrief at 7518f516 (FOLLOW-UP).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-lfs7 (Agent-review a real mission's HTML debrief)
