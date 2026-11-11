---
schema: atelier.review
schema_version: 2
id: atelier-9ge3
title: finding on atelier-u1ko @d41d3bcd
target: atelier-u1ko
identity: GPT-6.1 Sol independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:51:42.245424932Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-u1ko.md:63,107 The only permitted way to publish this branch for CI is conditioned on no approval of the item existing, and is forbidden once any reviewer has approved. But Planning on the Base records planning approvals against this same item before implementation, and review list retains all approval events, including stale/predates-fork ones (crates/atelier-cli/src/commands/review.rs:378-403). An approved plan therefore makes this instruction unusable before the first CI run; a later code change after a work approval has the same problem. The old merge really does push/open the PR before gates (merge_push.rs:75-88), and its gate distinguishes a current work approval from earlier/stale approvals (atelier-app/src/review_gate.rs:444-455). State and prove the guard in terms of review.current refusing the committed implementation head, with no reviewer concurrently approving it, rather than absence of historical approvals. Keep the hold once a current work approval exists. Include a planning-approved fixture in proof that publication starts CI without moving master; do not use --check --push, which the parser rejects.
