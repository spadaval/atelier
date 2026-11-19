---
name: agent-factory
description: Use in repositories tracked with Atelier, to plan or shape work, lead a mission, implement an item, review a change or a plan, go over, sweep or reorder a backlog, or garden or audit a repository. It says how to decide; `atelier status` says what to run.
---

# Agent Factory

Run `atelier status` first. If `atelier` is not found, or `status` fails
because this is not a Git checkout or has no Atelier config, this is not an
Atelier repository: stop using this skill and do the task directly.

## Who Works With the Human

The planner works with the human in conversation, and builds a spike with them
when the plan needs one. A gardener takes its focus
from them when they name one at the start, and settles its run's page with them
in a session once the run is approved. Every other role
runs unattended: it never waits on them, and takes a question to the thread
that started it or to an item labelled `needs-human`, as its role file says.

## Independent Review

Every non-trivial change is reviewed before it merges by a thread that did
not write it. A change is trivial only when its correctness is evident from
the diff alone, such as a typo; rewording a document is not trivial. No
thread approves its own work or settles a finding on it, and an implementer
passes no rule on its own work: a fix-now finding is fixed, rebutted or
taken to the mission's lead. The lead settles it, and may depart from any of
this, a review included, with its reason on the command.

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
