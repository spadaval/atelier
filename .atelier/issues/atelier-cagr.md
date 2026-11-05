---
created_at: "2026-11-05T02:39:54.652320320+00:00"
id: "atelier-cagr"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-j79t"
    type: "supersedes"
  - kind: "review"
    id: "atelier-dswi"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T06:02:30.512669172+00:00"
status: "superseded"
title: "Decide a debrief child's Open items by its own follow-ups"
updated_at: "2026-11-05T06:02:30.512669172+00:00"
---

## Goal

In the debrief, every child's "Left open" shows only when the parent entry's follow-ups are empty, and that list gathers follow-ups filed on the parent and all its children, so a follow-up filed on one child hides another child's unrelated Open items in both forms (review finding atelier-dswi on atelier-ov97; crates/atelier-cli/src/commands/mission_review.rs around :954 and :1938).

## Done when

Each child's Open items are hidden only by follow-ups filed on that child, shown by a CLI fixture where a follow-up on one child leaves a sibling's Open items visible in both forms.

## Close Reason

Same debrief rule as atelier-j79t (mission_review.rs:961 hides Open items whenever an entry has follow-ups). j79t is the broader defect and now carries this item's per-child fixture in its Done when.
