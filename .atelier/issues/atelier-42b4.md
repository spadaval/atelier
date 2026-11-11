---
created_at: "2026-11-11T02:20:59.603957056+00:00"
id: "atelier-42b4"
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
    id: "atelier-t2jo"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Run fresh agents through a rebuttal, an escalation and a refusal that offers the skip: no fresh-agent run has met them"
updated_at: "2026-11-11T02:20:59.603957056+00:00"
---

## Goal

Test the paths of the new work loop that no fresh agent has walked. Mission atelier-fcb7 ran fresh agents through the plain path (`evals/agent-factory/scenarios/next-loop/`) and through one round of changes requested with an easy fix (`next-loop-changes/`). In 22 runs no agent met a refusal that offers `ATELIER_SKIP`, wrote a rebuttal, ran `escalate` or `decide`, or reached a third round. A fixture whose finding is wrong, or right but not worth fixing, would test them.

## Done when

- [ ] A scenario built on `next-loop-changes` starts from a finding the implementer should rebut or escalate, and its grader checks the state left behind and counts refused and errored calls as the other two do.
- [ ] Its results file records runs on Claude Opus and on Codex, every fumble, and what wording changed for it.

## Context

Results so far: `evals/agent-factory/results/2026-11-10-next-loop.md` and `2026-11-10-next-loop-changes.md`. Pick this up when the skill for the new tool is written (mission 3), since the skill is what tells an agent when to rebut and when to escalate.
