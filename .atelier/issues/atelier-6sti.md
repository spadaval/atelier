---
created_at: "2026-11-10T00:56:06.207448953+00:00"
id: "atelier-6sti"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-6sti
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3kr7"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-5fsj"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-c71h"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jah8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-10T17:27:04.367724405+00:00"
status: "done"
title: "Write the baseline report, and propose a target for each cost measure"
updated_at: "2026-11-10T17:27:04.367724405+00:00"
---

## Goal

Put the baseline in one place that later missions are judged against, and propose targets for Sumeet to accept or change when he reads this mission's result.

## Done when

- [x] `docs/architecture/quality/scorecard-baseline-2026-11.md` gives every measure in the product specification's Scorecard table, per repository, with its method in one line, its coverage and its limits, and links the files under `docs/architecture/quality/scorecard-baseline/`.
- [x] It proposes a target for process share, confusion and tokens per merged item, each with its reason, and states the constraint that defects after merge must not rise. The targets are marked as proposals for Sumeet.
- [x] The Before column of the Scorecard table in `docs/strategies/redesign-product.md` holds the report's numbers.
- [x] The report says which numbers differ from the assessment's and why.

## Scope

Assembly and targets. No new measurement.

## Context

Strategy: `docs/strategies/redesign-1-baseline.md`. The assessment's numbers, which the Scorecard table holds today, are in `/var/tmp/atelier-assessment/reports/`.

## Notes

- 2026-11-10 [note] Handoff: approved by GPT-6.1 Sol (atelier-4aeu) at 9ef39081 after two review rounds. Round 1's two fix-now findings (atelier-0gcc, model mix lowers tokens per item; atelier-zj1b, an equal window caps merge age but does not equalise it) were fixed in cd2dd902 and 865a9623. No follow-ups filed, nothing undisposed. Outcome and open items are in the Report.

## Report

All four Done when lines hold. `docs/architecture/quality/scorecard-baseline-2026-11.md` (about 270 lines) opens with what the numbers say and the targets Sumeet is asked to decide, then gives all twelve scorecard rows per repository, each with its method, coverage and limits, and links every file under `scorecard-baseline/`. It proposes a target for process share, confusion and tokens per merged item, marked as proposals, and states the defects constraint in checkable form (astromech 18 of 361 and atelier 12 of 277, defects.md's denominator, a 27-day window counted at its end, void if bug filing falls). The Before column of the Scorecard table in `redesign-product.md` holds the report's numbers, and a section lists each number that differs from the assessment's and why. Proof, recorded as evidence: every number in the report and the Before column was found in its source file with the line, or is a derivation listed with its factors (442 numbers, 0 unmatched); all 51 links in the four changed documents resolve, anchors included; the Scorecard table still has 14 table lines of 3 columns and its Group and Measure columns are unchanged; `git diff --check` is clean; `atelier check atelier-6sti` passes.

### Decisions

- Targets are halves, applied the same way to each repository with at least 20 merged items tied to a thread: process share at most half its baseline, confusion at most half, and tokens per merged item lowered only by the process tokens the first target removes, since review rounds are not a cost to cut. The baseline cannot split process commands by subcommand, so half is stated as a judgment.
- The defects constraint compares rates per 100 merged items on defects.md's denominator, counting only causes merged in the window (atelier 12, not 15), with astromech as the main comparison because only 4 of its 52 bug fixes are untraceable.
- The report states both merged-item denominators (counted.md's 243 and 339, defects.md's 277 and 361) and which rows use each.
- The note under the Scorecard table in `redesign-product.md` points to the report, since several cells cannot hold their limits.

### Open items

- The two counts of done items in the window do not reconcile exactly: defects.md's merged items plus its excluded types give atelier 316, demiurge 108 and life-on-rails 52, against counted.md's done counts (missions excluded) of 305, 107 and 51. The report does not mix them; the cause was not investigated, since that would be new measurement.
- The assessment's numbers that the report quotes when explaining differences come from `/var/tmp/atelier-assessment/reports/`, outside the repository.
