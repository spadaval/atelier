---
schema: atelier.item
schema_version: 3
id: atelier-qut2
title: Rewrite the skill for plans as outcomes and lighter later rounds, from what this mission's own plan review showed
status: building
mission: atelier-oqsz
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A97bcfb9a-33e0-42f5-882a-004f5d9c6644%3Adelegate-task%3Aoqsz-impl-qut2
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T07:19:32.527359035Z
---

## Goal

Plan reviews spiral. `atelier-ww7w`'s plan of 2,400 words drew 45 new
findings in five rounds (20, 12, 7, 4, 2), most about how a mechanism
would behave. The skill gives no reason for them not to:
`roles/planner.md` says nothing about what level a plan is written at,
`roles/reviewer.md` is written for a code change and tells a plan's
reviewer to "report every other finding you can support", and a new
mission's plan is six empty headings.

Code reviews have a smaller fault. In `hub-v2-bg14` round one found a
blocking fault in eight of nine merged items; round two approved all
eight and added five findings, none blocking, at the cost of a full
review each.

This mission's plan was written as outcomes and reviewed under the old
text, and `notes.md` holds what that review showed. Change the skill by
what it showed. A review that converged in one or two rounds on outcome
findings points at the level a plan is written at. One that still drew
mechanism findings or ran long points at the reviewer's instructions and
the missing stop. It is one review, and this plan differs from
`atelier-ww7w`'s in subject, size and reviewers, so it is evidence and
not proof.

On 2026-11-27 Sumeet steered this item, in thread
`cf983ef7-3317-404a-9c82-373f732b9e07`. Two faults the plan did not name
are added. A planner writes each later item's checklist before anything
is built, and its lead then holds the work to it. And an item's reviewer
checks the change against the item's own Done when and never asks whether
that line was worth building: of 50 reviews of five changes this
repository built and later removed, none questioned one
(`atelier-8az0`). He also set the second review of `atelier-ww7w`'s plan
aside: this plan's own review is signal enough, and Atelier's work on
itself stops when this mission closes.

## Done when

- [x] The Report says what the plan review's counts in `notes.md` suggest
  and what they leave open, and each change to the skill names what it
  follows from: a count there, or the human's direction of 2026-11-27.
- [x] The skill says what level a plan is written at, and tells a planner
  not to write the checklist of how a later item is built before the work
  ahead of it has landed.
- [x] `roles/lead.md` says a lead may split, shrink or rewrite an item's
  Done when as earlier items land, inside the mission's Outcome and
  Authority, with its reason on the record.
- [x] A plan's reviewer is told what a plan review is for, where a finding
  about a mechanism goes, and when plan review stops.
- [x] An item's reviewer is told that a Done when line asking for more
  than the mission's Outcome needs is a `Shrink:` finding against the
  item, not a box to tick.
- [x] A later code-review round is lighter than the first, or the Report
  says why the evidence does not support that.
- [x] `roles/lead.md` has the lead give the human the pace numbers when it
  reports on a mission.
- [x] `roles/reviewer.md` and `SKILL.md` together hold no more words when
  this item merges than when it started. The Report gives both counts.
- [x] `evals/agent-factory/test.sh` passes. No eval matrix is run, and no
  plan is reviewed again.

## Scope

`skills/agent-factory/`, the documents that repeat it, and the brief
`atelier review <mission>` prints, in `crates/` and its tests. It starts
once `atelier-oiwy` is merged or dropped, and builds on what that item
left in `roles/reviewer.md`. The skill costs a reviewer real defects
already (`atelier-7h71`: 4 of 26 found with it, 15 of 26 without), so
what this item adds for a reviewer displaces something or goes in the
brief. `atelier-b0vb`, the per-part question on plans, waits on the
human: read it and leave it. No new role.

## Report

Plans now say what level they are written at, plan reviewers are told to
defer findings about mechanisms, an item's first reviewer sees its
mission's Outcome and is told to raise `Shrink:` against a Done when line
that asks for more, and a later code-review round no longer reviews the
whole change again. `roles/reviewer.md` and `SKILL.md` did not grow.

### What the counts show

**Plan review** (`missions/atelier-oqsz/notes.md`). This mission's plan
was about 800 words written as outcomes. It drew 4 new findings in two
rounds, all about outcomes and none about mechanisms, over 17 minutes.
`atelier-ww7w`'s 2,400-word plan drew 45 findings in five rounds over 80
minutes, most about mechanisms. Both findings that blocked were in this
item, the one item that named a procedure.

- What it suggests: the level a plan is written at decides most of what
  its review raises. Under the old reviewer text, an outcome plan drew no
  mechanism findings.
- What it leaves open: it is one review. The plan was a third the size,
  on another subject, with Sol where Opus stood, and the planner marked
  its own plan's findings. Nothing tests the new plan-reviewer text,
  because the human set aside the second review of `atelier-ww7w`'s plan.
  The pace line on the next real missions is the only test left.

**Code review** (`hub-v2-bg14`, counted by hand from its review files at
`49f96611` in `industrial-experience-hub`). A round's time is from the
review file's `created_at` to the commit that adds it.

| Round | Items | Median minutes | Total minutes | New findings | New fix-now |
| --- | --- | --- | --- | --- | --- |
| 1 | 11 | 15 | 180 | 39 | 27 |
| 2 | 10 | 15 | 175 | 6 | 0 |
| 3 | 1 | 3 | 3 | 0 | 0 |

- What it suggests: round 2 cost as much as round 1 and found nothing
  that blocked. A later round can be lighter.
- What it leaves open, and why the whole diff stays: the survey of
  2026-11-19 in `docs/design.md` found 7 of 18 findings from round 3 on
  still important. A planted defect in unchanged code blocked in every run
  of that eval. So the later brief still gives the whole change's diff and
  still lets a serious defect in unchanged code block. No eval has run the
  new text.
- These counts differ from the mission's Why (eight of nine items, five
  findings), because the Hub's records have moved on since. The Why's
  "over an hour each" does not show in the file times either: round 2 took
  3 to 32 minutes from opening the review to committing it. Any wait
  before the review opened, such as for CI or a reviewer to start, is not
  in the records.

### Each change and what it follows from

- `roles/planner.md`, "The Level of a Plan": write a plan as outcomes.
  This follows from the plan review's 4 outcome findings and 0 mechanism
  findings, with both blockers in the one item that named a procedure. A
  later item gets its Goal and what it owes the Outcome, and no checklist
  of how it is built until the work ahead of it has landed. That follows
  from the human's direction of 2026-11-27. The rounds text moved under
  its own heading, "Plan Review Rounds", unchanged.
- `roles/lead.md`, "Shaping Items as Work Lands": a lead may split, shrink
  or rewrite an item's Done when inside the Outcome and Authority, with
  the reason in `notes.md`. It also reads the deferred plan findings that
  name an item before that item starts, so a deferred mechanism finding
  reaches someone. Both follow from the human's direction.
- `roles/lead.md`: the lead gives the human the pace numbers when it
  reports on a mission. This follows from the human's direction and the
  Why's "your last five reviews took over an hour each and caught
  nothing". The numbers are the line `atelier-f0sn` adds to `status`.
- The plan brief (`PLAN_REVIEW_IS_FOR` in
  `crates/atelier-cli/src/output.rs`, every round): what a plan review is
  for, that a mechanism or unstated-case finding is `defer` and names its
  item unless the plan cannot reach its Outcome without it, and that
  review stops once both reviewers of a round approve. This follows from
  the plan review's counts and `atelier-ww7w`'s 45 findings. It goes in
  the brief and not in `roles/reviewer.md`, as the Scope says, because
  `atelier-7h71` found the skill already costs reviewers defects.
- The item brief, round 1 only: it prints the mission's Outcome and says
  a Done when line asking for more than it needs is a `Shrink:` finding
  against the item, not a box to tick. This follows from the human's
  direction (`atelier-8az0`: none of 50 reviews questioned a Done when
  line). Before this change the reviewer never saw the Outcome, so the
  rule needed the text beside it. `crates/atelier-cli/src/review.rs` reads
  the Outcome from the checkout's mission record.
- The later-round item brief (`later_round`): "this round does not review
  it again". It reads the commits since the last round, runs what proves
  them, and leaves the size and the untouched Done when lines and Report
  claims as the earlier rounds judged them. `roles/reviewer.md` now sizes
  the change "In round 1". This follows from the `hub-v2-bg14` table.
- `docs/design.md`: the brief paragraphs and a decision row, with the
  counts above.

**Word counts.** At the start `SKILL.md` had 521 words and
`roles/reviewer.md` had 515, 1,036 together. Now they have 521 and 515,
1,036 together. Adding "In round 1," was paid for by cutting "as well" and
"basic".

**Not done.** `start --mission` still writes six empty headings, because
a different template would change a record's form, which the mission's
hosts criterion rules out. `roles/planner.md` says what goes in them
instead. `atelier-b0vb` is untouched. No eval matrix was run and no plan
was reviewed again.

**Checks.** `cargo nextest run -p atelier-cli`: 280 passed.
`cargo clippy -p atelier-cli --all-targets -- -D warnings`,
`cargo fmt -- --check` and `git diff --check` are clean.
`evals/agent-factory/test.sh` passes. New assertions in
`tests/planning.rs` cover the plan brief's text and the Outcome and
`Shrink:` lines in an item's round-1 brief. `tests/scenarios.rs` covers
the new later-round text.
