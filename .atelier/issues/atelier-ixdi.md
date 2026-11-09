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
status: "todo"
title: "Start every child agent with T3's delegate_task, whatever its provider"
updated_at: "2026-11-08T21:55:32.592760222+00:00"
---

## Goal

Orchestrators start children two ways today. In the 30 days to 2026-11-08, T3 recorded 1,591 children started with Claude's native `Agent` tool and 144 started with T3's `delegate_task`. T3's own orchestration prompt (`packages/provider-core/src/server/orchestrationInstructions.ts`, upstream) sends same-provider children to the native tool and only other providers to `delegate_task`.

Using `delegate_task` for every child gives one mechanism for every model, so moving a role from Opus to GPT-6.1 Sol is a model choice rather than a different tool. Every child is also a T3 thread: Sumeet can open it, the orchestrator can answer an escalation with `t3_thread_send`, and T3 records each child's brief, model and cost, which cost per issue needs.

Cost per session is about the same either way (implementation $6.34 native against $6.89 delegated; review $1.16 against $1.08). A T3 child starts at about 37K tokens of context against 21K for a native subagent. T3 gives subagent threads no sidebar row and shows them under their nearest listed ancestor (`resolveSidebarRowThreadId` in `apps/web/src/components/Sidebar.logic.ts`), so more children don't crowd the sidebar.

## Done when

- [ ] Agent Factory tells every agent that starts a child (orchestrators, and implementers starting reviewers) to use `delegate_task` when it runs under T3, overriding T3's same-provider default, without a patch to T3's prompt. Outside T3 it uses the harness's own subagent tool, and nothing in the skill requires T3 or a second provider.
- [ ] Every new review round is a new `delegate_task`, never a `t3_thread_send` to an earlier reviewer, matching T3's rule.

## Context

- Part of atelier-ss4y, which defines the review loop these children run.
