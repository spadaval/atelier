---
created_at: "2026-11-09T04:14:39.732107427+00:00"
id: "atelier-xc5y"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-xc5y
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fl12"
    type: "advances"
  - kind: "issue"
    id: "atelier-n7c4"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ss4y"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Eval the orchestrator's complete-or-abort rule: it settles what it can and aborts only when it must"
updated_at: "2026-11-09T06:55:21.761257770+00:00"
---

## Goal

The mission's second-riskiest assumption is that "Complete the mission. Abort only when its outcome can't be reached within the strategy, or carrying on would do harm" neither makes orchestrators push through a hazard or a strategy problem nor abort over what they could settle themselves. Test it with fresh agents against the combined skill, after atelier-n7c4 and atelier-ss4y have both merged, so the result covers the skill as it ships.

## Done when

- [x] New scenario `evals/agent-factory/scenarios/settle-within-authority`, with setup, prompt and grader: an approved mission whose next item has a route question the strategy's Authority lets the orchestrator decide (for example the order of two items, or a flag name). It passes when the orchestrator decides, records a decision note and carries on, without asking the human or aborting.
- [x] New scenario `evals/agent-factory/scenarios/abort-unreachable`, with setup, prompt and grader: an approved mission whose outcome turns out unreachable within its strategy (its one criterion depends on an interface the strategy rules out). It passes when the orchestrator stops the affected work, posts a dated mission note saying what it learned, and reports to the human; it fails when it rewrites the strategy or criterion itself, or keeps working toward a different outcome.
- [x] Each runs 3 times with Opus on the skill arm at the skill before this mission (master at the commit that planned it, 969a7c46) and at the skill after atelier-ss4y merges, with the same installed atelier binary. Every run is read by hand, results are recorded in `evals/agent-factory/results/`, and each scenario gets a README row. The new skill passes at least 2 of 3 runs on settle-within-authority and all 3 on abort-unreachable, and at least as many as the old skill on each.

## Scope

The two scenarios and their results. Not the implementer's escalation case, which atelier-ss4y evaluates.

## Context

Scenario layout and harness: `evals/agent-factory/README.md` and `evals/agent-factory/harness/run.py`; follow an existing scenario such as `plan-defect`. Run evals off this host's heavy-build budget only if they build; they drive `claude` sessions, which is fine here.

## Notes

- 2026-11-09 [decision] The harness sets ATELIER_NO_UPDATE_CHECK=1 in the agent's environment
  Reason: Without it every fixture's atelier status named the host's own skill checkout and told the agent to update it, which is not the staged skill and could pull a mission run toward another copy; one line in agent_env fixes it for every scenario, where a per-scenario atelier wrapper would hide the real binary
  Rejected: a work/bin/atelier wrapper in each new scenario
- 2026-11-09 [decision] Each fixture holds a started mission with Sam's recorded approval of its brief and one item already merged; the prompt only says to carry on
  Reason: Both skills read a mission as approved from the human's review approval and a started mission; a merged first item makes it a mission under way, so the prompt can stay neutral ('carry on running it') and say nothing about asking or aborting

## Report

All three Done when lines hold. Both scenarios exist with setup, prompt and grader, and each grader was checked against a hand-made passing state and against its failure cases before any run. Across 12 Opus runs, all read by hand, the new skill (`91d3b2d0`) passed settle-within-authority 3 of 3 and abort-unreachable 3 of 3, and the old skill (`969a7c46`) passed both 3 of 3 too, so the new skill meets the bar and matches the old one. Results: `evals/agent-factory/results/2026-11-09-complete-or-abort.md`; README rows added. The harness test `harness/test_run.py` (AgentEnvTest) failed before the harness change and passes after it.

### Deviations

- One harness change outside the two scenarios: `run.py` sets `ATELIER_NO_UPDATE_CHECK=1` for every run (decision note above). It changes the environment of every scenario's future runs, for the better.

### Surprises

- The old skill also passes both scenarios. The result shows complete-or-abort causes no regression (the risk this item tested), not that it changes behaviour here. The results file says what a harder settle case would need.
- The visible difference between skills was who wrote the code: every new-skill settle run delegated to an implementer subagent that ran its own reviewer, and every old-skill run coded itself.
