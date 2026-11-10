---
schema: atelier.review
schema_version: 2
id: atelier-deu5
title: finding on atelier-jq24 @949a8b62
target: atelier-jq24
identity: gpt-6.1-sol
commit: 949a8b62c729bd70f049a47c4539fae77f01f578
created_at: 2026-11-10T20:54:36.620181154Z
event:
  event: finding
  recommendation: fix_now
---

R3, medium: next/crates/atelier-app/src/acts.rs:587 filters the candidate findings to this file and earlier rounds, but passes every sibling finding to is_open. Another review of the same round can therefore hide an earlier finding the caller omitted. Reproduced with the pinned binary: round 1 records changes with one fix-now finding; two detached worktrees open blind round-2 reviews; reviewer B keeps that finding and records changes; reviewer A deletes the carried section. A verdict approve correctly refuses before importing B, then succeeds after cherry-picking Bs verdict commit, with no edit to As review. is_open treats Bs Was line as moving the round-1 finding to B, while the outer filter excludes B. This contradicts the specified own-file-and-earlier-rounds approval check and lets a review omit an unresolved finding without judging it. Evaluate openness against the same own-file-and-earlier-rounds set for a plan verdict, and add this two-reviewer round-2 regression. Overall verdict: merge after R1-R3 are fixed; no shrink needed. Sizes: complexity high (shared lifecycle and review contracts), value high (required step B exits and plan reviews), integration risk low (isolated next/ remains uninstalled). Contract and Fitness fail for these reproduced gaps. Fresh proof: required root whitespace, fmt and locked metadata checks pass; next/ fmt and locked metadata pass; all 158 nextest tests pass with four build jobs and four test threads; clippy --all-targets -D warnings passes. Every added moving part traces to the requested lifecycle exits, plan reviews or their shared readers. No approval recorded.
