---
created_at: "2026-11-10T00:56:07.538002081+00:00"
id: "atelier-67nn"
issue_type: "spike"
labels:
- "redesign"
workflow_branch:
  branch_base: master
  work_branch: atelier-67nn
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-syyj"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Run a fresh agent through the loop with only `status` to guide it, on Claude Opus and Codex"
updated_at: "2026-11-10T18:41:37.472953783+00:00"
---

## Goal

Test the redesign's riskiest assumption before anything else is built on it: that an agent shown its status and next command follows them without fumbling. Change the wording and shape of the six commands until it does.

## Done when

- [x] `evals/agent-factory/scenarios/next-loop/` holds a scenario: a fixture repository with a config that names no forge and one ready item asking for a small real change, `atelier` on the PATH resolving to the new workspace's binary, no Agent Factory skill installed, and the prompt "Run `atelier status` and do what it says."
- [x] Its grader checks the state left behind: the item is done on the base, a review file has the verdict approve and a reviewer that is not the implementer, and the change's own test passes. It counts refused and errored `atelier` calls from the transcript.
- [x] `evals/agent-factory/results/<date>-next-loop.md` records three runs with Claude Opus and three with a Codex model: pass or fail, refused and errored calls, `atelier` calls, and total tool calls, with the transcript excerpt for each failure and each refusal.
- [x] No run in the final round has more than one refused or errored `atelier` call. The results file says what changed between rounds.

## Scope

The six commands of step A, on one fixture. Changes this item makes are to wording, help and the shape of `status`, inside the product specification. A change the specification does not allow is escalated. If three rounds of changes do not bring every run to at most one refused or errored call, escalate with what was tried.

## Context

This is the mission criterion `fresh-agent-loop` and the evidence boundary before step B. Harness: `evals/agent-factory/harness/run.py` and `evals/agent-factory/README.md`, which covers isolation from the host's own instructions.

## Unknowns

- Whether the eval harness can let the agent start its own reviewer child under Codex. If it cannot, the scenario runs the reviewer as a second invocation given the handoff line `submit` printed, and the results file says so.

## Report

The criterion holds. In the final round (binary at ce0c2d4), three Claude Opus 5.5 runs and three Codex `gpt-6.1-sol` runs each took the ready item to done with a reviewer that was another thread; every run passed the grader, five had no refused or errored `atelier` call and one had one (Codex 9487: `submit` refused once because it had not ticked the Done when lines). The scenario is `evals/agent-factory/scenarios/next-loop/`; the grader's fixtures (`python3 -m unittest` there, with `NEXT_ATELIER` set, 8 tests OK) show it failing an approved but unmerged item, a review whose reviewers include the implementer, tests that pass on the old code, and a failing test on main. The results, every fumble with its excerpt, and the changes between rounds are in `evals/agent-factory/results/2026-11-10-next-loop.md`. `cargo fmt --check`, clippy with `-D warnings` and `cargo nextest run` in `next/` (105 tests) pass; the harness tests (7) pass; `git diff --check` is clean; nothing under `crates/` changed.

### Decisions

- The reviewer is a thread the harness starts, not a second invocation. A Claude Code `Agent` subagent carries its parent's `CLAUDE_CODE_SESSION_ID` (headless and under T3) and its parent's `T3CODE_THREAD_ID` (under T3); a Codex `spawn_agent` child shares `CODEX_SESSION_ID` and has its own `CODEX_THREAD_ID`, which Atelier does not read. So the harness stands in for T3 Code: each agent gets its own `T3CODE_THREAD_ID`, and a `delegate` command, described in the system prompt and nowhere else, starts another as a new thread. The results file and the architecture specification's Threads and roles record this.
- `acts::next` offers a former reviewer nothing rather than the take-over (architecture specification, Status). Taking the item would put that thread on both sides of it.
- Codex 9487's one refusal was left as it is: the refusal named both unticked lines and the agent recovered in one step.

### Open items

- Every reviewer approved in round 1 of review, so the changes-requested loop (findings, Responses, a second round) is untested by agents. A second fixture whose change has a defect a reviewer should catch would test it.
- Under real T3 Code, Claude's guidance prefers its native `Agent` tool for a same-provider child, which Atelier refuses as the reviewer. The handoff now names that case; no run tested recovery from it.
- Reading `CODEX_THREAD_ID` before `CODEX_SESSION_ID` would let a Codex `spawn_agent` child review its parent's item. That changes the identity rule in the architecture specification, so it is for the orchestrator or Sumeet, not this item.
