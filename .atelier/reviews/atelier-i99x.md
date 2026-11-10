---
schema: atelier.review
schema_version: 2
id: atelier-i99x
title: finding on atelier-jq24 @949a8b62
target: atelier-jq24
identity: gpt-6.1-sol
commit: 949a8b62c729bd70f049a47c4539fae77f01f578
created_at: 2026-11-10T20:54:36.195266904Z
event:
  event: finding
  recommendation: fix_now
---

R2, medium: next/crates/atelier-cli/src/plan.rs:385 copies only item.md when dropping an item that exists solely on its branch, leaving its body references without their review files on the base. Reproduced: t1 runs start --new in a worktree; submit; r1 reviews and records changes with Finding 1; t1 adds a valid Report Responses line naming that finding and commits. On the clean base, drop <item> --reason Cancelled exits 1: Responses: <review>.1 names no finding of this items reviews. The base remains unchanged, so this reviewed work cannot be cancelled with drop. save inserts the copied item with reviews: Vec::new(), then validates its Responses and Decisions against that empty context. Handle the referenced review context when materializing the base drop record so it validates without changing the work branch, and cover the branch-only item with a real review and Responses in a scenario. The architecture at lines 486-488 explicitly promises this start --new case.
