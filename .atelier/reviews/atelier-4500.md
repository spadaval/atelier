---
schema: atelier.review
schema_version: 2
id: atelier-4500
title: finding on atelier-u1ko @d41d3bcd
target: atelier-u1ko
identity: GPT-6 Astra independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:50:10.326293706Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-u1ko.md:63,107 The CI publication route is allowed only when no approval of this item exists, including planning approvals these very reviews are supposed to record on master before implementation. Such records survive into the work branch, and an approval followed by a later code fix creates the same problem: the plan forbids obtaining CI for the revised code. merge_push.rs:82-94 does publish before gates, but :136-138 then really merges if they pass; omitting --wait is not a dry run. Specify a verified refusing gate for the exact candidate code (distinguishing stale planning/earlier approvals from a current implementation approval) and the allowed route for subsequent rounds, while preserving the absolute no-merge hold. Do not rely on literal absence of all historical approvals.
