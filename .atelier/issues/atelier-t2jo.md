---
created_at: "2026-11-10T21:30:48.251549219+00:00"
id: "atelier-t2jo"
issue_type: "spike"
labels:
- "redesign"
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
status: "todo"
title: "Run fresh agents through a round of changes requested, the path most items take"
updated_at: "2026-11-10T21:30:48.251549219+00:00"
---

## Goal

Test the redesign's riskiest assumption on the path most items take. Step A showed that a fresh agent told only "Run `atelier status` and do what it says." takes a ready item to done when its reviewer approves at once. Most real items go through a round of changes first. This item runs fresh agents from `changes_requested`, and changes the wording of what the commands print until they get through.

## Done when

- [ ] `evals/agent-factory/scenarios/next-loop-changes/` holds a scenario built on `next-loop`'s harness support: its fixture is a repository with no forge and one item in `changes_requested`, held by the thread the harness gives the agent, with a round-1 review file by another thread that holds one fix-now finding about a real defect in the change. The prompt is "Run `atelier status` and do what it says."
- [ ] Its grader checks the state left behind: the item is done on the base; a round-2 review file has the verdict approve, a reviewer that is neither the implementer nor round 1's reviewer, and the carried finding marked fixed; the Report's Responses has a line for the finding; and the change's own test, which fails on the fixture's defect, passes. It counts refused and errored `atelier` calls as `next-loop`'s grader does, and its tests show it failing a run for each of those reasons.
- [ ] `evals/agent-factory/results/<date>-next-loop-changes.md` records each round's runs on Claude Opus and on Codex: pass or fail, refused and errored calls, `atelier` calls and total tool calls, with the transcript excerpt for each failure and each refusal, and what changed between rounds.
- [ ] In the final round, two runs on Claude Opus and two on Codex each pass with at most one refused or errored `atelier` call.
- [ ] The `next-loop` scenario still passes after any wording change: one run on each agent, recorded in the same results file.

## Scope

One scenario and its grader, and changes to wording, help and the shape of what the commands print, inside the product specification. A change the specification does not allow is escalated. If three rounds of changes do not bring every run to at most one refused or errored call, escalate with what was tried. The harness changes only if this scenario cannot run without it. No file under `crates/` is changed.

## Context

The first scenario, its grader and the harness's stand-in for T3's threads are atelier-67nn's: `evals/agent-factory/scenarios/next-loop/`, `evals/agent-factory/harness/` and `evals/agent-factory/results/2026-11-10-next-loop.md`. `escalate` and `decide` (atelier-axfa) change what `status` prints for an item in `changes_requested`, which is why this item follows it.

This is the third of step C's three items in `docs/strategies/redesign-2-core.md`. It adds proof, not product behaviour, and it is not a mission criterion.
