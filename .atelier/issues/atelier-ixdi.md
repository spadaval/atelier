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
updated_at: "2026-11-08T21:40:08.085314652+00:00"
---

## Goal

Orchestrators start children two ways today. In the 30 days to 2026-11-08, T3 recorded 1,591 children started with Claude's native `Agent` tool and 144 started with T3's `delegate_task`. T3's own orchestration prompt (`packages/provider-core/src/server/orchestrationInstructions.ts`, upstream) sends same-provider children to the native tool and only other providers to `delegate_task`.

Using `delegate_task` for every child gives one mechanism for every model, so moving a role from Opus to GPT-6.1 Sol is a model choice rather than a different tool. Every child also becomes a T3 thread: other threads can message it with `t3_thread_send` (needed for reviewers to reach implementers directly), Sumeet can open it, and T3 records its brief, model and cost per thread, which cost per issue needs.

Cost per session is about the same either way (implementation $6.34 native against $6.89 delegated; review $1.16 against $1.08). A T3 child starts at about 37K tokens of context against 21K for a native subagent.

## Done when

- [ ] Agent Factory's orchestrate procedure tells orchestrators to start every child with `delegate_task`, overriding T3's same-provider default, without a patch to T3's prompt.
- [ ] It's checked first that T3 nests child threads under their parent, so about 1,700 child threads a month don't crowd the sidebar; if they do crowd it, that's reported before this change lands.
- [ ] Fix rounds go to the same implementer thread with `t3_thread_send`, and every new review is still a new `delegate_task`.

## Context

- Agent Factory's source is `spadaval/.agents` (ADR 0023).
- Part of the orchestrator-implementer rework discussed on 2026-11-08 (orchestrator writes no code; fix loop runs between reviewer and implementer through Atelier records).
