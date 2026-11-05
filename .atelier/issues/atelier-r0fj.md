---
created_at: "2026-11-04T23:12:48.493454620+00:00"
id: "atelier-r0fj"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-r0fj
priority: "P3"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-0a52"
  - kind: "issue"
    id: "atelier-b3qt"
  - kind: "issue"
    id: "atelier-jnec"
  - kind: "issue"
    id: "atelier-mg7w"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T02:01:07.967834848+00:00"
status: "done"
title: "Make the debrief and brief report history and next steps accurately"
updated_at: "2026-11-05T02:01:07.967834848+00:00"
---

## Goal

Fix the mission-review follow-ups from atelier-r6bt's reviews and validation, which bear on S4 (debrief) and S5 (brief): a debrief from a work branch that merged the base (atelier-jnec), one clock for the timeline (atelier-b3qt), the brief's approval footer (atelier-mg7w) and which outcome the brief shows (atelier-0a52). One branch and one review; jnec before b3qt because both depend on the first-parent walk in record_history.rs.

## Done when

Every child is terminal with its own Done when met and proven, and one independent review of the branch is approved here.

## Context

Runs in parallel with atelier-2bjz, whose atelier-w5hx also edits crates/atelier-cli/src/commands/mission_review.rs, crates/atelier-app/src/mission_review.rs and docs/product/mission-reviews.md (one-way decision ordering and output). Whichever branch lands second runs `atelier merge --sync`, resolves those files, and needs a fresh CI run and review of the resolution.

## Close Reason

The debrief and brief now report history and next steps accurately: a debrief from a work branch that merged the base reports the base's start and deviations (atelier-jnec), the timeline dates additions and deliveries by one clock, including on such branches (atelier-b3qt), the brief offers plan approval only until one is recorded (atelier-mg7w), and the brief shows the strategy's Outcome (atelier-0a52).

### Deviations
Two review rounds (atelier-uev7, atelier-a5vf) reshaped jnec's walk: the branch's line is entered at its first commit holding the fork, and each version carries the commit its change counts from. mg7w also fixed the HTML footer.

### Decisions
On the children (`atelier work decisions atelier-r6bt`).

### Surprises
Walking the branch's own line from the fork reordered record states and moved link dates; both were caught only by review fixtures with branch record edits before the base merge.

### Open items
By design, a branch's own record changes before its last base merge count as one change; the base is read from the local base branch and falls back to the old walk if it is missing.
