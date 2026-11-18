---
schema: atelier.item
schema_version: 3
id: atelier-nv22
title: List every rule a mission's commits passed, with its reason and thread, in atelier show <mission>
status: draft
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-18T20:28:22.241930090Z
---

## Goal

A lead may pass a rule by giving its reason, and what keeps that honest is
that the reason is read. Today it is kept and not shown: the commit carries
an `Atelier-Skip` trailer and `atelier log` prints it at the end of a line,
but `atelier show <mission>`, where the human reads a mission, lists none.
One rule has been passed since the cutover, and it can only be found with
`grep`.

The trailer also does not say who passed the rule. It holds the rule and
the reason, and `atelier log` names a merge's thread from the item's
implementers. A lead that merges an implementer's item past its review
would be shown as the implementer.

## Done when

- [ ] A commit that passes a rule records the thread that ran the command,
  in its trailers, beside the rule and the reason. The installed binary
  still reads a trailer without one.
- [ ] `atelier show <mission>` lists each rule a commit of the mission or of
  its items passed: the item, the rule, the reason and the thread that
  passed it. `atelier log` names the same thread. For a trailer written
  before this change both say the thread is not recorded.
- [ ] A mission with none says nothing about it.
- [ ] A test covers a fixture mission whose lead merged an implementer's
  item past a rule: both name the lead, not the implementer.
- [ ] `docs/design.md` says what a passed rule records and where it is
  shown.

## Scope

Where a passed rule is written to a commit and read back, `atelier show
<mission>` and `atelier log`, in `crates/` and their tests; `docs/design.md`.
Not the Desk, and no new record.
