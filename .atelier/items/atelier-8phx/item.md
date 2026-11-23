---
schema: atelier.item
schema_version: 3
id: atelier-8phx
title: Tell every role to file what it finds outside its work, and show with an eval case that the line changes what agents do
status: building
mission: atelier-dh7a
labels: []
priority: medium
blocked_by:
- atelier-u6oy
implementers:
- thread:delegated-task:command%3Amcp%3A134a3ba0-c5f0-4828-a968-a4cfb219e451%3Adelegate-task%3Ac05f2162-4393-45a6-9e3f-c9bfc89ea590
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T15:25:03.272209421Z
---

## Goal

No role file tells an agent to record a problem it finds outside the work
in hand. `roles/implementer.md` is silent, a reviewer's instruction covers
findings on the change it reviews, and only the gardener is told to file
something broken. With atelier-u6oy merged, filing costs one command on the
branch the agent already has. This item adds the instruction, and an eval
case that shows whether it changes what agents do. The human said on
2026-11-18 that evals decide what the skill holds, so the line stays only
on a gain.

## Done when

- [ ] The instruction is written and tried: one commit adds to the skill,
  where every role reads it, that a problem found outside the work in hand
  is filed with `atelier new` on the branch you are on, with what was seen
  and where, and the work carries on, and that a reviewer's finding on the
  change under review stays a finding. Whether the merged skill holds it
  is the last line's to say. (The lead changed this line on 2026-11-23:
  as first written it required the skill to say so, which cannot hold
  beside the last line once the runs showed no gain, 5 of 5 filing in
  each arm.)
- [x] `roles/lead.md` and `procedures/garden.md` no longer send a lead or a
  gardener to a backlog branch to file an item when they hold a branch
  already.
- [x] A new case in `evals/agent-factory` gives an implementer an item
  whose work runs into a planted problem outside its Scope. Code reads
  whether the item merged and whether a new item's record reached main;
  the judge reads whether that record describes the planted problem. The
  case sets up and grades under `test.sh` with no model.
- [x] The case is run with the line and without it, at least five runs per
  arm in the `house` setup. The README gives for each arm the runs that
  filed the problem, the runs that merged their item, and time and tokens.
- [x] If more runs file with the line, it stays. If not, the line is
  removed, the Report says so, and the case stays only if the README says
  what it guards.

## Scope

`skills/agent-factory/SKILL.md`, `roles/` and `procedures/garden.md`;
one case and its fixture and rubric in `evals/agent-factory`, with the
README's row for it. Not `procedures/backlog.md`'s rules for a sweep. No
new grader, harness feature or setup: a case that needs one is cut, as
atelier-kw8u's Authority says. At most two eval agents alive at once.

## Report

The line made no difference to filing, so it is gone. In `house`, Opus
implementers filed the planted problem and merged their item in 5 of 5 runs
without the line and in 5 of 5 with it. Without it, each one chose to file
by itself, after `atelier new --help` showed that `new` works on its branch
since atelier-u6oy. As this item and the mission's Authority say, the line
is removed: 161da1c50 reverts a8ae546b6. The first Done when line cannot
hold now, so it is not ticked, and the lead has said it will amend that
line.

The runs did show one other effect. Without the line, each Codex reviewer
also wrote the bug up as a deferred finding, so it was recorded twice. With
the line, which says a finding is for the change under review, none did.
That is less to settle later, but filing was the measure, and filing did
not move.

What stays:

- `outside-problem`, the case. The item moves a ledger reader and must leave
  both commands' output as it is, so the implementer runs `summary`, which
  adds a refund where README.md says it subtracts one. Every run passed,
  and the README says what it guards: that an implementer files on its
  branch and still merges, which a change to `new` or to the skill would
  show.
- `roles/lead.md` and `procedures/garden.md` file on the branch the lead
  or gardener holds (162946875). A gardener's bugs get their labels after
  its run merges, because `set` is refused on an item's branch
  (atelier-fuzq), so until then they are open with no `needs-human`.

### Results

[results/2026-11-23-told-to-file.md](../../../evals/agent-factory/results/2026-11-23-told-to-file.md),
five runs an arm, one at a time, none graded null or made again:

| Arm | Filed | Merged | Median s | Median tokens in / out |
| --- | --- | --- | --- | --- |
| without the line (162946875) | 5 | 5 | 121 | 553k / 8.4k |
| with the line (a8ae546b6) | 5 | 5 | 108 | 434k / 7.0k |

The implementer filed in all ten runs, and no reviewer did, so no record
went to a judge of its author's lab. I read all ten records and final
replies by hand: each describes the problem, where it is and how it shows,
as the judge found. No transcript read the case's files. The runs' work
directories are kept under `/var/tmp/af-evals/` and their records under
`~/.cache/atelier-8phx-runs/`; no run built anything.

### Validation, with no model

- `evals/agent-factory/test.sh target/debug/atelier` exits 0 on a binary
  built from this branch: 565 tests, 13 of them this case's. The four
  skipped are the sized controls' checks of a lean reference they lack. The
  case's tests cover filing by the implementer, by a reviewer after
  approval and on a backlog branch, and each way of not filing.
- `run_one` with a stand-in agent and judge, once per arm in `house`:
  each stages its skill for both Claude Code and Codex, the section is
  present only at a8ae546b6, and both grade.
- By hand on the built binary: `new --in <mission>` after approval on an
  item's branch merges with no new round, and `set` there is refused.
- `atelier check` and `git diff --check` pass. No Rust changed, so CI runs
  only the evals job.
- `test.sh` left 54 directories in `TMPDIR`, which atelier-5erp already
  holds.
