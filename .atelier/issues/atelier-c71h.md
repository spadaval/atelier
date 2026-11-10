---
created_at: "2026-11-10T00:56:05.944032168+00:00"
id: "atelier-c71h"
issue_type: "task"
labels:
- "redesign"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
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
status: "todo"
title: "Count defects after merge, and where each defect was caught, in the five repositories"
updated_at: "2026-11-10T00:56:05.944032168+00:00"
---

## Goal

Apply the method the spike chose to all five repositories, and say at which stage defects were caught, so the redesign can be checked for making correctness worse.

## Done when

- [ ] `docs/architecture/quality/scorecard-baseline/defects.md` gives, for each repository over the window: items merged, and defects found after merge by the chosen method, with the sample or the full list behind the number.
- [ ] It gives where defects were caught: in review, as the count of fix-now findings times the review-yield rate for that repository; in CI, where the repository has it, as failed runs followed by a fix on the same branch; and after merge. It says plainly that defects an author's own tests caught are not observable before the redesign.
- [ ] The rule and the seed are those in `docs/architecture/quality/scorecard-baseline/defect-method.md`, unchanged, or the file says what changed and why.

## Scope

The five repositories on this host. No change to any of them.

## Context

Strategy: `docs/strategies/redesign-1-baseline.md`. Private repositories: report process facts only.

## Unknowns

- Which method applies is decided by the spike, so this item's size is not known until then.
- Review yield per repository comes from the classification item. Until it lands, use the assessment's rates and mark them provisional.
