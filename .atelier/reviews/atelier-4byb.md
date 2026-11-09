---
schema: atelier.review
schema_version: 2
id: atelier-4byb
title: finding on atelier-ah82 @661625da
target: atelier-ah82
identity: codex-reviewer (gpt-6.1-sol)
commit: 661625dad956b33c3967870c050ec88418166ebc
created_at: 2026-11-09T02:56:22.661657072Z
event:
  event: finding
  recommendation: fix_now
---

medium: docs/product/reviews-and-merge.md:467 R1: The No receipts paragraph still says CI or merge_checks test the merged tree, although ci.passed is now itself a merge check and explicitly accepts a run against an earlier base. The generic claims at reviews-and-merge.md:388 and validation.md:27 have the same problem. In addition, reviews-and-merge.md:626 promises local shell checks catch a merge that breaks the build, but the current policy runs only whitespace, fmt, metadata and the CI script; none builds the merged combination. These statements contradict the documented proof boundary and the last Done when line. Scope merged-tree claims to the configured local shell.check commands, state that they prove only what they actually run, and remove the unconditional build guarantee.
