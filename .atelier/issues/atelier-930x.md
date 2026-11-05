---
created_at: "2026-11-05T04:40:09.822181923+00:00"
id: "atelier-930x"
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
title: "Show every receipt in a debrief validation run, not only those with criteria"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Goal

The Validation section groups receipts by producer and commit but drops receipts that carry no criteria: atelier-z0q5 (S1's negative check, same producer, commit and second as its run) is missing, and atelier-a755 and atelier-m2ht appear nowhere, though gddg's text points at its evidence.

## Done when

A CLI fixture that reproduces the case in a scratch repository shows the problem fixed in both the Markdown and HTML debrief. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11, so its debrief cannot stay the check.)

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).
