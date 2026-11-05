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
  - kind: "review"
    id: "atelier-krdy"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T23:28:37.172144040+00:00"
status: "done"
title: "Order the debrief timeline by one clock"
updated_at: "2026-11-04T23:28:37.172144040+00:00"
---

## Goal

The debrief timeline mixes clocks (review of atelier-8bil, finding atelier-krdy): delivered items sit at their record close time, set on the work branch, while work added to the scope sits at the first-parent commit that linked it, usually the merge. On atelier-r6bt, atelier-stsa shows as added at 15:04 after it was delivered as atelier-ho7o's child at 14:44. Order every event by one clock, or say which clock an event uses.

## Done when

A fixture where a child is closed on its parent's branch and merged later shows the child's addition no later than its delivery, shown by a CLI test.

## Context

crates/atelier-app/src/mission_review.rs `events`; crates/atelier-app/src/record_history.rs `mission_timeline` (first-parent walk). The nexd decision to show close times rather than start times explains why record times are used for delivery.

## Notes

- 2026-11-04 [decision] Date work added to the scope by the record clock delivery uses: the linking record's updated_at in the first commit that held the link among the commits its first-parent version brought in, no earlier than the mission's start; keep the start and close at their commits
  Reason: Delivery is a record time (nexd), so the addition must be one too; a merge brings in the branch's commits, where the link was written; the start and close are made on the base, where the commit and record clocks agree, and clamping to the start keeps 'added after the start' true on the page
- 2026-11-04 [decision] At one time, list an addition before deliveries
  Reason: A child linked and closed in one commit has equal record times, and an item joins the scope before it can be delivered

## Close Reason

The timeline now uses the record clock for work added to the scope, the same clock delivery uses. `mission_timeline` still decides "added after the start" by first-parent position. It then dates each addition from the first commit that held the link, among the commits that the joining first-parent commit brought in (for a merge, the merged branch's commits; for a work branch's entry commit, the branch's commits since the fork). The date is the holding record's `updated_at` in that commit, never earlier than the mission's start. When the times are equal, the addition is listed before deliveries. The start and close still use their commits' times. Two CLI fixtures cover this: `a_child_closed_on_its_parents_branch_is_added_before_it_is_delivered` (the branch merges into the base) and `a_child_closed_on_a_branch_before_it_merges_the_base_is_added_first` (the branch merges the base in first, debriefed from the branch). Each failed before its fix, with the child added after its delivery, and both pass now. docs/product/mission-reviews.md says which clock each event uses.

### Deviations
None.

### Decisions
Two decision notes on this item: which clock dates an addition (with the clamp to the start), and that at equal times an addition comes before deliveries.

### Surprises
A link written on a branch before the mission started, and merged after the start, would have been dated before "started the mission" while still labelled added after the start. The clamp to the start time covers this. Review finding atelier-a5vf showed that atelier-jnec's entry commit, which stands for its change from the fork, was searched only from its first parent. Versions now carry the commit their change counts from.

### Open items
None.
