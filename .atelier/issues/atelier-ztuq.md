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
    id: "atelier-gg6b"
    type: "advances"
  - kind: "issue"
    id: "atelier-lfs7"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Tidy small debrief inaccuracies from the S7 review"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Goal

Nits from the S7 review: the mission chip card's Outcome shows raw '### Outcome'; the verdict omits zero counts; no year appears anywhere; each 'Delivered after this validation' list includes the validation item itself; gddg (split out by a fix-now finding) shows as plainly Added.

## Done when

A CLI fixture that reproduces the case in a scratch repository shows the problem fixed in both the Markdown and HTML debrief. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11, so its debrief cannot stay the check.)

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).
