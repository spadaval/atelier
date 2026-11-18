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

## Done when

- [ ] `atelier show <mission>` lists each rule a commit of the mission or of
  its items passed: the item, the rule, the reason and the thread. It reads
  them from the same trailers `atelier log` reads.
- [ ] A mission with none says nothing about it.
- [ ] A test covers a fixture mission with an item merged past a rule.
- [ ] `docs/design.md` says where a passed rule is shown.

## Scope

`atelier show <mission>` in `crates/` and its tests; `docs/design.md`. Not
the Desk, and no new record.
