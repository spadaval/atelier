---
schema: atelier.item
schema_version: 3
id: atelier-8phx
title: Tell every role to file what it finds outside its work, and show with an eval case that the line changes what agents do
status: draft
mission: atelier-dh7a
labels: []
priority: medium
blocked_by:
- atelier-u6oy
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T15:25:03.272209421Z
---

## Goal

No role file tells an agent to record a problem it finds outside the work
in hand. `roles/implementer.md` is silent, a reviewer's instruction covers
findings on the change it reviews, and only the gardener is told to file
something broken. With atelier-u6oy merged, filing costs one command on the
branch the agent already has. This item adds the instruction, and an eval
case that shows whether it changes what agents do. The human said on
2026-11-18 that evals decide what the skill holds, so the line stays only
on a gain.

## Done when

- [ ] The skill says once, where every role reads it or in each role file
  that needs it, that a problem found outside the work in hand is filed
  with `atelier new` on the branch you are on, with what was seen and
  where, and the work carries on. It says a reviewer's finding on the
  change under review stays a finding.
- [ ] `roles/lead.md` and `procedures/garden.md` no longer send a lead or a
  gardener to a backlog branch to file an item when they hold a branch
  already.
- [ ] A new case in `evals/agent-factory` gives an implementer an item
  whose work runs into a planted problem outside its Scope. Code reads
  whether the item merged and whether a new item's record reached main;
  the judge reads whether that record describes the planted problem. The
  case sets up and grades under `test.sh` with no model.
- [ ] The case is run with the line and without it, at least five runs per
  arm in the `house` setup. The README gives for each arm the runs that
  filed the problem, the runs that merged their item, and time and tokens.
- [ ] If more runs file with the line, it stays. If not, the line is
  removed, the Report says so, and the case stays only if the README says
  what it guards.

## Scope

`skills/agent-factory/SKILL.md`, `roles/` and `procedures/garden.md`;
one case and its fixture and rubric in `evals/agent-factory`, with the
README's row for it. Not `procedures/backlog.md`'s rules for a sweep. No
new grader, harness feature or setup: a case that needs one is cut, as
atelier-kw8u's Authority says. At most two eval agents alive at once.
