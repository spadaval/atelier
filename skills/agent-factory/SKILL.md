---
name: agent-factory
description: Use in repositories tracked with Atelier, to plan or shape work, lead a mission, implement an item, review a change or a plan, answer a question an implementer escalated, or garden, groom or audit a repository. It says how to decide; `atelier status` says what to run.
---

# Agent Factory

Run `atelier status` first. If `atelier` is not found, or `status` fails
because this is not a Git checkout or has no Atelier config, this is not an
Atelier repository: stop using this skill and do the task directly.

## Your Role

Read one role file, then follow what `status` prints. Where `status` names
your role, read that role's file:

- the implementer of an item: [roles/implementer.md](roles/implementer.md);
- the reviewer of an item or a plan: [roles/reviewer.md](roles/reviewer.md);
- the lead of a mission: [roles/lead.md](roles/lead.md).

Two roles are routed by the task you were given, whatever `status` suggests
you start:

- Asked to plan or shape work, such as a mission or a draft item:
  [roles/planner.md](roles/planner.md).
- Answering a question with `atelier decide`, because a task line started you
  to or because you lead the item's mission:
  [roles/decider.md](roles/decider.md).

Holding nothing else, read the file of the role your task gives you: a task
to start an item makes you its implementer, and one to lead a mission its
lead. A garden or audit task also reads
[procedures/garden.md](procedures/garden.md), and a groom task
[procedures/groom.md](procedures/groom.md).

## Who Works With Sumeet

The planner works with Sumeet in conversation. A gardener takes its focus
from him when he names one at the start, and settles its run's page with him
in a session once the run is approved. Every other role
runs unattended: it never waits on him, and takes a question to `atelier
escalate`, the mission's lead or an item labelled `needs-human`, as its role
file says.

## Independent Review

Every non-trivial change is reviewed before it merges by a thread that did
not write it. No thread approves its own work or decides a finding on it. A
change is trivial only when its correctness is evident from the diff alone,
such as a typo; rewording a document is not trivial.

## Starting Another Thread

Atelier never starts an agent. When a command prints a task for another
thread, start that thread yourself with fresh context, and give it that task
line and nothing else: not a fork of your context, and none of your
reasoning.

- Under T3 Code, `delegate_task` starts a new thread, whatever the provider.
- Under Codex, a subagent from `spawn_agent` is a thread of its own and may
  review or decide. Pass `fork_turns: "none"`; its default copies your
  context.
- Under Claude Code, a subagent from the `Agent` tool runs as your thread and
  is refused.
