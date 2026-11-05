---
created_at: "2026-11-05T04:40:10.371706005+00:00"
id: "atelier-okfy"
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
title: "Show the mission's own close report and item notes in the debrief"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Goal

The mission's own Close Reason report (outcome, deviations, decisions, surprises, open items) is not shown, including vt8q's explanation that two later code commits don't affect S1, S3 or S4; item notes such as g8cn's v0.3.1 publication note are missing although Left open points at them.

## Done when

A CLI fixture that reproduces the case in a scratch repository shows the problem fixed in both the Markdown and HTML debrief. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11, so its debrief cannot stay the check.)

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).
