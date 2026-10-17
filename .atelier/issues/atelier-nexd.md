---
created_at: "2026-10-17T21:34:10.181899518+00:00"
id: "atelier-nexd"
issue_type: "feature"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-nexd
priority: "P1"
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
closed_at: "2026-10-17T22:03:41.515652552+00:00"
status: "done"
title: "Restructure the debrief as a review page: needs-you, why, criteria, work timeline"
updated_at: "2026-10-17T22:03:41.515652552+00:00"
---

## Goal

The debrief page reads as a dump of everything that happened (human review of atelier-r6bt's debrief, 2026-10-17). Restructure it so a human sees what needs them, why the mission existed, whether it met its criteria, and what each piece of work achieved, in the order it happened, with its changes, decisions, review and follow-ups beside it.

## Done when

- The HTML page has a persistent sidebar (progress, navigation), then: Needs your review (open decision records, needs-human items, failing and unproven criteria); Why (the strategy's Why section); Success criteria with the work that delivered each; Work as a timeline of done items in closing order with times in a left rail, each with its outcome, children, what changed (report deviations and derived deviations), surprises, decisions (one-way first), review summary and the follow-ups it left; Open work (planned, from review with its source, dropped); and the decision log.
- Issue references show titles with a popover giving the full title, a summary, type, status and ID; no bare IDs in the HTML text. No explanatory text; minimal entries, no nested cards.
- The Markdown form follows the same structure and keeps IDs. Fixture tests check both forms and that their facts match.
- `atelier debrief atelier-r6bt --html` produces the page the human approved as a mockup.

## Context

Approved mockup: .atelier/runtime/reviews/debrief-mockup.html (not committed). Mission atelier-r6bt, S4 and S7.

## Notes

- 2026-10-17 [decision] Attach each derived deviation, decision and follow-up to the item it belongs to, and drop the separate Deviations and Follow-ups lists
  Reason: The human found the separate lists tacked on; each fact is about one piece of work and reads best beside it
- 2026-10-17 [decision] Credit a criterion to an item when its Goal's first sentence names it outside parentheses, when the criterion's text names the item, or when it holds the deciding receipt
  Reason: Goals mention other criteria in passing inside parentheses; the leading clause names what the item delivers
  Rejected: any mention in Goal or Done when
- 2026-10-17 [decision] Show each work item's close time, not a start time
  Reason: Items start on branches, and the first-parent history only sees them when they merge, so a start time would be a guess
- 2026-10-17 [decision] Name records by title in the HTML, with the ID in a popover, and turn IDs mentioned in record text into the same popovers
  Reason: The human found bare IDs unhelpful; the Markdown keeps IDs for agents

## Close Reason

The debrief now reads as a review: what needs the human, why the mission existed, its criteria with the work that delivered them, the work as a timeline with each item's changes, decisions, review and follow-ups beside it, then open work, decisions and mission reviews, as Markdown with IDs and as an HTML page that names records by title with popovers.

### Deviations
Built from the human's review of the first debrief through two mockups rather than from the original S4 wording. Review atelier-exkf found children of open parents dropped out; they now stand as their own entries.

### Decisions
See this item's decision notes: facts attached to the item they concern, criterion credit by Goal's leading clause, close times only, and titles with popovers instead of IDs.

### Surprises
Record text mentions many records by ID, so turning mentions into popovers mattered as much as the page's own labels. The shared sccache server on this host had a deleted temp directory, so builds ran without it.

### Open items
- atelier-lfs7 and atelier-9y66: the human's S7 review and choice of S7 candidate.
