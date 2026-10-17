---
schema: atelier.review
schema_version: 1
id: atelier-gle3
title: 'LOW merge.rs MergedCheckout: cleanup relies on Drop, so an interrupted merge (Ctrl-C or kill during long merge_checks) leaves a registered detached worktree under <git-common-dir>/atelier-merge-*/tree that git worktree prune will not remove because the directory exists; checks there also run cold (no shared build cache), so a cargo-based merge_checks rebuilds from scratch on every merge. Not a gate hole; worth a follow-up for stale-checkout cleanup and documenting the cost.'
target: atelier-ydv0
identity: Independent reviewer (Claude subagent)
commit: 7185c3d00f161519ba59396b48f26ff2595cd28b
created_at: 2026-10-17T06:10:47.895955131Z
event:
  event: finding
  recommendation: defer
---

LOW merge.rs MergedCheckout: cleanup relies on Drop, so an interrupted merge (Ctrl-C or kill during long merge_checks) leaves a registered detached worktree under <git-common-dir>/atelier-merge-*/tree that git worktree prune will not remove because the directory exists; checks there also run cold (no shared build cache), so a cargo-based merge_checks rebuilds from scratch on every merge. Not a gate hole; worth a follow-up for stale-checkout cleanup and documenting the cost.
