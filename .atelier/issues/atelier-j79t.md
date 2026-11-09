---
created_at: "2026-11-05T04:40:10.011847246+00:00"
id: "atelier-j79t"
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
    id: "atelier-cagr"
    type: "supersedes"
  - kind: "issue"
    id: "atelier-lfs7"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Keep an item's Open items in the debrief when review filed follow-ups"
updated_at: "2026-11-05T06:03:04.102700+00:00"
---

## Goal

Open items are dropped whenever review filed follow-ups for the entry, but the two are not the same list: lnwf's untested push-event diff check and macOS timing headroom, 7v9q's unexercised tag-push path, and 5tl2's pre-release installer 404 appear nowhere. Show Open items alongside Filed, or decide the rule again.

## Done when

- [ ] A debrief entry shows its Open items alongside Filed follow-ups, or the rule for hiding them is decided again and written in docs/product/mission-reviews.md.
- [ ] A CLI fixture covers it in both forms, including atelier-cagr's case: a follow-up filed on one child leaves a sibling's Open items visible.

## Context

Found by the S7 agent review of atelier-vt8q's debrief at c73d55e8 (FOLLOW-UP).
