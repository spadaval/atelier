---
schema: atelier.review
schema_version: 2
id: atelier-gle3
title: 'LOW merge.rs MergedCheckout: cleanup relies on Drop, so an interrupted merge (Ctrl-C or kill during long merge_checks) leaves a registered detached worktree under <git-common-dir>/atelier-merge-*/tree that git worktree prune will not remove because the directory exists; checks there also run cold (no shared build cache), so a cargo-based merge_checks rebuilds from scratch on every merge. Not a gate hole; worth a follow-up for stale-checkout cleanup and documenting the cost.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: 7185c3d00f161519ba59396b48f26ff2595cd28b
created_at: 2026-10-17T06:10:47.895955131Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude Opus 5.5)
    commit: 94560845091c8c5fb700e580dc84e43e14a86b3c
    recorded_at: 2026-10-17T06:11:27.563559754Z
    follow_up: atelier-rcnk
    reason: Leftover checkouts after an interrupted merge and build-cache reuse are tracked in atelier-rcnk; they don't let unchecked content reach the base.
    migrated_from: atelier-unnt
---

LOW merge.rs MergedCheckout: cleanup relies on Drop, so an interrupted merge (Ctrl-C or kill during long merge_checks) leaves a registered detached worktree under <git-common-dir>/atelier-merge-*/tree that git worktree prune will not remove because the directory exists; checks there also run cold (no shared build cache), so a cargo-based merge_checks rebuilds from scratch on every merge. Not a gate hole; worth a follow-up for stale-checkout cleanup and documenting the cost.
