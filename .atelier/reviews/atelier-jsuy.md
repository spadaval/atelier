---
schema: atelier.review
schema_version: 2
id: atelier-jsuy
title: docs/product/cli-surface.md:3-5 still says 'During the rebuild use target/debug/atelier; update the installed binary only after the new model is stable', the same stale rebuild text this branch removed from development-setup.md and AGENTS.md; it now directly contradicts both (hooks run the installed CLI; reinstall after merging session changes). Fix here with the other stale rebuild lines (and add it to bw0b Scope), or explicitly assign it to atelier-sdb0, which already edits cli-surface.md.
target: atelier-bw0b
identity: Independent reviewer 3 (Claude subagent)
commit: 6c51447373552e58ab54acc8783d3a64c6a9991e
created_at: 2026-10-16T23:56:15.345602209Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (author)
    commit: 30dffbfe201ad35937d483d46fa38f5aa9d125ee
    recorded_at: 2026-10-16T23:57:14.131490134Z
    resolution_commit: 30dffbfe201ad35937d483d46fa38f5aa9d125ee
    reason: cli-surface.md stale rebuild text replaced; added to bw0b Scope
    migrated_from: atelier-ohj3
---

docs/product/cli-surface.md:3-5 still says 'During the rebuild use target/debug/atelier; update the installed binary only after the new model is stable', the same stale rebuild text this branch removed from development-setup.md and AGENTS.md; it now directly contradicts both (hooks run the installed CLI; reinstall after merging session changes). Fix here with the other stale rebuild lines (and add it to bw0b Scope), or explicitly assign it to atelier-sdb0, which already edits cli-surface.md.
