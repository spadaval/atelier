---
created_at: "2026-10-18T06:33:47.022557698+00:00"
id: "atelier-8bil"
issue_type: "feature"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-8bil
priority: "P2"
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
status: "in_progress"
title: "Rework the debrief as an after-action report"
updated_at: "2026-10-18T06:53:35.680645285+00:00"
---

## Goal

Rework `atelier debrief` into an after-action report, as the human directed on 2026-10-18 after reviewing the nexd page: the problem, what we set out to do, what we did, then where we are (results, how the mission was validated, plan against actual, what needs the human, what is left, what was found along the way). Records appear as ID chips with title and status that open a full card, decisions appear only where they were made in the timeline, and the timeline also carries mission validation, mission reviews, scope added, work filed and work dropped at their times.

## Done when

Both forms of `atelier debrief` follow that order; the separate decision log, Open Work and Mission Reviews sections are gone; the HTML names every issue mention as a chip with its ID, title and status and a card with its summary, outcome and parent; a Validation section shows each item-review total, each mission validation run (who, when, at which commit, each receipt's criteria and summary, work delivered after it) and the criteria still awaiting proof; open work splits into what is left of the plan and what was found along the way. Shown by CLI tests, and by the regenerated debrief of atelier-r6bt.

## Scope

crates/atelier-app/src/mission_review.rs (view model), crates/atelier-cli/src/commands/mission_review.rs (debrief renderers only; the brief is unchanged), crates/atelier-cli/src/review_page.rs (review styles), crates/atelier-cli/tests/cli_integration/debrief.rs, docs/product/mission-reviews.md. No record schema change: discovered work is told apart heuristically until atelier-tq1j adds `discovered_from`, and there is no retrospective until a mission close report exists.

## Context

Postmortem shapes compared on 2026-10-18: the After Action Review's four questions, Google SRE's postmortem (header verdict, lessons, action items, timeline), Etsy's debriefing guide (decisions told where they were made), Shape Up's imagined versus discovered tasks, Atlassian Smart Links (inline key, title and status with a hover card). atelier-lyow's label tidy-ups become moot once mentions are ID chips.

## Unknowns

S4's wording names a decision log with one-way decisions first and derived deviations as lists; the new page shows decisions per item with a one-way index and deviations as plan against actual. Whether that still reads as S4 is the human's call.

## Notes

- 2026-10-18 [decision] Count open work added after the start as found only while it is parked in backlog or draft; added work someone has taken on is left to do
  Reason: Until atelier-tq1j records where work was found, parking is the only record-level sign that added work was filed for later rather than taken into scope; this run's own rework item would otherwise read as found
  Rejected: treat all work added after the start as found
  Rejected: treat all of it as scope
- 2026-10-18 [decision] Group receipts that judge mission criteria into validation runs by producer and pinned commit
  Reason: A validator records one receipt per criterion against one commit; grouping shows one validation with who ran it, where, and what changed since, instead of a receipt list
  Rejected: one entry per receipt
  Rejected: group by a time window
- 2026-10-18 [decision] Show decisions only in the timeline entry where they were made, with a sidebar index of the one-way ones; drop the debrief's decision log sections
  Reason: The human found the separate log confusing beside the timeline; atelier work decisions still lists the whole log one-way first
  Rejected: keep a decision log section after the timeline
- 2026-10-18 [decision] Show every issue the page names as a chip with its ID, title and status that opens a card, and review events and receipts as chips with their ID and kind
  Reason: The human directed on 2026-10-18 that IDs are stable terms people use, reversing atelier-nexd's choice to hide them behind titles
