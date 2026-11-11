---
created_at: "2026-11-05T04:40:10.556389088+00:00"
id: "atelier-wg6p"
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
    id: "atelier-lfs7"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Don't turn record mentions inside paths and branch names into chips"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Goal

Chips split paths and branch names: '~/.cache/[atelier-g6a1]/s4-check.sh', 'task/[atelier-ot1c] e2e2cf0d', 'bug/[atelier-vo0z] e70496f7'. Mentions inside a path or ref should stay text.

## Done when

A CLI fixture that reproduces the case in a scratch repository shows the problem fixed in both the Markdown debrief and the site's debrief page. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11, so its debrief cannot stay the check.)

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).
