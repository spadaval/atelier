---
schema: atelier.item
schema_version: 3
id: atelier-oiwy
title: Try four passages back in the skill, each as an arm of its own, and keep what changes an outcome
status: draft
mission: atelier-kw8u
labels: []
priority: medium
blocked_by:
- atelier-id3v
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-18T21:03:01.247579892Z
---

## Goal

Use the evals for what the human asked of them on 2026-11-18: to decide what
the skill holds. Three passages were cut on 2026-11-11 (b22a03359) with no
case that could have shown their worth, and the human has a frame for the
lead that one sentence might carry better than several rules. Try each, and
keep what the runs support.

## Done when

- [ ] Four branches each add one passage to the skill as atelier-mtzj left
  it: the reviewer's "What to judge"; the lead's "Close against the
  Outcome, not the item count"; the implementer's two lines on building to
  the Goal, Done when and Scope and on a Done when that cannot hold; and one
  sentence in `roles/lead.md` saying the lead is the captain, mission
  control is far away and the recorder is running. Each is brought to
  today's commands and says nothing the binary prints.
- [ ] Each runs by `--ref` as an arm of its own, in both setups, on the
  cases that can tell: the review cases for the first, the lead's cases and
  the mission case for the second and fourth, the implementer's cases for
  the third. The results file gives each cell beside the same cell without
  the passage, with time and tokens.
- [ ] Each arm is run as two separate matrices of at least five runs per
  cell. A passage is merged into the skill where some cell passes at least
  two more runs with it in each of the two, and no cell passes fewer in
  either. The others are left out, and the results say so in a line each
  and give the runs every keep rests on.
- [ ] The README's table is brought up to date with what was kept.

## Scope

`skills/agent-factory/SKILL.md` and `roles/`; `evals/agent-factory/results/`
and `README.md`. The four branches are not merged; a kept passage is merged
on this item's branch.
