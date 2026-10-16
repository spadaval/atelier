---
created_at: "2026-10-16T18:33:32.869081622+00:00"
id: "atelier-7hh4"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Harden clean base merge detection"
updated_at: "2026-10-16T18:33:32.869081622+00:00"
---

## Goal

Close the gaps review found in atelier-n7g1's clean base merge rule.

## Done when

1. `clean_merge` cannot be satisfied by a configured merge driver or attributes producing content from neither side (atelier-6hpy): run `git merge-tree` isolated from repository merge drivers, or document the caveat.
2. With no work owner, the clean-merge side check uses the real base branch tip, not `HEAD^` (atelier-9l63).
3. A test covers an approval pinned before the branch left the base being refused after the base is merged in (atelier-1y1z).

## Context

Deferred findings from the atelier-n7g1 review (approval atelier-s7ea).
