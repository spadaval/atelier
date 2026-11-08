---
schema: atelier.review
schema_version: 2
id: atelier-uplq
title: finding on atelier-pmib @859f41a2
target: atelier-pmib
identity: plan-reviewer-g6rp
commit: 859f41a2454c4278bc33d18868729b139e7e90f7
created_at: 2026-11-08T05:58:06.986942393Z
event:
  event: finding
  recommendation: fix_now
---

[medium] The ADR item retires ADR 0020 Decision 3 in full, but Decision 3 has two halves: 'the skill maps concepts; Atelier owns syntax' (the reference names commands and defers flags to atelier help), and the rule that a command change updates the upstream skill in the same piece of work. Only the second becomes moot. The strategy's Shape keeps the first ('the skill says what to do and why, the CLI's help says how'), so an ADR that follows this Done when literally would delete the principle the mission relies on. Fix: retire only Decision 3's sync sentence and restate that the concepts/syntax split stands (as amended by ADR 0022).
