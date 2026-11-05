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
closed_at: "2026-11-05T02:27:39.649685090+00:00"
status: "done"
title: "Show each done child's report in the debrief"
updated_at: "2026-11-05T02:27:39.649685090+00:00"
---

## Goal

S4 of docs/strategies/review-and-introspection.md says the debrief contains each item's report. A done child shows only as a `└ <child>` line under its parent: its Deviations, Surprises and Open items appear in neither form, and its Outcome only in the HTML hover card (acceptance validation receipt atelier-obcr; in `atelier debrief atelier-r6bt`, atelier-w5hx's Surprises and Open items appear nowhere).

## Done when

Both debrief forms show every done child's Outcome and its non-empty report parts, shown by a CLI fixture with a child closed with Deviations and Surprises under a done parent.

## Context

crates/atelier-cli/src/commands/mission_review.rs renders a WorkEntry's own report and lists children as item lines; review_page.rs builds the HTML. Whether the debrief also needs a separate decision log (the receipt's second failure) is a question for the human, not this item.

## Notes

- 2026-11-04 [decision] A child's reported Deviations make its parent's entry change course, listed under the parent's ID in Plan vs actual
  Reason: Its derived changes (reopened, failing proof, trivial waiver) already roll into the parent's entry, so the reported ones follow the same rule
  Rejected: listing the child's own ID under Changed course
- 2026-11-04 [decision] A child's Open items are shown only when review filed no follow-ups for its entry
  Reason: Mirrors the parent's rule, where filed follow-ups replace the Open items text; the entry's follow-ups already gather its children's

## Close Reason

Both debrief forms now show each done child's report under its child line: its Outcome, then any Deviations ("Changed"), Surprises ("Surprised by") and Open items ("Left open"). The fixture `a_done_childs_report_sits_with_its_child_line` closes a child with Deviations, Surprises and Open items under a done parent and finds each fact directly under the child's line in the Markdown and after the child's chip in the parent's HTML entry; in this repository, `debrief atelier-r6bt` now shows atelier-w5hx's Surprises and Open items in both forms.

### Deviations
A child's reported Deviations now also count in the parent entry's "Changed course" and in the quiet `deviations=` count, and make the parent's HTML entry notable (not folded). The spec in docs/product/mission-reviews.md says so.

### Decisions
See the two decision notes: a child's Deviations change its parent's course; a child's Open items show only when review filed no follow-ups for its entry.

### Surprises
None.

### Open items
The receipt's separate decision-log question (atelier-obcr failure 2) and the unrendered `### Decisions` report part (failure 3) are untouched, as assigned. Independent review is outstanding; only the targeted tests ran locally, the full suite runs in CI on the pull request.
