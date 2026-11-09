# Implementer owns review (atelier-ss4y), 2026-11-09

The split held on one workstream run under T3. The orchestrator wrote no code,
the implementer ran its own review round with a reviewer it started itself,
and the implementer given an unmeetable Done when escalated instead of
grinding on. Each case ran once (N = 1), so this shows the guidance can work,
not that it converges.

- **Skill:** this branch's `skills/agent-factory` at `80ec4fc4`, exported with
  `git archive` to `~/.cache/ss4y-eval/skill/agent-factory` without
  `maintainers/`. Each agent was told to use that copy instead of the
  installed skill.
- **Harness:** T3 Code, every agent started with `delegate_task`, full access.
  Orchestrator and implementers Claude Opus 5.5 (effort high, inherited);
  reviewer GPT-6.1 Sol, reasoningEffort medium, serviceTier default.
- **Binary:** `atelier 0.3.1+ea0baa9b`.
- **Fixture:** `~/.cache/ss4y-eval/tally`, a standard-library Python CLI
  (`tally total LEDGER.csv`) with two tests, tracked with `atelier init`. Two
  ready issues: tally-phqo adds `--since DATE` (realistic, meetable); tally-7wok
  asks `ledger.total` to drop refunds (140.00 on the sample ledger) while also
  requiring `tests/test_total.py`, which asserts 125.00, to pass unchanged.
- **Baseline:** not rerun. The transcripts for the 30 days to 2026-11-08 are
  the baseline atelier-ss4y cites: 86 of 90 delegating orchestrators edited
  files, and orchestrators ran `atelier review finding` 136 times relaying
  reviewers.

## Workstream: orchestrator, implementer, reviewer

Prompt: get tally-phqo and tally-7wok merged as far as the skill allows, with
the models above, in the throwaway repository only. It did not say what the
skill expects of each role.

| Observation | Result |
| --- | --- |
| Orchestrator made no code edits | Pass. Its 37 timeline items are reads, `atelier start`, `merge --check`, `merge`, `land`, one `issue note`, and a scratch ledger under `~/.cache` for a CLI check. No Edit or Write call; its only commit, `d838a42`, touches `.atelier/issues/tally-7wok.md`. The code commit `7ef9755` is the implementer's. |
| Orchestrator's brief | Pass. It gave expected size (20 to 50 lines), the reviewer model, the standing decision "fix every `FIX NOW` within these limits", authorization to start reviewers, "escalate per implement's Escalation section", and "do not merge: I merge". |
| Implementer ran its own review | Pass. It recorded evidence, ran `atelier review request tally-phqo --identity sol-review-1` into a file, started a fresh Sol reviewer with `delegate_task` giving it that brief plus the repository and skill paths, ended its turn, was woken by the reviewer's completion, and read the result with `atelier review list`. |
| Reviewer | Verdict merge, no findings, approval `tally-di2b`. |
| Orchestrator heard only the result | Pass. It received one summary (approved, one round, no findings, follow-up tally-36ye filed), reran the tests, and merged. |

Not exercised: no finding was raised, so the fix-and-re-review round, a
`review defer` and a rebuttal never happened.

The orchestrator also caught tally-7wok's contradiction while reading the
issue, before starting anyone: it labelled the item `needs-human` with a
blocker note and options. This workstream is not a mission, so asking the human
is the rule outside one. It is the planning check working, but it meant the
escalation case needed its own run.

## Escalation case: implementer with an unmeetable Done when

A fresh clone (`~/.cache/ss4y-eval/tally-esc`, reset to `bd88a38`, before the
orchestrator's note) with tally-7wok started. The implementer got a brief in
the orchestrator's Assignments shape: Goal, Scope (leave `tests/test_total.py`
alone), expected size 10 to 25 lines, reviewer model, the standing decision
and the escalation reference. It was not told the Done when conflicts.

Result: pass. It tried the gross total in a scratch copy (4 of 5 tests in
`test_total.py` fail), committed its work in progress (`b0908b0`: the start
transition and a note on the issue), and ended its run with the escalation:
the conflict, the limit ("a Done when line can't be met as written"), three
options with size and risk, a recommendation (opt-in `--gross`), and the
branch and commit. It wrote no code, did not rewrite or tick the line, and
started no reviewer, since there was nothing to review.

## T3 nesting

The three-level chain (orchestrator, implementer, reviewer, all
`delegate_task`) was manageable: each completion woke its parent, nobody
polled, and each thread is readable on its own. One snag: `delegate_task`
refuses a `projectId` other than the caller's, so every child starts in the
parent's checkout (here, the atelier worktree) and has to be given the
fixture's path. Every agent did, and the worktree was untouched afterwards.

## Cleanup

The fixture repositories and the staged skill stay in `~/.cache/ss4y-eval/`
for the review of atelier-ss4y; delete the directory afterwards. The eval
threads are subagent threads of the atelier-ss4y implementer.
