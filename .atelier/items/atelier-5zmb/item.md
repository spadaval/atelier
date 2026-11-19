---
schema: atelier.item
schema_version: 3
id: atelier-5zmb
title: A Codex thread with no skill will not start the reviewer that submit tells it to
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T03:20:32.190750433Z
---

## Goal

`submit` and `status` tell a thread to start a reviewer or an implementer
itself, "without waiting to be asked". A Codex thread does not take that
from a command's output. Codex 0.169 adds a rule of its own when it offers
`spawn_agent`: start no subagent unless the user, an `AGENTS.md` or a skill
explicitly asks. So in a repository with the binary and no Agent Factory
skill, a Codex thread stops and asks its user for leave: 1 of 2 no-skill
`sol` leads did in atelier-69oh's runs, after being told how to start a
thread, and the runs of 2026-11-11 and 2026-11-17 show the same. With the
skill the rule is met, since the skill asks.

Decide whether a repository that uses Atelier with no skill should work
under Codex, and if so where the ask goes: an `AGENTS.md` line that
`atelier setup` offers is one place Codex's rule accepts.

## Done when

- [ ] The human or a planner has decided whether this is worth fixing for a
  repository with no skill.
- [ ] If it is, a Codex thread with the binary and no skill starts the
  reviewer `submit` names without asking its user, shown by the `loop` eval
  case in `sol` with no skill.

## Scope

What `atelier setup` writes and the binary prints; `evals/agent-factory`
for the proof. Not the skill.
