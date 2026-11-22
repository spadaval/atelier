---
name: agent-factory
description: Use in repositories tracked with Atelier, to plan or shape work, lead a mission, implement an item, review a change or a plan, go over, sweep or reorder a backlog, or garden or audit a repository. It says how to decide; `atelier status` says what to run.
---

# Agent Factory

Run `atelier status` first. If your session started in a different
repository from your task, run it again in the task's checkout.
If `atelier` is not found, or `status` fails
because this is not a Git checkout or has no Atelier config, this is not an
Atelier repository: stop using this skill and do the task directly.

## Your Role

Read one role file, then follow what `status` prints. Where `status` names
your role, read that role's file:

- the implementer of an item: [roles/implementer.md](roles/implementer.md);
- the reviewer of an item or a plan: [roles/reviewer.md](roles/reviewer.md);
- the lead of a mission: [roles/lead.md](roles/lead.md).

Asked to plan or shape work, such as a mission or an item, or to go
over, sweep or reorder the backlog, read [roles/planner.md](roles/planner.md),
whatever `status` suggests you start.

Holding nothing else, read the file of the role your task gives you: a task
to start an item makes you its implementer, and one to lead a mission its
lead. A garden or audit task also reads
[procedures/garden.md](procedures/garden.md).

## Who Works With the Human

The planner works with the human in conversation, and builds a spike with them
when the plan needs one. A gardener takes its focus from them when they name
one at the start, merges its run once review approves it, and then leaves its
proposals and bugs to them in `needs-human` items; a planner applies their
answers, as [procedures/garden.md](procedures/garden.md) says. A started
gardener, like every other role, runs unattended: it never waits on them, and
takes a question to the thread that started it or to an item labelled
`needs-human`, as its role file says.

## Problems Outside Your Work

When you find a problem outside the work in hand, file it and carry on with
your work: run `atelier new "<title>"` on the branch you are on, write what
you saw and where in the file it names, and commit it with the command it
prints. With no branch, file on one from `atelier start --backlog <slug>`,
and as a plan's reviewer only once your verdict is pushed. What a reviewer
finds in the change under review stays a finding.

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
