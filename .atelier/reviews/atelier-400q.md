---
schema: atelier.review
schema_version: 2
id: atelier-400q
title: finding on atelier-zrss @fccfc15e
target: atelier-zrss
identity: reviewer-sol-r3
commit: fccfc15e4f37cfa212b6afa84e841dd3559edf41
created_at: 2026-11-11T00:30:09.967920624Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-cli/src/merge.rs:237: R5. Dirt snapshots every directory as empty bytes, so it misses changes to an already-dirty Git submodule. Reproduced at the pinned head in a scratch repository with a tracked submodule committed at A and an approved item: check out B in the submodule, then let a configured merge check check out C there. With ATELIER_SKIP set, merge exits 0 and advances main, although the submodule HEAD changed while the checks ran; both snapshots contain only the same path, parent-index gitlink and directory mode. This leaves the moved-input guarantee in Done when 4 incomplete. Snapshot the submodule state that can change, or refuse an unsupported dirty directory as not skippable, and add a regression proving the base stays unchanged when it moves. Required diff, formatting and locked metadata checks pass in both workspaces; next/ clippy and all 222 nextest tests pass; all 10 next-loop grader tests pass. The existing proof does not exercise submodule changes.
