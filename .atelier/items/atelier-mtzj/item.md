---
schema: atelier.item
schema_version: 3
id: atelier-mtzj
title: 'Put the lead in charge in the skill: the process as its default, and any departure said with its reason'
status: building
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A15ed7340-46a6-4e3a-b0c2-fdc0661b938f%3Adelegate-task%3A96c8fe21-678c-4553-9659-a7cabe5c18ed
ready:
  by: 'the human, in conversation on 2026-11-18: put the mission together and then run it'
  at: 2026-11-18T23:49:47.284668623Z
trivial: null
caused_by: null
created_at: 2026-11-18T20:05:51.803607972Z
---

## Goal

The human said on 2026-11-18 that a lead is responsible for finishing its
mission and may do more or less anything to get there, giving a reason for
each departure from the process, a skipped review included. The binary
already records a reason given with `ATELIER_SKIP`. The skill says the
opposite: `roles/lead.md` opens "You write no code" and says of a disputed
finding that it "is not yours to settle", and `roles/implementer.md` says to
fix a kept finding even where the fix breaks a Done when line, which leaves
an item that can never merge.

## Done when

- [ ] `roles/lead.md` says the lead is responsible for finishing the
  mission. It gives the process as the default: the lead does itself what
  needs no review (records, and a change whose correctness is evident from
  its diff), sends the rest to an implementer, and has each change reviewed
  by a thread that did not write it.
- [ ] It says the lead may depart from any of that, a review included, by
  giving its reason with `ATELIER_SKIP` on the command, and never silently:
  not by writing a status by hand, changing a hook, or calling a change
  trivial that is not.
- [ ] It says the lead settles a dispute between an implementer and a
  reviewer in its mission, either way, and records the ruling.
- [ ] It says the lead may do less than the plan asks, where the work turns
  out much harder than the plan took it to be: drop an item, or drop the
  mission, with the reason `atelier drop` records. It gives the steps the
  binary takes today for a started item, which is dropped on its own branch
  and then its copy on the base on a backlog branch, and says the mission's
  Result names at its close what was not done and why. It may not do more
  or other than the plan asks: what the Authority withholds and the
  product's direction stay the human's.
- [ ] `roles/implementer.md` says a kept finding is fixed or taken to the
  lead, and that an implementer passes no rule on its own work. `SKILL.md`'s
  Independent Review says the same as both.
- [ ] The line `status` prints for a kept finding names the lead beside
  "fix it", and every line the binary prints to a lead agrees with the
  skill, wherever it lives. One is in `crates/atelier-cli/src/view_output.rs`:
  the heading over a mission's disputes says a lead steps in by moving the
  disputed work out of the item's Scope. A test shows a lead merging an item
  past an open fix-now finding with a reason, and the reason in `atelier
  log`.
- [ ] `docs/design.md` records the decision with its date and the human's
  reasons. The Report gives the word count of `SKILL.md` and `roles/` before
  and after; they are 1,231 today.
- [ ] `evals/agent-factory/test.sh` passes with the new text and binary.

## Scope

`skills/agent-factory/SKILL.md`, `roles/lead.md` and `roles/implementer.md`;
the text the binary prints about a kept finding and to a lead, in
`crates/atelier-core/src/say.rs`, `crates/atelier-cli/src/view_output.rs`
and wherever else it is found, with its tests; `docs/design.md`. No new
command, role or record.
