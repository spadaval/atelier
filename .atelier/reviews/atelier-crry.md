---
schema: atelier.review
schema_version: 2
id: atelier-crry
title: finding on atelier-zrss @c9c00da5
target: atelier-zrss
identity: reviewer-sol-r2
commit: c9c00da59318bbc2d39783f5e6e8cae76aba8f29
created_at: 2026-11-11T00:24:49.470078938Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-cli/src/merge.rs:229: R4. Dirt follows symbolic links with fs::read and turns every read error into empty bytes, so it does not snapshot the link target itself. Reproduced at the pinned head in a scratch repository with an approved item and a tracked link.txt: change the link from its committed target to missing-a, then have a merge check replace it with missing-b while merge runs with ATELIER_SKIP set. Both targets are dangling; merge exits 0 and advances main even though the dirty link changed during its checks. The same miss occurs for two targets with identical contents. This leaves atelier-2xgh only partly fixed and violates Done when 4, which says moved inputs cannot be skipped. Snapshot symlinks with read_link rather than following them, distinguish expected absence from failed I/O, and add a regression test that changes an already-dirty link target during checks. The required diff, formatting and locked metadata checks pass in both workspaces; next/ nextest passes all 222 tests and the next-loop grader passes all 10 tests, so the existing proof does not cover this case.
