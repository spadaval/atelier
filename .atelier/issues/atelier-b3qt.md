---
created_at: "2026-10-18T07:07:54.292074500+00:00"
id: "atelier-b3qt"
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
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Order the debrief timeline by one clock"
updated_at: "2026-10-18T07:07:54.292074500+00:00"
---

## Goal

The debrief timeline mixes clocks (review of atelier-8bil, finding atelier-krdy): delivered items sit at their record close time, set on the work branch, while work added to the scope sits at the first-parent commit that linked it, usually the merge. On atelier-r6bt, atelier-stsa shows as added at 15:04 after it was delivered as atelier-ho7o's child at 14:44. Order every event by one clock, or say which clock an event uses.

## Done when

A fixture where a child is closed on its parent's branch and merged later shows the child's addition no later than its delivery, shown by a CLI test.

## Context

crates/atelier-app/src/mission_review.rs `events`; crates/atelier-app/src/record_history.rs `mission_timeline` (first-parent walk). The nexd decision to show close times rather than start times explains why record times are used for delivery.
