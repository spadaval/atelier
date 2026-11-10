---
created_at: "2026-11-10T00:56:05.944032168+00:00"
id: "atelier-c71h"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-c71h
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
    id: "atelier-jah8"
    type: "advances"
  - kind: "issue"
    id: "atelier-scx2"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z5bp"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Count defects after merge, and where each defect was caught, in the five repositories"
updated_at: "2026-11-10T15:49:25.708452013+00:00"
---

## Goal

Apply the method the spike chose to all five repositories, and say at which stage defects were caught, so the redesign can be checked for making correctness worse.

## Done when

- [x] `docs/architecture/quality/scorecard-baseline/defects.md` gives, for each repository over the window: items merged, and defects found after merge by the chosen method, with the sample or the full list behind the number.
- [x] It gives where defects were caught: in review, as the count of fix-now findings times the review-yield rate `classified.md` gives for that repository; in CI, where the repository has it, as failed runs followed by a fix on the same branch; and after merge. It says plainly that defects an author's own tests caught are not observable before the redesign.
- [x] The rule and the seed are those in `docs/architecture/quality/scorecard-baseline/defect-method.md`, unchanged, or the file says what changed and why.

## Scope

The five repositories on this host. No change to any of them.

## Context

Strategy: `docs/strategies/redesign-1-baseline.md`. Private repositories: report process facts only.

## Unknowns

- Which method applies is decided by the spike, so this item's size is not known until then.

## Notes

- 2026-11-10 [note] Handoff: approved by review-sol-c71h-r1 (atelier-u7bn) at 72231734 in round 1, no findings. Not merged: the orchestrator merges. See ## Report for outcome, deviations and open items.

## Report

All three Done when lines hold. `docs/architecture/quality/scorecard-baseline/defects.md` traces every bug fix in the window, 130 in all (77 atelier, 52 astromech of which 30 reuse defect-method.md's rows, 1 demiurge; life-on-rails and cosplay-studio fixed none), and gives items merged per repository: 33 defects after merge from 780 merged items (atelier 15 of 277, astromech 18 of 361, the others 0). It gives review (fix-now findings times classified.md's rate, both factors shown), CI (failed runs fixed by a later commit on the same branch, from one stated `gh run list` query) and after merge, and says plainly that author-test catches are not observable. It names what changed from the method and why. Proof: the document's candidate loop, rerun, gives 77/52/1/0/0; the tables recount to the summary's totals (130 rows: 33 after merge, 25 before merge, 20 before adoption, 12 no item, 40 not traceable); the documented CI snippet reproduces the CI counts from the saved run lists. Three helper-traced atelier rows (3xq4, cej5, vo0z) were repeated in full and matched.

### Deviations

- Step 6 changed: the outcome compares the bug's filing with when the cause reached the base, read from history, because items were often closed on their branch before review and bugs filed from that review read as after merge by `closed_at`. 18 rows move to before merge, including defect-method.md's row 25 (astromech-5t34). The totals under step 6 as written are in the document.
- Read as written, with the alternative reading's effect stated: step 1 for fixes inside a parent's commit (15 atelier rows not traceable), and step 5 for atelier's 2026-10-01 rebuild commits (12 rows no item; credited to atelier-1ioo they would be after merge).
- The document is about 525 lines, above the brief's 250 to 450, because atelier's table alone is 77 rows with long blame lists.

### Surprises

- 35 of atelier's 77 bug fixes are not traceable, against 4 of astromech's 52: atelier's bug items are often fixed inside a parent's commit or hold several defects each. Its after-merge count is a floor.
- Every astromech defect after merge comes from items merged 2026-10-16 to 10-22; later merges have none found yet.

### Open items

- The CI counts depend on GitHub run history read at 2026-11-10T15:21:29Z; a rerun may see fewer runs.
- Rows resting on step 4's conventions are named in the notes; astromech 51 would be before merge by the first convention.
