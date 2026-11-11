---
created_at: "2026-11-10T21:30:48.251549219+00:00"
id: "atelier-t2jo"
issue_type: "spike"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-t2jo
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-axfa"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Run fresh agents through a round of changes requested, the path most items take"
updated_at: "2026-11-11T00:51:31.584904962+00:00"
---

## Goal

Test the redesign's riskiest assumption on the path most items take. Step A showed that a fresh agent told only "Run `atelier status` and do what it says." takes a ready item to done when its reviewer approves at once. Most real items go through a round of changes first. This item runs fresh agents from `changes_requested`, and changes the wording of what the commands print until they get through.

## Done when

- [x] `evals/agent-factory/scenarios/next-loop-changes/` holds a scenario built on `next-loop`'s harness support: its fixture is a repository with no forge and one item in `changes_requested`, held by the thread the harness gives the agent, with a round-1 review file by another thread that holds one fix-now finding about a real defect in the change. The prompt is "Run `atelier status` and do what it says."
- [x] Its grader checks the state left behind: the item is done on the base; a round-2 review file has the verdict approve, a reviewer that is neither the implementer nor round 1's reviewer, and the carried finding marked fixed; the Report's Responses has a line for the finding; and the change's own test, which fails on the fixture's defect, passes. It counts refused and errored `atelier` calls as `next-loop`'s grader does, and its tests show it failing a run for each of those reasons.
- [x] `evals/agent-factory/results/<date>-next-loop-changes.md` records each round's runs on Claude Opus and on Codex: pass or fail, refused and errored calls, `atelier` calls and total tool calls, with the transcript excerpt for each failure and each refusal, and what changed between rounds.
- [x] In the final round, two runs on Claude Opus and two on Codex each pass with at most one refused or errored `atelier` call.
- [x] The `next-loop` scenario still passes after any wording change: one run on each agent, recorded in the same results file.

## Scope

One scenario and its grader, and changes to wording, help and the shape of what the commands print, inside the product specification. A change the specification does not allow is escalated. If three rounds of changes do not bring every run to at most one refused or errored call, escalate with what was tried. The harness changes only if this scenario cannot run without it. No file under `crates/` is changed.

## Context

The first scenario, its grader and the harness's stand-in for T3's threads are atelier-67nn's: `evals/agent-factory/scenarios/next-loop/`, `evals/agent-factory/harness/` and `evals/agent-factory/results/2026-11-10-next-loop.md`. `escalate` and `decide` (atelier-axfa) change what `status` prints for an item in `changes_requested`, which is why this item follows it.

This is the third of step C's three items in `docs/strategies/redesign-2-core.md`. It adds proof, not product behaviour, and it is not a mission criterion.

## Report

Fresh agents get through a round of changes requested. In the final round
(binary b9a34be), two runs on Claude Opus 5.5 and two on Codex `gpt-6.1-sol`
each passed with no refused or errored `atelier` call. Across both rounds, ten
runs passed and one call failed. In round 1, a Codex implementer wrote prose
under its response line in `### Responses`, so `submit` errored on the item.
It fixed the line on its next try. `status` and a refused `submit` now print
the line to write for the real finding, one line each. No agent set
`ATELIER_SKIP` or escalated. Every round-2 reviewer marked the carried
finding `Now: fixed` before approving. next-loop still passes on the final
binary (one run on each agent). The runs, transcript excerpts and limits are
in `evals/agent-factory/results/2026-11-10-next-loop-changes.md`.

Proof behind each Done when line:

- The scenario is `evals/agent-factory/scenarios/next-loop-changes/`. Its
  `setup.sh` drives the real binary from next-loop's fixture to
  `changes_requested`. The item is held by the thread the harness gives the
  agent, and the round-1 review comes from another thread.
- `test_grade.py` shows the grader failing a run for each reason: not
  merged, round 1's reviewer approving, the implementer among the
  approvers, the finding withdrawn rather than fixed, no response line (by
  way of a skip), no test catching the defect, and an escalation. It also
  checks that next-loop's counting patterns catch this path's refusals and
  errors. All nine tests pass, and next-loop's ten still pass.
- The results file holds both rounds' tables, the one failure's excerpt, and
  the change between rounds.
- `cargo fmt --check`, clippy with `-D warnings`, and `cargo nextest run
  --locked` in `next/` (223 tests) pass. The new assertion in
  `scenarios.rs` failed on the old wording first. The harness's tests pass,
  `git diff --check` is clean, and nothing under `crates/` changed.

### Deviations

- `harness/run.py` changed by five lines. It now makes the first thread's ID
  before `setup.sh` and passes it in `T3CODE_THREAD_ID`. Without that, the
  fixture cannot make the agent's own thread the holder, which the first
  Done when line requires.
- Only two rounds ran. Round 1 had one failed call in six runs, inside the
  limit. Round 2 is the final round because a wording change followed round
  1.

### Surprises

- The refusal for approving with a carried finding still `kept`, which was
  expected to trip reviewers, never fired. The brief's instruction to set
  `Now:` was enough in all ten runs.
- No refusal of a rule fired at all, so no agent saw the skip offer. The
  runs do not test how an agent reads a refusal that offers the skip.

### Open items

- The fixture's finding is easy. Rebuttal, the one-try rule, `escalate` and
  `decide` have no fresh-agent run. A scenario where the finding is wrong,
  or not worth fixing, would test them.
- `status` does not name the round-1 review file. Every agent found it in one
  or two calls, so nothing was changed.
- Run directories are under `~/.cache/atelier-fcb7/t2jo/`. Passing runs'
  work directories under `/var/tmp/af-evals` were deleted by the harness.
