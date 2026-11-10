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
  - kind: "issue"
    id: "atelier-scx2"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Classify interventions, refusals and fix-now findings on seeded samples, with a second rater"
updated_at: "2026-11-10T01:14:39.342323710+00:00"
---

## Goal

Produce the three scorecard measures that need judgment: Sumeet's interventions per mission by kind, gate precision, and review yield. Write the rubric down so the same judgment can be made again after the cutover.

## Done when

- [ ] `docs/architecture/quality/scorecard-baseline/rubric.md` defines every category of the three measures, with two examples each.
- [ ] `docs/architecture/quality/scorecard-baseline/classified.md` gives interventions by kind for every mission closed in the window in the five repositories; gate precision on a seeded sample of 100 refusals; and review yield on a seeded sample of 100 fix-now findings, 20 from each repository. The seeds are stated.
- [ ] For each measure, `docs/architecture/quality/scorecard-baseline/samples/<measure>.json` holds the sample without labels, `<measure>.labels.json` the first rater's labels, and `<measure>.second.json` a second rater's labels for the first 30 entries.
- [ ] The second rater was a child agent of a different lab from the implementer, started by the implementer and given only `rubric.md` and the unlabelled sample file. The Report names its model and thread.
- [ ] `classified.md` reports the two raters' agreement for each measure, as a percentage and as Cohen's kappa, computed by a script in `scripts/scorecard/`.
- [ ] Sumeet's messages are counted and classified, and quoted only where they are remarks about process. The committed sample files hold identifiers and process facts, not private message text.

## Scope

Sampling and classification. Reads the baseline snapshot; never the live stores.

## Context

Categories for review yield are the assessment's seven: a bug a user would hit; a bug in development, CI or experiment tooling; a missing or vacuous test; a wrong statement in docs or output; a wording or style nit; a tracker-record or process nit; overbuilt code. A refusal either stopped a real mistake or was friction. An intervention is a status question, unsticking a stall, or correcting direction. The snapshot is at `~/.local/share/atelier-baseline/2026-11/`. The assessment's own samples are in `/var/tmp/atelier-assessment/records/`, `/other-repos/` and `/transcripts/`; draw new seeded samples, and use those only to check the rubric.
