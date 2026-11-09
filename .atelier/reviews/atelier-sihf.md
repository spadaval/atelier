---
schema: atelier.review
schema_version: 2
id: atelier-sihf
title: finding on atelier-fad5 @0ae80c75
target: atelier-fad5
identity: codex-reviewer (gpt-6.1-sol)
commit: 0ae80c75f4325cfeab5d0802e3318af8b08f818a
created_at: 2026-11-09T04:34:50.091828430Z
event:
  event: finding
  recommendation: fix_now
---

medium: R3 crates/atelier-cli/src/commands/merge_push.rs:93 Pull-request confirmation silently disappears when the local head is an unpushed record-only approval commit, or when the lookup fails. The lookup asks only commits/<integrated.head>/pulls, erases every error with ok().flatten(), and treats the result as no pull request. CI deliberately accepts an earlier commit with identical code, and record-only approvals need no new CI run. Reproduced with the code commit pushed to docs and open PR #7, passing PR CI on that commit, then a local approval record: ci.passed named #7, merge published successfully, but the lookup for the unknown approval SHA failed, no pulls/7 state read occurred, and neither a merged confirmation nor the still-open note printed. The added docs/product/reviews-and-merge.md:698 exact-head clause narrows the reviewed confirmation contract around this ordinary accepted path. Find the existing branch/base PR or retain its identity from the accepted CI run; preserve lookup failures as an explicit inability to confirm rather than claiming no PR exists. Add a test with CI on the pushed code commit, an unpushed approval commit, and a fake gh that rejects unknown commit SHAs. Proof output: /home/supadava/.cache/atelier-tmp/fad5-review/fad5-probes-wx6rgw0c/unpushed-approval/output.txt. Required proof passed independently at the pinned head: forge_ci, merge_gates and session, 255 tests without test-hooks and 259 with test-hooks (-j 4); cargo fmt -- --check, both diff whitespace checks, cargo metadata --locked, and branch-build check atelier-fad5 passed. No approval: R1-R3 remain fix_now. Recommendation: FIX NOW.
