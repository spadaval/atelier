---
created_at: "2026-11-05T05:17:15.293246972+00:00"
id: "atelier-1hbk"
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
title: "Name every part a collapsed debrief entry hides"
updated_at: "2026-11-05T05:17:15.293246972+00:00"
---

## Goal

On quiet entries (vt8q's g8cn, l7gl, esu1) the "Review, decisions and follow-ups" toggle also hides Surprised by and Left open, including l7gl's note that the plan-mode build and publish jobs first run on the next tag push, the one unproven part of S2.

## Done when

The collapsed label names every part it hides, or Surprised by and Left open stay visible, with a fixture test.

## Context

Found by the S7 agent re-review of atelier-vt8q's debrief at 7518f516 (FOLLOW-UP).
