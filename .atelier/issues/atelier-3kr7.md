---
created_at: "2026-11-10T00:56:05.678725791+00:00"
id: "atelier-3kr7"
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
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Classify interventions, refusals and fix-now findings on seeded samples, with a second rater"
updated_at: "2026-11-10T00:56:05.678725791+00:00"
---

## Goal

Produce the three scorecard measures that need judgment: Sumeet's interventions per mission by kind, gate precision, and review yield. Write the rubric down so the same judgment can be made again after the cutover.

## Done when

- [ ] `docs/architecture/quality/scorecard-baseline/rubric.md` defines every category of the three measures, with two examples each.
- [ ] `docs/architecture/quality/scorecard-baseline/classified.md` gives interventions by kind for every mission closed in the window in the five repositories; gate precision on a seeded sample of 100 refusals; and review yield on a seeded sample of 100 fix-now findings, 20 from each repository. The seeds are stated, and each sample with its labels is stored as JSON beside the file.
- [ ] A second rater from a different lab classified 30 items of each sample without seeing the first labels. The file reports their agreement for each measure, as a percentage and as Cohen's kappa.
- [ ] Sumeet's messages are counted and classified, and quoted only where they are remarks about process.

## Scope

Sampling and classification. The thread reader from the counted-measures item may be reused once it exists, but this item does not wait for it.

## Context

Categories for review yield are the assessment's seven: a bug a user would hit; a bug in development, CI or experiment tooling; a missing or vacuous test; a wrong statement in docs or output; a wording or style nit; a tracker-record or process nit; overbuilt code. A refusal either stopped a real mistake or was friction. An intervention is a status question, unsticking a stall, or correcting direction. The assessment's own samples are in `/var/tmp/atelier-assessment/records/`, `/other-repos/` and `/transcripts/`; draw new seeded samples, and use those only to check the rubric.
