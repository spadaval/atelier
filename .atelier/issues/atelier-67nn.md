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

- [ ] `evals/agent-factory/scenarios/next-loop/` holds a scenario: a fixture repository with a config that names no forge and one ready item asking for a small real change, `atelier` on the PATH resolving to the new workspace's binary, no Agent Factory skill installed, and the prompt "Run `atelier status` and do what it says."
- [ ] Its grader checks the state left behind: the item is done on the base, a review file has the verdict approve and a reviewer that is not the implementer, and the change's own test passes. It counts refused and errored `atelier` calls from the transcript.
- [ ] `evals/agent-factory/results/<date>-next-loop.md` records three runs with Claude Opus and three with a Codex model: pass or fail, refused and errored calls, `atelier` calls, and total tool calls, with the transcript excerpt for each failure and each refusal.
- [ ] No run in the final round has more than one refused or errored `atelier` call. The results file says what changed between rounds.

## Scope

The six commands of step A, on one fixture. Changes this item makes are to wording, help and the shape of `status`, inside the product specification. A change the specification does not allow is escalated. If three rounds of changes do not bring every run to at most one refused or errored call, escalate with what was tried.

## Context

This is the mission criterion `fresh-agent-loop` and the evidence boundary before step B. Harness: `evals/agent-factory/harness/run.py` and `evals/agent-factory/README.md`, which covers isolation from the host's own instructions.

## Unknowns

- Whether the eval harness can let the agent start its own reviewer child under Codex. If it cannot, the scenario runs the reviewer as a second invocation given the handoff line `submit` printed, and the results file says so.
