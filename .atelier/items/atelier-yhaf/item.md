---
schema: atelier.item
schema_version: 3
id: atelier-yhaf
title: Stop status telling the base's checkout that start switches it to a held item's branch
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-18T01:41:53.419888169Z
---

## Goal

In the base's own checkout, with main out, `atelier status` for a thread
that holds a building item ends with `Then: atelier start <id> (switches to
its branch)`. Run there, `start` does not switch: it prints "This checkout
has the base out, so `start` left it as it is" and the `git worktree add`
to run, then the same status with the same `Then:` line. An agent that
follows `Then:` lines loops. Found by atelier-rt3o on f4d29ba76 (fixture
next-loop: `atelier start slug-k3fa` twice in work/repo).

## Done when

- [ ] In the base's checkout, `status` for a thread holding a started item
  names the `git worktree add` (or the checkout that already has the
  branch), not `start`; a test covers it.
- [ ] Elsewhere, where `start` does switch, `status` still names `start`.

## Scope
