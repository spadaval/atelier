---
schema: atelier.review
schema_version: 2
id: atelier-h103
title: finding on atelier-ah82 @661625da
target: atelier-ah82
identity: codex-reviewer (gpt-6.1-sol)
commit: 661625dad956b33c3967870c050ec88418166ebc
created_at: 2026-11-09T02:56:23.123761478Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/product/reviews-and-merge.md:638 R2: The ci_pending Next command always drops --push. On the normal merge --push path where CI is pending, following Next: atelier merge <id> --wait merges locally and deletes the work branch but never publishes the base, exactly as line 731 specifies for --wait alone. The user asked to publish and cannot finish by rerunning merge --push after the branch is deleted. Preserve --push in the Next command when the refused invocation requested it, including timeout refusals, and show that case in the example.
