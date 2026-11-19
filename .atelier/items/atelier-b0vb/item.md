---
schema: atelier.item
schema_version: 3
id: atelier-b0vb
title: 'Try the per-part question on plans: does a plan reviewer ask which Outcome each item serves'
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T10:33:26.896403926Z
---

## Goal

Mission atelier-8az0 found where unneeded work gets past review. A Sol
reviewer names correct but overbuilt work when the excess goes beyond what
the item asks: 59 of 60 reviews of atelier-s6xk's cases. It never questions
work the item itself asks for: 0 of 50 reviews of five changes this
repository built and later removed as unneeded (atelier-ad0z), where a
Done when line named each part. So the excess entered at the plan, and an
item's reviewer is the wrong place to catch it.

The per-part question the old skill held covers plans too: for a plan the
parts are its items, criteria and mechanisms, each asked against the
Outcome. atelier-ufbm, which would have tried it on item reviews, was
dropped because no case could test it there. Planning is the human's
conversation, so whether and how to try it on plan reviews is theirs to
direct.

## Done when

- [ ] The human has said whether to try it.
- [ ] If so, plan-review cases exist where a plan asks for a part its
  Outcome does not need, with controls, and the passage is run with and
  without on them. The plans behind atelier-ad0z's five removed changes are
  real ones to draw on.

## Scope

`skills/agent-factory/roles/reviewer.md` and `roles/planner.md`, and
`evals/agent-factory/`.
