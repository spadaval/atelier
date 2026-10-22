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
status: "in_progress"
title: "Default to merge_commit merging when the repository defines a test check"
updated_at: "2026-10-22T20:47:01.226059885+00:00"
---

## Goal

Under `fast_forward_only`, each merge forces every other open branch to catch up, re-prove and re-approve: 122 `Merge branch 'master' into ...` commits, 41 fresh-approval commits and 27 post-catch-up suite reruns in three days, rising daily (13, 44, 64). Since this repo switched to `merge_commit` (qymu, 4f952222) the two branches that landed needed no last-mile catch-up. Make `merge_commit` the default when the policy has a test `shell.check`, so new repositories don't start in the churn mode.

## Done when

`atelier init` and a policy without `merge_strategy` select `merge_commit` when a `shell.check` merge check exists, and `fast_forward_only` otherwise. `docs/product/workflow-configuration.md` states the default and why. Tests cover both defaults.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). The w56o guidance alone did not reduce churn (catch-ups per branch rose 1.42 -> 1.97); the strategy did.

## Unknowns

k8h8 S3 leaves the default to Sumeet; this item is that decision plus its implementation. The post-switch sample is two branches, so S6's count from a larger run may be wanted first.

## Notes

- 2026-10-22 [decision] The starter policy omits merge_strategy so the parser's rule decides; the rule lives only in the parser
  Reason: atelier init then gives the same result as any policy without the field (fast_forward_only with no shell.check, merge_commit once one is added), with no second copy of the rule to drift. Existing repositories set merge_strategy explicitly and parse unchanged; an installed binary older than this change reads an omitted value as squash, which merge already refuses, so only newly initialised repositories need the new binary.
  Rejected: Writing the resolved strategy into the starter: preserves old-binary reads but leaves a new repository on fast_forward_only after it adds its first test check
