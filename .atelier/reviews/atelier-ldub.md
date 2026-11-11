---
schema: atelier.review
schema_version: 2
id: atelier-ldub
title: finding on atelier-zrss @c18993d8
target: atelier-zrss
identity: reviewer-sol-r4
commit: c18993d814a74d506b05bf04c5cda3f1f948cba6
created_at: 2026-11-11T00:38:27.428962434Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-cli/src/merge.rs:94: R7. The pre-existing base_dirty guard still refuses directly instead of going through skip::refuse. With an approved item, a separate main checkout, and an uncommitted edit to an unrelated tracked notes.txt there, merge with ATELIER_SKIP="Keep unrelated base checkout edits" exits 1 and says the refusal is not a rule. No operation failed and the item exists: this is a guard on work state, so Done when 1 requires it to be skippable. Reproduced at the pinned head in a scratch repo; Git fast-forwarded that same base checkout to the item branch successfully with --ff-only --no-overwrite-ignore and preserved notes.txt. The new architecture list and Report classify this guard as a non-rule, narrowing the requirement without justification. Route the initial dirty-base guard through the bypass, print and trail it, snapshot the allowed base edits to detect later movement, and retain unskippable Git failures for genuinely conflicting local edits. Add a regression for merging an approved item with unrelated dirty base work and a reason. No approval in round 4. Required root and next fmt and locked metadata checks and whitespace pass; nextest passes all 222 tests; next-loop grader passes all 10 tests. R1-R6 fixes were inspected and their regression tests pass.
