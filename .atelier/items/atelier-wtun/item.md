---
schema: atelier.item
schema_version: 3
id: atelier-wtun
title: Give each eval run its own ports, so two runs of one case cannot collide
status: draft
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T04:30:44.066503886Z
---

## Goal

Runs of a matrix now go side by side (atelier-u708). In `refused-push` the
agent may start the fixture's server, which listens on port 8080 unless
told otherwise, so two runs of that case at once can collide and one fails
for a reason that has nothing to do with the agent. atelier-u708's reviewer
deferred it, and the README names it. Until it is fixed that case's cells
are run one at a time.

## Done when

- [ ] Two runs of any case at once cannot take the same port: each run is
  given ports of its own, or a network of its own.
- [ ] A test starts two runs of `refused-push`'s fixture server at once
  with no model and both listen.
- [ ] The README's note on the collision is gone.

## Scope

`evals/agent-factory/harness/`, the `refused-push` case and the README.
