---
name: agent-factory
description: Use in repositories tracked with Atelier, to plan or shape work, lead a mission, implement an item, review a change or a plan, go over, sweep or reorder a backlog, or garden or audit a repository. It says how to decide; `atelier status` says what to run.
---

# Agent Factory

## Starting Another Thread

Atelier never starts an agent. When a command prints a task for another
thread, start that thread yourself with fresh context, and give it that task
line and nothing else: not a fork of your context, and none of your
reasoning.

- Under T3 Code, `delegate_task` starts a new thread, whatever the provider.
- Under Codex, a subagent from `spawn_agent` is a thread of its own and may
  review. Pass `fork_turns: "none"`; its default copies your
  context.
- Under Claude Code, a subagent from the `Agent` tool runs as your thread and
  is refused.
