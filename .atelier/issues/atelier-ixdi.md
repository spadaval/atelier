---
created_at: "2026-11-08T21:40:08.085314652+00:00"
id: "atelier-ixdi"
issue_type: "task"
labels:
- "agent-factory"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Under T3, start every child agent with delegate_task, whatever its provider"
updated_at: "2026-11-09T06:11:58.279303879+00:00"
---

## Goal

This item applies when Agent Factory runs under T3 Code. Outside T3, agents keep using their harness's own subagent tool, and nothing here requires T3 or a second provider.

Under T3, orchestrators start children two ways today. In the 30 days to 2026-11-08, T3 recorded 1,591 children started with Claude's native `Agent` tool and 144 started with T3's `delegate_task`. T3's own orchestration prompt (`packages/provider-core/src/server/orchestrationInstructions.ts`, upstream) sends same-provider children to the native tool and only other providers to `delegate_task`.

Using `delegate_task` for every child gives one mechanism for every model, so moving a role from Opus to GPT-6.1 Sol is a model choice rather than a different tool. Every child is also a T3 thread: Sumeet can open it, the orchestrator can answer an escalation with `t3_thread_send`, and T3 records each child's brief, model and cost, which cost per issue needs.

Cost per session is about the same either way (implementation $6.34 native against $6.89 delegated; review $1.16 against $1.08). A T3 child starts at about 37K tokens of context against 21K for a native subagent. T3 gives subagent threads no sidebar row and shows them under their nearest listed ancestor (`resolveSidebarRowThreadId` in `apps/web/src/components/Sidebar.logic.ts`), so more children don't crowd the sidebar.

## Done when

- [x] Agent Factory tells every agent that starts a child (orchestrators, and implementers starting reviewers) to use `delegate_task` when it runs under T3, overriding T3's same-provider default, without a patch to T3's prompt. Outside T3 it uses the harness's own subagent tool, and nothing in the skill requires T3 or a second provider.
- [x] Every new review round starts a new reviewer, never a message to an earlier one: under T3 a new `delegate_task`, never a `t3_thread_send`, matching T3's rule; elsewhere a new subagent from the harness's own tool.

## Context

- Part of atelier-ss4y, which defines the review loop these children run.

## Notes

- 2026-11-09 [note] Implementer handoff: see ## Report. Independent review outstanding.

## Report

Outcome: both Done when lines hold at `80ec4fc4`. SKILL.md's core rule "Start children fresh, one way" tells every agent that starts a child, orchestrators and implementers alike, to use `delegate_task` under T3 Code whatever the provider, overriding T3's advice to use the native tool, with no change to T3's prompt; elsewhere the harness's own subagent tool, and no step needs T3 or a second provider. SKILL.md's Who arranges it and implement.md's Review step 4 start a new reviewer each round, never a message to an earlier one. In the evaluation (`evals/agent-factory/results/2026-11-09-implementer-owns-review.md`) the orchestrator started its implementer and the implementer started its Sol reviewer with `delegate_task`, and no one used `t3_thread_send` or a native `Agent`.

### Changes and Proof

- Changes: SKILL.md's core rules and Who arranges it, implement.md's Review, at `80ec4fc4` on branch `atelier-ss4y`.
- Proof: the evaluation above; links checked with atelier-ss4y.

### Review

Independent review outstanding, with atelier-ss4y.
