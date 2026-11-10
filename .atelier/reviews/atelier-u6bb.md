---
schema: atelier.review
schema_version: 2
id: atelier-u6bb
title: finding on atelier-xtvh @83d1b75e
target: atelier-xtvh
identity: gpt-6.1-sol
commit: 83d1b75ee594974f6a6676a39eddeaa491420ce2
created_at: 2026-11-10T17:46:47.024902679Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-xtvh.md:40 newly gates both the branch push and opening the pull request on the remote branch being missing or its tip being ineligible. But line 38 explicitly permits submit to push successfully and then fail to open its pull request, returning 0 with the item in_review. After approval the remote submit commit is still eligible under line 39, so neither condition in line 40 holds. A worker following this literally skips PR creation and waits repeatedly for pull_request CI that cannot start; without a CI workflow it can merge without opening the required PR. The prerequisite at atelier-syyj edc10b23, next/crates/atelier-cli/src/merge.rs:122-131, treats pushing and finding/creating the PR as separate operations, and .github/workflows/ci.yml:4-7 runs branch CI on pull_request, not a feature-branch push. Separate the rules: skip the branch push when its remote tip is eligible, but independently find or open the PR before waiting. Add a submit-push-succeeds/PR-creation-fails recovery test showing that merge opens the missing PR without another branch push and then completes after CI passes. This is a regression in the plan's fix for atelier-y1tc, not a new feature.

Round 2 planning verdict: merge after this readiness defect is fixed and re-reviewed. Sizes: complexity medium (CI polling, eligible-run selection and merge concurrency); value high (implements the two human decisions); risk medium (forge writes and merge validation, isolated in next/). Contract: fail on the PR-recovery path. Fitness: fail until the two preparation conditions are separate. The four round-1 findings are substantively addressed by 83d1b75e: fresh post-wait validation and changed-base/branch scenarios (atelier-lmx5), a deadline covering a stalled poll and ending its process (atelier-o80t), a defined preparation/wait snapshot boundary and concrete push policy (atelier-y1tc), and inclusion of product Merge check 3 (atelier-5m52). No approval recorded while this new fix_now stands. Read-only review of the plan, specifications, mission decision and prerequisite branch; atelier check atelier-xtvh and the planning diff's whitespace check passed. No build or runtime scenario run; the item is unimplemented and atelier-syyj remains in progress.
