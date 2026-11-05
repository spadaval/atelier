---
created_at: "2026-11-05T04:40:11.289744306+00:00"
id: "atelier-ztuq"
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
title: "Tidy small debrief inaccuracies from the S7 review"
updated_at: "2026-11-05T04:40:11.289744306+00:00"
---

## Goal

Nits from the S7 review: the mission chip card's Outcome shows raw '### Outcome'; the verdict omits zero counts; no year appears anywhere; each 'Delivered after this validation' list includes the validation item itself; gddg (split out by a fix-now finding) shows as plainly Added.

## Done when

The debrief of atelier-vt8q no longer shows the problem, and a fixture test covers it.

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).
