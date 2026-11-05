---
created_at: "2026-11-05T05:17:15.686356292+00:00"
id: "atelier-rdsa"
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
title: "Tidy debrief Results and validation links"
updated_at: "2026-11-05T06:07:19.198719+00:00"
---

## Goal

Nits: Results proof chips show only the receipt ID without its kind; each timeline 'How' link goes to the top of Validation, not its run; the Results header leaves the criterion and mark columns unlabelled; chips stand out from the page by text alone (border 1.13:1); a records-only trivial waiver lists an item under Changed course.

## Done when

Each nit is fixed or consciously kept (with the reason in the Close Reason), and a CLI fixture that reproduces the fixed nits shows them fixed in the HTML debrief. (Rewritten 2026-11-05: atelier-vt8q's records become prunable after 2026-11-11.)

## Context

Found by the S7 agent re-review of atelier-vt8q's debrief at 7518f516 (FOLLOW-UP).
