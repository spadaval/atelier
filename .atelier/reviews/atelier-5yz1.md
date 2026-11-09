---
schema: atelier.review
schema_version: 2
id: atelier-5yz1
title: finding on atelier-yfqu @28ae3d61
target: atelier-yfqu
identity: codex-reviewer (gpt-6.1-sol)
commit: 28ae3d61ecce47e8befb0e1469e16122d8addf04
created_at: 2026-11-09T00:51:49.770096403Z
event:
  event: finding
  recommendation: fix_now
---

medium: Git status and refs cannot prove the server makes no repository writes.
.atelier/issues/atelier-zpmb.md:35,44; .gitignore:14; crates/atelier-cli/src/commands/mission_review.rs:237-247.
The route test checks only git status and refs. The existing debrief command writes .atelier/runtime/reviews, which is ignored, so invoking that writer on every request would pass the proposed test despite violating the read-only server contract. Git object writes would also escape that assertion. Require a pure rendering path separate from the CLI file writer, and prove all routes work with the repository, Git directory and ignored runtime paths unwritable, or use a filesystem-write observer that covers them. Retain the ref/status assertions as additional checks.
Recommendation: FIX NOW; the named proof cannot fail for an existing, directly reusable violating implementation.
