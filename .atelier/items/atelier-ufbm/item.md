---
schema: atelier.item
schema_version: 3
id: atelier-ufbm
title: Try the per-part question as a passage, and keep it if reviewers name the unneeded part more often
status: ready
mission: atelier-8az0
labels: []
priority: medium
blocked_by:
- atelier-s6xk
implementers: []
ready:
  by: 'the human, by a side chat on 2026-11-18: tell the main thread about all of this'
  at: 2026-11-19T04:37:19.810241548Z
trivial: null
caused_by: null
created_at: 2026-11-19T04:12:27.755450740Z
---

## Goal

The old skill asked a reviewer one question of every new part, and it was
cut on 2026-11-10 with no case that exercised it: "For each moving part the
change adds (a type, layer, flag, option, cache or branch), ask which
requirement needs it. A part that traces to none is a finding ... the
author answers by naming the requirement or removing the part"
(`git show ff6b56fe4:skills/agent-factory/procedures/review.md`, line 90).
atelier-s6xk's cases can now show whether it helps a reviewer name what a
change does not need.

## Done when

- [ ] A branch adds the passage to `roles/reviewer.md`: the question, that
  a part tracing to no requirement is a blocking finding like `Shrink:`,
  that the reviewer need not design the alternative, and that for a plan
  the parts are its items, criteria and mechanisms, asked against its
  Outcome.
- [ ] atelier-s6xk's five cases run with it as a third arm, on
  `gpt-6.1-sol` at `high`, as two separate matrices of ten runs a cell. The
  skill arm and this arm are staged from two commits that differ in the
  passage alone, and the results name both.
- [ ] The passage is merged into the skill where the three overbuilt cases
  pass more runs with it in each of the two matrices and neither control
  passes fewer in either. Otherwise it is left out. The results say which,
  with the runs it rests on.

## Scope

`skills/agent-factory/roles/reviewer.md`, and
`evals/agent-factory/results/`.
