---
created_at: "2026-11-05T02:21:07.690164399+00:00"
id: "atelier-ov97"
issue_type: "bug"
labels: []
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-ov97
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "evidence"
    id: "atelier-obcr"
    type: "discovered_from"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Show each done child's report in the debrief"
updated_at: "2026-11-05T02:21:11.933869708+00:00"
---

## Goal

S4 of docs/strategies/review-and-introspection.md says the debrief contains each item's report. A done child shows only as a `└ <child>` line under its parent: its Deviations, Surprises and Open items appear in neither form, and its Outcome only in the HTML hover card (acceptance validation receipt atelier-obcr; in `atelier debrief atelier-r6bt`, atelier-w5hx's Surprises and Open items appear nowhere).

## Done when

Both debrief forms show every done child's Outcome and its non-empty report parts, shown by a CLI fixture with a child closed with Deviations and Surprises under a done parent.

## Context

crates/atelier-cli/src/commands/mission_review.rs renders a WorkEntry's own report and lists children as item lines; review_page.rs builds the HTML. Whether the debrief also needs a separate decision log (the receipt's second failure) is a question for the human, not this item.
