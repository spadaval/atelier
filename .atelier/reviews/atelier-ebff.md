---
schema: atelier.review
schema_version: 1
id: atelier-ebff
title: 'low: crates/atelier-cli/src/commands/merge.rs:653-669. 1hjj moves the report after taking the mutation locks and the base read. Paths checked: records-only move before or under the locks retries with no report, and only the returning attempt prints (once); a non-retry move (code commit, rewrite, base gone) and the later head/snapshot/checkout refusals print the report once before the error; exhausted retries print none, as before. Two behaviour changes are acceptable: a lock I/O error (lock_exclusive blocks, so only an I/O failure) now returns without the report, as the Open items say; and the base-ref check now precedes require_clean and the head checks, so when both the base moved records-only and the branch head or checkout changed, merge retries instead of refusing; the retry rereads head and snapshot in owned_branch and refuses or proves afresh, so nothing unproven integrates. No change needed.'
target: atelier-fngz
identity: Independent reviewer (Claude subagent)
commit: 97e785a471c9a95cea99441e15682e56a1c90826
created_at: 2026-10-22T23:43:27.390677421Z
event:
  event: finding
  recommendation: no_action
---

low: crates/atelier-cli/src/commands/merge.rs:653-669. 1hjj moves the report after taking the mutation locks and the base read. Paths checked: records-only move before or under the locks retries with no report, and only the returning attempt prints (once); a non-retry move (code commit, rewrite, base gone) and the later head/snapshot/checkout refusals print the report once before the error; exhausted retries print none, as before. Two behaviour changes are acceptable: a lock I/O error (lock_exclusive blocks, so only an I/O failure) now returns without the report, as the Open items say; and the base-ref check now precedes require_clean and the head checks, so when both the base moved records-only and the branch head or checkout changed, merge retries instead of refusing; the retry rereads head and snapshot in owned_branch and refuses or proves afresh, so nothing unproven integrates. No change needed.
