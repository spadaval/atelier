---
created_at: "2026-10-17T19:00:36.350119527+00:00"
id: "atelier-2yl7"
issue_type: "feature"
labels:
- "merge"
- "retro-2026-10-17"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-6mgg"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Default to merge_commit merging when the repository defines a test check"
updated_at: "2026-10-22T19:58:19.278824634+00:00"
---

## Goal

Under `fast_forward_only`, each merge forces every other open branch to catch up, re-prove and re-approve: 122 `Merge branch 'master' into ...` commits, 41 fresh-approval commits and 27 post-catch-up suite reruns in three days, rising daily (13, 44, 64). Since this repo switched to `merge_commit` (qymu, 4f952222) the two branches that landed needed no last-mile catch-up. Make `merge_commit` the default when the policy has a test `shell.check`, so new repositories don't start in the churn mode.

## Done when

`atelier init` and a policy without `merge_strategy` select `merge_commit` when a `shell.check` merge check exists, and `fast_forward_only` otherwise. `docs/product/workflow-configuration.md` states the default and why. Tests cover both defaults.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). The w56o guidance alone did not reduce churn (catch-ups per branch rose 1.42 -> 1.97); the strategy did.

## Unknowns

k8h8 S3 leaves the default to Sumeet; this item is that decision plus its implementation. The post-switch sample is two branches, so S6's count from a larger run may be wanted first.
