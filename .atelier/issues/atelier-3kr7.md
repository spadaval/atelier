---
created_at: "2026-11-10T00:56:05.678725791+00:00"
id: "atelier-3kr7"
issue_type: "task"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-3kr7
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
status: "in_progress"
title: "Classify interventions, refusals and fix-now findings on seeded samples, with a second rater"
updated_at: "2026-11-10T14:15:30.767281960+00:00"
---

## Goal

Produce the three scorecard measures that need judgment: Sumeet's interventions per mission by kind, gate precision, and review yield. Write the rubric down so the same judgment can be made again after the cutover.

## Done when

- [x] `docs/architecture/quality/scorecard-baseline/rubric.md` defines every category of the three measures, with two examples each.
- [x] `docs/architecture/quality/scorecard-baseline/classified.md` gives interventions by kind for every mission closed in the window in the five repositories; gate precision on a seeded sample of 100 refusals; and review yield on a seeded sample of 100 fix-now findings, 20 from each repository. The seeds are stated.
- [x] For each measure, `docs/architecture/quality/scorecard-baseline/samples/<measure>.json` holds the sample without labels, `<measure>.labels.json` the first rater's labels, and `<measure>.second.json` a second rater's labels for the first 30 entries.
- [x] The second rater was a child agent of a different lab from the implementer, started by the implementer and given only `rubric.md` and the unlabelled sample file. The Report names its model and thread.
- [x] `classified.md` reports the two raters' agreement for each measure as a percentage, with the entries they disagreed on listed.
- [x] Sumeet's messages are counted and classified, and quoted only where they are remarks about process. The committed sample files hold identifiers and process facts, not private message text.

## Scope

Sampling and classification. Reads the baseline snapshot; never the live stores.

## Context

Categories for review yield are the assessment's seven: a bug a user would hit; a bug in development, CI or experiment tooling; a missing or vacuous test; a wrong statement in docs or output; a wording or style nit; a tracker-record or process nit; overbuilt code. A refusal either stopped a real mistake or was friction. An intervention is a status question, unsticking a stall, or correcting direction. The snapshot is at `~/.local/share/atelier-baseline/2026-11/`. The assessment's own samples are in `/var/tmp/atelier-assessment/records/`, `/other-repos/` and `/transcripts/`; draw new seeded samples, and use those only to check the rubric.

## Report

All six Done when lines hold. `rubric.md` defines the five intervention labels (three kinds of intervention plus approval and other), the three refusal labels and the seven finding categories, each with two examples: atelier review record IDs outside the sample for findings, invented examples for the rest. `classified.md` classifies all 142 typed messages in the 16 started missions of the 21 closed in the window, gate precision as 11 of 100 refusals (430 in the population), and review yield as 46 of 100 fix-now findings, 20 per repository, seed 20261110. The second rater was Grok 4.6 (`grok`, reasoningEffort high), thread `thread:delegated-task:command%3Amcp%3Acaf86c2c-d62b-4bc9-add2-22f98de38fe3%3Adelegate-task%3A8f60c506-46e3-4971-886a-afa3c0fa0f61`, given only the rubric and the unlabelled sample paths after the first labels were committed at a3753273; agreement is 73%, 83% and 97%, every disagreement listed. Proof, as evidence records: the samples redraw byte-identical (atelier-oy57); every sample-file string is an identifier, timestamp, path or label and no six-word run of private text appears in a committed file, a check that caught a planted quote (atelier-k85z); the agreement recomputes from the label files (atelier-yd9n); `freeze.py verify` passes and no journal file sits beside the database (atelier-ed9h).

### Decisions

- A mission's start is the first commit at which its record left draft; many missions went from todo to done without in_progress.
- A mission's threads include the threads above the ID-tied ones in T3's lineage. Without them only 18 typed messages were found; the owner talks to orchestrators from their parent threads.
- Approval and other are labels that are not interventions, so the three intervention kinds stay as the specification names them while every typed message is still classified.
- A refusal is atelier's `<command>: refused` or `Error: ` line; argument-parser errors belong to the confusion measure. 130 refusals are drawn so 100 remain after exclusions.
- Product defects are user bugs plus tooling bugs, the definition behind the assessment's 39%. Findings on plans are process findings by rule.

### Surprises

- 44% of the owner's typed messages inside mission windows were about other work, mostly planning the next mission in the thread above an orchestrator.
- The interventions rubric's first question ("is this about the mission?") drew most of the disagreement: 6 of 8 were the first rater's `other` against an intervention.

### Open items

- The privacy check lives at `~/.cache/atelier-3kr7/checks/privacy_check.py`, outside the repository, since the item allows one script.
- The rubric's tests for "about this mission" and for "a state the agent already knew" need sharpening before the after-cutover run; classified.md says so. The labels were left as they were.
