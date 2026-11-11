---
schema: atelier.item
schema_version: 3
id: atelier-1hbk
title: Name every part a collapsed debrief entry hides
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
created_at: 2026-11-05T05:17:15.293246972Z
---

## Goal

On quiet entries (vt8q's g8cn, l7gl, esu1) the "Review, decisions and follow-ups" toggle also hides Surprised by and Left open, including l7gl's note that the plan-mode build and publish jobs first run on the next tag push, the one unproven part of S2.

## Done when

- [ ] The collapsed label names every part it hides, or Surprised by and Left open stay visible, with a fixture test.

## Context

Found by the S7 agent re-review of atelier-vt8q's debrief at 7518f516 (FOLLOW-UP).

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: atelier-lfs7 (Agent-review a real mission's HTML debrief)
