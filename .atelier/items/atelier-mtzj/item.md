---
schema: atelier.item
schema_version: 3
id: atelier-mtzj
title: Let a lead do what needs no review, and send everything else to an implementer
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

`roles/lead.md` says a lead writes no code, and sends every change to an
implementer, "a review finding's fix included". The human said on 2026-11-18
that this is the wrong line: a lead is not a project manager. What the rule
protects is review, and a lead that never has its own work in review can
never re-scope or settle a dispute on it. So the line is whether a change
needs review, which `SKILL.md` already defines, not who types it.

## Done when

- [ ] `roles/lead.md` says a lead does itself what needs no review: records,
  and a change whose correctness is evident from its diff, by the rule in
  `SKILL.md`'s Independent Review. It says how: `atelier start --new`, the
  commit, and `atelier merge --trivial "<why>"`. Everything else goes to an
  implementer through the task lines `status` prints, and a lead is never
  the implementer of a reviewed item in its own mission.
- [ ] Every line the binary prints to a mission's lead agrees with that
  text. The line in `say.rs` that keeps a lead from taking over a started
  item stays where it is right, and a test shows a lead can start a new item
  and merge it with `--trivial`.
- [ ] The file is no longer than it is today.
- [ ] `evals/agent-factory/test.sh` passes with the new text and binary.

## Scope

`skills/agent-factory/roles/lead.md`, `crates/atelier-core/src/say.rs` and
its tests. No other skill text.
