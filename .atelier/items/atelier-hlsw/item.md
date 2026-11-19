---
schema: atelier.item
schema_version: 3
id: atelier-hlsw
title: Give the implementer a way to say no, in the skill and in what the binary prints
status: draft
mission: atelier-8az0
labels: []
priority: medium
blocked_by:
- atelier-zwh1
implementers: []
ready: null
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
  It says a finding that does not block may be left with a basic reason in
  its line under Responses.
- [ ] It keeps what must hold: an implementer does not ignore a blocking
  finding and passes no rule on its own work. The lead's ruling is the
  lead's to record.
- [ ] `roles/reviewer.md`, the line `status` prints for answering findings,
  and what a reviewer is told about a rebuttal agree with it. A test covers
  each printed line.
- [ ] atelier-zwh1's case is run with this text and without it, with
  `gpt-6.1-sol` at `high` in every role, ten runs a cell. The results give
  both, each of the three findings apart. The text stays where the real
  defect is fixed no less often with it.
- [ ] `docs/design.md` records the decision, its date, and the evidence.

## Scope

`skills/agent-factory/roles/implementer.md` and `roles/reviewer.md`; the
lines the binary prints about answering and rebutting a finding, in
`crates/`, with their tests; `docs/design.md`;
`evals/agent-factory/results/`. No new command, role or record.
