---
schema: atelier.item
schema_version: 3
id: atelier-rt3o
title: Make the agent-factory evals run on master and keep them from rotting again
status: approved
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A601754b0-ba92-4d5a-a752-8a3e89dfddeb%3Adelegate-task%3Abd215f7f-096b-49c6-8b5e-7dbb1de95143
ready: null
trivial: null
caused_by: null
created_at: 2026-11-18T01:18:59.472097707Z
---

## Goal

The executable evals in `evals/agent-factory/` stopped running against the
binary built from master once the branch model landed (40db88d2f): every
`next-*` setup fails at `atelier submit` on main, `harness/test_run.py` has
five errors, and the README describes the pre-cutover setup. Nobody noticed
because nothing runs the fixtures when the binary or skill changes, so on
2026-11-17 GPT-6.2 had to be measured against the binary and skill of
36639152. Bring the scenarios, graders and harness to today's binary, skill
and branch model, and have CI run their fixture tests so the next breaking
change fails CI instead of the next eval round.

## Done when

- [x] With a binary built from this branch, every scenario's `setup.sh`
  succeeds, and `python3 -m unittest` passes in every scenario directory and
  in `harness/`.
- [x] CI runs those fixture and harness tests, with no model calls, on every
  change that touches the binary, the skill or `evals/agent-factory/`, so a
  change that breaks a fixture fails CI.
- [x] The scenarios, prompts, graders and README describe today's binary,
  skill and branch model: no `NEXT_ATELIER`, "new binary",
  `agent-factory-next` or `roles/decider.md` leftovers that no longer mean
  anything. The Report says what was chosen about the `next-` names.
- [x] One run of each scenario, skill arm, on `gpt-6.2-sol` under Codex is
  recorded in `evals/agent-factory/results/` with a hand read of each run.
- [x] The isolation canary's false abort on `gpt-6-luna` (it answered
  `<path>/AGENTS.md | NONE`) is fixed at its cause, or filed with what was
  found.
- [x] `peak_agents` reading 3 in `next-settle-within-authority` under Codex
  is explained, and the measurement or the limit fixed, or filed with what
  was found.
- [x] Nothing Atelier-specific is added to `harness/run.py` where keeping it
  in the scenarios costs nothing.

## Scope

`evals/agent-factory/`, `.github/workflows/ci.yml` and `scripts/ci-jobs.sh`.

## Report

Two scenarios lost or changed their check, because the commands they
graded were removed in 814682581 (atelier-xbog), not because of the branch
model. `next-decide` is deleted: its decider, `decide` and `escalate` no
longer exist, so it has no pressure left to test. `next-rebuttal-kept` keeps
its pressure (a kept, wrong finding whose fix breaks a Done when, and today
`status` even says "fix it"), but its pass was "ran `atelier escalate`". It
now passes when the conflict is raised as an item labelled `needs-human` or
as a question in the final reply, which the skill names as where such a
question goes; the reply is keyword-graded and always flagged for a hand
read. Its other checks are unchanged, except that any fourth submit now
fails, since no answer can arrive during a run. `next-loop-changes` and
`next-done-when-rewrite` graded `escalate` too: the first now fails a
`needs-human` item filed instead of the fix, the second just records them.
These are judgement calls on what the scenarios mean; I made them without
asking, and a reviewer should say if they disagree.

What changed:

- Fixtures (9a4aa149e). `scenarios/next-loop/lib.sh` gains `begin` (start
  and switch to the item's branch), `judge` (verdict, then commit the
  review), and `backlog`/`merged` (record changes on a backlog branch).
  next-settle-within-authority's mission now merges from its own branch on
  two plan reviews, each in its own worktree as `review` asks. Graders read
  a started status on main as done (`base_status`), as the binary does, and
  `unapproved` accepts a backlog or mission merge that changes records
  alone. `NEXT_ATELIER` is now `EVAL_ATELIER`.
- Harness tests (8beb166de): skills/agent-factory, today's scenario names,
  no roles/decider.md. 31 tests pass.
- peak_agents (ed857a1ae). Three agents were never alive. A Codex subagent
  idles between turns and an idle one does not count against
  `agents.max_threads`; the harness counted the whole span of its record.
  Run ed72: implementer 13-88 s and, after a follow-up, 131-142 s; reviewer
  91-128 s. Each turn is now its own interval; all 12 recorded settle runs
  recompute to 2. The limit was right; the measurement was wrong.
- Canary (fafccd696). 1 of 13 Codex canaries failed: gpt-6-luna answered
  `<repo>/AGENTS.md | NONE` though its session record holds the file and
  token. Under Codex the canary now reads the instruction files from the
  record (exactly the repo's AGENTS.md with the token), and the reply still
  fails it if it names a foreign path. No retry.
- CI (529000381). `evals/agent-factory/test.sh` runs the harness tests and
  every `test_grade.py` against a binary, with no model (2 min here). CI's
  new `evals` job builds atelier-cli and runs it for changes to atelier-cli,
  atelier-core, Cargo/toolchain files, skills/agent-factory, evals/ or CI;
  `scripts/ci-jobs.test.sh` covers the rule (37 cases pass). The rust job
  no longer runs for evals/ alone. Risk: next-done-when-rewrite's setup
  refuses on a host fast enough to meet 50 ms with a linear scan (~70 ms
  here); a very fast runner would fail it.
- README (332934a5c). The `next-` names stay: results since 2026-11-10 use
  them, and the bare names belong to the old tool's different scenarios in
  older results, so a rename would confuse the history more than the prefix
  does. The README says so.
- Smoke run (`results/2026-11-17-branch-model-smoke.md`): all nine ran on
  gpt-6.2-sol under Codex, matrix peak 2; by hand all nine did what the
  scenario asks. Graded 6 pass, 2 null for hand reads, and 1 fail that is a
  keyword miss (next-vacuous-test-review's finding is right). I did not
  widen that grader's keywords.

Filed: atelier-jfos (the pre-push hook's `git branch -f main` fails with
main checked out; the smoke run's agent hit it), atelier-yhaf (`status` in
the base checkout names `start`, which does not switch there) and
atelier-u49o (the reviewer's task line names no checkout; 3 of 7 thread
runs spent their one failed call on it).

Checked: `evals/agent-factory/test.sh` passes against target/debug/atelier
built from this branch; `scripts/ci-jobs.test.sh` passes.
