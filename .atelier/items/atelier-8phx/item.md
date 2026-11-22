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

- [x] The skill says once, where every role reads it or in each role file
  that needs it, that a problem found outside the work in hand is filed
  with `atelier new` on the branch you are on, with what was seen and
  where, and the work carries on. It says a reviewer's finding on the
  change under review stays a finding.
- [x] `roles/lead.md` and `procedures/garden.md` no longer send a lead or a
  gardener to a backlog branch to file an item when they hold a branch
  already.
- [x] A new case in `evals/agent-factory` gives an implementer an item
  whose work runs into a planted problem outside its Scope. Code reads
  whether the item merged and whether a new item's record reached main;
  the judge reads whether that record describes the planted problem. The
  case sets up and grades under `test.sh` with no model.
- [ ] The case is run with the line and without it, at least five runs per
  arm in the `house` setup. The README gives for each arm the runs that
  filed the problem, the runs that merged their item, and time and tokens.
- [ ] If more runs file with the line, it stays. If not, the line is
  removed, the Report says so, and the case stays only if the README says
  what it guards.

## Scope

`skills/agent-factory/SKILL.md`, `roles/` and `procedures/garden.md`;
one case and its fixture and rubric in `evals/agent-factory`, with the
README's row for it. Not `procedures/backlog.md`'s rules for a sweep. No
new grader, harness feature or setup: a case that needs one is cut, as
atelier-kw8u's Authority says. At most two eval agents alive at once.

## Report

Not ready for review: the instruction and the eval case are committed, and
the runs that decide whether the instruction stays have not been made. The
`house` setup reviews and judges with Codex `gpt-6.1-sol`, whose weekly
allowance is spent until 2026-11-23 00:04 New York time. No model was run
for this item so far.

What changed:

- `SKILL.md` has Problems Outside Your Work, which every role reads: file
  with `atelier new` on the branch you are on, say what you saw and where,
  and carry on. It keeps the two exceptions the binary's briefs make: a
  thread with no branch uses a backlog branch, and a plan's reviewer files
  only once its verdict is pushed. It is a commit of its own (a8ae546b6),
  so reverting that commit removes the line and nothing else.
- `roles/lead.md` files on the branch the lead is on. `procedures/garden.md`
  files bugs on the run's branch. Their labels and cause still wait for
  `backlog/garden-proposals`, because `set` is refused on an item's branch
  (atelier-fuzq). Until then such a bug is open with no `needs-human`.
- `outside-problem` is the new case. The item moves a ledger reader and must
  leave both commands' output as it is, so the implementer runs `summary`,
  which adds a refund where the README says it subtracts one. Scope leaves
  what the commands compute out. Nothing in the fixture mentions filing.

How the runs are read, fixed before any was made:

- A run filed the problem when the judge passes `problem-filed`: a record on
  main describes it, whichever thread filed it. A reviewer reads the line
  too, and its brief already allows filing, so `new_called_by` says who
  filed.
- The arms are the skill at 162946875 (without the line) and at a8ae546b6
  (with it). The lead's and gardener's wording is in both.
- The one judge of `problem-filed` is the implementer's, Codex. A record
  that the Codex reviewer filed is therefore judged by its own lab, so I
  read each such record by hand.

### Runs left

`bash ~/.cache/atelier-8phx-runs/runs.sh` on this host runs, one run at a
time, for each ref above:

    EVAL_ATELIER=/var/tmp/af-evals/bin/atelier-a8ae546b6 \
      python3 evals/agent-factory/harness/run.py outside-problem --setup house \
      --arms skill --ref <ref> --runs 5 --jobs 1 --keep --out <a directory for that arm>

The two arms need a directory each, since `table.py` counts a setup, case
and arm as one cell.

After the runs: write the results file and the README's row, then keep the
line if more runs file with it. If not, revert a8ae546b6, untick the first
Done when line and take that line to the lead, since it cannot hold then.

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
