---
schema: atelier.item
schema_version: 3
id: atelier-mtzj
title: 'Put the lead in charge in the skill: the process as its default, and any departure said with its reason'
status: draft
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
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
  reviewer in its mission, either way, and records the ruling. The mission's
  Authority and the product's direction stay the human's.
- [ ] `roles/implementer.md` says a kept finding is fixed or taken to the
  lead, and that an implementer passes no rule on its own work. `SKILL.md`'s
  Independent Review says the same as both.
- [ ] The line `status` prints for a kept finding names the lead beside
  "fix it", and every line the binary prints to a lead agrees with the
  skill. A test shows a lead merging an item past an open fix-now finding
  with a reason, and the reason in `atelier log`.
- [ ] `docs/design.md` records the decision with its date and the human's
  reasons. The Report gives the word count of `SKILL.md` and `roles/` before
  and after; they are 1,231 today.
- [ ] `evals/agent-factory/test.sh` passes with the new text and binary.

## Scope

`skills/agent-factory/SKILL.md`, `roles/lead.md` and `roles/implementer.md`;
`crates/atelier-core/src/say.rs` and its tests; `docs/design.md`. No new
command, role or record.
