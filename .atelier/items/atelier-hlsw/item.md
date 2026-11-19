---
schema: atelier.item
schema_version: 3
id: atelier-hlsw
title: Give the implementer a way to say no, in the skill and in what the binary prints
status: approved
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

- [x] `roles/implementer.md` gives a finding that blocks three exits: fix
  it; rebut it once, where it is wrong, with a fact or a test; or, where it
  is real and not worth building, take it straight to the mission's lead
  with a basic reason, and with no mission to the thread that started you.
  It says a finding that does not block may be left with a basic reason.
- [x] A line under Responses takes a third form that says a finding was
  not built, with its reason, and is accepted wherever `fixed in` and
  `rebutted:` are. On a finding that blocks, `status` shows the line to the
  mission's lead, the next round's reviewer is told the finding waits on
  the lead and does not judge it again, and `merge` refuses while it is
  open unless the lead passes the rule with a reason. An item with no such
  line reads as before.
- [x] It keeps what must hold: an implementer does not ignore a blocking
  finding and passes no rule on its own work. The lead's ruling is the
  lead's to record.
- [x] `roles/reviewer.md`, the line `status` prints for answering findings,
  and what a reviewer is told about a rebuttal agree with it. A test covers
  each printed line.
- [x] atelier-zwh1's case is run with this change and without it, with
  `gpt-6.1-sol` at `high` in every role, ten runs a cell, each arm staged
  from one of two commits that differ in this change alone. The results
  name both commits and give each of the three findings apart. The change
  stays where the real defect is fixed no less often with it.
- [x] The same two arms are also run in `house`, ten runs a cell. The
  survey's 27 findings that were built without need were built by Opus and
  Fable, and atelier-zwh1's ten Sol implementers built none: they rebutted
  the finding that was true and not worth building, and a reviewer then
  deferred it. So `sol` cannot show the caving this change is for, and
  `house` may.
- [x] `docs/design.md` records the decision, its date, and the evidence.

## Scope

`skills/agent-factory/roles/implementer.md` and `roles/reviewer.md`; the
third form of a Responses line, where the two are parsed, shown and
checked in `crates/`; the lines the binary prints about answering and
rebutting a finding, with their tests; `docs/design.md`;
`evals/agent-factory/results/`. No new command, role or section of a
record.

## Report

An implementer can now answer a fix-now finding `<finding>: not built:
<reason>`, which takes it straight to the mission's lead. In atelier-zwh1's
case, Opus implementers (`house`) built the finding not worth building in 6
of 10 runs before this change and 0 of 10 after. All 20 runs after answered
it `not built` and stopped for the lead. The real defect was fixed in all
40 runs. Results: `evals/agent-factory/results/2026-11-19-say-no.md`.

What the binary does with the line (commit 4581a77ef):

- It parses as `Response::NotBuilt` wherever `fixed in` and `rebutted:` are
  read, and an unreadable line now names all three forms. The Desk's item
  page shows it.
- `submit` counts it as the finding's answer. It also stays the answer of
  the round-2 finding that carries it, until the implementer answers that
  one otherwise (`atelier_core::answer`, `waits_on_lead`).
- The lead's `status` lists each such open finding under "Waiting on your
  ruling", apart from what is merely disputed, with how to rule either way.
  `show <mission>` and `list --disputed` list it too.
- The next round's brief and template tell the reviewer the finding waits
  on the lead and not to judge it again. It stops neither `verdict approve`
  nor the record check of an approval. `verdict changes` on it alone is
  refused.
- `merge` refuses while one is open (`Condition::WaitsOnLead`). That
  refusal is a rule, and the lead passes it with
  `ATELIER_SKIP="<ruling>" atelier merge`. The implementer's `status`
  names the lead's two choices and says the implementer passes no rule on
  its own work.

An item with no such line reads as before. Tests: `disputes.rs` walks the
whole path. It covers the lead's status, the brief, the template, a refused
`verdict changes`, `approve`, the implementer's status, the implementer's
refused merge and the lead's skip. A second test covers a finding not
built and later fixed. `scenarios.rs` checks the new answering and one-try
lines, and `records.rs` checks the parse and the three-form error. All 402
Rust tests pass, along with clippy, `cargo fmt --check`, the Desk's
typecheck and `evals/agent-factory/test.sh`.

The skill (af618c727) gives `roles/implementer.md` the three exits.
`roles/reviewer.md` says what each recommendation costs the implementer,
and that a finding not built is the lead's to rule on. `SKILL.md`'s review
paragraph and `docs/product.md`'s Review section say the same. The
decision and its evidence are in `docs/design.md`'s table, and the
mechanism is under "Findings not built".

The eval arms are `7a73e7b4b` and `af618c727`, staged and built separately.
`git diff 7a73e7b4b af618c727` is the code and skill change alone.

Departures and open points:

- Nothing checks that whoever passes `merge`'s rule is the lead. The
  binary does not know who started an item with no mission, and the
  project's checks are nudges, so the skill and the printed lines say it
  instead. No run in the *with* arms skipped its own merge.
- The Desk's Disputed page does not list findings not built yet; it reads
  the core's `Disputes.rebuttals` and `plans` only. The lead's `status`,
  `show` and `list --disputed` do. That page is a follow-up for the lead to
  file, and adding it here would grow the Desk's API and app beyond this
  item's Scope.
- `docs/product.md` is outside the Scope's list. I changed it because it
  said "two exits" and "cannot dismiss a finding", which would contradict
  the skill.
- As in atelier-zwh1, no lead runs in the case, so no run shows a ruling.

### Responses

- 1-gzl9.1: fixed in e01638753
- 1-gzl9.2: fixed in 57090dfc8
