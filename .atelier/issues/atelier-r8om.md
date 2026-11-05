---
created_at: "2026-11-05T04:40:10.911964810+00:00"
id: "atelier-r8om"
issue_type: "bug"
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
title: "Tell planning reviews from delivery reviews in the debrief"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Goal

l7gl's approval by claude-review-planning-2 (sdkp) and its finding ywqm were reviews of the plan record, but the debrief counts them as review of the delivered work. Records have no field to tell them apart.

## Done when

A CLI fixture that reproduces the case in a scratch repository shows the problem fixed in both the Markdown and HTML debrief. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11, so its debrief cannot stay the check.)

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).
