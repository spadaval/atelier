---
schema: atelier.review
schema_version: 1
id: atelier-ybdb
title: 'crates/atelier-cli/src/commands/issue.rs:1383 (with atelier-core DecisionNote::note_parts): `issue note --kind decision` writes notes that `check` then refuses. note_parts flattens --reason and --rejected but only trims the choice, and nothing round-trips the result through DecisionNote::parse. Failure (reproduced): `issue note X --kind decision $''Use V\nReason: injected'' --reason R` or `issue note X --kind decision ''Reason: starts with label'' --reason R` both succeed, then check reports ''has more than one Reason: line''. Fix: flatten the choice like the reason (or refuse a choice line starting with Reason:/Rejected:), and validate the built note with DecisionNote::from_note before writing so the command never stores a note check refuses; add a test.'
target: atelier-fip8
identity: Independent reviewer (Claude subagent)
commit: cc8bd55a9665bf7504da8bdb128b240bab442e8a
created_at: 2026-10-17T17:43:39.460884070Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/issue.rs:1383 (with atelier-core DecisionNote::note_parts): `issue note --kind decision` writes notes that `check` then refuses. note_parts flattens --reason and --rejected but only trims the choice, and nothing round-trips the result through DecisionNote::parse. Failure (reproduced): `issue note X --kind decision $'Use V\nReason: injected' --reason R` or `issue note X --kind decision 'Reason: starts with label' --reason R` both succeed, then check reports 'has more than one Reason: line'. Fix: flatten the choice like the reason (or refuse a choice line starting with Reason:/Rejected:), and validate the built note with DecisionNote::from_note before writing so the command never stores a note check refuses; add a test.
