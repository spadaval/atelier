---
schema: atelier.review
schema_version: 2
id: atelier-ja82
title: finding on atelier-z4cr @63af769d
target: atelier-z4cr
identity: GPT-6 Astra independent planning reviewer A
commit: 63af769de9dd7478e60b48bf9906ac5e8c36beff
created_at: 2026-11-11T02:22:43.113989392Z
event:
  event: finding
  recommendation: fix_now
---

R1 (medium) docs/strategies/desk.md:99-103: The proposed act executor cannot run either of its required commands. next/crates/atelier-cli/src/plan.rs:218-221 and :512-523 refuse ready outside the configured base branch, while questions.rs:80-98 requires decide on the item branch, where the pending question lives. A detached worktree at the base tip satisfies neither contract; committing and moving the base afterwards cannot repair a refused command. Authority at desk.md:177-180 also forbids changing tracker command behavior, so the implementer cannot resolve this within the approved strategy. Settle a command-compatible write design before approval, including the destination of decisions, human identity, concurrent writes and existing checkout/index preservation; require proof of both ready and decide against the new binary, not only the ready happy path. This also prevents the act-is-a-record criterion from being achievable as designed.
