---
schema: atelier.item
schema_version: 3
id: atelier-o2vv
title: Keep status from pointing a delegated thread at an unrelated item
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-11T16:48:44.562022233Z
---

## Goal

A thread that another thread starts with one task can be sent the wrong way by what `status` prints before it has done anything. On the first two items run under the new tool, the session-start hook's `atelier status` ended with `Then: atelier start <the top-priority ready item>`, which was not the item either thread had been given, and for the thread working in another repository's worktree it ran in the wrong repository altogether, because the hook runs where the session starts. Both threads followed their task and not the line, and said the line would mislead an agent that trusted it.

## Done when

- [ ] Provisional: a thread that holds nothing is not told to start a particular item as its next step: `status` lists what can start and leaves the choice to the task the thread was given. The product page's description of `status` on the base says so.
- [ ] Provisional: the skill or `AGENTS.md` says what to do when a session starts in one repository and its task is in another: run `atelier status` again in the task's checkout.

## Context

Preliminary; plan it before starting. Seen on 2026-11-11 in the runs of atelier-2mlh and of worldcrafter-s9lp in demiurge. `SKILL.md` already routes a thread by its task where `status` names no role, which is why neither run went wrong.
