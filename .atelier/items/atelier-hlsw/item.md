---
schema: atelier.item
schema_version: 3
id: atelier-hlsw
title: Give the implementer a way to say no, in the skill and in what the binary prints
status: building
mission: atelier-8az0
labels: []
priority: medium
blocked_by:
- atelier-zwh1
implementers:
- thread:delegated-task:command%3Amcp%3A3a9fab39-7f29-4732-8284-d6c07f2deac6%3Adelegate-task%3Ac7daf521-b7c3-41df-9032-9593fd6dbdc0
ready:
  by: 'the human, by a side chat on 2026-11-18: tell the main thread about all of this'
  at: 2026-11-19T04:37:19.810241548Z
trivial: null
caused_by: null
created_at: 2026-11-19T04:12:27.576691099Z
---

## Goal

The human's design, of 2026-11-18: "the reviewer can mark a subset of
findings as must-fix and they cannot be ignored, the implementer can defer
anything else with only a basic reason or ask the lead to override a
fix-now. The lead can override everything." Today `roles/implementer.md`
says "You cannot dismiss a finding", a rebuttal needs "a fact or a test",
and the lead is reached only after a rebuttal is lost. This was their
design once before (f684caa15, 2026-11-08). `escalate` and `decide` were
removed on 2026-11-13 as unused, and `docs/design.md` asks for new evidence
before they come back. The evidence is the survey in the mission's Why and
atelier-zwh1's case. No command comes back: the lead already passes a rule
with a recorded reason.

## Done when

- [ ] `roles/implementer.md` gives a finding that blocks three exits: fix
  it; rebut it once, where it is wrong, with a fact or a test; or, where it
  is real and not worth building, take it straight to the mission's lead
  with a basic reason, and with no mission to the thread that started you.
  It says a finding that does not block may be left with a basic reason.
- [ ] A line under Responses takes a third form that says a finding was
  not built, with its reason, and is accepted wherever `fixed in` and
  `rebutted:` are. On a finding that blocks, `status` shows the line to the
  mission's lead, the next round's reviewer is told the finding waits on
  the lead and does not judge it again, and `merge` refuses while it is
  open unless the lead passes the rule with a reason. An item with no such
  line reads as before.
- [ ] It keeps what must hold: an implementer does not ignore a blocking
  finding and passes no rule on its own work. The lead's ruling is the
  lead's to record.
- [ ] `roles/reviewer.md`, the line `status` prints for answering findings,
  and what a reviewer is told about a rebuttal agree with it. A test covers
  each printed line.
- [ ] atelier-zwh1's case is run with this change and without it, with
  `gpt-6.1-sol` at `high` in every role, ten runs a cell, each arm staged
  from one of two commits that differ in this change alone. The results
  name both commits and give each of the three findings apart. The change
  stays where the real defect is fixed no less often with it.
- [ ] The same two arms are also run in `house`, ten runs a cell. The
  survey's 27 findings that were built without need were built by Opus and
  Fable, and atelier-zwh1's ten Sol implementers built none: they rebutted
  the finding that was true and not worth building, and a reviewer then
  deferred it. So `sol` cannot show the caving this change is for, and
  `house` may.
- [ ] `docs/design.md` records the decision, its date, and the evidence.

## Scope

`skills/agent-factory/roles/implementer.md` and `roles/reviewer.md`; the
third form of a Responses line, where the two are parsed, shown and
checked in `crates/`; the lines the binary prints about answering and
rebutting a finding, with their tests; `docs/design.md`;
`evals/agent-factory/results/`. No new command, role or section of a
record.
