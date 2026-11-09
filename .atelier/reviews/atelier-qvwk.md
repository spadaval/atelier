---
schema: atelier.review
schema_version: 2
id: atelier-qvwk
title: finding on atelier-yfqu @a9cd1265
target: atelier-yfqu
identity: gpt-6.1-sol
commit: a9cd1265fafcb1a44917fc15c10033f3120137b2
created_at: 2026-11-09T14:38:19.366563715Z
event:
  event: finding
  recommendation: fix_now
---

Medium: Discovery must choose the base ref before it can read policy at the base commit.
Locations: docs/strategies/fleet-dashboard.md:113-118; .atelier/issues/atelier-zpmb.md:33-34,47.
The design says to discover each base branch from its workflow policy at the base commit, while forbidding HEAD lookups and working-tree reads, but does not specify how the first base ref is selected. This is not provided by the current loader: workflow_policy::load calls base_policy::working (workflow_policy.rs:210-211), which reads the working policy and otherwise falls back through HEAD (base_policy.rs:105-130). from_captured only resolves policy after its caller already has a commit. A detached checkout, or a main checkout on a work branch with a different base_branch policy, cannot establish the intended base using the stated interface; guessing main/master also breaks repositories with another configured base. Choosing the wrong ref silently makes every page authoritative for the wrong history.
Recommendation: FIX NOW. Specify a deterministic bootstrap source and ambiguity/error behavior before the loader is ready, consistent with the no-working-tree contract. Add a discovery fixture with a nonstandard base name and a divergent checkout whose policy names a different base, as well as the existing stale-worktree case. This settles the source-of-truth choice without changing the chosen frontend stack.
