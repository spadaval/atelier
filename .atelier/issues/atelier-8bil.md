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
updated_at: "2026-10-18T06:34:08.569987575+00:00"
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
