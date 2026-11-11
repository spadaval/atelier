---
schema: atelier.review
schema_version: 2
id: atelier-2xgh
title: finding on atelier-zrss @5959a1ba
target: atelier-zrss
identity: reviewer-sol-r1
commit: 5959a1ba4d5f77746047b7eca95f9699bc275362
created_at: 2026-11-11T00:16:17.499197547Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-cli/src/merge.rs:501: R1. The moved-input check compares only the list of uncommitted paths, so a file that was already dirty can change again while merge checks run without being detected. Reproduced with an approved item, a dirty hello.txt, ATELIER_SKIP set, and a merge check paused on a release file: changing hello.txt from dirty before to changed during checks before releasing the check still exits 0 and advances main. This violates Done when 4 and the specification that inputs which moved cannot be skipped. Capture and compare the index and working-tree state, including contents and modes of the paths permitted to remain dirty, and add a regression test that edits an already-dirty file during the check.
