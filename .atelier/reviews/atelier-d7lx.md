---
schema: atelier.review
schema_version: 2
id: atelier-d7lx
title: finding on atelier-pz1y @e5ad73f8
target: atelier-pz1y
identity: sol-pz1y-r1
commit: e5ad73f85965ffb06f34863d8d3e8d66e86ca848
created_at: 2026-11-10T22:30:42.357596098Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-records/src/load.rs:116: R1. Reviews whose item or mission is missing or does not parse are dropped before validate runs. In a scratch repo with the real pre-commit hook, adding .atelier/items/demo-ab12/reviews/1-abcd.md with round: 2 and no item.md commits successfully (exit 0); the missing-owner problem names item.md, so check.rs:93-95 filters it out. The same invalid review also commits when item.md is already broken and unchanged. Standalone check reports only the missing/broken owner and omits the review round problem; adding a valid owner makes it report that problem. This breaks validation of every review and lets Git commit an invalid staged record. Validate reviews independently of owner parsing, keep their diagnostics on the review path, and test missing and unreadable owners for both items and missions. Approval withheld. Fresh proof at the pinned head passed: all 181 next tests including 10 hook scenarios, clippy with -D warnings, locked all-targets build, root and next fmt, locked root metadata and the requested diff check.
