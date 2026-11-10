---
schema: atelier.review
schema_version: 2
id: atelier-k3iu
title: finding on atelier-pz1y @1e0e6d51
target: atelier-pz1y
identity: gpt-6.1-sol
commit: 1e0e6d51877194c06d65736e7f7c7e760715d6e7
created_at: 2026-11-10T21:36:08.937156971Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-pz1y.md:41: Step D gives post-checkout incompatible exit requirements. This line says it never fails the checkout, but atelier-ym3z.md:33 requires check --hook <name> to fail whenever the kept .local hook fails, without excluding post-checkout; atelier-zrss.md:37 also requires every hook to refuse an empty ATELIER_SKIP. Git propagates the post-checkout exit status as the exit status of git switch/checkout even though the worktree has already switched (installed githooks(5), post-checkout). next/crates/atelier-cli/src/git.rs:233 treats non-zero git switch as failure, so the contradiction reaches start and take-over after the branch has moved. Smallest fix: settle one explicit post-checkout exit contract across these three bodies, distinguishing Atelier orientation failures, kept project-hook failures and invalid bypass input; specify whether each is diagnostic-only or returns non-zero. Keep the specification that post-checkout adds orientation rather than a lifecycle gate. Add scenarios for failed status, a failing kept post-checkout hook and blank ATELIER_SKIP, asserting both the Git exit status and the resulting branch, not only printed status. Recommendation: FIX NOW. Planning verdict: merge after the hook interfaces agree.
